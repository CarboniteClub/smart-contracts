use crate::*;

#[macro_export]
macro_rules! require {
    ( $a:expr, $b:expr ) => {
        if !$a {
            near_sdk::env::panic($b.as_bytes());
        }
    };
}

impl Contract {
    /// Assert that Predecessor A/c is the Owner of the Contract
    pub fn assert_owner(&self) {
        require!(
            near_sdk::env::predecessor_account_id() == self.owner_id,
            "It is a owner only method"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub mod test_utils {

    use crate::*;
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::Balance;
    use near_sdk::VMContext;

    // Helper functions

    pub fn alice() -> ValidAccountId {
        ValidAccountId::try_from("alice.near").unwrap()
    }
    pub fn bob() -> ValidAccountId {
        ValidAccountId::try_from("bob.near").unwrap()
    }
    pub fn carol() -> ValidAccountId {
        ValidAccountId::try_from("carol.near").unwrap()
    }
    pub fn nft() -> ValidAccountId {
        ValidAccountId::try_from("nft.catchlabs.near").unwrap()
    }
    pub fn marketplace() -> ValidAccountId {
        ValidAccountId::try_from("marketplace.near").unwrap()
    }

    pub fn get_context(predecessor_account_id: AccountId, attached_deposit: Balance) -> VMContext {
        VMContext {
            current_account_id: "nft.catchlabs.near".to_string(),
            signer_account_id: predecessor_account_id.clone(),
            signer_account_pk: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000 * 10u128.pow(24),
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    pub fn create_contract() -> Contract {
        todo!()
    }
}
