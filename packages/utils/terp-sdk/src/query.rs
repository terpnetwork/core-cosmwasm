use cosmwasm_schema::cw_serde;

use cosmwasm_std::CustomQuery;

#[cw_serde]
pub enum TerpQuery {}

impl CustomQuery for TerpQuery {}
