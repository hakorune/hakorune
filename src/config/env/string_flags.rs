//! String-related environment flags.

use super::env_bool;

/// NYASH_STR_CP=1: use Unicode code point indexing for string operations.
/// Default: OFF (byte indexing).
pub fn string_codepoint_mode() -> bool {
    env_bool("NYASH_STR_CP")
}
