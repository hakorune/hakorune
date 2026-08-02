//! Migration-only evidence for Generic post-effect retry debt.
//!
//! This receipt records which legacy composer/result stage produced debt. It
//! is not a semantic route policy, recipe input, or physicalization command.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LegacyGenericComposerV1 {
    V0,
    V1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LegacyGenericResultKindV1 {
    StrictShadowLowerReturnedNone,
    ReleaseVerifierRejected,
    ReleaseLowerReturnedNone,
    ReleaseLowerFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LegacyComposerResultReceiptV1 {
    composer: LegacyGenericComposerV1,
    result: LegacyGenericResultKindV1,
}

impl LegacyComposerResultReceiptV1 {
    pub(crate) const fn new(
        composer: LegacyGenericComposerV1,
        result: LegacyGenericResultKindV1,
    ) -> Self {
        Self { composer, result }
    }

    pub(crate) const fn composer(self) -> LegacyGenericComposerV1 {
        self.composer
    }

    pub(crate) const fn result_kind(self) -> LegacyGenericResultKindV1 {
        self.result
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LegacyComposerResultReceiptV1, LegacyGenericComposerV1, LegacyGenericResultKindV1,
    };

    #[test]
    fn generic_receipt_keeps_composer_and_result_stage_distinct() {
        let results = [
            LegacyGenericResultKindV1::StrictShadowLowerReturnedNone,
            LegacyGenericResultKindV1::ReleaseVerifierRejected,
            LegacyGenericResultKindV1::ReleaseLowerReturnedNone,
            LegacyGenericResultKindV1::ReleaseLowerFailed,
        ];
        for composer in [LegacyGenericComposerV1::V0, LegacyGenericComposerV1::V1] {
            for result in results {
                let receipt = LegacyComposerResultReceiptV1::new(composer, result);
                assert_eq!(receipt.composer(), composer);
                assert_eq!(receipt.result_kind(), result);
            }
        }
    }
}
