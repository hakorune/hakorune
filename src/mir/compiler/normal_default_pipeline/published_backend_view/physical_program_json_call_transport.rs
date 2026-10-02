//! Closed parameter and Call transport. Source proofs stay with their existing
//! owners; this encoder never infers an opaque formal from payload bits.
use super::*;

pub(super) fn encode_parameters(
    function: &PublishedLifecyclePhysicalFunctionV1<'_>,
) -> Result<Vec<Value>, String> {
    function.params().iter().skip(usize::from(function.role().has_receiver()))
                    .zip(function.param_types().iter().skip(usize::from(function.role().has_receiver())))
                    .enumerate()
                    .map(|(index, (param, param_type))| {
                        // The signature-issued carrier is the representation
                        // authority: `CheckedMapStorage` alone spells `map`.
                        // The `Box("MapBox")` name corroborates the carrier —
                        // either side asserting without the other is drift.
                        let carrier = function
                            .param_carriers()
                            .and_then(|carriers| {
                                carriers.get(index + usize::from(function.role().has_receiver()))
                            })
                            .copied();
                        let named_map = matches!(
                            param_type,
                            crate::mir::MirType::Box(name) if name == "MapBox"
                        );
                        let representation = match (carrier, named_map) {
                            (Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue), _) => {
                                return Err(fault("borrowed-carrier-activation-missing"));
                            }
                            (Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::CheckedMapStorage), true) => {
                                // Borrowed checked-map storage pointer, not an i64 payload.
                                "map"
                            }
                            (Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::CheckedMapStorage), false)
                            | (_, true) => {
                                return Err(fault("param-carrier-drift"));
                            }
                            _ if function.role().ordinary_target().is_some() => "i64",
                            _ => "kind_payload_v1",
                        };
                        Ok(json!({
                            "value": value(param),
                            "representation": representation,
                        }))
                    })
                    .collect::<Result<Vec<_>, String>>()
}

pub(super) fn encode_ordinary_call(
    call: &crate::mir::definitions::MirCall,
    result_kind: InvokeCallResultKind,
    ordinary: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    call_context: Option<CallContext<'_>>,
) -> Result<Value, String> {
    let key = super::super::physical_program::ordinary_callable_key(&call.callee)?;
    let target = ordinary
        .get(&key)
        .ok_or_else(|| fault("ordinary-target-missing"))?;
    if call.dst.is_some() {
        return Err(fault("ordinary-destination"));
    }
    // Corroborated actual/formal per ordinal: the caller's sealed
    // `value_types` row is the only actual authority, and the
    // callee's signature-installed carrier is the only formal
    // authority. The bounded handoff admits exactly the
    // (`Box("MapBox")` actual, `CheckedMapStorage` formal) pair as
    // `"map"`; everything else stays `"i64"`, and a map actual or
    // map formal without its counterpart is ABI drift.
    let (functions, caller) = call_context.ok_or_else(|| fault("ordinary-call-context"))?;
    let callee = functions
        .get(*target as usize)
        .ok_or_else(|| fault("ordinary-target-missing"))?;
    let args = call
        .args
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if matches!(
        callee.param_carriers().and_then(|carriers| carriers.get(index)),
        Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
            ) {
        return Err(fault("borrowed-carrier-activation-missing"));
            }
            let map_actual = matches!(
        caller.value_types().get(value),
        Some(crate::mir::MirType::Box(name)) if name == "MapBox"
            );
            let map_formal = matches!(
        callee.param_carriers().and_then(|carriers| carriers.get(index)),
        Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::CheckedMapStorage)
            );
            if map_actual != map_formal {
        return Err(fault("ordinary-argument-kind"));
            }
            Ok(json!({
        "kind": if map_actual { "map" } else { "i64" }, "value": value.0,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let call = match super::super::physical_program::ordinary_call_receiver(&call.callee)? {
        Some(receiver) => json!({
            "target": target,
            "receiver": value(&receiver),
            "args": args,
            "dst": Value::Null,
        }),
        None => json!({
            "target": target,
            "args": args,
            "dst": Value::Null,
        }),
    };
    Ok(json!({
        "kind": "ordinary_call",
        "call": call,
        "result": match result_kind {
            InvokeCallResultKind::Map => "map",
            InvokeCallResultKind::Handle => "handle",
            InvokeCallResultKind::NullableHandle => "nullable_handle",
            _ => "i64",
        },
    }))
}

pub(super) fn encode_birth_call(
    call: &crate::mir::definitions::MirCall,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    let Callee::BirthConstructor { key, receiver } = &call.callee else {
        return Err(fault("call-not-birth"));
    };
    let target = births
        .get(key)
        .ok_or_else(|| fault("birth-target-foreign"))?;
    let input = abi_input.ok_or_else(|| fault("birth-input-missing"))?;
    let mut matches = input.entry().birth_calls().iter().filter(|issued| {
        issued.caller_function_index() == caller_function_index
            && issued.function_index() == *target
            && issued.receiver() == *receiver
            && issued.arguments().eq(call.args.iter().copied())
    });
    let issued = matches
        .next()
        .ok_or_else(|| fault("birth-actual-missing"))?;
    if matches.next().is_some() {
        return Err(fault("birth-actual-duplicate"));
    }
    let args = issued
        .actual()
        .arguments()
        .iter()
        .map(|argument| {
            Ok(
                json!({ "kind": super::super::physical_abi::scalar_actual_kind(argument.source().kind())?,
            "value": argument.value().0 }),
            )
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({ "target": target, "receiver": value(receiver),
        "args": args, "dst": call.dst.map(|value| value.0) }))
}
