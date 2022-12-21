use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FungibleTokenMetadata {
    /*
        FILL THIS IN
    */
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub decimal: u8,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

pub trait FungibleTokenMetadataProvider {
    // View call for returning the contract metadata
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}
