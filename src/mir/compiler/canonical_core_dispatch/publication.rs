//! Shared publication boundary for verified canonical source-entry candidates.
//!
//! Family transactions seal their own source semantics. This module only
//! projects those retained facts into the neutral published invocation; it
//! never scans AST, Return instructions, signatures, or module symbols.

use super::{
    CompletedCanonicalCoreSourceEntryCandidateV1, CompletedCanonicalCoreSourceEntryFamilyV1,
    NormalSourcePlanReceiptV1, VerifiedCanonicalCoreSourcePlanAdmissionV1,
};
use crate::mir::builder::{
    CompletedNormalMainModuleCandidateV1, CompletedNormalMainModuleEvidenceV1,
    CompletedNormalScriptModuleCandidateV1, CompletedNormalScriptModuleEvidenceV1,
    NormalMainCandidateVerificationReceiptV1, NormalScriptCandidateVerificationReceiptV1,
    VerifiedScriptEntryResultContractV1,
};
use crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkResultV1;
use crate::mir::compiler::source_entry_published_invocation::{
    CanonicalPublishedSourceEntryMembershipV1, PendingPublishedSourceEntryTargetV1,
    PublishedSourceEntryInvocationV1, PublishedSourceEntryMembershipV1,
    PublishedSourceEntryResultContractV1, PublishedSourceEntryTargetErrorV1,
    PublishedUnitPhysicalContractV1, VerifiedPublishedSourceEntryTargetV1,
};
use crate::mir::compiler::source_entry_result::UnitOriginV1;
use crate::mir::function::MirModule;
use crate::mir::resolved_control_flow::FunctionUnitOriginV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalPublishedFamilyKindV1 {
    Main,
    Script,
}

/// Complete family-specific evidence retained after canonical publication.
#[derive(Debug)]
pub(in crate::mir) enum PublishedCanonicalFamilyEvidenceV1 {
    Main {
        evidence: CompletedNormalMainModuleEvidenceV1,
        verification: NormalMainCandidateVerificationReceiptV1,
    },
    Script {
        evidence: CompletedNormalScriptModuleEvidenceV1,
        verification: NormalScriptCandidateVerificationReceiptV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalPublicationVerificationReceiptV1 {
    family: CanonicalPublishedFamilyKindV1,
    candidate_function_count: usize,
    _seal: CanonicalPublicationVerificationReceiptSealV1,
}

#[derive(Debug)]
struct CanonicalPublicationVerificationReceiptSealV1;

#[derive(Debug)]
struct PreparedCanonicalPublishedOwnerV1 {
    module: MirModule,
    family: PublishedCanonicalFamilyEvidenceV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    source_receipt: NormalSourcePlanReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedCanonicalSourceEntryPublicationV1 {
    owner: PreparedCanonicalPublishedOwnerV1,
    target: VerifiedPublishedSourceEntryTargetV1,
    result: PublishedSourceEntryResultContractV1,
    membership: PublishedSourceEntryMembershipV1,
    verification: CanonicalPublicationVerificationReceiptV1,
    _seal: PreparedCanonicalSourceEntryPublicationSealV1,
}

#[derive(Debug)]
struct PreparedCanonicalSourceEntryPublicationSealV1;

/// Canonical publication retains the complete candidate evidence exactly once.
#[derive(Debug)]
pub(in crate::mir) struct PublishedCanonicalSourceEntryOwnerV1 {
    module: MirModule,
    family: PublishedCanonicalFamilyEvidenceV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    source_receipt: NormalSourcePlanReceiptV1,
    verification: CanonicalPublicationVerificationReceiptV1,
    _seal: PublishedCanonicalSourceEntryOwnerSealV1,
}

#[derive(Debug)]
struct PublishedCanonicalSourceEntryOwnerSealV1;

pub(in crate::mir) type PublishedCanonicalSourceEntryInvocationV1 =
    PublishedSourceEntryInvocationV1<PublishedCanonicalSourceEntryOwnerV1>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalSourceEntryPublicationStageV1 {
    CandidateEvidence,
    TargetProjection,
    ResultProjection,
    MembershipProjection,
    Pairing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalSourceEntryPublicationErrorV1 {
    MainEvidenceMismatch,
    ScriptEvidenceMismatch,
    Target(PublishedSourceEntryTargetErrorV1),
}

#[derive(Debug)]
pub(crate) struct RejectedCanonicalSourceEntryPublicationV1 {
    owner: CompletedCanonicalCoreSourceEntryCandidateV1,
    stage: CanonicalSourceEntryPublicationStageV1,
    cause: CanonicalSourceEntryPublicationErrorV1,
}

impl CompletedCanonicalCoreSourceEntryCandidateV1 {
    pub(crate) fn prepare_canonical_publication(
        self,
    ) -> Result<PreparedCanonicalSourceEntryPublicationV1, RejectedCanonicalSourceEntryPublicationV1>
    {
        prepare(self)
    }

    #[cfg(test)]
    pub(crate) fn canonical_publication_summary_for_test(
        self,
    ) -> Result<CanonicalPublicationSummaryForTestV1, RejectedCanonicalSourceEntryPublicationV1>
    {
        let published = self.prepare_canonical_publication()?.commit();
        Ok(published.publication_summary_for_test())
    }
}

impl PreparedCanonicalSourceEntryPublicationV1 {
    /// Every fallible pairing check has completed before this move-only commit.
    pub(crate) fn commit(self) -> PublishedCanonicalSourceEntryInvocationV1 {
        let Self {
            owner,
            target,
            result,
            membership,
            verification,
            _seal: _,
        } = self;
        let owner = PublishedCanonicalSourceEntryOwnerV1 {
            module: owner.module,
            family: owner.family,
            admission: owner.admission,
            source_receipt: owner.source_receipt,
            verification,
            _seal: PublishedCanonicalSourceEntryOwnerSealV1,
        };
        PublishedSourceEntryInvocationV1::from_verified_parts(owner, target, result, membership)
    }
}

impl PublishedCanonicalSourceEntryOwnerV1 {
    pub(in crate::mir) fn module(&self) -> &MirModule {
        &self.module
    }

    pub(in crate::mir) const fn family_kind(&self) -> CanonicalPublishedFamilyKindV1 {
        self.verification.family
    }
}

#[cfg(test)]
impl PublishedCanonicalSourceEntryInvocationV1 {
    pub(crate) fn publication_summary_for_test(&self) -> CanonicalPublicationSummaryForTestV1 {
        CanonicalPublicationSummaryForTestV1 {
            target_symbol: self.target().symbol().to_owned(),
            target_arity: self.target().arity(),
            result_kind: match self.result() {
                PublishedSourceEntryResultContractV1::Unit { .. } => "unit",
                PublishedSourceEntryResultContractV1::Integer => "integer",
                PublishedSourceEntryResultContractV1::Bool => "bool",
                PublishedSourceEntryResultContractV1::Float => "float",
                PublishedSourceEntryResultContractV1::String => "string",
            },
            family: match self.membership() {
                PublishedSourceEntryMembershipV1::Raw { .. } => unreachable!("canonical owner"),
                PublishedSourceEntryMembershipV1::Canonical(
                    CanonicalPublishedSourceEntryMembershipV1::Main { .. },
                ) => "main",
                PublishedSourceEntryMembershipV1::Canonical(
                    CanonicalPublishedSourceEntryMembershipV1::Script,
                ) => "script",
            },
        }
    }
}

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct CanonicalPublicationSummaryForTestV1 {
    pub(crate) target_symbol: String,
    pub(crate) target_arity: usize,
    pub(crate) result_kind: &'static str,
    pub(crate) family: &'static str,
}

impl RejectedCanonicalSourceEntryPublicationV1 {
    pub(in crate::mir) const fn stage(&self) -> CanonicalSourceEntryPublicationStageV1 {
        self.stage
    }

    pub(in crate::mir) fn cause(&self) -> &CanonicalSourceEntryPublicationErrorV1 {
        &self.cause
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

fn prepare(
    candidate: CompletedCanonicalCoreSourceEntryCandidateV1,
) -> Result<PreparedCanonicalSourceEntryPublicationV1, RejectedCanonicalSourceEntryPublicationV1> {
    let CompletedCanonicalCoreSourceEntryCandidateV1 {
        family,
        admission,
        receipt,
        _seal: _,
    } = candidate;
    match family {
        CompletedCanonicalCoreSourceEntryFamilyV1::Main(candidate) => {
            prepare_main(candidate, admission, receipt)
        }
        CompletedCanonicalCoreSourceEntryFamilyV1::Script(candidate) => {
            prepare_script(candidate, admission, receipt)
        }
    }
}

fn prepare_main(
    candidate: CompletedNormalMainModuleCandidateV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
) -> Result<PreparedCanonicalSourceEntryPublicationV1, RejectedCanonicalSourceEntryPublicationV1> {
    let target = match PendingPublishedSourceEntryTargetV1::new(
        candidate.physical_symbol(),
        candidate.physical_arity(),
    )
    .seal()
    {
        Ok(target) => target,
        Err(rejected) => {
            let cause = rejected.error().clone();
            rejected.discard();
            return Err(reject_main(
                candidate,
                admission,
                receipt,
                CanonicalSourceEntryPublicationStageV1::TargetProjection,
                CanonicalSourceEntryPublicationErrorV1::Target(cause),
            ));
        }
    };
    if candidate.source_owner() != candidate.entry_source_owner()
        || candidate.schema_row_count() != 2
        || candidate.verification_count() != 2
        || candidate.module().functions.len() != 2
    {
        return Err(reject_main(
            candidate,
            admission,
            receipt,
            CanonicalSourceEntryPublicationStageV1::Pairing,
            CanonicalSourceEntryPublicationErrorV1::MainEvidenceMismatch,
        ));
    }
    let result = project_main_result(candidate.result());
    let membership = PublishedSourceEntryMembershipV1::Canonical(
        CanonicalPublishedSourceEntryMembershipV1::Main {
            source_owner: candidate.source_owner(),
        },
    );
    let (module, evidence, verification) = candidate.into_publication_parts();
    Ok(PreparedCanonicalSourceEntryPublicationV1 {
        owner: PreparedCanonicalPublishedOwnerV1 {
            module,
            family: PublishedCanonicalFamilyEvidenceV1::Main {
                evidence,
                verification,
            },
            admission,
            source_receipt: receipt,
        },
        target,
        result,
        membership,
        verification: CanonicalPublicationVerificationReceiptV1 {
            family: CanonicalPublishedFamilyKindV1::Main,
            candidate_function_count: 2,
            _seal: CanonicalPublicationVerificationReceiptSealV1,
        },
        _seal: PreparedCanonicalSourceEntryPublicationSealV1,
    })
}

fn prepare_script(
    candidate: CompletedNormalScriptModuleCandidateV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
) -> Result<PreparedCanonicalSourceEntryPublicationV1, RejectedCanonicalSourceEntryPublicationV1> {
    let evidence = candidate.evidence();
    let target = match PendingPublishedSourceEntryTargetV1::new(
        evidence.target().symbol(),
        evidence.target().arity(),
    )
    .seal()
    {
        Ok(target) => target,
        Err(rejected) => {
            let cause = rejected.error().clone();
            rejected.discard();
            return Err(reject_script(
                candidate,
                admission,
                receipt,
                CanonicalSourceEntryPublicationStageV1::TargetProjection,
                CanonicalSourceEntryPublicationErrorV1::Target(cause),
            ));
        }
    };
    if !evidence.target().is_main()
        || evidence.schema_row_count() != 1
        || candidate.verification().function_count() != 1
        || candidate.module().functions.len() != 1
    {
        return Err(reject_script(
            candidate,
            admission,
            receipt,
            CanonicalSourceEntryPublicationStageV1::Pairing,
            CanonicalSourceEntryPublicationErrorV1::ScriptEvidenceMismatch,
        ));
    }
    let result = project_script_result(evidence.result());
    let membership = PublishedSourceEntryMembershipV1::Canonical(
        CanonicalPublishedSourceEntryMembershipV1::Script,
    );
    let (module, evidence, verification) = candidate.into_publication_parts();
    Ok(PreparedCanonicalSourceEntryPublicationV1 {
        owner: PreparedCanonicalPublishedOwnerV1 {
            module,
            family: PublishedCanonicalFamilyEvidenceV1::Script {
                evidence,
                verification,
            },
            admission,
            source_receipt: receipt,
        },
        target,
        result,
        membership,
        verification: CanonicalPublicationVerificationReceiptV1 {
            family: CanonicalPublishedFamilyKindV1::Script,
            candidate_function_count: 1,
            _seal: CanonicalPublicationVerificationReceiptSealV1,
        },
        _seal: PreparedCanonicalSourceEntryPublicationSealV1,
    })
}

fn reject_main(
    candidate: CompletedNormalMainModuleCandidateV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    stage: CanonicalSourceEntryPublicationStageV1,
    cause: CanonicalSourceEntryPublicationErrorV1,
) -> RejectedCanonicalSourceEntryPublicationV1 {
    reject_candidate(
        CompletedCanonicalCoreSourceEntryFamilyV1::Main(candidate),
        admission,
        receipt,
        stage,
        cause,
    )
}

fn reject_script(
    candidate: CompletedNormalScriptModuleCandidateV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    stage: CanonicalSourceEntryPublicationStageV1,
    cause: CanonicalSourceEntryPublicationErrorV1,
) -> RejectedCanonicalSourceEntryPublicationV1 {
    reject_candidate(
        CompletedCanonicalCoreSourceEntryFamilyV1::Script(candidate),
        admission,
        receipt,
        stage,
        cause,
    )
}

fn reject_candidate(
    family: CompletedCanonicalCoreSourceEntryFamilyV1,
    admission: VerifiedCanonicalCoreSourcePlanAdmissionV1,
    receipt: NormalSourcePlanReceiptV1,
    stage: CanonicalSourceEntryPublicationStageV1,
    cause: CanonicalSourceEntryPublicationErrorV1,
) -> RejectedCanonicalSourceEntryPublicationV1 {
    RejectedCanonicalSourceEntryPublicationV1 {
        owner: CompletedCanonicalCoreSourceEntryCandidateV1 {
            family,
            admission,
            receipt,
            _seal: super::CompletedCanonicalCoreSourceEntryCandidateSealV1,
        },
        stage,
        cause,
    }
}

fn project_main_result(
    result: VerifiedNormalMainThunkResultV1,
) -> PublishedSourceEntryResultContractV1 {
    match result {
        VerifiedNormalMainThunkResultV1::Unit { origin } => {
            PublishedSourceEntryResultContractV1::Unit {
                origin: match origin {
                    FunctionUnitOriginV1::EmptyBody => UnitOriginV1::EmptyBody,
                    FunctionUnitOriginV1::ImplicitFallthrough => UnitOriginV1::ImplicitFallthrough,
                    FunctionUnitOriginV1::ExplicitVoid => UnitOriginV1::ExplicitVoid,
                    FunctionUnitOriginV1::ExplicitNull => UnitOriginV1::ExplicitNull,
                    FunctionUnitOriginV1::BareReturn => UnitOriginV1::BareReturn,
                },
                physical: PublishedUnitPhysicalContractV1::ExactVoid,
            }
        }
        VerifiedNormalMainThunkResultV1::Integer => PublishedSourceEntryResultContractV1::Integer,
        VerifiedNormalMainThunkResultV1::Bool => PublishedSourceEntryResultContractV1::Bool,
        VerifiedNormalMainThunkResultV1::Float => PublishedSourceEntryResultContractV1::Float,
    }
}

fn project_script_result(
    result: &VerifiedScriptEntryResultContractV1,
) -> PublishedSourceEntryResultContractV1 {
    match result {
        VerifiedScriptEntryResultContractV1::Unit { origin, .. } => PublishedSourceEntryResultContractV1::Unit {
            origin: match origin {
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::EmptyBody => UnitOriginV1::EmptyBody,
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::VoidExpression => UnitOriginV1::ExplicitVoid,
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::PrintStatement => UnitOriginV1::PrintStatement,
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::LocalStatement => UnitOriginV1::LocalStatement,
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::AssignmentStatement => UnitOriginV1::AssignmentStatement,
                crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1::CompoundAssignmentStatement => UnitOriginV1::CompoundAssignmentStatement,
            },
            physical: PublishedUnitPhysicalContractV1::ExactVoid,
        },
        VerifiedScriptEntryResultContractV1::Integer => PublishedSourceEntryResultContractV1::Integer,
        VerifiedScriptEntryResultContractV1::Bool => PublishedSourceEntryResultContractV1::Bool,
        VerifiedScriptEntryResultContractV1::Float => PublishedSourceEntryResultContractV1::Float,
        VerifiedScriptEntryResultContractV1::String => PublishedSourceEntryResultContractV1::String,
    }
}
