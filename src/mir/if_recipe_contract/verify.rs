//! Structural verifier for the fixed-shell portable If recipe.

use std::collections::{BTreeMap, BTreeSet};

use super::error::IfRecipeRejectReasonV1 as Reject;
use super::ids::{IfBindingKeyV1, IfBlockKeyV1, IfValueKeyV1};
use super::schema::{
    IfBlockRoleV1, IfContinuationV1, IfElseDispositionV1, IfOperationV1, IfRecipeArtifactV1,
    IfRecipeBlockV1, IfRecipeV1, IfValueClassV1, IF_RECIPE_SCHEMA_VERSION_V1,
};
use super::source_binding::{IfRecipeSourceClaimVerifierV1, VerifiedIfRecipeSourceClaimV1};

#[derive(Debug)]
pub(crate) struct VerifiedIfRecipeV1(IfRecipeV1);

impl VerifiedIfRecipeV1 {
    pub(crate) fn as_recipe(&self) -> &IfRecipeV1 {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedIfRecipeArtifactV1 {
    provenance: super::schema::IfRecipeProvenanceV1,
    source_binding: VerifiedIfRecipeSourceClaimV1,
    recipe: VerifiedIfRecipeV1,
}

impl VerifiedIfRecipeArtifactV1 {
    pub(crate) fn provenance(&self) -> &super::schema::IfRecipeProvenanceV1 {
        &self.provenance
    }

    pub(crate) fn source_binding(&self) -> &VerifiedIfRecipeSourceClaimV1 {
        &self.source_binding
    }

    pub(crate) fn recipe(&self) -> &VerifiedIfRecipeV1 {
        &self.recipe
    }
}

pub(crate) struct IfRecipeVerifierV1;

impl IfRecipeVerifierV1 {
    pub(crate) fn verify_artifact(
        artifact: IfRecipeArtifactV1,
    ) -> Result<VerifiedIfRecipeArtifactV1, Reject> {
        if artifact.schema_version != IF_RECIPE_SCHEMA_VERSION_V1 {
            return Err(Reject::UnsupportedVersion {
                found: artifact.schema_version,
            });
        }
        let disposition = artifact.recipe.else_disposition;
        let profile_matches = matches!(
            (artifact.provenance.profile, disposition),
            (
                super::schema::IfRecipeProfileV1::ResolvedTrivialExplicitElse,
                IfElseDispositionV1::Explicit
            ) | (
                super::schema::IfRecipeProfileV1::ResolvedTrivialImplicitElse,
                IfElseDispositionV1::ImplicitFallthrough
            )
        );
        if !profile_matches {
            return Err(Reject::ProfileDispositionMismatch);
        }
        let recipe = Self::verify(artifact.recipe)?;
        let source_binding = IfRecipeSourceClaimVerifierV1::verify(artifact.source_binding)?;
        let direct_ops = direct_static_call_count(recipe.as_recipe());
        let direct_claims = source_binding
            .as_source_binding()
            .claims
            .iter()
            .filter(|claim| claim.role == super::schema::IfSourceClaimRoleV1::DirectStaticCall)
            .count();
        if direct_ops != direct_claims {
            return Err(Reject::DirectStaticCallCountMismatch { found: direct_ops });
        }
        let (then_ops, else_ops) = direct_static_call_branch_counts(recipe.as_recipe());
        let (then_claims, else_claims) =
            direct_static_call_claim_branch_counts(source_binding.as_source_binding());
        if (then_ops, else_ops) != (then_claims, else_claims) {
            return Err(Reject::DirectStaticCallBranchMismatch {
                then_ops,
                else_ops,
                then_claims,
                else_claims,
            });
        }
        Ok(VerifiedIfRecipeArtifactV1 {
            provenance: artifact.provenance,
            source_binding,
            recipe,
        })
    }

    pub(crate) fn verify(recipe: IfRecipeV1) -> Result<VerifiedIfRecipeV1, Reject> {
        verify_blocks(&recipe)?;
        let bindings = verify_bindings(&recipe)?;
        let values = verify_values(&recipe)?;
        let merge_targets = recipe
            .bindings
            .iter()
            .filter(|binding| matches!(binding.role, super::schema::IfBindingRoleV1::MergeTarget))
            .count();
        if merge_targets != 1 {
            return Err(Reject::MergeTargetCountMismatch {
                found: merge_targets,
            });
        }
        let mut definitions = recipe.inputs.iter().copied().collect::<BTreeSet<_>>();
        for input in &recipe.inputs {
            if !values.contains_key(input) {
                return Err(Reject::DanglingValue { value: *input });
            }
        }
        let mut item_cursor = 0;
        verify_condition_block(
            &recipe,
            &bindings,
            &values,
            &mut definitions,
            &mut item_cursor,
        )?;
        if !definitions.contains(&recipe.condition) {
            return Err(Reject::ConditionNotBool {
                value: recipe.condition,
            });
        }
        if values.get(&recipe.condition) != Some(&IfValueClassV1::Bool) {
            return Err(Reject::ConditionNotBool {
                value: recipe.condition,
            });
        }

        let then_write = verify_branch_block(
            &recipe.then_block,
            &bindings,
            &values,
            &definitions,
            "then",
            &mut item_cursor,
        )?;
        let merge_binding = recipe
            .bindings
            .iter()
            .find(|binding| matches!(binding.role, super::schema::IfBindingRoleV1::MergeTarget))
            .map(|binding| binding.key)
            .ok_or(Reject::MergeTargetCountMismatch { found: 0 })?;
        if then_write.0 != merge_binding {
            return Err(Reject::BranchBindingMismatch);
        }
        let explicit_else_write = match recipe.else_disposition {
            IfElseDispositionV1::Explicit => {
                let Some(else_block) = recipe.else_block.as_ref() else {
                    return Err(Reject::ExplicitElseRequired);
                };
                let else_write = verify_branch_block(
                    else_block,
                    &bindings,
                    &values,
                    &definitions,
                    "else",
                    &mut item_cursor,
                )?;
                if then_write.0 != else_write.0 || then_write.1 != else_write.1 {
                    return Err(Reject::BranchBindingMismatch);
                }
                Some(else_write)
            }
            IfElseDispositionV1::ImplicitFallthrough => {
                if recipe.else_block.is_some() {
                    return Err(Reject::ImplicitElseBlockPresent);
                }
                None
            }
        };
        if recipe.joins.len() != 1 {
            return Err(Reject::JoinRowCountMismatch {
                found: recipe.joins.len(),
            });
        }
        let join = recipe.joins[0];
        if join.binding != then_write.0
            || join.class != then_write.1
            || !recipe.inputs.contains(&join.entry_value)
            || values.get(&join.entry_value) != Some(&join.class)
        {
            return Err(Reject::JoinBindingMismatch);
        }
        if join.then_value != then_write.2 {
            return Err(Reject::JoinValueMismatch);
        }
        if let Some(else_write) = explicit_else_write {
            if join.else_value != else_write.2 {
                return Err(Reject::JoinValueMismatch);
            }
        } else if join.else_value != join.entry_value {
            return Err(Reject::ImplicitBaselineMismatch);
        }

        verify_continuation_block(
            &recipe.continuation_block,
            &bindings,
            &values,
            &recipe.continuation,
            then_write.0,
            &mut item_cursor,
        )?;
        Ok(VerifiedIfRecipeV1(recipe))
    }
}

fn direct_static_call_count(recipe: &IfRecipeV1) -> usize {
    recipe
        .condition_block
        .items
        .iter()
        .chain(recipe.then_block.items.iter())
        .chain(
            recipe
                .else_block
                .iter()
                .flat_map(|block| block.items.iter()),
        )
        .chain(recipe.continuation_block.items.iter())
        .filter(|item| matches!(item.operation, IfOperationV1::DirectStaticCall { .. }))
        .count()
}

fn direct_static_call_branch_counts(recipe: &IfRecipeV1) -> (usize, usize) {
    let count = |block: &IfRecipeBlockV1| {
        block
            .items
            .iter()
            .filter(|item| matches!(item.operation, IfOperationV1::DirectStaticCall { .. }))
            .count()
    };
    (
        count(&recipe.then_block),
        recipe.else_block.as_ref().map(count).unwrap_or(0),
    )
}

fn direct_static_call_claim_branch_counts(
    binding: &super::schema::IfRecipeSourceBindingV1,
) -> (usize, usize) {
    binding
        .claims
        .iter()
        .filter(|claim| {
            claim.role == super::schema::IfSourceClaimRoleV1::DirectStaticCall
        })
        .fold((0, 0), |(then_count, else_count), claim| {
            match claim.path.steps.as_slice() {
                [
                    super::schema::IfSourcePathStepV1::BodyItem { .. },
                    super::schema::IfSourcePathStepV1::IfThenItem { .. },
                    super::schema::IfSourcePathStepV1::AssignmentValue,
                ] => (then_count + 1, else_count),
                [
                    super::schema::IfSourcePathStepV1::BodyItem { .. },
                    super::schema::IfSourcePathStepV1::IfElseItem { .. },
                    super::schema::IfSourcePathStepV1::AssignmentValue,
                ] => (then_count, else_count + 1),
                _ => (then_count, else_count),
            }
        })
}

fn verify_blocks(recipe: &IfRecipeV1) -> Result<(), Reject> {
    let blocks: [(Option<&IfRecipeBlockV1>, IfBlockRoleV1, &str); 4] = [
        (
            Some(&recipe.condition_block),
            IfBlockRoleV1::Condition,
            "condition",
        ),
        (Some(&recipe.then_block), IfBlockRoleV1::Then, "then"),
        (recipe.else_block.as_ref(), IfBlockRoleV1::Else, "else"),
        (
            Some(&recipe.continuation_block),
            IfBlockRoleV1::Continuation,
            "continuation",
        ),
    ];
    let mut seen = BTreeSet::new();
    for (block, role, name) in blocks {
        let Some(block) = block else {
            if name == "else" && recipe.else_disposition == IfElseDispositionV1::ImplicitFallthrough
            {
                continue;
            }
            return Err(Reject::MissingBlockRole { role: name });
        };
        if block.role != role || block.key.raw() != seen.len() as u32 {
            return Err(Reject::InvalidBlockRole { block: block.key });
        }
        if !seen.insert(block.key) {
            return Err(Reject::DuplicateBlock { block: block.key });
        }
    }
    if recipe.else_disposition == IfElseDispositionV1::ImplicitFallthrough
        && recipe.else_block.is_some()
    {
        return Err(Reject::ImplicitElseBlockPresent);
    }
    Ok(())
}

fn verify_bindings(
    recipe: &IfRecipeV1,
) -> Result<BTreeMap<IfBindingKeyV1, IfValueClassV1>, Reject> {
    let mut bindings = BTreeMap::new();
    for (index, binding) in recipe.bindings.iter().enumerate() {
        if binding.key.raw() != index as u32 {
            return Err(Reject::NonCanonicalKeyOrder { domain: "bindings" });
        }
        if !matches!(
            binding.role,
            super::schema::IfBindingRoleV1::Input | super::schema::IfBindingRoleV1::MergeTarget
        ) {
            return Err(Reject::InvalidBindingRole {
                binding: binding.key,
            });
        }
        if bindings.insert(binding.key, binding.class).is_some() {
            return Err(Reject::DanglingBinding {
                binding: binding.key,
            });
        }
    }
    Ok(bindings)
}

fn verify_values(recipe: &IfRecipeV1) -> Result<BTreeMap<IfValueKeyV1, IfValueClassV1>, Reject> {
    let mut values = BTreeMap::new();
    for (index, value) in recipe.values.iter().enumerate() {
        if value.key.raw() != index as u32 {
            return Err(Reject::NonCanonicalKeyOrder { domain: "values" });
        }
        if values.insert(value.key, value.class).is_some() {
            return Err(Reject::DuplicateValueDefinition { value: value.key });
        }
    }
    let mut previous: Option<IfValueKeyV1> = None;
    for input in &recipe.inputs {
        if previous.is_some_and(|old| old.raw() >= input.raw()) {
            return Err(Reject::NonCanonicalKeyOrder { domain: "inputs" });
        }
        previous = Some(*input);
    }
    Ok(values)
}

fn verify_condition_block(
    recipe: &IfRecipeV1,
    bindings: &BTreeMap<IfBindingKeyV1, IfValueClassV1>,
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    definitions: &mut BTreeSet<IfValueKeyV1>,
    item_cursor: &mut u32,
) -> Result<(), Reject> {
    if recipe.condition_block.items.iter().any(|item| {
        matches!(
            item.operation,
            IfOperationV1::WriteBinding { .. } | IfOperationV1::DirectStaticCall { .. }
        )
    }) {
        return Err(Reject::UnsupportedOperation);
    }
    verify_operations(
        &recipe.condition_block,
        bindings,
        values,
        definitions,
        false,
        item_cursor,
    )
}

fn verify_branch_block(
    block: &IfRecipeBlockV1,
    bindings: &BTreeMap<IfBindingKeyV1, IfValueClassV1>,
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    initial: &BTreeSet<IfValueKeyV1>,
    branch: &'static str,
    item_cursor: &mut u32,
) -> Result<(IfBindingKeyV1, IfValueClassV1, IfValueKeyV1), Reject> {
    let mut definitions = initial.clone();
    let mut writes = Vec::new();
    for item in &block.items {
        if let IfOperationV1::WriteBinding { binding, value } = item.operation {
            writes.push((binding, value));
        }
    }
    if writes.is_empty() {
        return Err(Reject::MissingBranchWrite { branch });
    }
    if writes.len() != 1 {
        return Err(Reject::BranchWriteCountMismatch {
            branch,
            found: writes.len(),
        });
    }
    verify_operations(block, bindings, values, &mut definitions, true, item_cursor)?;
    let (binding, value) = writes[0];
    let class = *bindings
        .get(&binding)
        .ok_or(Reject::DanglingBinding { binding })?;
    if values.get(&value) != Some(&class) {
        return Err(Reject::ValueClassMismatch { value });
    }
    Ok((binding, class, value))
}

fn verify_continuation_block(
    block: &IfRecipeBlockV1,
    bindings: &BTreeMap<IfBindingKeyV1, IfValueClassV1>,
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    initial: &IfContinuationV1,
    binding: IfBindingKeyV1,
    item_cursor: &mut u32,
) -> Result<(), Reject> {
    let reads = block
        .items
        .iter()
        .filter_map(|item| match item.operation {
            IfOperationV1::ReadBinding { binding, result } => Some((binding, result)),
            _ => None,
        })
        .collect::<Vec<_>>();
    if reads.is_empty() {
        return Err(Reject::MissingContinuationRead);
    }
    if reads.len() != 1 {
        return Err(Reject::BranchWriteCountMismatch {
            branch: "continuation-read",
            found: reads.len(),
        });
    }
    if block.items.len() != 1 {
        return Err(Reject::BranchWriteCountMismatch {
            branch: "continuation",
            found: block.items.len(),
        });
    }
    if initial.required_read != binding || reads[0].0 != binding {
        return Err(Reject::ContinuationBindingMismatch);
    }
    let mut definitions = BTreeSet::new();
    verify_operations(
        block,
        bindings,
        values,
        &mut definitions,
        false,
        item_cursor,
    )?;
    Ok(())
}

fn verify_operations(
    block: &IfRecipeBlockV1,
    bindings: &BTreeMap<IfBindingKeyV1, IfValueClassV1>,
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    definitions: &mut BTreeSet<IfValueKeyV1>,
    allow_write: bool,
    item_cursor: &mut u32,
) -> Result<(), Reject> {
    for item in &block.items {
        if item.key.raw() != *item_cursor {
            return Err(Reject::NonCanonicalKeyOrder { domain: "items" });
        }
        *item_cursor += 1;
        match item.operation {
            IfOperationV1::ReadBinding { binding, result } => {
                let class = *bindings
                    .get(&binding)
                    .ok_or(Reject::DanglingBinding { binding })?;
                define_value(values, definitions, result, class)?;
            }
            IfOperationV1::ConstI64 { result, .. } => {
                define_value(values, definitions, result, IfValueClassV1::I64)?;
            }
            IfOperationV1::ConstBool { result, .. } => {
                define_value(values, definitions, result, IfValueClassV1::Bool)?;
            }
            IfOperationV1::BinaryI64 {
                left,
                right,
                result,
                ..
            } => {
                use_value(values, definitions, left, IfValueClassV1::I64)?;
                use_value(values, definitions, right, IfValueClassV1::I64)?;
                define_value(values, definitions, result, IfValueClassV1::I64)?;
            }
            IfOperationV1::CompareI64 {
                left,
                right,
                result,
                ..
            } => {
                use_value(values, definitions, left, IfValueClassV1::I64)?;
                use_value(values, definitions, right, IfValueClassV1::I64)?;
                define_value(values, definitions, result, IfValueClassV1::Bool)?;
            }
            IfOperationV1::DirectStaticCall { result } => {
                define_value(values, definitions, result, IfValueClassV1::I64)?;
            }
            IfOperationV1::WriteBinding { binding, value } => {
                if !allow_write {
                    return Err(Reject::UnsupportedOperation);
                }
                let class = *bindings
                    .get(&binding)
                    .ok_or(Reject::DanglingBinding { binding })?;
                use_value(values, definitions, value, class)?;
            }
        }
    }
    Ok(())
}

fn define_value(
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    definitions: &mut BTreeSet<IfValueKeyV1>,
    value: IfValueKeyV1,
    class: IfValueClassV1,
) -> Result<(), Reject> {
    if values.get(&value) != Some(&class) {
        return Err(Reject::ValueClassMismatch { value });
    }
    if !definitions.insert(value) {
        return Err(Reject::DuplicateValueDefinition { value });
    }
    Ok(())
}

fn use_value(
    values: &BTreeMap<IfValueKeyV1, IfValueClassV1>,
    definitions: &BTreeSet<IfValueKeyV1>,
    value: IfValueKeyV1,
    class: IfValueClassV1,
) -> Result<(), Reject> {
    if values.get(&value) != Some(&class) {
        return Err(Reject::ValueClassMismatch { value });
    }
    if !definitions.contains(&value) {
        return Err(Reject::ValueUseBeforeDefinition { value });
    }
    Ok(())
}
