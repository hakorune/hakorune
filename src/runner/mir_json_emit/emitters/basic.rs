use serde_json::json;

use crate::mir::{BinaryOp, CompareOp, ConstValue, MirType, TypeOpKind, UnaryOp, ValueId};

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
        let lhs_is_str = match value_types.get(lhs) {
            Some(MirType::String) => true,
            Some(MirType::Box(bt)) if bt == "StringBox" => true,
            _ => false,
        };
        let rhs_is_str = match value_types.get(rhs) {
            Some(MirType::String) => true,
            Some(MirType::Box(bt)) if bt == "StringBox" => true,
            _ => false,
        };
        if lhs_is_str && rhs_is_str {
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
