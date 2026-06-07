use crate::mir::function::{
    FastMemBlockNextFact, FastMemFreeHeadNonEmptyFact, FastMemLocalFreeNonEmptyFact,
    FastMemRegionMetadata, FastMemRemoteOwnerFact, FastMemSameOwnerFact, FastMemTableLengthFact,
    RangeIndexFact,
};
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

pub(crate) struct FastMemFactStore<'a> {
    pub(crate) regions: &'a [FastMemRegionMetadata],
    pub(crate) table_length_facts: &'a [FastMemTableLengthFact],
    pub(crate) same_owner_facts: &'a [FastMemSameOwnerFact],
    pub(crate) remote_owner_facts: &'a [FastMemRemoteOwnerFact],
    pub(crate) block_next_facts: &'a [FastMemBlockNextFact],
    pub(crate) local_free_non_empty_facts: &'a [FastMemLocalFreeNonEmptyFact],
    pub(crate) free_head_non_empty_facts: &'a [FastMemFreeHeadNonEmptyFact],
    pub(crate) remote_drain_token_facts: &'a [FastMemRemoteDrainTokenFact],
    pub(crate) range_index_facts: &'a [RangeIndexFact],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FastMemRemoteDrainTokenFact {
    pub(crate) region: FastMemRegionId,
    pub(crate) page_value: ValueId,
    pub(crate) token_value: ValueId,
    pub(crate) block: BasicBlockId,
    pub(crate) instruction_index: usize,
}

impl<'a> FastMemFactStore<'a> {
    pub(crate) fn table_length(
        &self,
        region: FastMemRegionId,
        table_id: &str,
        table_value: ValueId,
    ) -> Option<&'a FastMemTableLengthFact> {
        self.table_length_facts.iter().find(|fact| {
            fact.region == region && fact.table_id == table_id && fact.table_value == table_value
        })
    }

    pub(crate) fn range_bounds_proof(
        &self,
        block: BasicBlockId,
        index_value: ValueId,
        length_fact: &FastMemTableLengthFact,
    ) -> Option<String> {
        self.range_index_facts.iter().find_map(|fact| {
            if fact.index_value == index_value
                && fact.upper_exclusive_value == length_fact.length_value
                && fact.body_bb == block
                && fact.step == 1
                && fact.end_exclusive
                && fact.index_body_read_only
                && !fact.loop_carried_writes_supported
            {
                Some(format!("range_fact:{}", fact.fact_id))
            } else {
                None
            }
        })
    }

    pub(crate) fn same_owner(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemSameOwnerFact> {
        self.same_owner_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    pub(crate) fn remote_owner(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemRemoteOwnerFact> {
        self.remote_owner_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    pub(crate) fn block_next(
        &self,
        region: FastMemRegionId,
        block_value: ValueId,
    ) -> Option<&'a FastMemBlockNextFact> {
        self.block_next_facts
            .iter()
            .find(|fact| fact.region == region && fact.block_value == block_value)
    }

    pub(crate) fn local_free_non_empty(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemLocalFreeNonEmptyFact> {
        self.local_free_non_empty_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    pub(crate) fn free_head_non_empty(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Option<&'a FastMemFreeHeadNonEmptyFact> {
        self.free_head_non_empty_facts
            .iter()
            .find(|fact| fact.region == region && fact.page_value == page_value)
    }

    pub(crate) fn remote_drain_token(
        &self,
        region: FastMemRegionId,
        page_value: ValueId,
        token_value: ValueId,
    ) -> Option<&'a FastMemRemoteDrainTokenFact> {
        self.remote_drain_token_facts.iter().find(|fact| {
            fact.region == region
                && fact.page_value == page_value
                && fact.token_value == token_value
        })
    }

    pub(crate) fn region_contract(&self, region: FastMemRegionId) -> Option<&str> {
        region_contract(self.regions, region)
    }
}

pub(crate) fn collect_remote_drain_token_facts(
    function: &MirFunction,
) -> Vec<FastMemRemoteDrainTokenFact> {
    let mut facts = Vec::new();
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
                ..
            } = sp.inst
            else {
                continue;
            };
            if *kind != MemOpKind::AtomicRemoteHeadDrain {
                continue;
            }
            let (Some(token_value), Some(page_value)) = (*dst, operands.first().copied()) else {
                continue;
            };
            facts.push(FastMemRemoteDrainTokenFact {
                region: *region,
                page_value,
                token_value,
                block: block_id,
                instruction_index,
            });
        }
    }
    facts
}

pub(crate) fn region_contract(
    regions: &[FastMemRegionMetadata],
    region: FastMemRegionId,
) -> Option<&str> {
    regions
        .iter()
        .find(|metadata| metadata.id == region)
        .map(|metadata| metadata.contract.as_str())
}
