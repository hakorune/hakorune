use serde_json::Value;

use crate::mir::extern_call_route_plan::{
    extern_call_route_specs, ExternCallRouteKind, ExternCallRouteSpec,
};

fn subset_legacy_externcall_route_spec(func: &str) -> Option<&'static ExternCallRouteSpec> {
    extern_call_route_specs().iter().find(|spec| {
        let spec = **spec;
        subset_legacy_route_kind(spec.kind()) && legacy_subset_symbol_matches(spec, func)
    })
}

fn subset_legacy_route_kind(kind: ExternCallRouteKind) -> bool {
    matches!(
        kind,
        ExternCallRouteKind::EnvGet
            | ExternCallRouteKind::HakoOsvmReserveBytesI64
            | ExternCallRouteKind::HakoOsvmCommitBytesI64
            | ExternCallRouteKind::HakoOsvmDecommitBytesI64
            | ExternCallRouteKind::HakoOsvmUnreserveBytesI64
    )
}

fn legacy_subset_symbol_matches(spec: ExternCallRouteSpec, func: &str) -> bool {
    let base = match spec.kind() {
        ExternCallRouteKind::EnvGet => "env.get",
        ExternCallRouteKind::HakoOsvmReserveBytesI64
        | ExternCallRouteKind::HakoOsvmCommitBytesI64
        | ExternCallRouteKind::HakoOsvmDecommitBytesI64
        | ExternCallRouteKind::HakoOsvmUnreserveBytesI64 => spec.symbol(),
        _ => return false,
    };
    if func == base {
        return true;
    }
    let Some(suffix) = func
        .strip_prefix(base)
        .and_then(|rest| rest.strip_prefix('/'))
    else {
        return false;
    };
    suffix.parse::<usize>().ok() == Some(spec.arity())
}

pub(super) fn validate_spec_backed_externcall_shape(inst: &Value) -> Result<bool, String> {
    let Some(func) = inst.get("func").and_then(|v| v.as_str()) else {
        return Ok(false);
    };
    let Some(spec) = subset_legacy_externcall_route_spec(func) else {
        return Ok(false);
    };
    validate_arity_externcall_shape(inst, legacy_subset_label(spec), spec.arity())?;
    Ok(true)
}

fn legacy_subset_label(spec: &ExternCallRouteSpec) -> &'static str {
    match spec.kind() {
        ExternCallRouteKind::EnvGet => "env.get",
        ExternCallRouteKind::HakoOsvmReserveBytesI64
        | ExternCallRouteKind::HakoOsvmCommitBytesI64
        | ExternCallRouteKind::HakoOsvmDecommitBytesI64
        | ExternCallRouteKind::HakoOsvmUnreserveBytesI64 => spec.symbol(),
        _ => spec.symbol(),
    }
}

fn validate_arity_externcall_shape(inst: &Value, label: &str, arity: usize) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("externcall({}:malformed)", label))?;
    if args.len() != arity {
        return Err(format!("externcall({}:args!={})", label, arity));
    }
    if arity == 1 {
        if args.first().and_then(|v| v.as_u64()).is_none() {
            return Err(format!("externcall({}:arg0:non-reg)", label));
        }
    } else if args.iter().any(|v| v.as_u64().is_none()) {
        return Err(format!("externcall({}:args:non-reg)", label));
    }
    if let Some(dst) = inst.get("dst") {
        if !(dst.is_u64() || dst.is_null()) {
            return Err(format!("externcall({}:dst:non-reg)", label));
        }
    }
    Ok(())
}

pub(super) fn validate_mirbuilder_emit_externcall_shape(inst: &Value) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "externcall(env.mirbuilder.emit:malformed)".to_string())?;
    if args.len() != 1 {
        return Err("externcall(env.mirbuilder.emit:args!=1)".to_string());
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err("externcall(env.mirbuilder.emit:arg0:non-reg)".to_string());
    }
    if let Some(dst) = inst.get("dst") {
        if !dst.is_null() && dst.as_u64().is_none() {
            return Err("externcall(env.mirbuilder.emit:dst:non-reg)".to_string());
        }
    }
    Ok(())
}

pub(super) fn validate_single_arg_externcall_shape(
    inst: &Value,
    label: &str,
) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("externcall({}:malformed)", label))?;
    if args.len() != 1 {
        return Err(format!("externcall({}:args!=1)", label));
    }
    if args.first().and_then(|v| v.as_u64()).is_none() {
        return Err(format!("externcall({}:arg0:non-reg)", label));
    }
    if let Some(dst) = inst.get("dst") {
        if !(dst.is_u64() || dst.is_null()) {
            return Err(format!("externcall({}:dst:non-reg)", label));
        }
    }
    Ok(())
}

pub(super) fn validate_no_arg_externcall_shape(inst: &Value, label: &str) -> Result<(), String> {
    let args = inst
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("externcall({}:malformed)", label))?;
    if !args.is_empty() {
        return Err(format!("externcall({}:args!=0)", label));
    }
    if let Some(dst) = inst.get("dst") {
        if !(dst.is_u64() || dst.is_null()) {
            return Err(format!("externcall({}:dst:non-reg)", label));
        }
    }
    Ok(())
}
