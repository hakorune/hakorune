//! Structural Normal/Fault verification, independent of compatibility env flags.
//! Runtime admission remains closed until the common Fault ABI consumer lands.

use super::{cfg, dom, ssa, utils};
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};

#[path = "invoke_map.rs"]
mod map;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction};

/// Which admission question a caller asks of the cataloged call-edge
/// domain rule. `Sealed` enforces the Integer-domain corridor ABI —
/// the contract selected consumers transport. `Document` verifies
/// canonical structure only: document publication serializes the
/// same edges without claiming the corridor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CatalogedCallEdgePolicyV1 {
    Sealed,
    Document,
}

/// Declaration references survive publication; a backend must never recover
/// a missing field from a diagnostic name or the receiver's physical origin.
pub(super) fn check_module(
    module: &crate::mir::MirModule,
    edge_policy: CatalogedCallEdgePolicyV1,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for function in module.functions.values() {
        for (id, block) in &function.blocks {
            for instruction in block.all_instructions() {
                if let MirInstruction::ObjectFieldGet { field, .. } = instruction {
                    if !module
                        .canonical_field_definition(*field)
                        .is_some_and(|definition| {
                            !definition.is_weak
                                && definition.declared_type_name.as_deref() == Some("i64")
                        })
                    {
                        errors.push(error(*id, "object-field-read-definition-invalid"));
                    }
                }
                let call = match instruction {
                    MirInstruction::Call(call) => Some(call),
                    MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call, .. },
                        ..
                    } => Some(call),
                    _ => None,
                };
                if let Some(call) = call {
                    check_call_edge(module, function, *id, call, edge_policy, &mut errors);
                }
                if let MirInstruction::Invoke { operation, .. } = instruction {
                    match operation {
                        InvokeOperation::Map(MapInvokeOperation::InstallIndexed { object, .. })
                        | InvokeOperation::NewBox { object }
                        | InvokeOperation::HomeRelease { object, .. }
                        | InvokeOperation::ReclaimUnpublished { object, .. }
                            if module.canonical_object_definition(*object).is_none() =>
                        {
                            errors.push(error(*id, "object-definition-missing"));
                        }
                        InvokeOperation::Map(MapInvokeOperation::InstallIndexed { object, .. })
                        | InvokeOperation::HomeRelease { object, .. }
                            if !module.canonical_object_definition(*object).is_some_and(|definition|
                                definition.destruction_disposition()
                                    == crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook) =>
                        {
                            errors.push(error(*id, "home-destruction-unavailable"));
                        }
                        InvokeOperation::FieldSet { field, .. }
                            if module.canonical_field_definition(*field).is_none() =>
                        {
                            errors.push(error(*id, "field-definition-missing"));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

pub(super) fn check_function(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    check_function_inner(function, None)
}

/// The module-aware lane: cataloged call edges resolve against the module,
/// so a borrowed Map argument must prove its callee — never its shape alone.
pub(super) fn check_function_in_module(
    module: &crate::mir::MirModule,
    function: &MirFunction,
) -> Result<(), Vec<VerificationError>> {
    check_function_inner(function, Some(module))
}

fn check_function_inner(
    function: &MirFunction,
    module: Option<&crate::mir::MirModule>,
) -> Result<(), Vec<VerificationError>> {
    let has_control = function.blocks.values().any(|block| {
        block.all_instructions().any(|inst| {
            matches!(
                inst,
                MirInstruction::Invoke { .. }
                    | MirInstruction::InvokeNormalResult { .. }
                    | MirInstruction::ReturnFault { .. }
                    | MirInstruction::FaultFrameEnter { .. }
                    | MirInstruction::ArrayResidenceRelease { .. }
            )
        })
    });
    if !has_control {
        return Ok(());
    }
    let mut errors = Vec::new();
    check_frame_entry(function, &mut errors);
    for result in [
        ssa::check_ssa_form(function),
        cfg::check_control_flow(function),
    ] {
        if let Err(mut found) = result {
            errors.append(&mut found);
        }
    }
    // Compute actual edges, not a potentially stale predecessor cache.
    let mut predecessors = std::collections::BTreeMap::<_, Vec<_>>::new();
    for (id, block) in &function.blocks {
        for target in block.successors_from_terminator() {
            predecessors.entry(target).or_default().push(*id);
        }
    }
    for (id, block) in &function.blocks {
        for instruction in &block.instructions {
            if matches!(
                instruction,
                MirInstruction::Invoke { .. } | MirInstruction::ReturnFault { .. }
            ) {
                errors.push(error(*id, "control-in-instruction-list"));
            }
        }
        if let Some(MirInstruction::Invoke {
            operation,
            normal_landing,
            fault_landing,
            ..
        }) = &block.terminator
        {
            if normal_landing == fault_landing {
                errors.push(error(*id, "identical-landings"));
            }
            // Entry has an implicit incoming execution edge; CFG predecessors
            // alone cannot prove that its result storage was initialized.
            if *normal_landing == function.entry_block || normal_landing == id {
                errors.push(error(*id, "normal-landing-before-invocation"));
            }
            if let InvokeOperation::Call { call, result } = operation {
                if call.dst.is_some() {
                    errors.push(error(*id, "embedded-call-destination"));
                }
                use crate::mir::instruction::InvokeCallResultKind as ResultKind;
                use hakorune_mir_defs::{
                    CanonicalGlobalTargetV1 as Global,
                    CanonicalSameModuleGlobalTargetV1 as SameModule, SameModuleCallableNamespaceV1,
                };
                let valid = match (&call.callee, result) {
                    (Callee::BirthConstructor { .. }, ResultKind::Unit) => true,
                    (
                        Callee::Global(
                            target @ Global::SameModule(SameModule::StaticBoxMethod { .. }),
                        ),
                        ResultKind::I64 | ResultKind::Map,
                    ) => target
                        .arity()
                        .is_some_and(|arity| arity as usize == call.args.len()),
                    (
                        Callee::Global(
                            target @ Global::SameModule(SameModule::FreeFunction { .. }),
                        ),
                        ResultKind::I64 | ResultKind::Map,
                    ) => target
                        .arity()
                        .is_some_and(|arity| arity as usize == call.args.len()),
                    (Callee::SameModuleInstance { key, .. }, ResultKind::I64 | ResultKind::Map) => {
                        key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                            && key.arity() as usize == call.args.len()
                    }
                    _ => false,
                };
                if !valid {
                    errors.push(error(*id, "call-result-contract-not-connected"));
                }
            }
            let value_result = operation.normal_result_kind().is_some();
            let projections = function
                .blocks
                .values()
                .flat_map(|b| b.all_instructions())
                .filter(|inst| {
                    matches!(inst, MirInstruction::InvokeNormalResult { invoke_block, .. }
                    if invoke_block == id)
                })
                .count();
            if projections != usize::from(value_result) {
                errors.push(error(*id, "normal-result-count"));
            }
            if predecessors.get(normal_landing).map(Vec::as_slice) != Some(&[*id]) {
                errors.push(error(*id, "normal-landing-not-exclusive"));
            }
        }
        for (index, instruction) in block.all_instructions().enumerate() {
            let MirInstruction::InvokeNormalResult { invoke_block, .. } = instruction else {
                continue;
            };
            let valid_origin = function.blocks.get(invoke_block).is_some_and(|origin| {
                matches!(&origin.terminator, Some(MirInstruction::Invoke { normal_landing, .. })
                    if normal_landing == id)
            });
            if !valid_origin {
                errors.push(error(*id, "foreign-normal-result-origin"));
            }
            if index >= block.instructions.len()
                || block.instructions[..index]
                    .iter()
                    .any(|inst| !matches!(inst, MirInstruction::Phi { .. }))
            {
                errors.push(error(*id, "normal-result-not-first"));
            }
        }
    }
    // Never let verify_allow_no_phi make a Fault-edge result use admissible.
    let definitions = utils::compute_def_blocks(function);
    let dominators = utils::compute_dominators(function);
    if let Err(mut found) =
        dom::check_dominance_with_policy(function, &definitions, &dominators, false)
    {
        errors.append(&mut found);
    }
    if let Err(reason) = map::check(function, module) {
        errors.push(error(function.entry_block, reason));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The entry definition is the operand sort. Source MirType metadata cannot
/// promote an integer/handle into a frame, and frame residence cannot escape.
fn check_frame_entry(function: &MirFunction, errors: &mut Vec<VerificationError>) {
    let mut frames = Vec::new();
    for (id, block) in &function.blocks {
        for (index, inst) in block.all_instructions().enumerate() {
            if let MirInstruction::FaultFrameEnter { dst, .. } = inst {
                frames.push(*dst);
                if *id != function.entry_block
                    || index >= block.instructions.len()
                    || block.instructions[..index]
                        .iter()
                        .any(|i| !matches!(i, MirInstruction::Phi { .. }))
                {
                    errors.push(error(*id, "frame-entry-position"));
                }
                if function.params.contains(dst) || function.metadata.value_types.contains_key(dst)
                {
                    errors.push(error(*id, "frame-source-value"));
                }
            }
        }
        if block
            .successors_from_terminator()
            .contains(&function.entry_block)
        {
            errors.push(error(*id, "frame-entry-reentered"));
        }
    }
    if frames.len() != 1 {
        errors.push(error(function.entry_block, "frame-entry-count"));
        return;
    }
    let frame = frames[0];
    for (id, block) in &function.blocks {
        for inst in block.all_instructions() {
            let ordinary_uses = match inst {
                MirInstruction::Invoke {
                    operation,
                    fault_frame,
                    ..
                } => {
                    if *fault_frame != frame {
                        errors.push(error(*id, "foreign-frame-operand"));
                    }
                    operation.used_values()
                }
                MirInstruction::ReturnFault { fault_frame } => {
                    if *fault_frame != frame {
                        errors.push(error(*id, "foreign-frame-operand"));
                    }
                    Vec::new()
                }
                _ => inst.used_values(),
            };
            if ordinary_uses.contains(&frame) {
                errors.push(error(*id, "frame-escaped-as-source-value"));
            }
        }
        if block
            .return_env
            .as_ref()
            .is_some_and(|values| values.contains(&frame))
        {
            errors.push(error(*id, "frame-escaped-as-source-value"));
        }
    }
}

/// The same-module call edges this verifier corroborates argument kinds on:
/// static-box and free calls plus instance methods resolve to cataloged
/// callee keys, the second element counting formals the receiver already
/// fills. Constructors, dynamic values and foreign targets have no sealed
/// formal authority here — a borrowed Map lease must never ride them.
fn cataloged_edge_key(
    call: &crate::mir::definitions::MirCall,
) -> Option<(hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, usize)> {
    use crate::mir::ssot::callable_key::{ordinary_call_receiver, ordinary_callable_key};
    let key = ordinary_callable_key(&call.callee).ok()?;
    let receiver_filled = ordinary_call_receiver(&call.callee).ok()?.is_some() as usize;
    Some((key, receiver_filled))
}

/// Resolve the edge to its cataloged callee definition; `None` keeps the
/// call outside this relation's authority (see `check_call_edge`).
fn cataloged_call_target<'module>(
    module: &'module crate::mir::MirModule,
    call: &crate::mir::definitions::MirCall,
) -> Option<(&'module MirFunction, usize)> {
    let (key, receiver_params) = cataloged_edge_key(call)?;
    module
        .canonical_callable_definition_symbol(&key)
        .and_then(|symbol| module.functions.get(symbol))
        .map(|callee| (callee, receiver_params))
}

/// One scalar Call edge only carries i64 argument values. When the callee is
/// a cataloged same-module definition, its published signature is the param
/// authority: a non-scalar formal (for example a borrowed `MapBox` storage
/// pointer) has no admitted argument kind on this edge, and a recorded
/// argument type must prove `Integer` — or stay unrecorded, matching the
/// physical lane's i64 spelling — never a concrete non-i64 kind.
/// Uncataloged callees are outside this relation's authority.
fn check_call_edge(
    module: &crate::mir::MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    call: &crate::mir::definitions::MirCall,
    edge_policy: CatalogedCallEdgePolicyV1,
    errors: &mut Vec<VerificationError>,
) {
    let Some((callee, receiver_params)) = cataloged_call_target(module, call) else {
        return;
    };
    let params = &callee.signature.params;
    if call.args.len() + receiver_params != params.len() {
        errors.push(error(block, "call-argument-type-drift"));
        return;
    }
    // The Integer-domain argument rule binds the sealed corridor: a
    // selected consumer spells every argument `i64` and only the
    // admitted MapBox pair may leave that domain. A document caller
    // serializes the same edges without claiming that corridor, so it
    // verifies arity above and never enforces the domain below.
    if matches!(edge_policy, CatalogedCallEdgePolicyV1::Document) {
        return;
    }
    for (index, (argument, parameter)) in call
        .args
        .iter()
        .zip(params.iter().skip(receiver_params))
        .enumerate()
    {
        let named_map = matches!(parameter, crate::mir::MirType::Box(name) if name == "MapBox");
        // Where signature-aligned carriers were issued, either the
        // `CheckedMapStorage` carrier or the `Box("MapBox")` name asserting a
        // map formal rejects the scalar edge — a disagreement is drift, not a
        // clean i64 formal.  Carrier-less callees keep the name contract.
        let map_formal = match callee.metadata.physical_param_carriers.as_deref() {
            Some(carriers) => {
                carriers.get(index + receiver_params).copied()
                    == Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::CheckedMapStorage)
                    || named_map
            }
            None => named_map,
        };
        // The bounded handoff admits exactly the corroborated
        // (`Box("MapBox")` actual, map formal) pair — the caller-owned
        // checked-map storage borrowed across the call. A map claim on one
        // side alone is ABI drift; every other pair stays scalar, where an
        // absent record keeps the untyped admission and a concrete non-i64
        // record rejects.
        let map_actual = matches!(
            function.metadata.value_types.get(argument),
            Some(crate::mir::MirType::Box(name)) if name == "MapBox"
        );
        let scalar_drift = matches!(
            function.metadata.value_types.get(argument),
            Some(kind) if !matches!(
                kind,
                crate::mir::MirType::Integer | crate::mir::MirType::Unknown
            ) && !map_actual
        );
        if (map_formal != map_actual) || (!map_formal && scalar_drift) {
            errors.push(error(block, "call-argument-type-drift"));
            return;
        }
    }
}

fn error(block: BasicBlockId, reason: &str) -> VerificationError {
    VerificationError::ControlFlowError {
        block,
        reason: format!("[freeze:contract][mir/invoke/{reason}]"),
    }
}

#[cfg(test)]
#[path = "invoke_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "invoke_array_tests.rs"]
mod array_tests;

#[cfg(test)]
#[path = "invoke_map_tests.rs"]
mod map_tests;

#[cfg(test)]
#[path = "invoke_map_view_tests.rs"]
mod map_view_tests;

#[cfg(test)]
#[path = "invoke_call_tests.rs"]
mod call_tests;
