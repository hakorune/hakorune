use std::collections::{BTreeMap, BTreeSet};

use crate::mir::contracts::fastmem_ops::is_fastmem_v0_memop_kind;
use crate::mir::escape_barrier::{classify_escape_uses, EscapeBarrier};
use crate::mir::function::MirFunction;
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
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
        if !memop_escape_tracked_kind(site.kind) {
            continue;
        }
        if let Some(dst) = site.dst {
            produced.insert(dst, (site.region, site.kind));
        }
    }
    extend_single_input_phi_aliases(function, &mut produced);
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
            for escape_use in fastmem_escape_uses(sp.inst) {
                let Some((region, kind)) = produced.get(&escape_use.value).copied() else {
                    continue;
                };
                if allowed_fastmem_escape_use(sp.inst, kind, escape_use.value) {
                    continue;
                }
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
                    &format!(
                        "memop-value-escapes kind={} barrier={}",
                        kind.display_name(),
                        escape_use.barrier
                    ),
                    errors,
                );
            }
        }
    }
}

fn extend_single_input_phi_aliases(
    function: &MirFunction,
    produced: &mut BTreeMap<ValueId, (FastMemRegionId, MemOpKind)>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        for block_id in function.block_ids() {
            let Some(block) = function.blocks.get(&block_id) else {
                continue;
            };
            for sp in block.all_spanned_instructions() {
                match sp.inst {
                    MirInstruction::Phi { dst, inputs, .. } => {
                        if produced.contains_key(dst) || inputs.len() != 1 {
                            if produced.contains_key(dst) || inputs.is_empty() {
                                continue;
                            }
                            let Some(origin) = phi_merge_origin(produced, inputs.as_slice()) else {
                                continue;
                            };
                            produced.insert(*dst, origin);
                            changed = true;
                            continue;
                        }
                        let Some((_, input)) = inputs.first() else {
                            continue;
                        };
                        let Some(origin) = produced.get(input).copied() else {
                            continue;
                        };
                        produced.insert(*dst, origin);
                        changed = true;
                    }
                    MirInstruction::Copy { dst, src } => {
                        if produced.contains_key(dst) {
                            continue;
                        }
                        let Some(origin @ (_, kind)) = produced.get(src).copied()
                        else {
                            continue;
                        };
                        if memop_escape_tracked_kind(kind) {
                            produced.insert(*dst, origin);
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn phi_merge_origin(
    produced: &BTreeMap<ValueId, (FastMemRegionId, MemOpKind)>,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<(FastMemRegionId, MemOpKind)> {
    let mut origin: Option<(FastMemRegionId, MemOpKind)> = None;
    for (_, input) in inputs {
        let current = produced.get(input).copied()?;
        if !phi_merge_aliasable_memop_kind(current.1) {
            return None;
        }
        match origin {
            None => origin = Some(current),
            Some(existing) if existing == current => {}
            Some(_) => return None,
        }
    }
    origin
}

fn phi_merge_aliasable_memop_kind(kind: MemOpKind) -> bool {
    matches!(kind, MemOpKind::TableIndex | MemOpKind::AtomicRemoteHeadDrain)
}

#[derive(Debug, Clone, Copy)]
struct FastMemEscapeUse {
    value: ValueId,
    barrier: FastMemEscapeBarrier,
}

#[derive(Debug, Clone, Copy)]
enum FastMemEscapeBarrier {
    Shared(EscapeBarrier),
    OrdinaryUse,
}

impl std::fmt::Display for FastMemEscapeBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Shared(barrier) => write!(f, "{}", barrier),
            Self::OrdinaryUse => f.write_str("ordinary_use"),
        }
    }
}

fn fastmem_escape_uses(inst: &MirInstruction) -> Vec<FastMemEscapeUse> {
    let shared = classify_escape_uses(inst);
    let mut uses: Vec<FastMemEscapeUse> = shared
        .iter()
        .map(|use_site| FastMemEscapeUse {
            value: use_site.value,
            barrier: FastMemEscapeBarrier::Shared(use_site.barrier),
        })
        .collect();
    let shared_values: BTreeSet<ValueId> = shared.iter().map(|use_site| use_site.value).collect();
    for value in inst.used_values() {
        if shared_values.contains(&value) {
            continue;
        }
        uses.push(FastMemEscapeUse {
            value,
            barrier: FastMemEscapeBarrier::OrdinaryUse,
        });
    }
    uses
}

fn allowed_fastmem_escape_use(inst: &MirInstruction, kind: MemOpKind, value: ValueId) -> bool {
    matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Branch { condition, .. }
    ) if *condition == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::FieldGet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::FieldSet { base, .. }
        ) if *base == value
        ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::OwnerEq,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::TableIndex,
            MirInstruction::Phi { inputs, .. }
        ) if inputs.iter().any(|(_, input)| *input == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::BinOp { lhs, rhs, .. }
        ) if *lhs == value || *rhs == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::Compare { lhs, rhs, .. }
        ) if *lhs == value || *rhs == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AddrOf,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::FieldGet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::FieldSet { base, .. }
        ) if *base == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Copy { src, .. }
        ) if *src == value
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Call {
                callee: Some(crate::mir::Callee::Extern(name) | crate::mir::Callee::Global(name)),
                args,
                ..
            }
        ) if name.starts_with("mem.") && args.iter().any(|arg| *arg == value)
    ) || matches!(
        (kind, inst),
        (
            MemOpKind::AtomicRemoteHeadDrain,
            MirInstruction::Phi { inputs, .. }
        ) if inputs.iter().any(|(_, input)| *input == value)
    )
}

fn memop_escape_tracked_kind(kind: MemOpKind) -> bool {
    matches!(
        kind,
        MemOpKind::AddrOf
            | MemOpKind::TableIndex
            | MemOpKind::AtomicRemoteHeadDrain
            | MemOpKind::OwnerEq
    )
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
    use crate::mir::types::BinaryOp;
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
        access: Option<MemOpAccess>,
        effects: EffectMask,
    ) -> MirInstruction {
        MirInstruction::MemOp {
            region: FastMemRegionId::new(0),
            kind,
            dst,
            operands,
            access,
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
                None,
                EffectMask::PURE,
            ),
            memop(
                MemOpKind::LogicalShr,
                Some(ValueId::new(2)),
                vec![ValueId::new(1), ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            memop(
                MemOpKind::FieldStore,
                None,
                vec![ValueId::new(2), ValueId::new(0)],
                Some(MemOpAccess::field("local_free_head")),
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
    fn accepts_owner_eq_memop_as_branch_condition() {
        let function = test_function(vec![
            memop(
                MemOpKind::OwnerEq,
                Some(ValueId::new(3)),
                vec![ValueId::new(1), ValueId::new(2)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Branch {
                condition: ValueId::new(3),
                then_bb: BasicBlockId::new(1),
                else_bb: BasicBlockId::new(2),
                then_edge_args: None,
                else_edge_args: None,
            },
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
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Return {
                value: Some(ValueId::new(1)),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=return"), "{}", text);
    }

    #[test]
    fn rejects_memop_value_escape_to_store_value() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Store {
                value: ValueId::new(1),
                ptr: ValueId::new(9),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=store_like"), "{}", text);
    }

    #[test]
    fn rejects_memop_value_escape_to_call_arg() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Call {
                dst: None,
                func: ValueId::INVALID,
                callee: Some(crate::mir::Callee::Extern("env.test.sink".to_string())),
                args: vec![ValueId::new(1)],
                effects: EffectMask::IO,
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=call"), "{}", text);
    }

    #[test]
    fn rejects_memop_value_escape_to_debug_observe() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Debug {
                value: ValueId::new(1),
                message: "observe".to_string(),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=debug_observe"), "{}", text);
    }

    #[test]
    fn rejects_layout_ref_escape_to_ordinary_use() {
        let function = test_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(1)),
                vec![ValueId::new(0), ValueId::new(9)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::BinOp {
                dst: ValueId::new(2),
                op: BinaryOp::Add,
                lhs: ValueId::new(1),
                rhs: ValueId::new(9),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=ordinary_use"), "{}", text);
    }

    #[test]
    fn accepts_addr_of_numeric_use_in_binop() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::BinOp {
                dst: ValueId::new(2),
                op: BinaryOp::Add,
                lhs: ValueId::new(1),
                rhs: ValueId::new(9),
            },
        ]);

        assert!(check_fastmem_regions(&function).is_ok(), "{}", error_text(&function));
    }

    #[test]
    fn accepts_table_index_bridge_uses_for_field_access_and_copy() {
        let function = test_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(1)),
                vec![ValueId::new(0), ValueId::new(9)],
                Some(MemOpAccess::table("page_table")),
                EffectMask::READ,
            ),
            MirInstruction::Copy {
                dst: ValueId::new(2),
                src: ValueId::new(1),
            },
            MirInstruction::FieldGet {
                dst: ValueId::new(3),
                base: ValueId::new(2),
                field: "used".to_string(),
                declared_type: None,
            },
            MirInstruction::FieldSet {
                base: ValueId::new(2),
                field: "used".to_string(),
                value: ValueId::new(3),
                declared_type: None,
            },
        ]);

        assert!(check_fastmem_regions(&function).is_ok(), "{}", error_text(&function));
    }

    #[test]
    fn single_input_phi_propagates_memop_escape_origin() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Phi {
                dst: ValueId::new(2),
                inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
                type_hint: None,
            },
            MirInstruction::Return {
                value: Some(ValueId::new(2)),
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=return"), "{}", text);
    }

    #[test]
    fn multi_input_phi_is_memop_escape_barrier() {
        let function = test_function(vec![
            memop(
                MemOpKind::AddrOf,
                Some(ValueId::new(1)),
                vec![ValueId::new(0)],
                None,
                EffectMask::PURE,
            ),
            MirInstruction::Phi {
                dst: ValueId::new(3),
                inputs: vec![
                    (BasicBlockId::new(0), ValueId::new(1)),
                    (BasicBlockId::new(1), ValueId::new(2)),
                ],
                type_hint: None,
            },
        ]);

        let text = error_text(&function);
        assert!(text.contains("memop-value-escapes"), "{}", text);
        assert!(text.contains("barrier=phi_merge"), "{}", text);
    }

    #[test]
    fn rejects_wrong_memop_effects() {
        let function = test_function(vec![memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            Some(MemOpAccess::field("owner_id")),
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
            access: None,
            effects: EffectMask::PURE,
        }]);
        function.metadata.fastmem_regions[0].emitted_memop_count = 0;

        let text = error_text(&function);
        assert!(text.contains("unknown-region"), "{}", text);
    }

    #[test]
    fn rejects_layout_table_memops_without_symbolic_access_ids() {
        let field = test_function(vec![memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
            EffectMask::READ,
        )]);
        let field_text = error_text(&field);
        assert!(
            field_text.contains("field-access-missing-field-id"),
            "{}",
            field_text
        );

        let table = test_function(vec![memop(
            MemOpKind::TableIndex,
            Some(ValueId::new(1)),
            vec![ValueId::new(0), ValueId::new(2)],
            None,
            EffectMask::READ,
        )]);
        let table_text = error_text(&table);
        assert!(
            table_text.contains("table-index-missing-table-id"),
            "{}",
            table_text
        );
    }
}
