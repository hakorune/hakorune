//! Caller-zero LoopCond Recipe producer.
//!
//! This profile consumes the sealed policy demand, emits the portable Recipe
//! envelope, verifies it, and delegates logical elaboration to the one shared
//! JoinSig owner. The product co-seals the source-bound Core, per-operation
//! evidence, and the initialized-local input set so the canonical physical
//! edge needs no second semantic pass. It does not inspect syntax or
//! allocate physical IDs.

use crate::mir::compiler::loop_cond_break_continue_typed_map::LoopCondTypedCompareV1;
use crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1;
use crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyDemandV1;
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1,
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
pub(crate) enum LoopCondBreakContinueRecipeProducerRejectV1 {
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
pub(crate) struct VerifiedLoopCondBreakContinueRecipeProductV1 {
    policy_receipt: crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1,
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
}

impl VerifiedLoopCondBreakContinueRecipeProductV1 {
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

    pub(crate) fn policy_receipt(
        &self,
    ) -> &crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1 {
        &self.policy_receipt
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::mir::loop_route_policy::VerifiedLoopCondBreakContinuePolicyReceiptV1,
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
    ) {
        (self.policy_receipt, self.operations, self.inputs)
    }
}

pub(crate) fn produce_loop_cond_break_continue_recipe_v1(
    demand: VerifiedLoopCondBreakContinuePolicyDemandV1,
    function: &VerifiedResolvedFunctionV1,
) -> Result<
    VerifiedLoopCondBreakContinueRecipeProductV1,
    LoopCondBreakContinueRecipeProducerRejectV1,
> {
    let (policy_receipt, map) = demand.into_parts();
    let (source_root, projection, carrier, loop_condition, branch_condition, _frame_key) =
        map.into_parts();
    if !policy_receipt
        .frame_key()
        .matches(projection.root_frame_key())
    {
        return Err(LoopCondBreakContinueRecipeProducerRejectV1::PolicyFrameMismatch);
    }
    let shape = projection.shape();
    let loop_site = shape.loop_site.clone();
    let compare_site = shape.branch_condition_site.clone();
    let loop_compare_site = shape.loop_condition_site.clone();
    let recipe = loop_cond_break_continue_recipe(&loop_condition, &branch_condition);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::LoopCondBreakContinueV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact.clone())
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::JoinSig)?;
    let binding = carrier.binding;
    let owner = binding.owner();
    let declaration = resolve_loop_binding_declaration_v1(function, binding)
        .ok_or(LoopCondBreakContinueRecipeProducerRejectV1::BindingDeclaration(binding))?;
    let initializer = declaration.initializer.clone().ok_or(
        LoopCondBreakContinueRecipeProducerRejectV1::BindingInitializer(binding),
    )?;
    let binding_rows = vec![LoopRecipeBindingRelationV1::new(
        LoopBindingKeyV1::new(0),
        binding,
        LoopValueClassV1::I64,
        BindingOriginV1::Source(declaration.declaration.clone()),
    )];
    let effects = effect_relations(owner, &loop_site, &loop_condition, &branch_condition, binding);
    let core =
        issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, binding_rows, effects)
            .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Core)?;
    let input_rows = vec![LoopInitializedLocalInputSourceRelationV1::new(
        declaration.declaration,
        initializer,
        binding,
        LoopValueKeyV1::new(0),
        LoopValueClassV1::I64,
    )];
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(
        owner,
        &loop_site,
        &loop_compare_site,
        &compare_site,
        &loop_condition,
        &branch_condition,
        binding,
    );
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(LoopCondBreakContinueRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedLoopCondBreakContinueRecipeProductV1 {
        policy_receipt,
        operations,
        inputs: input_set,
    })
}

fn effect_relations(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceStmtSiteV1,
    loop_condition: &LoopCondTypedCompareV1,
    branch_condition: &LoopCondTypedCompareV1,
    binding: BindingRefV1,
) -> Vec<LoopBindingEffectRelationV1> {
    let key = LoopBindingKeyV1::new(0);
    let expr = |site: &SourceExprSiteV1| {
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site.clone()))
    };
    vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            key,
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
            key,
            binding,
            LoopValueClassV1::I64,
            expr(&loop_condition.read_site),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            key,
            binding,
            LoopValueClassV1::I64,
            expr(&branch_condition.read_site),
        ),
    ]
}

fn operation_evidence(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceStmtSiteV1,
    loop_compare_site: &SourceExprSiteV1,
    branch_compare_site: &SourceExprSiteV1,
    loop_condition: &LoopCondTypedCompareV1,
    branch_condition: &LoopCondTypedCompareV1,
    binding: BindingRefV1,
) -> Vec<LoopOperationSourceEvidenceV1> {
    let row = |item: u32, block: u32, site: SourceExprSiteV1, source_binding| {
        LoopOperationSourceEvidenceV1::new(
            LoopItemKeyV1::new(item),
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
            loop_site.clone(),
            LoopNodeKeyV1::new(0),
            LoopBlockKeyV1::new(block),
            source_binding,
        )
    };
    vec![
        row(0, 0, loop_condition.read_site.clone(), Some(binding)),
        row(1, 0, loop_condition.bound_site.clone(), None),
        row(2, 0, loop_compare_site.clone(), None),
        row(3, 1, branch_condition.read_site.clone(), Some(binding)),
        row(4, 1, branch_condition.bound_site.clone(), None),
        row(5, 1, branch_compare_site.clone(), None),
    ]
}

fn compare_op(operator: SyntaxBinaryOperatorV1) -> LoopCompareI64OpV1 {
    match operator {
        SyntaxBinaryOperatorV1::Less => LoopCompareI64OpV1::Less,
        SyntaxBinaryOperatorV1::LessEqual => LoopCompareI64OpV1::LessEqual,
        SyntaxBinaryOperatorV1::Equal => LoopCompareI64OpV1::Equal,
        // The typed map seals only the compare vocabulary; any other
        // operator cannot reach this producer.
        _ => unreachable!("typed LoopCond compare operator outside the sealed vocabulary"),
    }
}

pub(super) fn loop_cond_break_continue_recipe(
    loop_condition: &LoopCondTypedCompareV1,
    branch_condition: &LoopCondTypedCompareV1,
) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let then_block = LoopBlockKeyV1::new(2);
    let else_block = LoopBlockKeyV1::new(3);
    let input = LoopValueKeyV1::new(0);
    let loop_read = LoopValueKeyV1::new(1);
    let loop_bound = LoopValueKeyV1::new(2);
    let loop_predicate = LoopValueKeyV1::new(3);
    let branch_read = LoopValueKeyV1::new(4);
    let branch_bound = LoopValueKeyV1::new(5);
    let branch_predicate = LoopValueKeyV1::new(6);
    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: loop_predicate,
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: (0..=2).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: (3..=6).map(LoopItemKeyV1::new).collect(),
            },
            LoopRecipeBlockV1 {
                key: then_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(7)],
            },
            LoopRecipeBlockV1 {
                key: else_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(8)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: loop_read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: loop_bound,
                        value: loop_condition.bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: compare_op(loop_condition.operator),
                        left: loop_read,
                        right: loop_bound,
                        result: loop_predicate,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(3),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ReadBinding {
                        binding,
                        result: branch_read,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(4),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::ConstI64 {
                        result: branch_bound,
                        value: branch_condition.bound,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(5),
                item: LoopRecipeItemV1::Operation {
                    operation: LoopOperationV1::CompareI64 {
                        op: compare_op(branch_condition.operator),
                        left: branch_read,
                        right: branch_bound,
                        result: branch_predicate,
                    },
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(6),
                item: LoopRecipeItemV1::If {
                    condition: branch_predicate,
                    then_block,
                    else_block: Some(else_block),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(7),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(8),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(1),
                },
            },
        ],
        bindings: vec![LoopRecipeBindingV1 {
            key: binding,
            label: "loop_cond_carrier".into(),
            class: LoopValueClassV1::I64,
        }],
        values: vec![
            LoopRecipeValueV1 {
                key: input,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: loop_predicate,
                class: LoopValueClassV1::Bool,
            },
            LoopRecipeValueV1 {
                key: branch_read,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: branch_bound,
                class: LoopValueClassV1::I64,
            },
            LoopRecipeValueV1 {
                key: branch_predicate,
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
