//! HEADERPORT0-I0-ROOTBATCH0-S0: Main/condition_fn root batch vocabulary.
//!
//! The batch is a disconnected, non-Clone owner.  It prepares all root
//! admissions before a collector is borrowed; collection and module drain are
//! deliberately outside this slice.

use super::main_pending_draft::PendingMainDraftV1;
use super::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use super::module_invocation_drain::ConditionFnPolicyV1;
use crate::mir::MirFunction;
use super::root_body_completion::CompletedRootBodyV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum RootDraftBatchErrorV1 {
    MissingConditionFn,
    UnexpectedConditionFn,
    ConditionSymbolMismatch { actual: String },
    ConditionArityMismatch { actual: usize },
    MainIdentityMismatch,
    DuplicateAdmissionSymbol { symbol: String },
    MissingRootBody,
}

impl std::fmt::Display for RootDraftBatchErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][root_batch] {self:?}")
    }
}

impl std::error::Error for RootDraftBatchErrorV1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct RootDraftAdmissionPlanV1 {
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: DraftPublicationPolicyV1,
}

impl RootDraftAdmissionPlanV1 {
    pub(in crate::mir::builder) fn key(&self) -> &FunctionDraftKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(in crate::mir::builder) fn arity(&self) -> usize {
        self.arity
    }

    pub(in crate::mir::builder) fn policy(&self) -> DraftPublicationPolicyV1 {
        self.policy
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PendingConditionFnDraftV1 {
    draft: MirFunction,
    _seal: PendingConditionFnDraftSealV1,
}

#[derive(Debug)]
struct PendingConditionFnDraftSealV1;

impl PendingConditionFnDraftV1 {
    pub(in crate::mir::builder) fn new(draft: MirFunction) -> Result<Self, RootDraftBatchErrorV1> {
        if draft.signature.name != "condition_fn" {
            return Err(RootDraftBatchErrorV1::ConditionSymbolMismatch {
                actual: draft.signature.name,
            });
        }
        let actual = draft.signature.params.len();
        if actual != 1 {
            return Err(RootDraftBatchErrorV1::ConditionArityMismatch { actual });
        }
        Ok(Self {
            draft,
            _seal: PendingConditionFnDraftSealV1,
        })
    }

    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(super) fn into_draft(self) -> MirFunction {
        self.draft
    }
}

/// Prepared root batch.  It owns the root and optional synthetic draft, but
/// no collector reference, module map, Builder, or publication capability.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRootDraftBatchV1 {
    main: PendingMainDraftV1,
    root_body: Option<CompletedRootBodyV1>,
    condition_fn: Option<PendingConditionFnDraftV1>,
    admissions: Box<[RootDraftAdmissionPlanV1]>,
    policy: ConditionFnPolicyV1,
    _seal: PreparedRootDraftBatchSealV1,
}

#[derive(Debug)]
struct PreparedRootDraftBatchSealV1;

impl PreparedRootDraftBatchV1 {
    pub(in crate::mir::builder) fn prepare(
        main: PendingMainDraftV1,
        condition_fn: Option<MirFunction>,
        policy: ConditionFnPolicyV1,
    ) -> Result<Self, RootDraftBatchErrorV1> {
        let mut main = main;
        let root_body = main
            .take_root_body()
            .ok_or(RootDraftBatchErrorV1::MissingRootBody)?;
        let identity = main.identity();
        if identity.symbol() != "main" || identity.arity() != 0 {
            return Err(RootDraftBatchErrorV1::MainIdentityMismatch);
        }

        let pending_condition = match (policy, condition_fn) {
            (ConditionFnPolicyV1::Required, None) => {
                return Err(RootDraftBatchErrorV1::MissingConditionFn)
            }
            (ConditionFnPolicyV1::Forbidden, Some(_)) => {
                return Err(RootDraftBatchErrorV1::UnexpectedConditionFn)
            }
            (_, Some(draft)) => Some(PendingConditionFnDraftV1::new(draft)?),
            (_, None) => None,
        };

        let mut admissions = vec![RootDraftAdmissionPlanV1 {
            key: FunctionDraftKeyV1::Main,
            symbol: "main".into(),
            arity: 0,
            policy: DraftPublicationPolicyV1::LegacyReplaceWholePair,
        }];
        if pending_condition.is_some() {
            admissions.push(RootDraftAdmissionPlanV1 {
                key: FunctionDraftKeyV1::SyntheticConditionFn,
                symbol: "condition_fn".into(),
                arity: 1,
                policy: DraftPublicationPolicyV1::CanonicalRejectDuplicate,
            });
        }

        let mut seen = std::collections::BTreeSet::new();
        for admission in &admissions {
            if !seen.insert(admission.symbol.clone()) {
                return Err(RootDraftBatchErrorV1::DuplicateAdmissionSymbol {
                    symbol: admission.symbol.to_string(),
                });
            }
        }

        Ok(Self {
            main,
            root_body: Some(root_body),
            condition_fn: pending_condition,
            admissions: admissions.into_boxed_slice(),
            policy,
            _seal: PreparedRootDraftBatchSealV1,
        })
    }

    pub(in crate::mir::builder) fn main(&self) -> &PendingMainDraftV1 {
        &self.main
    }

    pub(in crate::mir::builder) fn root_body(&self) -> Option<&CompletedRootBodyV1> {
        self.root_body.as_ref()
    }

    pub(in crate::mir::builder) fn condition_fn(&self) -> Option<&PendingConditionFnDraftV1> {
        self.condition_fn.as_ref()
    }

    pub(in crate::mir::builder) fn admissions(&self) -> &[RootDraftAdmissionPlanV1] {
        &self.admissions
    }

    pub(in crate::mir::builder) fn policy(&self) -> ConditionFnPolicyV1 {
        self.policy
    }

    pub(in crate::mir::builder) fn take_root_body(&mut self) -> Option<CompletedRootBodyV1> {
        self.root_body.take()
    }

    /// Consume the already-validated root batch into exact physical entries.
    /// No caller can add, remove, or reorder an admission at this boundary.
    pub(super) fn into_collector_entries(self) -> Box<[RootDraftCollectorEntryV1]> {
        let Self {
            main,
            condition_fn,
            admissions,
            root_body: _,
            policy: _,
            _seal: _,
        } = self;
        let mut drafts = vec![main.into_draft()];
        if let Some(condition_fn) = condition_fn {
            drafts.push(condition_fn.into_draft());
        }
        admissions
            .into_vec()
            .into_iter()
            .zip(drafts)
            .map(|(admission, draft)| RootDraftCollectorEntryV1 { admission, draft })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

/// Consuming bridge used only by the collector-owned atomic batch preflight.
#[derive(Debug)]
pub(super) struct RootDraftCollectorEntryV1 {
    admission: RootDraftAdmissionPlanV1,
    draft: MirFunction,
}

impl RootDraftCollectorEntryV1 {
    pub(super) fn into_parts(
        self,
    ) -> (
        FunctionDraftKeyV1,
        String,
        usize,
        DraftPublicationPolicyV1,
        MirFunction,
    ) {
        (
            self.admission.key,
            self.admission.symbol.into_string(),
            self.admission.arity,
            self.admission.policy,
            self.draft,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::main_pending_draft::{
        MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    };
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirModule, MirType};

    fn main_draft() -> PendingMainDraftV1 {
        let body = RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::NoValue)
            .unwrap();
        let request = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), body, false);
        let headers = MirModule::new("headers".into());
        request
            .finish(
                crate::mir::MirFunction::new(
                    FunctionSignature {
                        name: "main".into(),
                        params: Vec::new(),
                        return_type: MirType::Void,
                        effects: EffectMask::PURE,
                    },
                    BasicBlockId::new(0),
                ),
                MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
            )
            .unwrap()
    }

    fn condition_fn() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "condition_fn".into(),
                params: vec![MirType::Integer],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn required_condition_fn_prepares_one_atomic_root_batch() {
        let batch = PreparedRootDraftBatchV1::prepare(
            main_draft(),
            Some(condition_fn()),
            ConditionFnPolicyV1::Required,
        )
        .unwrap();
        assert_eq!(batch.admissions().len(), 2);
        assert_eq!(batch.admissions()[0].key(), &FunctionDraftKeyV1::Main);
        assert_eq!(batch.admissions()[1].symbol(), "condition_fn");
        assert!(batch.condition_fn().is_some());
    }

    #[test]
    fn optional_missing_and_forbidden_present_are_explicit() {
        let optional =
            PreparedRootDraftBatchV1::prepare(main_draft(), None, ConditionFnPolicyV1::Optional)
                .unwrap();
        assert_eq!(optional.admissions().len(), 1);
        assert_eq!(optional.policy(), ConditionFnPolicyV1::Optional);

        assert_eq!(
            PreparedRootDraftBatchV1::prepare(
                main_draft(),
                Some(condition_fn()),
                ConditionFnPolicyV1::Forbidden,
            )
            .unwrap_err(),
            RootDraftBatchErrorV1::UnexpectedConditionFn
        );
    }

    #[test]
    fn malformed_condition_fn_is_rejected_before_batch_product() {
        let malformed = crate::mir::MirFunction::new(
            FunctionSignature {
                name: "condition_fn".into(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        assert_eq!(
            PreparedRootDraftBatchV1::prepare(
                main_draft(),
                Some(malformed),
                ConditionFnPolicyV1::Required,
            )
            .unwrap_err(),
            RootDraftBatchErrorV1::ConditionArityMismatch { actual: 0 }
        );
    }
}
