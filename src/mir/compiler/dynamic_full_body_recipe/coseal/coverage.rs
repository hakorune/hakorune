//! Complete source-role and binding-role coverage verification.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::loop_recipe_contract::{
    LoopExitKindV2, LoopRecipeItemV2, LoopValueClassV2, VerifiedLoopRecipeV2,
};

use super::super::super::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
};
use super::super::claims::{
    DynamicFullLoopBindingClaimV2, DynamicFullLoopClaimTargetV2, DynamicFullLoopRecipeClaimsV2,
    DynamicFullLoopSourceClaimV2,
};
use super::super::DynamicFullLoopRetainedSourceV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicFullLoopCoverageRejectV2 {
    BindingCardinality,
    SourceCardinality,
    MissingOrDuplicateBindingRole,
    MissingOrDuplicateSourceRole,
    ClaimMappingMismatch,
    SourceSiteKindMismatch,
    UnknownClaimTarget,
    RecipeItemCoverageMismatch,
    CompletionSiteMismatch,
    IterationLocalBecameCarrier,
    OuterReturnBecameRecipeExit,
}

#[derive(Debug)]
pub(super) struct VerifiedDynamicFullLoopClaimCoverageV2 {
    bindings: Box<[DynamicFullLoopBindingClaimV2]>,
    sources: Box<[DynamicFullLoopSourceClaimV2]>,
    recipe_source_count: usize,
    deferred_local_count: usize,
    deferred_tail_count: usize,
}

impl VerifiedDynamicFullLoopClaimCoverageV2 {
    #[cfg(test)]
    pub(super) const fn counts(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.bindings.len(),
            self.sources.len(),
            self.recipe_source_count,
            self.deferred_local_count,
            self.deferred_tail_count,
        )
    }

    pub(super) fn binding_target(
        &self,
        role: DynamicFullBodyBindingRoleV1,
    ) -> Option<DynamicFullLoopClaimTargetV2> {
        self.bindings
            .iter()
            .find(|row| row.role == role)
            .map(|row| row.target)
    }

    pub(super) fn source_target(
        &self,
        role: DynamicFullBodySourceRoleV1,
    ) -> Option<DynamicFullLoopClaimTargetV2> {
        self.sources
            .iter()
            .find(|row| row.role == role)
            .map(|row| row.target)
    }
}

pub(super) fn verify_complete_claim_coverage_v2(
    source: &DynamicFullLoopRetainedSourceV1,
    recipe: &VerifiedLoopRecipeV2,
    claims: DynamicFullLoopRecipeClaimsV2,
) -> Result<VerifiedDynamicFullLoopClaimCoverageV2, DynamicFullLoopCoverageRejectV2> {
    let (bindings, sources) = claims.into_parts();
    verify_binding_roles(source, &bindings)?;
    verify_source_roles(source, &sources)?;
    verify_expected_claim_mapping(&bindings, &sources)?;
    verify_targets(recipe, &bindings, &sources)?;
    verify_item_coverage(recipe, &sources)?;
    verify_completion_partition(source, recipe)?;

    Ok(VerifiedDynamicFullLoopClaimCoverageV2 {
        bindings,
        sources,
        recipe_source_count: 25,
        deferred_local_count: 1,
        deferred_tail_count: 2,
    })
}

fn verify_binding_roles(
    source: &DynamicFullLoopRetainedSourceV1,
    claims: &[DynamicFullLoopBindingClaimV2],
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    if source.bindings.len() != 6 || claims.len() != 6 {
        return Err(DynamicFullLoopCoverageRejectV2::BindingCardinality);
    }
    let source_roles = source
        .bindings
        .iter()
        .map(|row| row.role())
        .collect::<BTreeSet<_>>();
    let claim_roles = claims.iter().map(|row| row.role).collect::<BTreeSet<_>>();
    if source_roles.len() != 6 || claim_roles.len() != 6 || source_roles != claim_roles {
        return Err(DynamicFullLoopCoverageRejectV2::MissingOrDuplicateBindingRole);
    }
    Ok(())
}

fn verify_source_roles(
    source: &DynamicFullLoopRetainedSourceV1,
    claims: &[DynamicFullLoopSourceClaimV2],
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    if source.rows.len() != 28 || claims.len() != 28 {
        return Err(DynamicFullLoopCoverageRejectV2::SourceCardinality);
    }
    let mut source_roles = BTreeSet::new();
    for row in source.rows.iter() {
        if !source_roles.insert(row.role()) {
            return Err(DynamicFullLoopCoverageRejectV2::MissingOrDuplicateSourceRole);
        }
        if is_statement_role(row.role()) != is_statement(row.site()) {
            return Err(DynamicFullLoopCoverageRejectV2::SourceSiteKindMismatch);
        }
    }
    let claim_roles = claims.iter().map(|row| row.role).collect::<BTreeSet<_>>();
    if source_roles.len() != 28 || claim_roles.len() != 28 || source_roles != claim_roles {
        return Err(DynamicFullLoopCoverageRejectV2::MissingOrDuplicateSourceRole);
    }
    Ok(())
}

fn verify_expected_claim_mapping(
    bindings: &[DynamicFullLoopBindingClaimV2],
    sources: &[DynamicFullLoopSourceClaimV2],
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    let (expected_bindings, expected_sources) = DynamicFullLoopRecipeClaimsV2::exact().into_parts();
    let binding_map = bindings
        .iter()
        .map(|row| (row.role, row.target))
        .collect::<BTreeMap<_, _>>();
    let source_map = sources
        .iter()
        .map(|row| (row.role, row.target))
        .collect::<BTreeMap<_, _>>();
    let expected_binding_map = expected_bindings
        .iter()
        .map(|row| (row.role, row.target))
        .collect::<BTreeMap<_, _>>();
    let expected_source_map = expected_sources
        .iter()
        .map(|row| (row.role, row.target))
        .collect::<BTreeMap<_, _>>();
    if binding_map != expected_binding_map || source_map != expected_source_map {
        return Err(DynamicFullLoopCoverageRejectV2::ClaimMappingMismatch);
    }
    Ok(())
}

fn verify_targets(
    recipe: &VerifiedLoopRecipeV2,
    bindings: &[DynamicFullLoopBindingClaimV2],
    sources: &[DynamicFullLoopSourceClaimV2],
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    let recipe = recipe.as_recipe();
    let value_class = |key| {
        recipe
            .values
            .iter()
            .find(|row| row.key == key)
            .map(|row| row.class)
    };
    let target_exists = |target: DynamicFullLoopClaimTargetV2| match target {
        DynamicFullLoopClaimTargetV2::Loop(key) => recipe.loops.iter().any(|row| row.key == key),
        DynamicFullLoopClaimTargetV2::Binding(key) => recipe
            .bindings
            .iter()
            .any(|row| row.key == key && row.class == LoopValueClassV2::Dynamic),
        DynamicFullLoopClaimTargetV2::Value(key) => {
            value_class(key) == Some(LoopValueClassV2::Dynamic)
        }
        DynamicFullLoopClaimTargetV2::Item(key) => recipe.items.iter().any(|row| row.key == key),
        DynamicFullLoopClaimTargetV2::Exit(key) => recipe.exits.iter().any(|row| {
            row.key == key && matches!(row.kind, LoopExitKindV2::Return { value: Some(_) })
        }),
        DynamicFullLoopClaimTargetV2::PreludeInduction {
            binding,
            carrier,
            entry,
        } => {
            recipe
                .bindings
                .iter()
                .any(|row| row.key == binding && row.class == LoopValueClassV2::Dynamic)
                && recipe.carriers.iter().any(|row| {
                    row.key == carrier
                        && row.binding == binding
                        && row.entry_value == entry
                        && row.class == LoopValueClassV2::Dynamic
                })
        }
        DynamicFullLoopClaimTargetV2::IterationLocal { value } => {
            value_class(value) == Some(LoopValueClassV2::Dynamic)
        }
        DynamicFullLoopClaimTargetV2::CallableTail { binding } => recipe
            .bindings
            .iter()
            .any(|row| row.key == binding && row.class == LoopValueClassV2::Dynamic),
    };
    if bindings.iter().any(|row| !target_exists(row.target))
        || sources.iter().any(|row| !target_exists(row.target))
    {
        return Err(DynamicFullLoopCoverageRejectV2::UnknownClaimTarget);
    }
    if recipe.bindings.len() != 1 || recipe.carriers.len() != 1 {
        return Err(DynamicFullLoopCoverageRejectV2::IterationLocalBecameCarrier);
    }
    Ok(())
}

fn verify_item_coverage(
    recipe: &VerifiedLoopRecipeV2,
    sources: &[DynamicFullLoopSourceClaimV2],
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    let recipe = recipe.as_recipe();
    let mut covered = BTreeSet::new();
    for row in sources {
        match row.target {
            DynamicFullLoopClaimTargetV2::Item(item) => {
                covered.insert(item);
            }
            DynamicFullLoopClaimTargetV2::Exit(exit) => {
                let Some(item) = recipe.items.iter().find_map(|row| {
                    matches!(row.item, LoopRecipeItemV2::Exit { exit: found } if found == exit)
                        .then_some(row.key)
                }) else {
                    return Err(DynamicFullLoopCoverageRejectV2::RecipeItemCoverageMismatch);
                };
                covered.insert(item);
            }
            _ => {}
        }
    }
    let expected = recipe
        .items
        .iter()
        .map(|row| row.key)
        .collect::<BTreeSet<_>>();
    if covered != expected {
        return Err(DynamicFullLoopCoverageRejectV2::RecipeItemCoverageMismatch);
    }
    Ok(())
}

fn verify_completion_partition(
    source: &DynamicFullLoopRetainedSourceV1,
    recipe: &VerifiedLoopRecipeV2,
) -> Result<(), DynamicFullLoopCoverageRejectV2> {
    use DynamicFullBodySourceRoleV1::{InnerReturn, OuterReturn};
    let statement = |role| {
        source.rows.iter().find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Statement(site) => Some(site),
                DynamicFullBodySourceSiteV1::Expression(_) => None,
            })?
        })
    };
    let (Some(inner), Some(outer)) = (statement(InnerReturn), statement(OuterReturn)) else {
        return Err(DynamicFullLoopCoverageRejectV2::CompletionSiteMismatch);
    };
    let completion = source
        .completion
        .explicit_sites()
        .iter()
        .collect::<BTreeSet<_>>();
    if completion != BTreeSet::from([inner, outer]) {
        return Err(DynamicFullLoopCoverageRejectV2::CompletionSiteMismatch);
    }
    if recipe.as_recipe().exits.len() != 1
        || !matches!(
            recipe.as_recipe().exits[0].kind,
            LoopExitKindV2::Return { value: Some(_) }
        )
    {
        return Err(DynamicFullLoopCoverageRejectV2::OuterReturnBecameRecipeExit);
    }
    Ok(())
}

const fn is_statement(site: &DynamicFullBodySourceSiteV1) -> bool {
    matches!(site, DynamicFullBodySourceSiteV1::Statement(_))
}

const fn is_statement_role(role: DynamicFullBodySourceRoleV1) -> bool {
    matches!(
        role,
        DynamicFullBodySourceRoleV1::PreludeLocalI
            | DynamicFullBodySourceRoleV1::Loop
            | DynamicFullBodySourceRoleV1::ChLocal
            | DynamicFullBodySourceRoleV1::InnerIf
            | DynamicFullBodySourceRoleV1::InnerReturn
            | DynamicFullBodySourceRoleV1::StepAssignment
            | DynamicFullBodySourceRoleV1::OuterReturn
    )
}
