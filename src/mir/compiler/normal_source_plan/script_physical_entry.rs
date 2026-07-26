//! Compiler-owned detached physical Script draft owner.

use crate::mir::builder::{
    canonical_normal_main_entry_target, CompletedScriptPhysicalFunctionV1,
    OpenScriptPhysicalEntrySessionV1, ScriptPhysicalEntrySessionErrorV1,
};
use crate::mir::compiler::normal_source_plan::{
    RetainedNormalScriptSourceV1, VerifiedNormalScriptRecipeV1,
};
use crate::mir::raw_root_body_recipe::RawScriptBodyRecipeV1;

use super::super::MirCompiler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalScriptPhysicalEntryStageV1 {
    FunctionOpen,
    BodyLowering,
}

pub(crate) struct OpenScriptPhysicalEntryV1 {
    source: RetainedNormalScriptSourceV1,
    recipe: RawScriptBodyRecipeV1,
    session: OpenScriptPhysicalEntrySessionV1,
}

#[derive(Debug)]
pub(crate) struct CompletedScriptPhysicalExitV1 {
    source: RetainedNormalScriptSourceV1,
    draft: CompletedScriptPhysicalFunctionV1,
}

pub(crate) struct RejectedNormalScriptPhysicalEntryV1 {
    source: RetainedNormalScriptSourceV1,
    recipe: RawScriptBodyRecipeV1,
    session: Option<OpenScriptPhysicalEntrySessionV1>,
    stage: NormalScriptPhysicalEntryStageV1,
    cause: ScriptPhysicalEntrySessionErrorV1,
}

impl OpenScriptPhysicalEntryV1 {
    pub(crate) fn open(
        compiler: &MirCompiler,
        recipe: VerifiedNormalScriptRecipeV1,
    ) -> Result<Self, RejectedNormalScriptPhysicalEntryV1> {
        let (source, recipe) = recipe.into_physical_parts();
        let session = match OpenScriptPhysicalEntrySessionV1::open(
            &compiler.builder,
            canonical_normal_main_entry_target(),
        ) {
            Ok(session) => session,
            Err(cause) => {
                return Err(RejectedNormalScriptPhysicalEntryV1 {
                    source,
                    recipe,
                    session: None,
                    stage: NormalScriptPhysicalEntryStageV1::FunctionOpen,
                    cause,
                })
            }
        };
        Ok(Self {
            source,
            recipe,
            session,
        })
    }

    pub(crate) fn prepare(
        self,
    ) -> Result<CompletedScriptPhysicalExitV1, RejectedNormalScriptPhysicalEntryV1> {
        match self.session.lower_and_complete(&self.recipe) {
            Ok(draft) => Ok(CompletedScriptPhysicalExitV1 {
                source: self.source,
                draft,
            }),
            Err((session, cause)) => Err(RejectedNormalScriptPhysicalEntryV1 {
                source: self.source,
                recipe: self.recipe,
                session: Some(session),
                stage: NormalScriptPhysicalEntryStageV1::BodyLowering,
                cause,
            }),
        }
    }
}

impl CompletedScriptPhysicalExitV1 {
    pub(crate) fn draft(&self) -> &crate::mir::MirFunction {
        self.draft.draft()
    }

    pub(crate) fn into_draft(self) -> crate::mir::MirFunction {
        self.draft.into_draft()
    }
}

impl RejectedNormalScriptPhysicalEntryV1 {
    pub(crate) const fn stage(&self) -> NormalScriptPhysicalEntryStageV1 {
        self.stage
    }

    pub(crate) fn cause(&self) -> &ScriptPhysicalEntrySessionErrorV1 {
        &self.cause
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}
