/**
* Non Fungible Token NEP-171 Token contract

* Enumeration NEP-181
* Metadata NEP-177
* Royalties and Payout NEP-199
*
* The aim of the contract is to provide a basic implementation of the improved function NFT standard.
*
* lib.rs is the main entry point.
* nft_core.rs implements and handles core function regarding nft transfers
* enumeration.rs implements NEP-181 standard for getter functions to retrieve data off-chain
* mint.rs implements nft_minting functionality
* metadata.rs implements NEP-177 standard for both Contract and NFT-specific metadata.
* events.rs extends NEP-297 for better indexing
* internal.rs contains internal methods.
**/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};
use std::collections::HashMap;

pub use crate::events::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::NonFungibleTokenCore;
pub use crate::utils::*;

mod enumeration;
mod events;
mod internal;
mod metadata;
mod mint;
mod nft_core;
mod utils;

#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[near_bindgen]
impl Contract {
    /// Initialize The Contract
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        metadata.assert_valid_metadata();
        Self {
            owner_id,
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        }
    }

    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Carbonite NFT Contract".to_string(),
                symbol: "CARBONITE".to_string(),
                icon: None,
                base_uri: "ipfs".to_string(),
                reference: "ipfs://example.com/hash".to_string(),
                reference_hash: Base64VecU8::from([5_u8; 32].to_vec()),
            },
        )
    }
}
