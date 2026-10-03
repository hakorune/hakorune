//! Closed parameter and Call transport. Source proofs stay with their existing
//! owners; this encoder never infers an opaque formal from payload bits.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;

pub(super) fn encode_parameters(
    function: &PublishedLifecyclePhysicalFunctionV1<'_>,
    ordinal: u32,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
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
                                let input = abi_input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
                                corroborate_function(function, ordinal, input)?;
                                if !input.entry().ordinary_calls().iter().any(|row| row.function_index() == ordinal && row.borrowed_actuals().is_some()) {
                                    return Err(fault("borrowed-carrier-source-missing"));
                                }
                                "borrowed_kind_payload_v1"
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
    caller_function_index: u32,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
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
    let (functions, caller, coordinate) =
        call_context.ok_or_else(|| fault("ordinary-call-context"))?;
    let callee = functions
        .get(*target as usize)
        .ok_or_else(|| fault("ordinary-target-missing"))?;
    let offset = usize::from(callee.role().has_receiver());
    let selected = callee
        .param_carriers()
        .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue));
    let issued_actuals = if selected {
        let input = abi_input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
        corroborate_function(caller, caller_function_index, input)?;
        corroborate_function(callee, *target, input)?;
        let mut rows = input.entry().ordinary_calls().iter().filter(|row| {
            row.caller_function_index() == caller_function_index
                && row.caller_block_id() == coordinate.0
                && row.caller_instruction_index() == coordinate.1
        });
        let row = rows
            .next()
            .ok_or_else(|| fault("borrowed-carrier-call-missing"))?;
        if rows.next().is_some()
            || row.function_index() != *target
            || row.call() != call
            || row.result() != result_kind
        {
            return Err(fault("borrowed-carrier-call-drift"));
        }
        Some(
            row.borrowed_actuals()
                .ok_or_else(|| fault("borrowed-carrier-source-missing"))?,
        )
    } else {
        None
    };
    let mut consumed = 0;
    let args = call.args.iter().enumerate().map(|(index, value)| {
        let carrier = callee.param_carriers().and_then(|rows| rows.get(index + offset));
        let mut proofs = issued_actuals.into_iter().flatten().filter(|row| row.ordinal as usize == index);
        let proof = proofs.next();
        if proofs.next().is_some() { return Err(fault("borrowed-carrier-ordinal-duplicate")); }
        if carrier == Some(&Carrier::BorrowedTaggedValue) {
            let proof = proof.ok_or_else(|| fault("borrowed-carrier-ordinal-missing"))?;
            consumed += 1;
            let kind = encode_borrowed_actual_kind(&proof.source);
            return Ok(json!({"kind": kind, "value": value.0}));
        }
        if proof.is_some() { return Err(fault("borrowed-carrier-ordinal-unselected")); }
        let map_actual = matches!(caller.value_types().get(value), Some(crate::mir::MirType::Box(name)) if name == "MapBox");
        let map_formal = carrier == Some(&Carrier::CheckedMapStorage);
        if map_actual != map_formal { return Err(fault("ordinary-argument-kind")); }
        Ok(json!({"kind": if map_actual { "map" } else { "i64" }, "value": value.0}))
    }).collect::<Result<Vec<_>, String>>()?;
    if issued_actuals.is_some_and(|proofs| proofs.len() != consumed) {
        return Err(fault("borrowed-carrier-source-residual"));
    }
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
    let caller = input
        .entry()
        .program()
        .functions()
        .get(caller_function_index as usize)
        .ok_or_else(|| fault("birth-caller-missing"))?;
    let args = issued
        .actual()
        .arguments()
        .iter()
        .map(|argument| {
            // An admitted dominated `new`-argument use carries the same lent
            // Integer view as a checked compare operand; the `"tagged"`
            // spelling hands the callee its proven kind==1 payload.
            if input.tagged_birth_actual(argument.source().new_site(), argument.source().ordinal())
            {
                return Ok(json!({ "kind": "tagged", "value": argument.value().0 }));
            }
            Ok(
                json!({ "kind": super::super::physical_abi::scalar_actual_kind(
                    argument.source().kind(), argument.value(), caller.value_types())?,
            "value": argument.value().0 }),
            )
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({ "target": target, "receiver": value(receiver),
        "args": args, "dst": call.dst.map(|value| value.0) }))
}

/// A removed column still requires its original source loan. Compare before
/// any encoding, so a producer mutation cannot ride an unchanged Call operand.
pub(super) fn verify_borrowed_projection(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<(), String> {
    let current = program.functions().iter().any(|function| {
        function
            .param_carriers()
            .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue))
    });
    let original = input.is_some_and(|input| {
        input
            .entry()
            .ordinary_calls()
            .iter()
            .any(|row| row.borrowed_actuals().is_some())
    });
    if !current && !original {
        return Ok(());
    }
    let input = input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
    if program.functions().len() != input.program().functions().len() {
        return Err(fault("borrowed-carrier-function-drift"));
    }
    for (ordinal, function) in program.functions().iter().enumerate() {
        corroborate_function(function, ordinal as u32, input)?;
    }
    Ok(())
}

/// Metadata corroborates an already issued source loan; it cannot issue one.
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn corroborate_function(
    function: &PublishedLifecyclePhysicalFunctionV1<'_>,
    ordinal: u32,
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    let original = input
        .program()
        .functions()
        .get(ordinal as usize)
        .ok_or_else(|| fault("borrowed-carrier-function-missing"))?;
    if std::ptr::eq(function, original) {
        return Ok(());
    }
    if function.name() != original.name()
        || function.role().wire_name() != original.role().wire_name()
        || function.role().ordinary_target() != original.role().ordinary_target()
        || function.role().birth_target() != original.role().birth_target()
        || function.role().receiver_object() != original.role().receiver_object()
        || function.role().has_receiver() != original.role().has_receiver()
        || function.params() != original.params()
        || function.param_types() != original.param_types()
        || function.param_carriers() != original.param_carriers()
        || function.value_types() != original.value_types()
        || function.entry() != original.entry()
        || function.blocks().len() != original.blocks().len()
    {
        return Err(fault("borrowed-carrier-function-drift"));
    }
    for (block, saved) in function.blocks().iter().zip(original.blocks()) {
        let same_row = |a: super::super::physical_program::PublishedLifecyclePhysicalInstructionRefV1<'_>,
                        b: super::super::physical_program::PublishedLifecyclePhysicalInstructionRefV1<'_>| {
            a.index() == b.index() && a.field_ref() == b.field_ref() && a.instruction() == b.instruction()
        };
        if block.id() != saved.id()
            || block.edges() != saved.edges()
            || block.instructions().len() != saved.instructions().len()
            || !same_row(block.terminator(), saved.terminator())
            || !block
                .instructions()
                .iter()
                .zip(saved.instructions())
                .all(|(a, b)| same_row(*a, *b))
        {
            return Err(fault("borrowed-carrier-projection-drift"));
        }
    }
    Ok(())
}

fn encode_borrowed_actual_kind(source: &Source) -> Value {
    match source {
        Source::Integer(_)
        | Source::Scalar {
            kind: SourceScalarKind::Integer,
            ..
        } => json!(1),
        Source::Bool(_)
        | Source::Scalar {
            kind: SourceScalarKind::Bool,
            ..
        } => json!(2),
        Source::TypedHome { .. } | Source::EntryReceiver { .. } => json!(3),
        Source::Forwarded { .. } => json!("tagged"),
    }
}

#[cfg(test)]
#[path = "physical_program_json_borrowed_transport_tests.rs"]
mod borrowed_transport_tests;
