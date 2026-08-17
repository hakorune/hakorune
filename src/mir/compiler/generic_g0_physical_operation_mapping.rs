//! Generic G0 source-operation to physical-operation mapping.
//!
//! The Generic source parent remains the sole operation/evidence authority.
//! This module only lends a complete, private mechanical view for the five
//! MIR-pure operation variants.  It does not create Builder state, ValueIds,
//! CFG/SSA state, or S6C provenance rows.

use std::collections::BTreeMap;

use crate::mir::loop_recipe_contract::{LoopOperationV1, LoopRecipeItemV1};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::generic_g0_source_parent::GenericG0SourceParentRefV1;
use crate::mir::loop_recipe_contract::{
    LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, VerifiedLoopOperationSourceEvidenceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0PhysicalOperationMappingRejectV1 {
    OwnerMismatch,
    DuplicateOperationItem,
    MissingEvidence { item: LoopItemKeyV1 },
    ForeignEvidence { item: LoopItemKeyV1 },
    PlacementMismatch { item: LoopItemKeyV1 },
    SourceOwnerMismatch { item: LoopItemKeyV1 },
    UnsupportedOperation { item: LoopItemKeyV1 },
    CoverageMismatch { expected: usize, found: usize },
}

#[derive(Debug)]
pub(crate) struct GenericG0PhysicalOperationRowV1<'loan> {
    item: LoopItemKeyV1,
    block: LoopBlockKeyV1,
    owner_loop: LoopNodeKeyV1,
    operation: LoopOperationV1,
    evidence: &'loan VerifiedLoopOperationSourceEvidenceV1,
}

impl GenericG0PhysicalOperationRowV1<'_> {
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.owner_loop
    }

    pub(crate) const fn operation(&self) -> LoopOperationV1 {
        self.operation
    }

    pub(crate) fn evidence(&self) -> &VerifiedLoopOperationSourceEvidenceV1 {
        self.evidence
    }
}

/// One complete Generic operation mapping.  The wrapper is intentionally
/// non-Clone so a physical consumer cannot silently split the mapping into
/// independently re-paired operation/evidence products.
#[derive(Debug)]
pub(crate) struct GenericG0PhysicalOperationMappingV1<'loan> {
    owner: FunctionOwnerIdV1,
    rows: Box<[GenericG0PhysicalOperationRowV1<'loan>]>,
}

impl GenericG0PhysicalOperationMappingV1<'_> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn rows(&self) -> &[GenericG0PhysicalOperationRowV1<'_>] {
        &self.rows
    }

    pub(crate) fn operation_count(&self) -> usize {
        self.rows.len()
    }
}

/// Borrow the same-parent Generic operation/evidence product and create one
/// private mechanical mapping.  Every failure is before any physical effect.
pub(crate) fn issue_generic_g0_physical_operation_mapping_v1<'view, 'loan, 'source>(
    parent: &'view GenericG0SourceParentRefV1<'loan, 'source>,
) -> Result<GenericG0PhysicalOperationMappingV1<'view>, GenericG0PhysicalOperationMappingRejectV1>
{
    let effect = parent.product().operation_effect();
    if effect.core().owner() != parent.owner() {
        return Err(GenericG0PhysicalOperationMappingRejectV1::OwnerMismatch);
    }

    let recipe = effect.core().recipe().as_recipe();
    let mut operation_rows = BTreeMap::new();
    for block in &recipe.blocks {
        for item in &block.items {
            let Some(row) = recipe.items.iter().find(|row| row.key == *item) else {
                return Err(GenericG0PhysicalOperationMappingRejectV1::ForeignEvidence {
                    item: *item,
                });
            };
            let LoopRecipeItemV1::Operation { operation } = &row.item else {
                continue;
            };
            if operation_rows
                .insert(*item, (block.key, block.owner_loop, *operation))
                .is_some()
            {
                return Err(GenericG0PhysicalOperationMappingRejectV1::DuplicateOperationItem);
            }
        }
    }

    let mut rows = Vec::with_capacity(effect.evidence().len());
    for evidence in effect.evidence() {
        let item = evidence.item();
        let Some((block, owner_loop, operation)) = operation_rows.get(&item).copied() else {
            return Err(GenericG0PhysicalOperationMappingRejectV1::MissingEvidence { item });
        };
        if evidence.block() != block || evidence.owner_loop() != owner_loop {
            return Err(GenericG0PhysicalOperationMappingRejectV1::PlacementMismatch { item });
        }
        if evidence.anchor().owner() != parent.owner()
            || evidence
                .source_binding()
                .is_some_and(|binding| binding.owner() != parent.owner())
        {
            return Err(GenericG0PhysicalOperationMappingRejectV1::SourceOwnerMismatch { item });
        }
        if !is_admitted_operation(operation) {
            return Err(GenericG0PhysicalOperationMappingRejectV1::UnsupportedOperation { item });
        }
        rows.push(GenericG0PhysicalOperationRowV1 {
            item,
            block,
            owner_loop,
            operation,
            evidence,
        });
    }

    if rows.len() != operation_rows.len() || rows.len() != effect.evidence().len() {
        return Err(GenericG0PhysicalOperationMappingRejectV1::CoverageMismatch {
            expected: operation_rows.len(),
            found: rows.len(),
        });
    }
    Ok(GenericG0PhysicalOperationMappingV1 {
        owner: parent.owner(),
        rows: rows.into_boxed_slice(),
    })
}

fn is_admitted_operation(operation: LoopOperationV1) -> bool {
    matches!(
        operation,
        LoopOperationV1::ReadBinding { .. }
            | LoopOperationV1::ConstI64 { .. }
            | LoopOperationV1::BinaryI64 { .. }
            | LoopOperationV1::CompareI64 { .. }
            | LoopOperationV1::WriteBinding { .. }
    )
}

#[cfg(test)]
mod tests {
    use super::issue_generic_g0_physical_operation_mapping_v1;
    use crate::mir::compiler::generic_g0_source_parent::with_generic_g0_source_parent_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    #[test]
    fn maps_complete_generic_operation_evidence_without_s6c_rows() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let result = with_generic_g0_source_parent_v1(input, selection, |parent| {
            let mapping = issue_generic_g0_physical_operation_mapping_v1(&parent)
                .expect("complete Generic operation mapping");
            assert_eq!(mapping.operation_count(), 15);
            assert_eq!(mapping.owner(), parent.owner());
            assert!(mapping.rows().iter().all(|row| row.evidence().item() == row.item()));
            assert!(mapping.rows().iter().all(|row| row.block() == row.evidence().block()));
            assert!(mapping
                .rows()
                .iter()
                .all(|row| row.owner_loop() == row.evidence().owner_loop()));
            // Item 4 is the nested-loop/carrier boundary, not a Generic
            // physical operation row.  Keeping it out is part of the
            // mapping's negative coverage contract.
            assert!(mapping.rows().iter().all(|row| row.item().raw() != 4));
            Ok::<_, String>(())
        });
        result.expect("Generic source parent").expect("mapping test");
    }
}
