use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::definitions::Callee;
use crate::mir::{EffectMask, MirInstruction, ValueId};
use serde_json::Value;

fn bump_max_value_id_from_call(block: &crate::mir::BasicBlock, max_value_id: &mut u32) {
    if let Some(arg_max) = block.instructions.last().and_then(|i| match i {
        MirInstruction::Call { args, .. } => args.iter().map(|v| v.as_u32()).max(),
        _ => None,
    }) {
        *max_value_id = (*max_value_id).max(arg_max + 1);
    }
}

pub(super) fn parse_v1_mir_call(
    inst: &Value,
    func_name: &str,
    block_ref: &mut crate::mir::BasicBlock,
    max_value_id: &mut u32,
) -> Result<(), String> {
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
        super::super::helpers::parse_effects_from(sub)
    } else {
        super::super::helpers::parse_effects_from(inst)
    };

    // args: support both flat/nested placement
    let mut argv: Vec<ValueId> = Vec::new();
    if let Some(arr) = inst.get("args").and_then(|a| a.as_array()).or_else(|| {
        inst.get("mir_call")
            .and_then(|m| m.get("args").and_then(|a| a.as_array()))
    }) {
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
        .ok_or_else(|| format!("mir_call missing callee in function '{}'", func_name))?;
    let ctype = callee_obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("mir_call callee.type missing in function '{}'", func_name))?;

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
                "env.console.log" | "env.console.warn" | "env.console.error" => {
                    "nyash.console.log".to_string()
                }
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
                callee: Some(Callee::Global(mapped)),
                args: argv,
                effects,
            });
            bump_max_value_id_from_call(block_ref, max_value_id);
            if let Some(d) = dst_opt {
                *max_value_id = (*max_value_id).max(d.as_u32() + 1);
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
                *max_value_id = (*max_value_id).max(arg_max + 1);
            }
            *max_value_id = (*max_value_id).max(dst.as_u32() + 1);
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
                callee: Some(Callee::Method {
                    box_name: box_name.clone(),
                    method,
                    receiver: Some(ValueId::new(recv_id)),
                    certainty: TypeCertainty::Known,
                    // JSON v1 bridge: assume all methods are runtime data boxes
                    box_kind: CalleeBoxKind::RuntimeData,
                }),
                args: argv,
                effects,
            });
            bump_max_value_id_from_call(block_ref, max_value_id);
            if let Some(d) = dst_opt {
                *max_value_id = (*max_value_id).max(d.as_u32() + 1);
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
                    format!("mir_call Closure requires dst in function '{}'", func_name)
                })?;
                // params: array of strings (optional)
                let mut params: Vec<String> = Vec::new();
                if let Some(arr) = callee_obj.get("params").and_then(Value::as_array) {
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
                if let Some(arr) = callee_obj.get("captures").and_then(Value::as_array) {
                    for e in arr {
                        let pair = e.as_array().ok_or_else(|| {
                            format!(
                                "mir_call Closure capture entry must be array in function '{}'",
                                func_name
                            )
                        })?;
                        if pair.len() != 2 {
                            return Err(
                                "mir_call Closure capture entry must have 2 elements".into()
                            );
                        }
                        let name = pair[0].as_str().ok_or_else(|| {
                            "mir_call Closure capture[0] must be string".to_string()
                        })?;
                        let id = pair[1].as_u64().ok_or_else(|| {
                            "mir_call Closure capture[1] must be integer".to_string()
                        })? as u32;
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
                *max_value_id = (*max_value_id).max(dst.as_u32() + 1);
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
                if let Some(caps) = callee_obj.get("captures").and_then(Value::as_array) {
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
                    callee: Some(Callee::Value(ValueId::new(fid))),
                    args: argv,
                    effects,
                });
                *max_value_id = (*max_value_id).max(fid + 1);
                bump_max_value_id_from_call(block_ref, max_value_id);
                if let Some(d) = dst_opt {
                    *max_value_id = (*max_value_id).max(d.as_u32() + 1);
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
                callee: Some(Callee::Extern(name)),
                args: argv,
                effects: EffectMask::IO,
            });
            bump_max_value_id_from_call(block_ref, max_value_id);
            if let Some(d) = dst_opt {
                *max_value_id = (*max_value_id).max(d.as_u32() + 1);
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
                callee: Some(Callee::Value(ValueId::new(fid))),
                args: argv,
                effects,
            });
            *max_value_id = (*max_value_id).max(fid + 1);
            bump_max_value_id_from_call(block_ref, max_value_id);
            if let Some(d) = dst_opt {
                *max_value_id = (*max_value_id).max(d.as_u32() + 1);
            }
        }
        other => {
            return Err(format!(
                "unsupported callee type '{}' in mir_call (Gate-C v1 bridge)",
                other
            ));
        }
    }

    Ok(())
}
