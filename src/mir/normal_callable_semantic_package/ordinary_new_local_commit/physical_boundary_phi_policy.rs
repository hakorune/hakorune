//! PHIs in recorded lifecycle blocks need a genuine join or one exact
//! canonical Loop role. A raw one-predecessor block is never permission.
use super::*;

// Existing source prefix loan, with an exact predecessor only for a call's
// checked Normal landing. LoopCond after joins keep their original topology.
pub(super) type SourcePhiRoles =
    BTreeMap<BasicBlockId, (Box<[MirInstruction]>, Option<BasicBlockId>)>;

impl PhysicalBoundary {
    pub(in super::super) fn capture_with_source_phi_roles(
        function: &MirFunction,
        bindings: &Bindings,
        copies: &[(ValueId, ValueId)],
        borrowed_copies: &Bindings,
        source_phi_roles: &SourcePhiRoles,
        selected_body_block: Option<BasicBlockId>,
    ) -> Result<Self, String> {
        Self::capture_with_phi_policy(
            function,
            bindings,
            copies,
            borrowed_copies,
            source_phi_roles,
            selected_body_block,
        )
    }
}

pub(super) fn validate_recorded_phi(
    function: &MirFunction,
    id: BasicBlockId,
    block: &crate::mir::BasicBlock,
    source_phi_roles: &SourcePhiRoles,
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
    if let Some((_, Some(expected))) = source_phi_roles.get(&id) {
        if predecessors.as_slice() != [*expected] {
            return Err(fault("phi-in-recorded-block"));
        }
    }
    if !phis.is_empty() && predecessors.len() < 2 {
        if predecessors.len() != 1
            || (selected_body_block != Some(id) && !source_phi_roles.contains_key(&id))
        {
            return Err(fault("phi-in-recorded-block"));
        }
        let predecessor = predecessors[0];
        if phis.iter().any(|instruction| {
            !matches!(instruction,
            MirInstruction::Phi { inputs, .. } if inputs.len() == 1 && inputs[0].0 == predecessor)
        }) {
            return Err(fault("phi-in-recorded-block"));
        }
    }
    if let Some((expected, _)) = source_phi_roles.get(&id) {
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
