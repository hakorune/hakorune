use serde_json::json;

use crate::mir::definitions::Callee;
use crate::mir::{EffectMask, ValueId};

use super::super::helpers::emit_unified_mir_call;

fn env_mode_enabled_default_true(name: &str) -> bool {
    match std::env::var(name)
        .ok()
        .as_deref()
        .map(|s| s.to_ascii_lowercase())
    {
        Some(s) if s == "0" || s == "false" || s == "off" => false,
        _ => true,
    }
}

fn unified_call_enabled() -> bool {
    env_mode_enabled_default_true("NYASH_MIR_UNIFIED_CALL")
}

fn methodize_enabled() -> bool {
    env_mode_enabled_default_true("HAKO_MIR_BUILDER_METHODIZE")
}

pub(crate) fn emit_call(
    dst: &Option<ValueId>,
    func: &ValueId,
    callee: Option<&Callee>,
    args: &[ValueId],
    effects: &EffectMask,
) -> Option<serde_json::Value> {
    let use_unified = unified_call_enabled();
    let methodize_on = methodize_enabled();

    if let Some(Callee::Method { .. }) = callee {
        if methodize_on || use_unified {
            let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
            let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
            let unified_call = emit_unified_mir_call(
                dst.map(|v| v.as_u32()),
                callee.unwrap(),
                &args_u32,
                &effects_str,
            );
            return Some(unified_call);
        }
    }

    if use_unified && callee.is_some() {
        // v1: Unified mir_call format
        let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
        let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
        let unified_call = emit_unified_mir_call(
            dst.map(|v| v.as_u32()),
            callee.unwrap(),
            &args_u32,
            &effects_str,
        );
        Some(unified_call)
    } else if !use_unified && callee.is_some() {
        // v0: When unified is OFF but callee exists, emit proper v0 format
        match callee.unwrap() {
            Callee::Method {
                method, receiver, ..
            } => {
                // Emit as boxcall for compatibility
                let box_val = receiver.unwrap_or(*func);
                let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
                let mut obj = json!({
                    "op":"boxcall",
                    "box": box_val.as_u32(),
                    "method": method,
                    "args": args_a,
                    "dst": dst.map(|d| d.as_u32())
                });
                // Add dst_type hints for known methods
                let m = method.as_str();
                let dst_ty = if m == "substring"
                    || m == "dirname"
                    || m == "join"
                    || m == "read_all"
                    || m == "read"
                {
                    Some(json!({"kind":"handle","box_type":"StringBox"}))
                } else if m == "length" || m == "lastIndexOf" {
                    Some(json!("i64"))
                } else {
                    None
                };
                if let Some(t) = dst_ty {
                    obj["dst_type"] = t;
                }
                Some(obj)
            }
            Callee::Global(name) => {
                if name == "print" || name == "println" {
                    // Keep v0 print route stable for vm_hako / parity scripts.
                    Some(emit_externcall_with_name(dst, "nyash.console.log", args))
                } else {
                    Some(emit_call_with_callee_v0(
                        dst,
                        func,
                        args,
                        json!({"type":"Global","name":name}),
                    ))
                }
            }
            Callee::Extern(name) => {
                // v0 keeps external route as externcall for compatibility.
                Some(emit_externcall_with_name(dst, name, args))
            }
            Callee::Constructor { box_type } => Some(emit_call_with_callee_v0(
                dst,
                func,
                args,
                json!({"type":"Constructor","name":box_type}),
            )),
            Callee::Value(value) => Some(emit_call_with_callee_v0(
                dst,
                func,
                args,
                json!({"type":"Value","value":value.as_u32()}),
            )),
            Callee::Closure {
                params,
                captures,
                me_capture,
            } => {
                let captures_json: Vec<serde_json::Value> = captures
                    .iter()
                    .map(|(name, vid)| json!([name, vid.as_u32()]))
                    .collect();
                Some(emit_call_with_callee_v0(
                    dst,
                    func,
                    args,
                    json!({
                        "type":"Closure",
                        "params": params,
                        "captures": captures_json,
                        "me_capture": me_capture.map(|v| v.as_u32())
                    }),
                ))
            }
        }
    } else {
        // v0: Legacy call format (no callee info)
        Some(emit_call_with_optional_func(dst, func, args, None))
    }
}

fn emit_call_with_callee_v0(
    dst: &Option<ValueId>,
    func: &ValueId,
    args: &[ValueId],
    callee: serde_json::Value,
) -> serde_json::Value {
    emit_call_with_optional_func(dst, func, args, Some(callee))
}

fn emit_call_with_optional_func(
    dst: &Option<ValueId>,
    func: &ValueId,
    args: &[ValueId],
    callee: Option<serde_json::Value>,
) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    let mut obj = json!({
        "op":"call",
        "args": args_a,
        "dst": dst.map(|d| d.as_u32())
    });
    if *func != ValueId::INVALID {
        obj["func"] = json!(func.as_u32());
    }
    if let Some(c) = callee {
        obj["callee"] = c;
    }
    obj
}

fn emit_externcall_with_name(
    dst: &Option<ValueId>,
    extern_name: &str,
    args: &[ValueId],
) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    let func_name = if let Some(rest) = extern_name.strip_prefix("env.console.") {
        format!("nyash.console.{}", rest)
    } else {
        extern_name.to_string()
    };
    json!({
        "op":"externcall",
        "func": func_name,
        "args": args_a,
        "dst": dst.map(|d| d.as_u32())
    })
}

#[allow(dead_code)]
pub(crate) fn emit_extern_call(
    dst: &Option<ValueId>,
    iface_name: &str,
    method_name: &str,
    args: &[ValueId],
) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    let func_name = if iface_name == "env.console" {
        format!("nyash.console.{}", method_name)
    } else {
        format!("{}.{}", iface_name, method_name)
    };
    let mut obj = json!({
        "op": "externcall",
        "func": func_name,
        "args": args_a,
        "dst": dst.map(|d| d.as_u32()),
    });
    // Minimal dst_type hints for known externs
    if iface_name == "env.console" {
        // console.* returns i64 status (ignored by user code)
        if dst.is_some() {
            obj["dst_type"] = json!("i64");
        }
    }
    obj
}

#[allow(dead_code)]
pub(crate) fn emit_box_call(
    dst: &Option<ValueId>,
    box_val: &ValueId,
    method: &str,
    method_id: Option<&u16>,
    args: &[ValueId],
) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    // Minimal dst_type hints
    let mut obj = json!({
        "op":"boxcall","box": box_val.as_u32(), "method": method, "args": args_a, "dst": dst.map(|d| d.as_u32())
    });
    // Phase 287 P4: Include method_id for universal slot tracking (toString[#0])
    if let Some(mid) = method_id {
        obj["method_id"] = json!(mid);
    }
    let m = method;
    let dst_ty =
        if m == "substring" || m == "dirname" || m == "join" || m == "read_all" || m == "read" {
            Some(json!({"kind":"handle","box_type":"StringBox"}))
        } else if m == "length" || m == "lastIndexOf" {
            Some(json!("i64"))
        } else {
            None
        };
    if let Some(t) = dst_ty {
        obj["dst_type"] = t;
    }
    obj
}

pub(crate) fn emit_new_box(dst: &ValueId, box_type: &str, args: &[ValueId]) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    json!({"op":"newbox","type": box_type, "args": args_a, "dst": dst.as_u32()})
}

pub(crate) fn emit_new_closure(
    dst: &ValueId,
    params: &[String],
    captures: &[(String, ValueId)],
    me: &Option<ValueId>,
) -> serde_json::Value {
    // NewClosure is already canonicalized callsite shape.
    // Emit as unified mir_call(callee=Closure) so vm-hako can dispatch via MirCallHandlerBox.
    let callee = Callee::Closure {
        params: params.to_vec(),
        captures: captures.to_vec(),
        me_capture: *me,
    };
    emit_unified_mir_call(Some(dst.as_u32()), &callee, &[], &[])
}

#[cfg(test)]
mod tests {
    use super::emit_call;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::definitions::Callee;
    use crate::mir::{EffectMask, ValueId};
    use std::sync::{Mutex, OnceLock};

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, &'static str)]) -> Self {
            let mut saved = Vec::with_capacity(vars.len());
            for (k, v) in vars {
                saved.push((*k, std::env::var(k).ok()));
                std::env::set_var(k, v);
            }
            Self { saved }
        }

        fn set_unified_off() -> Self {
            Self::set(&[("NYASH_MIR_UNIFIED_CALL", "0")])
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (k, old) in self.saved.drain(..) {
                if let Some(v) = old {
                    std::env::set_var(k, v);
                } else {
                    std::env::remove_var(k);
                }
            }
        }
    }

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn v0_global_call_with_invalid_func_omits_numeric_func_field() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set_unified_off();
        let v = emit_call(
            &Some(ValueId::new(3)),
            &ValueId::INVALID,
            Some(&Callee::Global("my_func/0".to_string())),
            &[ValueId::new(1)],
            &EffectMask::PURE,
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("call"));
        assert!(v.get("func").is_none(), "func must be omitted when INVALID");
        let callee = v.get("callee").expect("callee must exist");
        assert_eq!(callee.get("type").and_then(|x| x.as_str()), Some("Global"));
        assert_eq!(
            callee.get("name").and_then(|x| x.as_str()),
            Some("my_func/0")
        );
    }

    #[test]
    fn v0_print_global_maps_to_externcall() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set_unified_off();
        let v = emit_call(
            &None,
            &ValueId::INVALID,
            Some(&Callee::Global("print".to_string())),
            &[ValueId::new(7)],
            &EffectMask::IO,
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("externcall"));
        assert_eq!(
            v.get("func").and_then(|x| x.as_str()),
            Some("nyash.console.log")
        );
    }

    #[test]
    fn method_call_prefers_mir_call_when_methodize_is_on_even_if_unified_is_off() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("NYASH_MIR_UNIFIED_CALL", "0"),
            ("HAKO_MIR_BUILDER_METHODIZE", "1"),
        ]);
        let v = emit_call(
            &Some(ValueId::new(9)),
            &ValueId::INVALID,
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2), ValueId::new(3)],
            &EffectMask::IO,
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("mir_call"));
        assert_eq!(
            v["mir_call"]["callee"]["type"].as_str(),
            Some("Method"),
            "methodized Stage1 route must stay on mir_call"
        );
    }

    #[test]
    fn method_call_keeps_boxcall_when_both_unified_and_methodize_are_off() {
        let _lock = env_lock().lock().expect("env lock poisoned");
        let _env = EnvGuard::set(&[
            ("NYASH_MIR_UNIFIED_CALL", "0"),
            ("HAKO_MIR_BUILDER_METHODIZE", "0"),
        ]);
        let v = emit_call(
            &Some(ValueId::new(9)),
            &ValueId::INVALID,
            Some(&Callee::Method {
                box_name: "FileBox".to_string(),
                method: "open".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            &[ValueId::new(2), ValueId::new(3)],
            &EffectMask::IO,
        )
        .expect("must emit call");

        assert_eq!(v.get("op").and_then(|x| x.as_str()), Some("boxcall"));
    }
}
