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
                ..
            } = invoke
            else {
                return Err(fault("ordered-path/ingress"));
            };
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
    for (index, expected) in order.normal().iter().enumerate() {
        clean = skip(function, clean)?;
        let (normal, fault_edge) = release(function, clean, expected.operation(), frame)?;
        pending(
            function,
            fault_edge,
            &order.fault_after_normal_step(index)?,
            frame,
        )?;
        clean = normal;
    }
    clean = skip(function, clean)?;
    if terminal(function, clean)? != &(MirInstruction::Return { value: result }) {
        return Err(fault("ordered-path/normal-terminal"));
    }
    Ok(())
}
fn terminal(function: &MirFunction, id: BasicBlockId) -> Result<&MirInstruction, String> {
    function
        .blocks
        .get(&id)
        .and_then(|b| b.terminator.as_ref())
        .ok_or_else(|| fault("ordered-path/missing-node"))
}
fn skip(function: &MirFunction, mut id: BasicBlockId) -> Result<BasicBlockId, String> {
    let mut visited = BTreeSet::new();
    loop {
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
fn release(
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
    mut id: BasicBlockId,
    expected: &[super::super::root_home::RootHomeReleaseOriginV1],
    frame: Option<crate::mir::ValueId>,
) -> Result<(), String> {
    for origin in expected {
        id = skip(function, id)?;
        let (normal, fault_edge) = release(function, id, origin.operation(), frame)?;
        if skip(function, normal)? != skip(function, fault_edge)? {
            return Err(fault("ordered-path/fault-outcomes"));
        }
        id = normal;
    }
    id = skip(function, id)?;
    if !matches!(terminal(function, id)?, MirInstruction::ReturnFault { fault_frame } if Some(*fault_frame) == frame)
    {
        return Err(fault("ordered-path/fault-terminal"));
    }
    Ok(())
}
