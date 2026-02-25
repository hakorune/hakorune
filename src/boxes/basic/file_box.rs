//! DEPRECATED: Use `crate::boxes::file::FileBox` instead
//!
//! This module is kept for backward compatibility only.
//! All new code should use the SSOT provider-based FileBox implementation.

#![deprecated(
    since = "0.1.0",
    note = "Use crate::boxes::file::FileBox instead. This module will be removed in a future version."
)]

// Re-export the new FileBox implementation for backward compatibility
pub use crate::boxes::file::FileBox;
