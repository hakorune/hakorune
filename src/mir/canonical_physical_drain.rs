//! The insert-only disposition carried by normal module publication.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalInsertedDispositionV1 {
    _private: (),
}

impl CanonicalInsertedDispositionV1 {
    pub(crate) const fn from_canonical_source() -> Self {
        Self { _private: () }
    }
}
