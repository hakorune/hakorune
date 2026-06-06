/*!
 * MIR-owned FastMemory layout/table access plans.
 *
 * `MemOpAccess` carries symbolic source ids. This module publishes the next
 * metadata seam: a function-local access-plan row for each layout/table MemOp
 * site. Verified rows are produced only by the memory-profile contract
 * resolver. LLVM GEP/load/store lowering remains closed until it consumes
 * verified rows without recomputing layout/table facts.
 */

use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{
    fastmem_layout_contract::{resolve_fastmem_field_contract, resolve_fastmem_table_contract},
    function::FastMemRegionMetadata,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemAccessPlanKind {
    TableIndex,
    FieldLoad,
    FieldStore,
}

impl FastMemAccessPlanKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TableIndex => "table_index",
            Self::FieldLoad => "field_load",
            Self::FieldStore => "field_store",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemAccessPlanStatus {
    SymbolicOnly,
    Verified,
    Rejected,
}

impl FastMemAccessPlanStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SymbolicOnly => "symbolic_only",
            Self::Verified => "verified",
            Self::Rejected => "rejected",
        }
    }

    pub fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastMemFieldAccessMode {
    Load,
    Store,
}

impl FastMemFieldAccessMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Load => "load",
            Self::Store => "store",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemFieldAccessPlan {
    pub layout_id: Option<String>,
    pub field_id: String,
    pub base: ValueId,
    pub value: Option<ValueId>,
    pub result: Option<ValueId>,
    pub mode: FastMemFieldAccessMode,
    pub byte_offset: Option<u32>,
    pub field_type: Option<String>,
    pub alignment: Option<u32>,
    pub mutability: Option<String>,
    pub field_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableAccessPlan {
    pub table_id: String,
    pub table: ValueId,
    pub index: ValueId,
    pub result: Option<ValueId>,
    pub element_layout_id: Option<String>,
    pub element_repr: Option<String>,
    pub element_stride: Option<u32>,
    pub length: Option<u64>,
    pub alignment: Option<u32>,
    pub index_policy: Option<String>,
    pub proof: FastMemTableAccessProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemTableAccessProof {
    pub table_length_resolved: bool,
    pub bounds_proof_valid: bool,
    pub stride_resolved: bool,
    pub field_offset_resolved: bool,
    pub overflow_proof_valid: bool,
    pub alignment_valid: bool,
    pub element_layout_verified: bool,
    pub table_length_policy: Option<String>,
    pub bounds_proof: Option<String>,
    pub overflow_proof: Option<String>,
    pub failure_reason: Option<String>,
}

impl FastMemTableAccessProof {
    pub fn is_lowerable(&self) -> bool {
        self.table_length_resolved
            && self.bounds_proof_valid
            && self.stride_resolved
            && self.field_offset_resolved
            && self.overflow_proof_valid
            && self.alignment_valid
            && self.element_layout_verified
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastMemAccessPlanPayload {
    Field(FastMemFieldAccessPlan),
    Table(FastMemTableAccessPlan),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FastMemAccessPlan {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub region: FastMemRegionId,
    pub kind: FastMemAccessPlanKind,
    pub status: FastMemAccessPlanStatus,
    pub failure_reason: Option<String>,
    pub payload: FastMemAccessPlanPayload,
}

impl FastMemAccessPlan {
    pub fn is_verified(&self) -> bool {
        self.status.is_verified()
    }
}

pub fn refresh_function_fastmem_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let regions = function.metadata.fastmem_regions.clone();

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
                ..
            } = sp.inst
            else {
                continue;
            };
            let Some(plan) = plan_from_memop(
                block_id,
                instruction_index,
                *region,
                *kind,
                *dst,
                operands,
                access.as_ref(),
                region_contract(&regions, *region),
            ) else {
                continue;
            };
            plans.push(plan);
        }
    }

    function.metadata.fastmem_access_plans = plans;
}

fn plan_from_memop(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    contract: Option<&str>,
) -> Option<FastMemAccessPlan> {
    match kind {
        MemOpKind::TableIndex => table_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            contract,
        ),
        MemOpKind::FieldLoad => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Load,
            contract,
        ),
        MemOpKind::FieldStore => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Store,
            contract,
        ),
        _ => None,
    }
}

fn region_contract(regions: &[FastMemRegionMetadata], region: FastMemRegionId) -> Option<&str> {
    regions
        .iter()
        .find(|metadata| metadata.id == region)
        .map(|metadata| metadata.contract.as_str())
}

fn table_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    contract: Option<&str>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let table_id = access.table_id.as_ref()?.clone();
    let table = operands.first().copied()?;
    let index = operands.get(1).copied()?;
    let resolved = contract.map(|contract| {
        resolve_fastmem_table_contract(contract, &table_id).map_err(|err| err.reason())
    });
    let (
        status,
        failure_reason,
        element_layout_id,
        element_repr,
        element_stride,
        length,
        alignment,
        index_policy,
    ) = match resolved {
        Some(Ok(resolved)) if resolved.lowerable => (
            FastMemAccessPlanStatus::Verified,
            None,
            Some(resolved.element_layout_id),
            Some(resolved.element_repr),
            Some(resolved.element_stride),
            resolved.length,
            Some(resolved.alignment),
            Some(resolved.index_policy),
        ),
        Some(Ok(resolved)) => (
            FastMemAccessPlanStatus::Rejected,
            resolved.non_lowerable_reason,
            Some(resolved.element_layout_id),
            Some(resolved.element_repr),
            Some(resolved.element_stride),
            resolved.length,
            Some(resolved.alignment),
            Some(resolved.index_policy),
        ),
        Some(Err(reason)) => (
            FastMemAccessPlanStatus::Rejected,
            Some(reason),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        None => (
            FastMemAccessPlanStatus::SymbolicOnly,
            Some("layout-table-contract-unresolved".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    let proof = FastMemTableAccessProof {
        table_length_resolved: length.is_some(),
        bounds_proof_valid: false,
        stride_resolved: element_stride.is_some(),
        field_offset_resolved: false,
        overflow_proof_valid: false,
        alignment_valid: alignment.is_some(),
        element_layout_verified: element_layout_id.is_some(),
        table_length_policy: length.map(|value| format!("const_len:{value}")),
        bounds_proof: None,
        overflow_proof: None,
        failure_reason: failure_reason.clone(),
    };
    let status = if status == FastMemAccessPlanStatus::Verified && !proof.is_lowerable() {
        FastMemAccessPlanStatus::Rejected
    } else {
        status
    };
    let failure_reason = failure_reason.or_else(|| {
        if status == FastMemAccessPlanStatus::Rejected && !proof.is_lowerable() {
            Some("verified-table-access-proof-incomplete".to_string())
        } else {
            None
        }
    });

    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: FastMemAccessPlanKind::TableIndex,
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::Table(FastMemTableAccessPlan {
            table_id,
            table,
            index,
            result: dst,
            element_layout_id,
            element_repr,
            element_stride,
            length,
            alignment,
            index_policy,
            proof,
        }),
    })
}

fn field_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
    mode: FastMemFieldAccessMode,
    contract: Option<&str>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let field_id = access.field_id.as_ref()?.clone();
    let base = operands.first().copied()?;
    let value = if mode == FastMemFieldAccessMode::Store {
        operands.get(1).copied()
    } else {
        None
    };
    let resolved = contract.map(|contract| {
        resolve_fastmem_field_contract(contract, &field_id, mode).map_err(|err| err.reason())
    });
    let (
        status,
        failure_reason,
        layout_id,
        canonical_field_id,
        byte_offset,
        field_type,
        alignment,
        mutability,
        field_class,
    ) = match resolved {
        Some(Ok(resolved)) => (
            FastMemAccessPlanStatus::Verified,
            None,
            Some(resolved.layout_id),
            resolved.field_id,
            Some(resolved.byte_offset),
            Some(resolved.field_type),
            Some(resolved.alignment),
            Some(resolved.mutability),
            Some(resolved.field_class),
        ),
        Some(Err(reason)) => (
            FastMemAccessPlanStatus::Rejected,
            Some(reason),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
        ),
        None => (
            FastMemAccessPlanStatus::SymbolicOnly,
            Some("layout-field-contract-unresolved".to_string()),
            access.layout_id.clone(),
            field_id,
            None,
            None,
            None,
            None,
            None,
        ),
    };
    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: match mode {
            FastMemFieldAccessMode::Load => FastMemAccessPlanKind::FieldLoad,
            FastMemFieldAccessMode::Store => FastMemAccessPlanKind::FieldStore,
        },
        status,
        failure_reason,
        payload: FastMemAccessPlanPayload::Field(FastMemFieldAccessPlan {
            layout_id,
            field_id: canonical_field_id,
            base,
            value,
            result: dst,
            mode,
            byte_offset,
            field_type,
            alignment,
            mutability,
            field_class,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::function::{FastMemRegionMetadata, FastMemRegionOrigin};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn make_function(instructions: Vec<MirInstruction>) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.fastmem/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        function
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id: FastMemRegionId::new(0),
                contract: "PageMapV0".to_string(),
                source_span: Span::unknown(),
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count: 1,
                emitted_memop_count: function
                    .blocks
                    .get(&BasicBlockId::new(0))
                    .map(|block| {
                        block
                            .instructions
                            .iter()
                            .filter(|instruction| {
                                matches!(instruction, MirInstruction::MemOp { .. })
                            })
                            .count()
                    })
                    .unwrap_or(0),
            });
        function
    }

    fn memop(
        kind: MemOpKind,
        dst: Option<ValueId>,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> MirInstruction {
        MirInstruction::MemOp {
            region: FastMemRegionId::new(0),
            kind,
            dst,
            operands,
            access,
            effects: kind.effect_mask(),
        }
    }

    #[test]
    fn refresh_verifies_page_meta_field_sites_and_rejects_unbounded_table() {
        let mut function = make_function(vec![
            memop(
                MemOpKind::TableIndex,
                Some(ValueId::new(10)),
                vec![ValueId::new(1), ValueId::new(2)],
                Some(MemOpAccess::table("page_table")),
            ),
            memop(
                MemOpKind::FieldLoad,
                Some(ValueId::new(11)),
                vec![ValueId::new(10)],
                Some(MemOpAccess::field("owner_id")),
            ),
            memop(
                MemOpKind::FieldStore,
                None,
                vec![ValueId::new(10), ValueId::new(3)],
                Some(MemOpAccess::field("local_free_head")),
            ),
        ]);

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 3);
        assert_eq!(
            function.metadata.fastmem_access_plans[0].status,
            FastMemAccessPlanStatus::Rejected
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[0]
                .failure_reason
                .as_deref(),
            Some("table-length-unresolved")
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[1].status,
            FastMemAccessPlanStatus::Verified
        );
        assert_eq!(
            function.metadata.fastmem_access_plans[2].status,
            FastMemAccessPlanStatus::Verified
        );
        let FastMemAccessPlanPayload::Field(field) =
            &function.metadata.fastmem_access_plans[1].payload
        else {
            panic!("expected owner field plan");
        };
        assert_eq!(field.layout_id.as_deref(), Some("PageMetaLayoutV0"));
        assert_eq!(field.field_id, "owner_worker_id");
        assert_eq!(field.byte_offset, Some(0));
        assert_eq!(field.field_class.as_deref(), Some("plain_scalar"));
        let FastMemAccessPlanPayload::Table(table) =
            &function.metadata.fastmem_access_plans[0].payload
        else {
            panic!("expected table plan");
        };
        assert_eq!(table.element_layout_id.as_deref(), Some("PageMetaLayoutV0"));
        assert_eq!(table.element_repr.as_deref(), Some("pointer_to_element"));
        assert!(!table.proof.is_lowerable());
        assert!(!table.proof.table_length_resolved);
        assert!(!table.proof.bounds_proof_valid);
        assert!(table.proof.stride_resolved);
        assert!(!table.proof.field_offset_resolved);
        assert!(!table.proof.overflow_proof_valid);
        assert!(table.proof.alignment_valid);
        assert!(table.proof.element_layout_verified);
        assert_eq!(
            table.proof.failure_reason.as_deref(),
            Some("table-length-unresolved")
        );
    }

    #[test]
    fn refresh_rejects_plain_store_to_atomic_remote_head() {
        let mut function = make_function(vec![memop(
            MemOpKind::FieldStore,
            None,
            vec![ValueId::new(10), ValueId::new(3)],
            Some(MemOpAccess::field("remote_head")),
        )]);

        refresh_function_fastmem_access_plans(&mut function);

        assert_eq!(function.metadata.fastmem_access_plans.len(), 1);
        let plan = &function.metadata.fastmem_access_plans[0];
        assert_eq!(plan.status, FastMemAccessPlanStatus::Rejected);
        assert_eq!(
            plan.failure_reason.as_deref(),
            Some("atomic-field-plain-store:remote_head")
        );
    }

    #[test]
    fn refresh_ignores_layout_table_memops_without_symbolic_ids() {
        let mut function = make_function(vec![memop(
            MemOpKind::FieldLoad,
            Some(ValueId::new(1)),
            vec![ValueId::new(0)],
            None,
        )]);

        refresh_function_fastmem_access_plans(&mut function);

        assert!(function.metadata.fastmem_access_plans.is_empty());
    }
}
