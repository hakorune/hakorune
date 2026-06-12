use super::*;

pub fn lower_box_call(
    func: &MirFunction,
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    known_f64: &HashMap<ValueId, f64>,
    float_box_values: &std::collections::HashSet<ValueId>,
    recv: &ValueId,
    method: &str,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) {
    if !crate::jit::config::current().hostcall {
        return;
    }
    match method {
        "len" | "length" => {
            if let Some(pidx) = param_index.get(recv).copied() {
                crate::jit::events::emit_lower(
                    serde_json::json!({"id": crate::jit::r#extern::collections::SYM_ARRAY_LEN, "decision":"allow", "reason":"sig_ok", "argc":1, "arg_types":["I64(index)"]}),
                    "hostcall",
                    "<jit>",
                );
                b.emit_param_i64(pidx as i64 as usize);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ARRAY_LEN,
                    1,
                    dst.is_some(),
                );
            } else {
                crate::jit::events::emit_lower(
                    serde_json::json!({
                        "id": crate::jit::r#extern::collections::SYM_ARRAY_LEN,
                        "decision": "fallback", "reason": "receiver_not_param",
                        "argc": 1, "arg_types": ["I64(index)"]
                    }),
                    "hostcall",
                    "<jit>",
                );
                b.emit_const_i64(-1);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ARRAY_LEN,
                    1,
                    dst.is_some(),
                );
            }
        }
        "isEmpty" | "is_empty" => {
            crate::jit::events::emit_lower(
                serde_json::json!({"id": crate::jit::r#extern::collections::SYM_ANY_IS_EMPTY_H, "decision":"allow", "reason":"sig_ok", "argc":1, "arg_types":["Handle"]}),
                "hostcall",
                "<jit>",
            );
            if let Some(pidx) = param_index.get(recv).copied() {
                b.emit_param_i64(pidx);
                b.emit_host_call(
                    crate::jit::r#extern::collections::SYM_ANY_IS_EMPTY_H,
                    1,
                    dst.is_some(),
                );
            }
        }
        m if m.starts_with("math.") => {
            let sym = format!("nyash.{}", m);
            use crate::jit::hostcall_registry::{check_signature, ArgKind};
            let mut observed: Vec<ArgKind> = Vec::new();
            for (i, v) in args.iter().enumerate() {
                let kind = if let Some(mt) = func.signature.params.get(i) {
                    match mt {
                        crate::mir::MirType::Float => ArgKind::F64,
                        crate::mir::MirType::Integer => ArgKind::I64,
                        crate::mir::MirType::Bool => ArgKind::I64,
                        crate::mir::MirType::String | crate::mir::MirType::Box(_) => {
                            ArgKind::Handle
                        }
                        _ => {
                            if known_f64.contains_key(v) || float_box_values.contains(v) {
                                ArgKind::F64
                            } else {
                                ArgKind::I64
                            }
                        }
                    }
                } else if known_f64.contains_key(v) || float_box_values.contains(v) {
                    ArgKind::F64
                } else {
                    ArgKind::I64
                };
                observed.push(kind);
            }
            let arg_types: Vec<&'static str> = observed
                .iter()
                .map(|k| match k {
                    ArgKind::I64 => "I64",
                    ArgKind::F64 => "F64",
                    ArgKind::Handle => "Handle",
                })
                .collect();
            match check_signature(&sym, &observed) {
                Ok(()) => {
                    crate::jit::events::emit_lower(
                        serde_json::json!({"id": sym, "decision":"allow", "reason":"sig_ok", "argc": observed.len(), "arg_types": arg_types}),
                        "hostcall",
                        "<jit>",
                    );
                    if crate::jit::config::current().native_f64 {
                        let (symbol, arity) = match method {
                            "math.sin" => ("nyash.math.sin_f64", 1),
                            "math.cos" => ("nyash.math.cos_f64", 1),
                            "math.abs" => ("nyash.math.abs_f64", 1),
                            "math.min" => ("nyash.math.min_f64", 2),
                            "math.max" => ("nyash.math.max_f64", 2),
                            _ => ("nyash.math.sin_f64", 1),
                        };
                        for i in 0..arity {
                            if let Some(v) = args.get(i) {
                                if let Some(fv) = known_f64.get(v).copied() {
                                    b.emit_const_f64(fv);
                                    continue;
                                }
                                if let Some(iv) = known_i64.get(v).copied() {
                                    b.emit_const_f64(iv as f64);
                                    continue;
                                }
                                let mut emitted = false;
                                'scan: for (_bb_id, bb) in func.blocks.iter() {
                                    for ins in bb.instructions.iter() {
                                        if let crate::mir::MirInstruction::NewBox {
                                            dst,
                                            box_type,
                                            args: nb_args,
                                        } = ins
                                        {
                                            if *dst == *v && box_type == "FloatBox" {
                                                if let Some(srcv) = nb_args.get(0) {
                                                    if let Some(fv) = known_f64.get(srcv).copied() {
                                                        b.emit_const_f64(fv);
                                                        emitted = true;
                                                        break 'scan;
                                                    }
                                                    if let Some(iv) = known_i64.get(srcv).copied() {
                                                        b.emit_const_f64(iv as f64);
                                                        emitted = true;
                                                        break 'scan;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !emitted {
                                    b.emit_const_f64(0.0);
                                }
                            } else {
                                b.emit_const_f64(0.0);
                            }
                        }
                        let kinds: Vec<super::builder::ParamKind> =
                            (0..arity).map(|_| super::builder::ParamKind::F64).collect();
                        b.emit_host_call_typed(symbol, &kinds, dst.is_some(), true);
                    }
                }
                Err(reason) => {
                    crate::jit::events::emit_lower(
                        serde_json::json!({"id": sym, "decision":"fallback", "reason": reason, "argc": observed.len(), "arg_types": arg_types}),
                        "hostcall",
                        "<jit>",
                    );
                }
            }
        }
        _ => {
            let pol = crate::jit::policy::current();
            match method {
                "charCodeAt" => {
                    crate::jit::events::emit_lower(
                        serde_json::json!({"id": crate::jit::r#extern::collections::SYM_STRING_CHARCODE_AT_H, "decision":"allow", "reason":"sig_ok", "argc":2, "arg_types":["Handle","I64"]}),
                        "hostcall",
                        "<jit>",
                    );
                    if let Some(pidx) = param_index.get(recv).copied() {
                        b.emit_param_i64(pidx);
                        let idx = args
                            .get(0)
                            .and_then(|v| known_i64.get(v).copied())
                            .unwrap_or(0);
                        b.emit_const_i64(idx);
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_STRING_CHARCODE_AT_H,
                            2,
                            dst.is_some(),
                        );
                    }
                }
                "size" => {
                    lower_map_size_simple(b, param_index, recv, dst.is_some());
                }
                "get" => {
                    if let Some(k) = args.get(0) {
                        lower_map_get_simple(b, param_index, known_i64, recv, k, dst.is_some());
                    }
                }
                "has" => {
                    if let Some(k) = args.get(0) {
                        lower_map_has_simple(b, param_index, known_i64, recv, k, dst.is_some());
                    }
                }
                "set" => {
                    if args.len() >= 2 {
                        lower_map_set_simple(b, param_index, known_i64, recv, &args[0], &args[1]);
                    }
                }
                "has" => {
                    use crate::jit::hostcall_registry::{check_signature, ArgKind};
                    let canonical = "nyash.map.has".to_string();
                    let mut observed_kinds: Vec<ArgKind> = Vec::new();
                    observed_kinds.push(ArgKind::Handle);
                    let key_vid = args.get(0).copied();
                    let key_kind = if let Some(kv) = key_vid {
                        if let Some(mt) = func.signature.params.iter().find(|_| true) {
                            match mt {
                                crate::mir::MirType::Float => ArgKind::I64,
                                crate::mir::MirType::Integer => ArgKind::I64,
                                crate::mir::MirType::Bool => ArgKind::I64,
                                crate::mir::MirType::String | crate::mir::MirType::Box(_) => {
                                    ArgKind::Handle
                                }
                                _ => ArgKind::Handle,
                            }
                        } else if known_i64.get(&kv).is_some() {
                            ArgKind::I64
                        } else {
                            ArgKind::Handle
                        }
                    } else {
                        ArgKind::Handle
                    };
                    observed_kinds.push(key_kind);
                    let arg_types: Vec<&'static str> = observed_kinds
                        .iter()
                        .map(|k| match k {
                            ArgKind::I64 => "I64",
                            ArgKind::F64 => "F64",
                            ArgKind::Handle => "Handle",
                        })
                        .collect();
                    let _ = check_signature(&canonical, &observed_kinds);
                    if let Some(pidx) = param_index.get(recv).copied() {
                        b.emit_param_i64(pidx);
                        if let Some(kv) = key_vid {
                            match key_kind {
                                ArgKind::I64 => {
                                    let kval = known_i64.get(&kv).copied().unwrap_or(0);
                                    b.emit_const_i64(kval);
                                    b.emit_host_call(
                                        crate::jit::r#extern::collections::SYM_MAP_GET_H,
                                        2,
                                        dst.is_some(),
                                    );
                                }
                                ArgKind::Handle => {
                                    if let Some(kp) = param_index.get(&kv).copied() {
                                        b.emit_param_i64(kp);
                                        b.emit_host_call(
                                            crate::jit::r#extern::collections::SYM_MAP_GET_HH,
                                            2,
                                            dst.is_some(),
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                "push" | "set" => {
                    let wh = &pol.hostcall_whitelist;
                    let sym = if method == "push" {
                        crate::jit::r#extern::collections::SYM_ARRAY_PUSH
                    } else {
                        crate::jit::r#extern::collections::SYM_ARRAY_SET
                    };
                    if wh.iter().any(|s| s == sym) {
                        crate::jit::events::emit_lower(
                            serde_json::json!({"id": sym, "decision":"allow", "reason":"whitelist", "argc": args.len()}),
                            "hostcall",
                            "<jit>",
                        );
                        b.emit_host_call(sym, 2, false);
                    } else {
                        crate::jit::events::emit_lower(
                            serde_json::json!({"id": sym, "decision":"fallback", "reason":"policy_denied_mutating", "argc": args.len()}),
                            "hostcall",
                            "<jit>",
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn lower_map_get(
    func: &MirFunction,
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    recv: &ValueId,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) {
    if let Some(pidx) = param_index.get(recv).copied() {
        let mut observed_kinds: Vec<crate::jit::hostcall_registry::ArgKind> = Vec::new();
        observed_kinds.push(crate::jit::hostcall_registry::ArgKind::Handle);
        let key_kind = if let Some(key_vid) = args.get(0) {
            if let Some(mt) = func.metadata.value_types.get(key_vid) {
                match mt {
                    crate::mir::MirType::Float => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::Integer => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::Bool => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::String | crate::mir::MirType::Box(_) => {
                        crate::jit::hostcall_registry::ArgKind::Handle
                    }
                    _ => {
                        if args.get(0).and_then(|v| known_i64.get(v)).is_some() {
                            crate::jit::hostcall_registry::ArgKind::I64
                        } else {
                            crate::jit::hostcall_registry::ArgKind::Handle
                        }
                    }
                }
            } else if args.get(0).and_then(|v| known_i64.get(v)).is_some() {
                crate::jit::hostcall_registry::ArgKind::I64
            } else {
                crate::jit::hostcall_registry::ArgKind::Handle
            }
        } else {
            crate::jit::hostcall_registry::ArgKind::I64
        };
        observed_kinds.push(key_kind);

        let arg_types: Vec<&'static str> = observed_kinds
            .iter()
            .map(|k| match k {
                crate::jit::hostcall_registry::ArgKind::I64 => "I64",
                crate::jit::hostcall_registry::ArgKind::F64 => "F64",
                crate::jit::hostcall_registry::ArgKind::Handle => "Handle",
            })
            .collect();
        let canonical = "nyash.map.get_h";
        match crate::jit::hostcall_registry::check_signature(canonical, &observed_kinds) {
            Ok(()) => {
                let event_id = if matches!(key_kind, crate::jit::hostcall_registry::ArgKind::Handle)
                    && args.get(0).and_then(|v| param_index.get(v)).is_some()
                {
                    crate::jit::r#extern::collections::SYM_MAP_GET_HH
                } else {
                    crate::jit::r#extern::collections::SYM_MAP_GET_H
                };
                crate::jit::events::emit_lower(
                    serde_json::json!({
                        "id": event_id,
                        "decision": "allow",
                        "reason": "sig_ok",
                        "argc": observed_kinds.len(),
                        "arg_types": arg_types
                    }),
                    "hostcall",
                    "<jit>",
                );
                if matches!(key_kind, crate::jit::hostcall_registry::ArgKind::I64) {
                    let key_i = args
                        .get(0)
                        .and_then(|v| known_i64.get(v))
                        .copied()
                        .unwrap_or(0);
                    b.emit_param_i64(pidx);
                    b.emit_const_i64(key_i);
                    b.emit_host_call(
                        crate::jit::r#extern::collections::SYM_MAP_GET_H,
                        2,
                        dst.is_some(),
                    );
                } else if let Some(kp) = args.get(0).and_then(|v| param_index.get(v)).copied() {
                    b.emit_param_i64(pidx);
                    b.emit_param_i64(kp);
                    b.emit_host_call(
                        crate::jit::r#extern::collections::SYM_MAP_GET_HH,
                        2,
                        dst.is_some(),
                    );
                }
            }
            Err(reason) => {
                crate::jit::events::emit_lower(
                    serde_json::json!({
                        "id": canonical,
                        "decision": "fallback",
                        "reason": reason,
                        "argc": observed_kinds.len(),
                        "arg_types": arg_types
                    }),
                    "hostcall",
                    "<jit>",
                );
            }
        }
    } else {
        let mut observed_kinds: Vec<crate::jit::hostcall_registry::ArgKind> = Vec::new();
        observed_kinds.push(crate::jit::hostcall_registry::ArgKind::Handle);
        let key_kind = if let Some(key_vid) = args.get(0) {
            if let Some(mt) = func.metadata.value_types.get(key_vid) {
                match mt {
                    crate::mir::MirType::Integer => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::Float => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::Bool => crate::jit::hostcall_registry::ArgKind::I64,
                    crate::mir::MirType::String | crate::mir::MirType::Box(_) => {
                        crate::jit::hostcall_registry::ArgKind::Handle
                    }
                    _ => crate::jit::hostcall_registry::ArgKind::Handle,
                }
            } else {
                crate::jit::hostcall_registry::ArgKind::Handle
            }
        } else {
            crate::jit::hostcall_registry::ArgKind::Handle
        };
        observed_kinds.push(key_kind);
        let arg_types: Vec<&'static str> = observed_kinds
            .iter()
            .map(|k| match k {
                crate::jit::hostcall_registry::ArgKind::I64 => "I64",
                crate::jit::hostcall_registry::ArgKind::F64 => "F64",
                crate::jit::hostcall_registry::ArgKind::Handle => "Handle",
            })
            .collect();
        let sym = "nyash.map.get_h";
        let decision = match crate::jit::hostcall_registry::check_signature(sym, &observed_kinds) {
            Ok(()) => ("fallback", "receiver_not_param"),
            Err(reason) => ("fallback", reason),
        };
        crate::jit::events::emit_lower(
            serde_json::json!({
                "id": sym,
                "decision": decision.0,
                "reason": decision.1,
                "argc": observed_kinds.len(),
                "arg_types": arg_types
            }),
            "hostcall",
            "<jit>",
        );
    }
}

pub fn lower_map_has(
    b: &mut dyn IRBuilder,
    param_index: &HashMap<ValueId, usize>,
    known_i64: &HashMap<ValueId, i64>,
    recv: &ValueId,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) {
    if let Some(pidx) = param_index.get(recv).copied() {
        let key = args
            .get(0)
            .and_then(|v| known_i64.get(v))
            .copied()
            .unwrap_or(0);
        b.emit_param_i64(pidx);
        b.emit_const_i64(key);
        b.emit_host_call(
            crate::jit::r#extern::collections::SYM_MAP_HAS_H,
            2,
            dst.is_some(),
        );
    }
}

pub fn lower_math_call(
    func: &MirFunction,
    b: &mut dyn IRBuilder,
    known_i64: &HashMap<ValueId, i64>,
    known_f64: &HashMap<ValueId, f64>,
    float_box_values: &std::collections::HashSet<ValueId>,
    method: &str,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) {
    use crate::jit::hostcall_registry::{check_signature, ArgKind};
    let sym = format!("nyash.math.{}", method);

    let mut observed_kinds: Vec<ArgKind> = Vec::new();
    for v in args.iter() {
        let kind = if let Some(mt) = func.metadata.value_types.get(v) {
            match mt {
                crate::mir::MirType::Float => ArgKind::F64,
                crate::mir::MirType::Integer => ArgKind::I64,
                crate::mir::MirType::Bool => ArgKind::I64,
                crate::mir::MirType::String | crate::mir::MirType::Box(_) => ArgKind::Handle,
                _ => {
                    if known_f64.contains_key(v) || float_box_values.contains(v) {
                        ArgKind::F64
                    } else {
                        ArgKind::I64
                    }
                }
            }
        } else if known_f64.contains_key(v) || float_box_values.contains(v) {
            ArgKind::F64
        } else {
            ArgKind::I64
        };
        observed_kinds.push(kind);
    }
    let arg_types: Vec<&'static str> = observed_kinds
        .iter()
        .map(|k| match k {
            ArgKind::I64 => "I64",
            ArgKind::F64 => "F64",
            ArgKind::Handle => "Handle",
        })
        .collect();

    match check_signature(&sym, &observed_kinds) {
        Ok(()) => {
            crate::jit::events::emit_lower(
                serde_json::json!({"id": sym, "decision":"allow", "reason":"sig_ok", "argc": observed_kinds.len(), "arg_types": arg_types}),
                "hostcall",
                "<jit>",
            );
            if crate::jit::config::current().native_f64 {
                let (symbol, arity) = match method {
                    "sin" => ("nyash.math.sin_f64", 1),
                    "cos" => ("nyash.math.cos_f64", 1),
                    "abs" => ("nyash.math.abs_f64", 1),
                    "min" => ("nyash.math.min_f64", 2),
                    "max" => ("nyash.math.max_f64", 2),
                    _ => ("nyash.math.sin_f64", 1),
                };
                for i in 0..arity {
                    if let Some(v) = args.get(i) {
                        if let Some(fv) = known_f64.get(v).copied() {
                            b.emit_const_f64(fv);
                            continue;
                        }
                        if let Some(iv) = known_i64.get(v).copied() {
                            b.emit_const_f64(iv as f64);
                            continue;
                        }
                        let mut emitted = false;
                        'scan: for (_bb_id, bb) in func.blocks.iter() {
                            for ins in bb.instructions.iter() {
                                if let crate::mir::MirInstruction::NewBox {
                                    dst,
                                    box_type,
                                    args: nb_args,
                                } = ins
                                {
                                    if *dst == *v && box_type == "FloatBox" {
                                        if let Some(srcv) = nb_args.get(0) {
                                            if let Some(fv) = known_f64.get(srcv).copied() {
                                                b.emit_const_f64(fv);
                                                emitted = true;
                                                break 'scan;
                                            }
                                            if let Some(iv) = known_i64.get(srcv).copied() {
                                                b.emit_const_f64(iv as f64);
                                                emitted = true;
                                                break 'scan;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !emitted {
                            b.emit_const_f64(0.0);
                        }
                    } else {
                        b.emit_const_f64(0.0);
                    }
                }
                let kinds: Vec<super::builder::ParamKind> =
                    (0..arity).map(|_| super::builder::ParamKind::F64).collect();
                b.emit_host_call_typed(symbol, &kinds, dst.is_some(), true);
            }
        }
        Err(reason) => {
            crate::jit::events::emit_lower(
                serde_json::json!({"id": sym, "decision":"fallback", "reason": reason, "argc": observed_kinds.len(), "arg_types": arg_types}),
                "hostcall",
                "<jit>",
            );
        }
    }
}
