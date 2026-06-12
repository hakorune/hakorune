use std::collections::{BTreeMap, BTreeSet};

use crate::mir::contracts::fastmem_ops::is_fastmem_v0_memop_kind;
use crate::mir::function::{FastMemRegionMetadata, MirFunction};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, EffectMask, MirInstruction, ValueId};

use super::{push_region_error, MemOpSite};

pub(super) fn collect_regions<'a>(
    function: &'a MirFunction,
    errors: &mut Vec<VerificationError>,
) -> BTreeMap<u32, &'a FastMemRegionMetadata> {
    let mut regions = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for (index, region) in function.metadata.fastmem_regions.iter().enumerate() {
        if region.id == FastMemRegionId::INVALID {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "invalid-region-id",
                errors,
            );
        }
        if region.id.0 as usize != index {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "region-id-does-not-match-side-table-index",
                errors,
            );
        }
        if region.contract.trim().is_empty() {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "empty-contract-id",
                errors,
            );
        }
        if region.body_statement_count == 0 {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "empty-fastmem-region",
                errors,
            );
        }
        if !seen.insert(region.id.0) {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "duplicate-region-id",
                errors,
            );
        }
        regions.insert(region.id.0, region);
    }
    regions
}

pub(super) fn collect_memop_sites<'a>(
    function: &'a MirFunction,
    regions: &BTreeMap<u32, &'a FastMemRegionMetadata>,
    errors: &mut Vec<VerificationError>,
) -> Vec<MemOpSite> {
    let mut sites = Vec::new();
    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            let MirInstruction::MemOp {
                region,
                kind,
                dst,
                operands,
                access,
                effects,
            } = sp.inst
            else {
                continue;
            };
            let contract = regions
                .get(&region.0)
                .map(|metadata| metadata.contract.clone());
            if !regions.contains_key(&region.0) {
                push_region_error(
                    function,
                    Some(block_id),
                    Some(instruction_index),
                    Some(region.0),
                    None,
                    "unknown-region",
                    errors,
                );
            }
            check_memop_shape(
                function,
                block_id,
                instruction_index,
                region,
                kind,
                *dst,
                operands,
                access.as_ref(),
                *effects,
                contract.clone(),
                errors,
            );
            sites.push(MemOpSite {
                region: *region,
                kind: *kind,
                dst: *dst,
            });
        }
    }
    sites
}

#[allow(clippy::too_many_arguments)]
fn check_memop_shape(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    region: &FastMemRegionId,
    kind: &MemOpKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    effects: EffectMask,
    contract: Option<String>,
    errors: &mut Vec<VerificationError>,
) {
    if !is_fastmem_v0_memop_kind(*kind) {
        push_region_error(
            function,
            Some(block),
            Some(instruction_index),
            Some(region.0),
            contract.clone(),
            "unsupported-memop-kind",
            errors,
        );
    }

    if dst.is_some() != kind.has_destination() {
        push_region_error(
            function,
            Some(block),
            Some(instruction_index),
            Some(region.0),
            contract.clone(),
            "dst-shape-mismatch",
            errors,
        );
    }
    if operands.len() != kind.operand_arity() {
        push_region_error(
            function,
            Some(block),
            Some(instruction_index),
            Some(region.0),
            contract.clone(),
            "operand-arity-mismatch",
            errors,
        );
    }
    if effects != kind.effect_mask() {
        push_region_error(
            function,
            Some(block),
            Some(instruction_index),
            Some(region.0),
            contract.clone(),
            "effect-mask-mismatch",
            errors,
        );
    }
    check_memop_access_shape(
        function,
        block,
        instruction_index,
        region,
        kind,
        access,
        contract,
        errors,
    );
}

#[allow(clippy::too_many_arguments)]
fn check_memop_access_shape(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    region: &FastMemRegionId,
    kind: &MemOpKind,
    access: Option<&MemOpAccess>,
    contract: Option<String>,
    errors: &mut Vec<VerificationError>,
) {
    let empty = |value: &Option<String>| value.as_deref().is_some_and(|s| s.trim().is_empty());
    if let Some(access) = access {
        if empty(&access.layout_id) || empty(&access.field_id) || empty(&access.table_id) {
            push_region_error(
                function,
                Some(block),
                Some(instruction_index),
                Some(region.0),
                contract.clone(),
                "empty-access-id",
                errors,
            );
        }
    }

    match kind {
        MemOpKind::TableIndex => {
            if access.and_then(|a| a.table_id.as_deref()).is_none() {
                push_region_error(
                    function,
                    Some(block),
                    Some(instruction_index),
                    Some(region.0),
                    contract,
                    "table-index-missing-table-id",
                    errors,
                );
            }
        }
        MemOpKind::FieldLoad | MemOpKind::FieldStore => {
            if access.and_then(|a| a.field_id.as_deref()).is_none() {
                push_region_error(
                    function,
                    Some(block),
                    Some(instruction_index),
                    Some(region.0),
                    contract,
                    "field-access-missing-field-id",
                    errors,
                );
            }
        }
        _ => {
            if access.is_some() {
                push_region_error(
                    function,
                    Some(block),
                    Some(instruction_index),
                    Some(region.0),
                    contract,
                    "unexpected-access-metadata",
                    errors,
                );
            }
        }
    }
}

pub(super) fn check_region_counts(
    function: &MirFunction,
    regions: &BTreeMap<u32, &FastMemRegionMetadata>,
    sites: &[MemOpSite],
    errors: &mut Vec<VerificationError>,
) {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for site in sites {
        *counts.entry(site.region.0).or_insert(0) += 1;
    }
    for region in regions.values() {
        let actual = counts.get(&region.id.0).copied().unwrap_or(0);
        if region.emitted_memop_count != actual {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "emitted-memop-count-mismatch",
                errors,
            );
        }
        if actual == 0 {
            push_region_error(
                function,
                None,
                None,
                Some(region.id.0),
                Some(region.contract.clone()),
                "region-has-no-memops",
                errors,
            );
        }
    }
}
