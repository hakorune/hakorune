//! Builder-free DirectAccum source-effect plan.
//!
//! This semantic product belongs beside structural facts.  It is deliberately
//! independent of the compiler profile and of MIR/SSA owners so resolved
//! lowering can borrow it without reversing the layer boundary.

use super::types::DirectAccumStructuralShapeV1;
use crate::mir::loop_recipe_contract::LoopBindingKeyV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SourceExprSiteV1,
};

/// The five source-effect roles that a DirectAccum execution must claim.
/// Literal RHS expressions are value coverage, not BindingRef identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum DirectAccumBindingEffectRoleV1 {
    ConditionInductionRead,
    UpdateAccumulatorRead,
    StepInductionRead,
    UpdateAccumulatorWrite,
    StepInductionWrite,
}

impl DirectAccumBindingEffectRoleV1 {
    pub(crate) const ALL: [Self; 5] = [
        Self::ConditionInductionRead,
        Self::UpdateAccumulatorRead,
        Self::StepInductionRead,
        Self::UpdateAccumulatorWrite,
        Self::StepInductionWrite,
    ];
}

/// One role-keyed source claim prepared for the resolved identity adapter.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DirectAccumBindingEffectEntryV1 {
    role: DirectAccumBindingEffectRoleV1,
    recipe_binding: LoopBindingKeyV1,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
}

impl DirectAccumBindingEffectEntryV1 {
    pub(crate) fn role(&self) -> DirectAccumBindingEffectRoleV1 {
        self.role
    }

    pub(crate) fn recipe_binding(&self) -> LoopBindingKeyV1 {
        self.recipe_binding
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
}

/// Builder-free source claims consumed by the canonical identity ledger.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumBindingEffectPlanV1 {
    owner: FunctionOwnerIdV1,
    frame_key: LoopExecutionFrameKeyV1,
    entries: [DirectAccumBindingEffectEntryV1; 5],
    _seal: VerifiedDirectAccumBindingEffectPlanSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct VerifiedDirectAccumBindingEffectPlanSealV1;

impl VerifiedDirectAccumBindingEffectPlanV1 {
    pub(crate) fn issue(
        owner: FunctionOwnerIdV1,
        frame_key: LoopExecutionFrameKeyV1,
        shape: &DirectAccumStructuralShapeV1,
    ) -> Self {
        Self {
            owner,
            frame_key,
            entries: [
                DirectAccumBindingEffectEntryV1 {
                    role: DirectAccumBindingEffectRoleV1::ConditionInductionRead,
                    recipe_binding: LoopBindingKeyV1::new(0),
                    site: shape.condition_lhs_site.clone(),
                    binding: shape.condition_binding,
                },
                DirectAccumBindingEffectEntryV1 {
                    role: DirectAccumBindingEffectRoleV1::UpdateAccumulatorRead,
                    recipe_binding: LoopBindingKeyV1::new(1),
                    site: shape.update.lhs_site.clone(),
                    binding: shape.update.binding,
                },
                DirectAccumBindingEffectEntryV1 {
                    role: DirectAccumBindingEffectRoleV1::StepInductionRead,
                    recipe_binding: LoopBindingKeyV1::new(0),
                    site: shape.step.lhs_site.clone(),
                    binding: shape.step.binding,
                },
                DirectAccumBindingEffectEntryV1 {
                    role: DirectAccumBindingEffectRoleV1::UpdateAccumulatorWrite,
                    recipe_binding: LoopBindingKeyV1::new(1),
                    site: shape.update.target_site.clone(),
                    binding: shape.update.binding,
                },
                DirectAccumBindingEffectEntryV1 {
                    role: DirectAccumBindingEffectRoleV1::StepInductionWrite,
                    recipe_binding: LoopBindingKeyV1::new(0),
                    site: shape.step.target_site.clone(),
                    binding: shape.step.binding,
                },
            ],
            _seal: VerifiedDirectAccumBindingEffectPlanSealV1,
        }
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame_key
    }

    pub(crate) fn entries(&self) -> &[DirectAccumBindingEffectEntryV1; 5] {
        &self.entries
    }

    pub(crate) fn entry(
        &self,
        role: DirectAccumBindingEffectRoleV1,
    ) -> &DirectAccumBindingEffectEntryV1 {
        self.entries
            .iter()
            .find(|entry| entry.role == role)
            .expect("all DirectAccum effect roles are sealed")
    }
}
