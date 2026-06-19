use serde_json::{json, Value};
use syn::parse::Parser;
use syn::{BinOp, Expr, Lit, Token};

use crate::cli::fail;
use crate::types::field_name;

pub(crate) fn expr_to_json(expr: &Expr) -> Value {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(value) => json!({
                "kind": "Literal",
                "type": "i64",
                "value": value.base10_parse::<i64>().unwrap_or_else(|err| {
                    fail(format!("integer literal out of v0 range: {err}"))
                }),
            }),
            Lit::Bool(value) => json!({
                "kind": "Literal",
                "type": "bool",
                "value": value.value,
            }),
            Lit::Str(value) => json!({
                "kind": "Literal",
                "type": "String",
                "value": value.value(),
            }),
            _ => unsupported_expr("unsupported literal"),
        },
        Expr::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| json!({"kind": "Name", "name": segment.ident.to_string()}))
            .unwrap_or_else(|| unsupported_expr("empty path expression")),
        Expr::Field(field) => json!({
            "kind": "Field",
            "base": expr_to_json(field.base.as_ref()),
            "field": field_name(&field.member),
        }),
        Expr::Index(index) => json!({
            "kind": "Index",
            "target": expr_to_json(index.expr.as_ref()),
            "index": expr_to_json(index.index.as_ref()),
        }),
        Expr::Binary(binary) => json!({
            "kind": "Binary",
            "op": binop(&binary.op),
            "left": expr_to_json(binary.left.as_ref()),
            "right": expr_to_json(binary.right.as_ref()),
        }),
        Expr::Call(call) => {
            let callee = match call.func.as_ref() {
                Expr::Path(path) => path
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string())
                    .unwrap_or_else(|| "unsupported_callee".to_string()),
                _ => "unsupported_callee".to_string(),
            };
            json!({
                "kind": "Call",
                "callee": callee,
                "args": call.args.iter().map(expr_to_json).collect::<Vec<_>>(),
            })
        }
        Expr::MethodCall(call) => json!({
            "kind": "MethodCall",
            "receiver": expr_to_json(call.receiver.as_ref()),
            "method": call.method.to_string(),
            "args": call.args.iter().map(expr_to_json).collect::<Vec<_>>(),
        }),
        Expr::ForLoop(_) => unsupported_expr("Rust for loop expression is out of v0 scope"),
        Expr::Match(_) => unsupported_expr("Rust match expression is out of v0 scope"),
        Expr::Macro(mac) if mac.mac.path.is_ident("vec") => vec_macro_to_json(mac),
        Expr::Paren(paren) => expr_to_json(paren.expr.as_ref()),
        Expr::Reference(reference) => expr_to_json(reference.expr.as_ref()),
        _ => unsupported_expr(format!("unsupported expression: {}", expr_kind(expr))),
    }
}

pub(crate) fn unsupported_expr(summary: impl Into<String>) -> Value {
    json!({
        "kind": "Unsupported",
        "reason": summary.into(),
    })
}

fn vec_macro_to_json(expr: &syn::ExprMacro) -> Value {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let elements = parser
        .parse2(expr.mac.tokens.clone())
        .map(|items| items.iter().map(expr_to_json).collect::<Vec<_>>())
        .unwrap_or_else(|err| {
            vec![unsupported_expr(format!(
                "unsupported vec! literal payload: {err}"
            ))]
        });
    json!({
        "kind": "ArrayLiteral",
        "elements": elements,
    })
}

fn binop(op: &BinOp) -> &'static str {
    match op {
        BinOp::Add(_) => "+",
        BinOp::Sub(_) => "-",
        BinOp::Mul(_) => "*",
        BinOp::Div(_) => "/",
        BinOp::Rem(_) => "%",
        BinOp::And(_) => "&&",
        BinOp::Or(_) => "||",
        BinOp::Eq(_) => "==",
        BinOp::Ne(_) => "!=",
        BinOp::Lt(_) => "<",
        BinOp::Le(_) => "<=",
        BinOp::Gt(_) => ">",
        BinOp::Ge(_) => ">=",
        _ => "unsupported_op",
    }
}

fn expr_kind(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "Array",
        Expr::Assign(_) => "Assign",
        Expr::Async(_) => "Async",
        Expr::Await(_) => "Await",
        Expr::Block(_) => "Block",
        Expr::Break(_) => "Break",
        Expr::Cast(_) => "Cast",
        Expr::Closure(_) => "Closure",
        Expr::Const(_) => "Const",
        Expr::Continue(_) => "Continue",
        Expr::ForLoop(_) => "ForLoop",
        Expr::If(_) => "If",
        Expr::Index(_) => "Index",
        Expr::Let(_) => "Let",
        Expr::Loop(_) => "Loop",
        Expr::Macro(_) => "Macro",
        Expr::Match(_) => "Match",
        Expr::Range(_) => "Range",
        Expr::Repeat(_) => "Repeat",
        Expr::Return(_) => "Return",
        Expr::Struct(_) => "Struct",
        Expr::Try(_) => "Try",
        Expr::Tuple(_) => "Tuple",
        Expr::Unary(_) => "Unary",
        Expr::Unsafe(_) => "Unsafe",
        Expr::While(_) => "While",
        Expr::Yield(_) => "Yield",
        _ => "Other",
    }
}
