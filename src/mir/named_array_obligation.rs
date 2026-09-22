//! Passive detection and physical correspondence for source Named Array obligations.
//! Metadata is never constructor authority; only the retained source handoff can
//! discharge it. Clone/refresh must preserve the detection rows.
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, ConstructionTarget,
    MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NamedArrayWriteMarkerV1 {
    pub(crate) owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(crate) allocation: ValueId,
    pub(crate) receiver: ValueId,
    pub(crate) argument: ValueId,
    pub(crate) write: ArrayWriteSiteId,
}

pub(crate) fn reject_unretained_module(module: &MirModule) -> Result<(), String> {
    if module
        .functions
        .values()
        .any(|function| !function.metadata.named_array_write_obligations.is_empty())
    {
        return Err(fault("retained-source-required"));
    }
    Ok(())
}

pub(crate) fn fault(reason: &str) -> String {
    format!("[freeze:contract][named-array/{reason}]")
}

/// Validate emitted identity only. This never issues source or backend authority.
pub(crate) fn validate_physical_marker(
    function: &MirFunction,
    marker: &NamedArrayWriteMarkerV1,
) -> Result<(), String> {
    let mut allocation_count = 0;
    let mut write_count = 0;
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if let MirInstruction::NewBox { dst, target, args } = instruction {
            if *dst == marker.allocation {
                if !matches!(target, ConstructionTarget::Named(name) if name == "ArrayBox")
                    || !args.is_empty()
                {
                    return Err(fault("allocation-drift"));
                }
                allocation_count += 1;
            }
        }
        if matches!(instruction, MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::ArrayElementWrite { site_id, .. }, ..
        } if *site_id == marker.write)
        {
            return Err(fault("checked-write-contract-unavailable"));
        }
        if let MirInstruction::ArrayElementWrite {
            site_id,
            dst,
            kind,
            producer,
            receiver,
            index,
            value,
        } = instruction
        {
            if *site_id == marker.write {
                if dst.is_some()
                    || *kind != ArrayElementWriteKind::Push
                    || *producer != ArrayWriteProducerKind::MethodCall
                    || index.is_some()
                    || *receiver != marker.receiver
                    || *value != marker.argument
                {
                    return Err(fault("write-drift"));
                }
                write_count += 1;
            }
        }
    }
    if allocation_count != 1 {
        return Err(fault("allocation-cardinality"));
    }
    if write_count != 1 {
        return Err(fault("write-cardinality"));
    }
    Ok(())
}

#[cfg(test)]
#[path = "named_array_obligation_tests.rs"]
mod tests;
