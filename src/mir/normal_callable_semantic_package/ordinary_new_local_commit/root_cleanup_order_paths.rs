//! Independently walk MIR paths against the SAME retained source obligation order.
//! Recorded MIR cannot shorten the expected Normal or Fault sequences.
use super::super::physical_boundary::FinishedBindings;
use super::super::root_home::{RootHomeCleanupOrderV1, RootHomeExitEntry};
use super::*;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate(
    function: &MirFunction,
    bindings: &Bindings,
    entry: &RootHomeExitEntry,
    order: &RootHomeCleanupOrderV1,
    projection: Option<&FinishedBindings>,
) -> Result<(), String> {
    let coordinate = |id| match projection {
        Some(p) => p
            .destination(id)
            .ok_or_else(|| fault("ordered-path/missing-projection")),
        None => Ok(id),
    };
    let binding = |row: &(BasicBlockId, MirInstruction)| match projection {
        Some(p) => p
            .binding(row.0, &row.1)?
            .ok_or_else(|| fault("ordered-path/missing-binding")),
        None => Ok(row.clone()),
    };
    let (mut clean, acquisition, frame, result) = match entry {
        RootHomeExitEntry::Call {
            invoke,
            projection: result_projection,
            ..
        }
        | RootHomeExitEntry::MapGet {
            invoke,
            projection: result_projection,
            ..
        } => {
            let (_, invoke) = binding(invoke)?;
            let (_, projected) = binding(result_projection)?;
            let MirInstruction::Invoke {
                normal_landing,
                fault_landing,
                fault_frame,
                operation,
            } = invoke
            else {
                return Err(fault("ordered-path/ingress"));
            };
            let kind = order.ingress_result_kind();
            match operation {
                InvokeOperation::Call { result, call } if result == kind && call.dst.is_none() => {}
                InvokeOperation::Map(
                    crate::mir::instruction::MapInvokeOperation::CheckedGetI64 { .. },
                ) if kind == crate::mir::instruction::InvokeCallResultKind::I64 => {}
                InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { field, object, .. })
                    if matches!(kind, crate::mir::instruction::InvokeCallResultKind::BorrowedNullableHandle {
                        support: crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { field: original, object: payload, .. }
                    } if original == field && payload == object) => {}
                _ => {
                    if crate::config::env::builder_debug_enabled() {
                        let entry_kind = if matches!(entry, RootHomeExitEntry::Call { .. }) { "Call" } else { "MapGet" };
                        eprintln!("[root-cleanup/ingress] function={} entry={entry_kind} expected={kind:?} operation={operation:?}", function.signature.name);
                    }
                    return Err(fault("ordered-path/ingress-kind"));
                },
            }
            let MirInstruction::InvokeNormalResult { dst, .. } = projected else {
                return Err(fault("ordered-path/result"));
            };
            (
                normal_landing,
                Some(fault_landing),
                Some(fault_frame),
                Some(dst),
            )
        }
        RootHomeExitEntry::Plain { .. } => {
            let start = bindings
                .last()
                .ok_or_else(|| fault("ordered-path/entry"))?
                .0;
            let result = bindings
                .iter()
                .find_map(|(_, i)| match i {
                    MirInstruction::Return { value } => Some(*value),
                    _ => None,
                })
                .ok_or_else(|| fault("ordered-path/return"))?;
            let frame = bindings.iter().find_map(|(_, i)| match i {
                MirInstruction::Invoke { fault_frame, .. } => Some(*fault_frame),
                _ => None,
            });
            (coordinate(start)?, None, frame, result)
        }
    };
    if let Some(acquisition) = acquisition {
        pending(function, acquisition, &order.acquisition_fault(), frame)?;
    }
    let normal: Vec<_> = order.normal().iter().map(|origin| origin.operation().clone()).collect();
    let residuals = (0..normal.len()).map(|step| order.fault_after_normal_step(step)
        .map(|rows| rows.into_iter().map(|origin| origin.operation().clone()).collect()))
        .collect::<Result<Vec<Vec<_>>, _>>()?;
    clean = normal_steps_with_resolver_v2(function, clean, &normal, &residuals, frame, None,
        &mut |_, operation| Ok(operation.clone()))?;
    clean = skip(function, clean)?;
    if terminal(function, clean)? != &(MirInstruction::Return { value: result }) {
        return Err(fault("ordered-path/normal-terminal"));
    }
    Ok(())
}
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn terminal(function: &MirFunction, id: BasicBlockId) -> Result<&MirInstruction, String> {
    match function.blocks.get(&id).and_then(|block| block.terminator.as_ref()) {
        Some(terminal) => Ok(terminal),
        None => {
            if crate::config::env::builder_debug_enabled() {
                eprintln!("[ordered-path-missing-node] function={} block={id:?} present={} instructions={:?}",
                    function.signature.name, function.blocks.contains_key(&id),
                    function.blocks.get(&id).map(|block| &block.instructions));
            }
            Err(fault("ordered-path/missing-node"))
        }
    }
}
fn skip(function: &MirFunction, id: BasicBlockId) -> Result<BasicBlockId, String> {
    skip_until(function, id, None)
}
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn skip_until(
    function: &MirFunction, mut id: BasicBlockId, stop: Option<BasicBlockId>,
) -> Result<BasicBlockId, String> {
    let mut visited = BTreeSet::new();
    loop {
        if Some(id) == stop { return Ok(id); }
        if !visited.insert(id) {
            return Err(fault("ordered-path/cycle"));
        }
        match terminal(function, id)? {
            MirInstruction::Jump {
                target,
                edge_args: None,
            } => id = *target,
            _ => return Ok(id),
        }
    }
}
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn release(
    function: &MirFunction,
    id: BasicBlockId,
    expected: &InvokeOperation,
    frame: Option<crate::mir::ValueId>,
) -> Result<(BasicBlockId, BasicBlockId), String> {
    match terminal(function, id)? {
        MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        } if operation == expected && Some(*fault_frame) == frame => {
            Ok((*normal_landing, *fault_landing))
        }
        _ => Err(fault("ordered-path/operation")),
    }
}
fn pending(
    function: &MirFunction,
    id: BasicBlockId,
    expected: &[super::super::root_home::RootHomeReleaseOriginV1],
    frame: Option<crate::mir::ValueId>,
) -> Result<(), String> {
    pending_operations(function, id, &expected.iter().map(|origin| origin.operation().clone()).collect::<Vec<_>>(), frame).map(drop)
}

/// Reused by body-expression prefixes, whose source includes String End before
/// the same prior-Home unwind. Expected operations never come from MIR census.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn pending_operations(
    function: &MirFunction, id: BasicBlockId, expected: &[InvokeOperation],
    frame: Option<crate::mir::ValueId>,
) -> Result<BTreeSet<BasicBlockId>, String> {
    pending_operations_with_resolver_v2(function, id, expected, frame, None, &mut |_, operation| Ok(operation.clone()))
}

fn pending_operations_with_resolver_v2(
    function: &MirFunction, mut id: BasicBlockId, expected: &[InvokeOperation],
    frame: Option<crate::mir::ValueId>, stop: Option<BasicBlockId>,
    resolve: &mut impl FnMut(BasicBlockId, &InvokeOperation) -> Result<InvokeOperation, String>,
) -> Result<BTreeSet<BasicBlockId>, String> {
    let mut consumed = BTreeSet::new();
    for operation in expected {
        id = skip_until(function, id, stop)?;
        if Some(id) == stop { return Err(fault("ordered-path/fault-continuation")); }
        consumed.insert(id);
        let expected = resolve(id, operation)?;
        let (normal, fault_edge) = release(function, id, &expected, frame)?;
        let normal = skip_until(function, normal, stop)?;
        let fault_edge = skip_until(function, fault_edge, stop)?;
        if Some(normal) == stop || Some(fault_edge) == stop { return Err(fault("ordered-path/fault-continuation")); }
        if normal != fault_edge {
            return Err(fault("ordered-path/fault-outcomes"));
        }
        id = normal;
    }
    id = skip_until(function, id, stop)?;
    if Some(id) == stop { return Err(fault("ordered-path/fault-continuation")); }
    if !matches!(terminal(function, id)?, MirInstruction::ReturnFault { fault_frame } if Some(*fault_frame) == frame)
    {
        return Err(fault("ordered-path/fault-terminal"));
    }
    Ok(consumed)
}

/// One Normal/Fault walk for root exits and infallible Loop continuations.
/// The callback lends an original operation's exact per-origin SSA projection;
/// it never selects the expected source operation by inspecting MIR.
fn normal_steps_with_resolver_v2(
    function: &MirFunction, mut clean: BasicBlockId, normal: &[InvokeOperation],
    residuals: &[Vec<InvokeOperation>], frame: Option<crate::mir::ValueId>, stop: Option<BasicBlockId>,
    resolve: &mut impl FnMut(BasicBlockId, &InvokeOperation) -> Result<InvokeOperation, String>,
) -> Result<BasicBlockId, String> {
    if normal.len() != residuals.len() { return Err(fault("ordered-path/residual-census")); }
    for (expected, residual) in normal.iter().zip(residuals) {
        clean = skip_until(function, clean, stop)?;
        if Some(clean) == stop { return Err(fault("ordered-path/early-continuation")); }
        let operation = resolve(clean, expected)?;
        let (next, fault_edge) = release(function, clean, &operation, frame)?;
        pending_operations_with_resolver_v2(function, fault_edge, residual, frame, stop, resolve)?;
        clean = next;
    }
    Ok(clean)
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_to_continuation_v2(
    function: &MirFunction, entry: BasicBlockId, continuation: BasicBlockId,
    normal: &[InvokeOperation], residuals: &[Vec<InvokeOperation>],
    frame: Option<crate::mir::ValueId>,
    resolve: &mut impl FnMut(BasicBlockId, &InvokeOperation) -> Result<InvokeOperation, String>,
) -> Result<(), String> {
    let clean = normal_steps_with_resolver_v2(function, entry, normal, residuals, frame, Some(continuation), resolve)?;
    if skip_until(function, clean, Some(continuation))? != continuation {
        return Err(fault("ordered-path/normal-continuation"));
    }
    Ok(())
}
