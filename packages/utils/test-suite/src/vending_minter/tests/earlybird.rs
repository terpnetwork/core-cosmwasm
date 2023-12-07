use cosmwasm_std::{coin, coins, Addr, Empty, Timestamp};
use cw721::{Cw721QueryMsg, TokensResponse};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_multi_test::Executor;
use factory_utils::tests::mock_collection_params_1;
use terp_sdk::{GENESIS_MINT_START_TIME, NATIVE_DENOM};

use crate::common_setup::msg::MinterCollectionResponse;
use crate::common_setup::setup_accounts_and_block::coins_for_msg;
use crate::common_setup::setup_collection_earlybird::{
    configure_collection_earlybird, setup_earlybird_contract,
};
use crate::common_setup::setup_minter::common::minter_params::{
    minter_params_all, minter_params_token,
};
use crate::common_setup::setup_minter::vending_minter::setup::{
    configure_minter, vending_minter_code_ids,
};
use crate::common_setup::templates::vending_minter_template;
use crate::common_setup::{
    contract_boxes::custom_mock_app,
    setup_accounts_and_block::{setup_accounts, setup_block_time},
};
use vending_minter::msg::{ExecuteMsg, MintCountResponse, MintPriceResponse, QueryMsg};
use vending_minter::ContractError;

use earlybird::msg::{
    AddMembersMsg, ConfigResponse as EarlybirdConfigResponse, ExecuteMsg as EarlybirdExecuteMsg,
    QueryMsg as EarlybirdQueryMsg,
};

const MINT_PRICE: u128 = 100_000_000;
const WHITELIST_AMOUNT: u128 = 66_000_000;
pub const MIN_MINT_PRICE: u128 = 50_000_000;

#[test]
fn invalid_earlybird_instantiate() {
    let mut router = custom_mock_app();
    let (creator, _) = setup_accounts(&mut router);

    let num_tokens = 10;
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let init_msg = vending_factory::msg::VendingMinterInitMsgExtension {
        base_token_uri: "ipfs://aldkfjads".to_string(),
        payment_address: None,
        start_time: Timestamp::from_nanos(GENESIS_MINT_START_TIME),
        num_tokens: 100,
        mint_price: coin(MIN_MINT_PRICE, NATIVE_DENOM),
        per_address_limit: 3,
        earlybird: Some("invalid address".to_string()),
    };

    let minter_params = minter_params_all(num_tokens, None, None, Some(init_msg));
    let code_ids = vending_minter_code_ids(&mut router);
    let minter_collection_response = configure_minter(
        &mut router,
        creator,
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    let err = minter_collection_response[0].error.as_ref();

    assert_eq!(
        err.unwrap().source().unwrap().source().unwrap().to_string(),
        "Generic error: Querier contract error: cw_multi_test::wasm::ContractData not found"
    )
}

#[test]
fn set_invalid_earlybird() {
    let vt = vending_minter_template(10);
    let (mut router, creator, _) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let minter_addr = vt.collection_response_vec[0].minter.clone().unwrap();
    let earlybird_addr = setup_earlybird_contract(&mut router, &creator, None, None);
    const EXPIRATION_TIME: Timestamp = Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000);

    // Set block to before genesis mint start time
    setup_block_time(&mut router, GENESIS_MINT_START_TIME - 1000, None);

    // Update to a start time after genesis
    let msg = ExecuteMsg::UpdateStartTime(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 1000));
    router
        .execute_contract(creator.clone(), minter_addr.clone(), &msg, &[])
        .unwrap();

    // update wl times
    const WL_START: Timestamp = Timestamp::from_nanos(GENESIS_MINT_START_TIME + 200);

    let wl_msg = EarlybirdExecuteMsg::UpdateStartTime(WL_START);
    let res = router.execute_contract(creator.clone(), earlybird_addr.clone(), &wl_msg, &[]);
    assert!(res.is_ok());
    let wl_msg = EarlybirdExecuteMsg::UpdateEndTime(EXPIRATION_TIME);
    let res = router.execute_contract(creator.clone(), earlybird_addr.clone(), &wl_msg, &[]);
    assert!(res.is_ok());

    // Set earlybird in minter contract
    let set_earlybird_msg = ExecuteMsg::SetEarlybird {
        earlybird: "invalid".to_string(),
    };
    let err = router
        .execute_contract(
            creator.clone(),
            minter_addr.clone(),
            &set_earlybird_msg,
            &[],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().source().unwrap().to_string(),
        "Generic error: Querier contract error: cw_multi_test::wasm::ContractData not found"
    );

    // move time to make wl start
    setup_block_time(&mut router, GENESIS_MINT_START_TIME + 201, Some(11));

    // check that the new earlybird exists
    let wl_res: EarlybirdConfigResponse = router
        .wrap()
        .query_wasm_smart(earlybird_addr.to_string(), &EarlybirdQueryMsg::Config {})
        .unwrap();

    assert!(wl_res.is_active);

    // Set earlybird in minter contract
    let set_earlybird_msg = ExecuteMsg::SetEarlybird {
        earlybird: earlybird_addr.to_string(),
    };

    let err = router
        .execute_contract(creator.clone(), minter_addr, &set_earlybird_msg, &[])
        .unwrap_err();
    assert_eq!(err.source().unwrap().to_string(), "EarlybirdAlreadyStarted");
}

#[test]
fn earlybird_mint_count_query() {
    let mut router = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut router);
    let num_tokens = 10;
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = vending_minter_code_ids(&mut router);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();
    let collection_addr = minter_collection_response[0].collection.clone().unwrap();

    let earlybird_addr = setup_earlybird_contract(&mut router, &creator, None, None);
    const EXPIRATION_TIME: Timestamp = Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000);

    // Set block to before genesis mint start time
    setup_block_time(&mut router, GENESIS_MINT_START_TIME - 1000, None);

    let wl_msg = EarlybirdExecuteMsg::UpdateEndTime(EXPIRATION_TIME);
    let res = router.execute_contract(creator.clone(), earlybird_addr.clone(), &wl_msg, &[]);
    assert!(res.is_ok());

    let wl_msg = EarlybirdExecuteMsg::UpdateStartTime(Timestamp::from_nanos(0));
    let res = router.execute_contract(creator.clone(), earlybird_addr.clone(), &wl_msg, &[]);
    assert!(res.is_ok());

    // Set earlybird in minter contract
    let set_earlybird_msg = ExecuteMsg::SetEarlybird {
        earlybird: earlybird_addr.to_string(),
    };
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &set_earlybird_msg,
        &[],
    );
    assert!(res.is_ok());

    // Update per address_limit
    let set_earlybird_msg = ExecuteMsg::UpdatePerAddressLimit {
        per_address_limit: 3,
    };
    let res = router.execute_contract(
        creator.clone(),
        minter_addr.clone(),
        &set_earlybird_msg,
        &[],
    );
    assert!(res.is_ok());

    // Add buyer to earlybird
    let inner_msg = AddMembersMsg {
        to_add: vec![buyer.to_string()],
    };
    let wasm_msg = EarlybirdExecuteMsg::AddMembers(inner_msg);
    let res = router.execute_contract(creator.clone(), earlybird_addr, &wasm_msg, &[]);
    assert!(res.is_ok());

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, Some(10));

    // Mint succeeds
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer.clone(),
        minter_addr.clone(),
        &mint_msg,
        &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Query count
    let res: MintCountResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr.clone(),
            &QueryMsg::MintCount {
                address: buyer.to_string(),
            },
        )
        .unwrap();
    assert_eq!(res.count, 1);
    assert_eq!(res.address, buyer.to_string());

    // Mint fails, over earlybird per address limit
    let mint_msg = ExecuteMsg::Mint {};
    let err = router
        .execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::MaxPerAddressLimitExceeded {}.to_string()
    );

    // Set time after wl ends
    setup_block_time(&mut router, GENESIS_MINT_START_TIME + 20_000, Some(11));

    // Public mint succeeds
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer.clone(),
        minter_addr.clone(),
        &mint_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Query count
    let res: MintCountResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr.clone(),
            &QueryMsg::MintCount {
                address: buyer.to_string(),
            },
        )
        .unwrap();
    assert_eq!(res.count, 2);
    assert_eq!(res.address, buyer.to_string());

    // get random mint token_id
    let tokens_msg = Cw721QueryMsg::Tokens {
        owner: buyer.to_string(),
        start_after: None,
        limit: None,
    };
    let res: TokensResponse = router
        .wrap()
        .query_wasm_smart(collection_addr.clone(), &tokens_msg)
        .unwrap();
    let sold_token_id: u32 = res.tokens[1].parse::<u32>().unwrap();
    // Buyer transfers NFT to creator
    // random mint token id: 8
    let transfer_msg: Cw721ExecuteMsg<Empty, Empty> = Cw721ExecuteMsg::TransferNft {
        recipient: creator.to_string(),
        // token_id: "8".to_string(),
        token_id: sold_token_id.to_string(),
    };
    let res = router.execute_contract(
        buyer.clone(),
        collection_addr,
        &transfer_msg,
        &coins_for_msg(coin(123, NATIVE_DENOM)),
    );
    assert!(res.is_ok());

    // Mint succeeds
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer.clone(),
        minter_addr.clone(),
        &mint_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Query count
    let res: MintCountResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr.clone(),
            &QueryMsg::MintCount {
                address: buyer.to_string(),
            },
        )
        .unwrap();
    assert_eq!(res.count, 3);
    assert_eq!(res.address, buyer.to_string());

    // Mint fails
    let mint_msg = ExecuteMsg::Mint {};
    let err = router
        .execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::MaxPerAddressLimitExceeded {}.to_string()
    );

    // Query count
    let res: MintCountResponse = router
        .wrap()
        .query_wasm_smart(
            minter_addr,
            &QueryMsg::MintCount {
                address: buyer.to_string(),
            },
        )
        .unwrap();
    assert_eq!(res.count, 3);
    assert_eq!(res.address, buyer.to_string());
}

#[test]
fn earlybird_already_started() {
    let mut router = custom_mock_app();
    let (creator, _) = setup_accounts(&mut router);
    let num_tokens = 1;
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = vending_minter_code_ids(&mut router);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();
    let earlybird_addr = setup_earlybird_contract(&mut router, &creator, None, None);

    setup_block_time(&mut router, GENESIS_MINT_START_TIME + 101, None);

    // set earlybird in minter contract
    let set_earlybird_msg = ExecuteMsg::SetEarlybird {
        earlybird: earlybird_addr.to_string(),
    };
    router
        .execute_contract(
            creator.clone(),
            minter_addr,
            &set_earlybird_msg,
            &coins(MINT_PRICE, NATIVE_DENOM),
        )
        .unwrap_err();
}

#[test]
fn earlybird_can_update_before_start() {
    let mut router = custom_mock_app();
    let (creator, _) = setup_accounts(&mut router);
    let num_tokens = 1;
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = vending_minter_code_ids(&mut router);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();
    let earlybird_addr = setup_earlybird_contract(&mut router, &creator, None, None);

    setup_block_time(&mut router, GENESIS_MINT_START_TIME - 1000, None);

    // set earlybird in minter contract
    let set_earlybird_msg = ExecuteMsg::SetEarlybird {
        earlybird: earlybird_addr.to_string(),
    };
    router
        .execute_contract(
            creator.clone(),
            minter_addr.clone(),
            &set_earlybird_msg,
            &[],
        )
        .unwrap();

    // can set twice before starting
    router
        .execute_contract(creator.clone(), minter_addr, &set_earlybird_msg, &[])
        .unwrap();
}

#[test]
fn earlybird_access_len_add_remove_expiration() {
    let mut router = custom_mock_app();
    let (creator, buyer) = setup_accounts(&mut router);
    let num_tokens = 1;
    let start_time = Timestamp::from_nanos(GENESIS_MINT_START_TIME);
    let collection_params = mock_collection_params_1(Some(start_time));
    let minter_params = minter_params_token(num_tokens);
    let code_ids = vending_minter_code_ids(&mut router);
    let minter_collection_response: Vec<MinterCollectionResponse> = configure_minter(
        &mut router,
        creator.clone(),
        vec![collection_params],
        vec![minter_params],
        code_ids,
    );
    let minter_addr = minter_collection_response[0].minter.clone().unwrap();
    let terp721_addr = minter_collection_response[0].collection.clone().unwrap();

    let earlybird_addr = configure_collection_earlybird(
        &mut router,
        creator.clone(),
        buyer.clone(),
        minter_addr.clone(),
    );

    // Mint fails, buyer is not on earlybird
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer.clone(),
        minter_addr.clone(),
        &mint_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert!(res.is_err());

    // Add buyer to earlybird
    let inner_msg = AddMembersMsg {
        to_add: vec![buyer.to_string()],
    };
    let wasm_msg = EarlybirdExecuteMsg::AddMembers(inner_msg);
    let res = router.execute_contract(creator.clone(), earlybird_addr.clone(), &wasm_msg, &[]);
    assert!(res.is_ok());

    // Mint fails, not earlybird price
    let mint_msg = ExecuteMsg::Mint {};
    router
        .execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(MINT_PRICE, NATIVE_DENOM),
        )
        .unwrap_err();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);

    // Query mint price
    let mint_price_response: MintPriceResponse = router
        .wrap()
        .query_wasm_smart(minter_addr.clone(), &QueryMsg::MintPrice {})
        .unwrap();

    assert_eq!(
        coin(WHITELIST_AMOUNT, NATIVE_DENOM),
        mint_price_response.earlybird_price.unwrap()
    );
    assert_eq!(
        coin(WHITELIST_AMOUNT, NATIVE_DENOM),
        mint_price_response.current_price
    );
    assert_eq!(
        coin(MINT_PRICE, NATIVE_DENOM),
        mint_price_response.public_price
    );

    // Mint succeeds with earlybird price
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer.clone(),
        minter_addr.clone(),
        &mint_msg,
        &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
    );
    assert!(res.is_ok());

    // Mint fails, over earlybird per address limit
    let mint_msg = ExecuteMsg::Mint {};
    let err = router
        .execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::MaxPerAddressLimitExceeded {}.to_string()
    );

    // Muyer is generous and transfers to creator
    let transfer_msg: Cw721ExecuteMsg<Empty, Empty> = Cw721ExecuteMsg::TransferNft {
        recipient: creator.to_string(),
        token_id: "1".to_string(),
    };
    let res = router.execute_contract(
        buyer.clone(),
        Addr::unchecked(terp721_addr),
        &transfer_msg,
        &coins_for_msg(coin(123, NATIVE_DENOM)),
    );
    assert!(res.is_ok());

    // Mint fails, buyer exceeded per address limit
    let mint_msg = ExecuteMsg::Mint {};
    let err = router
        .execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(WHITELIST_AMOUNT, NATIVE_DENOM),
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::MaxPerAddressLimitExceeded {}.to_string()
    );

    // Remove buyer from earlybird
    let inner_msg = AddMembersMsg { to_add: vec![] };
    let wasm_msg = EarlybirdExecuteMsg::AddMembers(inner_msg);
    let res = router.execute_contract(creator, earlybird_addr, &wasm_msg, &[]);
    assert!(res.is_ok());

    // Mint fails
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        buyer,
        minter_addr,
        &mint_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert!(res.is_err());
}
