/*!
 * MIR-owned FastMemory layout/table access plans.
 *
 * `MemOpAccess` carries symbolic source ids. This module publishes the next
 * metadata seam: a function-local access-plan row for each layout/table MemOp
 * site. Verified rows are produced only by the memory-profile contract
 * resolver. LLVM GEP/load/store lowering remains closed until it consumes
 * verified rows without recomputing layout/table facts.
 */

mod fact_store;
mod field;
mod free_list;
mod head_access;
mod linked_list;
mod remote;
mod table;
mod types;

use crate::mir::function::FastMemFreeHeadNonEmptyProofKind;
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use fact_store::{collect_remote_drain_token_facts, region_contract, FastMemFactStore};
use field::field_plan;
use free_list::{free_head_plan, local_free_plan, maybe_add_derived_free_head_non_empty_fact};
use remote::{atomic_remote_head_plan, drain_remote_list_to_local_plan};
use table::{table_field_access_links, table_plan};
pub use types::{
    FastMemAccessPlan, FastMemAccessPlanKind, FastMemAccessPlanPayload, FastMemAccessPlanStatus,
    FastMemAtomicRemoteHeadPlan, FastMemDrainRemoteListToLocalPlan, FastMemFieldAccessMode,
    FastMemFieldAccessPlan, FastMemFreeHeadListPlan, FastMemLocalFreeListPlan,
    FastMemTableAccessPlan, FastMemTableAccessProof, FastMemTableFieldAccessLink,
};

pub fn refresh_function_fastmem_access_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let regions = function.metadata.fastmem_regions.clone();
    let table_length_facts = function.metadata.fastmem_table_length_facts.clone();
    let same_owner_facts = function.metadata.fastmem_same_owner_facts.clone();
    let remote_owner_facts = function.metadata.fastmem_remote_owner_facts.clone();
    let block_next_facts = function.metadata.fastmem_block_next_facts.clone();
    let local_free_non_empty_facts = function.metadata.fastmem_local_free_non_empty_facts.clone();
    let mut free_head_non_empty_facts: Vec<_> = function
        .metadata
        .fastmem_free_head_non_empty_facts
        .iter()
        .filter(|fact| fact.proof_kind != FastMemFreeHeadNonEmptyProofKind::DerivedFromFreeHeadPush)
        .cloned()
        .collect();
    let range_index_facts = function.metadata.range_index_facts.clone();
    let remote_drain_token_facts = collect_remote_drain_token_facts(function);

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
            let fact_store = FastMemFactStore {
                regions: &regions,
                table_length_facts: &table_length_facts,
                same_owner_facts: &same_owner_facts,
                remote_owner_facts: &remote_owner_facts,
                block_next_facts: &block_next_facts,
                local_free_non_empty_facts: &local_free_non_empty_facts,
                free_head_non_empty_facts: &free_head_non_empty_facts,
                remote_drain_token_facts: &remote_drain_token_facts,
                range_index_facts: &range_index_facts,
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
                &fact_store,
            ) else {
                continue;
            };
            maybe_add_derived_free_head_non_empty_fact(&plan, &mut free_head_non_empty_facts);
            plans.push(plan);
        }
    }

    let table_field_links = table_field_access_links(&mut plans);
    function.metadata.fastmem_access_plans = plans;
    function.metadata.fastmem_table_field_access_links = table_field_links;
    function.metadata.fastmem_free_head_non_empty_facts = free_head_non_empty_facts;
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
    facts: &FastMemFactStore<'_>,
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
            facts,
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
        MemOpKind::LocalFreePush => local_free_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::LocalFreePush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::LocalFreePop => local_free_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::LocalFreePop,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::FreeHeadPop => free_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::FreeHeadPop,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::FreeHeadPush => free_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::FreeHeadPush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::AtomicRemoteHeadPush => atomic_remote_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::AtomicRemoteHeadPush,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::AtomicRemoteHeadDrain => atomic_remote_head_plan(
            block,
            instruction_index,
            region,
            FastMemAccessPlanKind::AtomicRemoteHeadDrain,
            dst,
            operands,
            contract,
            facts,
        ),
        MemOpKind::DrainRemoteListToLocal => {
            drain_remote_list_to_local_plan(block, instruction_index, region, dst, operands, facts)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests;
