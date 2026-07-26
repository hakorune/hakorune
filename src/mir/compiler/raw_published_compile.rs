//! S3-OWNER0: one typed Raw publication compile kernel.
//!
//! The compatibility ingress and the explicit VM-reference lane share this
//! owner chain.  Only the final caller decides whether to erase the published
//! owner into `MirCompileResult` or continue to exact VM execution.

use super::lowering_input::LegacyModuleLoweringInputV1;
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::raw_root_callable_main::RejectedRawCallableMainInvocationV1;
use super::raw_root_children::RejectedRawRootChildrenInvocationV1;
use super::raw_root_decl_access::{
    RejectedRawRootBatchInvocationV1, RejectedRawRootBodyInvocationV1,
    RejectedRawRootEnvironmentInvocationV1,
};
use super::raw_root_drain::RejectedRawDrainInvocationV1;
use super::raw_root_eligibility::{RejectedRawRootEligibilityV1, RejectedRawRootPhysicalOpenV1};
use super::raw_root_external_commit::RejectedRawExternalCommitInvocationV1;
use super::raw_root_finalization::RejectedRawDrainFinalizationInvocationV1;
use super::raw_root_package::RejectedRawRootPlanningV1;
use super::raw_root_postprocess::RejectedRawPostprocessInvocationV1;
use super::raw_root_publication::{RawPublishedInvocationV1, RejectedRawPublicationInvocationV1};
use super::raw_source_binding::RejectedRawSourceBindingV1;
use crate::mir::RawPublishedCompileRequestV1;

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
        let report = RawPublishedCompileFailureReportV1::from_stage(self.stage());
        self.discard();
        // Keep the established public prefix stable while exposing a bounded
        // stage/code/detail report instead of formatting the owner graph.
        format!(
            "[raw-public/{stage}/rejected] {}: {}",
            report.code,
            report.detail()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RawPublishedCompileFailureReportV1 {
    code: &'static str,
    detail: &'static str,
}

impl RawPublishedCompileFailureReportV1 {
    const fn from_stage(stage: RawPublishedCompileStageV1) -> Self {
        let (code, detail) = match stage {
            RawPublishedCompileStageV1::SourceBinding => {
                ("source-binding-rejected", "typed source binding rejection")
            }
            RawPublishedCompileStageV1::RootPackage => {
                ("root-package-rejected", "typed root package rejection")
            }
            RawPublishedCompileStageV1::Eligibility => {
                ("eligibility-rejected", "typed eligibility rejection")
            }
            RawPublishedCompileStageV1::PhysicalOpen => {
                ("physical-open-rejected", "typed physical-open rejection")
            }
            RawPublishedCompileStageV1::Children => {
                ("children-rejected", "typed children rejection")
            }
            RawPublishedCompileStageV1::CallableMain => {
                ("callable-main-rejected", "typed callable-main rejection")
            }
            RawPublishedCompileStageV1::DeclarationAccess => (
                "declaration-access-rejected",
                "typed declaration-access rejection",
            ),
            RawPublishedCompileStageV1::Body => ("body-rejected", "typed body rejection"),
            RawPublishedCompileStageV1::RootBatch => {
                ("root-batch-rejected", "typed root-batch rejection")
            }
            RawPublishedCompileStageV1::Drain => ("drain-rejected", "typed drain rejection"),
            RawPublishedCompileStageV1::Finalization => {
                ("finalization-rejected", "typed finalization rejection")
            }
            RawPublishedCompileStageV1::Postprocess => {
                ("postprocess-rejected", "typed postprocess rejection")
            }
            RawPublishedCompileStageV1::ExternalCommit => (
                "external-commit-rejected",
                "typed external-commit rejection",
            ),
            RawPublishedCompileStageV1::Publication => {
                ("publication-rejected", "typed publication rejection")
            }
        };
        Self { code, detail }
    }

    const fn detail(self) -> &'static str {
        self.detail
    }
}

impl super::MirCompiler {
    pub(in crate::mir) fn compile_raw_published_v1(
        &mut self,
        request: RawPublishedCompileRequestV1,
    ) -> Result<RawPublishedInvocationV1, RejectedRawPublishedCompileV1> {
        let RawPublishedCompileRequestV1 {
            ast,
            source_file,
            module_name,
            profile,
        } = request;
        let (eligibility, imports, callable_main) = profile.into_parts();
        let package = self
            .bind_raw_source_for_public(
                LegacyModuleLoweringInputV1::bare_ast(ast),
                source_file.as_deref(),
                module_name,
                callable_main,
                imports,
            )
            .map_err(|error| RejectedRawPublishedCompileV1::SourceBinding(Box::new(error)))?;
        let root_package = package
            .into_root_package()
            .map_err(|error| RejectedRawPublishedCompileV1::RootPackage(Box::new(error)))?;
        let eligible = root_package
            .prepare_public_eligibility(eligibility)
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
