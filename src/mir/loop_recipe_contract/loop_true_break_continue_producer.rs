//! Caller-zero LoopTrue Recipe producer.
//!
//! This profile consumes the sealed policy demand, emits the portable Recipe
//! envelope, verifies it, and delegates logical elaboration to the one shared
//! JoinSig owner. The product co-seals the source-bound Core, per-operation
//! evidence, and the initialized-local input set so the canonical physical
//! edge needs no second semantic pass. It does not inspect syntax or
//! allocate physical IDs.

use crate::mir::compiler::loop_true_break_continue_projection::VerifiedLoopTrueBreakContinueSourceShapeV1;
use crate::mir::loop_route_policy::{
    VerifiedLoopTrueBreakContinuePolicyDemandV1, VerifiedLoopTrueBreakContinuePolicyReceiptV1,
};
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::binding_declaration::resolve_loop_binding_declaration_v1;
use super::error::LoopRecipeRejectReasonV1;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::input_source::{
    issue_initialized_local_input_source_set_v1, LoopInitializedLocalInputSourceRelationV1,
    LoopInitializedLocalInputSourceSetRejectV1, VerifiedLoopInitializedLocalInputSourceSetV1,
};
use super::join_sig::{LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1};
use super::operation_effect::{
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1,
    VerifiedLoopOperationEffectProductV1,
};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1, LoopNodeV1, LoopOperationV1,
    LoopRecipeArtifactV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1,
};
use super::source_bound_core::{
    issue_source_bound_core_from_artifact_v1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
};
use super::verify::LoopRecipeVerifierV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopTrueBreakContinueRecipeProducerRejectV1 {
    PolicyFrameMismatch,
    BindingDeclaration(BindingRefV1),
    BindingInitializer(BindingRefV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

/// Physical-ready caller-zero product: the policy receipt stays attached for
/// the family edge while the operation/effect Core and input set carry every
/// source relation the canonical physicalizer needs.
#[derive(Debug)]
pub(crate) struct VerifiedLoopTrueBreakContinueRecipeProductV1 {
    policy_receipt: VerifiedLoopTrueBreakContinuePolicyReceiptV1,
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
}

impl VerifiedLoopTrueBreakContinueRecipeProductV1 {
    pub(crate) fn recipe(&self) -> &super::verify::VerifiedLoopRecipeV1 {
        self.operations.core().recipe()
    }

    pub(crate) fn join_sig(&self) -> &super::join_sig::VerifiedLoopJoinSigV1 {
        self.operations.core().join_sig()
    }

    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }

    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }

    pub(crate) fn policy_receipt(&self) -> &VerifiedLoopTrueBreakContinuePolicyReceiptV1 {
        &self.policy_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopTrueBreakContinuePolicyReceiptV1,
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
    ) {
        (self.policy_receipt, self.operations, self.inputs)
    }
}

pub(crate) fn produce_loop_true_break_continue_recipe_v1(
    demand: VerifiedLoopTrueBreakContinuePolicyDemandV1,
    function: &VerifiedResolvedFunctionV1,
) -> Result<
    VerifiedLoopTrueBreakContinueRecipeProductV1,
    LoopTrueBreakContinueRecipeProducerRejectV1,
> {
    let (policy_receipt, projection) = demand.into_parts();
    if !policy_receipt
        .frame_key()
        .matches(projection.root_frame_key())
    {
        return Err(LoopTrueBreakContinueRecipeProducerRejectV1::PolicyFrameMismatch);
    }
    let (source_root, shape, _frame_key) = projection.into_parts();
    let loop_site = shape.loop_site.clone();
    let recipe = loop_true_break_continue_recipe(&shape);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopTrueBreakContinueV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact.clone())
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::JoinSig)?;
    let owner = shape.branch_condition_binding.owner();
    let binding = shape.branch_condition_binding;
    let declaration = resolve_loop_binding_declaration_v1(function, binding)
        .ok_or(LoopTrueBreakContinueRecipeProducerRejectV1::BindingDeclaration(binding))?;
    let initializer = declaration.initializer.clone().ok_or(
        LoopTrueBreakContinueRecipeProducerRejectV1::BindingInitializer(binding),
    )?;
    let binding_rows = vec![LoopRecipeBindingRelationV1::new(
        LoopBindingKeyV1::new(0),
        binding,
        LoopValueClassV1::I64,
        BindingOriginV1::Source(declaration.declaration.clone()),
    )];
    let effects = vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            LoopBindingKeyV1::new(0),
            binding,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: loop_site.clone(),
                carrier: LoopCarrierKeyV1::new(0),
            },
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            LoopBindingKeyV1::new(0),
            binding,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(
                owner,
                shape.branch_condition_lhs_site.clone(),
            )),
        ),
    ];
    let core =
        issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, binding_rows, effects)
            .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Core)?;
    let input_rows = vec![LoopInitializedLocalInputSourceRelationV1::new(
        declaration.declaration,
        initializer,
        binding,
        LoopValueKeyV1::new(0),
        LoopValueClassV1::I64,
    )];
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(owner, &loop_site, &shape);
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(LoopTrueBreakContinueRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedLoopTrueBreakContinueRecipeProductV1 {
        policy_receipt,
        operations,
        inputs: input_set,
    })
}

fn operation_evidence(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceStmtSiteV1,
    shape: &VerifiedLoopTrueBreakContinueSourceShapeV1,
) -> Vec<LoopOperationSourceEvidenceV1> {
    let row = |item: u32, site: crate::mir::resolved_semantics::SourceExprSiteV1, binding| {
        LoopOperationSourceEvidenceV1::new(
            LoopItemKeyV1::new(item),
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
            loop_site.clone(),
            LoopNodeKeyV1::new(0),
            LoopBlockKeyV1::new(0),
            binding,
        )
    };
    vec![
        row(
            0,
            shape.branch_condition_lhs_site.clone(),
            Some(shape.branch_condition_binding),
        ),
        row(1, shape.branch_condition_rhs_site.clone(), None),
        row(2, shape.branch_condition_site.clone(), None),
    ]
}

pub(super) fn loop_true_break_continue_recipe(
    shape: &VerifiedLoopTrueBreakContinueSourceShapeV1,
) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let then_block = LoopBlockKeyV1::new(1);
    let else_block = LoopBlockKeyV1::new(2);
    let input = LoopValueKeyV1::new(0);
    let read = LoopValueKeyV1::new(1);
    let bound = LoopValueKeyV1::new(2);
    let condition = LoopValueKeyV1::new(3);
    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Always,
            body: condition_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: (0..=3).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: then_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(4)],
            },
            LoopRecipeBlockV1 {
                key: else_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(5)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: bound,
                        value: shape.branch_condition_bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: LoopCompareI64OpV1::Equal,
                        left: read,
                        right: bound,
                        result: condition,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(3),
                item: LoopRecipeItemV1::If {
                    condition,
                    then_block,
                    else_block: Some(else_block),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(4),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(5),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(1),
                },
            },
        ],
        bindings: vec![LoopRecipeBindingV1 {
            key: binding,
            label: "loop_true_branch_condition".into(),
            class: LoopValueClassV1::I64,
        }],
        values: vec![
            LoopRecipeValueV1 {
                key: input,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: condition,
                class: LoopValueClassV1::Bool,
            },
        ],
        inputs: vec![input],
        carriers: vec![LoopRecipeCarrierV1 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: loop_key,
            binding,
            class: LoopValueClassV1::I64,
            entry_value: input,
        }],
        exits: vec![
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(0),
                owner_loop: loop_key,
                kind: LoopExitKindV1::Break {
                    target_loop: loop_key,
                },
            },
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(1),
                owner_loop: loop_key,
                kind: LoopExitKindV1::Continue {
                    target_loop: loop_key,
                },
            },
        ],
    }
}
