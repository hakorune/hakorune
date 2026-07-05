use serde_json::json;

use crate::mir::{BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, ValueId};

fn mir_type_is_string_like(ty: Option<&MirType>) -> bool {
    match ty {
        Some(MirType::String) => true,
        Some(MirType::Box(bt)) if bt == "StringBox" => true,
        _ => false,
    }
}

fn mir_type_allows_string_compare(ty: Option<&MirType>) -> bool {
    match ty {
        None | Some(MirType::Unknown) => true,
        other => mir_type_is_string_like(other),
    }
}

pub(crate) fn emit_copy(dst: &ValueId, src: &ValueId) -> serde_json::Value {
    json!({"op":"copy","dst": dst.as_u32(), "src": src.as_u32()})
}

pub(crate) fn emit_unary_op(dst: &ValueId, op: &UnaryOp, operand: &ValueId) -> serde_json::Value {
    let kind = match op {
        UnaryOp::Neg => "neg",
        UnaryOp::Not => "not",
        UnaryOp::BitNot => "bitnot",
    };
    json!({"op":"unop","operation": kind, "src": operand.as_u32(), "dst": dst.as_u32()})
}

pub(crate) fn emit_const(dst: &ValueId, value: &ConstValue) -> serde_json::Value {
    match value {
        ConstValue::Integer(i) => {
            json!({"op":"const","dst": dst.as_u32(), "value": {"type": "i64", "value": i}})
        }
        ConstValue::Float(fv) => {
            json!({"op":"const","dst": dst.as_u32(), "value": {"type": "f64", "value": fv}})
        }
        ConstValue::Bool(b) => {
            json!({"op":"const","dst": dst.as_u32(), "value": {"type": "i64", "value": if *b {1} else {0}}})
        }
        ConstValue::String(s) => json!({
            "op":"const",
            "dst": dst.as_u32(),
            "value": {
                "type": {"kind":"handle","box_type":"StringBox"},
                "value": s
            }
        }),
        ConstValue::Null | ConstValue::Void => {
            json!({"op":"const","dst": dst.as_u32(), "value": {"type": "void", "value": 0}})
        }
    }
}

pub(crate) fn emit_static_data_load(
    dst: &ValueId,
    source_name: &str,
    symbol: &str,
    element: &str,
    len: u32,
    align: u32,
    index: &ValueId,
) -> serde_json::Value {
    json!({
        "op": "static_data_load",
        "dst": dst.as_u32(),
        "source_name": source_name,
        "symbol": symbol,
        "element": element,
        "len": len,
        "align": align,
        "index": index.as_u32(),
    })
}

pub(crate) fn emit_type_op(
    dst: &ValueId,
    op: &TypeOpKind,
    value: &ValueId,
    ty: &MirType,
) -> serde_json::Value {
    let op_s = match op {
        TypeOpKind::Check => "check",
        TypeOpKind::Cast => "cast",
    };
    let ty_s = match ty {
        MirType::Integer => "Integer".to_string(),
        MirType::Float => "Float".to_string(),
        MirType::Bool => "Bool".to_string(),
        MirType::String => "String".to_string(),
        MirType::Void => "Void".to_string(),
        MirType::Box(name) => name.clone(),
        _ => "Unknown".to_string(),
    };
    json!({
        "op":"typeop",
        "operation": op_s,
        "src": value.as_u32(),
        "dst": dst.as_u32(),
        "target_type": ty_s,
    })
}

pub(crate) fn emit_bin_op(
    dst: &ValueId,
    op: &BinaryOp,
    lhs: &ValueId,
    rhs: &ValueId,
    value_types: &std::collections::BTreeMap<ValueId, MirType>,
) -> serde_json::Value {
    let op_s = match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::BitAnd => "&",
        BinaryOp::BitOr => "|",
        BinaryOp::BitXor => "^",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
        BinaryOp::And => "&",
        BinaryOp::Or => "|",
    };
    let mut obj = json!({"op":"binop","operation": op_s, "lhs": lhs.as_u32(), "rhs": rhs.as_u32(), "dst": dst.as_u32()});
    // Phase 131-15-P1: dst_type only when type is KNOWN (not Unknown)
    // Operand TypeFacts take priority over dst_type hint in Python
    if matches!(op, BinaryOp::Add) {
        if mir_type_is_string_like(value_types.get(lhs))
            || mir_type_is_string_like(value_types.get(rhs))
        {
            obj["dst_type"] = json!({"kind":"handle","box_type":"StringBox"});
            return obj;
        }

        let dst_type = value_types.get(dst);
        match dst_type {
            Some(MirType::Box(bt)) if bt == "StringBox" => {
                obj["dst_type"] = json!({"kind":"handle","box_type":"StringBox"});
            }
            Some(MirType::Integer) => {
                // Explicitly mark as i64 for integer addition
                obj["dst_type"] = json!("i64");
            }
            Some(MirType::Unknown) | None => {
                // Unknown: DO NOT emit dst_type
                // Let Python side infer from operand TypeFacts
            }
            _ => {
                // Other known types: use conservative i64
                obj["dst_type"] = json!("i64");
            }
        }
    }
    obj
}

pub(crate) fn emit_compare(
    dst: &ValueId,
    op: &CompareOp,
    lhs: &ValueId,
    rhs: &ValueId,
    value_types: &std::collections::BTreeMap<ValueId, MirType>,
) -> serde_json::Value {
    let op_s = match op {
        CompareOp::Ge => ">=",
        CompareOp::Le => "<=",
        CompareOp::Gt => ">",
        CompareOp::Lt => "<",
        CompareOp::Eq => "==",
        CompareOp::Ne => "!=",
    };
    let mut obj = json!({"op":"compare","operation": op_s, "lhs": lhs.as_u32(), "rhs": rhs.as_u32(), "dst": dst.as_u32()});
    // cmp_kind hint for string equality
    if matches!(op, CompareOp::Eq | CompareOp::Ne) {
        let lhs_ty = value_types.get(lhs);
        let rhs_ty = value_types.get(rhs);
        let lhs_is_str = mir_type_is_string_like(lhs_ty);
        let rhs_is_str = mir_type_is_string_like(rhs_ty);
        if (lhs_is_str && mir_type_allows_string_compare(rhs_ty))
            || (rhs_is_str && mir_type_allows_string_compare(lhs_ty))
        {
            obj["cmp_kind"] = json!("string");
        }
    }
    obj
}

pub(crate) fn emit_select(
    dst: &ValueId,
    cond: &ValueId,
    then_val: &ValueId,
    else_val: &ValueId,
) -> serde_json::Value {
    json!({
        "op":"select",
        "dst": dst.as_u32(),
        "cond": cond.as_u32(),
        "then_val": then_val.as_u32(),
        "else_val": else_val.as_u32()
    })
}

pub(crate) fn emit_debug(value: &ValueId, message: &str) -> serde_json::Value {
    json!({
        "op":"debug",
        "value": value.as_u32(),
        "message": message
    })
}

pub(crate) fn emit_safepoint() -> serde_json::Value {
    json!({"op": "safepoint"})
}

pub(crate) fn emit_future_new(dst: &ValueId, value: &ValueId) -> serde_json::Value {
    json!({
        "op": "future_new",
        "dst": dst.as_u32(),
        "value": value.as_u32(),
    })
}

pub(crate) fn emit_future_set(future: &ValueId, value: &ValueId) -> serde_json::Value {
    json!({
        "op": "future_set",
        "future": future.as_u32(),
        "value": value.as_u32(),
    })
}

pub(crate) fn emit_await(dst: &ValueId, future: &ValueId) -> serde_json::Value {
    json!({
        "op": "await",
        "dst": dst.as_u32(),
        "future": future.as_u32(),
    })
}
