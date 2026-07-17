use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::core_method_result_kind::CoreMethodContractResultRowV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetV1;
use crate::mir::source_core_receiver::SourceCoreReceiverFactV1;

/// Exact evidence retained by one disconnected source-call result row.
///
/// Same-module targets are borrowed from the sealed target catalog rather
/// than cloned into a second target authority.  The Core receiver fact can
/// enter only through the private constructor used after bounded source proof.
#[derive(Debug, Clone)]
pub(crate) enum VerifiedCallableResultEvidenceV1<'target> {
    SameModuleStatic {
        source_target: &'target VerifiedSourceStaticCallTargetV1,
        callee_required_i64_arguments: Box<[u32]>,
    },
    CoreStringMethod {
        receiver_fact: SourceCoreReceiverFactV1,
        contract: &'static CoreMethodContractResultRowV1,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct VerifiedCallableResultCallSiteV1<'target> {
    evidence: VerifiedCallableResultEvidenceV1<'target>,
    required_i64_arguments: Box<[u32]>,
}

impl<'target> VerifiedCallableResultCallSiteV1<'target> {
    pub(super) fn same_module_static(
        source_target: &'target VerifiedSourceStaticCallTargetV1,
        callee_required_i64_arguments: Box<[u32]>,
        required_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            evidence: VerifiedCallableResultEvidenceV1::SameModuleStatic {
                source_target,
                callee_required_i64_arguments,
            },
            required_i64_arguments,
        }
    }

    pub(super) fn core_string_method(
        receiver_fact: SourceCoreReceiverFactV1,
        contract: &'static CoreMethodContractResultRowV1,
    ) -> Self {
        Self {
            evidence: VerifiedCallableResultEvidenceV1::CoreStringMethod {
                receiver_fact,
                contract,
            },
            required_i64_arguments: Box::new([]),
        }
    }

    pub(crate) const fn evidence(&self) -> &VerifiedCallableResultEvidenceV1<'target> {
        &self.evidence
    }

    pub(crate) fn required_i64_arguments(&self) -> &[u32] {
        &self.required_i64_arguments
    }

    pub(crate) fn static_target_key(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        match &self.evidence {
            VerifiedCallableResultEvidenceV1::SameModuleStatic { source_target, .. } => {
                Some(source_target.target())
            }
            VerifiedCallableResultEvidenceV1::CoreStringMethod { .. } => None,
        }
    }

    pub(super) fn semantically_matches(&self, other: &Self) -> bool {
        self.required_i64_arguments == other.required_i64_arguments
            && match (&self.evidence, &other.evidence) {
                (
                    VerifiedCallableResultEvidenceV1::SameModuleStatic {
                        source_target: left_target,
                        callee_required_i64_arguments: left_required,
                    },
                    VerifiedCallableResultEvidenceV1::SameModuleStatic {
                        source_target: right_target,
                        callee_required_i64_arguments: right_required,
                    },
                ) => std::ptr::eq(*left_target, *right_target) && left_required == right_required,
                (
                    VerifiedCallableResultEvidenceV1::CoreStringMethod {
                        receiver_fact: left_fact,
                        contract: left_contract,
                    },
                    VerifiedCallableResultEvidenceV1::CoreStringMethod {
                        receiver_fact: right_fact,
                        contract: right_contract,
                    },
                ) => left_fact == right_fact && std::ptr::eq(*left_contract, *right_contract),
                _ => false,
            }
    }
}

pub(super) type CallableResultCallRowsV1<'target> = std::collections::BTreeMap<
    (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
    VerifiedCallableResultCallSiteV1<'target>,
>;
