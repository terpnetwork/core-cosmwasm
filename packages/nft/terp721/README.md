# TERP-721

TERP-721 is a cw721-compatible spec that adds on-chain contract metadata, including residuals.

```rs
pub struct CollectionInfo<T> {
    pub creator: String,
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub trading_start_time: Option<Timestamp>,
    pub residual_info: Option<T>,
}

pub struct ResidualInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

```

The above is set when the contract is instantiated. The contract inherits everything else from cw721-base.
