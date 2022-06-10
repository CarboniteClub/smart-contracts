use std::fmt;

use crate::*;

pub const NFT_STANDARD_NAME: &str = "nep171";

pub const NFT_METADATA_SPEC: &str = "1.0.0";

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    NftMint(Vec<NftMintLog>),
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintLog {
    pub owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl NftMintLog {
    pub fn emit(mint_logs: Vec<NftMintLog>) {
        let event_log = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftMint(mint_logs),
        };

        env::log(event_log.to_string().as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{test_utils, testing_env};
    #[test]
    fn batch_mint_events() {
        testing_env!(get_context(carol().to_string(), 0));

        let expected = r#"EVENT_JSON:{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{"owner_id":"foundation.near","token_ids":["aurora","proximitylabs"]},{"owner_id":"user1.near","token_ids":["meme"]}]}"#;

        let mint_logs = vec![
            NftMintLog {
                owner_id: "foundation.near".to_owned(),
                token_ids: vec!["aurora".to_string(), "proximitylabs".to_string()],
                memo: None,
            },
            NftMintLog {
                owner_id: "user1.near".to_owned(),
                token_ids: vec!["meme".to_string()],
                memo: None,
            },
        ];
        NftMintLog::emit(mint_logs);
        let logs = &test_utils::get_logs()[0];
        assert_eq!(expected, logs);
    }
}
