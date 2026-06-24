/// Application constants to avoid magic numbers.

/// Default timestamp value used for legacy items without creation date
pub const DEFAULT_TIMESTAMP: u64 = 0;

/// Maximum allowed length for command names
pub const MAX_NAME_LENGTH: usize = 100;

/// Maximum allowed length for commands
pub const MAX_COMMAND_LENGTH: usize = 1000;

/// Maximum allowed length for descriptions
pub const MAX_DESC_LENGTH: usize = 500;

/// Default status message shown in normal mode
pub const DEFAULT_STATUS: &str = "Ready";

/// File name for the command vault storage
pub const VAULT_FILENAME: &str = ".cmd-vault.json";

/// Default search mode status message
pub const SEARCH_MODE_STATUS: &str = "SEARCH MODE ACTIVE";

/// Default form mode status message  
pub const FORM_MODE_STATUS: &str = "FORM MODE: Complete fields architecture setup";