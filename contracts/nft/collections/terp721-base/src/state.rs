use cosmwasm_std::{Empty, Timestamp};
use cw_storage_plus::Item;
use serde::{de::DeserializeOwned, Serialize};
use terp721::{CollectionInfo, ResidualInfo};
use terp_sdk::TerpMsgWrapper;
use std::ops::Deref;

type Parent<'a, T> = cw721_base::Cw721Contract<'a, T, TerpMsgWrapper, Empty, Empty>;
pub struct Terp721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub parent: Parent<'a, T>,
    pub collection_info: Item<'a, CollectionInfo<ResidualInfo>>,

    /// Instantiate set to false by the minter, then true by creator to freeze collection info
    pub frozen_collection_info: Item<'a, bool>,
    pub royalty_updated_at: Item<'a, Timestamp>,
}

impl<'a, T> Default for Terp721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Terp721Contract {
            parent: cw721_base::Cw721Contract::default(),
            collection_info: Item::new("collection_info"),
            frozen_collection_info: Item::new("frozen_collection_info"),
            royalty_updated_at: Item::new("royalty_updated_at"),
        }
    }
}

impl<'a, T> Deref for Terp721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    type Target = Parent<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
