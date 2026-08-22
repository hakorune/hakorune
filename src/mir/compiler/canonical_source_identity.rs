//! Canonical source identity issued once at the normal-file read boundary.
//!
//! The digest is intentionally byte-based and AST-free.  Path/display names
//! remain diagnostics only; downstream source-plan owners carry this opaque
//! value without re-reading or re-hashing the source.

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CanonicalSourceBytesDigestV1([u8; 32]);

impl CanonicalSourceBytesDigestV1 {
    pub(crate) fn from_utf8_bytes(bytes: &[u8]) -> Self {
        let digest: [u8; 32] = Sha256::digest(bytes).into();
        Self(digest)
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalSourceBytesDigestV1;

    #[test]
    fn identical_bytes_share_one_digest() {
        assert_eq!(
            CanonicalSourceBytesDigestV1::from_utf8_bytes(b"42"),
            CanonicalSourceBytesDigestV1::from_utf8_bytes(b"42")
        );
    }

    #[test]
    fn one_byte_drift_changes_digest() {
        assert_ne!(
            CanonicalSourceBytesDigestV1::from_utf8_bytes(b"42"),
            CanonicalSourceBytesDigestV1::from_utf8_bytes(b"43")
        );
    }
}
