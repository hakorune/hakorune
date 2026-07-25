//! PUBLICATION0: RawDirect live-Builder publication.

use super::module_postprocess::ModulePostprocessScheduleV1;
use super::publication_kernel::{
    publish_once, PublishedModuleTransferV1, SealedPublicationPayloadV1,
};
use super::raw_root_external_commit::{
    PreparedRawExternalCommitV1, RawExternalCommitPublicationFactsV1,
    RawExternalCommitPublicationPartsV1,
};
use super::source_entry_selection::SelectedSourceEntryContinuationV1;
use crate::mir::builder::{
    check_builder_external_commit_quiescence, BuilderCommitReadinessErrorV1,
    BuilderPublicationReceiptV1, MirBuilder, PreparedBuilderExternalCommitV1,
    RawExternalCommitModuleV1, RawPublishedModuleV1,
};
use crate::mir::module_invocation_identity::{
    ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawPublicationFailureStageV1 {
    Target,
    Identity,
    Evidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPublicationErrorV1 {
    TargetNotQuiescent(BuilderCommitReadinessErrorV1),
    NonRawFamily,
    ForeignBrand,
    ScheduleMismatch,
    VerificationMismatch,
    ProgressNotSealed,
}

#[derive(Debug)]
pub(in crate::mir) struct RawPublicationSealV1 {
    receipt: BuilderPublicationReceiptV1,
    _seal: RawPublicationSealInnerV1,
}

#[derive(Debug)]
struct RawPublicationSealInnerV1;

#[derive(Debug)]
pub(in crate::mir) struct RawPublishedInvocationCoreV1 {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) module: RawPublishedModuleV1,
    pub(in crate::mir) evidence: super::raw_root_postprocess::RawPostprocessEvidenceV1,
    pub(in crate::mir) publication: RawPublicationSealV1,
}

impl RawPublishedInvocationCoreV1 {
    pub(in crate::mir) fn selected_entry(&self) -> &SelectedSourceEntryContinuationV1 {
        self.evidence.selected_entry()
    }
}

impl RawPublishedInvocationV1 {
    pub(in crate::mir) fn selected_entry(&self) -> &SelectedSourceEntryContinuationV1 {
        match self {
            Self::Script(value) => value.core.selected_entry(),
            Self::App(value) => value.core.selected_entry(),
        }
    }

    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn execute_exact_vm_entry(
        self,
        symbol: &str,
    ) -> (
        Self,
        Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError>,
    ) {
        let result = match &self {
            Self::Script(value) => value.core.module.execute_exact_vm_entry(symbol),
            Self::App(value) => value.core.module.execute_exact_vm_entry(symbol),
        };
        (self, result)
    }

    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn vm_decode_plan(
        &self,
    ) -> Result<super::source_entry_vm_reference::VmSourceEntryDecodePlanV1, ()> {
        match self {
            Self::Script(value) => value.core.evidence.vm_decode_plan(),
            Self::App(value) => value.core.evidence.vm_decode_plan(),
        }
    }

    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn main_entry_target_matches(&self) -> bool {
        match self {
            Self::Script(value) => value.core.evidence.main_entry_target_matches(),
            Self::App(value) => value.core.evidence.main_entry_target_matches(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptPublishedInvocationV1 {
    pub(in crate::mir) core: RawPublishedInvocationCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppPublishedInvocationV1 {
    pub(in crate::mir) core: RawPublishedInvocationCoreV1,
}

#[derive(Debug)]
pub(in crate::mir) enum RawPublishedInvocationV1 {
    Script(RawScriptPublishedInvocationV1),
    App(RawAppPublishedInvocationV1),
}

#[derive(Debug)]
struct RawDirectPublicationPayload {
    route: super::raw_root_postprocess::RawPostprocessRouteKindV1,
    token: ModuleInvocationTokenV1,
    evidence: super::raw_root_postprocess::RawPostprocessEvidenceV1,
}

impl SealedPublicationPayloadV1 for RawDirectPublicationPayload {
    type Published = RawPublishedInvocationV1;

    fn finish(
        self,
        receipt: BuilderPublicationReceiptV1,
        module: PublishedModuleTransferV1,
    ) -> Self::Published {
        let core = RawPublishedInvocationCoreV1 {
            token: self.token,
            module: match module {
                PublishedModuleTransferV1::Raw(module) => module,
                PublishedModuleTransferV1::None => {
                    unreachable!("RawDirect module transfer missing")
                }
            },
            evidence: self.evidence,
            publication: RawPublicationSealV1 {
                receipt,
                _seal: RawPublicationSealInnerV1,
            },
        };
        match self.route {
            super::raw_root_postprocess::RawPostprocessRouteKindV1::Script => {
                RawPublishedInvocationV1::Script(RawScriptPublishedInvocationV1 { core })
            }
            super::raw_root_postprocess::RawPostprocessRouteKindV1::App => {
                RawPublishedInvocationV1::App(RawAppPublishedInvocationV1 { core })
            }
        }
    }
}

struct PreparedRawPublicationV1<'target> {
    target: &'target mut MirBuilder,
    builder: PreparedBuilderExternalCommitV1,
    payload: RawDirectPublicationPayload,
    module: RawExternalCommitModuleV1,
}

impl PreparedRawPublicationV1<'_> {
    fn publish(self) -> RawPublishedInvocationV1 {
        publish_once(self.target, self.builder, self.payload, Some(self.module))
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPublicationInvocationV1 {
    owner: PreparedRawExternalCommitV1,
    stage: RawPublicationFailureStageV1,
    error: RawPublicationErrorV1,
}

impl RejectedRawPublicationInvocationV1 {
    pub(in crate::mir) fn stage(&self) -> RawPublicationFailureStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawPublicationErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

impl super::MirCompiler {
    pub(in crate::mir) fn publish_raw_direct(
        &mut self,
        prepared: PreparedRawExternalCommitV1,
    ) -> Result<RawPublishedInvocationV1, RejectedRawPublicationInvocationV1> {
        if let Err(error) = check_builder_external_commit_quiescence(&self.builder) {
            return Err(RejectedRawPublicationInvocationV1 {
                owner: prepared,
                stage: RawPublicationFailureStageV1::Target,
                error: RawPublicationErrorV1::TargetNotQuiescent(error),
            });
        }
        let facts = prepared.publication_facts();
        if let Err((stage, error)) = validate_facts(&facts) {
            return Err(RejectedRawPublicationInvocationV1 {
                owner: prepared,
                stage,
                error,
            });
        }
        let (route, parts) = prepared.into_publication_parts();
        let payload = payload_from_parts(route, parts);
        let target = &mut self.builder;
        Ok(PreparedRawPublicationV1 {
            target,
            builder: payload.1,
            payload: payload.0,
            module: payload.2,
        }
        .publish())
    }
}

fn validate_facts(
    facts: &RawExternalCommitPublicationFactsV1,
) -> Result<(), (RawPublicationFailureStageV1, RawPublicationErrorV1)> {
    if facts.token_family != ModuleInvocationFamilyV1::Raw
        || facts.builder_family != ModuleInvocationFamilyV1::Raw
    {
        return Err((
            RawPublicationFailureStageV1::Identity,
            RawPublicationErrorV1::NonRawFamily,
        ));
    }
    if facts.token_brand != facts.builder_brand
        || facts.token_brand != facts.witness_brand
        || facts.token_brand != facts.finalization_brand
        || facts.token_brand != facts.postprocess_brand
    {
        return Err((
            RawPublicationFailureStageV1::Identity,
            RawPublicationErrorV1::ForeignBrand,
        ));
    }
    if facts.schedule != ModulePostprocessScheduleV1::for_family(ModuleInvocationFamilyV1::Raw) {
        return Err((
            RawPublicationFailureStageV1::Evidence,
            RawPublicationErrorV1::ScheduleMismatch,
        ));
    }
    if !facts.verification_is_raw {
        return Err((
            RawPublicationFailureStageV1::Evidence,
            RawPublicationErrorV1::VerificationMismatch,
        ));
    }
    if facts.progress != crate::mir::builder::RawPostprocessProgressV1::ParitySealed {
        return Err((
            RawPublicationFailureStageV1::Evidence,
            RawPublicationErrorV1::ProgressNotSealed,
        ));
    }
    Ok(())
}

fn payload_from_parts(
    route: super::raw_root_postprocess::RawPostprocessRouteKindV1,
    parts: RawExternalCommitPublicationPartsV1,
) -> (
    RawDirectPublicationPayload,
    PreparedBuilderExternalCommitV1,
    RawExternalCommitModuleV1,
) {
    let RawExternalCommitPublicationPartsV1 {
        token,
        builder,
        module,
        evidence,
    } = parts;
    (
        RawDirectPublicationPayload {
            route,
            token,
            evidence,
        },
        builder,
        module,
    )
}
