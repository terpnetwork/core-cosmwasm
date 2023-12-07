use crate::route::TerpRoute;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg};
static MSG_DATA_VERSION: &str = "1.0.0";

/// TerpMsg is an override of CosmosMsg::Custom to add support for Terp's custom message types
#[cw_serde]
pub struct TerpMsgWrapper {
    pub route: TerpRoute,
    pub msg_data: TerpMsg,
    pub version: String,
}

impl From<TerpMsgWrapper> for CosmosMsg<TerpMsgWrapper> {
    fn from(original: TerpMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

impl CustomMsg for TerpMsgWrapper {}

#[cw_serde]
pub enum TerpMsg {
    // ClaimFor {
    //     address: String,
    //     action: ClaimAction,
    // },
    FundCommunityPool {
        amount: Vec<Coin>,
    },
    // FundFairburnPool {
    //     amount: Vec<Coin>,
    // },
}

// #[cw_serde]
// pub enum ClaimAction {
//     #[serde(rename = "mint_nft")]
//     MintNFT,
//     #[serde(rename = "bid_nft")]
//     BidNFT,
// }

// pub fn create_claim_for_msg(address: String, action: ClaimAction) -> CosmosMsg<TerpMsgWrapper> {
//     TerpMsgWrapper {
//         route: TerpRoute::Claim,
//         msg_data: TerpMsg::ClaimFor { address, action },
//         version: MSG_DATA_VERSION.to_owned(),
//     }
//     .into()
// }

pub fn create_fund_community_pool_msg(amount: Vec<Coin>) -> CosmosMsg<TerpMsgWrapper> {
    TerpMsgWrapper {
        route: TerpRoute::Distribution,
        msg_data: TerpMsg::FundCommunityPool { amount },
        version: MSG_DATA_VERSION.to_owned(),
    }
    .into()
}

// pub fn create_fund_fairburn_pool_msg(amount: Vec<Coin>) -> CosmosMsg<TerpMsgWrapper> {
//     TerpMsgWrapper {
//         route: TerpRoute::Alloc,
//         msg_data: TerpMsg::FundFairburnPool { amount },
//         version: MSG_DATA_VERSION.to_owned(),
//     }
//     .into()
// }
