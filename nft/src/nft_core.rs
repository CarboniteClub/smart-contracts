use crate::*;

pub trait NonFungibleTokenCore {
    //get information about the NFT token passed in
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;

    fn update_carbonite_metadata(
        &mut self,
        token_id: TokenId,
        carbonite_metadata: CarboniteMetadata,
    );

    fn customize_nft(&mut self, token_id: TokenId, media: String, media_hash: Base64VecU8);
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    //get the information for a specific token ID
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        if let Some(token) = self.tokens_by_id.get(&token_id) {
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            Some(JsonToken {
                token_id,
                owner_id: token.owner_id,
                metadata,
                approved_account_ids: token.approved_account_ids,
                royalty: token.royalty,
            })
        } else {
            //if there wasn't a token ID in the tokens_by_id collection, we return None
            None
        }
    }

    #[payable]
    fn update_carbonite_metadata(
        &mut self,
        token_id: TokenId,
        carbonite_metadata: CarboniteMetadata,
    ) {
        self.assert_owner();
        require!(
            !self.tokens_by_id.get(&token_id).is_none(),
            "The given Token doesn't exist"
        );

        let initial_storage_usage = env::storage_usage();

        let mut total_xp = 0;
        let mut token_metadata = self.token_metadata_by_id.get(&token_id).unwrap();
        token_metadata.carbonite_metadata = carbonite_metadata;
        for xp in token_metadata.carbonite_metadata.skills.values() {
            total_xp += xp;
        }
        token_metadata.total_xp = total_xp;
        self.token_metadata_by_id.insert(&token_id, &token_metadata);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    fn customize_nft(&mut self, token_id: TokenId, media: String, media_hash: Base64VecU8) {
        self.assert_owner();
        require!(
            !self.tokens_by_id.get(&token_id).is_none(),
            "The given Token doesn't exist"
        );

        let initial_storage_usage = env::storage_usage();

        let mut token_metadata = self.token_metadata_by_id.get(&token_id).unwrap();
        token_metadata.media = media;
        token_metadata.media_hash = media_hash;

        self.token_metadata_by_id.insert(&token_id, &token_metadata);

        let required_storage_in_bytes = env::storage_usage()
            .checked_sub(initial_storage_usage)
            .unwrap_or_else(|| 0);
        refund_deposit(required_storage_in_bytes);
    }
}
