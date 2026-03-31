use crate::mir::{
    BarrierOp, ConstValue, MirType, TypeOpKind, ValueId,
};
use serde_json::Value;

pub(super) fn require_u64(node: &Value, key: &str, context: &str) -> Result<u64, String> {
    node.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{} missing field '{}'", context, key))
}

pub(super) fn parse_value_id_array(
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

pub(super) fn parse_function_param_ids(func: &Value, func_name: &str) -> Result<Vec<u32>, String> {
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

pub(super) fn parse_const_value(value_obj: &Value) -> Result<ConstValue, String> {
    // Delegate to common generic parser (int/float/bool/string/handle(StringBox)).
    // Keeps behavior superset of previous (int-only) without changing existing callers.
    crate::runner::mir_json::common::parse_const_value_generic(value_obj)
        .map_err(|e| format!("{}", e))
}

pub(super) fn parse_compare(op: &str) -> Result<crate::mir::types::CompareOp, String> {
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

pub(super) fn parse_binop(op: &str) -> Result<crate::mir::types::BinaryOp, String> {
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

pub(super) fn parse_barrier_op(raw: &str, field: &str) -> Result<BarrierOp, String> {
    match raw {
        "read" | "Read" => Ok(BarrierOp::Read),
        "write" | "Write" => Ok(BarrierOp::Write),
        _ => Err(format!("unsupported barrier {} '{}'", field, raw)),
    }
}

pub(super) fn parse_typeop_kind(inst: &Value) -> Result<TypeOpKind, String> {
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

pub(super) fn parse_typeop_target_type(inst: &Value) -> Result<MirType, String> {
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
