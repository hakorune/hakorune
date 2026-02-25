use crate::mir::{BasicBlockId, ConstValue, ValueId};
use serde_json::Value;

/// Generic const parser used by MIR JSON loaders (v0/v1).
/// Supports minimal set: i64/f64/bool/string and handle(StringBox)->String.
pub fn parse_const_value_generic(value_obj: &Value) -> Result<ConstValue, String> {
    // Shapes:
    // 1) { "type": "i64", "value": 123 }
    // 2) { "type": {"kind":"handle","box_type":"StringBox"}, "value": "str" }
    // 3) When "type" is omitted, infer from value
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

    if let Some(Value::String(s)) = type_desc.as_ref() {
        return match s.as_str() {
            "i64" | "int" => raw_val
                .as_i64()
                .map(ConstValue::Integer)
                .ok_or_else(|| "const value expected integer".to_string()),
            "f64" | "float" => raw_val
                .as_f64()
                .map(ConstValue::Float)
                .ok_or_else(|| "const value expected float".to_string()),
            "i1" | "bool" => Ok(match raw_val {
                Value::Bool(b) => ConstValue::Bool(b),
                Value::Number(n) => ConstValue::Bool(n.as_i64().unwrap_or(0) != 0),
                Value::String(ref s) => ConstValue::Bool(s == "true" || s == "1"),
                _ => ConstValue::Bool(false),
            }),
            "string" | "String" => raw_val
                .as_str()
                .map(|s| ConstValue::String(s.to_string()))
                .ok_or_else(|| "const value expected string".to_string()),
            "void" => Ok(ConstValue::Void),
            other => Err(format!(
                "unsupported const type '{}' in MIR JSON bridge",
                other
            )),
        };
    }

    if let Some(Value::Object(map)) = type_desc.as_ref() {
        if let Some(Value::String(kind)) = map.get("kind") {
            if kind == "handle" {
                if let Some(Value::String(box_type)) = map.get("box_type") {
                    return match box_type.as_str() {
                        "StringBox" => raw_val
                            .as_str()
                            .map(|s| ConstValue::String(s.to_string()))
                            .ok_or_else(|| "StringBox const expects string value".to_string()),
                        other => Err(format!(
                            "unsupported const handle type '{}' in MIR JSON bridge",
                            other
                        )),
                    };
                }
            }
        }
        return Err("unsupported const type object in MIR JSON bridge".to_string());
    }

    match raw_val {
        Value::Number(n) => n
            .as_i64()
            .map(ConstValue::Integer)
            .ok_or_else(|| "integer expected".to_string()),
        Value::Bool(b) => Ok(ConstValue::Bool(b)),
        Value::String(s) => Ok(ConstValue::String(s)),
        _ => Err("const value has unsupported type descriptor".to_string()),
    }
}

/// Parse canonical PHI incoming list from MIR JSON.
///
/// Canonical tuple shape is `[value_id, pred_block_id]`.
pub fn parse_phi_incoming_generic(inst: &Value) -> Result<Vec<(BasicBlockId, ValueId)>, String> {
    let incoming = inst
        .get("incoming")
        .and_then(Value::as_array)
        .ok_or_else(|| "phi incoming missing".to_string())?;
    let mut pairs = Vec::with_capacity(incoming.len());
    for entry in incoming {
        let pair = entry
            .as_array()
            .ok_or_else(|| "phi incoming entry must be array".to_string())?;
        if pair.len() != 2 {
            return Err("phi incoming entry must have 2 elements".into());
        }
        let val = pair[0]
            .as_u64()
            .ok_or_else(|| "phi incoming value must be integer".to_string())?
            as u32;
        let pred_bb = pair[1]
            .as_u64()
            .ok_or_else(|| "phi incoming predecessor block must be integer".to_string())?
            as u32;
        pairs.push((BasicBlockId::new(pred_bb), ValueId::new(val)));
    }
    Ok(pairs)
}
