use crate::mir::EffectMask;
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
