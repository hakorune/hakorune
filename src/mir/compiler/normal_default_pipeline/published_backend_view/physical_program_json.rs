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
use super::physical_program::{
    PublishedLifecyclePhysicalFunctionV1, PublishedLifecyclePhysicalProgramV1,
};

#[path = "physical_program_json_call_transport.rs"]
mod call_transport;
#[path = "physical_program_json_object_return.rs"]
mod object_return;
#[path = "physical_program_json_numeric.rs"]
mod numeric;
#[path = "physical_program_json_operation.rs"]
mod operation;
use operation::encode_invoke;
#[cfg(test)]
pub(super) use object_return::encode_object_return;
#[cfg(test)]
pub(super) use call_transport::corroborate_function as corroborate_borrowed_function_snapshot;

const SCHEMA: &str = "hako.published-lifecycle-physical-program.v2";

pub(super) fn emit_lifecycle_physical_program_value(
    program: &PublishedLifecyclePhysicalProgramV1<'_>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
) -> Result<Value, String> {
    call_transport::verify_borrowed_projection(program, abi_input)?;
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
                                Some((program.functions(), function, (block.id(), row.index()))),
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
                                Some((program.functions(), function, (block.id(), block.terminator().index()))),
                            )?,
                        },
                        "edges": block.edges().iter().map(encode_edge).collect::<Vec<_>>(),
                    }))
                })
                .collect::<Result<Vec<_>, String>>()?;
            let mut wire = json!({
                "name": function.name(),
                "role": function.role().wire_name(),
                "receiver": function.role().receiver_value(function.params()).map(|value| value.0),
                "receiver_object": function
                    .role()
                    .receiver_object()
                    .map(|object| object.declaration_index()),
                "params": call_transport::encode_parameters(function, function_ordinal, abi_input)?,
                "entry": function.entry().0,
                "blocks": blocks,
            });
            if let Some(result) = function.role().ordinary_result() {
                call_transport::encode_result_support_v1(result, &mut wire)?;
            }
            Ok(wire)
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
                            "owned_residences": layout.owned_residences(),
                            "owned_object_residences": layout.owned_object_residences().iter()
                                .map(|(ordinal, child)| json!({"field_ordinal": ordinal, "child_object_id": child}))
                                .collect::<Vec<_>>(),
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

/// The ordinary-call corroboration pair: the published function table and
/// the caller row whose sealed `value_types` names each actual. Only the
/// `InvokeOperation::Call` arm consumes it.
type CallContext<'module> = (
    &'module [PublishedLifecyclePhysicalFunctionV1<'module>],
    &'module PublishedLifecyclePhysicalFunctionV1<'module>,
    (crate::mir::BasicBlockId, u32),
);

#[allow(clippy::too_many_arguments)]
fn encode_instruction(
    field_ref: Option<CanonicalFieldRefV1>,
    instruction: &MirInstruction,
    births: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    caller_function_index: u32,
    diagnostic_site: Option<u64>,
    abi_input: Option<&PublishedLifecyclePhysicalAbiInputV1<'_>>,
    ordinary: &BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, u32>,
    call_context: Option<CallContext<'_>>,
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
            value: ConstValue::Null,
        } => json!({ "op": "const_null", "dst": value(dst) }),
        MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        } => json!({ "op": "const_unit", "dst": value(dst) }),
        MirInstruction::BinOp { dst, op, lhs, rhs } => {
            let operator = match op {
                BinaryOp::Add => "add", BinaryOp::Mul => "mul", BinaryOp::Sub => "sub",
                BinaryOp::Div => "div", BinaryOp::Mod => "mod",
                _ => return Err(fault("instruction-unsupported")),
            };
            json!({ "op": operator, "dst": value(dst), "lhs": value(lhs), "rhs": value(rhs) })
        }
        MirInstruction::Compare { dst, op, lhs, rhs } => {
            let predicate = match op {
                crate::mir::CompareOp::Eq => "eq",
                crate::mir::CompareOp::Ne => "ne",
                crate::mir::CompareOp::Lt => "slt",
                crate::mir::CompareOp::Le => "sle",
                crate::mir::CompareOp::Gt => "sgt",
                crate::mir::CompareOp::Ge => "sge",
            };
            // Exactly one operand produced by the `ConstValue::Null` row
            // makes this the borrowed/null equality, never an integer
            // compare: the physical row keeps the operands in source order
            // and the emitter reads the carrier's kind lane only. Without
            // the function context (serializer tests) the row keeps the
            // ordinary spelling.
            let null_equality =
                matches!(*op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
                    && call_context.is_some_and(|(_, function, _)| {
                        null_producer(function, *lhs) ^ null_producer(function, *rhs)
                    });
            if null_equality {
                json!({ "op": "borrowed_null_compare", "dst": value(dst), "lhs": value(lhs),
                    "rhs": value(rhs), "predicate": predicate })
            } else {
                json!({ "op": "compare", "dst": value(dst), "lhs": value(lhs),
                    "rhs": value(rhs), "predicate": predicate })
            }
        }
        // Optimizer-only SSA liveness hint; no Home, lease or runtime retain.
        MirInstruction::KeepAlive { values } => json!({ "op": "keep_alive", "values": values.iter().map(|value| value.0).collect::<Vec<_>>() }),
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
        MirInstruction::FieldSet {
            base,
            value: stored,
            ..
        } => {
            let field = field_ref.ok_or_else(|| fault("field-set-route-missing"))?;
            let site = required_site(diagnostic_site, abi_input.is_some())?;
            let runtime_check = numeric::range_check(abi_input, caller_function_index, call_context);
            json!({
                "op": "field_set", "base": value(base),
                "object_id": field.object().declaration_index(),
                "field_ordinal": field.declaration_ordinal(),
                "value": value(stored), "site": site,
                "exact_numeric_runtime_check": runtime_check,
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
                call_context,
            )?,
            "fault_frame": value(fault_frame), "normal": normal_landing.0, "fault": fault_landing.0,
        }),
        MirInstruction::InvokeNormalResult { invoke_block, dst } => {
            json!({ "op": "invoke_normal_result", "invoke_block": invoke_block.0, "dst": value(dst) })
        }
        MirInstruction::ArrayElementWrite {
            kind: crate::mir::ArrayElementWriteKind::Set,
            producer: crate::mir::ArrayWriteProducerKind::MethodCall,
            receiver,
            index: Some(index),
            value: stored,
            ..
        } => {
            let site = required_site(diagnostic_site, abi_input.is_some())?;
            json!({ "op": "array_set", "array": value(receiver),
                "index": value(index), "value": value(stored), "site": site })
        }
        MirInstruction::ArrayRead {
            dst,
            receiver,
            operation: crate::mir::instruction::ArrayReadOperationV1::Element { index },
            ..
        } => {
            json!({ "op": "array_get", "array": value(receiver),
                "index": value(index), "dst": dst.map(|value| value.0) })
        }
        MirInstruction::ArrayRead { dst, receiver,
            operation: crate::mir::instruction::ArrayReadOperationV1::Count, ..
        } => json!({ "op": "array_length", "array": value(receiver), "dst": dst.map(|value| value.0) }),
        MirInstruction::StringResidenceInstall { value: installed, replaces, returning } => {
            let input = abi_input.ok_or_else(|| fault("string-residence-input-missing"))?;
            let (_, caller, _) = call_context.ok_or_else(|| fault("string-residence-caller-missing"))?;
            call_transport::corroborate_function(caller, caller_function_index, input)?;
            let mut event = json!({ "op": "string_residence_install", "value": value(installed) });
            if let Some(previous) = replaces { event["replaces"] = json!(value(previous)); }
            if *returning { event["returning"] = json!(true); }
            event
        }
        MirInstruction::ArrayResidenceRelease { value: released } => {
            // The issued function already checked native Script or exact
            // owned-holder source membership. Borrow that SAME ABI function;
            // a Script-only flag cannot represent the owned Claim Fault path.
            let input = abi_input.ok_or_else(|| fault("array-residence-input-missing"))?;
            let (_, caller, _) = call_context.ok_or_else(|| fault("array-residence-caller-missing"))?;
            call_transport::corroborate_function(caller, caller_function_index, input)?;
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
            let mut wire = json!({ "op": "return", "value": result.map(|value| value.0) });
            if let Some((_, function, coordinate)) = call_context {
                if let Some(origin) = object_return::encode_object_return(function, coordinate, instruction)? {
                    wire["result_origin"] = origin;
                }
            }
            wire
        }
        MirInstruction::Call(call) => {
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                return Err(fault("call-not-birth"));
            };
            let birth = crate::mir::instruction::BirthInvokeV1::from_value_arguments(
                key.clone(), *receiver, call.args.clone(), call.effects);
            let mut wire = call_transport::encode_birth_call(&birth, births, caller_function_index, abi_input)?;
            // Preserve the raw Compatibility row exactly; its destination is
            // never silently erased by the checked Unit-only representation.
            wire["dst"] = json!(call.dst.map(|value| value.0));
            json!({ "op": "birth_call", "call": wire })
        }
        _ => return Err(fault("instruction-unsupported")),
    })
}

/// Whether `operand`'s producer inside `function` is the exact
/// `ConstValue::Null` row — the null sentinel materialization, never a
/// borrowed carrier or a forged zero.
fn null_producer(function: &PublishedLifecyclePhysicalFunctionV1<'_>, operand: ValueId) -> bool {
    function.blocks().iter().any(|block| {
        block
            .instructions()
            .iter()
            .copied()
            .chain(std::iter::once(block.terminator()))
            .any(|row| {
                matches!(
                    row.instruction(),
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Null
                    } if *dst == operand
                )
            })
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

fn required_site(site: Option<u64>, required: bool) -> Result<Option<u64>, String> {
    if required && site.is_none() {
        return Err(fault("site-missing"));
    }
    Ok(site)
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
