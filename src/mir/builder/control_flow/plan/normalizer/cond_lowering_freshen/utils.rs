use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Remap a ValueId through the map (no allocation, lookup only)
pub(crate) fn remap_value_id(value_map: &BTreeMap<ValueId, ValueId>, old_id: ValueId) -> ValueId {
    value_map.get(&old_id).copied().unwrap_or(old_id)
}

/// Remap a Vec<ValueId>
pub(crate) fn remap_value_ids(
    value_map: &BTreeMap<ValueId, ValueId>,
    old_ids: &[ValueId],
) -> Vec<ValueId> {
    old_ids
        .iter()
        .map(|&v| remap_value_id(value_map, v))
        .collect()
}

pub(crate) fn map_block_id(
    builder: &mut MirBuilder,
    block_map: &mut BTreeMap<BasicBlockId, BasicBlockId>,
    block_id: BasicBlockId,
) -> BasicBlockId {
    if let Some(mapped) = block_map.get(&block_id) {
        return *mapped;
    }
    let new_id = builder.next_block_id();
    block_map.insert(block_id, new_id);
    new_id
}
