//! Deterministic producer for the variable-accumulator recurrence.
//!
//! The producer consumes one atomic neutral Facts product and projects it into
//! the existing LoopRecipe/JoinSig/Core algebra.  It does not inspect source,
//! classify families, or select a physical route.

use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, VariableAccumRecurrenceBindingRoleV1,
    VariableAccumRecurrenceInputRoleV1,
    VerifiedVariableAccumRecurrenceFactsV1,
};
use crate::mir::resolved_semantics::{BindingOriginV1, OwnedExprSiteV1};

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

#[derive(Debug)]
pub(crate) enum VariableAccumRecurrenceRecipeProducerRejectV1 {
    SourceBinding,
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

/// One terminal product for all source relations.  The operation product owns
/// the Core; the initialized-input set is checked against that same Core.
#[derive(Debug)]
pub(crate) struct VerifiedVariableAccumRecurrenceRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
}

impl VerifiedVariableAccumRecurrenceRecipeProductV1 {
    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }

    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }
}

pub(crate) fn produce_variable_accum_recurrence_recipe_v1(
    facts: VerifiedVariableAccumRecurrenceFactsV1,
) -> Result<
    VerifiedVariableAccumRecurrenceRecipeProductV1,
    VariableAccumRecurrenceRecipeProducerRejectV1,
> {
    let (source, owner, _scope, bindings, inputs, condition, update, step, _coverage) =
        facts.into_parts();
    let loop_site = source.site().clone();
    let source_root = bind_resolved_loop_root_v1(source)
        .map_err(|_| VariableAccumRecurrenceRecipeProducerRejectV1::SourceBinding)?;
    let recipe = recurrence_recipe(condition.bound(), step.delta());
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumRecurrenceV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact.clone())
        .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::JoinSig)?;
    let binding_rows = binding_relations(&bindings);
    let effects = effect_relations(owner, &loop_site, &condition, &update, &step);
    let core =
        issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, binding_rows, effects)
            .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::Core)?;
    let input_rows = input_relations(&inputs);
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(owner, &loop_site, &condition, &update, &step);
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(VariableAccumRecurrenceRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedVariableAccumRecurrenceRecipeProductV1 {
        operations,
        inputs: input_set,
    })
}

fn recurrence_recipe(bound: i64, delta: i64) -> LoopRecipeV1 {
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
    let row = |key, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(key),
        item,
    };
    let items = vec![
        row(
            0,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(3),
                    value: bound,
                },
            },
        ),
        row(
            1,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: induction,
                    result: LoopValueKeyV1::new(2),
                },
            },
        ),
        row(
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
        row(
            3,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: accumulator,
                    result: LoopValueKeyV1::new(5),
                },
            },
        ),
        row(
            4,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: induction,
                    result: LoopValueKeyV1::new(6),
                },
            },
        ),
        row(
            5,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::BinaryI64 {
                    op: LoopBinaryI64OpV1::Add,
                    left: LoopValueKeyV1::new(5),
                    right: LoopValueKeyV1::new(6),
                    result: LoopValueKeyV1::new(7),
                },
            },
        ),
        row(
            6,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::WriteBinding {
                    binding: accumulator,
                    value: LoopValueKeyV1::new(7),
                },
            },
        ),
        row(
            7,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ReadBinding {
                    binding: induction,
                    result: LoopValueKeyV1::new(8),
                },
            },
        ),
        row(
            8,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::ConstI64 {
                    result: LoopValueKeyV1::new(9),
                    value: delta,
                },
            },
        ),
        row(
            9,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::BinaryI64 {
                    op: LoopBinaryI64OpV1::Add,
                    left: LoopValueKeyV1::new(8),
                    right: LoopValueKeyV1::new(9),
                    result: LoopValueKeyV1::new(10),
                },
            },
        ),
        row(
            10,
            LoopRecipeItemV1::Operation {
                operation: LoopOperationV1::WriteBinding {
                    binding: induction,
                    value: LoopValueKeyV1::new(10),
                },
            },
        ),
    ];
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

fn binding_relations(
    rows: &[crate::mir::loop_structural_facts::VariableAccumRecurrenceBindingObservationV1; 2],
) -> Vec<LoopRecipeBindingRelationV1> {
    rows.iter()
        .map(|row| {
            LoopRecipeBindingRelationV1::new(
                recipe_binding(row.role()),
                row.binding(),
                LoopValueClassV1::I64,
                BindingOriginV1::Source(row.declaration().clone()),
            )
        })
        .collect()
}

fn input_relations(
    rows: &[crate::mir::loop_structural_facts::VariableAccumRecurrenceInputObservationV1; 2],
) -> Vec<LoopInitializedLocalInputSourceRelationV1> {
    rows.iter()
        .map(|row| {
            LoopInitializedLocalInputSourceRelationV1::new(
                row.declaration().clone(),
                row.initializer().clone(),
                row.binding(),
                recipe_input(row.role()),
                LoopValueClassV1::I64,
            )
        })
        .collect()
}

fn effect_relations(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    loop_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    condition: &crate::mir::loop_structural_facts::VariableAccumRecurrenceConditionObservationV1,
    update: &crate::mir::loop_structural_facts::VariableAccumRecurrenceAccumulatorUpdateV1,
    step: &crate::mir::loop_structural_facts::VariableAccumRecurrenceInductionStepV1,
) -> Vec<LoopBindingEffectRelationV1> {
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let expr = |site| LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site));
    vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            induction,
            condition.induction(),
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: loop_site.clone(),
                carrier: LoopCarrierKeyV1::new(0),
            },
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            accumulator,
            update.accumulator(),
            LoopValueClassV1::I64,
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner,
                source_loop: loop_site.clone(),
                carrier: LoopCarrierKeyV1::new(1),
            },
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            induction,
            condition.induction(),
            LoopValueClassV1::I64,
            expr(condition.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            induction,
            update.induction(),
            LoopValueClassV1::I64,
            expr(update.rhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 2 },
            induction,
            step.induction(),
            LoopValueClassV1::I64,
            expr(step.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            accumulator,
            update.accumulator(),
            LoopValueClassV1::I64,
            expr(update.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            accumulator,
            update.accumulator(),
            LoopValueClassV1::I64,
            expr(update.target().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            induction,
            step.induction(),
            LoopValueClassV1::I64,
            expr(step.target().clone()),
        ),
    ]
}

fn operation_evidence(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    loop_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    condition: &crate::mir::loop_structural_facts::VariableAccumRecurrenceConditionObservationV1,
    update: &crate::mir::loop_structural_facts::VariableAccumRecurrenceAccumulatorUpdateV1,
    step: &crate::mir::loop_structural_facts::VariableAccumRecurrenceInductionStepV1,
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
    vec![
        row(0, 0, condition.rhs().clone(), None),
        row(1, 0, condition.lhs().clone(), Some(condition.induction())),
        row(2, 0, condition.site().clone(), None),
        row(3, 1, update.lhs().clone(), Some(update.accumulator())),
        row(4, 1, update.rhs().clone(), Some(update.induction())),
        row(5, 1, update.value().clone(), None),
        row(6, 1, update.target().clone(), Some(update.accumulator())),
        row(7, 1, step.lhs().clone(), Some(step.induction())),
        row(8, 1, step.rhs().clone(), None),
        row(9, 1, step.value().clone(), None),
        row(10, 1, step.target().clone(), Some(step.induction())),
    ]
}

fn recipe_binding(role: VariableAccumRecurrenceBindingRoleV1) -> LoopBindingKeyV1 {
    match role {
        VariableAccumRecurrenceBindingRoleV1::Induction => LoopBindingKeyV1::new(0),
        VariableAccumRecurrenceBindingRoleV1::Accumulator => LoopBindingKeyV1::new(1),
    }
}

fn recipe_input(role: VariableAccumRecurrenceInputRoleV1) -> LoopValueKeyV1 {
    match role {
        VariableAccumRecurrenceInputRoleV1::InductionInitial => LoopValueKeyV1::new(0),
        VariableAccumRecurrenceInputRoleV1::AccumulatorInitial => LoopValueKeyV1::new(1),
    }
}
