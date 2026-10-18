//! Exact physical Phi closure shared by both Loop entry forms.
//! Source producers/JoinSig/permission remain with the original caller.
use super::loop_cond_after_role::freeze;
use crate::mir::{BasicBlockId, ValueId, MirInstruction, MirFunction};
use std::collections::{BTreeMap, BTreeSet};

pub(in crate::mir::builder) type LoopPhiInputRowsV2 = Vec<(BasicBlockId, ValueId, Vec<(BasicBlockId, ValueId)>)>;
pub(in crate::mir) fn capture_exact_loop_phis_v2(function: &MirFunction,
    rows: &[(BasicBlockId, ValueId, Vec<(BasicBlockId, ValueId)>)],
) -> Result<BTreeMap<BasicBlockId, Box<[MirInstruction]>>, String> {
    let coordinates: BTreeSet<_> = rows.iter().map(|row| (row.0, row.1)).collect();
    if coordinates.len() != rows.len() || rows.iter().map(|row| row.1).collect::<BTreeSet<_>>().len() != rows.len() {
        return Err(freeze("plan-destination-duplicate"));
    }
    let mut expected = BTreeMap::<_, Vec<_>>::new();
    for (phi_block, phi_dst, phi_inputs) in rows {
        // The canonical lowerer can delete an unreferenced empty step Phi.
        // Source LoopCond carriers always have a real predecessor payload.
        if phi_inputs.is_empty() { return Err(freeze("plan-input-missing")); }
        if !function.blocks.contains_key(&*phi_block) { return Err(freeze("block-missing")); }
        let rows: Vec<_> = function.blocks.iter().flat_map(|(id, block)|
            block.all_instructions().filter(move |row| row.dst_value() == Some(*phi_dst)).map(move |row| (*id, row))).collect();
        let [(block, row)] = rows.as_slice() else { return Err(freeze("phi-definition-cardinality")); };
        if *block != *phi_block || function.params.contains(&*phi_dst) { return Err(freeze("phi-definition-cardinality")); }
        if !matches!(row, MirInstruction::Phi { dst, inputs, .. } if *dst == *phi_dst && inputs == phi_inputs) {
            return Err(freeze("plan-phi-drift"));
        }
        let mut incoming = BTreeSet::new();
        if phi_inputs.iter().any(|(pred, _)| !incoming.insert(*pred)) { return Err(freeze("duplicate-predecessor")); }
        let predecessors: BTreeSet<_> = function.blocks.iter().filter_map(|(id, block)|
            block.successors_from_terminator().contains(&*phi_block).then_some(*id)).collect();
        if incoming != predecessors { return Err(freeze("predecessor-coverage")); }
        expected.entry(*phi_block).or_default().push((**row).clone());
    }
    let mut loop_phis = BTreeMap::new();
    for (block, rows) in expected {
        let prefix: Vec<_> = function.blocks[&block].instructions.iter()
            .take_while(|row| matches!(row, MirInstruction::Phi { .. })).cloned().collect();
        if prefix != rows { return Err(freeze("phi-prefix-drift")); }
        loop_phis.insert(block, rows.into_boxed_slice());
    }
    Ok(loop_phis)
}
