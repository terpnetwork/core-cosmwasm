use crate::helpers::utils::assert_error;
use crate::setup::setup_accounts::setup_accounts;
use crate::setup::setup_contracts::{contract_residual_registry, setup_residual_registry};
use crate::setup::setup_minter::standard_minter_template;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;
use terp_multi_test::TerpApp;
use terp_sdk::GENESIS_MINT_START_TIME;
use terp_residual_registry::state::{ResidualDefault, ResidualEntry, ResidualProtocol};
use terp_residual_registry::ContractError;
use terp_residual_registry::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::Config,
};
use test_suite::common_setup::setup_accounts_and_block::setup_block_time;

#[test]
fn try_instantiate() {
    let mut app = TerpApp::default();
    let residual_registry_id = app.store_code(contract_residual_registry());
    let (_owner, _bidder, creator) = setup_accounts(&mut app).unwrap();

    let update_wait_period = 6;
    let max_share_delta = Decimal::percent(1);

    let msg = InstantiateMsg {
        config: Config {
            update_wait_period,
            max_share_delta,
        },
    };

    let residual_registry = app
        .instantiate_contract(residual_registry_id, creator, &msg, &[], "auction", None)
        .unwrap();

    let config: Config = app
        .wrap()
        .query_wasm_smart(residual_registry, &QueryMsg::Config {})
        .unwrap();

    assert_eq!(config.update_wait_period, update_wait_period);
    assert_eq!(config.max_share_delta, max_share_delta);
}

#[test]
fn try_initialize_collection_residual() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let _block_time = router.block_info().time;

    let residual_entry: Option<ResidualEntry> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(residual_entry.is_none());

    // Anyone can initialize a collection residual default
    let msg = ExecuteMsg::InitializeCollectionResidual {
        collection: collection.to_string(),
    };

    let response = router.execute_contract(bidder.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_default: Option<ResidualDefault> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_default,
        Some(ResidualDefault {
            collection,
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(creator),
                share: Decimal::percent(10),
                updated: None
            }
        })
    );

    // Initialize cannot be invoked twice
    let response = router.execute_contract(bidder, residual_registry, &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionResidual(
            "Collection residual already initialized".to_string(),
        )
        .to_string(),
    );
}

#[test]
fn try_set_collection_residual_default() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let residual_entry: Option<ResidualEntry> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(residual_entry.is_none());

    // Non collection owner cannot set collection residual default
    let msg = ExecuteMsg::SetCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: bidder.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(bidder, residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot set collection residual default above 100%
    let msg = ExecuteMsg::SetCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(101),
    };
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionResidual(
            "Residual share must be less than or equal to 1".to_string(),
        )
        .to_string(),
    );

    // Collection owner can set collection residual default
    let msg = ExecuteMsg::SetCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_default: Option<ResidualDefault> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_default,
        Some(ResidualDefault {
            collection: collection.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(creator.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner cannot set collection residual default twice
    let msg = ExecuteMsg::SetCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator, residual_registry, &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionResidual(
            "Collection residual already initialized".to_string(),
        )
        .to_string(),
    );
}

#[test]
fn try_update_collection_residual_default() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();

    let config: Config = router
        .wrap()
        .query_wasm_smart(residual_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Non collection owner cannot update collection residual default
    let msg = ExecuteMsg::UpdateCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: None,
        decrement: None,
    };

    let response = router.execute_contract(bidder.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot update collection residual default within wait period
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Residual entry cannot be updated yet".to_string()).to_string(),
    );

    // Collection owner can update collection residual default outside of wait period
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_default: Option<ResidualDefault> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_default,
        Some(ResidualDefault {
            collection: collection.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can increment collection residual default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_default: Option<ResidualDefault> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_default,
        Some(ResidualDefault {
            collection: collection.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(11),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can decrement collection residual default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionResidualDefault {
        collection: collection.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    let response = router.execute_contract(creator, residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_default: Option<ResidualDefault> = router
        .wrap()
        .query_wasm_smart(
            residual_registry,
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_default,
        Some(ResidualDefault {
            collection,
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );
}

#[test]
fn try_set_collection_residual_protocol() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let residual_entry: Option<ResidualEntry> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualDefault {
                collection: collection.to_string(),
            },
        )
        .unwrap();

    assert!(residual_entry.is_none());

    // Non collection owner cannot set collection residual protocol
    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: bidder.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(bidder, residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner can set collection residual protocol
    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_protocol: Option<ResidualProtocol> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_protocol,
        Some(ResidualProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(creator.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner cannot set collection residual default twice
    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::InvalidCollectionResidual(
            "Collection residual protocol already initialized".to_string(),
        )
        .to_string(),
    );

    let residual_protocols = router
        .wrap()
        .query_wasm_smart::<Vec<ResidualProtocol>>(
            residual_registry,
            &QueryMsg::ResidualProtocolByCollection {
                collection: collection.to_string(),
                query_options: None,
            },
        )
        .unwrap();

    assert_eq!(residual_protocols.len(), 1);
    assert_eq!(
        residual_protocols[0],
        ResidualProtocol {
            collection,
            protocol,
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(creator),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        }
    );
}

#[test]
fn try_update_collection_residual_protocol() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(residual_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Non collection owner cannot update collection residual default
    let msg = ExecuteMsg::UpdateCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: None,
        decrement: None,
    };

    let response = router.execute_contract(bidder.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
            .to_string(),
    );

    // Collection owner cannot update collection residual default within wait period
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert_error(
        response,
        ContractError::Unauthorized("Residual entry cannot be updated yet".to_string()).to_string(),
    );

    // Collection owner can update collection residual default outside of wait period
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_protocol: Option<ResidualProtocol> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_protocol,
        Some(ResidualProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can increment collection residual default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_protocol: Option<ResidualProtocol> = router
        .wrap()
        .query_wasm_smart(
            residual_registry.clone(),
            &QueryMsg::CollectionResidualProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_protocol,
        Some(ResidualProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder.clone()),
                share: Decimal::percent(11),
                updated: Some(block_time)
            }
        })
    );

    // Collection owner can decrement collection residual default shares, but not more than max_share_delta
    setup_block_time(
        &mut router,
        block_time.plus_seconds(config.update_wait_period).nanos(),
        None,
    );
    let block_time = router.block_info().time;
    let msg = ExecuteMsg::UpdateCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: None,
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    let response = router.execute_contract(creator, residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    let residual_protocol: Option<ResidualProtocol> = router
        .wrap()
        .query_wasm_smart(
            residual_registry,
            &QueryMsg::CollectionResidualProtocol {
                collection: collection.to_string(),
                protocol: protocol.to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        residual_protocol,
        Some(ResidualProtocol {
            collection,
            protocol,
            residual_entry: ResidualEntry {
                recipient: Addr::unchecked(bidder),
                share: Decimal::percent(10),
                updated: Some(block_time)
            }
        })
    );
}

#[test]
fn try_over_100_percent_residual() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(residual_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let mut block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Collection owner can not exceed 100% residual. Test 101% residual
    let msg = ExecuteMsg::UpdateCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: Some(Decimal::percent(10)),
        decrement: None,
    };
    for i in 1..=91 {
        block_time = block_time.plus_seconds(config.update_wait_period);
        setup_block_time(&mut router, block_time.nanos(), None);
        let response =
            router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
        // 10 + 91 = 101% > 100% max residual
        if i == 91 {
            assert_error(
                response,
                ContractError::InvalidCollectionResidual(
                    "Residual share must be less than or equal to 1".to_string(),
                )
                .to_string(),
            );
        } else {
            assert!(response.is_ok());
        }
    }
}

#[test]
fn try_0_residual() {
    let vt = standard_minter_template(1);
    let (mut router, creator, bidder) = (vt.router, vt.accts.creator, vt.accts.bidder);
    let residual_registry = setup_residual_registry(&mut router, creator.clone());
    let collection = vt.collection_response_vec[0].collection.clone().unwrap();
    let protocol = Addr::unchecked("protocol");

    let config: Config = router
        .wrap()
        .query_wasm_smart(residual_registry.clone(), &QueryMsg::Config {})
        .unwrap();

    setup_block_time(&mut router, GENESIS_MINT_START_TIME, None);
    let mut block_time = router.block_info().time;

    let msg = ExecuteMsg::SetCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: creator.to_string(),
        share: Decimal::percent(10),
    };

    let response = router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
    assert!(response.is_ok());

    // Collection owner can decrement to 0% residual
    let msg = ExecuteMsg::UpdateCollectionResidualProtocol {
        collection: collection.to_string(),
        protocol: protocol.to_string(),
        recipient: Some(bidder.to_string()),
        share_delta: Some(Decimal::percent(10)),
        decrement: Some(true),
    };
    for _ in 1..=10 {
        block_time = block_time.plus_seconds(config.update_wait_period);
        setup_block_time(&mut router, block_time.nanos(), None);

        let response =
            router.execute_contract(creator.clone(), residual_registry.clone(), &msg, &[]);
        assert!(response.is_ok());
    }
}
