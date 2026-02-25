use crate::mir::{ConstValue, EffectMask};
use serde_json::Value;

pub(super) fn parse_effects_from(node: &Value) -> EffectMask {
    if let Some(arr) = node.get("effects").and_then(Value::as_array) {
        let mut m = EffectMask::PURE;
        for e in arr {
            if let Some(s) = e.as_str() {
                match s {
                    "write" | "mut" | "WriteHeap" => {
                        m = m.union(EffectMask::WRITE);
                    }
                    "read" | "ReadHeap" => {
                        m = m.union(EffectMask::READ);
                    }
                    "io" | "IO" | "ffi" | "FFI" | "debug" => {
                        m = m.union(EffectMask::IO);
                    }
                    "control" | "Control" => {
                        m = m.union(EffectMask::CONTROL);
                    }
                    _ => {}
                }
            }
        }
        return m;
    }
    EffectMask::PURE
}

#[allow(dead_code)]
pub(super) fn parse_const_value(value_obj: &Value) -> Result<ConstValue, String> {
    // Accept both shapes:
    // 1) { "type": "i64", "value": 123 }
    // 2) { "type": {"kind":"handle","box_type":"StringBox"}, "value": "str" }
    // 3) Minimal fallback: when "type" is omitted, assume integer/string directly
    let (type_desc, raw_val) = if let Some(t) = value_obj.get("type") {
        (
            Some(t.clone()),
            value_obj
                .get("value")
                .cloned()
                .ok_or_else(|| "const value missing 'value' field".to_string())?,
        )
    } else {
        (None, value_obj.clone())
    };

    // String type descriptor
    if let Some(Value::String(s)) = type_desc.as_ref() {
        match s.as_str() {
            // Integer
            "i64" | "int" => {
                let val = raw_val
                    .as_i64()
                    .ok_or_else(|| "const value expected integer".to_string())?;
                return Ok(ConstValue::Integer(val));
            }
            // Float
            "f64" | "float" => {
                let val = raw_val
                    .as_f64()
                    .ok_or_else(|| "const value expected float".to_string())?;
                return Ok(ConstValue::Float(val));
            }
            // Bool (allow explicit bool schema even if current emitter uses i64)
            "i1" | "bool" => {
                let b = match raw_val {
                    Value::Bool(v) => v,
                    Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
                    Value::String(ref s) => s == "true" || s == "1",
                    _ => false,
                };
                return Ok(ConstValue::Bool(b));
            }
            // String explicit
            "string" | "String" => {
                let s = raw_val
                    .as_str()
                    .ok_or_else(|| "const value expected string".to_string())?;
                return Ok(ConstValue::String(s.to_string()));
            }
            // Void/Null
            "void" => {
                return Ok(ConstValue::Void);
            }
            other => {
                return Err(format!(
                    "unsupported const type '{}' in Gate-C v1 bridge",
                    other
                ));
            }
        }
    }

    // Object descriptor (e.g., handle/StringBox)
    if let Some(Value::Object(map)) = type_desc.as_ref() {
        if let Some(Value::String(kind)) = map.get("kind") {
            if kind == "handle" {
                if let Some(Value::String(box_type)) = map.get("box_type") {
                    match box_type.as_str() {
                        // StringBox handle is serialized with raw string payload
                        "StringBox" => {
                            let s = raw_val.as_str().ok_or_else(|| {
                                "StringBox const expects string value".to_string()
                            })?;
                            return Ok(ConstValue::String(s.to_string()));
                        }
                        // Other handle kinds are not yet supported in the bridge
                        other => {
                            return Err(format!(
                                "unsupported const handle type '{}' in Gate-C v1 bridge",
                                other
                            ));
                        }
                    }
                }
            }
        }
        return Err("unsupported const type object in Gate-C v1 bridge".to_string());
    }

    // No explicit type: heuristics
    match raw_val {
        Value::Number(n) => Ok(ConstValue::Integer(
            n.as_i64().ok_or_else(|| "integer expected".to_string())?,
        )),
        Value::Bool(b) => Ok(ConstValue::Bool(b)),
        Value::String(s) => Ok(ConstValue::String(s)),
        _ => Err("const value has unsupported type descriptor".to_string()),
    }
}

pub(super) fn require_u64(node: &Value, key: &str, context: &str) -> Result<u64, String> {
    node.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{} missing field '{}'", context, key))
}

pub(super) fn parse_binop(op: &str) -> Result<crate::mir::types::BinaryOp, String> {
    use crate::mir::types::BinaryOp;
    let bop = match op {
        "+" => BinaryOp::Add,
        "-" => BinaryOp::Sub,
        "*" => BinaryOp::Mul,
        "/" => BinaryOp::Div,
        "%" => BinaryOp::Mod,
        "&" | "bitand" => BinaryOp::BitAnd,
        "|" | "bitor" => BinaryOp::BitOr,
        "^" | "bitxor" => BinaryOp::BitXor,
        "shl" => BinaryOp::Shl,
        "shr" => BinaryOp::Shr,
        "and" => BinaryOp::And,
        "or" => BinaryOp::Or,
        other => return Err(format!("unsupported binop '{}'", other)),
    };
    Ok(bop)
}

pub(super) fn parse_compare(op: &str) -> Result<crate::mir::types::CompareOp, String> {
    use crate::mir::types::CompareOp;
    let cop = match op {
        "==" => CompareOp::Eq,
        "!=" => CompareOp::Ne,
        "<" => CompareOp::Lt,
        "<=" => CompareOp::Le,
        ">" => CompareOp::Gt,
        ">=" => CompareOp::Ge,
        other => return Err(format!("unsupported compare op '{}'", other)),
    };
    Ok(cop)
}
