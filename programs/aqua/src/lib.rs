use zephyr_sdk::{
    prelude::*, 
    soroban_sdk::{xdr::{LedgerEntryData, ScVal, ScString}, Symbol, Address, Vec as SorobanVec, String as SorobanString},  
    EnvClient, DatabaseDerive, PrettyContractEvent
};

fn soroban_string_to_string(env: &EnvClient, string: SorobanString) -> String {
    let sc_val: ScVal = env.to_scval(string);
    if let ScVal::String(ScString(s)) = sc_val {
        s.to_utf8_string().unwrap()
    } else {
        panic!("value is not a string");
    }
}

// https://amm-api.aqua.network/pools/?format=json
// CBQDHNBFBZYE4MKPWBSJOPIYLW4SFSXAXUTSXJN76GNKYVYPCKWC6QUK
// pub(crate) const AQUA_LP_ROUTER_CONTRACT_ADDRESS: [u8; 32] = [96, 51, 180, 37, 14, 112, 78, 49, 79, 176, 100, 151, 61, 24, 93, 185, 34, 202, 224, 189, 39, 43, 165, 191, 241, 154, 172, 87, 15, 18, 172, 47];
// const AQUA_LP_ROUTER_CONTRACT_ADDRESS: &'static str = "CBQDHNBFBZYE4MKPWBSJOPIYLW4SFSXAXUTSXJN76GNKYVYPCKWC6QUK";

#[derive(DatabaseDerive, Clone, Debug)]
#[with_name("aqa_pairs")]
struct PairsTable {
    address: ScVal,
    token_a: ScVal,
    token_b: ScVal,
    reserve_a: ScVal,
    reserve_b: ScVal,
    fee: ScVal
}

#[no_mangle]
pub extern "C" fn on_close() {
    let env = EnvClient::new();
    let contract_address_str: &'static str = env!("AQUA_ROUTER");
    env.log().debug(format!("Executing Zephyr Program. Indexing Aqua Router Contract: {:?}", &contract_address_str), None);

    let lp_router_contract = stellar_strkey::Contract::from_string(&contract_address_str).unwrap().0;

    let rows = env.read::<PairsTable>();

    let lp_router_contract_events: Vec<PrettyContractEvent> = {
        let events = env.reader().pretty().soroban_events();
        events
            .iter()
            .filter_map(|x| {
                if x.contract == lp_router_contract {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect()
    };

    for t_event in lp_router_contract_events {
        let action: Symbol = env.from_scval(&t_event.topics[0]);
        let tokens: SorobanVec<Address> = env.from_scval(&t_event.topics[1]);

        if action == Symbol::new(env.soroban(), "deposit") {
            env.log().debug("Deposit Event", None);
                        
            handle_add(&env, &t_event, 3, &rows, &tokens)

        } else if action == Symbol::new(env.soroban(), "swap") {
            env.log().debug("Swap Event", None);
            
            handle_add(&env, &t_event, 5, &rows, &tokens)

        } else if action == Symbol::new(env.soroban(), "withdraw") {
            env.log().debug("Withdraw Event", None);

            handle_add(&env, &t_event, 3, &rows, &tokens)

        } else if action == Symbol::new(env.soroban(), "add_pool") {
            env.log().debug("Add Pool Event", None);
            
            if let ScVal::Vec(Some(data_vec)) = &t_event.data {

                if data_vec.len() == 4 {
                    let pool_id = data_vec[0].clone();
        
                    let table = PairsTable {
                        address: pool_id,
                        reserve_a: env.to_scval(0),
                        reserve_b: env.to_scval(0),
                        token_a: env.to_scval(tokens.first()),
                        token_b: env.to_scval(tokens.last()),
                        fee: env.to_scval(0)
                    };
        
                    env.log().debug(format!("New action Table: {:?}", table), None);
        
                    if rows.iter().any(|row| row.address == table.address) {
                        env.update().column_equal_to_xdr("address", &table.address).execute(&table).ok();
                    } else {
                        table.put(&env);
                    }
                } else {
                    env.log().debug("Unexpected data structure", None);
                }
            } else {
                env.log().debug("Event data is not a Vec", None);
            }
        }
    } 
}      

pub(crate) fn handle_add(env: &EnvClient, event: &PrettyContractEvent, data_lenght: usize, rows: &Vec<PairsTable>, tokens: &SorobanVec<Address>) {
    if let ScVal::Vec(Some(data_vec)) = &event.data {

        if data_vec.len() == data_lenght {
            let pool_id = data_vec[0].clone();
            let pool_address: Address = env.from_scval(&pool_id);
            let lp_router_contract = stellar_strkey::Contract::from_string(&soroban_string_to_string(env, pool_address.to_string())).unwrap().0;

            let instance_storage = env.read_contract_instance(lp_router_contract).unwrap();
            
            let mut reserve_a: u128 = 0;
            let mut reserve_b: u128 = 0;
            let mut fee_fraction: u32 = 0;

            if let Some(instance) = instance_storage {
                if let LedgerEntryData::ContractData(contract_data_entry) = &instance.entry.data {
                    if let ScVal::ContractInstance(contract_instance) = &contract_data_entry.val {
                        if let Some(storage_map) = &contract_instance.storage {

                            for entry in storage_map.iter() {
                                if let ScVal::Vec(Some(vec)) = &entry.key {
                                    if let Some(ScVal::Symbol(symbol)) = vec.first() {

                                        if symbol.to_string() == "ReserveA" {
                                            if let ScVal::U128(parts) = &entry.val {
                                                env.log().debug(format!("ReseveA: {:?}", entry), None);
                                                reserve_a = parts.lo.into()
                                            }
                                        } else if symbol.to_string() == "ReserveB" {
                                            if let ScVal::U128(parts) = &entry.val {
                                                env.log().debug(format!("ReseveB: {:?}", entry), None);
                                                reserve_b = parts.lo.into()
                                            }
                                        } else if symbol.to_string() == "FeeFraction" {
                                            if let ScVal::U32(fee) = &entry.val {
                                                env.log().debug(format!("FeeFraction: {:?}", entry), None);
                                                fee_fraction = fee.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                env.log().debug("No instance storage", None);
            }
            

            let table = PairsTable {
                address: pool_id,
                reserve_a: env.to_scval(reserve_a),
                reserve_b: env.to_scval(reserve_b),
                token_a: env.to_scval(tokens.first()),
                token_b: env.to_scval(tokens.last()),
                fee: env.to_scval(fee_fraction)
            };

            env.log().debug(format!("New action Table: {:?}", table), None);

            if rows.iter().any(|row| row.address == table.address) {
                env.update().column_equal_to_xdr("address", &table.address).execute(&table).ok();
            } else {
                table.put(&env);
            }


        } else {
            env.log().debug("Unexpected data structure", None);
        }
    } else {
        env.log().debug("Event data is not a Vec", None);
    }
}