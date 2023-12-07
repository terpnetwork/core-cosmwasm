use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, CustomQuery, Empty, Querier, QuerierResult, Storage,
};
use cosmwasm_std::{BankMsg, OwnedDeps};
use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, Module, WasmKeeper,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use terp_sdk::{TerpMsgWrapper, TerpQuery};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct TerpModule {}

pub type TerpDeps = OwnedDeps<MockStorage, MockApi, MockQuerier, TerpQuery>;

pub fn mock_deps() -> TerpDeps {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}

impl TerpModule {}

impl Module for TerpModule {
    type ExecT = TerpMsgWrapper;
    type QueryT = Empty;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: TerpMsgWrapper,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        let TerpMsgWrapper {
            route: _,
            msg_data,
            version: _,
        } = msg;

        match msg_data {
            terp_sdk::TerpMsg::FundCommunityPool { amount } => {
                let msg = BankMsg::Send {
                    to_address: "community_pool".to_owned(),
                    amount,
                }
                .into();
                router.execute(api, storage, block, sender, msg)?;
                Ok(AppResponse::default())
            }
            // terp_sdk::TerpMsg::FundFairburnPool { amount } => {
            //     let msg = BankMsg::Send {
            //         to_address: "fairburn_pool".to_owned(),
            //         amount,
            //     }
            //     .into();
            //     router.execute(api, storage, block, sender, msg)?;
            //     Ok(AppResponse::default())
            // }
            // terp_sdk::TerpMsg::ClaimFor {
            //     action: _,
            //     address: _,
            // } => Ok(AppResponse::default()),
        }
    }
    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!("sudo not implemented")
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: Empty,
    ) -> anyhow::Result<Binary> {
        bail!("custom query not implemented {:?}", request)
    }
}

pub type TerpBasicApp =
    App<BankKeeper, MockApi, MockStorage, TerpModule, WasmKeeper<TerpMsgWrapper, Empty>>;

pub struct TerpApp(TerpBasicApp);

impl Deref for TerpApp {
    type Target = TerpBasicApp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TerpApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Querier for TerpApp {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.0.raw_query(bin_request)
    }
}

impl TerpApp {
    pub fn new() -> Self {
        Self(
            BasicAppBuilder::<TerpMsgWrapper, Empty>::new_custom()
                .with_custom(TerpModule {})
                .build(|_, _, _| {}),
        )
    }
}

impl Default for TerpApp {
    fn default() -> Self {
        Self::new()
    }
}
