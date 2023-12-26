use crate::common_setup::setup_accounts_and_block::setup_block_time;
use crate::common_setup::setup_collection_earlybird::configure_collection_earlybird;
use crate::headstash_airdrop::constants::claim_constants::{CONFIG_PLAINTEXT, TERP_WALLET_01};
use crate::headstash_airdrop::constants::collection_constants::{
    AIRDROP_ADDR_STR, MINT_PRICE, WHITELIST_AMOUNT,
};
use crate::headstash_airdrop::setup::collection_earlybird_helpers::{
    execute_airdrop_claim, execute_mint_fail_not_on_earlybird, execute_mint_success,
    send_funds_to_address, update_admin_for_earlybird,
};
use crate::headstash_airdrop::setup::execute_msg::instantiate_contract;
use crate::headstash_airdrop::setup::setup_signatures::{get_msg_plaintext, get_wallet_and_sig};
use crate::headstash_airdrop::setup::test_msgs::InstantiateParams;
use cosmwasm_std::Addr;
use headstash_airdrop::msg::QueryMsg;
use terp_sdk::GENESIS_MINT_START_TIME;
extern crate earlybird_immutable;
use crate::common_setup::templates::vending_minter_template;
use headstash_airdrop::contract::INSTANTIATION_FEE;

#[test]
fn test_set_minter_contract_success() {
    let vt = vending_minter_template(1);
    let (mut app, creator) = (vt.router, vt.accts.creator);
    let minter_addr = vt.collection_response_vec[0].minter.clone().unwrap();

    let claim_plaintext = &get_msg_plaintext(TERP_WALLET_01.to_string());
    let (_, _, _, eth_addr_str) = get_wallet_and_sig(claim_plaintext.clone());

    let contract_admin = Addr::unchecked(creator);
    let params = InstantiateParams {
        addresses: vec![eth_addr_str],
        funds_amount: WHITELIST_AMOUNT + INSTANTIATION_FEE,
        expected_airdrop_contract_id: 4,
        minter_address: minter_addr.clone(),
        admin_account: contract_admin,
        app: &mut app,
        per_address_limit: 1,
        claim_msg_plaintext: CONFIG_PLAINTEXT.to_string(),
    };
    instantiate_contract(params).unwrap();
    let airdrop_contract = Addr::unchecked("contract3");
    let query_msg = QueryMsg::GetMinter {};
    let result: Addr = app
        .wrap()
        .query_wasm_smart(airdrop_contract, &query_msg)
        .unwrap();
    assert_eq!(result, minter_addr);
}

#[test]
fn test_claim_added_to_minter_earlybird() {
    let vt = vending_minter_template(1);
    let (mut app, creator, buyer) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let minter_addr = vt.collection_response_vec[0].minter.clone().unwrap();
    let earlybird_addr =
        configure_collection_earlybird(&mut app, creator.clone(), buyer, minter_addr.clone());
    setup_block_time(&mut app, GENESIS_MINT_START_TIME, None);
    let claim_plaintext = &get_msg_plaintext(TERP_WALLET_01.to_string());
    let (_, eth_sig_str, _, eth_addr_str) = get_wallet_and_sig(claim_plaintext.clone());

    let airdrop_contract = Addr::unchecked(AIRDROP_ADDR_STR);
    let params = InstantiateParams {
        addresses: vec![eth_addr_str.clone()],
        funds_amount: WHITELIST_AMOUNT + INSTANTIATION_FEE,
        expected_airdrop_contract_id: 5,
        minter_address: minter_addr.clone(),
        admin_account: creator.clone(),
        app: &mut app,
        per_address_limit: 1,
        claim_msg_plaintext: CONFIG_PLAINTEXT.to_string(),
    };
    instantiate_contract(params).unwrap();

    let terp_wallet_01 = Addr::unchecked(TERP_WALLET_01);
    update_admin_for_earlybird(&mut app, creator, airdrop_contract.clone(), earlybird_addr);
    send_funds_to_address(&mut app, TERP_WALLET_01, MINT_PRICE);
    execute_mint_fail_not_on_earlybird(&mut app, minter_addr.clone());
    execute_airdrop_claim(
        &mut app,
        eth_addr_str,
        eth_sig_str,
        terp_wallet_01.clone(),
        airdrop_contract,
    );
    execute_mint_success(&mut app, terp_wallet_01, minter_addr);
}
