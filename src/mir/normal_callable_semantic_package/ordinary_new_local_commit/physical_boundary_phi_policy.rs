//! PHIs in recorded lifecycle blocks need a genuine join or one exact
//! canonical Loop role. A raw one-predecessor block is never permission.
use super::*;

impl PhysicalBoundary {
    pub(in super::super) fn capture_with_loop_after_roles(
        function: &MirFunction,
        bindings: &Bindings,
        copies: &[(ValueId, ValueId)],
        borrowed_copies: &Bindings,
        loop_after_blocks: &BTreeMap<BasicBlockId, Box<[MirInstruction]>>,
        selected_body_block: Option<BasicBlockId>,
    ) -> Result<Self, String> {
        Self::capture_with_phi_policy(
            function,
            bindings,
            copies,
            borrowed_copies,
            loop_after_blocks,
            selected_body_block,
        )
    }
}

pub(super) fn validate_recorded_phi(
    function: &MirFunction,
    id: BasicBlockId,
    block: &crate::mir::BasicBlock,
    loop_after_blocks: &BTreeMap<BasicBlockId, Box<[MirInstruction]>>,
    selected_body_block: Option<BasicBlockId>,
) -> Result<(), String> {
    let mut saw_non_phi = false;
    let mut phis = Vec::new();
    for instruction in &block.instructions {
        match instruction {
            MirInstruction::Phi { .. } if !saw_non_phi => phis.push(instruction),
            MirInstruction::Phi { .. } => return Err(fault("phi-in-recorded-block")),
            _ => saw_non_phi = true,
        }
    }
    if phis.is_empty() {
        return Ok(());
    }
    let predecessors: Vec<_> = function
        .blocks
        .values()
        .flat_map(|other| {
            other
                .out_edges()
                .into_iter()
                .map(move |edge| (other.id, edge))
        })
        .filter(|(_, edge)| edge.target == id)
        .map(|(pred, _)| pred)
        .collect();
    if predecessors.len() >= 2 {
        return Ok(());
    }
    if predecessors.len() != 1
        || (selected_body_block != Some(id) && !loop_after_blocks.contains_key(&id))
    {
        return Err(fault("phi-in-recorded-block"));
    }
    let predecessor = predecessors[0];
    if phis.iter().any(|instruction| {
        !matches!(instruction, MirInstruction::Phi { inputs, .. }
            if inputs.len() == 1 && inputs[0].0 == predecessor)
    }) {
        return Err(fault("phi-in-recorded-block"));
    }
    if let Some(expected) = loop_after_blocks.get(&id) {
        if phis.len() != expected.len()
            || phis
                .iter()
                .zip(expected.iter())
                .any(|(actual, sealed)| *actual != sealed)
        {
            return Err(fault("loop-after-phi-prefix-drift"));
        }
    }
    Ok(())
}
