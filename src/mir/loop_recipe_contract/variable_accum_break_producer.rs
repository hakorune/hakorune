//! Deterministic producer for the bounded accumulator-with-break fixture.
//!
//! This module consumes one atomic source Facts product and emits the existing
//! Recipe, JoinSig, source-bound Core, input set, and operation evidence.  It
//! never rereads syntax, selects a route, or creates physical identities.

use crate::mir::loop_structural_facts::{
    bind_resolved_loop_root_v1, VariableAccumBreakAssignmentObservationV1,
    VariableAccumBreakBindingRoleV1, VariableAccumBreakConditionObservationV1,
    VariableAccumBreakInputRoleV1, VerifiedVariableAccumBreakFactsV1,
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
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1, LoopOperationV1,
    LoopRecipeArtifactV1, LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1,
    LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1,
};
use super::source_bound_core::{
    issue_source_bound_core_from_artifact_v1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
};
use super::verify::LoopRecipeVerifierV1;

#[derive(Debug)]
pub(crate) enum VariableAccumBreakRecipeProducerRejectV1 {
    SourceBinding,
    Recipe(LoopRecipeRejectReasonV1),
    JoinSig(LoopJoinSigRejectReasonV1),
    Core(LoopRecipeRejectReasonV1),
    Inputs(LoopInitializedLocalInputSourceSetRejectV1),
    Operations(LoopOperationEffectRejectV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedVariableAccumBreakRecipeProductV1 {
    operations: VerifiedLoopOperationEffectProductV1,
    inputs: VerifiedLoopInitializedLocalInputSourceSetV1,
    control_source: VariableAccumBreakControlSourceReceiptV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VariableAccumBreakControlSourceReceiptV1 {
    branch_site: crate::mir::resolved_semantics::SourceStmtSiteV1,
    break_site: crate::mir::resolved_semantics::SourceStmtSiteV1,
}

impl VariableAccumBreakControlSourceReceiptV1 {
    pub(crate) fn branch_site(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.branch_site
    }

    pub(crate) fn break_site(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.break_site
    }
}

impl VerifiedVariableAccumBreakRecipeProductV1 {
    pub(crate) fn operations(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operations
    }
    pub(crate) fn inputs(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.inputs
    }
    pub(crate) fn control_source(&self) -> &VariableAccumBreakControlSourceReceiptV1 {
        &self.control_source
    }
    pub(crate) fn recipe(&self) -> &super::verify::VerifiedLoopRecipeV1 {
        self.operations.core().recipe()
    }
}

pub(crate) fn produce_variable_accum_break_recipe_v1(
    facts: VerifiedVariableAccumBreakFactsV1,
) -> Result<VerifiedVariableAccumBreakRecipeProductV1, VariableAccumBreakRecipeProducerRejectV1> {
    let (
        source,
        owner,
        _scope,
        bindings,
        inputs,
        loop_condition,
        branch_condition,
        terminal_update,
        normal_update,
        induction_step,
        branch_site,
        break_site,
        _coverage,
    ) = facts.into_parts();
    let loop_site = source.site().clone();
    let source_root = bind_resolved_loop_root_v1(source)
        .map_err(|_| VariableAccumBreakRecipeProducerRejectV1::SourceBinding)?;
    let recipe = break_recipe(loop_condition.bound(), branch_condition.bound());
    let verified_for_source = LoopRecipeVerifierV1::verify(recipe.clone())
        .map_err(VariableAccumBreakRecipeProducerRejectV1::Recipe)?;
    let source_binding = source_root.into_root_claim(&verified_for_source);
    let artifact = LoopRecipeArtifactV1::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::VariableAccumBreakV1),
        source_binding,
        recipe,
    );
    let verified_artifact = LoopRecipeVerifierV1::verify_artifact(artifact.clone())
        .map_err(VariableAccumBreakRecipeProducerRejectV1::Recipe)?;
    let verified_recipe = verified_artifact.into_recipe();
    let join_sig = LoopJoinSigElaboratorV1::elaborate(&verified_recipe)
        .map_err(VariableAccumBreakRecipeProducerRejectV1::JoinSig)?;
    let binding_rows = binding_relations(&bindings);
    let effects = effect_relations(
        owner,
        &loop_site,
        &loop_condition,
        &branch_condition,
        &terminal_update,
        &normal_update,
        &induction_step,
    );
    let core =
        issue_source_bound_core_from_artifact_v1(artifact, join_sig, owner, binding_rows, effects)
            .map_err(VariableAccumBreakRecipeProducerRejectV1::Core)?;
    let input_rows = input_relations(&inputs);
    let input_set = issue_initialized_local_input_source_set_v1(&core, input_rows)
        .map_err(VariableAccumBreakRecipeProducerRejectV1::Inputs)?;
    let operation_rows = operation_evidence(
        owner,
        &loop_site,
        &loop_condition,
        &branch_condition,
        &terminal_update,
        &normal_update,
        &induction_step,
    );
    let operations = VerifiedLoopOperationEffectProductV1::issue(core, operation_rows)
        .map_err(VariableAccumBreakRecipeProducerRejectV1::Operations)?;
    Ok(VerifiedVariableAccumBreakRecipeProductV1 {
        operations,
        inputs: input_set,
        control_source: VariableAccumBreakControlSourceReceiptV1 {
            branch_site,
            break_site,
        },
    })
}

fn break_recipe(loop_bound: i64, branch_bound: i64) -> LoopRecipeV1 {
    let l0 = LoopNodeKeyV1::new(0);
    let b0 = LoopBlockKeyV1::new(0);
    let b1 = LoopBlockKeyV1::new(1);
    let b2 = LoopBlockKeyV1::new(2);
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let value = |raw: u32, class| LoopRecipeValueV1 {
        key: LoopValueKeyV1::new(raw),
        class,
    };
    let row = |raw: u32, item| LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(raw),
        item,
    };
    let op = |operation| LoopRecipeItemV1::Operation { operation };
    let items = vec![
        row(
            0,
            op(LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(3),
                value: loop_bound,
            }),
        ),
        row(
            1,
            op(LoopOperationV1::ReadBinding {
                binding: induction,
                result: LoopValueKeyV1::new(2),
            }),
        ),
        row(
            2,
            op(LoopOperationV1::CompareI64 {
                op: LoopCompareI64OpV1::Less,
                left: LoopValueKeyV1::new(2),
                right: LoopValueKeyV1::new(3),
                result: LoopValueKeyV1::new(4),
            }),
        ),
        row(
            3,
            op(LoopOperationV1::ReadBinding {
                binding: induction,
                result: LoopValueKeyV1::new(5),
            }),
        ),
        row(
            4,
            op(LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(6),
                value: branch_bound,
            }),
        ),
        row(
            5,
            op(LoopOperationV1::CompareI64 {
                op: LoopCompareI64OpV1::Equal,
                left: LoopValueKeyV1::new(5),
                right: LoopValueKeyV1::new(6),
                result: LoopValueKeyV1::new(7),
            }),
        ),
        row(
            6,
            LoopRecipeItemV1::If {
                condition: LoopValueKeyV1::new(7),
                then_block: b2,
                else_block: None,
            },
        ),
        row(
            7,
            op(LoopOperationV1::ReadBinding {
                binding: accumulator,
                result: LoopValueKeyV1::new(8),
            }),
        ),
        row(
            8,
            op(LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(9),
                value: 10,
            }),
        ),
        row(
            9,
            op(LoopOperationV1::BinaryI64 {
                op: LoopBinaryI64OpV1::Add,
                left: LoopValueKeyV1::new(8),
                right: LoopValueKeyV1::new(9),
                result: LoopValueKeyV1::new(10),
            }),
        ),
        row(
            10,
            op(LoopOperationV1::WriteBinding {
                binding: accumulator,
                value: LoopValueKeyV1::new(10),
            }),
        ),
        row(
            11,
            LoopRecipeItemV1::Exit {
                exit: super::ids::LoopExitKeyV1::new(0),
            },
        ),
        row(
            12,
            op(LoopOperationV1::ReadBinding {
                binding: accumulator,
                result: LoopValueKeyV1::new(11),
            }),
        ),
        row(
            13,
            op(LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(12),
                value: 1,
            }),
        ),
        row(
            14,
            op(LoopOperationV1::BinaryI64 {
                op: LoopBinaryI64OpV1::Add,
                left: LoopValueKeyV1::new(11),
                right: LoopValueKeyV1::new(12),
                result: LoopValueKeyV1::new(13),
            }),
        ),
        row(
            15,
            op(LoopOperationV1::WriteBinding {
                binding: accumulator,
                value: LoopValueKeyV1::new(13),
            }),
        ),
        row(
            16,
            op(LoopOperationV1::ReadBinding {
                binding: induction,
                result: LoopValueKeyV1::new(14),
            }),
        ),
        row(
            17,
            op(LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(15),
                value: 1,
            }),
        ),
        row(
            18,
            op(LoopOperationV1::BinaryI64 {
                op: LoopBinaryI64OpV1::Add,
                left: LoopValueKeyV1::new(14),
                right: LoopValueKeyV1::new(15),
                result: LoopValueKeyV1::new(16),
            }),
        ),
        row(
            19,
            op(LoopOperationV1::WriteBinding {
                binding: induction,
                value: LoopValueKeyV1::new(16),
            }),
        ),
    ];
    LoopRecipeV1 {
        root_loop: l0,
        loops: vec![super::schema::LoopNodeV1 {
            key: l0,
            parent: None,
            condition: LoopConditionV1::Predicate {
                block: b0,
                value: LoopValueKeyV1::new(4),
            },
            body: b1,
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: b0,
                owner_loop: l0,
                items: vec![
                    LoopItemKeyV1::new(0),
                    LoopItemKeyV1::new(1),
                    LoopItemKeyV1::new(2),
                ],
            },
            LoopRecipeBlockV1 {
                key: b1,
                owner_loop: l0,
                items: vec![
                    LoopItemKeyV1::new(3),
                    LoopItemKeyV1::new(4),
                    LoopItemKeyV1::new(5),
                    LoopItemKeyV1::new(6),
                    LoopItemKeyV1::new(12),
                    LoopItemKeyV1::new(13),
                    LoopItemKeyV1::new(14),
                    LoopItemKeyV1::new(15),
                    LoopItemKeyV1::new(16),
                    LoopItemKeyV1::new(17),
                    LoopItemKeyV1::new(18),
                    LoopItemKeyV1::new(19),
                ],
            },
            LoopRecipeBlockV1 {
                key: b2,
                owner_loop: l0,
                items: vec![
                    LoopItemKeyV1::new(7),
                    LoopItemKeyV1::new(8),
                    LoopItemKeyV1::new(9),
                    LoopItemKeyV1::new(10),
                    LoopItemKeyV1::new(11),
                ],
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
        values: (0..=16)
            .map(|raw| {
                value(
                    raw,
                    if raw == 4 || raw == 7 {
                        LoopValueClassV1::Bool
                    } else {
                        LoopValueClassV1::I64
                    },
                )
            })
            .collect(),
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(0),
                owner_loop: l0,
                binding: induction,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(0),
            },
            LoopRecipeCarrierV1 {
                key: LoopCarrierKeyV1::new(1),
                owner_loop: l0,
                binding: accumulator,
                class: LoopValueClassV1::I64,
                entry_value: LoopValueKeyV1::new(1),
            },
        ],
        exits: vec![LoopRecipeExitV1 {
            key: super::ids::LoopExitKeyV1::new(0),
            owner_loop: l0,
            kind: LoopExitKindV1::Break { target_loop: l0 },
        }],
    }
}

fn binding_relations(
    rows: &[crate::mir::loop_structural_facts::VariableAccumBreakBindingObservationV1; 2],
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
    rows: &[crate::mir::loop_structural_facts::VariableAccumBreakInputObservationV1; 2],
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
    loop_condition: &VariableAccumBreakConditionObservationV1,
    branch_condition: &VariableAccumBreakConditionObservationV1,
    terminal: &VariableAccumBreakAssignmentObservationV1,
    normal: &VariableAccumBreakAssignmentObservationV1,
    step: &VariableAccumBreakAssignmentObservationV1,
) -> Vec<LoopBindingEffectRelationV1> {
    let induction = LoopBindingKeyV1::new(0);
    let accumulator = LoopBindingKeyV1::new(1);
    let expr = |site| LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site));
    vec![
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::DerivedCarrierEntry,
            induction,
            loop_condition.binding(),
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
            terminal.target_binding(),
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
            loop_condition.binding(),
            LoopValueClassV1::I64,
            expr(loop_condition.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            induction,
            branch_condition.binding(),
            LoopValueClassV1::I64,
            expr(branch_condition.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 2 },
            induction,
            step.lhs_binding(),
            LoopValueClassV1::I64,
            expr(step.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            accumulator,
            terminal.lhs_binding(),
            LoopValueClassV1::I64,
            expr(terminal.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            accumulator,
            normal.lhs_binding(),
            LoopValueClassV1::I64,
            expr(normal.lhs().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            accumulator,
            terminal.target_binding(),
            LoopValueClassV1::I64,
            expr(terminal.target().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 1 },
            accumulator,
            normal.target_binding(),
            LoopValueClassV1::I64,
            expr(normal.target().clone()),
        ),
        LoopBindingEffectRelationV1::new(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            induction,
            step.target_binding(),
            LoopValueClassV1::I64,
            expr(step.target().clone()),
        ),
    ]
}

fn operation_evidence(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    loop_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    loop_condition: &VariableAccumBreakConditionObservationV1,
    branch_condition: &VariableAccumBreakConditionObservationV1,
    terminal: &VariableAccumBreakAssignmentObservationV1,
    normal: &VariableAccumBreakAssignmentObservationV1,
    step: &VariableAccumBreakAssignmentObservationV1,
) -> Vec<LoopOperationSourceEvidenceV1> {
    let row = |item, block, site, binding| {
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
        row(0, 0, loop_condition.rhs().clone(), None),
        row(
            1,
            0,
            loop_condition.lhs().clone(),
            Some(loop_condition.binding()),
        ),
        row(2, 0, loop_condition.site().clone(), None),
        row(
            3,
            1,
            branch_condition.lhs().clone(),
            Some(branch_condition.binding()),
        ),
        row(4, 1, branch_condition.rhs().clone(), None),
        row(5, 1, branch_condition.site().clone(), None),
        row(7, 2, terminal.lhs().clone(), Some(terminal.lhs_binding())),
        row(8, 2, terminal.rhs().clone(), None),
        row(9, 2, terminal.value().clone(), None),
        row(
            10,
            2,
            terminal.target().clone(),
            Some(terminal.target_binding()),
        ),
        row(12, 1, normal.lhs().clone(), Some(normal.lhs_binding())),
        row(13, 1, normal.rhs().clone(), None),
        row(14, 1, normal.value().clone(), None),
        row(
            15,
            1,
            normal.target().clone(),
            Some(normal.target_binding()),
        ),
        row(16, 1, step.lhs().clone(), Some(step.lhs_binding())),
        row(17, 1, step.rhs().clone(), None),
        row(18, 1, step.value().clone(), None),
        row(19, 1, step.target().clone(), Some(step.target_binding())),
    ]
}

fn recipe_binding(role: VariableAccumBreakBindingRoleV1) -> LoopBindingKeyV1 {
    match role {
        VariableAccumBreakBindingRoleV1::Induction => LoopBindingKeyV1::new(0),
        VariableAccumBreakBindingRoleV1::Accumulator => LoopBindingKeyV1::new(1),
    }
}

fn recipe_input(role: VariableAccumBreakInputRoleV1) -> LoopValueKeyV1 {
    match role {
        VariableAccumBreakInputRoleV1::InductionInitial => LoopValueKeyV1::new(0),
        VariableAccumBreakInputRoleV1::AccumulatorInitial => LoopValueKeyV1::new(1),
    }
}
