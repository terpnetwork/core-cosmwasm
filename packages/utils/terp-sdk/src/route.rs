use cosmwasm_schema::cw_serde;

/// TerpRoute is enum type to represent terp query route path
#[cw_serde]
pub enum TerpRoute {
    // Alloc,
    // Claim,
    Distribution,
}
