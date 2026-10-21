//! Checked operation transport inside the sole lifecycle JSON owner.
use super::*;

pub(super) fn encode_invoke(
    operation: &InvokeOperation,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    ordinary: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    diagnostic_site: Option<u64>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
    call_context: Option<CallContext<'_>>,
) -> Result<Value, String> {
    Ok(match operation {
        InvokeOperation::ObjectEndIfOwned { object, value: ended, end } => {
            use crate::mir::instruction::ObjectEndStepV1 as Step;
            let steps = end.iter().map(|step| match step {
                Step::FieldResidence { mode, field } => json!({ "kind": match mode {
                    crate::mir::instruction::FieldResidenceEndModeV1::ArrayResidence => "array_field",
                    crate::mir::instruction::FieldResidenceEndModeV1::DynamicResidence => "dynamic_field",
                },
                    "field_ordinal": field.declaration_ordinal() }),
                Step::ObjectField { field, child } => json!({ "kind": "object_field",
                    "field_ordinal": field.declaration_ordinal(),
                    "child_object_id": child.declaration_index() }),
                Step::ReleaseOuter => json!({ "kind": "release_outer" }),
            }).collect::<Vec<_>>();
            with_site(json!({ "kind": "object_end_if_owned",
                "object_id": object.declaration_index(), "value": value(ended), "end": steps }),
                required_site(diagnostic_site, true)?)?
        }
        InvokeOperation::OwnedArray(operation) => {
            use crate::mir::instruction::OwnedArrayInvokeOperation as Op;
            let descriptor = operation.descriptor();
            let kind = match operation {
                Op::Claim { .. } => "array_owned_claim",
                Op::Append { .. } => "array_owned_append",
                Op::Get { .. } => "array_owned_get",
                Op::End { .. } => "array_owned_end",
            };
            let mut encoded = json!({"kind":kind, "array":value(&operation.array()),
                "object_id":descriptor.object().declaration_index(), "nullable":descriptor.nullable()});
            match operation {
                Op::Append { candidate, .. } => { encoded["value"] = value(candidate).into(); }
                Op::Get { index, .. } => { encoded["index"] = value(index).into(); }
                Op::Claim { .. } | Op::End { .. } => {}
            }
            with_site(encoded, required_site(diagnostic_site, true)?)?
        }
        InvokeOperation::String(operation) => {
            use crate::mir::instruction::StringInvokeOperation as Op;
            let operand = encode_string_operand_v1;
            let encoded = match operation {
                Op::Add { left, right } => json!({"kind":"string_add", "left":operand(left), "right":operand(right)}),
                Op::Compare { op, left, right } => json!({"kind":"string_compare", "comparison":match op {
                    crate::mir::CompareOp::Eq => "eq", crate::mir::CompareOp::Ne => "ne",
                    crate::mir::CompareOp::Lt => "lt", crate::mir::CompareOp::Le => "le",
                    crate::mir::CompareOp::Gt => "gt", crate::mir::CompareOp::Ge => "ge",
                }, "left":operand(left), "right":operand(right)}),
                Op::Materialize { value } => json!({"kind":"string_materialize", "value":operand(value)}),
                Op::Length { value } => json!({"kind":"string_length", "value":operand(value)}),
                Op::Substring { value:v, start, end } => json!({"kind":"string_substring", "value":operand(v), "start":value(start), "end":value(end)}),
                Op::IndexOf { value, needle } => json!({"kind":"string_index_of", "value":operand(value), "needle":operand(needle)}),
                Op::Print { value } => json!({"kind":"string_print", "value":operand(value)}),
                Op::End { value:v } => json!({"kind":"string_end", "value":value(v)}),
            };
            with_site(encoded, required_site(diagnostic_site, abi_input.is_some())?)?
        }
        InvokeOperation::Map(operation) => {
            use crate::mir::instruction::MapInvokeOperation as Map;
            let encoded = match operation {
                Map::New => json!({"kind": "map_new"}),
                Map::FieldHas { field, base, key } => json!({"kind":"map_field_has",
                    "object_id":field.object().declaration_index(), "field_ordinal":field.declaration_ordinal(),
                    "base":value(base), "key":encode_string_operand_v1(key)}),
                Map::FieldGetIndexed { field, base, key, object } => json!({"kind":"map_field_get_indexed",
                    "object_id":field.object().declaration_index(), "field_ordinal":field.declaration_ordinal(),
                    "base":value(base), "key":encode_string_operand_v1(key), "payload_object_id":object.declaration_index()}),
                Map::PrepareKey { utf8 } => json!({"kind": "map_prepare_key", "utf8": utf8}),
                Map::InstallIndexed {
                    map,
                    key,
                    object,
                    value: stored,
                } => json!({
                    "kind": "map_install_indexed", "map": value(map), "key": value(key),
                    "object_id": object.declaration_index(), "value": value(stored),
                }),
                Map::InstallValue {
                    map,
                    key,
                    value: stored,
                    kind,
                } => json!({
                    "kind": "map_install_value", "map": value(map), "key": value(key),
                    // NYRT_MAP_VALUE_* checked-map kinds — a different
                    // namespace from the static-V2 HAKO_LLVMC_MAP_VALUE_*
                    // table, where the same field name maps 3 to F64.
                    "value": value(stored), "value_kind": match kind {
                        crate::mir::instruction::MapValueKind::I64 => 1u32,
                        crate::mir::instruction::MapValueKind::Bool => 2u32,
                        crate::mir::instruction::MapValueKind::BorrowedHandle => 3u32,
                    },
                }),
                Map::InstallText { map, key, utf8 } => json!({
                    "kind": "map_install_text", "map": value(map), "key": value(key),
                    "utf8": utf8,
                }),
                Map::InstallEmptyArray { map, key } => json!({
                    "kind": "map_install_empty_array", "map": value(map), "key": value(key),
                }),
                Map::InstallBorrowedArray { map, key, elements } => json!({
                    "kind": "map_install_borrowed_array",
                    "map": value(map),
                    "key": value(key),
                    "elements": elements.iter().map(value).collect::<Vec<_>>(),
                }),
                Map::EndOutcome { outcome } => {
                    json!({"kind": "map_end_outcome", "outcome": value(outcome)})
                }
                Map::End { map } => json!({"kind": "map_end", "map": value(map)}),
                Map::CheckedGetI64 { map, utf8 } => {
                    json!({"kind": "map_checked_get", "map": value(map), "utf8": utf8})
                }
                Map::ArrayLength { map, utf8 } => {
                    json!({"kind": "map_array_length", "map": value(map), "utf8": utf8})
                }
                Map::ArrayIndexMap { map, utf8, index } => json!({
                    "kind": "map_array_index_map", "map": value(map),
                    "utf8": utf8, "index": index,
                }),
                Map::MapGetText { map, utf8 } => {
                    json!({"kind": "map_get_text", "map": value(map), "utf8": utf8})
                }
            };
            with_site(
                encoded,
                required_site(diagnostic_site, abi_input.is_some())?,
            )?
        }
        InvokeOperation::ScriptArgumentsNew { values } => with_site(
            json!({"kind": "script_arguments_new", "values": values}),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::IntrinsicArrayNew => with_site(
            json!({"kind": "array_new"}),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::ArrayStateContractClaim { contract_id, array } => {
            let input = require_native_input(abi_input)?;
            let mut rows = input
                .entry()
                .array_claims()
                .iter()
                .filter(|row| row.array == *array && row.contract_id == contract_id);
            let issued = rows.next().ok_or_else(|| fault("array-claim-unissued"))?;
            if rows.next().is_some() {
                return Err(fault("array-claim-duplicate"));
            }
            with_site(
                json!({"kind": "array_claim", "array": value(array),
                "element_tag": super::super::physical_abi::array_element_tag(issued.spec)}),
                required_site(diagnostic_site, true)?,
            )?
        }
        InvokeOperation::ArrayElementWrite {
            site_id,
            kind,
            producer,
            receiver,
            index,
            value: operand,
        } => {
            let input = abi_input.ok_or_else(|| fault("array-write-input-missing"))?;
            let native = input.program().is_native_array();
            if native { require_native_input(abi_input)?; }
            let shape = if native {
                *kind == crate::mir::ArrayElementWriteKind::LiteralAppend
                    && *producer == crate::mir::ArrayWriteProducerKind::Literal
            } else {
                *kind == crate::mir::ArrayElementWriteKind::Push
                    && *producer == crate::mir::ArrayWriteProducerKind::MethodCall
            };
            if !shape || index.is_some() { return Err(fault("array-write-shape")); }
            let (_, caller, (block, instruction)) = call_context.ok_or_else(|| fault("array-write-coordinate-missing"))?;
            call_transport::corroborate_function(caller, caller_function_index, input)?;
            let mut rows = input.entry().array_writes().iter().filter(|row| {
                row.function_index == caller_function_index && row.block == block
                    && row.instruction_index == instruction
                    && row.site == *site_id && row.array == *receiver && row.value.operand() == *operand
            });
            let issued = rows.next().ok_or_else(|| fault("array-write-unissued"))?;
            if rows.next().is_some() {
                return Err(fault("array-write-duplicate"));
            }
            use super::super::compiled_entry_contract::{CompiledEntryArrayValueKindV1 as Kind, CompiledEntryArrayWriteValueV1 as Payload};
            if !operand.valid_for(*kind, *producer, *index) { return Err(fault("array-write-shape")); }
            let (representation, payload) = match &issued.value {
                Payload::Scalar { value: scalar, kind } => {
                    if !native && *kind != Kind::I64 { return Err(fault("field-array-write-representation")); }
                    (match kind { Kind::I64 => "i64", Kind::Bool => "bool", Kind::F64 => "f64" }, json!(value(scalar)))
                }
                Payload::Text(value) if !native && value.is_text() => ("text", json!(value)),
                _ => return Err(fault("field-array-write-representation")),
            };
            with_site(
                json!({"kind": "array_append", "array": value(receiver),
                "value": payload, "representation": representation}),
                required_site(diagnostic_site, true)?,
            )?
        }

        InvokeOperation::Birth(call) => {
            if diagnostic_site.is_some() {
                return Err(fault("site-on-birth-call"));
            }
            json!({ "kind": "birth_call", "call": call_transport::encode_birth_call(call, births, caller_function_index, abi_input)? })
        }
        InvokeOperation::Call {
            call,
            result:
                result_kind @ (InvokeCallResultKind::Unit | InvokeCallResultKind::I64 | InvokeCallResultKind::OwnedString
                | InvokeCallResultKind::Bool
                | InvokeCallResultKind::Map
                | InvokeCallResultKind::Handle
                | InvokeCallResultKind::NullableHandle
                | InvokeCallResultKind::BorrowedNullableHandle { .. }
                | InvokeCallResultKind::MixedHandle { .. }),
        } => {
            if diagnostic_site.is_some() {
                return Err(fault("site-on-ordinary-call"));
            }
            call_transport::encode_ordinary_call(
                call,
                *result_kind,
                ordinary,
                call_context,
                caller_function_index,
                abi_input,
            )?
        }
        InvokeOperation::NewBox { object } => with_site(
            json!({
                "kind": "new_box", "object_id": object.declaration_index(),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::FieldGet { field, base } => with_site(
            json!({
                "kind": "field_get", "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::FieldSet { mode, field, base, value: stored } => {
            let mut encoded = with_site(json!({
                "kind": "field_set", "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base), "value": value(stored),
                "exact_numeric_runtime_check": numeric::range_check(abi_input, caller_function_index, call_context),
            }), required_site(diagnostic_site, abi_input.is_some())?)?;
            use crate::mir::instruction::FieldSetModeV1 as Mode;
            match mode {
                Mode::ExactSlot => {},
                Mode::DynamicI64Replace => encoded["transaction"] = "dynamic_i64_replace".into(),
                Mode::DynamicI64Initialize => encoded["transaction"] = "dynamic_i64_initialize".into(),
                Mode::DynamicTextInitialize => return Err("[freeze:contract][construction-store/source-or-cleanup-contract]".into()),
                Mode::DynamicMapInitialize => encoded["transaction"] = "dynamic_map_initialize".into(),
            }
            encoded
        },
        InvokeOperation::ObjectFieldSet {
            field,
            base,
            value: stored,
            child,
        } => with_site(
            json!({
                "kind": "object_field_set", "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base),
                "value": value(stored), "child_object_id": child.declaration_index(),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::HomeRelease {
            object,
            value: released,
        } => with_site(
            json!({ "kind": "home_release", "object_id": object.declaration_index(), "value": value(released),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::HomeReleaseIfLive {
            object,
            value: released,
        } => with_site(
            json!({ "kind": "home_release_if_live", "object_id": object.declaration_index(), "value": value(released),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::OwnedFieldResidenceRelease { mode, field, base } => {
            let mut encoded = with_site(json!({ "kind": "field_residence_release",
                "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base),
            }), required_site(diagnostic_site, abi_input.is_some())?)?;
            if *mode == crate::mir::instruction::FieldResidenceEndModeV1::DynamicResidence {
                encoded["transaction"] = "take_dynamic".into();
            }
            encoded
        },
        InvokeOperation::OwnedObjectFieldRelease { field, base, child } => with_site(
            json!({ "kind": "object_field_release",
                "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base),
                "child_object_id": child.declaration_index(),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::ReclaimUnpublished {
            object,
            value: reclaimed,
        } => with_site(
            json!({ "kind": "reclaim_unpublished", "object_id": object.declaration_index(), "value": value(reclaimed),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
    })
}

fn require_native_input<'input, 'module>(
    input: Option<&'input PublishedLifecyclePhysicalAbiInputV1<'module>>,
) -> Result<&'input PublishedLifecyclePhysicalAbiInputV1<'module>, String> {
    input
        .filter(|input| input.program().is_native_array())
        .ok_or_else(|| fault("native-array-input-missing"))
}

fn with_site(mut operation: Value, site: Option<u64>) -> Result<Value, String> {
    if let Some(site) = site {
        operation
            .as_object_mut()
            .ok_or_else(|| fault("operation-object"))?
            .insert("site".into(), json!(site));
    }
    Ok(operation)
}

/// Shared descriptor spelling; only the Call/source owners issue an operand.
pub(super) fn encode_string_operand_v1(operand: &crate::mir::instruction::StringInvokeOperand) -> Value {
    json!(operand)
}
