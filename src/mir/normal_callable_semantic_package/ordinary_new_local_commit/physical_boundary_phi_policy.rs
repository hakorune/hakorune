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

impl PhysicalBoundary {
    pub(in super::super) fn capture_with_source_loop_roles_v2(
        function: &MirFunction, bindings: &Bindings, copies: &[(ValueId, ValueId)], borrowed_copies: &Bindings,
        phi_roles: &SourcePhiRoles, selected_body: Option<BasicBlockId>,
        roles: &[std::rc::Rc<crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1>],
    ) -> Result<Self, String> {
        Self::capture_with_phi_policy_and_source_loops(function, bindings, copies, borrowed_copies, phi_roles, selected_body, roles)
    }
}
pub(super) fn source_loop_feedback_v2(function: &MirFunction, nodes: &BTreeMap<BasicBlockId, Node>,
    roles: &[std::rc::Rc<crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1>],
) -> Result<BTreeSet<(BasicBlockId, BasicBlockId)>, String> {
    let mut feedback = BTreeSet::new();
    for role in roles {
        let Some(proof) = role.source_proof_v2() else { continue };
        if role.owner() != proof.product().source.source.owner() || role.site() != proof.product().source.source.site()
            || role.loop_phis().get(&proof.header()).is_none_or(|rows| rows.is_empty()) {
            return Err(fault("source-loop-feedback-role"));
        }
        for (from, to) in proof.feedback_edges_v2() {
            // A partial boundary does not cut an edge beyond its own scope.
            if !nodes.contains_key(&from) || !nodes.contains_key(&to) { continue; }
            let expected = MirInstruction::Jump { target: to, edge_args: None };
            if to != proof.header() || nodes[&from].terminal != expected
                || function.blocks.get(&from).and_then(|block| block.terminator.as_ref()) != Some(&expected)
                || !feedback.insert((from, to)) { return Err(fault("source-loop-feedback-edge")); }
        }
    }
    Ok(feedback)
}
pub(super) fn is_source_loop_auxiliary_const_v2(
    roles: &[std::rc::Rc<crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1>], function: &MirFunction,
    id: BasicBlockId, row: &MirInstruction,
) -> bool {
    let MirInstruction::Const { dst, value: crate::mir::ConstValue::Void } = row else { return false };
    !function.params.contains(dst) && function.blocks.values().all(|block|
        block.all_instructions().all(|row| !row.used_values().contains(dst))
            && block.return_env.as_ref().is_none_or(|values| !values.contains(dst)))
        && roles.iter().filter_map(|role| role.source_proof_v2())
            .any(|proof| proof.result_binding() == &(id, row.clone()))
}
