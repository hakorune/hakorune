//! Logical transfer obligations for the verified fixed-shell If recipe.
//!
//! This module is caller-zero.  It has no source navigation, physical IDs,
//! Builder access, CFG allocation, or PHI authority.  The input recipe has
//! already been structurally verified; this layer only seals its fixed logical
//! edge/value correspondence for the later physical consumer.

use super::ids::{IfBindingKeyV1, IfValueKeyV1};
use super::schema::{IfElseDispositionV1, IfRecipeBindingV1, IfValueClassV1};
use super::verify::VerifiedIfRecipeV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum IfJoinPortV1 {
    Entry,
    Condition,
    Then,
    Else,
    Baseline,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum IfJoinEdgeRoleV1 {
    Enter,
    True,
    False,
    ThenTransfer,
    ElseTransfer,
    ImplicitBaseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IfJoinValueEdgeV1 {
    pub(crate) value: IfValueKeyV1,
    pub(crate) class: IfValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IfJoinEdgeV1 {
    pub(crate) from: IfJoinPortV1,
    pub(crate) to: IfJoinPortV1,
    pub(crate) role: IfJoinEdgeRoleV1,
    pub(crate) value: IfJoinValueEdgeV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IfJoinObligationV1 {
    pub(crate) binding: IfBindingKeyV1,
    pub(crate) class: IfValueClassV1,
    pub(crate) entry_value: IfValueKeyV1,
    pub(crate) then_value: IfValueKeyV1,
    pub(crate) else_value: IfValueKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IfJoinSigV1 {
    pub(crate) disposition: IfElseDispositionV1,
    pub(crate) ports: [IfJoinPortV1; 5],
    pub(crate) edges: [IfJoinEdgeV1; 5],
    pub(crate) join: IfJoinObligationV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedIfJoinSigV1(IfJoinSigV1);

impl VerifiedIfJoinSigV1 {
    pub(crate) fn as_sig(&self) -> &IfJoinSigV1 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IfJoinSigRejectReasonV1 {
    UnsupportedElseDisposition,
    PredecessorCountMismatch { found: usize },
    NonDistinctPredecessor,
    MissingJoinBinding { binding: IfBindingKeyV1 },
    MissingJoinValue { value: IfValueKeyV1 },
    ValueClassMismatch { value: IfValueKeyV1 },
    MissingContinuationTransfer,
    LogicalEdgeMismatch,
}

pub(crate) struct IfJoinSigElaboratorV1;

impl IfJoinSigElaboratorV1 {
    pub(crate) fn elaborate(
        verified: &VerifiedIfRecipeV1,
    ) -> Result<VerifiedIfJoinSigV1, IfJoinSigRejectReasonV1> {
        let recipe = verified.as_recipe();
        let disposition = recipe.else_disposition;
        if disposition == IfElseDispositionV1::Explicit && recipe.else_block.is_none() {
            return Err(IfJoinSigRejectReasonV1::UnsupportedElseDisposition);
        }
        if disposition == IfElseDispositionV1::ImplicitFallthrough && recipe.else_block.is_some() {
            return Err(IfJoinSigRejectReasonV1::UnsupportedElseDisposition);
        }
        if recipe.joins.len() != 1 {
            return Err(IfJoinSigRejectReasonV1::PredecessorCountMismatch {
                found: recipe.joins.len(),
            });
        }
        let join = recipe.joins[0];
        if !is_merge_target(&recipe.bindings, join.binding) {
            return Err(IfJoinSigRejectReasonV1::MissingJoinBinding {
                binding: join.binding,
            });
        }

        let entry_class = value_class(recipe, join.entry_value)?;
        let then_class = value_class(recipe, join.then_value)?;
        let else_class = value_class(recipe, join.else_value)?;
        if entry_class != join.class || then_class != join.class {
            return Err(IfJoinSigRejectReasonV1::ValueClassMismatch {
                value: join.entry_value,
            });
        }
        if else_class != join.class {
            return Err(IfJoinSigRejectReasonV1::ValueClassMismatch {
                value: join.else_value,
            });
        }
        if disposition == IfElseDispositionV1::ImplicitFallthrough
            && join.else_value != join.entry_value
        {
            return Err(IfJoinSigRejectReasonV1::LogicalEdgeMismatch);
        }
        if !recipe.inputs.contains(&join.entry_value) {
            return Err(IfJoinSigRejectReasonV1::MissingJoinValue {
                value: join.entry_value,
            });
        }
        if recipe.continuation.required_read != join.binding {
            return Err(IfJoinSigRejectReasonV1::MissingContinuationTransfer);
        }

        let condition = value_class(recipe, recipe.condition)?;
        if condition != IfValueClassV1::Bool {
            return Err(IfJoinSigRejectReasonV1::ValueClassMismatch {
                value: recipe.condition,
            });
        }
        let ports = match disposition {
            IfElseDispositionV1::Explicit => [
                IfJoinPortV1::Entry,
                IfJoinPortV1::Condition,
                IfJoinPortV1::Then,
                IfJoinPortV1::Else,
                IfJoinPortV1::Continuation,
            ],
            IfElseDispositionV1::ImplicitFallthrough => [
                IfJoinPortV1::Entry,
                IfJoinPortV1::Condition,
                IfJoinPortV1::Then,
                IfJoinPortV1::Baseline,
                IfJoinPortV1::Continuation,
            ],
        };
        let edges = match disposition {
            IfElseDispositionV1::Explicit => [
                edge(
                    IfJoinPortV1::Entry,
                    IfJoinPortV1::Condition,
                    IfJoinEdgeRoleV1::Enter,
                    join.entry_value,
                    join.class,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Then,
                    IfJoinEdgeRoleV1::True,
                    recipe.condition,
                    IfValueClassV1::Bool,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Else,
                    IfJoinEdgeRoleV1::False,
                    recipe.condition,
                    IfValueClassV1::Bool,
                ),
                edge(
                    IfJoinPortV1::Then,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ThenTransfer,
                    join.then_value,
                    join.class,
                ),
                edge(
                    IfJoinPortV1::Else,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ElseTransfer,
                    join.else_value,
                    join.class,
                ),
            ],
            IfElseDispositionV1::ImplicitFallthrough => [
                edge(
                    IfJoinPortV1::Entry,
                    IfJoinPortV1::Condition,
                    IfJoinEdgeRoleV1::Enter,
                    join.entry_value,
                    join.class,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Then,
                    IfJoinEdgeRoleV1::True,
                    recipe.condition,
                    IfValueClassV1::Bool,
                ),
                edge(
                    IfJoinPortV1::Condition,
                    IfJoinPortV1::Baseline,
                    IfJoinEdgeRoleV1::False,
                    recipe.condition,
                    IfValueClassV1::Bool,
                ),
                edge(
                    IfJoinPortV1::Then,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ThenTransfer,
                    join.then_value,
                    join.class,
                ),
                edge(
                    IfJoinPortV1::Baseline,
                    IfJoinPortV1::Continuation,
                    IfJoinEdgeRoleV1::ImplicitBaseline,
                    join.entry_value,
                    join.class,
                ),
            ],
        };
        if edges[0].from != IfJoinPortV1::Entry
            || edges[1].role != IfJoinEdgeRoleV1::True
            || edges[2].role != IfJoinEdgeRoleV1::False
            || edges[3].to != IfJoinPortV1::Continuation
            || edges[4].to != IfJoinPortV1::Continuation
        {
            return Err(IfJoinSigRejectReasonV1::LogicalEdgeMismatch);
        }
        Ok(VerifiedIfJoinSigV1(IfJoinSigV1 {
            disposition,
            ports,
            edges,
            join: IfJoinObligationV1 {
                binding: join.binding,
                class: join.class,
                entry_value: join.entry_value,
                then_value: join.then_value,
                else_value: join.else_value,
            },
        }))
    }
}

fn edge(
    from: IfJoinPortV1,
    to: IfJoinPortV1,
    role: IfJoinEdgeRoleV1,
    value: IfValueKeyV1,
    class: IfValueClassV1,
) -> IfJoinEdgeV1 {
    IfJoinEdgeV1 {
        from,
        to,
        role,
        value: IfJoinValueEdgeV1 { value, class },
    }
}

fn is_merge_target(bindings: &[IfRecipeBindingV1], expected: IfBindingKeyV1) -> bool {
    bindings.iter().any(|binding| {
        binding.key == expected
            && matches!(binding.role, super::schema::IfBindingRoleV1::MergeTarget)
    })
}

fn value_class(
    recipe: &super::schema::IfRecipeV1,
    value: IfValueKeyV1,
) -> Result<IfValueClassV1, IfJoinSigRejectReasonV1> {
    match recipe.values.iter().find(|row| row.key == value) {
        Some(row) => Ok(row.class),
        None => Err(IfJoinSigRejectReasonV1::MissingJoinValue { value }),
    }
}
