use nyash_rust::ast::{BinaryOperator, LiteralValue, ParamDecl, UnaryOperator};
use serde_json::{json, Value};

pub(crate) fn lit_to_json(v: &LiteralValue) -> Value {
    match v {
        LiteralValue::String(s) => json!({"type":"string","value":s}),
        LiteralValue::Integer(i) => json!({"type":"int","value":i}),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => json!({"type":"typed_int","value":value,"declared_type":declared_type_name}),
        LiteralValue::Float(f) => json!({"type":"float","value":f}),
        LiteralValue::Bool(b) => json!({"type":"bool","value":b}),
        LiteralValue::Null => json!({"type":"null"}),
        LiteralValue::Void => json!({"type":"void"}),
    }
}

pub(crate) fn json_to_lit(v: &Value) -> Option<LiteralValue> {
    let t = v.get("type")?.as_str()?;
    Some(match t {
        "string" => LiteralValue::String(v.get("value")?.as_str()?.to_string()),
        "int" => LiteralValue::Integer(v.get("value")?.as_i64()?),
        "typed_int" => LiteralValue::TypedInteger {
            value: v.get("value")?.as_i64()?,
            declared_type_name: v.get("declared_type")?.as_str()?.to_string(),
        },
        "float" => LiteralValue::Float(v.get("value")?.as_f64()?),
        "bool" => LiteralValue::Bool(v.get("value")?.as_bool()?),
        "null" => LiteralValue::Null,
        "void" => LiteralValue::Void,
        _ => return None,
    })
}

pub(crate) fn bin_to_str(op: &BinaryOperator) -> &'static str {
    match op {
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::Modulo => "%",
        BinaryOperator::BitAnd => "&",
        BinaryOperator::BitOr => "|",
        BinaryOperator::BitXor => "^",
        BinaryOperator::Shl => "<<",
        BinaryOperator::Shr => ">>",
        BinaryOperator::Equal => "==",
        BinaryOperator::NotEqual => "!=",
        BinaryOperator::Less => "<",
        BinaryOperator::Greater => ">",
        BinaryOperator::LessEqual => "<=",
        BinaryOperator::GreaterEqual => ">=",
        BinaryOperator::And => "&&",
        BinaryOperator::Or => "||",
    }
}

pub(crate) fn str_to_bin(s: &str) -> Option<BinaryOperator> {
    Some(match s {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Subtract,
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        "%" => BinaryOperator::Modulo,
        "&" => BinaryOperator::BitAnd,
        "|" => BinaryOperator::BitOr,
        "^" => BinaryOperator::BitXor,
        "<<" => BinaryOperator::Shl,
        ">>" => BinaryOperator::Shr,
        "==" => BinaryOperator::Equal,
        "!=" => BinaryOperator::NotEqual,
        "<" => BinaryOperator::Less,
        ">" => BinaryOperator::Greater,
        "<=" => BinaryOperator::LessEqual,
        ">=" => BinaryOperator::GreaterEqual,
        "&&" => BinaryOperator::And,
        "||" => BinaryOperator::Or,
        _ => return None,
    })
}

pub(crate) fn un_to_str(op: &UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Minus => "-",
        UnaryOperator::Not => "!",
        UnaryOperator::BitNot => "~",
        UnaryOperator::Weak => "weak",
    }
}

pub(crate) fn str_to_un(s: &str) -> Option<UnaryOperator> {
    Some(match s {
        "-" => UnaryOperator::Minus,
        "!" => UnaryOperator::Not,
        "~" => UnaryOperator::BitNot,
        "weak" => UnaryOperator::Weak,
        _ => return None,
    })
}

pub(crate) fn is_compare_op(op: &BinaryOperator) -> bool {
    matches!(
        op,
        BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
    )
}

pub(crate) fn param_decls_to_json(param_decls: &[ParamDecl], params: &[String]) -> Vec<Value> {
    ParamDecl::with_name_fallback(param_decls, params)
        .iter()
        .map(|decl| {
            json!({
                "name": decl.name,
                "declared_type": decl.declared_type_name,
            })
        })
        .collect()
}

pub(crate) fn json_to_param_decls(v: &Value, params: &[String]) -> Option<Vec<ParamDecl>> {
    let Some(values) = v.get("param_decls").and_then(Value::as_array) else {
        return Some(ParamDecl::from_names(params));
    };
    if values.len() != params.len() {
        return None;
    }
    let mut decls = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        let name = value.get("name")?.as_str()?.to_string();
        if name != params[index] {
            return None;
        }
        let declared_type_name = value
            .get("declared_type")
            .and_then(Value::as_str)
            .map(str::to_string);
        decls.push(ParamDecl {
            name,
            declared_type_name,
        });
    }
    Some(decls)
}
