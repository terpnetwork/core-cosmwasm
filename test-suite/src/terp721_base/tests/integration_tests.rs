#[cfg(test)]
mod tests {
    use anyhow::Error;
    use cosmwasm_std::{coin, Addr};
    use cw721::NumTokensResponse;
    use cw_multi_test::{AppResponse, BankSudo, Executor, SudoMsg};
    use factory_utils::msg::CreateMinterMsg;
    use factory_utils::tests::mock_collection_params;
    use terp721::ExecuteMsg as Terp721ExecuteMsg;
    use terp721::{CollectionInfo, InstantiateMsg};
    use terp_multi_test::TerpApp;
    use vending_factory::helpers::FactoryContract;
    use vending_factory::msg::{
        ExecuteMsg, InstantiateMsg as FactoryInstantiateMsg, VendingMinterInitMsgExtension,
    };

    use crate::common_setup::contract_boxes::{
        contract_terp721_base, contract_vending_factory, contract_vending_minter, custom_mock_app,
    };
    use crate::common_setup::setup_minter::common::constants::CREATION_FEE;
    use crate::common_setup::setup_minter::vending_minter::mock_params::{
        mock_create_minter, mock_init_extension, mock_params,
    };
    use cosmwasm_std::Empty;
    use cw721_base::msg::ExecuteMsg as cw721ExecuteMsg;
    use cw721_base::Ownership;

    const GOVERNANCE: &str = "governance";
    const ADMIN: &str = "admin";
    const NATIVE_DENOM: &str = "uterp";

    pub fn assert_error(res: Result<AppResponse, Error>, expected: String) {
        assert_eq!(res.unwrap_err().source().unwrap().to_string(), expected);
    }

    fn proper_instantiate_factory() -> (TerpApp, FactoryContract) {
        let mut app = custom_mock_app();
        let factory_id = app.store_code(contract_vending_factory());
        let minter_id = app.store_code(contract_vending_minter());

        let mut params = mock_params(None);
        params.code_id = minter_id;

        let msg = FactoryInstantiateMsg { params };
        let factory_addr = app
            .instantiate_contract(
                factory_id,
                Addr::unchecked(GOVERNANCE),
                &msg,
                &[],
                "factory",
                Some(GOVERNANCE.to_string()),
            )
            .unwrap();

        let factory_contract = FactoryContract(factory_addr);

        (app, factory_contract)
    }

    fn proper_instantiate() -> (TerpApp, Addr) {
        let (mut app, factory_contract) = proper_instantiate_factory();
        let terp721_id = app.store_code(contract_terp721_base());

        let collection_params = mock_collection_params();
        let mut m = mock_create_minter(None, collection_params, None);
        m.collection_params.code_id = terp721_id;
        let msg = ExecuteMsg::CreateMinter(m);

        let creation_fee = coin(CREATION_FEE, NATIVE_DENOM);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: ADMIN.to_string(),
            amount: vec![creation_fee.clone()],
        }))
        .unwrap();

        let bal = app.wrap().query_all_balances(ADMIN).unwrap();
        assert_eq!(bal, vec![creation_fee.clone()]);

        // this should create the minter + terp721
        let cosmos_msg = factory_contract.call_with_funds(msg, creation_fee).unwrap();

        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        assert!(res.is_ok());

        (app, Addr::unchecked("contract2"))
    }

    fn custom_proper_instantiate(
        custom_create_minter_msg: CreateMinterMsg<VendingMinterInitMsgExtension>,
    ) -> (TerpApp, Addr) {
        let (mut app, factory_contract) = proper_instantiate_factory();
        let terp721_id = app.store_code(contract_terp721_base());

        let mut m = custom_create_minter_msg;
        m.collection_params.code_id = terp721_id;
        let msg = ExecuteMsg::CreateMinter(m);

        let creation_fee = coin(CREATION_FEE, NATIVE_DENOM);

        app.sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: ADMIN.to_string(),
            amount: vec![creation_fee.clone()],
        }))
        .unwrap();

        let bal = app.wrap().query_all_balances(ADMIN).unwrap();
        assert_eq!(bal, vec![creation_fee.clone()]);

        // this should create the minter + terp721
        let cosmos_msg = factory_contract.call_with_funds(msg, creation_fee).unwrap();

        let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
        assert!(res.is_ok());

        (app, Addr::unchecked("contract2"))
    }

    mod init {
        use cw721_base::MinterResponse;

        use crate::common_setup::setup_minter::vending_minter::mock_params::mock_create_minter_init_msg;

        use super::*;
        use terp721_base::msg::QueryMsg;
        use vending_minter::msg::{ConfigResponse, QueryMsg as VendingMinterQueryMsg};

        #[test]
        fn create_terp721_base_collection() {
            let (app, contract) = proper_instantiate();

            let res: NumTokensResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::NumTokens {})
                .unwrap();
            assert_eq!(res.count, 0);
        }

        #[test]
        fn check_ready_unauthorized() {
            let mut app = custom_mock_app();
            let terp721_id = app.store_code(contract_terp721_base());
            let msg = InstantiateMsg {
                name: "terp721".to_string(),
                symbol: "TERP".to_string(),
                minter: ADMIN.to_string(),
                collection_info: CollectionInfo {
                    creator: ADMIN.to_string(),
                    description: "description".to_string(),
                    image: "description".to_string(),
                    external_link: None,
                    explicit_content: None,
                    start_trading_time: None,
                    residual_info: None,
                },
            };
            let res = app.instantiate_contract(
                terp721_id,
                Addr::unchecked(GOVERNANCE),
                &msg,
                &[],
                "terp721-only",
                None,
            );
            // should not let create the contract.
            assert!(res.is_err());
        }

        #[test]
        fn check_ready_authorized() {
            let (_, _) = proper_instantiate();
        }

        #[test]
        fn sanitize_base_token_uri() {
            let base_token_uri = " ipfs://somecidhere ".to_string();
            let init_msg = VendingMinterInitMsgExtension {
                base_token_uri: base_token_uri.clone(),
                ..mock_init_extension(None, None)
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(mock_collection_params(), init_msg);
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            // query minter config to confirm base_token_uri got trimmed
            let res: MinterResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::Minter {})
                .unwrap();
            let minter = res.minter;
            let minter = minter.unwrap();
            let res: ConfigResponse = app
                .wrap()
                .query_wasm_smart(minter, &VendingMinterQueryMsg::Config {})
                .unwrap();
            assert_eq!(res.base_token_uri, base_token_uri.trim().to_string());

            // test sanitizing base token uri IPFS -> ipfs
            let base_token_uri = " IPFS://somecidhereipfs ".to_string();
            let init_msg = VendingMinterInitMsgExtension {
                base_token_uri,
                ..mock_init_extension(None, None)
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(mock_collection_params(), init_msg);
            // let custom_create_minter_msg = mock_create_minter(None, mock_collection_params(), None);
            // let custom_create_minter_msg =
            //     custom_mock_create_minter(init_msg, mock_collection_params());

            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            // query minter config to confirm base_token_uri got trimmed and starts with ipfs
            let res: MinterResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::Minter {})
                .unwrap();
            let minter = res.minter.unwrap();
            let res: ConfigResponse = app
                .wrap()
                .query_wasm_smart(minter, &VendingMinterQueryMsg::Config {})
                .unwrap();
            assert_eq!(res.base_token_uri, "ipfs://somecidhereipfs");

            // test case sensitive ipfs IPFS://aBcDeF -> ipfs://aBcDeF
            let base_token_uri = "IPFS://aBcDeF".to_string();
            let init_msg = VendingMinterInitMsgExtension {
                base_token_uri,
                ..mock_init_extension(None, None)
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(mock_collection_params(), init_msg);

            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);
            let res: MinterResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::Minter {})
                .unwrap();
            let minter = res.minter.unwrap();
            let res: ConfigResponse = app
                .wrap()
                .query_wasm_smart(minter, &VendingMinterQueryMsg::Config {})
                .unwrap();
            assert_eq!(res.base_token_uri, "ipfs://aBcDeF");
        }
    }

    mod start_trading_time {
        use cosmwasm_std::{Decimal, Empty};
        use terp721::{ResidualInfoResponse, UpdateCollectionInfoMsg};

        use crate::common_setup::{
            setup_accounts_and_block::setup_block_time,
            setup_minter::vending_minter::mock_params::mock_create_minter_init_msg,
        };

        use super::*;
        use terp721_base::{
            msg::{CollectionInfoResponse, QueryMsg},
            ContractError,
        };

        #[test]
        fn royalty_updates() {
            let mut params = mock_collection_params();
            params.info.external_link = None;
            params.info.residual_info = None;
            let custom_create_minter_msg =
                mock_create_minter_init_msg(params, mock_init_extension(None, None));
            let (mut app, contract) = custom_proper_instantiate(custom_create_minter_msg);
            let creator = Addr::unchecked("creator".to_string());

            let residual_info: Option<ResidualInfoResponse> = Some(ResidualInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(11),
            });
            let res = app.execute_contract(
                creator,
                contract,
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: None,
                        image: None,
                        external_link: None,
                        explicit_content: None,
                        residual_info: Some(residual_info),
                    },
                },
                &[],
            );
            assert!(res.is_err());
        }
        #[test]
        fn update_collection_info() {
            // customize params so external_link is None
            let mut params = mock_collection_params();
            params.info.external_link = None;
            let custom_create_minter_msg =
                mock_create_minter_init_msg(params.clone(), mock_init_extension(None, None));
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg.clone());

            // default trading start time is start time + default trading start time offset
            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::CollectionInfo {})
                .unwrap();
            let default_start_time = mock_init_extension(None, None)
                .start_time
                .plus_seconds(mock_params(None).max_trading_offset_secs);
            assert_eq!(res.start_trading_time, Some(default_start_time));

            // update collection info
            let (mut app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let creator = Addr::unchecked("creator".to_string());

            // succeeds
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: None,
                        residual_info: None,
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            // royalty cannot be updated before a day has passed
            let residual_info: Option<ResidualInfoResponse> = Some(ResidualInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(7),
            });
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: None,
                        residual_info: Some(residual_info),
                    },
                },
                &[],
            );
            assert_error(
                res,
                ContractError::InvalidRoyalties(
                    "Royalties can only be updated once per day".to_string(),
                )
                .to_string(),
            );

            // lower residual_info by more than 2% succeeds
            let block_time = app.block_info().time;
            setup_block_time(
                &mut app,
                block_time.plus_seconds(24 * 60 * 60).nanos(),
                None,
            );
            let residual_info: Option<ResidualInfoResponse> = Some(ResidualInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(7),
            });
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: None,
                        residual_info: Some(residual_info),
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            // raise residual_info by more than 2% throws error
            let block_time = app.block_info().time;
            setup_block_time(
                &mut app,
                block_time.plus_seconds(24 * 60 * 60).nanos(),
                None,
            );
            let residual_info: Option<ResidualInfoResponse> = Some(ResidualInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(10),
            });
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: None,
                        residual_info: Some(residual_info),
                    },
                },
                &[],
            );
            assert_error(
                res,
                ContractError::InvalidRoyalties(
                    "Share increase cannot be greater than 2%".to_string(),
                )
                .to_string(),
            );

            // raise residual_info by 2% succeeds
            let residual_info: Option<ResidualInfoResponse> = Some(ResidualInfoResponse {
                payment_address: creator.to_string(),
                share: Decimal::percent(9),
            });
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: None,
                        residual_info: Some(residual_info.clone()),
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();
            assert_eq!(res.residual_info.unwrap(), residual_info.unwrap());

            // update explicit content
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link.clone()),
                        explicit_content: Some(true),
                        residual_info: None,
                    },
                },
                &[],
            );
            assert!(res.is_ok());

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();
            // check explicit content changed to true
            assert!(res.explicit_content.unwrap());

            // freeze collection throw err if not creator
            let res = app.execute_contract(
                Addr::unchecked("badguy"),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::FreezeCollectionInfo {},
                &[],
            );
            assert!(res.is_err());
            // freeze collection to prevent further updates
            let res = app.execute_contract(
                creator.clone(),
                contract.clone(),
                &Terp721ExecuteMsg::<Empty, Empty>::FreezeCollectionInfo {},
                &[],
            );
            assert!(res.is_ok());

            // trying to update collection after frozen should throw err
            let res = app.execute_contract(
                creator,
                contract,
                &Terp721ExecuteMsg::<Empty, Empty>::UpdateCollectionInfo {
                    collection_info: UpdateCollectionInfoMsg {
                        description: Some(params.info.description.clone()),
                        image: Some(params.info.image.clone()),
                        external_link: Some(params.info.external_link),
                        explicit_content: None,
                        residual_info: None,
                    },
                },
                &[],
            );
            assert!(res.is_err());
        }
    }

    mod royalty_payout {
        use super::*;

        use crate::common_setup::setup_minter::vending_minter::mock_params::mock_create_minter_init_msg;
        use cosmwasm_std::{Decimal, Response, Uint128};
        use factory_utils::msg::CollectionParams;
        use terp721::ResidualInfoResponse;
        use terp721_base::msg::{CollectionInfoResponse, QueryMsg};

        #[test]
        fn standard_payout() {
            let (app, contract) = proper_instantiate();

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();

            // payout 100uthiolx, royalty share 10%, royalty payout 10uthiolx
            let payment = Uint128::from(100000000u128);
            let royalty_share = res.clone().residual_info.unwrap().share;
            let royalty_payout = res
                .royalty_payout(
                    contract,
                    payment,
                    Uint128::from(10000000u128),
                    None,
                    &mut Response::default(),
                )
                .unwrap();
            assert_eq!(royalty_payout, payment * royalty_share);
        }

        #[test]
        fn payout_0_royalties() {
            let init_msg = mock_init_extension(None, None);
            let custom_collection_params = CollectionParams {
                info: CollectionInfo {
                    creator: "creator".to_string(),
                    description: String::from("Terp Monkeys"),
                    image: "https://example.com/image.png".to_string(),
                    external_link: Some("https://example.com/external.html".to_string()),
                    start_trading_time: None,
                    explicit_content: Some(false),
                    residual_info: Some(ResidualInfoResponse {
                        payment_address: "creator".to_string(),
                        share: Decimal::percent(0),
                    }),
                },
                ..mock_collection_params()
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(custom_collection_params, init_msg);
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();

            // payout 100uthiolx, royalty share 0%, royalty payout 0uthiolx
            let payment = Uint128::from(100000000u128);
            let royalty_payout = res
                .royalty_payout(
                    contract,
                    payment,
                    Uint128::from(10000000u128),
                    None,
                    &mut Response::default(),
                )
                .unwrap();
            assert_eq!(royalty_payout, Uint128::zero());
        }

        #[test]
        fn payout_too_much_royalties() {
            let init_msg = mock_init_extension(None, None);
            let custom_collection_params = CollectionParams {
                info: CollectionInfo {
                    creator: "creator".to_string(),
                    description: String::from("Terp Monkeys"),
                    image: "https://example.com/image.png".to_string(),
                    external_link: Some("https://example.com/external.html".to_string()),
                    start_trading_time: None,
                    explicit_content: Some(false),
                    residual_info: Some(ResidualInfoResponse {
                        payment_address: "creator".to_string(),
                        share: Decimal::percent(91),
                    }),
                },
                ..mock_collection_params()
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(custom_collection_params, init_msg);
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();

            // payout 100uthiolx, royalty share 91%, royalty payout fails
            // fees exceed payment
            let payment = Uint128::from(100000000u128);
            let res = res.royalty_payout(
                contract,
                payment,
                Uint128::from(10000000u128),
                None,
                &mut Response::default(),
            );
            assert!(res.is_err());
        }

        #[test]
        fn payout_odd_royalties() {
            // uint * decimal::percent
            let init_msg = mock_init_extension(None, None);
            let custom_collection_params = CollectionParams {
                info: CollectionInfo {
                    creator: "creator".to_string(),
                    description: String::from("Terp Monkeys"),
                    image: "https://example.com/image.png".to_string(),
                    external_link: Some("https://example.com/external.html".to_string()),
                    start_trading_time: None,
                    explicit_content: Some(false),
                    residual_info: Some(ResidualInfoResponse {
                        payment_address: "creator".to_string(),
                        share: Decimal::percent(3),
                    }),
                },
                ..mock_collection_params()
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(custom_collection_params, init_msg);
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();

            // payout 100uthiolx, royalty share 1%, royalty payout 10uthiolx
            let payment = Uint128::from(1111111111121111111u128);
            let royalty_share = res.clone().residual_info.unwrap().share;
            let royalty_payout = res
                .royalty_payout(
                    contract,
                    payment,
                    Uint128::from(10000000u128),
                    None,
                    &mut Response::default(),
                )
                .unwrap();
            assert_eq!(royalty_payout, payment * royalty_share);
        }

        #[test]
        fn payout_royalties_none() {
            let init_msg = mock_init_extension(None, None);
            let custom_collection_params = CollectionParams {
                info: CollectionInfo {
                    creator: "creator".to_string(),
                    description: String::from("Terp Monkeys"),
                    image: "https://example.com/image.png".to_string(),
                    external_link: Some("https://example.com/external.html".to_string()),
                    start_trading_time: None,
                    explicit_content: Some(false),
                    residual_info: None,
                },
                ..mock_collection_params()
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(custom_collection_params, init_msg);
            let (app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            let res: CollectionInfoResponse = app
                .wrap()
                .query_wasm_smart(contract.clone(), &QueryMsg::CollectionInfo {})
                .unwrap();

            // payout 100uthiolx, royalty share none, royalty payout 0uthiolx
            let payment = Uint128::from(100000000u128);
            let royalty_payout = res
                .royalty_payout(
                    contract,
                    payment,
                    Uint128::from(10000000u128),
                    None,
                    &mut Response::default(),
                )
                .unwrap();
            assert_eq!(royalty_payout, Uint128::zero());
        }
    }

    mod ownership {
        use cosmwasm_std::Attribute;
        use cw721_base::MinterResponse;

        use crate::common_setup::setup_minter::vending_minter::mock_params::mock_create_minter_init_msg;

        use super::*;
        use terp721_base::msg::QueryMsg;
        use vending_minter::msg::{ConfigResponse, QueryMsg as VendingMinterQueryMsg};

        #[test]
        fn test_update_ownership() {
            let base_token_uri = "ipfs://somecidhere".to_string();
            let init_msg = VendingMinterInitMsgExtension {
                base_token_uri,
                ..mock_init_extension(None, None)
            };
            let custom_create_minter_msg =
                mock_create_minter_init_msg(mock_collection_params(), init_msg);
            let (mut app, contract) = custom_proper_instantiate(custom_create_minter_msg);

            // query minter config to confirm base_token_uri got trimmed
            let res: MinterResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::Minter {})
                .unwrap();
            let minter = res.minter;
            let minter = minter.unwrap();
            let res: ConfigResponse = app
                .wrap()
                .query_wasm_smart(minter.clone(), &VendingMinterQueryMsg::Config {})
                .unwrap();
            let terp721_address = res.terp721_address;

            let update_ownership_msg: cw721ExecuteMsg<Empty, Empty> =
                cw721ExecuteMsg::UpdateOwnership(cw_ownable::Action::TransferOwnership {
                    new_owner: "new_owner".to_string(),
                    expiry: None,
                });
            let res = app.execute_contract(
                Addr::unchecked(minter),
                Addr::unchecked(terp721_address.clone()),
                &update_ownership_msg,
                &[],
            );
            let attribute_owner_response = res.unwrap().events[1].clone().attributes[2].clone();
            let expected_attribute = Attribute {
                key: "pending_owner".to_string(),
                value: "new_owner".to_string(),
            };
            assert_eq!(attribute_owner_response, expected_attribute);
            let res: cw_ownable::Ownership<Addr> = app
                .wrap()
                .query_wasm_smart(
                    terp721_address.clone(),
                    &terp721_base::msg::QueryMsg::Ownership {},
                )
                .unwrap();
            let pending_owner = res.pending_owner;
            let expected_pending_owner = Some(Addr::unchecked("new_owner".to_string()));
            assert_eq!(pending_owner, expected_pending_owner);

            let accept_ownership_msg: cw721ExecuteMsg<Empty, Empty> =
                cw721ExecuteMsg::UpdateOwnership(cw_ownable::Action::AcceptOwnership {});
            let res = app.execute_contract(
                Addr::unchecked("new_owner".to_string()),
                Addr::unchecked(terp721_address.clone()),
                &accept_ownership_msg,
                &[],
            );
            let pending_owner_response = res.unwrap().events[1].clone().attributes[2].clone();
            let expected_pending_owner_response = Attribute {
                key: "pending_owner".to_string(),
                value: "none".to_string(),
            };
            assert_eq!(pending_owner_response, expected_pending_owner_response);

            let res: cw_ownable::Ownership<Addr> = app
                .wrap()
                .query_wasm_smart(terp721_address, &terp721_base::msg::QueryMsg::Ownership {})
                .unwrap();

            let expected_onwership_response = Ownership {
                owner: Some(Addr::unchecked("new_owner".to_string())),
                pending_owner: None,
                pending_expiry: None,
            };
            assert_eq!(res, expected_onwership_response);
        }
    }

    mod terp721_mutable {
        use cosmwasm_std::{coin, Addr};
        use cw721::NumTokensResponse;
        use cw_multi_test::{BankSudo, Executor, SudoMsg};
        use factory_utils::tests::mock_collection_params;
        use terp721_base::msg::QueryMsg;
        use terp_multi_test::TerpApp;
        use terp_sdk::NATIVE_DENOM;
        const ADMIN: &str = "admin";

        use crate::common_setup::setup_minter::common::constants::CREATION_FEE;
        use crate::{
            common_setup::{
                contract_boxes::contract_terp721_updatable,
                setup_minter::vending_minter::mock_params::mock_create_minter,
            },
            terp721_base::tests::integration_tests::tests::proper_instantiate_factory,
        };
        use vending_factory::msg::ExecuteMsg;

        fn proper_instantiate() -> (TerpApp, Addr) {
            let (mut app, factory_contract) = proper_instantiate_factory();
            let terp721_id = app.store_code(contract_terp721_updatable());

            let collection_params = mock_collection_params();
            let mut m = mock_create_minter(None, collection_params, None);
            m.collection_params.code_id = terp721_id;
            let msg = ExecuteMsg::CreateMinter(m);

            let creation_fee = coin(CREATION_FEE, NATIVE_DENOM);

            app.sudo(SudoMsg::Bank(BankSudo::Mint {
                to_address: ADMIN.to_string(),
                amount: vec![creation_fee.clone()],
            }))
            .unwrap();

            let bal = app.wrap().query_all_balances(ADMIN).unwrap();
            assert_eq!(bal, vec![creation_fee.clone()]);

            // this should create the minter + terp721
            let cosmos_msg = factory_contract.call_with_funds(msg, creation_fee).unwrap();

            let res = app.execute(Addr::unchecked(ADMIN), cosmos_msg);
            assert!(res.is_ok());

            (app, Addr::unchecked("contract2"))
        }

        #[test]
        fn create_terp721_mutable_collection() {
            let (app, contract) = proper_instantiate();

            let res: NumTokensResponse = app
                .wrap()
                .query_wasm_smart(contract, &QueryMsg::NumTokens {})
                .unwrap();
            assert_eq!(res.count, 0);
        }
    }
}
