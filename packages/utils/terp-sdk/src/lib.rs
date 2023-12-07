mod msg;
mod query;
mod route;


pub const NATIVE_DENOM: &str = "uterp";

pub const NATIVE_BOND_DENOM: &str = "uterp";
pub const NATIVE_FEE_DENOM: &str = "uthiol";

pub const TEST_BOND_DENOM: &str = "uterpx";
pub const TEST_FEE_DENOM: &str = "uthiolx";

pub const GENESIS_MINT_START_TIME: u64 = 1647032400000000000;


use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin};
pub use msg::{
     create_fund_community_pool_msg,
     TerpMsg, TerpMsgWrapper,
};

pub type Response = cosmwasm_std::Response<TerpMsgWrapper>;
pub type SubMsg = cosmwasm_std::SubMsg<TerpMsgWrapper>;
pub type CosmosMsg = cosmwasm_std::CosmosMsg<TerpMsgWrapper>;

pub use query::TerpQuery;
pub use route::TerpRoute;

// This export is added to all contracts that import this package, signifying that they require
// "terpnet" support on the chain they run on.

// #[no_mangle]
// extern "C" fn requires_terpnetwork() {}

pub fn terps(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), NATIVE_BOND_DENOM)
}
pub fn thiols(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), NATIVE_FEE_DENOM)
}

pub fn test_terps(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), TEST_BOND_DENOM)
}
pub fn test_thiols(amount: impl Into<u128>) -> Vec<Coin> {
    coins(amount.into(), TEST_FEE_DENOM)
}

pub fn terp(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_BOND_DENOM)
}

pub fn thiol(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_FEE_DENOM)
}

pub fn test_terp(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_BOND_DENOM)
}

pub fn test_thiol(amount: impl Into<u128>) -> Coin {
    coin(amount.into(), NATIVE_FEE_DENOM)
}

pub fn send_terps_msg(to_address: &Addr, amount: impl Into<u128>) -> BankMsg {
    BankMsg::Send {
        to_address: to_address.to_string(),
        amount: terps(amount),
    }
}

pub fn send_thiols_msg(to_address: &Addr, amount: impl Into<u128>) -> BankMsg {
    BankMsg::Send {
        to_address: to_address.to_string(),
        amount: thiols(amount),
    }
}
