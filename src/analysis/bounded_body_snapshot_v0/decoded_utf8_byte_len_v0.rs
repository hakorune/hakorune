//! Environment-independent UTF-8 byte-length leaf for SnapshotV0.
//!
//! This is an analysis/internal operation, not a public string-method surface.
//! It intentionally has no aliases and does not consult string indexing mode.

pub(crate) struct DecodedUtf8ByteLenV0;

impl DecodedUtf8ByteLenV0 {
    pub(crate) fn count(value: &str) -> usize {
        value.as_bytes().len()
    }
}
