use cw_multi_test::{Contract, ContractWrapper};
use terp_multi_test::TerpApp;
use terp_sdk::TerpMsgWrapper;

pub fn custom_mock_app() -> TerpApp {
    TerpApp::default()
}

pub fn contract_vending_factory() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        vending_factory::contract::execute,
        vending_factory::contract::instantiate,
        vending_factory::contract::query,
    )
    .with_sudo(vending_factory::contract::sudo);
    Box::new(contract)
}

pub fn contract_open_edition_factory() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        open_edition_factory::contract::execute,
        open_edition_factory::contract::instantiate,
        open_edition_factory::contract::query,
    )
    .with_sudo(open_edition_factory::contract::sudo);
    Box::new(contract)
}

pub fn contract_base_factory() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        base_factory::contract::execute,
        base_factory::contract::instantiate,
        base_factory::contract::query,
    )
    .with_sudo(base_factory::contract::sudo);
    Box::new(contract)
}

pub fn contract_base_minter() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        base_minter::contract::execute,
        base_minter::contract::instantiate,
        base_minter::contract::query,
    )
    .with_reply(base_minter::contract::reply);
    Box::new(contract)
}

pub fn contract_nt_collection() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        terp721_nt::entry::execute,
        terp721_nt::entry::instantiate,
        terp721_nt::entry::query,
    );
    Box::new(contract)
}

pub fn contract_collection_earlybird() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        earlybird::contract::execute,
        earlybird::contract::instantiate,
        earlybird::contract::query,
    );
    Box::new(contract)
}

pub fn contract_open_edition_minter() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        open_edition_minter::contract::execute,
        open_edition_minter::contract::instantiate,
        open_edition_minter::contract::query,
    )
    .with_reply(open_edition_minter::contract::reply);
    Box::new(contract)
}

pub fn contract_vending_minter() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        vending_minter::contract::execute,
        vending_minter::contract::instantiate,
        vending_minter::contract::query,
    )
    .with_reply(vending_minter::contract::reply);
    Box::new(contract)
}

pub fn contract_terp721_base() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        terp721_base::entry::execute,
        terp721_base::entry::instantiate,
        terp721_base::entry::query,
    );
    Box::new(contract)
}

pub fn contract_terp721_updatable() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        terp721_base::entry::execute,
        terp721_base::entry::instantiate,
        terp721_base::entry::query,
    )
    .with_migrate(terp721_updatable::entry::migrate);
    Box::new(contract)
}

pub fn contract_splits() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new_with_empty(
        terp_splits::contract::execute,
        terp_splits::contract::instantiate,
        terp_splits::contract::query,
    );
    Box::new(contract)
}

pub fn contract_group() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new_with_empty(
        cw4_group::contract::execute,
        cw4_group::contract::instantiate,
        cw4_group::contract::query,
    );
    Box::new(contract)
}

// pub fn contract_eth_airdrop() -> Box<dyn Contract<TerpMsgWrapper>> {
//     let contract = ContractWrapper::new(
//         headstash_airdrop::contract::execute,
//         headstash_airdrop::contract::instantiate,
//         headstash_airdrop::query::query,
//     )
//     .with_reply(headstash_airdrop::reply::reply);
//     Box::new(contract)
// }

pub fn contract_earlybird_immutable() -> Box<dyn Contract<TerpMsgWrapper>> {
    let contract = ContractWrapper::new(
        earlybird_immutable::contract::execute,
        earlybird_immutable::contract::instantiate,
        earlybird_immutable::contract::query,
    );
    Box::new(contract)
}
