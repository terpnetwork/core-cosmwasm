use crate::state::{Config, ResidualDefault, ResidualProtocol};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use terp_index_query::QueryOptions;

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
}

#[cw_serde]
pub enum ExecuteMsg {
    InitializeCollectionResidual {
        collection: String,
    },
    SetCollectionResidualDefault {
        collection: String,
        recipient: String,
        share: Decimal,
    },
    UpdateCollectionResidualDefault {
        collection: String,
        recipient: Option<String>,
        share_delta: Option<Decimal>,
        decrement: Option<bool>,
    },
    SetCollectionResidualProtocol {
        collection: String,
        protocol: String,
        recipient: String,
        share: Decimal,
    },
    UpdateCollectionResidualProtocol {
        collection: String,
        protocol: String,
        recipient: Option<String>,
        share_delta: Option<Decimal>,
        decrement: Option<bool>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Option<ResidualDefault>)]
    CollectionResidualDefault { collection: String },
    #[returns(Option<ResidualProtocol>)]
    CollectionResidualProtocol {
        collection: String,
        protocol: String,
    },
    #[returns(Vec<ResidualProtocol>)]
    ResidualProtocolByCollection {
        collection: String,
        query_options: Option<QueryOptions<String>>,
    },
    #[returns(ResidualPaymentResponse)]
    ResidualPayment {
        collection: String,
        protocol: Option<String>,
    },
}

#[cw_serde]
pub struct ResidualPaymentResponse {
    pub residual_default: Option<ResidualDefault>,
    pub residual_protocol: Option<ResidualProtocol>,
}

#[cw_serde]
pub enum SudoMsg {
    UpdateConfig { config: Config },
}
