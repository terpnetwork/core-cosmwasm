use crate::setup::setup_contracts::setup_residual_registry;
use crate::setup::setup_dummy_contract::{setup_dummy_contract, TestExecuteMsg};
use crate::setup::setup_minter::standard_minter_template;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use terp_residual_registry::msg::QueryMsg;
use terp_residual_registry::msg::ResidualPaymentResponse;

#[test]
fn try_fetch_or_set_residuals() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let dummy_contract = setup_dummy_contract(&mut router, creator);
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let _protocol = Addr::unchecked("protocol");

    // Assert there is no default residual entry for a collection to start
    let residual_payment_response: ResidualPaymentResponse = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::ResidualPayment {
                collection: collection.to_string(),
                protocol: None,
            },
        )
        .unwrap();
    assert!(residual_payment_response.residual_default.is_none());
    assert!(residual_payment_response.residual_protocol.is_none());

    // Invoke fetch_or_set_residuals with no protocol address, should set default residuals
    let msg = TestExecuteMsg::TestFetchOrSetRoyalties {
        residual_registry: residual_registry.to_string(),
        collection: collection.to_string(),
        protocol: None,
    };
    let response = router.execute_contract(bidder, dummy_contract, &msg, &[]);
    assert!(response.is_ok());

    let residual_payment_response: ResidualPaymentResponse = router
        .wrap()
        .query_wasm_smart(
            residual_registry,
            &QueryMsg::ResidualPayment {
                collection: collection.to_string(),
                protocol: None,
            },
        )
        .unwrap();
    assert!(residual_payment_response.residual_default.is_some());
    assert!(residual_payment_response.residual_protocol.is_none());
}
