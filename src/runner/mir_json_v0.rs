use super::mir_json::common as mirjson_common;
use crate::mir::{
    function::{FunctionSignature, MirFunction, MirModule},
    BasicBlock, BasicBlockId, Callee, EffectMask, MirInstruction, MirType, ValueId, WeakRefOp,
};
use serde_json::Value;

#[path = "mir_json_v0/call.rs"]
mod call;
#[path = "mir_json_v0/helpers.rs"]
mod helpers;
#[cfg(test)]
#[path = "mir_json_v0/tests.rs"]
mod tests;

use call::*;
use helpers::*;

/// Parse minimal MIR JSON v0 (no schema_version, root has `functions` and each function has `blocks`).
/// Supported ops (minimal): const, copy, load, array_get, array_set, store, binop, compare, typeop, ref_new, weak_new, weak_load, future_new, future_set, await, branch, jump, phi, ret, newbox, boxcall, call, mir_call, externcall, safepoint, keepalive, release_strong, debug, select, barrier.
pub fn parse_mir_v0_to_module(json: &str) -> Result<MirModule, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("invalid JSON: {}", e))?;
    let functions = value
        .get("functions")
        .and_then(|f| f.as_array())
        .ok_or_else(|| "JSON missing functions array".to_string())?;

    let mut module = MirModule::new("mir_json_v0".to_string());

    for func in functions {
        let func_name = func
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("main")
            .to_string();

        let blocks = func
            .get("blocks")
            .and_then(|b| b.as_array())
            .ok_or_else(|| format!("function '{}' missing blocks array", func_name))?;

        if blocks.is_empty() {
            return Err(format!("function '{}' has no blocks", func_name));
        }

        let entry_id = blocks
            .get(0)
            .and_then(|b| b.get("id"))
            .and_then(|id| id.as_u64())
            .ok_or_else(|| format!("function '{}' entry block missing id", func_name))?
            as u32;
        let entry_bb = BasicBlockId::new(entry_id);

        let param_value_ids = parse_function_param_ids(func, &func_name)?;
        let param_count = param_value_ids.len();

        let mut signature = FunctionSignature {
            name: func_name.clone(),
            // Preserve parameter arity from JSON route so ValueId(0..N-1)
            // are initialized as function parameters in VM execution.
            params: vec![MirType::Unknown; param_count],
            return_type: MirType::Unknown,
            effects: crate::mir::EffectMask::PURE,
        };
        let mut mir_fn = MirFunction::new(signature.clone(), entry_bb);
        if !param_value_ids.is_empty() {
            mir_fn.params = param_value_ids.iter().copied().map(ValueId::new).collect();
        }
        let mut max_value_id: u32 = param_value_ids
            .iter()
            .copied()
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);

        for block in blocks {
            let block_id = block
                .get("id")
                .and_then(|id| id.as_u64())
                .ok_or_else(|| format!("function '{}' block missing id", func_name))?
                as u32;
            let bb_id = BasicBlockId::new(block_id);
            if mir_fn.get_block(bb_id).is_none() {
                mir_fn.add_block(BasicBlock::new(bb_id));
            }
            let block_ref = mir_fn
                .get_block_mut(bb_id)
                .expect("block must exist after insertion");

            let instructions = block
                .get("instructions")
                .and_then(|insts| insts.as_array())
                .ok_or_else(|| {
                    format!(
                        "function '{}' block {} missing instructions array",
                        func_name, block_id
                    )
                })?;

            for inst in instructions {
                let op = inst.get("op").and_then(|o| o.as_str()).ok_or_else(|| {
                    format!(
                        "function '{}' block {} missing op field",
                        func_name, block_id
                    )
                })?;
                match op {
                    "const" => {
                        let dst = require_u64(inst, "dst", "const dst")? as u32;
                        let vobj = inst
                            .get("value")
                            .ok_or_else(|| "const missing value".to_string())?;
                        let val = parse_const_value(vobj)?;
                        block_ref.add_instruction(MirInstruction::Const {
                            dst: ValueId::new(dst),
                            value: val,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "copy" => {
                        let dst = require_u64(inst, "dst", "copy dst")? as u32;
                        let src = require_u64(inst, "src", "copy src")? as u32;
                        block_ref.add_instruction(MirInstruction::Copy {
                            dst: ValueId::new(dst),
                            src: ValueId::new(src),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "load" => {
                        let dst = require_u64(inst, "dst", "load dst")? as u32;
                        let ptr = require_u64(inst, "ptr", "load ptr")? as u32;
                        block_ref.add_instruction(MirInstruction::Load {
                            dst: ValueId::new(dst),
                            ptr: ValueId::new(ptr),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "array_get" => {
                        let dst = require_u64(inst, "dst", "array_get dst")? as u32;
                        let array = require_u64(inst, "array", "array_get array")? as u32;
                        let _index = require_u64(inst, "index", "array_get index")? as u32;
                        // Legacy ArrayGet shim for vm-hako JSON parity lane:
                        // model as copy from array register (index is shape-validated).
                        block_ref.add_instruction(MirInstruction::Copy {
                            dst: ValueId::new(dst),
                            src: ValueId::new(array),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "array_set" => {
                        let array = require_u64(inst, "array", "array_set array")? as u32;
                        let _index = require_u64(inst, "index", "array_set index")? as u32;
                        let value = require_u64(inst, "value", "array_set value")? as u32;
                        // Legacy ArraySet shim for vm-hako JSON parity lane:
                        // model as register-slot write (index is shape-validated only).
                        block_ref.add_instruction(MirInstruction::Copy {
                            dst: ValueId::new(array),
                            src: ValueId::new(value),
                        });
                        max_value_id = max_value_id.max(array + 1);
                    }
                    "store" => {
                        let ptr = require_u64(inst, "ptr", "store ptr")? as u32;
                        let value = require_u64(inst, "value", "store value")? as u32;
                        block_ref.add_instruction(MirInstruction::Store {
                            ptr: ValueId::new(ptr),
                            value: ValueId::new(value),
                        });
                    }
                    "binop" => {
                        let dst = require_u64(inst, "dst", "binop dst")? as u32;
                        let lhs = require_u64(inst, "lhs", "binop lhs")? as u32;
                        let rhs = require_u64(inst, "rhs", "binop rhs")? as u32;
                        let operation = inst
                            .get("operation")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "binop missing operation".to_string())?;
                        let bop = parse_binop(operation)?;
                        block_ref.add_instruction(MirInstruction::BinOp {
                            dst: ValueId::new(dst),
                            op: bop,
                            lhs: ValueId::new(lhs),
                            rhs: ValueId::new(rhs),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "compare" => {
                        let dst = require_u64(inst, "dst", "compare dst")? as u32;
                        let lhs = require_u64(inst, "lhs", "compare lhs")? as u32;
                        let rhs = require_u64(inst, "rhs", "compare rhs")? as u32;
                        let operation = inst
                            .get("operation")
                            .and_then(Value::as_str)
                            .unwrap_or("==");
                        let cop = parse_compare(operation)?;
                        block_ref.add_instruction(MirInstruction::Compare {
                            dst: ValueId::new(dst),
                            op: cop,
                            lhs: ValueId::new(lhs),
                            rhs: ValueId::new(rhs),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "typeop" => {
                        let dst = require_u64(inst, "dst", "typeop dst")? as u32;
                        let src = inst
                            .get("src")
                            .or_else(|| inst.get("value"))
                            .and_then(Value::as_u64)
                            .ok_or_else(|| "typeop missing src/value".to_string())?
                            as u32;
                        let op = parse_typeop_kind(inst)?;
                        let ty = parse_typeop_target_type(inst)?;
                        block_ref.add_instruction(MirInstruction::TypeOp {
                            dst: ValueId::new(dst),
                            op,
                            value: ValueId::new(src),
                            ty,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "variant_make" => {
                        let dst = require_u64(inst, "dst", "variant_make dst")? as u32;
                        let enum_name = inst
                            .get("enum")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "variant_make missing enum".to_string())?
                            .to_string();
                        let variant = inst
                            .get("variant")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "variant_make missing variant".to_string())?
                            .to_string();
                        let tag = require_u64(inst, "tag", "variant_make tag")? as u32;
                        let payload = inst
                            .get("payload")
                            .and_then(Value::as_u64)
                            .map(|value| ValueId::new(value as u32));
                        let payload_type = parse_optional_type_name(inst, "payload_type")?;
                        block_ref.add_instruction(MirInstruction::VariantMake {
                            dst: ValueId::new(dst),
                            enum_name,
                            variant,
                            tag,
                            payload,
                            payload_type,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "variant_tag" => {
                        let dst = require_u64(inst, "dst", "variant_tag dst")? as u32;
                        let value = require_u64(inst, "value", "variant_tag value")? as u32;
                        let enum_name = inst
                            .get("enum")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "variant_tag missing enum".to_string())?
                            .to_string();
                        block_ref.add_instruction(MirInstruction::VariantTag {
                            dst: ValueId::new(dst),
                            value: ValueId::new(value),
                            enum_name,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "variant_project" => {
                        let dst = require_u64(inst, "dst", "variant_project dst")? as u32;
                        let value = require_u64(inst, "value", "variant_project value")? as u32;
                        let enum_name = inst
                            .get("enum")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "variant_project missing enum".to_string())?
                            .to_string();
                        let variant = inst
                            .get("variant")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "variant_project missing variant".to_string())?
                            .to_string();
                        let tag = require_u64(inst, "tag", "variant_project tag")? as u32;
                        let payload_type = parse_optional_type_name(inst, "payload_type")?;
                        block_ref.add_instruction(MirInstruction::VariantProject {
                            dst: ValueId::new(dst),
                            value: ValueId::new(value),
                            enum_name,
                            variant,
                            tag,
                            payload_type,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "ref_new" => {
                        let dst = require_u64(inst, "dst", "ref_new dst")? as u32;
                        let box_val = require_u64(inst, "box_val", "ref_new box_val")? as u32;
                        block_ref.add_instruction(MirInstruction::RefNew {
                            dst: ValueId::new(dst),
                            box_val: ValueId::new(box_val),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "weak_new" => {
                        let dst = require_u64(inst, "dst", "weak_new dst")? as u32;
                        let box_val = require_u64(inst, "box_val", "weak_new box_val")? as u32;
                        block_ref.add_instruction(MirInstruction::WeakRef {
                            dst: ValueId::new(dst),
                            op: WeakRefOp::New,
                            value: ValueId::new(box_val),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "weak_load" => {
                        let dst = require_u64(inst, "dst", "weak_load dst")? as u32;
                        let weak_ref = require_u64(inst, "weak_ref", "weak_load weak_ref")? as u32;
                        block_ref.add_instruction(MirInstruction::WeakRef {
                            dst: ValueId::new(dst),
                            op: WeakRefOp::Load,
                            value: ValueId::new(weak_ref),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "future_new" => {
                        let dst = require_u64(inst, "dst", "future_new dst")? as u32;
                        let value = require_u64(inst, "value", "future_new value")? as u32;
                        block_ref.add_instruction(MirInstruction::FutureNew {
                            dst: ValueId::new(dst),
                            value: ValueId::new(value),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "future_set" => {
                        let future = require_u64(inst, "future", "future_set future")? as u32;
                        let value = require_u64(inst, "value", "future_set value")? as u32;
                        block_ref.add_instruction(MirInstruction::FutureSet {
                            future: ValueId::new(future),
                            value: ValueId::new(value),
                        });
                    }
                    "await" => {
                        let dst = require_u64(inst, "dst", "await dst")? as u32;
                        let future = require_u64(inst, "future", "await future")? as u32;
                        block_ref.add_instruction(MirInstruction::Await {
                            dst: ValueId::new(dst),
                            future: ValueId::new(future),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "branch" => {
                        let cond = require_u64(inst, "cond", "branch cond")? as u32;
                        let then_bb = require_u64(inst, "then", "branch then")? as u32;
                        let else_bb = require_u64(inst, "else", "branch else")? as u32;
                        block_ref.add_instruction(MirInstruction::Branch {
                            condition: ValueId::new(cond),
                            then_bb: BasicBlockId::new(then_bb),
                            else_bb: BasicBlockId::new(else_bb),
                            then_edge_args: None,
                            else_edge_args: None,
                        });
                    }
                    "jump" => {
                        let target = require_u64(inst, "target", "jump target")? as u32;
                        block_ref.add_instruction(MirInstruction::Jump {
                            target: BasicBlockId::new(target),
                            edge_args: None,
                        });
                    }
                    "phi" => {
                        let dst = require_u64(inst, "dst", "phi dst")? as u32;
                        let pairs = mirjson_common::parse_phi_incoming_generic(inst)?;
                        block_ref.add_instruction(MirInstruction::Phi {
                            dst: ValueId::new(dst),
                            inputs: pairs,
                            type_hint: None, // Phase 63-6
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "ret" => {
                        let value = inst
                            .get("value")
                            .and_then(|v| v.as_u64())
                            .map(|v| ValueId::new(v as u32));
                        block_ref.add_instruction(MirInstruction::Return { value });
                        if let Some(val) = value {
                            signature.return_type = MirType::Integer;
                            max_value_id = max_value_id.max(val.as_u32() + 1);
                        } else {
                            signature.return_type = MirType::Void;
                        }
                    }
                    "newbox" => {
                        let dst = require_u64(inst, "dst", "newbox dst")? as u32;
                        let ty = inst
                            .get("type")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "newbox missing type".to_string())?
                            .to_string();
                        let args = parse_value_id_array(inst, "args", "newbox arg")?;
                        block_ref.add_instruction(MirInstruction::NewBox {
                            dst: ValueId::new(dst),
                            box_type: ty,
                            args,
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    "boxcall" => {
                        // { op:"boxcall", box:<vid>, method:"name", args:[vid...], dst?:<vid> }
                        let box_id = require_u64(inst, "box", "boxcall box")? as u32;
                        let method = inst
                            .get("method")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "boxcall missing method".to_string())?
                            .to_string();
                        let dst_opt = inst
                            .get("dst")
                            .and_then(Value::as_u64)
                            .map(|v| ValueId::new(v as u32));
                        let args = parse_value_id_array(inst, "args", "boxcall arg")?;
                        let box_name = inst
                            .get("box_name")
                            .and_then(Value::as_str)
                            .unwrap_or("RuntimeDataBox")
                            .to_string();
                        block_ref.add_instruction(MirInstruction::Call {
                            dst: dst_opt,
                            func: ValueId::INVALID,
                            callee: Some(Callee::Method {
                                box_name,
                                method,
                                receiver: Some(ValueId::new(box_id)),
                                certainty:
                                    crate::mir::definitions::call_unified::TypeCertainty::Union,
                                box_kind:
                                    crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                            }),
                            args,
                            effects: EffectMask::READ,
                        });
                        if let Some(dv) = dst_opt {
                            max_value_id = max_value_id.max(dv.as_u32() + 1);
                        }
                    }
                    "call" => {
                        let (call_inst, dst_opt) = build_call_instruction(inst, inst, "call")?;
                        block_ref.add_instruction(call_inst);
                        if let Some(dv) = dst_opt {
                            max_value_id = max_value_id.max(dv.as_u32() + 1);
                        }
                    }
                    "mir_call" => {
                        // Unified call JSON (flat or nested):
                        //  - { op:"mir_call", callee:{...}, args:[...], effects:[...], dst?:<vid|null> }
                        //  - { op:"mir_call", mir_call:{callee:{...}, args:[...], effects:[...]}, dst?:<vid|null> }
                        let call_node = inst.get("mir_call").unwrap_or(inst);
                        let (call_inst, dst_opt) =
                            build_call_instruction(inst, call_node, "mir_call")?;
                        block_ref.add_instruction(call_inst);
                        if let Some(dv) = dst_opt {
                            max_value_id = max_value_id.max(dv.as_u32() + 1);
                        }
                    }
                    "externcall" => {
                        // { op:"externcall", func:"iface.method", args:[vid...], dst?:<vid|null> }
                        let func = inst
                            .get("func")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "externcall missing func".to_string())?;
                        let (iface_name, method_name) =
                            if let Some(rest) = func.strip_prefix("nyash.console.") {
                                ("env.console".to_string(), rest.to_string())
                            } else if let Some((iface, method)) = func.rsplit_once('.') {
                                (iface.to_string(), method.to_string())
                            } else {
                                return Err(format!("externcall func must contain '.': {}", func));
                            };
                        let dst_opt = inst
                            .get("dst")
                            .and_then(Value::as_u64)
                            .map(|v| ValueId::new(v as u32));
                        let args = parse_value_id_array(inst, "args", "externcall arg")?;
                        block_ref.add_instruction(crate::mir::ssot::extern_call::extern_call(
                            dst_opt,
                            iface_name,
                            method_name,
                            args,
                            EffectMask::IO,
                        ));
                        if let Some(dv) = dst_opt {
                            max_value_id = max_value_id.max(dv.as_u32() + 1);
                        }
                    }
                    // Retired op: accepted for backward compatibility and lowered away.
                    "nop" => {}
                    "safepoint" => {
                        block_ref.add_instruction(MirInstruction::Safepoint);
                    }
                    "keepalive" => {
                        let values = parse_value_id_array(inst, "values", "keepalive value")?;
                        block_ref.add_instruction(MirInstruction::KeepAlive { values });
                    }
                    "release_strong" => {
                        let values = parse_value_id_array(inst, "values", "release_strong value")?;
                        block_ref.add_instruction(MirInstruction::ReleaseStrong { values });
                    }
                    "debug" => {
                        let value = require_u64(inst, "value", "debug value")? as u32;
                        let message = inst
                            .get("message")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "debug missing message".to_string())?
                            .to_string();
                        block_ref.add_instruction(MirInstruction::Debug {
                            value: ValueId::new(value),
                            message,
                        });
                    }
                    // Retired op: canonicalize to one or more Debug instructions.
                    "debug_log" => {
                        let message = inst
                            .get("message")
                            .and_then(Value::as_str)
                            .ok_or_else(|| "debug_log missing message".to_string())?
                            .to_string();
                        let values = parse_value_id_array(inst, "values", "debug_log value")?;
                        for (idx, value) in values.iter().copied().enumerate() {
                            let per_value_message = if values.len() <= 1 {
                                message.clone()
                            } else {
                                format!("{}[{}]", message, idx)
                            };
                            block_ref.add_instruction(MirInstruction::Debug {
                                value,
                                message: per_value_message,
                            });
                        }
                    }
                    "barrier" => {
                        let ptr = require_u64(inst, "ptr", "barrier ptr")? as u32;
                        let barrier_kind = inst.get("kind").and_then(Value::as_str).unwrap_or("");
                        let op_kind = inst.get("op_kind").and_then(Value::as_str).unwrap_or("");
                        let op = if !barrier_kind.is_empty() {
                            parse_barrier_op(barrier_kind, "kind")?
                        } else if !op_kind.is_empty() {
                            parse_barrier_op(op_kind, "op_kind")?
                        } else {
                            return Err("barrier missing kind/op_kind".to_string());
                        };
                        block_ref.add_instruction(MirInstruction::Barrier {
                            op,
                            ptr: ValueId::new(ptr),
                        });
                    }
                    "select" => {
                        let dst = require_u64(inst, "dst", "select dst")? as u32;
                        let cond = require_u64(inst, "cond", "select cond")? as u32;
                        let then_val = require_u64(inst, "then_val", "select then_val")? as u32;
                        let else_val = require_u64(inst, "else_val", "select else_val")? as u32;
                        block_ref.add_instruction(MirInstruction::Select {
                            dst: ValueId::new(dst),
                            cond: ValueId::new(cond),
                            then_val: ValueId::new(then_val),
                            else_val: ValueId::new(else_val),
                        });
                        max_value_id = max_value_id.max(dst + 1);
                    }
                    other => {
                        return Err(format!("unsupported op '{}' in mir_json_v0 loader", other));
                    }
                }
            }
        }

        // Set max value id (best effort)
        mir_fn.next_value_id = max_value_id;
        module.functions.insert(func_name.clone(), mir_fn);
    }

    // Canonicalize legacy callsites from selfhost JSON route before VM preflight.
    // This keeps runtime acceptance aligned with MCL lane (BoxCall/ExternCall -> Call(callee=...)).
    let _ = crate::mir::passes::callsite_canonicalize::canonicalize_callsites(&mut module);

    Ok(module)
}
