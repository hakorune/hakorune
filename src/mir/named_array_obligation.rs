//! Passive detection and physical correspondence for source Named Array obligations.
//! Metadata is never constructor authority; only the retained source handoff can
//! discharge it. Clone/refresh must preserve the detection rows.
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteProducerKind, ArrayWriteSiteId, ConstructionTarget,
    MirFunction, MirInstruction, MirModule, ValueId,
};

/// Physical correspondence for the receiver's allocation. A same-function
/// `new ArrayBox()` binds by ValueId; a field-resident receiver binds to the
/// owning Box's `birth` provider recorded by source identity, never by name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayAllocationRefV1 {
    LocalValue(ValueId),
    FieldResidence {
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        provider_caller: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        provider_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NamedArrayWriteMarkerV1 {
    pub(crate) owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    pub(crate) allocation: NamedArrayAllocationRefV1,
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
    module: &MirModule,
    function: &MirFunction,
    marker: &NamedArrayWriteMarkerV1,
) -> Result<(), String> {
    let mut write_count = 0;
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if let MirInstruction::NewBox { dst, target, args } = instruction {
            if matches!(&marker.allocation, NamedArrayAllocationRefV1::LocalValue(allocation) if *dst == *allocation)
            {
                if !matches!(target, ConstructionTarget::Named(name) if name == "ArrayBox")
                    || !args.is_empty()
                {
                    return Err(fault("allocation-drift"));
                }
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
    if write_count != 1 {
        return Err(fault("write-cardinality"));
    }
    match &marker.allocation {
        NamedArrayAllocationRefV1::LocalValue(allocation) => {
            let mut allocation_count = 0;
            for instruction in function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
            {
                if let MirInstruction::NewBox { dst, target, args } = instruction {
                    if *dst == *allocation {
                        if !matches!(target, ConstructionTarget::Named(name) if name == "ArrayBox")
                            || !args.is_empty()
                        {
                            return Err(fault("allocation-drift"));
                        }
                        allocation_count += 1;
                    }
                }
            }
            if allocation_count != 1 {
                return Err(fault("allocation-cardinality"));
            }
        }
        NamedArrayAllocationRefV1::FieldResidence {
            field,
            provider_caller,
            provider_site,
        } => {
            let definition = module
                .canonical_field_definition(*field)
                .ok_or_else(|| fault("provider-field-foreign"))?;
            // A declared field type must be exactly ArrayBox. An untyped
            // `init` field is proven below by the provider's bare `ArrayBox`
            // `NewBox` target instead.
            if definition
                .declared_type_name
                .as_deref()
                .is_some_and(|ty| ty != "ArrayBox")
            {
                return Err(fault("provider-field-type"));
            }
            let provider_symbol = module
                .canonical_callable_definition_symbol(provider_caller)
                .ok_or_else(|| fault("provider-symbol-missing"))?;
            let provider = module
                .functions
                .get(provider_symbol)
                .ok_or_else(|| fault("provider-function-missing"))?;
            let allocation = provider
                .metadata
                .named_array_field_allocations
                .get(provider_site)
                .ok_or_else(|| fault("provider-allocation-missing"))?;
            let mut allocation_count = 0;
            for instruction in provider
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
            {
                if let MirInstruction::NewBox { dst, target, args } = instruction {
                    if dst == allocation {
                        if !matches!(target, ConstructionTarget::Named(name) if name == "ArrayBox")
                            || !args.is_empty()
                        {
                            return Err(fault("allocation-drift"));
                        }
                        allocation_count += 1;
                    }
                }
            }
            if allocation_count != 1 {
                return Err(fault("provider-allocation-cardinality"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "named_array_obligation_tests.rs"]
mod tests;
