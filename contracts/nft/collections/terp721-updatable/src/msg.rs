use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cosmwasm_std::Timestamp;
use cw_utils::Expiration;
use terp721::{ResidualInfoResponse, UpdateCollectionInfoMsg};
use terp721_base::msg::QueryMsg as Terp721QueryMsg;
use terp721_base::ExecuteMsg as Terp721ExecuteMsg;

#[cw_serde]
pub enum ExecuteMsg<T, E> {
    /// Freeze token metadata so creator can no longer update token uris
    FreezeTokenMetadata {},
    /// Creator calls can update token uris
    UpdateTokenMetadata {
        token_id: String,
        token_uri: Option<String>,
    },
    /// Enable updatable for updating token metadata. One time migration fee for terp721-base to terp721-updatable.
    EnableUpdatable {},
    // Sg721Base msgs
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Burn {
        token_id: String,
    },
    UpdateCollectionInfo {
        collection_info: UpdateCollectionInfoMsg<ResidualInfoResponse>,
    },
    UpdateStartTradingTime(Option<Timestamp>),
    FreezeCollectionInfo {},
    Mint {
        /// Unique ID of the NFT
        token_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: T,
    },
    Extension {
        msg: E,
    },
}

impl<T, E> From<ExecuteMsg<T, E>> for Terp721ExecuteMsg
where
    T: Clone + PartialEq + Into<Option<cosmwasm_std::Empty>>,
    Option<cosmwasm_std::Empty>: From<T>,
{
    fn from(msg: ExecuteMsg<T, E>) -> Terp721ExecuteMsg {
        match msg {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Terp721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            },
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Terp721ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            },
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Terp721ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            },
            ExecuteMsg::ApproveAll { operator, expires } => {
                Terp721ExecuteMsg::ApproveAll { operator, expires }
            }
            ExecuteMsg::Revoke { spender, token_id } => {
                Terp721ExecuteMsg::Revoke { spender, token_id }
            }
            ExecuteMsg::RevokeAll { operator } => Terp721ExecuteMsg::RevokeAll { operator },
            ExecuteMsg::Burn { token_id } => Terp721ExecuteMsg::Burn { token_id },
            ExecuteMsg::UpdateCollectionInfo { collection_info } => {
                Terp721ExecuteMsg::UpdateCollectionInfo { collection_info }
            }
            ExecuteMsg::FreezeCollectionInfo {} => Terp721ExecuteMsg::FreezeCollectionInfo {},
            ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => Terp721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension: extension.into(),
            },
            ExecuteMsg::UpdateStartTradingTime(start_trading_time) => {
                Terp721ExecuteMsg::UpdateStartTradingTime(start_trading_time)
            }
            _ => unreachable!("Invalid ExecuteMsg"),
        }
    }
}

#[cw_serde]
pub enum QueryMsg {
    EnableUpdatable {},
    EnableUpdatableFee {},
    FreezeTokenMetadata {},
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    ContractInfo {},
    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    Minter {},
    CollectionInfo {},
}

impl From<QueryMsg> for Terp721QueryMsg {
    fn from(msg: QueryMsg) -> Terp721QueryMsg {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Terp721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Terp721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Terp721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Terp721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            QueryMsg::NumTokens {} => Terp721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Terp721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Terp721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Terp721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Terp721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Terp721QueryMsg::AllTokens { start_after, limit }
            }
            QueryMsg::Minter {} => Terp721QueryMsg::Minter {},
            QueryMsg::CollectionInfo {} => Terp721QueryMsg::CollectionInfo {},
            _ => unreachable!("cannot convert {:?} to Terp721QueryMsg", msg),
        }
    }
}

#[cw_serde]
pub struct EnableUpdatableResponse {
    pub enabled: bool,
}

#[cw_serde]
pub struct FrozenTokenMetadataResponse {
    pub frozen: bool,
}
