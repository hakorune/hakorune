use std::collections::{BTreeMap, BTreeSet};

use crate::mir::contracts::fastmem_ops::{is_fastmem_v0_memop_kind, memop_kind_name};
use crate::mir::function::MirFunction;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, EffectMask, MirInstruction, ValueId};

pub(super) fn check_fastmem_regions(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    let regions = collect_regions(function, &mut errors);
    let memop_sites = collect_memop_sites(function, &regions, &mut errors);

    check_region_counts(function, &regions, &memop_sites, &mut errors);
    check_memop_escape(function, &memop_sites, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Debug, Clone, Copy)]
struct MemOpSite {
    region: FastMemRegionId,
    kind: MemOpKind,
    dst: Option<ValueId>,
}

fn collect_regions<'a>(
    function: &'a MirFunction,
    errors: &mut Vec<VerificationError>,
) -> BTreeMap<u32, &'a crate::mir::function::FastMemRegionMetadata> {
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

fn collect_memop_sites<'a>(
    function: &'a MirFunction,
    regions: &BTreeMap<u32, &'a crate::mir::function::FastMemRegionMetadata>,
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

    let (expected_dst, expected_operands, expected_effects) = expected_shape(*kind);
    if dst.is_some() != expected_dst {
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
    if operands.len() != expected_operands {
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
    if effects != expected_effects {
        push_region_error(
            function,
            Some(block),
            Some(instruction_index),
            Some(region.0),
            contract,
            "effect-mask-mismatch",
            errors,
        );
    }
}

fn expected_shape(kind: MemOpKind) -> (bool, usize, EffectMask) {
    match kind {
        MemOpKind::AddrOf => (true, 1, EffectMask::PURE),
        MemOpKind::LogicalShr | MemOpKind::BitAnd | MemOpKind::Add | MemOpKind::Sub => {
            (true, 2, EffectMask::PURE)
        }
        MemOpKind::TableIndex => (true, 2, EffectMask::READ),
        MemOpKind::FieldLoad => (true, 1, EffectMask::READ),
        MemOpKind::FieldStore => (false, 2, EffectMask::WRITE),
        MemOpKind::CurrentAllocOwnerId => (true, 0, EffectMask::PURE),
        MemOpKind::OwnerEq => (true, 2, EffectMask::PURE),
    }
}

fn check_region_counts(
    function: &MirFunction,
    regions: &BTreeMap<u32, &crate::mir::function::FastMemRegionMetadata>,
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

fn check_memop_escape(
    function: &MirFunction,
    sites: &[MemOpSite],
    errors: &mut Vec<VerificationError>,
) {
    let mut produced: BTreeMap<ValueId, (FastMemRegionId, MemOpKind)> = BTreeMap::new();
    for site in sites {
        if let Some(dst) = site.dst {
            produced.insert(dst, (site.region, site.kind));
        }
    }
    if produced.is_empty() {
        return;
    }

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, sp) in block.all_spanned_instructions_enumerated() {
            let is_memop = matches!(sp.inst, MirInstruction::MemOp { .. });
            if is_memop {
                continue;
            }
            for used in sp.inst.used_values() {
                let Some((region, kind)) = produced.get(&used).copied() else {
                    continue;
                };
                push_region_error(
                    function,
                    Some(block_id),
                    Some(instruction_index),
                    Some(region.0),
                    function
                        .metadata
                        .fastmem_regions
                        .iter()
                        .find(|metadata| metadata.id == region)
                        .map(|metadata| metadata.contract.clone()),
                    &format!("memop-value-escapes kind={}", memop_kind_name(kind)),
                    errors,
                );
            }
        }
    }
}

fn push_region_error(
    function: &MirFunction,
    block: Option<BasicBlockId>,
    instruction_index: Option<usize>,
    region: Option<u32>,
    contract: Option<String>,
    reason: impl Into<String>,
    errors: &mut Vec<VerificationError>,
) {
    errors.push(VerificationError::FastMemContractViolation {
        function: function.signature.name.clone(),
        block,
        instruction_index,
        region,
        contract,
        reason: format!("[freeze:contract][fastmem/{}]", reason.into()),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::function::{FastMemRegionMetadata, FastMemRegionOrigin};
    use crate::mir::{FunctionSignature, MirType};

    fn test_function(instructions: Vec<MirInstruction>) -> MirFunction {
        let signature = FunctionSignature {
            name: "Main.fastmem/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id: FastMemRegionId::new(0),
                contract: "PageMapV0".to_string(),
                source_span: Span::unknown(),
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count: 1,
                emitted_memop_count: instructions
                    .iter()
                    .filter(|instruction| matches!(instruction, MirInstruction::MemOp { .. }))
                    .count(),
            });
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        function
    }

    fn memop(
        kind: MemOpKind,
        dst: Option<ValueId>,
        operands: Vec<ValueId>,
        effects: EffectMask,
    ) -> MirInstruction {
        MirInstruction::MemOp {
            region: FastMemRegionId::new(0),
            kind,
            dst,
            operands,
            effects,
        }
    }

    fn error_text(function: &MirFunction) -> String {
        check_fastmem_regions(function)
            .expect_err("expected fastmem verification violation")
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn accepts_region_metadata_and_memop_shapes() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                EffectMask::PURE,
            ),
            memop(
                MemOpKind::LogicalShr,
                Some(ValueId::new(2)),
                vec![ValueId::new(1), ValueId::new(0)],
                EffectMask::PURE,
            ),
            memop(
                MemOpKind::FieldStore,
                None,
                vec![ValueId::new(2), ValueId::new(0)],
                EffectMask::WRITE,
            ),
        ]);

        assert!(
            check_fastmem_regions(&function).is_ok(),
            "{}",
            error_text(&function)
        );
    }

    #[test]
    fn rejects_memop_value_escape_to_return() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                EffectMask::PURE,
            ),
            MirInstruction::Return {
                value: Some(ValueId::new(1)),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
    }

    #[test]
    fn rejects_wrong_memop_effects() {
        let function = test_function(vec![memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            EffectMask::PURE,
        )]);

        let text = error_text(&function);
        assert!(text.contains("effect-mask-mismatch"), "{}", text);
    }

    #[test]
    fn rejects_unknown_region() {
        let mut function = test_function(vec![MirInstruction::MemOp {
            region: FastMemRegionId::new(7),
            kind: MemOpKind::AddrOf,
            dst: Some(ValueId::new(1)),
            operands: vec![ValueId::new(0)],
            effects: EffectMask::PURE,
        }]);
        function.metadata.fastmem_regions[0].emitted_memop_count = 0;

        let text = error_text(&function);
        assert!(text.contains("unknown-region"), "{}", text);
    }
}
