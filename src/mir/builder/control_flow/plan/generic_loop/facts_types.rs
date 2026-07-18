//! Type definitions for generic loop facts

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::facts::no_exit_block::NoExitBlockRecipe;
use crate::mir::builder::control_flow::generic_loop_canon::StepPlacement;
use crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe;
use crate::mir::builder::control_flow::recipes::RecipeBody;
use crate::mir::policies::BodyLoweringPolicy;

/// Closed semantic role for the selected GenericLoop carrier.
///
/// Facts own this role only. MIR representation remains a Builder-side
/// lowering-time decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum GenericLoopCarrierRoleV1 {
    NumericProgression,
    BodyManagedState,
}

/// Final successful step result retained by the canonical v1 extractor.
///
/// Candidate rejection state stays inside extraction. This product records
/// only the disposition that survived all existing validation and fallback.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum GenericLoopV1StepDispositionV1 {
    NumericProgression {
        placement: StepPlacement,
        canonical_body_len: usize,
    },
    BodyManagedState,
}

impl GenericLoopV1StepDispositionV1 {
    pub(super) fn carrier_role(&self) -> GenericLoopCarrierRoleV1 {
        match self {
            Self::NumericProgression { .. } => GenericLoopCarrierRoleV1::NumericProgression,
            Self::BodyManagedState => GenericLoopCarrierRoleV1::BodyManagedState,
        }
    }

    pub(super) fn is_body_managed(&self) -> bool {
        matches!(self, Self::BodyManagedState)
    }
}

/// Facts extracted for generic loop v0 (ExitIf-capable, no carriers)
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV0Facts {
    pub carrier_role: GenericLoopCarrierRoleV1,
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: RecipeBody,
}

/// Facts extracted for generic loop v1
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV1Facts {
    pub carrier_role: GenericLoopCarrierRoleV1,
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: RecipeBody,
    pub body_lowering_policy: BodyLoweringPolicy,
    pub body_exit_allowed: Option<ExitAllowedBlockRecipe>,
    pub body_no_exit: Option<NoExitBlockRecipe>,
}

/// Canonical successful v1 extraction result.
///
/// Fields are private so facts and the successful step witness cannot be
/// reconstructed or paired independently.
#[derive(Debug)]
pub(in crate::mir::builder) struct GenericLoopV1ExtractionV1 {
    facts: GenericLoopV1Facts,
    step: GenericLoopV1StepDispositionV1,
}

impl GenericLoopV1ExtractionV1 {
    pub(super) fn new(mut facts: GenericLoopV1Facts, step: GenericLoopV1StepDispositionV1) -> Self {
        facts.carrier_role = step.carrier_role();
        Self { facts, step }
    }

    pub(in crate::mir::builder) fn facts(&self) -> &GenericLoopV1Facts {
        &self.facts
    }

    pub(in crate::mir::builder) fn step(&self) -> &GenericLoopV1StepDispositionV1 {
        &self.step
    }

    pub(in crate::mir::builder) fn into_facts(self) -> GenericLoopV1Facts {
        self.facts
    }
}
