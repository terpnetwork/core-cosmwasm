use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw721::Cw721ReceiveMsg;
use std::fmt;

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_fee: Decimal,
    pub min_increment: Decimal,
    pub min_reserve_price: Uint128,
    pub max_royalty_fee: Decimal,
    pub duration: u64,
    pub min_duration: u64,
    pub accepted_denom: Vec<String>,
    pub protocol_addr: String,
}

/// This is like Cw721HandleMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    CancelAuction {
        auction_id: Uint128,
    },
    PlaceBid {
        auction_id: Uint128,
    },
    Settle {
        auction_id: Uint128,
    },
    // admin
    AdminChangeConfig {
        protocol_fee: Decimal,
        min_increment: Decimal,
        min_reserve_price: Uint128,
        max_royalty_fee: Decimal,
        duration: u64,
        min_duration: u64,
        accepted_denom: Vec<String>,
        protocol_addr: String,
    },
    AdminCancelAuction {
        auction_id: Uint128,
    },
    SetRoyaltyFee {
        contract_addr: String,
        creator: String,
        royalty_fee: Decimal,
    },
    SetRoyaltyAdmin {
        address: String,
        enable: bool,
    },
    // stop create new auction
    AdminPause {},
    AdminResume {},
    SettleHook {
        nft_contract: String,
        token_id: String,
        owner: String,
    },
}

/// This is like Cw721HandleMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State {},
    #[returns(AuctionResponse)]
    Auction { auction_id: Uint128 },
    #[returns(RoyaltyFeeResponse)]
    RoyaltyFee { contract_addr: String },
    #[returns(RoyaltyAdminResponse)]
    RoyaltyAdmin { address: String },
    #[returns(AllRoyaltyListResponse)]
    AllRoyaltyFee {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(CalculatePriceResponse)]
    CalculatePrice {
        nft_contract: String,
        token_id: String,
        amount: Uint128,
    },
    #[returns(AuctionResponse)]
    NftAuction {
        nft_contract: String,
        token_id: String,
    },
    #[returns(BidHistoryByAuctionIdResponse)]
    BidHistoryByAuctionId {
        auction_id: Uint128,
        limit: Option<u32>,
    },
    #[returns(BidsCountResponse)]
    BidsCount { auction_id: Uint128 },
    #[returns(AuctionListResponse)]
    AuctionByContract {
        nft_contract: String,
        limit: Option<u32>,
    },
    #[returns(AuctionListResponse)]
    AuctionBySeller { seller: String, limit: Option<u32> },
    #[returns(AuctionListResponse)]
    AuctionByAmount {
        nft_contract: String,
        amount: Uint128,
        limit: Option<u32>,
    },
    #[returns(AuctionListResponse)]
    AuctionByEndTime {
        nft_contract: String,
        end_time: u64,
        limit: Option<u32>,
        is_desc: Option<bool>,
    },
    #[returns(AuctionListResponse)]
    NotStartedAuction {
        nft_contract: String,
        start_after: Option<u128>,
        limit: Option<u32>,
        is_desc: Option<bool>,
    },
    #[returns(AuctionListResponse)]
    AuctionByBidder {
        bidder: String,
        start_after: Option<u128>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct Royalty {
    pub royalty_fee: Decimal,
    pub creator: Addr,
}

#[cw_serde]
pub struct RoyaltyResponse {
    pub royalty_fee: Decimal,
    pub creator: String,
}

#[cw_serde]
pub struct RoyaltyAdminResponse {
    pub address: String,
    pub enable: bool,
}

#[cw_serde]
pub struct RoyaltyFeeResponse {
    pub royalty_fee: Option<RoyaltyResponse>,
}

#[cw_serde]
pub struct AllRoyaltyFeeResponse {
    pub contract_addr: String,
    pub royalty_fee: Decimal,
    pub creator: String,
}

#[cw_serde]
pub struct AllRoyaltyListResponse {
    pub royalty_fees: Vec<AllRoyaltyFeeResponse>,
}

#[cw_serde]
pub struct BidHistoryByAuctionIdResponse {
    pub bids: Vec<Bid>,
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub protocol_fee: Decimal,
    pub min_reserve_price: Uint128,
    pub min_increment: Decimal,
    pub duration: u64,
    pub min_duration: u64,
    pub accepted_denom: Vec<String>,
    pub protocol_addr: String,
    pub max_royalty_fee: Decimal,
}

#[cw_serde]
pub struct StateResponse {
    pub next_auction_id: Uint128,
    pub is_freeze: bool,
}

#[cw_serde]
pub struct AuctionListResponse {
    pub auctions: Vec<AuctionResponse>,
}

#[cw_serde]
pub struct BidsCountResponse {
    pub count: Uint128,
}

#[cw_serde]
pub struct AuctionResponse {
    pub auction_id: Uint128,
    pub auction_type: AuctionType,
    pub nft_contract: String,
    pub token_id: String,
    pub seller: String,
    pub duration: u64,
    pub min_duration: u64,
    pub denom: String,
    pub reserve_price: Uint128,
    pub end_time: u64,
    pub bidder: Option<String>,
    pub amount: Uint128,
    pub creator_address: Option<String>,
    pub royalty_fee: Decimal,
    pub is_settled: bool,
}
#[cw_serde]
pub struct CalculatePriceResponse {
    pub nft_contract: String,
    pub token_id: String,
    pub amount: Uint128,
    pub protocol_fee: Uint128,
    pub royalty_fee: Uint128,
    pub seller_amount: Uint128,
}

#[cw_serde]
pub struct Bid {
    pub auction_id: Uint128,
    pub bidder: Addr,
    pub time: u64,
    pub denom: String,
    pub amount: Uint128,
}

#[cw_serde]
pub enum AuctionType {
    Auction,
    BuyNow,
}

impl fmt::Display for AuctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuctionType::Auction => write!(f, "auction"),
            AuctionType::BuyNow => write!(f, "buy_now"),
        }
    }
}

#[cw_serde]
pub enum Cw721HookMsg {
    CreateAuction {
        denom: String,
        reserve_price: Uint128,
        is_instant_sale: bool, // default is false
    },
}

#[cw_serde]
pub struct MigrateMsg {}
