use zephyr_sdk::{
    soroban_sdk::{xdr::ScVal, Address, Symbol},
    PrettyContractEvent, EnvClient,
};
use thiserror::Error;
use crate::ProvidersTable;
use crate::providers::types::{MintEvent, BurnEvent};

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Invalid liquidity value: {0}")]
    InvalidLiquidity(String),
    #[error("Invalid total supply: {0}")]
    InvalidTotalSupply(String),
    #[error("Failed to convert address: {0}")]
    AddressConversionError(String),
    #[error("Failed to process event data: {0}")]
    EventDataError(String),
    #[error("Failed to process action: {0}")]
    ActionProcessingError(String),
}

#[allow(dead_code)]
type Result<T> = std::result::Result<T, ProviderError>;

/// Validates and processes provider data
#[allow(dead_code)]
fn validate_provider_data(liquidity: i128, total_supply: i128) -> Result<f64> {
    if liquidity < 0 {
        return Err(ProviderError::InvalidLiquidity(
            "Liquidity cannot be negative".to_string(),
        ));
    }

    if total_supply <= 0 {
        return Err(ProviderError::InvalidTotalSupply(
            "Total supply must be positive".to_string(),
        ));
    }

    let share_percentage = (liquidity as f64 / total_supply as f64) * 100.0;
    Ok(share_percentage)
}

#[allow(dead_code)]
pub(crate) fn get_provider_from_mint(
    env: &EnvClient,
    data: &ScVal,
    address: ScVal,
    pool_address: ScVal,
    total_supply: i128,
    tx_hash: [u8; 32],
) -> Result<ProvidersTable> {
    let values: Result<MintEvent> = Ok(env.from_scval::<MintEvent>(data));
    let values = match values {
        Ok(v) => v,
        Err(e) => return Err(ProviderError::EventDataError(format!("Failed to deserialize mint event: {}", e))),
    };

    let share_percentage = validate_provider_data(values.liquidity, total_supply)?;

    let provider = ProvidersTable {
        provider_address: address,
        pool_address,
        shares: env.to_scval(values.liquidity),
        share_percentage,
        token_a_amount: env.to_scval(values.amount_0),
        token_b_amount: env.to_scval(values.amount_1),
        timestamp: env.to_scval(env.reader().ledger_timestamp()),
        action: "MINT".to_string(),
        tx_hash: env.to_scval(tx_hash.to_vec()),
    };

    Ok(provider)
}

#[allow(dead_code)]
pub(crate) fn get_provider_from_burn(
    env: &EnvClient,
    data: &ScVal,
    address: ScVal,
    pool_address: ScVal,
    total_supply: i128,
    tx_hash: [u8; 32],
) -> Result<ProvidersTable> {
    let values: Result<BurnEvent> = Ok(env.from_scval::<BurnEvent>(data));
    let values = match values {
        Ok(v) => v,
        Err(e) => return Err(ProviderError::EventDataError(format!("Failed to deserialize burn event: {}", e))),
    };

    let share_percentage = validate_provider_data(values.liquidity, total_supply)?;

    let provider = ProvidersTable {
        provider_address: address,
        pool_address,
        shares: env.to_scval(values.liquidity),
        share_percentage,
        token_a_amount: env.to_scval(values.amount_0),
        token_b_amount: env.to_scval(values.amount_1),
        timestamp: env.to_scval(env.reader().ledger_timestamp()),
        action: "BURN".to_string(),
        tx_hash: env.to_scval(tx_hash.to_vec()),
    };

    Ok(provider)
}

#[allow(dead_code)]
pub(crate) fn handle_contract_events(
    env: &EnvClient,
    contract_events: Vec<(PrettyContractEvent, [u8; 32])>,
    pool_address: ScVal,
    total_supply: i128,
) {
    for (event, tx_hash) in contract_events {
        match process_event(env, event, &pool_address, total_supply, tx_hash) {
            Ok(_) => {
                env.log().debug("Successfully processed event", None);
            }
            Err(e) => {
                env.log().error(
                    format!("Failed to process event: {}", e),
                    Some(format!("{:?}", e).into()),
                );
                continue;
            }
        }
    }
}

#[allow(dead_code)]
fn process_event(
    env: &EnvClient,
    event: PrettyContractEvent,
    pool_address: &ScVal,
    total_supply: i128,
    tx_hash: [u8; 32],
) -> Result<()> {
    let action = env
        .try_from_scval::<Symbol>(&event.topics[1])
        .map_err(|e| ProviderError::ActionProcessingError(format!("Invalid action: {}", e)))?;

    let action_str = action.to_string(); // Convert Symbol to String

    let provider_address = match event.topics.get(2) {
        Some(address_scval) => env
            .try_from_scval::<Address>(address_scval)
            .map_err(|e| ProviderError::AddressConversionError(format!("Invalid address: {}", e)))?,
        None => {
            return Err(ProviderError::AddressConversionError(
                "Missing provider address".to_string(),
            ))
        }
    };

    let data = &event.data;
    let provider_scval = env.to_scval(provider_address);

    match action_str.as_str() {
        "mint" => {
            let provider_table = get_provider_from_mint(
                env,
                data,
                provider_scval,
                pool_address.clone(),
                total_supply,
                tx_hash,
            )?;
            provider_table.put(env);

            env.log().debug("Provider mint event recorded", None);
        }
        "burn" => {
            let provider_table = get_provider_from_burn(
                env,
                data,
                provider_scval,
                pool_address.clone(),
                total_supply,
                tx_hash,
            )?;
            provider_table.put(env);
            env.log().debug("Provider burn event recorded", None);
        }
        _ => {
            env.log().debug(format!("Unhandled action: {:?}", action), None);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use zephyr_sdk::EnvClient;
    use soroban_sdk::{xdr::ScVal, Address, Bytes};

    #[test]
    fn test_get_provider_from_mint() {
        let env = EnvClient::new();
    
        // Manually create the MintEvent data as a ScVal
        let data = env.to_scval(MintEvent {
            liquidity: 100,
            amount_0: 50,
            amount_1: 50,
        });
    
        // Use from_string with a reference to a String
        let address_str = String::from("GAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        let address = Address::from_string(&address_str);
        let pool_address = Address::from_string(&address_str);
    
        // Convert addresses to ScVal
        let address_scval = env.to_scval(address);
        let pool_address_scval = env.to_scval(pool_address);
    
        let total_supply = 1000;
        let tx_hash: [u8; 32] = [0; 32];

        // Call the function being tested
        let result = get_provider_from_mint(
            &env, 
            &data, 
            address_scval, 
            pool_address_scval, 
            total_supply, 
            tx_hash
        );

        // Assertions
        assert!(result.is_ok());

        let provider = result.unwrap();
        assert_eq!(provider.provider_address, address_scval);
        assert_eq!(provider.pool_address, pool_address_scval);
        assert_eq!(provider.shares, env.to_scval(100));
        assert_eq!(provider.share_percentage, 10.0);
        assert_eq!(provider.token_a_amount, env.to_scval(50));
        assert_eq!(provider.token_b_amount, env.to_scval(50));
        assert_eq!(provider.action, "MINT");
        assert_eq!(provider.tx_hash, env.to_scval(tx_hash.to_vec()));
    }

    #[test]
    fn test_get_provider_from_burn() {
        let env = EnvClient::new();
        let data = ScVal::from(r#"{"liquidity": 100, "amount_0": 50, "amount_1": 50}"#).unwrap();
        let address = env.to_scval(Address::from_string(String::from("GAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")).unwrap());
        let pool_address = env.to_scval(Address::from_string(String::from("GAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")).unwrap());
        let total_supply = 1000;
        let tx_hash: [u8; 32] = [0; 32];

        let result = get_provider_from_burn(&env, &data, address, pool_address, total_supply, tx_hash);
        assert!(result.is_ok());

        let provider = result.unwrap();
        assert_eq!(provider.provider_address, address);
        assert_eq!(provider.pool_address, pool_address);
        assert_eq!(provider.shares, env.to_scval(100));
        assert_eq!(provider.share_percentage, 10.0);
        assert_eq!(provider.token_a_amount, env.to_scval(50));
        assert_eq!(provider.token_b_amount, env.to_scval(50));
        assert_eq!(provider.timestamp, env.to_scval(env.reader().ledger_timestamp()));
        assert_eq!(provider.action, "BURN");
        assert_eq!(provider.tx_hash, env.to_scval(tx_hash.to_vec()));
    }
}