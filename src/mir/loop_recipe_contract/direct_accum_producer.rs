//! Caller-zero Direct Accum producer for the portable Loop recipe contract.
//!
//! This module consumes one already-selected semantic demand. It does not
//! inspect syntax, select routes, or allocate physical identities. The
//! product co-seals the source-bound Core (source claim, binding/effect
//! relations), the per-operation evidence rows, and the initialized-local
//! input set so the canonical physical edge needs no second semantic pass.

use crate::mir::loop_structural_facts::{
    DirectAccumFactsPayloadRejectV1, DirectAccumStructuralShapeV1, DirectAccumUpdateShapeV1,
    LoopRootSourceBindingRejectV1, VerifiedSelectedLoopRecipeDemandV1,
};
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::binding_declaration::resolve_loop_binding_declaration_v1;
use super::error::LoopRecipeRejectReasonV1;
use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopValueKeyV1,
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
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopOperationV1, LoopRecipeArtifactV1,
    LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeItemRowV1,
    LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1, LoopRecipeValueV1, LoopValueClassV1,
};
use super::source_bound_core::{
    issue_source_bound_core_from_artifact_v1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
};
use super::verify::LoopRecipeVerifierV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumRecipeProducerRejectV1 {
    FactsPayload(DirectAccumFactsPayloadRejectV1),
    ConflictingBindingRoles,
    InductionRoleMismatch,
    AccumulatorRoleMismatch,
    SourceBinding(LoopRootSourceBindingRejectV1),
    BindingDeclaration(BindingRefV1),
    BindingInitializer(BindingRefV1),
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

/// Physical-ready caller-zero product: the operation/effect Core and the
/// initialized-local input set are co-sealed with the verified Recipe.
#[derive(Debug)]
pub(crate) struct VerifiedDirectAccumRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
}

impl VerifiedDirectAccumRecipeProductV1 {
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

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
    ) {
        (self.operations, self.inputs)
    }
}

pub(crate) fn produce_direct_accum_recipe_v1(
    demand: VerifiedSelectedLoopRecipeDemandV1,
    function: &VerifiedResolvedFunctionV1,
) -> Result<VerifiedDirectAccumRecipeProductV1, DirectAccumRecipeProducerRejectV1> {
    let (facts, source) = demand.into_parts();
    let shape = facts
        .into_direct_accum_v1()
        .map_err(DirectAccumRecipeProducerRejectV1::FactsPayload)?;
    validate_roles(&shape)?;
    let loop_site = source.site().clone();
    let source_root = crate::mir::loop_structural_facts::bind_resolved_loop_root_v1(source)
        .map_err(DirectAccumRecipeProducerRejectV1::SourceBinding)?;
    let recipe = direct_accum_recipe(&shape);
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(DirectAccumRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::DirectAccumV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact.clone())
        .map_err(DirectAccumRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(DirectAccumRecipeProducerRejectV1::JoinSig)?;
    let owner = shape.induction.owner();
    let binding_rows = binding_relations(function, &shape)?;
    let effects = effect_relations(owner, &loop_site, &shape);
    let core =
        issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, binding_rows, effects)
            .map_err(DirectAccumRecipeProducerRejectV1::Core)?;
    let input_rows = input_relations(function, &shape)?;
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(DirectAccumRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(owner, &loop_site, &shape);
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(DirectAccumRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedDirectAccumRecipeProductV1 {
        operations,
        inputs: input_set,
    })
}

fn validate_roles(
    shape: &DirectAccumStructuralShapeV1,
) -> Result<(), DirectAccumRecipeProducerRejectV1> {
    if shape.induction == shape.accumulator {
        return Err(DirectAccumRecipeProducerRejectV1::ConflictingBindingRoles);
    }
    if shape.condition_binding != shape.induction || shape.step.binding != shape.induction {
        return Err(DirectAccumRecipeProducerRejectV1::InductionRoleMismatch);
    }
    if shape.update.binding != shape.accumulator {
        return Err(DirectAccumRecipeProducerRejectV1::AccumulatorRoleMismatch);
    }
    Ok(())
}

fn declaration_site(
    function: &VerifiedResolvedFunctionV1,
    binding: BindingRefV1,
) -> Result<super::binding_declaration::ResolvedLoopBindingDeclarationV1, DirectAccumRecipeProducerRejectV1>
{
    resolve_loop_binding_declaration_v1(function, binding)
        .ok_or(DirectAccumRecipeProducerRejectV1::BindingDeclaration(binding))
}

fn binding_relations(
    function: &VerifiedResolvedFunctionV1,
    shape: &DirectAccumStructuralShapeV1,
) -> Result<Vec<LoopRecipeBindingRelationV1>, DirectAccumRecipeProducerRejectV1> {
    [
        (LoopBindingKeyV1::new(0), shape.induction),
        (LoopBindingKeyV1::new(1), shape.accumulator),
    ]
    .into_iter()
    .map(|(recipe_binding, source_binding)| {
        Ok(LoopRecipeBindingRelationV1::new(
            recipe_binding,
            source_binding,
            LoopValueClassV1::I64,
            BindingOriginV1::Source(declaration_site(function, source_binding)?.declaration),
        ))
    })
    .collect()
}

fn input_relations(
    function: &VerifiedResolvedFunctionV1,
    shape: &DirectAccumStructuralShapeV1,
) -> Result<Vec<LoopInitializedLocalInputSourceRelationV1>, DirectAccumRecipeProducerRejectV1> {
    [
        (LoopValueKeyV1::new(0), shape.induction),
        (LoopValueKeyV1::new(1), shape.accumulator),
    ]
    .into_iter()
    .map(|(recipe_value, source_binding)| {
        let declaration = declaration_site(function, source_binding)?;
        let initializer = declaration.initializer.clone().ok_or(
            DirectAccumRecipeProducerRejectV1::BindingInitializer(source_binding),
        )?;
        Ok(LoopInitializedLocalInputSourceRelationV1::new(
            declaration.declaration,
            initializer,
            source_binding,
            recipe_value,
            LoopValueClassV1::I64,
        ))
    })
    .collect()
}

fn update_effects(
    owner: FunctionOwnerIdV1,
    update: &DirectAccumUpdateShapeV1,
    recipe_binding: LoopBindingKeyV1,
    read_ordinal: u32,
) -> [LoopBindingEffectRelationV1; 2] {
    let expr = |site: &crate::mir::resolved_semantics::SourceExprSiteV1| {
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site.clone()))
    };
    [
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead {
                ordinal: read_ordinal,
            },
            recipe_binding,
            update.binding,
            LoopValueClassV1::I64,
            expr(&update.lhs_site),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            recipe_binding,
            update.binding,
            LoopValueClassV1::I64,
            expr(&update.target_site),
        ),
    ]
}

fn effect_relations(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceStmtSiteV1,
    shape: &DirectAccumStructuralShapeV1,
) -> Vec<LoopBindingEffectRelationV1> {
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let carrier = |carrier: u32, recipe_binding, source_binding| {
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            recipe_binding,
            source_binding,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: loop_site.clone(),
                carrier: LoopCarrierKeyV1::new(carrier),
            },
        )
    };
    let mut effects = vec![
        carrier(0, induction, shape.induction),
        carrier(1, accumulator, shape.accumulator),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            induction,
            shape.induction,
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(
                owner,
                shape.condition_lhs_site.clone(),
            )),
        ),
    ];
    effects.extend(update_effects(owner, &shape.update, accumulator, 0));
    effects.extend(update_effects(owner, &shape.step, induction, 1));
    effects
}

fn operation_evidence(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceStmtSiteV1,
    shape: &DirectAccumStructuralShapeV1,
) -> Vec<LoopOperationSourceEvidenceV1> {
    let row =
        |item: u32, block: u32, site: crate::mir::resolved_semantics::SourceExprSiteV1, binding| {
            LoopOperationSourceEvidenceV1::new(
                LoopItemKeyV1::new(item),
                LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
                loop_site.clone(),
                LoopNodeKeyV1::new(0),
                LoopBlockKeyV1::new(block),
                binding,
            )
        };
    let update_rows = |start: u32, update: &DirectAccumUpdateShapeV1| {
        [
            row(start, 1, update.lhs_site.clone(), Some(update.binding)),
            row(start + 1, 1, update.rhs_site.clone(), None),
            row(start + 2, 1, update.value_site.clone(), None),
            row(start + 3, 1, update.target_site.clone(), Some(update.binding)),
        ]
    };
    let mut rows = vec![
        row(0, 0, shape.condition_site.clone(), None),
        row(1, 0, shape.condition_lhs_site.clone(), Some(shape.induction)),
        row(2, 0, shape.condition_site.clone(), None),
    ];
    rows.extend(update_rows(3, &shape.update));
    rows.extend(update_rows(7, &shape.step));
    rows
}

pub(super) fn direct_accum_recipe(shape: &DirectAccumStructuralShapeV1) -> LoopRecipeV1 {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let values = (0..=10)
        .map(|raw| LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(raw),
            class: if raw == 4 {
                LoopValueClassV1::Bool
            } else {
                LoopValueClassV1::I64
            },
        })
        .collect();
    let item = |key, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item,
    };
    let mut items = vec![
        item(
            0,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(3),
                    value: shape.condition_bound,
                },
            },
        ),
        item(
            1,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: induction,
                    result: LoopValueKeyV1::new(2),
                },
            },
        ),
        item(
            2,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::CompareI64 {
                    op: LoopCompareI64OpV1::Less,
                    left: LoopValueKeyV1::new(2),
                    right: LoopValueKeyV1::new(3),
                    result: LoopValueKeyV1::new(4),
                },
            },
        ),
    ];
    items.extend(body_update_items(3, accumulator, shape.update.delta));
    items.extend(body_update_items(7, induction, shape.step.delta));

    LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![super::schema::LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: condition_block,
                value: LoopValueKeyV1::new(4),
            },
            body: body_block,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: vec![
                    LoopItemKeyV1::new(0),
                    LoopItemKeyV1::new(1),
                    LoopItemKeyV1::new(2),
                ],
            },
            LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: (3..=10).map(LoopItemKeyV1::new).collect(),
            },
        ],
        items,
        bindings: vec![
            LoopRecipeBindingV1 {
                key: induction,
                label: "induction".into(),
                class: LoopValueClassV1::I64,
            },
            LoopRecipeBindingV1 {
                key: accumulator,
                label: "accumulator".into(),
                class: LoopValueClassV1::I64,
            },
        ],
        values,
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(0),
                owner_loop: loop_key,
                binding: induction,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(0),
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(1),
                owner_loop: loop_key,
                binding: accumulator,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(1),
            },
        ],
        exits: Vec::new(),
    }
}

fn body_update_items(
    start: u32,
    binding: LoopBindingKeyV1,
    delta: i64,
) -> [LoopRecipeItemRowV1; 4] {
    let read = if binding == LoopBindingKeyV1::new(1) {
        5
    } else {
        8
    };
    let constant = if binding == LoopBindingKeyV1::new(1) {
        6
    } else {
        9
    };
    let result = if binding == LoopBindingKeyV1::new(1) {
        7
    } else {
        10
    };
    [
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding,
                    result: LoopValueKeyV1::new(read),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 1),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(constant),
                    value: delta,
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 2),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::BinaryI64 {
                    op: LoopBinaryI64OpV1::Add,
                    left: LoopValueKeyV1::new(read),
                    right: LoopValueKeyV1::new(constant),
                    result: LoopValueKeyV1::new(result),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(start + 3),
            item: LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::WriteBinding {
                    binding,
                    value: LoopValueKeyV1::new(result),
                },
            },
        },
    ]
}
