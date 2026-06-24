use serde::{Deserialize, Serialize};

/// Represents a single saved command entry in the vault.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandItem {
    pub name: String,
    pub command: String,
    pub desc: String,
    /// Unix timestamp (seconds) when the command was added. Optional for backwards compat.
    #[serde(default)]
    pub created_at: u64,
}
