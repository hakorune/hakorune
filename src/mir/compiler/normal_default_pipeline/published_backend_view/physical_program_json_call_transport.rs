//! Closed parameter and Call transport. Source proofs stay with their existing
//! owners; this encoder never infers an opaque formal from payload bits.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::normal_callable_semantic_package::BorrowedFormalActualSourceV1 as Source;
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;
use super::super::physical_program::PublishedLifecyclePhysicalFunctionRoleV1;

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
                        let birth_text = match function.role() {
                            PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } =>
                                abi.text_formal_source_v1(index as u32)?.is_some(),
                            _ => false,
                        };
                        let (birth_dynamic_object, birth_dynamic_i64) = match function.role() {
                            PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } => (
                                abi.dynamic_object_formal_source_v1(index as u32)?,
                                abi.dynamic_i64_formal_source_v1(index as u32)?,
                            ),
                            _ => (None, false),
                        };
                        if birth_text && carrier != Some(Carrier::BorrowedTaggedValue) {
                            return Err(fault("param-carrier-drift"));
                        }
                        if (birth_dynamic_object.is_some() || birth_dynamic_i64)
                            && carrier != Some(Carrier::ExistingCallableI64)
                        {
                            return Err(fault("param-carrier-drift"));
                        }
                        let named_map = matches!(
                            param_type,
                            crate::mir::MirType::Box(name) if name == "MapBox"
                        );
                        let representation = match (carrier, named_map) {
                            (Some(crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue), _) => {
                                let input = abi_input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
                                corroborate_function(function, ordinal, input)?;
                                if !birth_text && !input.entry().has_borrowed_entry(ordinal) {
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
                            _ if birth_dynamic_i64 => "i64",
                            _ if birth_dynamic_object.is_some() => "nullable_kind_payload_v1",
                            _ if function.role().ordinary_target().is_some() => "i64",
                            _ => {
                                // A birth formal the sealed contract proves
                                // feeds only object-typed declared fields
                                // carries the nullable kind/payload lane —
                                // the rep is read from the compiled-entry
                                // contract, never guessed from the type.
                                let nullable = u32::try_from(index).ok().is_some_and(|formal_ordinal| {
                                    abi_input
                                        .and_then(|input| {
                                            input.entry().births().iter().find(|birth| {
                                                birth.function_index() == ordinal
                                            })
                                        })
                                        .and_then(|birth| {
                                            birth.formals().iter().find(|formal| {
                                                formal.source_ordinal()
                                                    == Some(formal_ordinal)
                                            })
                                        })
                                        .and_then(|formal| formal.contract())
                                        .is_some_and(|contract| {
                                            matches!(
                                                contract.uses(),
                                                crate::mir::normal_callable_semantic_package
                                                    ::BirthFormalUseCoverageV1::ObjectFieldStores { .. }
                                            )
                                        })
                                });
                                if nullable {
                                    "nullable_kind_payload_v1"
                                } else {
                                    "kind_payload_v1"
                                }
                            }
                        };
                        // A sealed formal object view rides the param row as
                        // `object_view`: the canonical object the guarded
                        // `formal.field` read must name — never a layout
                        // guess the C consumer could mint on its own.
                        let mut row = json!({
                            "value": value(param),
                            "representation": representation,
                        });
                        if representation == "borrowed_kind_payload_v1" {
                            let input = abi_input
                                .ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
                            row["object_view"] = json!(
                                input.entry().borrowed_object_view(ordinal, param.0)
                            );
                            if birth_text {
                                row["text_view"] = json!(true);
                            }
                            if let Some(domain) = input.entry().borrowed_formal_domain_v1(ordinal, param.0) {
                                let view = if domain.text_only() { Some("text_view") }
                                    // Completed Uses domains lend the same checked scalar entry
                                    // view without granting finite-expression use permissions.
                                    else if domain.signed_only() { Some("integer_view") }
                                    else if input.entry().has_finite_borrowed_input_v1(ordinal) && domain.bool_only() { Some("bool_view") } else { None };
                                if let Some(view) = view {
                                    if function.role().ordinary_target().is_none() {
                                        return Err(format!("{} function={} role={} ordinal={index} view={view}",
                                            fault("borrowed-text/parameter-role"), function.name(), function.role().wire_name()));
                                    }
                                    row[view] = json!(true);
                                } else if domain.single_object_class_v1().is_some() {
                                    if input.entry().borrowed_object_view(ordinal, param.0).is_none() || function.role().ordinary_target().is_none() {
                                        return Err(fault("borrowed-carrier-source-missing"));
                                    }
                                    row["object_nonnull_view"] = json!(true);
                                } else if input.entry().has_finite_borrowed_input_v1(ordinal) {
                                    use crate::mir::normal_callable_semantic_package::SourceValueAlternativeV1 as Alternative;
                                    let mut alternatives = domain.finite_alternatives_v1()
                                        .ok_or_else(|| fault("borrowed-carrier-source-missing"))?.iter().map(|alternative| match alternative {
                                            Alternative::Null => Ok((0, "null")), Alternative::SignedI64 => Ok((1, "i64")),
                                            Alternative::Bool => Ok((2, "bool")), Alternative::Text => Ok((4, "text")),
                                            _ => Err(fault("borrowed-carrier-source-missing")),
                                        }).collect::<Result<Vec<_>, _>>()?;
                                    alternatives.sort_unstable_by_key(|(tag, _)| *tag);
                                    row["finite_domain"] = json!(alternatives.into_iter().map(|(_, name)| name).collect::<Vec<_>>());
                                }
                            }
                        }
                        Ok(row)
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
    if callee.role().ordinary_result() != Some(result_kind) {
        return Err(fault("ordinary-result-role-correspondence"));
    }
    let offset = usize::from(callee.role().has_receiver());
    let selected = callee
        .param_carriers()
        .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue));
    let issued_call = if selected {
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
        row.borrowed_actuals().ok_or_else(|| fault("borrowed-carrier-source-missing"))?;
        Some(row)
    } else {
        None
    };
    let issued_actuals = issued_call.and_then(|row| row.borrowed_actuals());
    let mut consumed = 0;
    let args = call.args.iter().enumerate().map(|(index, value)| {
        let carrier = callee.param_carriers().and_then(|rows| rows.get(index + offset));
        let mut proofs = issued_actuals.into_iter().flatten().filter(|row| row.ordinal as usize == index);
        let proof = proofs.next();
        if proofs.next().is_some() { return Err(fault("borrowed-carrier-ordinal-duplicate")); }
        if carrier == Some(&Carrier::BorrowedTaggedValue) {
            let proof = proof.ok_or_else(|| fault("borrowed-carrier-ordinal-missing"))?;
            consumed += 1;
            if matches!(proof.source, Source::Expression(_)) {
                let issued = issued_call.ok_or_else(|| fault("borrowed-expression/call-missing"))?;
                let operand = issued.borrowed_computed_operand_v1(proof.ordinal)
                    .ok_or_else(|| fault("borrowed-expression/descriptor-missing"))?;
                let parameter = callee.params().get(index + offset).ok_or_else(|| fault("borrowed-expression/formal-missing"))?;
                let input = abi_input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?;
                let domain = input.entry().borrowed_formal_domain_v1(*target, parameter.0)
                    .filter(|domain| domain.ordinal() as usize == index)
                    .ok_or_else(|| fault("borrowed-expression/finite-domain-transport-unconnected"))?;
                return match (&proof.source, operand) {
                    (Source::Expression(source), crate::mir::instruction::StringInvokeOperand::NormalInteger(original))
                        if source.frame().is_i64() && original == value && domain.permits_integer_actual_v1() =>
                            Ok(json!({"kind":1, "value":value.0})),
                    (Source::Expression(source), crate::mir::instruction::StringInvokeOperand::OwnedString(original))
                        if !source.frame().is_i64() && original == value && domain.permits_text_actual_v1() =>
                            Ok(json!({"kind":"borrowed_text", "operand":super::operation::encode_string_operand_v1(operand)})),
                    _ => Err(format!("{} caller={} callee={} ordinal={index} operand={operand:?} actual={value:?} source_i64={} text_member={} integer_member={}",
                        fault("borrowed-expression/operand-drift"), caller.name(), callee.name(),
                        matches!(&proof.source, Source::Expression(source) if source.frame().is_i64()),
                        domain.permits_text_actual_v1(), domain.permits_integer_actual_v1())),
                };
            }
            if issued_call.and_then(|row| row.borrowed_computed_operand_v1(proof.ordinal)).is_some() {
                return Err(fault("borrowed-expression/unselected-descriptor"));
            }
            let kind = encode_borrowed_call_actual_kind(
                &proof.source, caller, caller_function_index,
                abi_input.ok_or_else(|| fault("borrowed-carrier-activation-missing"))?,
            )?;
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
    let text_argument_cleanup = &call.text_argument_cleanup;
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
    let mut wire = json!({
        "kind": "ordinary_call",
        "call": call,
        "result": match result_kind {
            InvokeCallResultKind::BorrowedNullableHandle { .. } => "borrowed_nullable_handle",
            InvokeCallResultKind::MixedHandle { .. } => "mixed_handle",
            InvokeCallResultKind::Bool => "bool",
            InvokeCallResultKind::Map => "map",
            InvokeCallResultKind::OwnedString => "text",
            InvokeCallResultKind::Handle => "handle",
            InvokeCallResultKind::NullableHandle => "nullable_handle",
            InvokeCallResultKind::I64 => "i64",
            InvokeCallResultKind::Unit => "unit",
        },
    });
    if !text_argument_cleanup.is_empty() {
        if result_kind != InvokeCallResultKind::OwnedString { return Err(fault("ordinary-argument-kind")); }
        wire["text_argument_cleanup"] = json!(text_argument_cleanup.iter().map(|value| value.0).collect::<Vec<_>>());
    }
    encode_result_support_v1(result_kind, &mut wire)?;
    Ok(wire)
}

pub(super) fn encode_birth_call(
    call: &crate::mir::instruction::BirthInvokeV1,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    let key = &call.key;
    let receiver = &call.receiver;
    let target = births
        .get(key)
        .ok_or_else(|| fault("birth-target-foreign"))?;
    let input = abi_input.ok_or_else(|| fault("birth-input-missing"))?;
    let mut matches = input.entry().birth_calls().iter().filter(|issued| {
        issued.caller_function_index() == caller_function_index
            && issued.function_index() == *target
            && issued.receiver() == *receiver
            && call.arguments.iter().eq(issued.arguments())
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
            if let crate::mir::instruction::BirthArgumentV1::Text(operand) = argument.payload() {
                let callee = input.program().functions().get(*target as usize)
                    .ok_or_else(|| fault("birth-target-foreign"))?;
                let PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = callee.role()
                    else { return Err(fault("birth-actual-formal-missing")) };
                if abi.text_formal_source_v1(argument.source().ordinal())?.is_none() {
                    return Err(fault("actual-kind-unavailable"));
                }
                return Ok(json!({"kind":"borrowed_text", "operand":super::operation::encode_string_operand_v1(operand)}));
            }
            // The source-admitted Integer view retains the exact receiving ABI:
            // an i64 formal consumes its payload, a pair formal both lanes.
            if let Some(transport) = input.borrowed_birth_actual(argument.source().new_site(), argument.source().ordinal()) {
                return Ok(json!({ "kind": transport.wire_kind(), "value": argument.value_only_v1()?.0 }));
            }
            // An `ObjectFieldStores` formal rides the nullable pair lane:
            // the sealed source kind selects (0,0) for `null` or
            // (3,payload) for an owned handle move — the flow layer
            // admits the live lease and discharges it at the call edge.
            if let Some(transport) = input.object_birth_actual(
                argument.source().new_site(), argument.source().ordinal(),
            ) {
                return Ok(json!({ "kind": transport.wire_kind(), "value": argument.value_only_v1()?.0 }));
            }
            Ok(
                json!({ "kind": super::super::physical_abi::scalar_actual_kind(
                    argument.source().kind(), argument.value_only_v1()?, caller.value_types())?,
            "value": argument.value_only_v1()?.0 }),
            )
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({ "target": target, "receiver": value(receiver),
        "args": args, "dst": Value::Null }))
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
    let original = input.is_some_and(|input| input.entry().has_borrowed_entries()
        || input.program().functions().iter().any(|function| function.param_carriers()
            .is_some_and(|rows| rows.contains(&Carrier::BorrowedTaggedValue))));
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
        || function.role().ordinary_result() != original.role().ordinary_result()
        || function.role().object_return_outcomes() != original.role().object_return_outcomes()
        || function.role().ordinary_target() != original.role().ordinary_target()
        || function.role().birth_target() != original.role().birth_target()
        || matches!((function.role(), original.role()),
            (PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: left },
             PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi: right }) if left != right)
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

/// Class authority alone does not select a physical pair. Borrow the original
/// caller entry and its installed carrier before preserving the incoming tag.
fn encode_borrowed_call_actual_kind(
    source: &Source,
    caller: &PublishedLifecyclePhysicalFunctionV1<'_>,
    caller_index: u32,
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<Value, String> {
    if let Source::DeclaredFormal { root, .. } = source {
        if input.entry().has_borrowed_entry(caller_index) {
            let original = input.program().handoff().root_source()
                .ok_or_else(|| fault("borrowed-declared/source-missing"))?;
            let entries = original.borrowed_ordinary_entry_values_v1(root.owner())?;
            let mut matches = entries.iter().filter(|(_, formal, _)| formal == root);
            let (_, _, value) = matches.next()
                .ok_or_else(|| fault("borrowed-declared/entry-missing"))?;
            if matches.next().is_some() {
                return Err(fault("borrowed-declared/entry-duplicate"));
            }
            let index = caller.params().iter().position(|param| param == value)
                .ok_or_else(|| fault("borrowed-declared/parameter-missing"))?;
            if caller.param_carriers().and_then(|rows| rows.get(index))
                != Some(&Carrier::BorrowedTaggedValue)
            { return Err(fault("borrowed-declared/carrier-drift")); }
            return Ok(json!("tagged"));
        }
    }
    encode_borrowed_actual_kind(source)
}

fn encode_borrowed_actual_kind(source: &Source) -> Result<Value, String> {
    Ok(match source {
        // A computed actual needs the descriptor from this SAME finalized Call
        // packet. Its source frame alone cannot manufacture a wire operand.
        Source::Expression(_) => return Err(fault("borrowed-expression/descriptor-missing")),
        // The exact `null` literal spells tag 0; the `value` field still
        // names the exact `const_null` producer, never a zero payload.
        Source::Null => json!(0),
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
        Source::TypedHome { .. }
        | Source::EntryReceiver { .. }
        | Source::LocalObjectBinding { .. }
        | Source::DeclaredFormal { .. } => json!(3),
        Source::ReceivedNullable { .. } | Source::ReceivedMixed { .. } | Source::Read(_) => json!("nullable_typed_object"),
        Source::Forwarded { .. } => json!("tagged"),
    })
}

#[cfg(test)]
#[path = "physical_program_json_borrowed_transport_tests.rs"]
mod borrowed_transport_tests;

/// Both declaration and Call encode the SAME support law. Receiver-field
/// storage remains explicit; consumers must validate the full slot/lifetime
/// tuple rather than substituting a formal ordinal.
pub(super) fn encode_result_support_v1(result: InvokeCallResultKind, wire: &mut Value) -> Result<(), String> {
    let ordinal = match result {
        InvokeCallResultKind::MixedHandle { formal_ordinal } => Some(formal_ordinal),
        InvokeCallResultKind::BorrowedNullableHandle { support } => match support {
            crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) => Some(ordinal),
            crate::mir::instruction::BorrowedResultSupportV1::ReceiverFieldSlot { field, key_ordinal, object } => {
                wire["result_support"] = json!({"kind":"receiver_field_slot",
                    "field":{"object_id":field.object().declaration_index(), "field_ordinal":field.declaration_ordinal()},
                    "key_ordinal":key_ordinal, "object_id":object.declaration_index()});
                None
            }
        },
        _ => None,
    };
    if let Some(ordinal) = ordinal { wire["result_formal_ordinal"] = json!(ordinal); }
    Ok(())
}
