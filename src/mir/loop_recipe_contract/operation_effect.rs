//! Passive item-keyed source/effect product for Loop operation physicalization.
//!
//! This module owns no AST, Builder, MIR, or route selection. It joins one
//! already sealed Core with one profile-issued source evidence ledger before
//! the topology-only physical boundary can discard source evidence.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::{BindingRefV1, SourcePathSegmentV1, SourceStmtSiteV1};

use super::ids::{LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::schema::{LoopOperationV1, LoopRecipeItemV1, LoopValueClassV1};
use super::source_bound_core::{
    LoopBindingEffectAnchorV1, LoopBindingEffectRoleV1, VerifiedLoopCoreProductV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopOperationSourceEvidenceV1 {
    item: LoopItemKeyV1,
    anchor: LoopBindingEffectAnchorV1,
    source_loop: SourceStmtSiteV1,
    owner_loop: LoopNodeKeyV1,
    block: LoopBlockKeyV1,
    source_binding: Option<BindingRefV1>,
}

impl LoopOperationSourceEvidenceV1 {
    pub(crate) fn new(
        item: LoopItemKeyV1,
        anchor: LoopBindingEffectAnchorV1,
        source_loop: SourceStmtSiteV1,
        owner_loop: LoopNodeKeyV1,
        block: LoopBlockKeyV1,
        source_binding: Option<BindingRefV1>,
    ) -> Self {
        Self {
            item,
            anchor,
            source_loop,
            owner_loop,
            block,
            source_binding,
        }
    }

    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) fn anchor(&self) -> &LoopBindingEffectAnchorV1 {
        &self.anchor
    }

    pub(crate) fn source_loop(&self) -> &SourceStmtSiteV1 {
        &self.source_loop
    }

    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.owner_loop
    }

    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn source_binding(&self) -> Option<BindingRefV1> {
        self.source_binding
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopOperationSourceEvidenceV1(LoopOperationSourceEvidenceV1);

impl VerifiedLoopOperationSourceEvidenceV1 {
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.0.item
    }

    pub(crate) fn anchor(&self) -> &LoopBindingEffectAnchorV1 {
        &self.0.anchor
    }

    pub(crate) fn source_loop(&self) -> &SourceStmtSiteV1 {
        &self.0.source_loop
    }

    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.0.owner_loop
    }

    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.0.block
    }

    pub(crate) const fn source_binding(&self) -> Option<BindingRefV1> {
        self.0.source_binding
    }

    #[cfg(test)]
    pub(crate) fn into_unverified_for_test(self) -> LoopOperationSourceEvidenceV1 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopOperationEffectRejectV1 {
    DuplicateEvidence { item: LoopItemKeyV1 },
    MissingEvidence { item: LoopItemKeyV1 },
    ForeignItem { item: LoopItemKeyV1 },
    NonOperationItem { item: LoopItemKeyV1 },
    OwnerMismatch { item: LoopItemKeyV1 },
    ForeignSourceBinding { item: LoopItemKeyV1 },
    InvalidSourceLoop { item: LoopItemKeyV1 },
    PlacementMismatch { item: LoopItemKeyV1 },
    MissingBindingEvidence { item: LoopItemKeyV1 },
    UnexpectedBindingEvidence { item: LoopItemKeyV1 },
    MissingCoreEffect { item: LoopItemKeyV1 },
    CoreEffectRoleMismatch { item: LoopItemKeyV1 },
    OperandClassMismatch { item: LoopItemKeyV1 },
}

/// The sole passive owner of operation source evidence after D0.
///
/// The Core is moved into this product. Recipe operations, operands, binding
/// relations, and effect relations remain views into that Core; they are not
/// copied into a second table.
#[derive(Debug)]
pub(crate) struct VerifiedLoopOperationEffectProductV1 {
    core: VerifiedLoopCoreProductV1,
    evidence: Box<[VerifiedLoopOperationSourceEvidenceV1]>,
}

impl VerifiedLoopOperationEffectProductV1 {
    pub(crate) fn issue(
        core: VerifiedLoopCoreProductV1,
        evidence: Vec<LoopOperationSourceEvidenceV1>,
    ) -> Result<Self, LoopOperationEffectRejectV1> {
        let operations = operation_membership(core.recipe().as_recipe());
        let mut seen = BTreeSet::new();
        let mut verified = Vec::with_capacity(evidence.len());
        for row in evidence {
            let item = row.item;
            if !seen.insert(item) {
                return Err(LoopOperationEffectRejectV1::DuplicateEvidence { item });
            }
            let Some((block, owner_loop, operation)) = operations.get(&item).copied() else {
                return if core
                    .recipe()
                    .as_recipe()
                    .items
                    .iter()
                    .any(|candidate| candidate.key == item)
                {
                    Err(LoopOperationEffectRejectV1::NonOperationItem { item })
                } else {
                    Err(LoopOperationEffectRejectV1::ForeignItem { item })
                };
            };
            if row.owner_loop != owner_loop || row.block != block {
                return Err(LoopOperationEffectRejectV1::PlacementMismatch { item });
            }
            if row.anchor.owner() != core.owner() {
                return Err(LoopOperationEffectRejectV1::OwnerMismatch { item });
            }
            if row
                .source_binding
                .is_some_and(|binding| binding.owner() != core.owner())
            {
                return Err(LoopOperationEffectRejectV1::ForeignSourceBinding { item });
            }
            if !is_loop_source_site(&row.source_loop) {
                return Err(LoopOperationEffectRejectV1::InvalidSourceLoop { item });
            }
            verify_operation(&core, item, operation, &row)?;
            verified.push(VerifiedLoopOperationSourceEvidenceV1(row));
        }
        if let Some(item) = operations.keys().find(|item| !seen.contains(item)) {
            return Err(LoopOperationEffectRejectV1::MissingEvidence { item: *item });
        }
        verified.sort_by_key(VerifiedLoopOperationSourceEvidenceV1::item);
        Ok(Self {
            core,
            evidence: verified.into_boxed_slice(),
        })
    }

    pub(crate) fn core(&self) -> &VerifiedLoopCoreProductV1 {
        &self.core
    }

    pub(crate) fn evidence(&self) -> &[VerifiedLoopOperationSourceEvidenceV1] {
        &self.evidence
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopCoreProductV1,
        Box<[VerifiedLoopOperationSourceEvidenceV1]>,
    ) {
        (self.core, self.evidence)
    }
}

fn operation_membership(
    recipe: &super::schema::LoopRecipeV1,
) -> BTreeMap<LoopItemKeyV1, (LoopBlockKeyV1, LoopNodeKeyV1, LoopOperationV1)> {
    let rows = recipe
        .items
        .iter()
        .map(|row| (row.key, &row.item))
        .collect::<BTreeMap<_, _>>();
    recipe
        .blocks
        .iter()
        .flat_map(|block| {
            block.items.iter().filter_map(|item| {
                let Some(LoopRecipeItemV1::Operation { operation }) = rows.get(item).copied()
                else {
                    return None;
                };
                Some((*item, (block.key, block.owner_loop, *operation)))
            })
        })
        .collect()
}

fn verify_operation(
    core: &VerifiedLoopCoreProductV1,
    item: LoopItemKeyV1,
    operation: LoopOperationV1,
    evidence: &LoopOperationSourceEvidenceV1,
) -> Result<(), LoopOperationEffectRejectV1> {
    let recipe = core.recipe().as_recipe();
    match operation {
        LoopOperationV1::ReadBinding { binding, result } => {
            require_binding(core, item, binding, evidence, true)?;
            let class = binding_class(recipe, binding);
            require_class(recipe, class, result, class, item)
        }
        LoopOperationV1::WriteBinding { binding, value } => {
            require_binding(core, item, binding, evidence, false)?;
            let expected = binding_class(recipe, binding);
            require_value(recipe, value, expected, item)
        }
        LoopOperationV1::ConstI64 { result, .. } => {
            reject_pure_binding(item, evidence)?;
            require_value(recipe, result, LoopValueClassV1::I64, item)
        }
        LoopOperationV1::BinaryI64 {
            left,
            right,
            result,
            ..
        } => {
            reject_pure_binding(item, evidence)?;
            require_value(recipe, left, LoopValueClassV1::I64, item)?;
            require_value(recipe, right, LoopValueClassV1::I64, item)?;
            require_value(recipe, result, LoopValueClassV1::I64, item)
        }
        LoopOperationV1::CompareI64 {
            left,
            right,
            result,
            ..
        } => {
            reject_pure_binding(item, evidence)?;
            require_value(recipe, left, LoopValueClassV1::I64, item)?;
            require_value(recipe, right, LoopValueClassV1::I64, item)?;
            require_value(recipe, result, LoopValueClassV1::Bool, item)
        }
    }
}

fn require_binding(
    core: &VerifiedLoopCoreProductV1,
    item: LoopItemKeyV1,
    binding: LoopBindingKeyV1,
    evidence: &LoopOperationSourceEvidenceV1,
    expected_read: bool,
) -> Result<(), LoopOperationEffectRejectV1> {
    let Some(source_binding) = evidence.source_binding else {
        return Err(LoopOperationEffectRejectV1::MissingBindingEvidence { item });
    };
    let found = core.effect_relations().iter().find(|row| {
        row.recipe_binding() == binding
            && row.source_binding() == source_binding
            && row.anchor() == &evidence.anchor
    });
    let Some(row) = found else {
        return Err(LoopOperationEffectRejectV1::MissingCoreEffect { item });
    };
    let expected_class = binding_class(core.recipe().as_recipe(), binding);
    if row.class() != expected_class {
        return Err(LoopOperationEffectRejectV1::OperandClassMismatch { item });
    }
    let role_matches = if expected_read {
        match evidence.anchor() {
            LoopBindingEffectAnchorV1::DerivedCarrierEntry { .. } => {
                matches!(row.role(), LoopBindingEffectRoleV1::DerivedCarrierEntry)
            }
            LoopBindingEffectAnchorV1::Expr(_) => {
                matches!(row.role(), LoopBindingEffectRoleV1::SourceRead { .. })
            }
        }
    } else {
        matches!(row.role(), LoopBindingEffectRoleV1::SourceWrite { .. })
    };
    if !role_matches {
        return Err(LoopOperationEffectRejectV1::CoreEffectRoleMismatch { item });
    }
    Ok(())
}

fn reject_pure_binding(
    item: LoopItemKeyV1,
    evidence: &LoopOperationSourceEvidenceV1,
) -> Result<(), LoopOperationEffectRejectV1> {
    if evidence.source_binding.is_some() {
        return Err(LoopOperationEffectRejectV1::UnexpectedBindingEvidence { item });
    }
    Ok(())
}

fn binding_class(recipe: &super::schema::LoopRecipeV1, key: LoopBindingKeyV1) -> LoopValueClassV1 {
    recipe
        .bindings
        .iter()
        .find(|row| row.key == key)
        .map(|row| row.class)
        .unwrap_or(LoopValueClassV1::Unit)
}

fn require_class(
    recipe: &super::schema::LoopRecipeV1,
    binding: LoopValueClassV1,
    value: LoopValueKeyV1,
    expected: LoopValueClassV1,
    item: LoopItemKeyV1,
) -> Result<(), LoopOperationEffectRejectV1> {
    if binding != expected || value_class(recipe, value) != Some(expected) {
        return Err(LoopOperationEffectRejectV1::OperandClassMismatch { item });
    }
    Ok(())
}

fn require_value(
    recipe: &super::schema::LoopRecipeV1,
    value: LoopValueKeyV1,
    expected: LoopValueClassV1,
    item: LoopItemKeyV1,
) -> Result<(), LoopOperationEffectRejectV1> {
    if value_class(recipe, value) != Some(expected) {
        return Err(LoopOperationEffectRejectV1::OperandClassMismatch { item });
    }
    Ok(())
}

fn value_class(
    recipe: &super::schema::LoopRecipeV1,
    key: LoopValueKeyV1,
) -> Option<LoopValueClassV1> {
    recipe
        .values
        .iter()
        .find(|row| row.key == key)
        .map(|row| row.class)
}

fn is_loop_source_site(site: &SourceStmtSiteV1) -> bool {
    site.node().segments().iter().any(|segment| {
        matches!(
            segment,
            SourcePathSegmentV1::LoopCondition
                | SourcePathSegmentV1::LoopBodyRoot
                | SourcePathSegmentV1::LoopBody(_)
                | SourcePathSegmentV1::Body(_)
        )
    })
}
