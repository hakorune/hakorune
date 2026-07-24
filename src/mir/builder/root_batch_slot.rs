//! HEADERPORT0-I0-ROOTBATCH0-S0: root admission identity SSOT.
//!
//! The root batch has two physical slots with different duplicate policy.
//! Keep their key, symbol, arity, and publication disposition together so
//! callers cannot drift by spelling one of those facts independently.

use super::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootBatchSlotV1 {
    Main,
    RequiredCondition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct RawRootBatchSlotContractV1 {
    slot: RawRootBatchSlotV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
}

impl RawRootBatchSlotV1 {
    /// Build the complete identity contract for this slot.
    ///
    /// This is the only constructor for the root Main/condition admission
    /// vocabulary.  Physical preparation remains a later ROOTBATCH0 row.
    pub(in crate::mir::builder) fn contract(self) -> RawRootBatchSlotContractV1 {
        match self {
            Self::Main => RawRootBatchSlotContractV1 {
                slot: self,
                key: FunctionDraftKeyV1::Main,
                symbol: "main".into(),
                arity: 0,
                policy: DraftPublicationPolicyV1::LegacyReplaceWholePair,
            },
            Self::RequiredCondition => RawRootBatchSlotContractV1 {
                slot: self,
                key: FunctionDraftKeyV1::SyntheticConditionFn,
                symbol: "condition_fn".into(),
                arity: 1,
                policy: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            },
        }
    }
}

impl RawRootBatchSlotContractV1 {
    pub(in crate::mir::builder) const fn slot(&self) -> RawRootBatchSlotV1 {
        self.slot
    }

    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) const fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir::builder) const fn policy(&self) -> DraftPublicationPolicyV1 {
        self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_slots_seal_distinct_exact_identities() {
        let main = RawRootBatchSlotV1::Main.contract();
        assert_eq!(main.slot(), RawRootBatchSlotV1::Main);
        assert_eq!(main.key(), &FunctionDraftKeyV1::Main);
        assert_eq!(main.symbol(), "main");
        assert_eq!(main.arity(), 0);
        assert_eq!(
            main.policy(),
            DraftPublicationPolicyV1::LegacyReplaceWholePair
        );

        let condition = RawRootBatchSlotV1::RequiredCondition.contract();
        assert_eq!(condition.slot(), RawRootBatchSlotV1::RequiredCondition);
        assert_eq!(condition.key(), &FunctionDraftKeyV1::SyntheticConditionFn);
        assert_eq!(condition.symbol(), "condition_fn");
        assert_eq!(condition.arity(), 1);
        assert_eq!(
            condition.policy(),
            DraftPublicationPolicyV1::CanonicalRejectDuplicate
        );
    }
}
