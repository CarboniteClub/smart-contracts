use crate::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintInfo {
    token_id: TokenId,
    metadata: TokenMetadata,
    receiver_id: AccountId,
    perpetual_royalties: Option<HashMap<AccountId, u32>>,
}

#[near_bindgen]
impl Contract {
    // this function will be capable of batch minting
    #[payable]
    pub fn nft_mint(&mut self, nft_info_list: Vec<NftMintInfo>) {
        self.assert_owner();

        let initial_storage_usage = env::storage_usage();

        let n = nft_info_list.len();
        let (mut royalty_sum, mut royalty, mut total_xp);
        let (mut receiver_id, mut token_id, mut metadata);
        let mut mint_logs: Vec<NftMintLog> = Vec::with_capacity(n);

        for nft_info in nft_info_list.into_iter() {
            if let Some(perpetual_royalties) = nft_info.perpetual_royalties {
                //make sure that the length of the perpetual royalties is below 7 since we won't have enough GAS to pay out that many people
                require!(
                    perpetual_royalties.len() < 7,
                    "Cannot add more than 6 perpetual royalty amounts"
                );
                royalty = HashMap::with_capacity(perpetual_royalties.len());
                royalty_sum = 0;

                for (account, amount) in perpetual_royalties {
                    royalty.insert(account, amount);
                    royalty_sum += amount;
                }

                require!(royalty_sum <= 10_000, "Invalid Royalty exceeding 100%");
            } else {
                royalty = HashMap::new();
            }

            total_xp = 0;
            receiver_id = nft_info.receiver_id;
            token_id = nft_info.token_id;
            metadata = nft_info.metadata;
            for xp in metadata.carbonite_metadata.skills.values() {
                total_xp += xp;
            }

            metadata.total_xp = total_xp;

            let token = Token {
                owner_id: receiver_id,
                approved_account_ids: Default::default(),
                next_approval_id: 0,
                royalty,
            };

            require!(
                self.tokens_by_id.insert(&token_id, &token).is_none(),
                "Token already exists"
            );

            self.token_metadata_by_id.insert(&token_id, &metadata);

            self.internal_add_token_to_owner(&token.owner_id, &token_id);

            mint_logs.push(NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: None,
            });
        }
        // Emitting Mint Event
        NftMintLog::emit(mint_logs);

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }
}
