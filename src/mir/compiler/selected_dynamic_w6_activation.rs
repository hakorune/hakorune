//! Receipt-consumption witness issued by the artifact root validator.

#[derive(Debug)]
pub(crate) struct StaticArtifactReceiptConsumedFenceV1 {
    _seal: StaticArtifactReceiptConsumedFenceSealV1,
}

#[derive(Debug)]
struct StaticArtifactReceiptConsumedFenceSealV1;

impl StaticArtifactReceiptConsumedFenceV1 {
    /// The root receipt validator is the sole issuer of this physical fence.
    /// A guard keeps this constructor's production caller count at one.
    pub(crate) fn issue_from_root_validator() -> Self {
        Self {
            _seal: StaticArtifactReceiptConsumedFenceSealV1,
        }
    }
}
