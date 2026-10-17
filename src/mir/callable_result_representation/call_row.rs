use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::core_method_result_kind::CoreMethodContractRowV2;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetV1;
use crate::mir::source_core_receiver::SourceCoreReceiverFactV1;

use super::VerifiedCallableResultRepresentationV1;

/// Exact evidence retained by one disconnected source-call result row.
///
/// Same-module targets are borrowed from the sealed target catalog rather
/// than cloned into a second target authority.  The Core receiver fact can
/// enter only through the private constructor used after bounded source proof.
#[derive(Debug, Clone)]
pub(crate) enum CallableResultEvidenceV1<Target> {
    SameModuleStatic {
        source_target: Target,
        result_representation: VerifiedCallableResultRepresentationV1,
        callee_required_i64_arguments: Box<[u32]>,
    },
    CoreStringMethod {
        receiver_fact: SourceCoreReceiverFactV1,
        contract: &'static CoreMethodContractRowV2,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct CallableResultCallSiteV1<Target> {
    evidence: CallableResultEvidenceV1<Target>,
    required_i64_arguments: Box<[u32]>,
}

/// Borrowed construction rows and passive owned rows share one vocabulary.
/// Owning a target projection does not issue target or execution authority.
pub(crate) type VerifiedCallableResultEvidenceV1<'target> =
    CallableResultEvidenceV1<&'target VerifiedSourceStaticCallTargetV1>;
pub(crate) type VerifiedCallableResultCallSiteV1<'target> =
    CallableResultCallSiteV1<&'target VerifiedSourceStaticCallTargetV1>;
pub(crate) type OwnedCallableResultCallSiteV1 =
    CallableResultCallSiteV1<VerifiedSourceStaticCallTargetV1>;

impl<Target: std::borrow::Borrow<VerifiedSourceStaticCallTargetV1>>
    CallableResultCallSiteV1<Target>
{
    pub(crate) const fn evidence(&self) -> &CallableResultEvidenceV1<Target> {
        &self.evidence
    }

    pub(crate) fn required_i64_arguments(&self) -> &[u32] {
        &self.required_i64_arguments
    }

    pub(crate) fn static_target_key(&self) -> Option<&CanonicalSameModuleCallableKeyV1> {
        match &self.evidence {
            CallableResultEvidenceV1::SameModuleStatic { source_target, .. } => {
                Some(std::borrow::Borrow::borrow(source_target).target())
            }
            CallableResultEvidenceV1::CoreStringMethod { .. } => None,
        }
    }

    pub(crate) fn result_representation(&self) -> VerifiedCallableResultRepresentationV1 {
        match &self.evidence {
            CallableResultEvidenceV1::SameModuleStatic {
                result_representation,
                ..
            } => result_representation.clone(),
            CallableResultEvidenceV1::CoreStringMethod { contract, .. } => {
                match contract.result_kind {
                    crate::mir::core_method_result_kind::CoreMethodResultKindV1::I64Value => {
                        VerifiedCallableResultRepresentationV1::ExactI64
                    }
                    crate::mir::core_method_result_kind::CoreMethodResultKindV1::StringValue => {
                        VerifiedCallableResultRepresentationV1::ExactString
                    }
                    _ => unreachable!("[freeze:contract][callable-result/core-result-kind]"),
                }
            }
        }
    }
}

impl<'target> VerifiedCallableResultCallSiteV1<'target> {
    pub(super) fn same_module_static(
        source_target: &'target VerifiedSourceStaticCallTargetV1,
        result_representation: VerifiedCallableResultRepresentationV1,
        callee_required_i64_arguments: Box<[u32]>,
        required_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            evidence: VerifiedCallableResultEvidenceV1::SameModuleStatic {
                source_target,
                result_representation,
                callee_required_i64_arguments,
            },
            required_i64_arguments,
        }
    }

    pub(super) fn core_string_method(
        receiver_fact: SourceCoreReceiverFactV1,
        contract: &'static CoreMethodContractRowV2,
    ) -> Self {
        Self {
            evidence: VerifiedCallableResultEvidenceV1::CoreStringMethod {
                receiver_fact,
                contract,
            },
            required_i64_arguments: Box::new([]),
        }
    }

    /// Borrowed exact target evidence for consumers that must co-seal this
    /// source result row with one target-catalog allocation.
    pub(super) fn same_module_static_evidence(
        &self,
    ) -> Option<(&VerifiedSourceStaticCallTargetV1, &[u32])> {
        match &self.evidence {
            VerifiedCallableResultEvidenceV1::SameModuleStatic {
                source_target,
                callee_required_i64_arguments,
                ..
            } => Some((source_target, callee_required_i64_arguments)),
            VerifiedCallableResultEvidenceV1::CoreStringMethod { .. } => None,
        }
    }

    /// Consume the SAME solver row. Only the already-sealed target is
    /// projected into owned storage; both dependency lists move unchanged.
    pub(super) fn into_owned(self) -> OwnedCallableResultCallSiteV1 {
        let evidence = match self.evidence {
            VerifiedCallableResultEvidenceV1::SameModuleStatic {
                source_target,
                result_representation,
                callee_required_i64_arguments,
            } => CallableResultEvidenceV1::SameModuleStatic {
                source_target: source_target.clone(),
                result_representation,
                callee_required_i64_arguments,
            },
            VerifiedCallableResultEvidenceV1::CoreStringMethod {
                receiver_fact,
                contract,
            } => CallableResultEvidenceV1::CoreStringMethod {
                receiver_fact,
                contract,
            },
        };
        CallableResultCallSiteV1 {
            evidence,
            required_i64_arguments: self.required_i64_arguments,
        }
    }

    pub(super) fn semantically_matches(&self, other: &Self) -> bool {
        self.required_i64_arguments == other.required_i64_arguments
            && match (&self.evidence, &other.evidence) {
                (
                    VerifiedCallableResultEvidenceV1::SameModuleStatic {
                        source_target: left_target,
                        result_representation: left_representation,
                        callee_required_i64_arguments: left_required,
                    },
                    VerifiedCallableResultEvidenceV1::SameModuleStatic {
                        source_target: right_target,
                        result_representation: right_representation,
                        callee_required_i64_arguments: right_required,
                    },
                ) => {
                    std::ptr::eq(*left_target, *right_target)
                        && left_representation == right_representation
                        && left_required == right_required
                }
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
