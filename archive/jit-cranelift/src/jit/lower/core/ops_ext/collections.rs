use super::super::builder::IRBuilder;
use super::LowerCore;
use crate::mir::{MirFunction, ValueId};

pub(super) fn lower_collection_box_method(
    lower: &mut LowerCore,
    _func: &MirFunction,
    b: &mut dyn IRBuilder,
    array: &ValueId,
    method: &str,
    args: &Vec<ValueId>,
    dst: Option<ValueId>,
) -> Result<bool, String> {
    match method {
        "push" => {
            let argc = 2usize;
            if let Some(pidx) = lower.param_index.get(array).copied() {
                b.emit_param_i64(pidx);
            } else {
                b.emit_const_i64(-1);
            }
            if let Some(v) = args.get(0).and_then(|vid| lower.known_i64.get(vid)).copied() {
                b.emit_const_i64(v);
            } else if let Some(v) = args.get(0) {
                lower.push_value_if_known_or_param(b, v);
            } else {
                b.emit_const_i64(0);
            }
            let decision =
                crate::jit::policy::invoke::decide_box_method("ArrayBox", "push", argc, false);
            match decision {
                crate::jit::policy::invoke::InvokeDecision::PluginInvoke {
                    type_id,
                    method_id,
                    box_type,
                    ..
                } => {
                    b.emit_plugin_invoke(type_id, method_id, argc, false);
                    crate::jit::observe::lower_plugin_invoke(
                        &box_type, "push", type_id, method_id, argc,
                    );
                }
                crate::jit::policy::invoke::InvokeDecision::HostCall { symbol, .. } => {
                    crate::jit::observe::lower_hostcall(
                        &symbol,
                        argc,
                        &["Handle", "I64"],
                        "allow",
                        "mapped_symbol",
                    );
                    b.emit_host_call(&symbol, argc, false);
                }
                _ => {
                    let sym = if lower.param_index.get(array).is_some() {
                        crate::jit::r#extern::collections::SYM_ARRAY_PUSH_H
                    } else {
                        crate::jit::r#extern::collections::SYM_ARRAY_PUSH
                    };
                    let arg_types = if lower.param_index.get(array).is_some() {
                        &["Handle", "I64"][..]
                    } else {
                        &["I64", "I64"][..]
                    };
                    crate::jit::observe::lower_hostcall(
                        sym,
                        argc,
                        arg_types,
                        "fallback",
                        "policy_or_unknown",
                    );
                    b.emit_host_call(sym, argc, false);
                }
            }
            return Ok(true);
        }
        "size" | "get" | "has" | "set" => {
            let is_set = method == "set";
            if is_set && crate::jit::policy::current().read_only {
                if dst.is_some() {
                    b.emit_const_i64(0);
                }
                return Ok(true);
            }
            let argc = match method {
                "size" => 1,
                "get" | "has" => 2,
                "set" => 3,
                _ => 1,
            };
            if lower.handle_values.contains(array) {
                lower.push_value_if_known_or_param(b, array);
                match method {
                    "size" => b.emit_host_call(
                        crate::jit::r#extern::collections::SYM_MAP_SIZE_H,
                        argc,
                        dst.is_some(),
                    ),
                    "get" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_GET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "has" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_HAS_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "set" => {
                        if let Some(k) = args.get(0) {
                            lower.push_value_if_known_or_param(b, k);
                        } else {
                            b.emit_const_i64(0);
                        }
                        if let Some(v) = args.get(1) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_SET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    _ => {}
                }
                return Ok(true);
            }
            if let Ok(ph) = crate::runtime::plugin_loader_unified::get_global_plugin_host().read() {
                if let Ok(h) = ph.resolve_method("MapBox", method) {
                    if let Some(pidx) = lower.param_index.get(array).copied() {
                        b.emit_param_i64(pidx);
                    } else {
                        b.emit_const_i64(-1);
                    }
                    match method {
                        "size" => {}
                        "get" | "has" => {
                            if let Some(v) = args.get(0) {
                                lower.push_value_if_known_or_param(b, v);
                            } else {
                                b.emit_const_i64(0);
                            }
                        }
                        "set" => {
                            if let Some(k) = args.get(0) {
                                lower.push_value_if_known_or_param(b, k);
                            } else {
                                b.emit_const_i64(0);
                            }
                            if let Some(v) = args.get(1) {
                                lower.push_value_if_known_or_param(b, v);
                            } else {
                                b.emit_const_i64(0);
                            }
                        }
                        _ => {}
                    }
                    b.emit_plugin_invoke(h.type_id, h.method_id, argc, dst.is_some());
                    crate::jit::events::emit_lower(
                        serde_json::json!({
                            "id": format!("plugin:{}:{}", h.box_type, method),
                            "decision":"allow","reason":"plugin_invoke","argc": argc,
                            "type_id": h.type_id, "method_id": h.method_id
                        }),
                        "plugin",
                        "<jit>",
                    );
                    return Ok(true);
                }
            }
            if let Some(pidx) = lower.param_index.get(array).copied() {
                b.emit_param_i64(pidx);
                match method {
                    "size" => b.emit_host_call(
                        crate::jit::r#extern::collections::SYM_MAP_SIZE_H,
                        argc,
                        dst.is_some(),
                    ),
                    "get" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_GET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "has" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_HAS_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "set" => {
                        if let Some(k) = args.get(0) {
                            lower.push_value_if_known_or_param(b, k);
                        } else {
                            b.emit_const_i64(0);
                        }
                        if let Some(v) = args.get(1) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_SET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    _ => {}
                }
            } else {
                lower.push_value_if_known_or_param(b, array);
                match method {
                    "size" => b.emit_host_call(
                        crate::jit::r#extern::collections::SYM_MAP_SIZE_H,
                        argc,
                        dst.is_some(),
                    ),
                    "get" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_GET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "has" => {
                        if let Some(v) = args.get(0) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_HAS_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    "set" => {
                        if let Some(k) = args.get(0) {
                            lower.push_value_if_known_or_param(b, k);
                        } else {
                            b.emit_const_i64(0);
                        }
                        if let Some(v) = args.get(1) {
                            lower.push_value_if_known_or_param(b, v);
                        } else {
                            b.emit_const_i64(0);
                        }
                        b.emit_host_call(
                            crate::jit::r#extern::collections::SYM_MAP_SET_H,
                            argc,
                            dst.is_some(),
                        )
                    }
                    _ => {}
                }
            }
            return Ok(true);
        }
        _ => {}
    }
    Ok(false)
}
