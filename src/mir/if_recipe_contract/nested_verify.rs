//! Structural verification for the isolated depth-one Nested If shell.

use std::collections::BTreeSet;

use super::nested_schema::{
    NestedIfExprKindV1, NestedIfRecipeArtifactV1, NestedIfRecipeProfileV1,
    NestedIfRecipeSourceBindingV1, NestedIfSourceClaimRoleV1, NestedIfSourcePathStepV1,
    NESTED_IF_RECIPE_SCHEMA_VERSION_V1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedIfRecipeRejectReasonV1 {
    UnsupportedVersion { found: u16 },
    ProfileMismatch,
    SourceClaimCountMismatch { found: usize },
    SourceClaimOrderMismatch,
    InvalidSourcePath,
    NodeCountMismatch { found: usize },
    ExpressionMissing,
    BindingCountMismatch { found: usize },
    BindingClassMismatch,
    NodeKeyMismatch,
    ChildShapeMismatch,
    AssignmentShapeMismatch,
    JoinShapeMismatch,
    ContinuationMismatch,
    DuplicateValueKey,
    DanglingValue,
    ValueClassMismatch,
    BinaryClassMismatch,
}

#[derive(Debug)]
pub(crate) struct VerifiedNestedIfRecipeArtifactV1 {
    artifact: NestedIfRecipeArtifactV1,
}

impl VerifiedNestedIfRecipeArtifactV1 {
    pub(crate) fn artifact(&self) -> &NestedIfRecipeArtifactV1 {
        &self.artifact
    }

    pub(crate) fn recipe(&self) -> &super::nested_schema::NestedIfRecipeV1 {
        &self.artifact.recipe
    }
}

pub(crate) struct NestedIfRecipeVerifierV1;

impl NestedIfRecipeVerifierV1 {
    pub(crate) fn verify_artifact(
        artifact: NestedIfRecipeArtifactV1,
    ) -> Result<VerifiedNestedIfRecipeArtifactV1, NestedIfRecipeRejectReasonV1> {
        if artifact.schema_version != NESTED_IF_RECIPE_SCHEMA_VERSION_V1 {
            return Err(NestedIfRecipeRejectReasonV1::UnsupportedVersion {
                found: artifact.schema_version,
            });
        }
        if artifact.provenance.profile
            != NestedIfRecipeProfileV1::ResolvedTrivialExplicitElseDepthOne
        {
            return Err(NestedIfRecipeRejectReasonV1::ProfileMismatch);
        }
        verify_source_binding(&artifact.source_binding)?;
        verify_recipe(&artifact.recipe)?;
        Ok(VerifiedNestedIfRecipeArtifactV1 { artifact })
    }
}

fn verify_source_binding(
    source: &NestedIfRecipeSourceBindingV1,
) -> Result<(), NestedIfRecipeRejectReasonV1> {
    if source.claims.len() != 8 {
        return Err(NestedIfRecipeRejectReasonV1::SourceClaimCountMismatch {
            found: source.claims.len(),
        });
    }
    let expected = [
        NestedIfSourceClaimRoleV1::OuterIfNode,
        NestedIfSourceClaimRoleV1::OuterCondition,
        NestedIfSourceClaimRoleV1::InnerIfNode,
        NestedIfSourceClaimRoleV1::InnerCondition,
        NestedIfSourceClaimRoleV1::InnerThenAssignment,
        NestedIfSourceClaimRoleV1::InnerElseAssignment,
        NestedIfSourceClaimRoleV1::OuterElseAssignment,
        NestedIfSourceClaimRoleV1::ContinuationRead,
    ];
    if source
        .claims
        .iter()
        .map(|claim| claim.role)
        .zip(expected)
        .any(|(found, expected)| found != expected)
    {
        return Err(NestedIfRecipeRejectReasonV1::SourceClaimOrderMismatch);
    }
    let root = match source.claims[0].path.steps.as_slice() {
        [NestedIfSourcePathStepV1::BodyItem(index)] => *index,
        _ => return Err(NestedIfRecipeRejectReasonV1::InvalidSourcePath),
    };
    if source.claims[1].path.steps
        != [
            NestedIfSourcePathStepV1::BodyItem(root),
            NestedIfSourcePathStepV1::IfCondition,
        ]
        || !matches!(
            source.claims[2].path.steps.as_slice(),
            [NestedIfSourcePathStepV1::BodyItem(found), NestedIfSourcePathStepV1::IfThenItem(_)]
                if *found == root
        )
    {
        return Err(NestedIfRecipeRejectReasonV1::InvalidSourcePath);
    }
    let inner_prefix = match source.claims[2].path.steps.as_slice() {
        [NestedIfSourcePathStepV1::BodyItem(_), NestedIfSourcePathStepV1::IfThenItem(index)] => {
            *index
        }
        _ => return Err(NestedIfRecipeRejectReasonV1::InvalidSourcePath),
    };
    let inner_prefix = [
        NestedIfSourcePathStepV1::BodyItem(root),
        NestedIfSourcePathStepV1::IfThenItem(inner_prefix),
    ];
    if source.claims[3].path.steps
        != [
            inner_prefix[0].clone(),
            inner_prefix[1].clone(),
            NestedIfSourcePathStepV1::IfCondition,
        ]
        || !matches!(
            source.claims[4].path.steps.as_slice(),
            [
                NestedIfSourcePathStepV1::BodyItem(found),
                NestedIfSourcePathStepV1::IfThenItem(_),
                NestedIfSourcePathStepV1::IfThenItem(_)
            ] if *found == root
        )
        || !matches!(
            source.claims[5].path.steps.as_slice(),
            [
                NestedIfSourcePathStepV1::BodyItem(found),
                NestedIfSourcePathStepV1::IfThenItem(_),
                NestedIfSourcePathStepV1::IfElseItem(_)
            ] if *found == root
        )
        || !matches!(
            source.claims[6].path.steps.as_slice(),
            [
                NestedIfSourcePathStepV1::BodyItem(found),
                NestedIfSourcePathStepV1::IfElseItem(_)
            ] if *found == root
        )
    {
        return Err(NestedIfRecipeRejectReasonV1::InvalidSourcePath);
    }
    let continuation = source.claims[7].path.steps.as_slice();
    if !matches!(
        continuation,
        [NestedIfSourcePathStepV1::BodyItem(index), NestedIfSourcePathStepV1::Value]
            if *index > root
    ) {
        return Err(NestedIfRecipeRejectReasonV1::InvalidSourcePath);
    }
    Ok(())
}

fn verify_recipe(
    recipe: &super::nested_schema::NestedIfRecipeV1,
) -> Result<(), NestedIfRecipeRejectReasonV1> {
    if recipe.nodes.len() != 2 {
        return Err(NestedIfRecipeRejectReasonV1::NodeCountMismatch {
            found: recipe.nodes.len(),
        });
    }
    if recipe.bindings.len() != 1 {
        return Err(NestedIfRecipeRejectReasonV1::BindingCountMismatch {
            found: recipe.bindings.len(),
        });
    }
    if recipe.bindings[0].class != super::nested_schema::NestedIfValueClassV1::I64 {
        return Err(NestedIfRecipeRejectReasonV1::BindingClassMismatch);
    }
    let outer = &recipe.nodes[0];
    let inner = &recipe.nodes[1];
    if outer.key.raw() != 0 || inner.key.raw() != 1 {
        return Err(NestedIfRecipeRejectReasonV1::NodeKeyMismatch);
    }
    if outer.then_child != Some(inner.key)
        || !outer.then_assignments.is_empty()
        || outer.else_assignments.len() != 1
        || inner.then_child.is_some()
        || inner.then_assignments.len() != 1
        || inner.else_assignments.len() != 1
    {
        return Err(NestedIfRecipeRejectReasonV1::ChildShapeMismatch);
    }
    if outer.join.binding != inner.join.binding
        || outer.join.class != inner.join.class
        || outer.join.entry_value != inner.join.entry_value
        || outer.join.then_value != inner.join.merge_value
        || outer.join.merge_value != recipe.outer_merge_value
        || inner.join.merge_value == recipe.entry_value
    {
        return Err(NestedIfRecipeRejectReasonV1::JoinShapeMismatch);
    }
    if outer.else_assignments[0].binding != outer.join.binding
        || inner.then_assignments[0].binding != inner.join.binding
        || inner.else_assignments[0].binding != inner.join.binding
    {
        return Err(NestedIfRecipeRejectReasonV1::AssignmentShapeMismatch);
    }
    if recipe.continuation.binding != outer.join.binding {
        return Err(NestedIfRecipeRejectReasonV1::ContinuationMismatch);
    }
    let mut keys = BTreeSet::new();
    for expression in &recipe.expressions {
        if !keys.insert(expression.key) {
            return Err(NestedIfRecipeRejectReasonV1::DuplicateValueKey);
        }
        match &expression.kind {
            NestedIfExprKindV1::ReadBinding { binding } => {
                if *binding != recipe.bindings[0].key {
                    return Err(NestedIfRecipeRejectReasonV1::DanglingValue);
                }
            }
            NestedIfExprKindV1::Binary { left, right, .. } => {
                if !keys.contains(left) || !keys.contains(right) {
                    return Err(NestedIfRecipeRejectReasonV1::DanglingValue);
                }
            }
            NestedIfExprKindV1::ConstI64 { .. } | NestedIfExprKindV1::ConstBool { .. } => {}
        }
    }
    let all_values = keys.into_iter().chain([
        recipe.entry_value,
        recipe.outer_merge_value,
        inner.join.merge_value,
    ]);
    let values: BTreeSet<_> = all_values.collect();
    for value in [
        outer.condition,
        inner.condition,
        outer.join.then_value,
        outer.join.else_value,
        inner.join.then_value,
        inner.join.else_value,
    ] {
        if !values.contains(&value) {
            return Err(NestedIfRecipeRejectReasonV1::DanglingValue);
        }
    }
    Ok(())
}
