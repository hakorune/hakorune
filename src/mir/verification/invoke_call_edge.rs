//! Cataloged Call-edge ABI correspondence over original tagged and take slots.
use super::{borrowed_calls, error, CatalogedCallEdgePolicyV1};
use crate::mir::{BasicBlockId, MirFunction};
use crate::mir::verification_types::VerificationError;

/// The same-module call edges this verifier corroborates argument kinds on:
/// static-box and free calls plus instance methods resolve to cataloged
/// callee keys, the second element counting formals the receiver already
/// fills. Constructors, dynamic values and foreign targets have no sealed
/// formal authority here — a borrowed Map lease must never ride them.
pub(super) fn cataloged_edge_key(
    call: &crate::mir::definitions::MirCall,
) -> Option<(hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, usize)> {
    use crate::mir::ssot::callable_key::{ordinary_call_receiver, ordinary_callable_key};
    let key = ordinary_callable_key(&call.callee).ok()?;
    let receiver_filled = ordinary_call_receiver(&call.callee).ok()?.is_some() as usize;
    Some((key, receiver_filled))
}

/// Resolve the edge to its cataloged callee definition; `None` keeps the
/// call outside this relation's authority (see `check_call_edge`).
pub(super) fn cataloged_call_target<'module>(
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
pub(super) fn check_call_edge(
    module: &crate::mir::MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    call: &crate::mir::definitions::MirCall,
    edge_policy: CatalogedCallEdgePolicyV1,
    borrowed_slots: Option<&std::collections::BTreeSet<usize>>,
    take_slots: Option<&std::collections::BTreeMap<usize, (crate::mir::ValueId, hakorune_mir_defs::CanonicalObjectIdV1)>>,
    source_bound: bool,
    receiver_callees: &std::collections::BTreeSet<String>,
    errors: &mut Vec<VerificationError>,
) {
    let Some((callee, receiver_params)) = cataloged_call_target(module, call) else {
        return;
    };
    if let Err(reason) = borrowed_calls::require_receiver_source(source_bound, &callee.signature.name, receiver_callees, borrowed_slots) {
        errors.push(error(block, reason));
        return;
    }
    let params = &callee.signature.params;
    if call.args.len() + receiver_params != params.len() {
        errors.push(call_argument_drift(
            function,
            callee,
            block,
            format!(
                "arity: actual={} receiver={} formal={}",
                call.args.len(),
                receiver_params,
                params.len()
            ),
        ));
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
        for kind in [crate::mir::MirType::Integer, crate::mir::MirType::Bool] {
            if super::string::has_text_read_scalar_origin(function, *argument, &kind)
                && parameter != &kind && parameter != &crate::mir::MirType::Unknown {
                let actual = if kind == crate::mir::MirType::Integer { "StringLength(I64)" } else { "TextRead(Bool)" };
                errors.push(call_argument_drift(function, callee, block,
                    format!("argument={index} value={argument:?} actual={actual} formal={parameter:?}")));
                return;
            }
        }
        if let Some((expected, object)) = take_slots.and_then(|slots| slots.get(&index)) {
            if !source_bound || expected != argument || module.canonical_object_definition(*object).is_none()
                || !matches!(parameter, crate::mir::MirType::Box(_))
                || callee.metadata.physical_param_carriers.as_deref().and_then(|rows| rows.get(index + receiver_params))
                    != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::ExistingCallableI64)
                || borrowed_slots.is_some_and(|slots| slots.contains(&index)) {
                errors.push(error(block, "borrowed-source-call-coverage"));
                return;
            }
            continue;
        }
        let tagged_formal = callee.metadata.physical_param_carriers.as_deref()
            .and_then(|rows| rows.get(index + receiver_params))
            == Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue);
        if source_bound
            && tagged_formal != borrowed_slots.is_some_and(|slots| slots.contains(&index))
        {
            errors.push(error(block, "borrowed-source-call-coverage"));
            return;
        }
        if borrowed_slots.is_some_and(|slots| slots.contains(&index)) {
            // Only the exact original entry/Call loan admits a tagged actual.
            // Full producer/use and physical publication checks remain mandatory.
            continue;
        }
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
            errors.push(call_argument_drift(
                function,
                callee,
                block,
                format!(
                "argument={index} value={argument:?} actual={:?} formal={parameter:?} carrier={:?}",
                function.metadata.value_types.get(argument),
                callee.metadata.physical_param_carriers.as_deref()
                    .and_then(|rows| rows.get(index + receiver_params))
            ),
            ));
            return;
        }
    }
}

/// A scalar (`I64`, `Bool` or `Unit`) Call result must be what the cataloged callee's published signature
/// declares. The caller's Invoke only names the kind; the callee definition is
/// the independent authority, so a result kind that disagrees with the callee's
/// declared return type is drift (the same edge the C shim corroborates).
pub(super) fn check_call_result(
    module: &crate::mir::MirModule,
    function: &MirFunction,
    block: BasicBlockId,
    call: &crate::mir::definitions::MirCall,
    result: crate::mir::instruction::InvokeCallResultKind,
    edge_policy: CatalogedCallEdgePolicyV1,
    errors: &mut Vec<VerificationError>,
) {
    use crate::mir::instruction::InvokeCallResultKind as Kind;
    // Only the scalar kinds name a return type the callee must declare; owned
    // string, carrier and Map results carry their own lifecycle contracts.
    let declared = match result {
        Kind::I64 => crate::mir::MirType::Integer,
        Kind::Bool => crate::mir::MirType::Bool,
        Kind::Unit => crate::mir::MirType::Void,
        _ => return,
    };
    if matches!(edge_policy, CatalogedCallEdgePolicyV1::Document) {
        return;
    }
    let Some((callee, _)) = cataloged_call_target(module, call) else {
        return;
    };
    if callee.signature.return_type != declared {
        errors.push(VerificationError::ControlFlowError {
            block,
            reason: format!(
                "[freeze:contract][mir/invoke/call-result-type-drift] caller={} callee={} result={result:?} declared={:?}",
                function.signature.name, callee.signature.name, callee.signature.return_type
            ),
        });
    }
}

fn call_argument_drift(
    caller: &MirFunction,
    callee: &MirFunction,
    block: BasicBlockId,
    detail: String,
) -> VerificationError {
    VerificationError::ControlFlowError {
        block,
        reason: format!(
            "[freeze:contract][mir/invoke/call-argument-type-drift] caller={} callee={} {detail}",
            caller.signature.name, callee.signature.name
        ),
    }
}
