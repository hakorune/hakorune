/*!
 * MIR-owned FastMemory layout/table access plans.
 *
 * `MemOpAccess` carries symbolic source ids. This module publishes the next
 * metadata seam: a function-local access-plan row for each layout/table MemOp
 * site.  The current slice is intentionally conservative; without canonical
 * layout/table contracts it records `symbolic_only` rows and keeps LLVM
 * GEP/load/store lowering closed.
 */

use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
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
) -> Option<FastMemAccessPlan> {
    match kind {
        MemOpKind::TableIndex => {
            table_plan(block, instruction_index, region, dst, operands, access)
        }
        MemOpKind::FieldLoad => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Load,
        ),
        MemOpKind::FieldStore => field_plan(
            block,
            instruction_index,
            region,
            dst,
            operands,
            access,
            FastMemFieldAccessMode::Store,
        ),
        _ => None,
    }
}

fn table_plan(
    block: BasicBlockId,
    instruction_index: usize,
    region: FastMemRegionId,
    dst: Option<ValueId>,
    operands: &[ValueId],
    access: Option<&MemOpAccess>,
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let table_id = access.table_id.as_ref()?.clone();
    let table = operands.first().copied()?;
    let index = operands.get(1).copied()?;
    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: FastMemAccessPlanKind::TableIndex,
        status: FastMemAccessPlanStatus::SymbolicOnly,
        failure_reason: Some("layout-table-contract-unresolved".to_string()),
        payload: FastMemAccessPlanPayload::Table(FastMemTableAccessPlan {
            table_id,
            table,
            index,
            result: dst,
            element_layout_id: None,
            element_repr: None,
            element_stride: None,
            length: None,
            alignment: None,
            index_policy: None,
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
) -> Option<FastMemAccessPlan> {
    let access = access?;
    let field_id = access.field_id.as_ref()?.clone();
    let base = operands.first().copied()?;
    let value = if mode == FastMemFieldAccessMode::Store {
        operands.get(1).copied()
    } else {
        None
    };
    Some(FastMemAccessPlan {
        block,
        instruction_index,
        region,
        kind: match mode {
            FastMemFieldAccessMode::Load => FastMemAccessPlanKind::FieldLoad,
            FastMemFieldAccessMode::Store => FastMemAccessPlanKind::FieldStore,
        },
        status: FastMemAccessPlanStatus::SymbolicOnly,
        failure_reason: Some("layout-field-contract-unresolved".to_string()),
        payload: FastMemAccessPlanPayload::Field(FastMemFieldAccessPlan {
            layout_id: access.layout_id.clone(),
            field_id,
            base,
            value,
            result: dst,
            mode,
            byte_offset: None,
            field_type: None,
            alignment: None,
            mutability: None,
            field_class: None,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn refresh_records_symbolic_layout_table_sites() {
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
        assert!(function
            .metadata
            .fastmem_access_plans
            .iter()
            .all(|plan| plan.status == FastMemAccessPlanStatus::SymbolicOnly));
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
