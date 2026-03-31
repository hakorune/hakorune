use super::helpers::{parse_binop, parse_compare, parse_effects_from, require_u64};
use crate::mir::{
    function::{FunctionSignature, MirFunction, MirModule},
    BasicBlock, BasicBlockId, EffectMask, MirInstruction, MirType, ValueId,
};
use serde_json::Value;

use super::super::mir_json::common as mirjson_common;

#[cfg(test)]
mod tests;

fn infer_param_count_from_v1_func(func: &Value, func_name: &str) -> Result<usize, String> {
    if let Some(params) = func.get("params").and_then(Value::as_array) {
        for (idx, p) in params.iter().enumerate() {
            let pid = p
                .as_u64()
                .or_else(|| p.get("id").and_then(Value::as_u64))
                .ok_or_else(|| {
                    format!(
                        "function '{}' params[{}] must be integer id (or object with id)",
                        func_name, idx
                    )
                })?;
            if pid != idx as u64 {
                return Err(format!(
                    "[freeze:contract][json_v1_bridge/params] function '{}' params must be canonical [0..N-1]; got id {} at index {}",
                    func_name, pid, idx
                ));
            }
        }
        return Ok(params.len());
    }
    if let Some((_box_name, _method, arity)) = crate::mir::naming::decode_static_method(func_name) {
        return Ok(arity);
    }
    Ok(0)
}

/// Try to parse MIR JSON v1 schema into a MIR module.
/// Returns Ok(None) when the input is not v1 (schema_version missing).
/// Currently supports a minimal subset required for Gate-C parity tests:
/// - const (integer)
/// - copy
/// - ret
#[allow(dead_code)]
pub fn try_parse_v1_to_module(json: &str) -> Result<Option<MirModule>, String> {
    let value: Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(e) => return Err(format!("invalid JSON: {}", e)),
    };

    let schema = match value.get("schema_version") {
        Some(Value::String(s)) => s.clone(),
        Some(other) => return Err(format!("expected schema_version string, found {}", other)),
        None => return Ok(None),
    };

    if !schema.starts_with('1') {
        return Err(format!(
            "unsupported schema_version '{}': expected 1.x",
            schema
        ));
    }

    let functions = value
        .get("functions")
        .and_then(|f| f.as_array())
        .ok_or_else(|| "v1 JSON missing functions array".to_string())?;

    let mut module = MirModule::new("ny_json_v1".to_string());

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
            .ok_or_else(|| format!("function '{}' entry block missing id", func_name))?;
        let entry_bb = BasicBlockId::new(entry_id as u32);

        let inferred_param_count = infer_param_count_from_v1_func(func, &func_name)?;
        let mut signature = FunctionSignature {
            name: func_name.clone(),
            params: vec![MirType::Unknown; inferred_param_count],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        let mut mir_fn = MirFunction::new(signature.clone(), entry_bb);
        let mut max_value_id: u32 = inferred_param_count as u32;

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
                        let dst = inst.get("dst").and_then(|d| d.as_u64()).ok_or_else(|| {
                            format!("const instruction missing dst in function '{}'", func_name)
                        })? as u32;
                        let value_obj = inst.get("value").ok_or_else(|| {
                            format!(
                                "const instruction missing value in function '{}'",
                                func_name
                            )
                        })?;
                        let const_val = mirjson_common::parse_const_value_generic(value_obj)?;
                        block_ref.add_instruction(MirInstruction::Const {
                            dst: ValueId::new(dst),
                            value: const_val,
                        });
                        if dst >= max_value_id {
                            max_value_id = dst + 1;
                        }
                    }
                    "copy" => {
                        let dst = inst.get("dst").and_then(|d| d.as_u64()).ok_or_else(|| {
                            format!("copy instruction missing dst in function '{}'", func_name)
                        })? as u32;
                        let src = inst.get("src").and_then(|d| d.as_u64()).ok_or_else(|| {
                            format!("copy instruction missing src in function '{}'", func_name)
                        })? as u32;
                        block_ref.add_instruction(MirInstruction::Copy {
                            dst: ValueId::new(dst),
                            src: ValueId::new(src),
                        });
                        max_value_id = max_value_id.max(dst + 1).max(src + 1);
                    }
                    "binop" => {
                        let dst = require_u64(inst, "dst", "binop dst")? as u32;
                        let lhs = require_u64(inst, "lhs", "binop lhs")? as u32;
                        let rhs = require_u64(inst, "rhs", "binop rhs")? as u32;
                        let operation =
                            inst.get("operation")
                                .and_then(Value::as_str)
                                .ok_or_else(|| {
                                    format!("binop operation missing in function '{}'", func_name)
                                })?;
                        let bop = parse_binop(operation)?;
                        block_ref.add_instruction(MirInstruction::BinOp {
                            dst: ValueId::new(dst),
                            op: bop,
                            lhs: ValueId::new(lhs),
                            rhs: ValueId::new(rhs),
                        });
                        max_value_id = max_value_id.max(dst + 1).max(lhs + 1).max(rhs + 1);
                    }
                    "compare" => {
                        let dst = require_u64(inst, "dst", "compare dst")? as u32;
                        let lhs = require_u64(inst, "lhs", "compare lhs")? as u32;
                        let rhs = require_u64(inst, "rhs", "compare rhs")? as u32;
                        // Accept both JSON shapes:
                        //  - operation: symbolic string ("<", ">=", "==", ...)
                        //  - cmp: spelled enum name ("Lt", "Le", "Gt", "Ge", "Eq", "Ne")
                        let op_sym_opt = inst
                            .get("operation")
                            .and_then(Value::as_str)
                            .map(|s| s.to_string());
                        let op_sym = if let Some(sym) = op_sym_opt {
                            sym
                        } else if let Some(name) = inst.get("cmp").and_then(Value::as_str) {
                            match name {
                                "Lt" => "<".to_string(),
                                "Le" => "<=".to_string(),
                                "Gt" => ">".to_string(),
                                "Ge" => ">=".to_string(),
                                "Eq" => "==".to_string(),
                                "Ne" => "!=".to_string(),
                                other => {
                                    return Err(format!(
                                        "unsupported compare cmp '{}' in Gate-C v1 bridge (function '{}')",
                                        other, func_name
                                    ));
                                }
                            }
                        } else {
                            return Err(format!(
                                "compare operation missing in function '{}'",
                                func_name
                            ));
                        };
                        let cop = parse_compare(&op_sym)?;
                        block_ref.add_instruction(MirInstruction::Compare {
                            dst: ValueId::new(dst),
                            op: cop,
                            lhs: ValueId::new(lhs),
                            rhs: ValueId::new(rhs),
                        });
                        max_value_id = max_value_id.max(dst + 1).max(lhs + 1).max(rhs + 1);
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
                        max_value_id = max_value_id.max(cond + 1);
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
                        let pairs = mirjson_common::parse_phi_incoming_generic(inst)
                            .map_err(|e| format!("{} in function '{}'", e, func_name))?;
                        block_ref.add_instruction(MirInstruction::Phi {
                            dst: ValueId::new(dst),
                            inputs: pairs,
                            type_hint: None, // Phase 63-6
                        });
                        let mut phi_max = dst + 1;
                        for (_pred, value) in block_ref
                            .instructions
                            .last()
                            .and_then(|i| match i {
                                MirInstruction::Phi { inputs, .. } => Some(inputs.as_slice()),
                                _ => None,
                            })
                            .unwrap_or(&[])
                        {
                            phi_max = phi_max.max(value.as_u32() + 1);
                        }
                        max_value_id = max_value_id.max(phi_max);
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
                    "mir_call" => {
                        // Minimal v1 mir_call support (Global/Method/Constructor/Extern/Value + Closure creation)
                        // Accept both shapes:
                        //  - flat:   { op:"mir_call", callee:{...}, args:[...], effects:[] }
                        //  - nested: { op:"mir_call", mir_call:{ callee:{...}, args:[...], effects:[] } }
                        // dst remains at the instruction root level in both forms.
                        let dst_opt = inst
                            .get("dst")
                            .and_then(|d| d.as_u64())
                            .map(|v| ValueId::new(v as u32));
                        let effects = if let Some(sub) = inst.get("mir_call") {
                            parse_effects_from(sub)
                        } else {
                            parse_effects_from(inst)
                        };
                        // args: support both flat/nested placement
                        let mut argv: Vec<ValueId> = Vec::new();
                        if let Some(arr) =
                            inst.get("args").and_then(|a| a.as_array()).or_else(|| {
                                inst.get("mir_call")
                                    .and_then(|m| m.get("args").and_then(|a| a.as_array()))
                            })
                        {
                            for a in arr {
                                let id = a.as_u64().ok_or_else(|| {
                                    format!(
                                        "mir_call arg must be integer value id in function '{}'",
                                        func_name
                                    )
                                })? as u32;
                                argv.push(ValueId::new(id));
                            }
                        }
                        // callee: support Global/Method/Extern/Value/Closure/Constructor (minimal)
                        let callee_obj = inst
                            .get("callee")
                            .or_else(|| inst.get("mir_call").and_then(|m| m.get("callee")))
                            .ok_or_else(|| {
                                format!("mir_call missing callee in function '{}'", func_name)
                            })?;
                        let ctype =
                            callee_obj
                                .get("type")
                                .and_then(Value::as_str)
                                .ok_or_else(|| {
                                    format!(
                                        "mir_call callee.type missing in function '{}'",
                                        func_name
                                    )
                                })?;
                        match ctype {
                            "Global" => {
                                let raw_name = callee_obj
                                    .get("name")
                                    .and_then(Value::as_str)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Global missing name in function '{}'",
                                            func_name
                                        )
                                    })?;
                                // Map known console aliases to interpreter-accepted names
                                let mapped = match raw_name {
                                    "print" => "print".to_string(),
                                    "nyash.builtin.print" => "nyash.builtin.print".to_string(),
                                    "nyash.console.log" => "nyash.console.log".to_string(),
                                    // Accept env.console.* as nyash.console.log (numeric only)
                                    "env.console.log" | "env.console.warn"
                                    | "env.console.error" => "nyash.console.log".to_string(),
                                    other => {
                                        return Err(format!(
                                            "unsupported Global callee '{}' in mir_call (Gate-C v1 bridge)",
                                            other
                                        ));
                                    }
                                };
                                block_ref.add_instruction(MirInstruction::Call {
                                    dst: dst_opt,
                                    func: ValueId::new(0),
                                    callee: Some(crate::mir::definitions::Callee::Global(mapped)),
                                    args: argv,
                                    effects,
                                });
                                if let Some(arg_max) =
                                    block_ref.instructions.last().and_then(|i| match i {
                                        MirInstruction::Call { args, .. } => {
                                            args.iter().map(|v| v.as_u32()).max()
                                        }
                                        _ => None,
                                    })
                                {
                                    max_value_id = max_value_id.max(arg_max + 1);
                                }
                                if let Some(d) = dst_opt {
                                    max_value_id = max_value_id.max(d.as_u32() + 1);
                                }
                            }
                            "Constructor" => {
                                // new box instance: canonical key `name` (legacy: box_type)
                                let bt = callee_obj
                                    .get("name")
                                    .or_else(|| callee_obj.get("box_type"))
                                    .and_then(Value::as_str)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Constructor missing name/box_type in function '{}'",
                                            func_name
                                        )
                                    })?;
                                // dst required for Constructor
                                let dst = dst_opt.ok_or_else(|| {
                                    format!(
                                        "mir_call Constructor requires dst in function '{}'",
                                        func_name
                                    )
                                })?;
                                block_ref.add_instruction(MirInstruction::NewBox {
                                    dst,
                                    box_type: bt.to_string(),
                                    args: argv.clone(),
                                });
                                if let Some(arg_max) = argv.iter().map(|v| v.as_u32()).max() {
                                    max_value_id = max_value_id.max(arg_max + 1);
                                }
                                max_value_id = max_value_id.max(dst.as_u32() + 1);
                            }
                            "Method" => {
                                // receiver: required u64, canonical method key is `name`
                                // (legacy fallback: `method` for transition tolerance)
                                let method = callee_obj
                                    .get("name")
                                    .or_else(|| callee_obj.get("method"))
                                    .and_then(Value::as_str)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Method missing name/method in function '{}'",
                                            func_name
                                        )
                                    })?
                                    .to_string();
                                let recv_id = callee_obj
                                    .get("receiver")
                                    .and_then(Value::as_u64)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Method missing receiver in function '{}'",
                                            func_name
                                        )
                                    })? as u32;
                                let box_name = callee_obj
                                    .get("box_name")
                                    .and_then(Value::as_str)
                                    .unwrap_or("")
                                    .to_string();
                                block_ref.add_instruction(MirInstruction::Call {
                                    dst: dst_opt,
                                    func: ValueId::new(0),
                                    callee: Some(crate::mir::definitions::Callee::Method {
                                        box_name: box_name.clone(),
                                        method,
                                        receiver: Some(ValueId::new(recv_id)),
                                        certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                                        // JSON v1 bridge: assume all methods are runtime data boxes
                                        box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                                    }),
                                    args: argv,
                                    effects,
                                });
                                if let Some(arg_max) =
                                    block_ref.instructions.last().and_then(|i| match i {
                                        MirInstruction::Call { args, .. } => {
                                            args.iter().map(|v| v.as_u32()).max()
                                        }
                                        _ => None,
                                    })
                                {
                                    max_value_id = max_value_id.max(arg_max + 1);
                                }
                                if let Some(d) = dst_opt {
                                    max_value_id = max_value_id.max(d.as_u32() + 1);
                                }
                            }
                            "Closure" => {
                                // Two shapes are seen in the wild:
                                // 1) NewClosure-style descriptor (params/captures/me_capture present) → NewClosure
                                // 2) Value-style descriptor (func present, optionally captures array) → Call(Callee::Value)
                                let has_new_fields = callee_obj.get("params").is_some()
                                    || callee_obj.get("captures").is_some()
                                    || callee_obj.get("me_capture").is_some();
                                if has_new_fields {
                                    // Closure creation (NewClosure equivalent)
                                    let dst = dst_opt.ok_or_else(|| {
                                        format!(
                                            "mir_call Closure requires dst in function '{}'",
                                            func_name
                                        )
                                    })?;
                                    // params: array of strings (optional)
                                    let mut params: Vec<String> = Vec::new();
                                    if let Some(arr) =
                                        callee_obj.get("params").and_then(Value::as_array)
                                    {
                                        for p in arr {
                                            let s = p.as_str().ok_or_else(|| {
                                                format!(
                                                    "mir_call Closure params must be strings in function '{}'",
                                                    func_name
                                                )
                                            })?;
                                            params.push(s.to_string());
                                        }
                                    }
                                    // captures: array of [name, id]
                                    let mut captures: Vec<(String, ValueId)> = Vec::new();
                                    if let Some(arr) =
                                        callee_obj.get("captures").and_then(Value::as_array)
                                    {
                                        for e in arr {
                                            let pair = e.as_array().ok_or_else(|| {
                                                format!(
                                                    "mir_call Closure capture entry must be array in function '{}'",
                                                    func_name
                                                )
                                            })?;
                                            if pair.len() != 2 {
                                                return Err(
                                                    "mir_call Closure capture entry must have 2 elements".into(),
                                                );
                                            }
                                            let name = pair[0].as_str().ok_or_else(|| {
                                                "mir_call Closure capture[0] must be string"
                                                    .to_string()
                                            })?;
                                            let id = pair[1].as_u64().ok_or_else(|| {
                                                "mir_call Closure capture[1] must be integer"
                                                    .to_string()
                                            })?
                                                as u32;
                                            captures.push((name.to_string(), ValueId::new(id)));
                                        }
                                    }
                                    // me_capture: optional u64
                                    let me_capture = callee_obj
                                        .get("me_capture")
                                        .and_then(Value::as_u64)
                                        .map(|v| ValueId::new(v as u32));
                                    // Body is not carried in v1; create empty body vector as placeholder
                                    block_ref.add_instruction(MirInstruction::NewClosure {
                                        dst,
                                        params,
                                        body_id: None,
                                        body: Vec::new(),
                                        captures,
                                        me: me_capture,
                                    });
                                    max_value_id = max_value_id.max(dst.as_u32() + 1);
                                } else {
                                    // Value-style closure: treat like Value(func id)
                                    let fid = callee_obj
                                        .get("func")
                                        .and_then(Value::as_u64)
                                        .ok_or_else(|| {
                                            format!(
                                                "mir_call callee Closure missing func in function '{}'",
                                                func_name
                                            )
                                        })? as u32;
                                    // Captures array (if present) are appended to argv for minimal parity
                                    if let Some(caps) =
                                        callee_obj.get("captures").and_then(Value::as_array)
                                    {
                                        for c in caps {
                                            let id = c.as_u64().ok_or_else(|| {
                                                format!(
                                                    "mir_call Closure capture must be integer in function '{}'",
                                                    func_name
                                                )
                                            })? as u32;
                                            argv.push(ValueId::new(id));
                                        }
                                    }
                                    block_ref.add_instruction(MirInstruction::Call {
                                        dst: dst_opt,
                                        func: ValueId::new(0),
                                        callee: Some(crate::mir::definitions::Callee::Value(
                                            ValueId::new(fid),
                                        )),
                                        args: argv,
                                        effects,
                                    });
                                    max_value_id = max_value_id.max(fid + 1);
                                    if let Some(arg_max) =
                                        block_ref.instructions.last().and_then(|i| match i {
                                            MirInstruction::Call { args, .. } => {
                                                args.iter().map(|v| v.as_u32()).max()
                                            }
                                            _ => None,
                                        })
                                    {
                                        max_value_id = max_value_id.max(arg_max + 1);
                                    }
                                    if let Some(d) = dst_opt {
                                        max_value_id = max_value_id.max(d.as_u32() + 1);
                                    }
                                }
                            }
                            "Extern" => {
                                let name = callee_obj
                                    .get("name")
                                    .and_then(Value::as_str)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Extern missing name in function '{}'",
                                            func_name
                                        )
                                    })?
                                    .to_string();
                                block_ref.add_instruction(MirInstruction::Call {
                                    dst: dst_opt,
                                    func: ValueId::new(0),
                                    callee: Some(crate::mir::definitions::Callee::Extern(name)),
                                    args: argv,
                                    effects: EffectMask::IO,
                                });
                                if let Some(arg_max) =
                                    block_ref.instructions.last().and_then(|i| match i {
                                        MirInstruction::Call { args, .. } => {
                                            args.iter().map(|v| v.as_u32()).max()
                                        }
                                        _ => None,
                                    })
                                {
                                    max_value_id = max_value_id.max(arg_max + 1);
                                }
                                if let Some(d) = dst_opt {
                                    max_value_id = max_value_id.max(d.as_u32() + 1);
                                }
                            }
                            "Value" => {
                                // dynamic function value id: canonical `value` (legacy: function_value/func)
                                let fid = callee_obj
                                    .get("value")
                                    .or_else(|| callee_obj.get("function_value"))
                                    .or_else(|| callee_obj.get("func"))
                                    .and_then(Value::as_u64)
                                    .ok_or_else(|| {
                                        format!(
                                            "mir_call callee Value missing value/function_value/func in function '{}'",
                                            func_name
                                        )
                                    })? as u32;
                                block_ref.add_instruction(MirInstruction::Call {
                                    dst: dst_opt,
                                    func: ValueId::new(0),
                                    callee: Some(crate::mir::definitions::Callee::Value(
                                        ValueId::new(fid),
                                    )),
                                    args: argv,
                                    effects,
                                });
                                max_value_id = max_value_id.max(fid + 1);
                                if let Some(arg_max) =
                                    block_ref.instructions.last().and_then(|i| match i {
                                        MirInstruction::Call { args, .. } => {
                                            args.iter().map(|v| v.as_u32()).max()
                                        }
                                        _ => None,
                                    })
                                {
                                    max_value_id = max_value_id.max(arg_max + 1);
                                }
                                if let Some(d) = dst_opt {
                                    max_value_id = max_value_id.max(d.as_u32() + 1);
                                }
                            }
                            // (no duplicate Closure arm; handled above)
                            other => {
                                return Err(format!(
                                    "unsupported callee type '{}' in mir_call (Gate-C v1 bridge)",
                                    other
                                ));
                            }
                        }
                    }
                    other => {
                        return Err(format!(
                            "unsupported instruction '{}' in function '{}' (Gate-C v1 bridge)",
                            other, func_name
                        ));
                    }
                }
            }
        }
        mir_fn.signature = signature;
        mir_fn.next_value_id = max_value_id.max(mir_fn.next_value_id);
        module.add_function(mir_fn);
    }

    Ok(Some(module))
}
