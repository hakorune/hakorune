//! Closed JSON transport for the final lifecycle physical program.
//!
//! This deliberately does not share generic MIR JSON: generic egress changes
//! function and PHI order, while this transport preserves issued physical order.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::mir::{BinaryOp, Callee, ConstValue, EdgeArgs, MirInstruction, ValueId};
use crate::mir::edge_args::JumpArgsLayout;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};

use super::physical_program::{
    PublishedLifecyclePhysicalProgramV1,
};
use super::physical_abi::PublishedLifecyclePhysicalAbiInputV1;

const SCHEMA: &str = "hako.published-lifecycle-physical-program.v1";

/// Serializes only the image already issued by the activated final view.
pub(crate) fn emit_lifecycle_physical_program_json(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<String, String> {
    serde_json::to_string(&emit_lifecycle_physical_program_value(program, None)?)
        .map_err(|error| fault(&format!("serialize:{error}")))
}

fn emit_lifecycle_physical_program_value(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    let births = birth_ordinals(program)?;
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
                                row.instruction(), &births,
                                diagnostic_site(abi_input, function_ordinal, block.id().0, row.index(), row.instruction())?,
                                abi_input.is_some(),
                            )?,
                        })) })
                        .collect::<Result<Vec<_>, String>>()?;
                    Ok(json!({
                        "id": block.id().0,
                        "instructions": instructions,
                        "terminator": {
                            "index": block.terminator().index(),
                            "instruction": encode_instruction(
                                block.terminator().instruction(), &births,
                                diagnostic_site(
                                    abi_input, function_ordinal, block.id().0,
                                    block.terminator().index(), block.terminator().instruction(),
                                )?,
                                abi_input.is_some(),
                            )?,
                        },
                        "edges": block.edges().iter().map(encode_edge).collect::<Vec<_>>(),
                    }))
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(json!({
                "name": function.name(),
                "role": function.role().wire_name(),
                "params": function.params().iter().map(value).collect::<Vec<_>>(),
                "entry": function.entry().0,
                "blocks": blocks,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(json!({ "schema": SCHEMA, "functions": functions }))
}

/// Extends the issued program transport with layout rows from the same final
/// view and the runtime-owned FaultFrame ABI revision.  It never reads MIR.
pub(crate) fn emit_lifecycle_physical_abi_json(
    input: &PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<String, String> {
    let mut root = emit_lifecycle_physical_program_value(input.program(), Some(input))?;
    let object = root.as_object_mut().ok_or_else(|| fault("program-root"))?;
    object.insert("process_result_site".into(), json!(input.process_result_site()));
    object.insert("fault_abi_version".into(), json!(input.fault_abi_version()));
    object.insert("storage_profile".into(), json!(input.storage_profile()));
    object.insert("layouts".into(), Value::Array(input.layouts().iter().map(|layout| json!({
        "object_id": layout.object_id(),
        "runtime_type_id": layout.runtime_type_id(),
        "field_count": layout.field_count(),
        "fields": layout.fields().iter().map(|field| json!({
            "declaration_ordinal": field.declaration_ordinal(),
            "runtime_slot": field.runtime_slot(),
            "storage_kind": field.storage_kind(),
        })).collect::<Vec<_>>(),
    })).collect()));
    serde_json::to_string(&root).map_err(|error| fault(&format!("serialize:{error}")))
}

fn birth_ordinals(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
) -> Result<BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>, String> {
    let mut result = BTreeMap::new();
    for (ordinal, function) in program.functions().iter().enumerate() {
        let Some(key) = function.role().birth_target() else { continue };
        let ordinal = u32::try_from(ordinal).map_err(|_| fault("function-ordinal"))?;
        if result.insert(key.clone(), ordinal).is_some() {
            return Err(fault("duplicate-birth-target"));
        }
    }
    Ok(result)
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
    instruction: &MirInstruction,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    diagnostic_site: Option<u64>,
    require_diagnostic_site: bool,
) -> Result<Value, String> {
    Ok(match instruction {
        MirInstruction::Const { dst, value: ConstValue::Integer(integer) } =>
            json!({ "op": "const_i64", "dst": value(dst), "value": integer }),
        MirInstruction::Const { dst, value: ConstValue::String(text) } =>
            json!({ "op": "const_string", "dst": value(dst), "value": text }),
        MirInstruction::Const { dst, value: ConstValue::Void } =>
            json!({ "op": "const_unit", "dst": value(dst) }),
        MirInstruction::BinOp { dst, op: BinaryOp::Add, lhs, rhs } =>
            json!({ "op": "add", "dst": value(dst), "lhs": value(lhs), "rhs": value(rhs) }),
        MirInstruction::Copy { dst, src } =>
            json!({ "op": "copy", "dst": value(dst), "src": value(src) }),
        MirInstruction::Phi { dst, inputs, .. } => json!({
            "op": "phi", "dst": value(dst),
            "inputs": inputs.iter().map(|(block, value)| json!({ "block": block.0, "value": value.0 })).collect::<Vec<_>>(),
        }),
        MirInstruction::ObjectFieldGet { dst, base, field } => json!({
            "op": "object_field_get", "dst": value(dst), "base": value(base),
            "object_id": field.object().declaration_index(), "field_ordinal": field.declaration_ordinal(),
        }),
        MirInstruction::Invoke { operation, fault_frame, normal_landing, fault_landing } => json!({
            "op": "invoke", "operation": encode_invoke(
                operation, births, diagnostic_site, require_diagnostic_site,
            )?,
            "fault_frame": value(fault_frame), "normal": normal_landing.0, "fault": fault_landing.0,
        }),
        MirInstruction::InvokeNormalResult { invoke_block, dst } =>
            json!({ "op": "invoke_normal_result", "invoke_block": invoke_block.0, "dst": value(dst) }),
        MirInstruction::ReturnFault { fault_frame } =>
            json!({ "op": "return_fault", "fault_frame": value(fault_frame) }),
        MirInstruction::FaultFrameEnter { dst, mode } => json!({
            "op": "fault_frame_enter", "dst": value(dst),
            "mode": match mode { FaultFrameMode::RootOwned => "root_owned", FaultFrameMode::Borrowed => "borrowed" },
        }),
        MirInstruction::Branch { condition, then_bb, else_bb, then_edge_args, else_edge_args } => json!({
            "op": "branch", "condition": value(condition), "then": then_bb.0, "else": else_bb.0,
            "then_args": then_edge_args.as_ref().map(encode_edge_args),
            "else_args": else_edge_args.as_ref().map(encode_edge_args),
        }),
        MirInstruction::Jump { target, edge_args } => json!({
            "op": "jump", "target": target.0, "args": edge_args.as_ref().map(encode_edge_args),
        }),
        MirInstruction::Return { value: result } =>
            json!({ "op": "return", "value": result.map(|value| value.0) }),
        MirInstruction::Call(call) =>
            json!({ "op": "birth_call", "call": encode_birth_call(call, births)? }),
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
    let Some(input) = abi_input else { return Ok(None) };
    let expected = super::physical_abi::PublishedLifecycleCheckedOperationKindV1::from_instruction(mir_instruction);
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
    diagnostic_site: Option<u64>,
    require_diagnostic_site: bool,
) -> Result<Value, String> {
    Ok(match operation {
        InvokeOperation::Call(call) => {
            if diagnostic_site.is_some() { return Err(fault("site-on-birth-call")); }
            json!({ "kind": "birth_call", "call": encode_birth_call(call, births)? })
        }
        InvokeOperation::NewBox { object } => with_site(json!({
            "kind": "new_box", "object_id": object.declaration_index(),
        }), required_site(diagnostic_site, require_diagnostic_site)?)?,
        InvokeOperation::FieldSet { field, base, value: stored } => with_site(json!({
            "kind": "field_set", "object_id": field.object().declaration_index(),
            "field_ordinal": field.declaration_ordinal(), "base": value(base), "value": value(stored),
        }), required_site(diagnostic_site, require_diagnostic_site)?)?,
        InvokeOperation::HomeRelease { object, value: released } =>
            with_site(json!({ "kind": "home_release", "object_id": object.declaration_index(), "value": value(released),
            }), required_site(diagnostic_site, require_diagnostic_site)?)?,
        InvokeOperation::ReclaimUnpublished { object, value: reclaimed } =>
            with_site(json!({ "kind": "reclaim_unpublished", "object_id": object.declaration_index(), "value": value(reclaimed),
            }), required_site(diagnostic_site, require_diagnostic_site)?)?,
    })
}

fn required_site(site: Option<u64>, required: bool) -> Result<Option<u64>, String> {
    if required && site.is_none() { return Err(fault("site-missing")); }
    Ok(site)
}

fn with_site(mut operation: Value, site: Option<u64>) -> Result<Value, String> {
    if let Some(site) = site {
        operation.as_object_mut().ok_or_else(|| fault("operation-object"))?
            .insert("site".into(), json!(site));
    }
    Ok(operation)
}

fn encode_birth_call(
    call: &crate::mir::definitions::MirCall,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
) -> Result<Value, String> {
    let Callee::BirthConstructor { key, receiver } = &call.callee else {
        return Err(fault("call-not-birth"));
    };
    let target = births.get(key).ok_or_else(|| fault("birth-target-foreign"))?;
    Ok(json!({
        "target": target, "receiver": value(receiver),
        "args": call.args.iter().map(|value| value.0).collect::<Vec<_>>(), "dst": call.dst.map(|value| value.0),
    }))
}

fn value(value: &ValueId) -> u32 { value.0 }

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-physical-json/{reason}]")
}

#[cfg(test)]
#[path = "physical_program_json_tests.rs"]
mod tests;
