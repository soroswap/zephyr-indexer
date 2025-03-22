use zephyr_sdk::{prelude::*, soroban_sdk::{xdr::{ScVal, Hash }, String as SorobanString, Address}, PrettyContractEvent,  EnvClient, DatabaseDerive};
use std::env;

pub mod router;
pub mod factory;
pub mod pairs;
pub mod providers;

#[derive(DatabaseDerive, Clone)]
#[with_name("ssw_rt_ev")]
struct EventsTable {
    e_type: ScVal,
    token_a: ScVal,
    token_b: ScVal,
    amount_a: ScVal,
    amount_b: ScVal,
    account: ScVal,
    timestamp: ScVal,
    tx_hash: ScVal,
}

#[derive(DatabaseDerive, Clone)]
#[with_name("ssw_pairs")]
struct PairsTable {
    address: ScVal,
    token_a: ScVal,
    token_b: ScVal,
    reserve_a: ScVal,
    reserve_b: ScVal,
}

#[derive(DatabaseDerive, Clone)]
#[with_name("ssw_rs_ch")]
struct ReservesChangeTable {
    address: ScVal,
    reserve_a: ScVal,
    reserve_b: ScVal,
    timestamp: ScVal,
}

#[derive(DatabaseDerive, Clone)]
#[with_name("ssw_providers")]
pub struct ProvidersTable {
    provider_address: ScVal,
    pool_address: ScVal,
    shares: ScVal,
    share_percentage: f64,
    token_a_amount: ScVal,
    token_b_amount: ScVal,
    timestamp: ScVal,
    action: String,
    tx_hash: ScVal,
}

impl ProvidersTable {
    pub fn put(&self, env: &EnvClient) {
        let default_limits = soroban_sdk::xdr::Limits::none();

        env.db_write(
            "ssw_providers",
            &[
                "provider_address", "pool_address", "shares", "share_percentage", "token_a_amount", "token_b_amount", "timestamp", "action", "tx_hash",
            ],
            &[
                &self.provider_address.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.pool_address.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.shares.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.share_percentage.to_le_bytes(),
                &self.token_a_amount.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.token_b_amount.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.timestamp.to_xdr(default_limits.clone()).unwrap_or_default(),
                &self.action.as_bytes(),
                &self.tx_hash.to_xdr(default_limits.clone()).unwrap_or_default(),
            ],
        ).unwrap();
    }
}
#[test]
fn test() {
    let factory_address_str: &'static str = env!("SOROSWAP_FACTORY");
    let router_address_str: &'static str = env!("SOROSWAP_ROUTER");
    println!("{:?}", factory_address_str);
    println!("{:?}", router_address_str);
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();

    let factory_address_str =
        env::var("SOROSWAP_FACTORY").expect("SOROSWAP_FACTORY environment variable not set");
    let router_address_str =
        env::var("SOROSWAP_ROUTER").expect("SOROSWAP_ROUTER environment variable not set");

    env.log().debug(
        format!("Indexing SSW Factory: {:?}", &factory_address_str),
        None,
    );
    env.log().debug(
        format!("Indexing SSW Router: {:?}", &router_address_str),
        None,
    );

    let factory_address_bytes: [u8; 32] =
        stellar_strkey::Contract::from_string(&factory_address_str)
            .unwrap()
            .0;
    let router_address_bytes: [u8; 32] = stellar_strkey::Contract::from_string(&router_address_str)
        .unwrap()
        .0;

    let contract_events_with_txhash: Vec<(PrettyContractEvent, [u8; 32])> =
        env.reader().pretty().soroban_events_and_txhash();

    let filtered_router_events_with_txhash: Vec<(PrettyContractEvent, [u8; 32])> =
        contract_events_with_txhash
            .clone()
            .into_iter()
            .filter(|(event, _txhash)| event.raw.contract_id == Some(Hash(router_address_bytes)))
            .collect();

    let filtered_factory_events_with_txhash: Vec<(PrettyContractEvent, [u8; 32])> =
        contract_events_with_txhash
            .clone()
            .into_iter()
            .filter(|(event, _txhash)| event.raw.contract_id == Some(Hash(factory_address_bytes)))
            .collect();

    //handle the events from the router contract
    router::events::handle_contract_events(&env, filtered_router_events_with_txhash);

    //handle the events from the factory contract
    factory::events::handle_contract_events(&env, filtered_factory_events_with_txhash);

    // At this point our PairsTable is up to date. Now we can handle the events from the Pairs
    let pairs_rows = env.read::<PairsTable>();

    env.log()
        .debug(format!("Before pairs_rows iteration"), None);

    for pair in pairs_rows {
        let pair_address: Address = env.from_scval(&pair.address);
        env.log()
            .debug(format!("Checking pair: {:?}", &pair_address), None);

        // We filter to see if there is any event related to our Pair
        let pair_contract_events: Vec<(PrettyContractEvent, [u8; 32])> =
            contract_events_with_txhash
                .clone()
                .into_iter()
                .filter(|(event, _txhash)| {
                    let contract_id_str = SorobanString::from_str(
                        &env.soroban(),
                        &stellar_strkey::Contract(event.raw.contract_id.as_ref().unwrap().0)
                            .to_string(),
                    );
                    contract_id_str == pair_address.to_string()
                })
                .collect();

        env.log().debug(
            format!(
                "We identified a number of events: {:?}",
                &pair_contract_events.len()
            ),
            None,
        );
        env.log().debug(
            format!("pair_contract_events: {:?}", &pair_contract_events),
            None,
        );

        // Handle the Event
        pairs::events::handle_contract_events(&env, pair_contract_events, pair);
    }

    env.log().debug(format!("After pairs_rows iteration"), None);
}