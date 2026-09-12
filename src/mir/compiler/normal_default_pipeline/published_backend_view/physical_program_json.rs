//! Closed JSON transport for the final lifecycle physical program.
//!
//! This deliberately does not share generic MIR JSON: generic egress changes
//! function and PHI order, while this transport preserves issued physical order.

use crate::mir::instruction::InvokeCallResultKind;
use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::mir::edge_args::JumpArgsLayout;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};
use crate::mir::{BinaryOp, Callee, ConstValue, EdgeArgs, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;

use super::physical_abi::PublishedLifecyclePhysicalAbiInputV1;
use super::physical_program::PublishedLifecyclePhysicalProgramV1;

const SCHEMA: &str = "hako.published-lifecycle-physical-program.v2";

fn emit_lifecycle_physical_program_value(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    let (births, ordinary) = function_ordinals(program)?;
    let functions = program
        .functions()
        .iter()
        .enumerate()
        .map(|(function_ordinal, function)| {
            let function_ordinal = u32::try_from(function_ordinal)
                .map_err(|_| fault("function-ordinal"))?;
            let blocks = function
                .blocks()
                .iter()
                .map(|block| {
                    let instructions = block
                        .instructions()
                        .iter()
                        .map(|row| -> Result<Value, String> { Ok(json!({
                            "index": row.index(),
                            "instruction": encode_instruction(
                                row.field_ref(), row.instruction(), &births, function_ordinal,
                                diagnostic_site(abi_input, function_ordinal, block.id().0, row.index(), row.instruction())?,
                                abi_input,
                                &ordinary,
                            )?,
                        })) })
                        .collect::<Result<Vec<_>, String>>()?;
                    Ok(json!({
                        "id": block.id().0,
                        "instructions": instructions,
                        "terminator": {
                            "index": block.terminator().index(),
                            "instruction": encode_instruction(
                                block.terminator().field_ref(), block.terminator().instruction(),
                                &births, function_ordinal,
                                diagnostic_site(
                                    abi_input, function_ordinal, block.id().0,
                                    block.terminator().index(), block.terminator().instruction(),
                                )?,
                                abi_input,
                                &ordinary,
                            )?,
                        },
                        "edges": block.edges().iter().map(encode_edge).collect::<Vec<_>>(),
                    }))
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(json!({
                "name": function.name(),
                "role": function.role().wire_name(),
                "receiver": function.role().receiver_value(function.params()).map(|value| value.0),
                "receiver_object": function
                    .role()
                    .receiver_object()
                    .map(|object| object.declaration_index()),
                "params": function.params().iter().skip(usize::from(function.role().has_receiver()))
                    .map(|param| json!({
                        "value": value(param),
                        "representation": if function.role().ordinary_target().is_some() {
                            "i64"
                        } else {
                            "kind_payload_v1"
                        },
                    }))
                    .collect::<Vec<_>>(),
                "entry": function.entry().0,
                "blocks": blocks,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(json!({ "schema": SCHEMA, "functions": functions }))
}

/// Extends the issued program transport with layout rows from the same final
/// view and the runtime-owned FaultFrame ABI revision. Uses the retained
/// program and argument relations without source reclassification.
pub(crate) fn emit_lifecycle_physical_abi_json(
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<String, String> {
    let mut root = emit_lifecycle_physical_program_value(input.program(), Some(input))?;
    let object = root.as_object_mut().ok_or_else(|| fault("program-root"))?;
    object.insert(
        "process_result_site".into(),
        json!(input.process_result_site()),
    );
    object.insert("fault_abi_version".into(), json!(input.fault_abi_version()));
    if input.program().is_native_array() {
        object.insert(
            "runtime_requirements".into(),
            json!({"kind": "native_array", "abi_version": 1}),
        );
    } else {
        object.insert(
            "storage_profile".into(),
            json!(input
                .storage_profile()
                .ok_or_else(|| fault("storage-profile-missing"))?),
        );
        object.insert(
            "layouts".into(),
            Value::Array(
                input
                    .layouts()
                    .iter()
                    .map(|layout| {
                        json!({
                            "object_id": layout.object_id(),
                            "runtime_type_id": layout.runtime_type_id(),
                            "field_count": layout.field_count(),
                            "fields": layout.fields().iter().map(|field| json!({
                                "declaration_ordinal": field.declaration_ordinal(),
                                "runtime_slot": field.runtime_slot(),
                                "storage_kind": field.storage_kind(),
                            })).collect::<Vec<_>>(),
                        })
                    })
                    .collect(),
            ),
        );
    }
    serde_json::to_string(&root).map_err(|error| fault(&format!("serialize:{error}")))
}

fn function_ordinals(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<
    (
        BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
        BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    ),
    String,
> {
    let mut births = BTreeMap::new();
    let mut ordinary = BTreeMap::new();
    for (ordinal, function) in program.functions().iter().enumerate() {
        let ordinal = u32::try_from(ordinal).map_err(|_| fault("function-ordinal"))?;
        if let Some(key) = function.role().birth_target() {
            if births.insert(key.clone(), ordinal).is_some() {
                return Err(fault("duplicate-birth-target"));
            }
        }
        if let Some(key) = function.role().ordinary_target() {
            if ordinary.insert(key.clone(), ordinal).is_some() {
                return Err(fault("duplicate-ordinary-target"));
            }
        }
    }
    Ok((births, ordinary))
}

fn encode_edge(edge: &super::physical_program::PublishedLifecyclePhysicalEdgeV1) -> Value {
    json!({ "target": edge.target().0, "args": edge.args().map(encode_edge_args) })
}

fn encode_edge_args(args: &EdgeArgs) -> Value {
    json!({
        "layout": match args.layout {
            JumpArgsLayout::CarriersOnly => "carriers_only",
            JumpArgsLayout::ExprResultPlusCarriers => "expr_result_plus_carriers",
        },
        "values": args.values.iter().map(|value| value.0).collect::<Vec<_>>(),
    })
}

fn encode_instruction(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    diagnostic_site: Option<u64>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
    ordinary: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
) -> Result<Value, String> {
    Ok(match instruction {
        MirInstruction::Const {
            dst,
            value: ConstValue::Bool(boolean),
        } => json!({ "op": "const_bool", "dst": value(dst), "value": boolean }),
        MirInstruction::Const {
            dst,
            value: ConstValue::Float(float),
        } if abi_input.is_some_and(|input| input.program().is_native_array()) => {
            json!({ "op": "const_f64_bits", "dst": value(dst), "bits": float.to_bits() })
        }
        MirInstruction::Const {
            dst,
            value: ConstValue::Integer(integer),
        } => json!({ "op": "const_i64", "dst": value(dst), "value": integer }),
        MirInstruction::Const {
            dst,
            value: ConstValue::String(text),
        } => json!({ "op": "const_string", "dst": value(dst), "value": text }),
        MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        } => json!({ "op": "const_unit", "dst": value(dst) }),
        MirInstruction::BinOp {
            dst,
            op: BinaryOp::Add,
            lhs,
            rhs,
        } => json!({ "op": "add", "dst": value(dst), "lhs": value(lhs), "rhs": value(rhs) }),
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            let predicate = match op {
                crate::mir::CompareOp::Eq => "eq",
                crate::mir::CompareOp::Ne => "ne",
                crate::mir::CompareOp::Lt => "slt",
                crate::mir::CompareOp::Le => "sle",
                crate::mir::CompareOp::Gt => "sgt",
                crate::mir::CompareOp::Ge => "sge",
            };
            json!({ "op": "compare", "dst": value(dst), "lhs": value(lhs),
                "rhs": value(rhs), "predicate": predicate })
        }
        MirInstruction::Copy { dst, src } => {
            json!({ "op": "copy", "dst": value(dst), "src": value(src) })
        }
        MirInstruction::Phi { dst, inputs, .. } => json!({
            "op": "phi", "dst": value(dst),
            "inputs": inputs.iter().map(|(block, value)| json!({ "block": block.0, "value": value.0 })).collect::<Vec<_>>(),
        }),
        MirInstruction::ObjectFieldGet { dst, base, field } => json!({
            "op": "object_field_get", "dst": value(dst), "base": value(base),
            "object_id": field.object().declaration_index(), "field_ordinal": field.declaration_ordinal(),
        }),
        MirInstruction::FieldGet { dst, base, .. } => {
            let field = field_ref.ok_or_else(|| fault("field-get-route-missing"))?;
            json!({
                "op": "object_field_get", "dst": value(dst), "base": value(base),
                "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(),
            })
        }
        MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        } => json!({
            "op": "invoke", "operation": encode_invoke(
                operation, births, ordinary, caller_function_index, diagnostic_site, abi_input,
            )?,
            "fault_frame": value(fault_frame), "normal": normal_landing.0, "fault": fault_landing.0,
        }),
        MirInstruction::InvokeNormalResult { invoke_block, dst } => {
            json!({ "op": "invoke_normal_result", "invoke_block": invoke_block.0, "dst": value(dst) })
        }
        MirInstruction::ArrayResidenceRelease { value: released } => {
            require_native_input(abi_input)?;
            json!({ "op": "array_residence_release", "value": value(released) })
        }
        MirInstruction::ReturnFault { fault_frame } => {
            json!({ "op": "return_fault", "fault_frame": value(fault_frame) })
        }
        MirInstruction::FaultFrameEnter { dst, mode } => json!({
            "op": "fault_frame_enter", "dst": value(dst),
            "mode": match mode { FaultFrameMode::RootOwned => "root_owned", FaultFrameMode::Borrowed => "borrowed" },
        }),
        MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        } => json!({
            "op": "branch", "condition": value(condition), "then": then_bb.0, "else": else_bb.0,
            "then_args": then_edge_args.as_ref().map(encode_edge_args),
            "else_args": else_edge_args.as_ref().map(encode_edge_args),
        }),
        MirInstruction::Jump { target, edge_args } => json!({
            "op": "jump", "target": target.0, "args": edge_args.as_ref().map(encode_edge_args),
        }),
        MirInstruction::Return { value: result } => {
            json!({ "op": "return", "value": result.map(|value| value.0) })
        }
        MirInstruction::Call(call) => {
            json!({ "op": "birth_call", "call": encode_birth_call(call, births, caller_function_index, abi_input)? })
        }
        _ => return Err(fault("instruction-unsupported")),
    })
}

fn diagnostic_site(
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
    function: u32,
    block: u32,
    instruction: u32,
    mir_instruction: &MirInstruction,
) -> Result<Option<u64>, String> {
    let Some(input) = abi_input else {
        return Ok(None);
    };
    let expected = super::physical_abi::PublishedLifecycleCheckedOperationKindV1::from_instruction(
        mir_instruction,
    );
    let issued = input.diagnostic_site_at(function, block, instruction);
    match (expected, issued) {
        (Some(expected), Some(issued)) if issued.kind() == expected => Ok(Some(issued.site())),
        (None, None) => Ok(None),
        _ => Err(fault("site-coordinate-drift")),
    }
}

fn encode_invoke(
    operation: &InvokeOperation,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    ordinary: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    diagnostic_site: Option<u64>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    Ok(match operation {
        InvokeOperation::Map(operation) => {
            use crate::mir::instruction::MapInvokeOperation as Map;
            let encoded = match operation {
                Map::New => json!({"kind": "map_new"}),
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
                    "value": value(stored), "value_kind": match kind {
                        crate::mir::instruction::MapValueKind::I64 => 1u32,
                        crate::mir::instruction::MapValueKind::Bool => 2u32,
                    },
                }),
                Map::EndOutcome { outcome } => {
                    json!({"kind": "map_end_outcome", "outcome": value(outcome)})
                }
                Map::End { map } => json!({"kind": "map_end", "map": value(map)}),
            };
            with_site(
                encoded,
                required_site(diagnostic_site, abi_input.is_some())?,
            )?
        }
        InvokeOperation::IntrinsicArrayNew => {
            require_native_input(abi_input)?;
            with_site(
                json!({"kind": "array_new"}),
                required_site(diagnostic_site, true)?,
            )?
        }
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
                "element_tag": super::physical_abi::array_element_tag(issued.spec)}),
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
            let input = require_native_input(abi_input)?;
            if *kind != crate::mir::ArrayElementWriteKind::LiteralAppend
                || *producer != crate::mir::ArrayWriteProducerKind::Literal
                || index.is_some()
            {
                return Err(fault("array-write-shape"));
            }
            let mut rows = input.entry().array_writes().iter().filter(|row| {
                row.site == *site_id && row.array == *receiver && row.value == *operand
            });
            let issued = rows.next().ok_or_else(|| fault("array-write-unissued"))?;
            if rows.next().is_some() {
                return Err(fault("array-write-duplicate"));
            }
            use super::compiled_entry_contract::CompiledEntryArrayValueKindV1 as Kind;
            let representation = match issued.kind {
                Kind::I64 => "i64",
                Kind::Bool => "bool",
                Kind::F64 => "f64",
            };
            with_site(
                json!({"kind": "array_append", "array": value(receiver),
                "value": value(operand), "representation": representation}),
                required_site(diagnostic_site, true)?,
            )?
        }
        InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::Unit,
        } => {
            if diagnostic_site.is_some() {
                return Err(fault("site-on-birth-call"));
            }
            json!({ "kind": "birth_call", "call": encode_birth_call(call, births, caller_function_index, abi_input)? })
        }
        InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::I64,
        } => {
            if diagnostic_site.is_some() {
                return Err(fault("site-on-ordinary-call"));
            }
            let key = super::physical_program::ordinary_callable_key(&call.callee)?;
            let target = ordinary
                .get(&key)
                .ok_or_else(|| fault("ordinary-target-missing"))?;
            if call.dst.is_some() {
                return Err(fault("ordinary-destination"));
            }
            let args = call.args.iter().map(|value| json!({
                "kind": "i64", "value": value.0,
            })).collect::<Vec<_>>();
            let call = match super::physical_program::ordinary_call_receiver(&call.callee)? {
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
            json!({
                "kind": "ordinary_call",
                "call": call,
                "result": "i64",
            })
        }
        InvokeOperation::NewBox { object } => with_site(
            json!({
                "kind": "new_box", "object_id": object.declaration_index(),
            }),
            required_site(diagnostic_site, abi_input.is_some())?,
        )?,
        InvokeOperation::FieldSet {
            field,
            base,
            value: stored,
        } => with_site(
            json!({
                "kind": "field_set", "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(), "base": value(base), "value": value(stored),
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

fn required_site(site: Option<u64>, required: bool) -> Result<Option<u64>, String> {
    if required && site.is_none() {
        return Err(fault("site-missing"));
    }
    Ok(site)
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

fn encode_birth_call(
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
                json!({ "kind": super::physical_abi::scalar_actual_kind(argument.source().kind())?,
            "value": argument.value().0 }),
            )
        })
        .collect::<Result<Vec<Value>, String>>()?;
    Ok(json!({ "target": target, "receiver": value(receiver),
        "args": args, "dst": call.dst.map(|value| value.0) }))
}

fn value(value: &ValueId) -> u32 {
    value.0
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-physical-json/{reason}]")
}

#[cfg(test)]
#[path = "physical_program_json_tests.rs"]
mod tests;
