//! S3-OWNER0: one typed Raw publication compile kernel.
//!
//! The compatibility ingress and the explicit VM-reference lane share this
//! owner chain.  Only the final caller decides whether to erase the published
//! owner into `MirCompileResult` or continue to exact VM execution.

use super::module_postprocess::ModulePostprocessOwnerV1;
use super::raw_root_callable_main::RejectedRawCallableMainInvocationV1;
use super::raw_root_children::RejectedRawRootChildrenInvocationV1;
use super::raw_root_decl_access::{
    RejectedRawRootBatchInvocationV1, RejectedRawRootBodyInvocationV1,
    RejectedRawRootEnvironmentInvocationV1,
};
use super::raw_root_drain::RejectedRawDrainInvocationV1;
use super::raw_root_eligibility::{
    RejectedRawRootEligibilityV1, RejectedRawRootPhysicalOpenV1,
};
use super::raw_root_external_commit::RejectedRawExternalCommitInvocationV1;
use super::raw_root_finalization::RejectedRawDrainFinalizationInvocationV1;
use super::raw_root_package::RejectedRawRootPlanningV1;
use super::raw_root_postprocess::RejectedRawPostprocessInvocationV1;
use super::raw_root_publication::{
    RejectedRawPublicationInvocationV1, RawPublishedInvocationV1,
};
use super::raw_source_binding::{RawCallableMainSelectionV1, RejectedRawSourceBindingV1};
use super::raw_root_helper_coverage::RawPublicEligibilityProfileV1;
use super::lowering_input::LegacyModuleLoweringInputV1;
use crate::ast::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawPublishedCompileStageV1 {
    SourceBinding,
    RootPackage,
    Eligibility,
    PhysicalOpen,
    Children,
    CallableMain,
    DeclarationAccess,
    Body,
    RootBatch,
    Drain,
    Finalization,
    Postprocess,
    ExternalCommit,
    Publication,
}

#[derive(Debug)]
pub(in crate::mir) enum RejectedRawPublishedCompileV1 {
    SourceBinding(Box<RejectedRawSourceBindingV1>),
    RootPackage(Box<RejectedRawRootPlanningV1>),
    Eligibility(Box<RejectedRawRootEligibilityV1>),
    PhysicalOpen(Box<RejectedRawRootPhysicalOpenV1>),
    Children(Box<RejectedRawRootChildrenInvocationV1>),
    CallableMain(Box<RejectedRawCallableMainInvocationV1>),
    DeclarationAccess(Box<RejectedRawRootEnvironmentInvocationV1>),
    Body(Box<RejectedRawRootBodyInvocationV1>),
    RootBatch(Box<RejectedRawRootBatchInvocationV1>),
    Drain(Box<RejectedRawDrainInvocationV1>),
    Finalization(Box<RejectedRawDrainFinalizationInvocationV1>),
    Postprocess(Box<RejectedRawPostprocessInvocationV1>),
    ExternalCommit(Box<RejectedRawExternalCommitInvocationV1>),
    Publication(Box<RejectedRawPublicationInvocationV1>),
}

impl RejectedRawPublishedCompileV1 {
    pub(in crate::mir) const fn stage(&self) -> RawPublishedCompileStageV1 {
        match self {
            Self::SourceBinding(_) => RawPublishedCompileStageV1::SourceBinding,
            Self::RootPackage(_) => RawPublishedCompileStageV1::RootPackage,
            Self::Eligibility(_) => RawPublishedCompileStageV1::Eligibility,
            Self::PhysicalOpen(_) => RawPublishedCompileStageV1::PhysicalOpen,
            Self::Children(_) => RawPublishedCompileStageV1::Children,
            Self::CallableMain(_) => RawPublishedCompileStageV1::CallableMain,
            Self::DeclarationAccess(_) => RawPublishedCompileStageV1::DeclarationAccess,
            Self::Body(_) => RawPublishedCompileStageV1::Body,
            Self::RootBatch(_) => RawPublishedCompileStageV1::RootBatch,
            Self::Drain(_) => RawPublishedCompileStageV1::Drain,
            Self::Finalization(_) => RawPublishedCompileStageV1::Finalization,
            Self::Postprocess(_) => RawPublishedCompileStageV1::Postprocess,
            Self::ExternalCommit(_) => RawPublishedCompileStageV1::ExternalCommit,
            Self::Publication(_) => RawPublishedCompileStageV1::Publication,
        }
    }

    pub(in crate::mir) fn discard(self) {
        match self {
            Self::SourceBinding(owner) => owner.discard(),
            Self::RootPackage(owner) => owner.discard(),
            Self::Eligibility(owner) => owner.discard(),
            Self::PhysicalOpen(owner) => owner.discard(),
            Self::Children(owner) => owner.discard(),
            Self::CallableMain(owner) => owner.discard(),
            Self::DeclarationAccess(owner) => owner.discard(),
            Self::Body(owner) => owner.discard(),
            Self::RootBatch(owner) => owner.discard(),
            Self::Drain(owner) => owner.discard(),
            Self::Finalization(owner) => owner.discard(),
            Self::Postprocess(owner) => owner.discard(),
            Self::ExternalCommit(owner) => owner.discard(),
            Self::Publication(owner) => owner.discard(),
        }
    }

    pub(in crate::mir) fn into_public_string(self) -> String {
        let stage = match self.stage() {
            RawPublishedCompileStageV1::SourceBinding => "source-binding",
            RawPublishedCompileStageV1::RootPackage => "root-package",
            RawPublishedCompileStageV1::Eligibility => "eligibility",
            RawPublishedCompileStageV1::PhysicalOpen => "physical-open",
            RawPublishedCompileStageV1::Children => "children",
            RawPublishedCompileStageV1::CallableMain => "callable-main",
            RawPublishedCompileStageV1::DeclarationAccess => "declaration-access",
            RawPublishedCompileStageV1::Body => "body",
            RawPublishedCompileStageV1::RootBatch => "root-batch",
            RawPublishedCompileStageV1::Drain => "drain",
            RawPublishedCompileStageV1::Finalization => "finalization",
            RawPublishedCompileStageV1::Postprocess => "postprocess",
            RawPublishedCompileStageV1::ExternalCommit => "external-commit-preparation",
            RawPublishedCompileStageV1::Publication => "publication",
        };
        let detail = format!("{:?}", self);
        self.discard();
        format!("[raw-public/{stage}/rejected] {detail}")
    }
}

impl super::MirCompiler {
    pub(in crate::mir) fn compile_raw_published_v1(
        &mut self,
        ast: ASTNode,
        source_file: Option<&str>,
    ) -> Result<RawPublishedInvocationV1, RejectedRawPublishedCompileV1> {
        let package = self
            .bind_raw_source_for_public(
                LegacyModuleLoweringInputV1::bare_ast(ast),
                source_file,
                "main",
                RawCallableMainSelectionV1::Omitted,
                super::raw_public_ingress::RawPublicImportDispositionV1::None,
            )
            .map_err(|error| RejectedRawPublishedCompileV1::SourceBinding(Box::new(error)))?;
        let root_package = package
            .into_root_package()
            .map_err(|error| RejectedRawPublishedCompileV1::RootPackage(Box::new(error)))?;
        let eligible = root_package
            .prepare_public_eligibility(RawPublicEligibilityProfileV1::narrow_v1())
            .map_err(|error| RejectedRawPublishedCompileV1::Eligibility(Box::new(error)))?;
        let opened = eligible
            .open_physical(&self.builder)
            .map_err(|error| RejectedRawPublishedCompileV1::PhysicalOpen(Box::new(error)))?;
        let pending = opened
            .prepare_children()
            .map_err(|error| RejectedRawPublishedCompileV1::Children(Box::new(error)))?;
        let complete = pending
            .complete_all()
            .map_err(|error| RejectedRawPublishedCompileV1::Children(Box::new(error)))?;
        let ready = complete
            .finish_callable_main()
            .map_err(|error| RejectedRawPublishedCompileV1::CallableMain(Box::new(error)))?;
        let declared = ready
            .declare_environment()
            .map_err(|error| RejectedRawPublishedCompileV1::DeclarationAccess(Box::new(error)))?;
        let body = declared
            .begin_body()
            .map_err(|error| RejectedRawPublishedCompileV1::Body(Box::new(error)))?;
        let batch = body
            .prepare_root_batch()
            .map_err(|error| RejectedRawPublishedCompileV1::RootBatch(Box::new(error)))?;
        let drained = batch
            .prepare_drain()
            .map_err(|error| RejectedRawPublishedCompileV1::Drain(Box::new(error)))?
            .drain();
        let finalized = drained
            .prepare_finalization()
            .map_err(|error| RejectedRawPublishedCompileV1::Finalization(Box::new(error)))?;
        let ready = finalized.prepare_postprocess();
        let postprocessed = ModulePostprocessOwnerV1::new(&mut self.verifier, self.optimize)
            .run_raw_ready(ready)
            .map_err(|error| RejectedRawPublishedCompileV1::Postprocess(Box::new(error)))?;
        let prepared = postprocessed
            .prepare_external_commit()
            .map_err(|error| RejectedRawPublishedCompileV1::ExternalCommit(Box::new(error)))?;
        self.publish_raw_direct(prepared)
            .map_err(|error| RejectedRawPublishedCompileV1::Publication(Box::new(error)))
    }
}
