use crate::mir::{MirInstruction, ValueId};
use serde_json::Value;

fn legacy_call_stop_error(func_name: &str, callee_type: &str) -> String {
    format!(
        "[freeze:contract][mir-json-v1/legacy-call-stopped] function '{}' uses legacy callee type '{}'; call-like JSON v1 ingress is disabled",
        func_name, callee_type
    )
}

pub(super) fn parse_v1_mir_call(
    inst: &Value,
    func_name: &str,
    block_ref: &mut crate::mir::BasicBlock,
    max_value_id: &mut u32,
) -> Result<(), String> {
    // v1 compatibility ingress retains only construction shapes; call-like
    // carriers stop before block mutation.
    // Accept both shapes:
    //  - flat:   { op:"mir_call", callee:{...}, args:[...], effects:[] }
    //  - nested: { op:"mir_call", mir_call:{ callee:{...}, args:[...], effects:[] } }
    // dst remains at the instruction root level in both forms.
    let dst_opt = inst
        .get("dst")
        .and_then(|d| d.as_u64())
        .map(|v| ValueId::new(v as u32));
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
        "Global" => return Err(legacy_call_stop_error(func_name, ctype)),
        "Constructor" => {
            let flat_args = inst.get("args");
            let nested_args = inst.get("mir_call").and_then(|nested| nested.get("args"));
            if flat_args.is_some() && nested_args.is_some() {
                return Err(format!(
                    "[freeze:contract][mir-json-v1/constructor-args-ambiguous] function '{}' provides both flat and nested args",
                    func_name
                ));
            }
            let args_node = flat_args.or(nested_args).ok_or_else(|| {
                format!(
                    "[freeze:contract][mir-json-v1/constructor-args-required] function '{}' requires an args array",
                    func_name
                )
            })?;
            if !args_node.is_array() {
                return Err(format!(
                    "[freeze:contract][mir-json-v1/constructor-args-must-be-array] function '{}' requires an args array",
                    func_name
                ));
            }

            // new box instance: canonical key `name` (legacy: box_type)
            let bt = match (callee_obj.get("name"), callee_obj.get("box_type")) {
                (Some(name), Some(box_type)) => {
                    let name = name.as_str().ok_or_else(|| {
                        format!(
                            "mir_call callee Constructor name must be a string in function '{}'",
                            func_name
                        )
                    })?;
                    let box_type = box_type.as_str().ok_or_else(|| {
                        format!(
                            "mir_call callee Constructor box_type must be a string in function '{}'",
                            func_name
                        )
                    })?;
                    if name != box_type {
                        return Err(format!(
                            "[freeze:contract][mir-json-v1/constructor-name-box-type-conflict] function '{}' has conflicting name/box_type",
                            func_name
                        ));
                    }
                    name
                }
                (Some(name), None) => name.as_str().ok_or_else(|| {
                    format!(
                        "mir_call callee Constructor name must be a string in function '{}'",
                        func_name
                    )
                })?,
                (None, Some(box_type)) => box_type.as_str().ok_or_else(|| {
                    format!(
                        "mir_call callee Constructor box_type must be a string in function '{}'",
                        func_name
                    )
                })?,
                (None, None) => {
                    return Err(format!(
                        "mir_call callee Constructor missing name/box_type in function '{}'",
                        func_name
                    ));
                }
            };
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
        "Method" => return Err(legacy_call_stop_error(func_name, ctype)),
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
                return Err(legacy_call_stop_error(func_name, ctype));
            }
        }
        "Extern" => return Err(legacy_call_stop_error(func_name, ctype)),
        "Value" => return Err(legacy_call_stop_error(func_name, ctype)),
        other => {
            return Err(format!(
                "unsupported callee type '{}' in mir_call (Gate-C v1 bridge)",
                other
            ));
        }
    }

    Ok(())
}
