use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    pub amount_0: i128,
    pub amount_1: i128,
    pub liquidity: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEvent {
    pub amount_0: i128,
    pub amount_1: i128,
    pub liquidity: i128,
}


// use serde::{Deserialize, Serialize};
// use soroban_sdk::{contracttype, Address};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[contracttype]
// pub struct MintEvent {
//     pub liquidity: i128,
//     pub amount_0: i128,
//     pub amount_1: i128,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[contracttype]
// pub struct BurnEvent {
//     pub liquidity: i128,
//     pub amount_0: i128,
//     pub amount_1: i128,
// }

// #[derive(Debug, Clone)]
// pub struct ProvidersTable {
//     pub provider_address: soroban_sdk::xdr::ScVal,
//     pub pool_address: soroban_sdk::xdr::ScVal,
//     pub shares: soroban_sdk::xdr::ScVal,
//     pub share_percentage: f64,
//     pub token_a_amount: soroban_sdk::xdr::ScVal,
//     pub token_b_amount: soroban_sdk::xdr::ScVal,
//     pub timestamp: soroban_sdk::xdr::ScVal,
//     pub action: String,
//     pub tx_hash: soroban_sdk::xdr::ScVal,
// }
