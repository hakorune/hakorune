//! Compatibility-v0 call projection owner (R6-S0 boundary S0-B).
//!
//! Owns the `"boxcall"`/`"externcall"`/`"call"` v0 wire projection and
//! the by-name `dst_type` hint table used when `JsonEgressProfile` is
//! not canonical v1. The v1 canonical writer lives in `calls.rs` via
//! `emit_unified_mir_call`. This compatibility projection is explicit
//! ingress only and must not re-enter canonical production; R6-S3/R7
//! owns its retirement.

use serde_json::json;

use crate::mir::definitions::Callee;
use crate::mir::{EffectMask, ValueId};

use super::super::helpers::emit_unified_mir_call;
use super::super::root::JsonEgressProfile;

pub(crate) fn emit_call_compat_v0(
    dst: &Option<ValueId>,
    func: &ValueId,
    callee: Option<&Callee>,
    args: &[ValueId],
    effects: &EffectMask,
    profile: JsonEgressProfile,
) -> Option<serde_json::Value> {
    // v0: CompatibilityV0 projects an existing Callee without reclassifying it.
    if let Some(callee @ Callee::Method { .. }) = callee {
        if profile.methodize() {
            let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
            let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
            return Some(emit_unified_mir_call(
                dst.map(|v| v.as_u32()),
                callee,
                &args_u32,
                &effects_str,
            ));
        }
    }

    if let Some(callee) = callee {
        match callee {
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
            Callee::SameModuleInstance { key, receiver } => {
                if profile.is_canonical_v1() {
                    let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
                    let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
                    return Some(emit_unified_mir_call(
                        dst.map(|v| v.as_u32()),
                        callee,
                        &args_u32,
                        &effects_str,
                    ));
                }
                Some(emit_call_with_callee_v0(
                    dst,
                    args,
                    json!({
                        "type": "SameModuleInstance",
                        "owner": key.owner(),
                        "name": key.name(),
                        "arity": key.arity(),
                        "receiver": receiver.as_u32()
                    }),
                ))
            }
            Callee::BirthConstructor { key, receiver } => {
                // Display transport only; never disguise Birth as Global/boxcall.
                if profile.is_canonical_v1() {
                    let effects_str: Vec<&str> = if effects.is_io() { vec!["IO"] } else { vec![] };
                    let args_u32: Vec<u32> = args.iter().map(|v| v.as_u32()).collect();
                    return Some(emit_unified_mir_call(
                        dst.map(|v| v.as_u32()),
                        callee,
                        &args_u32,
                        &effects_str,
                    ));
                }
                Some(emit_call_with_callee_v0(
                    dst,
                    args,
                    json!({
                        "type": "BirthConstructor", "namespace": "BirthConstructor",
                        "owner": key.owner(), "name": key.name(), "arity": key.arity(),
                        "receiver": receiver.as_u32()
                    }),
                ))
            }
            Callee::Global(name) => {
                let wire_name = name.display_name();
                if wire_name == "print" || wire_name == "println" {
                    // Keep v0 print route stable for vm_hako / parity scripts.
                    Some(emit_externcall_with_name(dst, "nyash.console.log", args))
                } else {
                    Some(emit_call_with_callee_v0(
                        dst,
                        args,
                        json!({"type":"Global","name":wire_name}),
                    ))
                }
            }
            Callee::Extern(name) => {
                // v0 keeps external route as externcall for compatibility.
                Some(emit_externcall_with_name(dst, name, args))
            }
            Callee::Constructor { box_type } => Some(emit_call_with_callee_v0(
                dst,
                args,
                json!({"type":"Constructor","name":box_type}),
            )),
            Callee::Value(value) => Some(emit_call_with_callee_v0(
                dst,
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
    args: &[ValueId],
    callee: serde_json::Value,
) -> serde_json::Value {
    let args_a: Vec<_> = args.iter().map(|v| json!(v.as_u32())).collect();
    json!({
        "op":"call",
        "args": args_a,
        "dst": dst.map(|d| d.as_u32()),
        "callee": callee
    })
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
