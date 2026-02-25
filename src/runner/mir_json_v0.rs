use super::mir_json::common as mirjson_common;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    function::{FunctionSignature, MirFunction, MirModule},
    BarrierOp, BasicBlock, BasicBlockId, Callee, ConstValue, Effect, EffectMask, MirInstruction,
    MirType, TypeOpKind, ValueId, WeakRefOp,
};
use serde_json::Value;

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

fn require_u64(node: &Value, key: &str, context: &str) -> Result<u64, String> {
    node.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{} missing field '{}'", context, key))
}

fn parse_value_id_array(
    node: &Value,
    key: &str,
    element_context: &str,
) -> Result<Vec<ValueId>, String> {
    let values_v = node
        .get(key)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut values: Vec<ValueId> = Vec::with_capacity(values_v.len());
    for a in values_v {
        let id = a
            .as_u64()
            .ok_or_else(|| format!("{} must be integer", element_context))? as u32;
        values.push(ValueId::new(id));
    }
    Ok(values)
}

fn parse_function_param_ids(func: &Value, func_name: &str) -> Result<Vec<u32>, String> {
    let params = func
        .get("params")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut out = Vec::with_capacity(params.len());
    let mut seen = std::collections::BTreeSet::new();
    for (idx, p) in params.into_iter().enumerate() {
        let id = p.as_u64().ok_or_else(|| {
            format!(
                "function '{}' params[{}] must be integer value id",
                func_name, idx
            )
        })? as u32;
        if !seen.insert(id) {
            return Err(format!(
                "function '{}' params contains duplicate value id: {}",
                func_name, id
            ));
        }
        let expected = idx as u32;
        if id != expected {
            return Err(format!(
                "function '{}' params must be contiguous [0..N-1]: params[{}]={} expected {}",
                func_name, idx, id, expected
            ));
        }
        out.push(id);
    }
    Ok(out)
}

fn parse_call_callee(inst: &Value) -> Result<Option<Callee>, String> {
    let callee_obj = match inst.get("callee") {
        Some(obj) => obj,
        None => return Ok(None),
    };
    let callee_type = callee_obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| "call callee missing type".to_string())?;
    match callee_type {
        "Global" => {
            let name = callee_obj
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Global missing name".to_string())?
                .to_string();
            Ok(Some(Callee::Global(name)))
        }
        "Extern" => {
            let name = callee_obj
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Extern missing name".to_string())?
                .to_string();
            Ok(Some(Callee::Extern(name)))
        }
        "Method" => {
            let method = callee_obj
                .get("method")
                .or_else(|| callee_obj.get("name"))
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Method missing method/name".to_string())?
                .to_string();
            let box_name = callee_obj
                .get("box_name")
                .and_then(Value::as_str)
                .unwrap_or("RuntimeDataBox")
                .to_string();
            let receiver = callee_obj
                .get("receiver")
                .and_then(Value::as_u64)
                .map(|v| ValueId::new(v as u32));
            let certainty = if box_name == "RuntimeDataBox" {
                TypeCertainty::Union
            } else {
                TypeCertainty::Known
            };
            Ok(Some(Callee::Method {
                box_name,
                method,
                receiver,
                certainty,
                box_kind: CalleeBoxKind::RuntimeData,
            }))
        }
        "Constructor" => {
            let box_type = callee_obj
                .get("box_type")
                .or_else(|| callee_obj.get("name"))
                .and_then(Value::as_str)
                .ok_or_else(|| "call callee Constructor missing box_type/name".to_string())?
                .to_string();
            Ok(Some(Callee::Constructor { box_type }))
        }
        "Value" => {
            let value_id = callee_obj
                .get("value")
                .or_else(|| callee_obj.get("func"))
                .and_then(Value::as_u64)
                .ok_or_else(|| "call callee Value missing value/func".to_string())?
                as u32;
            Ok(Some(Callee::Value(ValueId::new(value_id))))
        }
        other => Err(format!("unsupported call callee.type '{}'", other)),
    }
}

fn build_call_instruction(
    inst: &Value,
    call_node: &Value,
    op_label: &str,
) -> Result<(MirInstruction, Option<ValueId>), String> {
    let callee = parse_call_callee(call_node)?;
    let func = if callee.is_some() {
        call_node
            .get("func")
            .and_then(Value::as_u64)
            .map(|v| ValueId::new(v as u32))
            .unwrap_or(ValueId::INVALID)
    } else {
        let ctx = format!("{} func", op_label);
        ValueId::new(require_u64(call_node, "func", &ctx)? as u32)
    };

    let dst_opt = inst
        .get("dst")
        .or_else(|| call_node.get("dst"))
        .and_then(Value::as_u64)
        .map(|v| ValueId::new(v as u32));
    let arg_ctx = format!("{} arg", op_label);
    let args = parse_value_id_array(call_node, "args", &arg_ctx)?;
    let effects = parse_call_effects(call_node)?;
    Ok((
        MirInstruction::Call {
            dst: dst_opt,
            func,
            callee,
            args,
            effects,
        },
        dst_opt,
    ))
}

fn parse_call_effects(node: &Value) -> Result<EffectMask, String> {
    let effects_v = match node.get("effects") {
        None => return Ok(EffectMask::READ),
        Some(v) => v,
    };
    if effects_v.is_null() {
        return Ok(EffectMask::READ);
    }

    if let Some(bits) = effects_v.as_u64() {
        if bits <= u16::MAX as u64 {
            let mask = EffectMask::from_bits(bits as u16);
            return Ok(if mask.bits() == 0 {
                EffectMask::READ
            } else {
                mask
            });
        }
        return Err(format!("call effects bits out of range: {}", bits));
    }

    let Some(arr) = effects_v.as_array() else {
        return Ok(EffectMask::READ);
    };
    if arr.is_empty() {
        return Ok(EffectMask::READ);
    }

    let mut mask = EffectMask::new();
    let mut parsed_any = false;
    for item in arr {
        if let Some(bits) = item.as_u64() {
            if bits <= u16::MAX as u64 {
                mask = mask.union(EffectMask::from_bits(bits as u16));
                parsed_any = true;
            }
            continue;
        }
        let Some(raw) = item.as_str() else {
            continue;
        };
        if let Some(mapped) = map_effect_name(raw) {
            mask = mask.union(mapped);
            parsed_any = true;
        }
    }

    if parsed_any {
        Ok(mask)
    } else {
        Ok(EffectMask::READ)
    }
}

fn map_effect_name(raw: &str) -> Option<EffectMask> {
    let lower = raw.to_ascii_lowercase();
    Some(match lower.as_str() {
        "pure" => EffectMask::PURE,
        "mut" => EffectMask::MUT,
        "io" => EffectMask::IO,
        "control" => EffectMask::CONTROL,
        "read" | "read_heap" => EffectMask::READ,
        "write" | "write_heap" => EffectMask::WRITE,
        "panic" => EffectMask::PANIC,
        "p2p" => EffectMask::P2P,
        "ffi" => EffectMask::from_bits(Effect::FFI as u16),
        "alloc" => EffectMask::from_bits(Effect::Alloc as u16),
        "global" => EffectMask::from_bits(Effect::Global as u16),
        "async" => EffectMask::from_bits(Effect::Async as u16),
        "unsafe" => EffectMask::from_bits(Effect::Unsafe as u16),
        "debug" => EffectMask::from_bits(Effect::Debug as u16),
        "barrier" => EffectMask::from_bits(Effect::Barrier as u16),
        _ => return None,
    })
}

fn parse_const_value(value_obj: &Value) -> Result<ConstValue, String> {
    // Delegate to common generic parser (int/float/bool/string/handle(StringBox))
    // Keeps behavior superset of previous (int-only) without changing existing callers.
    mirjson_common::parse_const_value_generic(value_obj).map_err(|e| format!("{}", e))
}

fn parse_compare(op: &str) -> Result<crate::mir::types::CompareOp, String> {
    use crate::mir::types::CompareOp;
    Ok(match op {
        "==" => CompareOp::Eq,
        "!=" => CompareOp::Ne,
        "<" => CompareOp::Lt,
        "<=" => CompareOp::Le,
        ">" => CompareOp::Gt,
        ">=" => CompareOp::Ge,
        s => return Err(format!("unsupported compare op '{}'", s)),
    })
}

fn parse_binop(op: &str) -> Result<crate::mir::types::BinaryOp, String> {
    use crate::mir::types::BinaryOp;
    Ok(match op {
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Sub,
        "*" => BinaryOp::Mul,
        "/" => BinaryOp::Div,
        "%" => BinaryOp::Mod,
        s => return Err(format!("unsupported binary op '{}'", s)),
    })
}

fn parse_barrier_op(raw: &str, field: &str) -> Result<BarrierOp, String> {
    match raw {
        "read" | "Read" => Ok(BarrierOp::Read),
        "write" | "Write" => Ok(BarrierOp::Write),
        _ => Err(format!("unsupported barrier {} '{}'", field, raw)),
    }
}

fn parse_typeop_kind(inst: &Value) -> Result<TypeOpKind, String> {
    let raw = inst
        .get("operation")
        .or_else(|| inst.get("op_kind"))
        .and_then(Value::as_str)
        .ok_or_else(|| "typeop missing operation/op_kind".to_string())?;
    if raw.eq_ignore_ascii_case("check") || raw.eq_ignore_ascii_case("is") {
        return Ok(TypeOpKind::Check);
    }
    if raw.eq_ignore_ascii_case("cast") || raw.eq_ignore_ascii_case("as") {
        return Ok(TypeOpKind::Cast);
    }
    Err(format!("unsupported typeop operation '{}'", raw))
}

fn parse_typeop_target_type(inst: &Value) -> Result<MirType, String> {
    let raw = inst
        .get("target_type")
        .or_else(|| inst.get("ty"))
        .and_then(Value::as_str)
        .ok_or_else(|| "typeop missing target_type/ty".to_string())?;
    let lower = raw.to_ascii_lowercase();
    Ok(match lower.as_str() {
        "integer" | "int" | "i64" | "integerbox" => MirType::Integer,
        "float" | "f64" | "floatbox" => MirType::Float,
        "bool" | "boolean" | "boolbox" => MirType::Bool,
        "string" | "str" | "stringbox" => MirType::String,
        "void" | "null" | "voidbox" | "nullbox" => MirType::Void,
        "weakref" => MirType::WeakRef,
        "array" | "arraybox" => MirType::Array(Box::new(MirType::Unknown)),
        "future" => MirType::Future(Box::new(MirType::Unknown)),
        "unknown" => MirType::Unknown,
        _ => MirType::Box(raw.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::parse_mir_v0_to_module;
    use crate::mir::{BasicBlockId, Callee, Effect, MirInstruction, ValueId};

    #[test]
    fn parse_call_accepts_extern_callee_without_func() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"const","dst":1,"value":{"type":"i64","value":7}},
                {"op":"call","dst":2,"callee":{"type":"Extern","name":"env.console.log"},"args":[1]},
                {"op":"ret","value":1}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert!(matches!(
            &insts[1],
            MirInstruction::Call {
                func,
                callee: Some(Callee::Extern(name)),
                args,
                dst: Some(dst),
                ..
            } if *func == ValueId::INVALID
                && name == "env.console.log"
                && args == &vec![ValueId::new(1)]
                && *dst == ValueId::new(2)
        ));
    }

    #[test]
    fn parse_call_accepts_method_callee_without_func() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"call","dst":4,"callee":{"type":"Method","box_name":"StringBox","method":"length","receiver":1},"args":[]},
                {"op":"ret","value":4}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert!(matches!(
            &insts[0],
            MirInstruction::Call {
                func,
                callee: Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
                dst: Some(dst),
                ..
            } if *func == ValueId::INVALID
                && box_name == "StringBox"
                && method == "length"
                && *receiver == ValueId::new(1)
                && *dst == ValueId::new(4)
        ));
    }

    #[test]
    fn parse_mir_call_accepts_nested_callee_shape() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Method","box_name":"StringBox","method":"length","receiver":1},"args":[],"effects":[]}},
                {"op":"ret","value":3}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert!(matches!(
            &insts[0],
            MirInstruction::Call {
                func,
                callee: Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    ..
                }),
                args,
                dst: Some(dst),
                ..
            } if *func == ValueId::INVALID
                && box_name == "StringBox"
                && method == "length"
                && *receiver == ValueId::new(1)
                && args.is_empty()
                && *dst == ValueId::new(3)
        ));
    }

    #[test]
    fn parse_mir_call_parses_effect_tokens() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"mir_call","dst":3,"mir_call":{"callee":{"type":"Extern","name":"env.console.log"},"args":[1],"effects":["io","write"]}},
                {"op":"ret","value":3}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert!(matches!(
            &insts[0],
            MirInstruction::Call { effects, .. }
                if effects.contains(Effect::Io) && effects.contains(Effect::WriteHeap)
        ));
    }

    #[test]
    fn parse_nop_is_lowered_away() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"const","dst":1,"value":{"type":"i64","value":7}},
                {"op":"nop"},
                {"op":"ret","value":1}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;
        assert_eq!(insts.len(), 1, "nop must be lowered away");
        assert!(matches!(
            &insts[0],
            MirInstruction::Const { dst, .. } if *dst == ValueId::new(1)
        ));
    }

    #[test]
    fn parse_debug_log_canonicalizes_to_debug_sequence() {
        let json = r#"{
          "functions":[
            {"name":"main","blocks":[
              {"id":0,"instructions":[
                {"op":"const","dst":1,"value":{"type":"i64","value":7}},
                {"op":"const","dst":2,"value":{"type":"i64","value":8}},
                {"op":"debug_log","message":"probe","values":[1,2]},
                {"op":"ret","value":1}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module.get_function("main").expect("main exists");
        let insts = &func
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("bb0 exists")
            .instructions;

        assert!(matches!(
            &insts[2],
            MirInstruction::Debug { value, message }
                if *value == ValueId::new(1) && message == "probe[0]"
        ));
        assert!(matches!(
            &insts[3],
            MirInstruction::Debug { value, message }
                if *value == ValueId::new(2) && message == "probe[1]"
        ));
    }

    #[test]
    fn parse_params_restores_valueid_zero_as_parameter() {
        let json = r#"{
          "functions":[
            {"name":"AddOperator.apply/2","params":[0,1],"blocks":[
              {"id":0,"instructions":[
                {"op":"copy","dst":2,"src":0},
                {"op":"ret","value":2}
              ]}
            ]}
          ]
        }"#;

        let module = parse_mir_v0_to_module(json).expect("must parse");
        let func = module
            .get_function("AddOperator.apply/2")
            .expect("function exists");

        assert_eq!(
            func.params,
            vec![ValueId::new(0), ValueId::new(1)],
            "params must preserve JSON parameter ids so src=0 is defined"
        );
        assert!(
            func.next_value_id >= 3,
            "next_value_id must be above dst/param range"
        );
    }

    #[test]
    fn parse_params_rejects_non_contiguous_ids() {
        let json = r#"{
          "functions":[
            {"name":"main","params":[1,2],"blocks":[
              {"id":0,"instructions":[
                {"op":"ret","value":1}
              ]}
            ]}
          ]
        }"#;

        let err = parse_mir_v0_to_module(json).expect_err("must reject non-contiguous params");
        assert!(
            err.contains("params must be contiguous [0..N-1]"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn parse_params_rejects_duplicate_ids() {
        let json = r#"{
          "functions":[
            {"name":"main","params":[0,0],"blocks":[
              {"id":0,"instructions":[
                {"op":"ret","value":0}
              ]}
            ]}
          ]
        }"#;

        let err = parse_mir_v0_to_module(json).expect_err("must reject duplicated params");
        assert!(
            err.contains("params contains duplicate"),
            "unexpected error: {err}"
        );
    }
}
