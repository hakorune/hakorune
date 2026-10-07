//! Compare two recorded Call cleanup ingresses against the same ordered Homes.
//! Coordinates are physical observations, not a new continuation authority.

use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::ValueId;

/// Called with existing FinishedBindings projections after contraction.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn ingress(
    function: &MirFunction,
    bindings: &Bindings,
    invoke: &(BasicBlockId, MirInstruction),
    projection: &(BasicBlockId, MirInstruction),
) -> Result<(BasicBlockId, BasicBlockId, ValueId, ValueId), String> {
    let (fault_frame, normal_landing, fault_landing) = match &invoke.1 {
        MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
            fault_frame,
            normal_landing,
            fault_landing,
        } if call.dst.is_none() => (*fault_frame, *normal_landing, *fault_landing),
        // The readable-Map terminal shares the exact ingress graph: one
        // checked read invoke, Normal result projection, Fault cleanup.
        MirInstruction::Invoke {
            operation:
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::CheckedGetI64 {
                    ..
                }),
            fault_frame,
            normal_landing,
            fault_landing,
        } => (*fault_frame, *normal_landing, *fault_landing),
        _ => return Err(fault("call-ingress")),
    };
    let MirInstruction::InvokeNormalResult {
        dst,
        invoke_block: origin,
    } = &projection.1
    else {
        return Err(fault("call-projection"));
    };
    let nodes: BTreeSet<_> = bindings.iter().map(|(id, _)| *id).collect();
    if *origin != invoke.0
        || projection.0 != normal_landing
        || normal_landing == fault_landing
        || nodes.contains(&invoke.0)
        || !nodes.contains(&normal_landing)
        || !nodes.contains(&fault_landing)
        || nodes.contains(&function.entry_block)
        || function
            .blocks
            .get(&invoke.0)
            .and_then(|b| b.terminator.as_ref())
            != Some(&invoke.1)
    {
        return Err(fault("call-ingress"));
    }
    let mut incoming = BTreeSet::new();
    for (id, block) in &function.blocks {
        if nodes.contains(id) {
            continue;
        }
        for (slot, edge) in block.out_edges().into_iter().enumerate() {
            if !nodes.contains(&edge.target) {
                continue;
            }
            if *id != invoke.0
                || edge.args.is_some()
                || !matches!((slot, edge.target), (0, target) if target == normal_landing)
                    && !matches!((slot, edge.target), (1, target) if target == fault_landing)
            {
                return Err(fault("call-foreign-incoming"));
            }
            incoming.insert(slot);
        }
    }
    if incoming != BTreeSet::from([0, 1]) {
        return Err(fault("call-missing-incoming"));
    }
    Ok((normal_landing, fault_landing, fault_frame, *dst))
}
