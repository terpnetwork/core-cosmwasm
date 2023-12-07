use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub fn contract_group() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw4_group::contract::execute,
        cw4_group::contract::instantiate,
        cw4_group::contract::query,
    );
    Box::new(contract)
}

pub fn contract_splits() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        terp_splits::contract::execute,
        terp_splits::contract::instantiate,
        terp_splits::contract::query,
    )
    .with_reply(terp_splits::contract::reply);
    Box::new(contract)
}
