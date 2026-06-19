use std::collections::BTreeSet;

use serde_json::{json, Value};
use syn::parse::Parser;
use syn::{BinOp, Expr, Lit, Token};

use crate::cli::fail;
use crate::names::{emitted_path, insert_path_name_metadata};
use crate::types::field_name;

#[derive(Default)]
pub(crate) struct ExprContext {
    tuple_struct_names: BTreeSet<String>,
}

impl ExprContext {
    pub(crate) fn new(tuple_struct_names: BTreeSet<String>) -> Self {
        Self { tuple_struct_names }
    }

    fn is_tuple_struct_constructor(&self, callee: &str) -> bool {
        self.tuple_struct_names.contains(callee)
    }
}

pub(crate) fn expr_to_json_with_context(expr: &Expr, context: &ExprContext) -> Value {
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
        Expr::Path(path) => {
            if path.path.segments.is_empty() {
                unsupported_expr("empty path expression")
            } else {
                let mut value = json!({"kind": "Name"});
                insert_path_name_metadata(&mut value, &path.path);
                value
            }
        }
        Expr::Field(field) => json!({
            "kind": "Field",
            "base": expr_to_json_with_context(field.base.as_ref(), context),
            "field": field_name(&field.member),
        }),
        Expr::Index(index) => json!({
            "kind": "Index",
            "target": expr_to_json_with_context(index.expr.as_ref(), context),
            "index": expr_to_json_with_context(index.index.as_ref(), context),
        }),
        Expr::Binary(binary) => json!({
            "kind": "Binary",
            "op": binop(&binary.op),
            "left": expr_to_json_with_context(binary.left.as_ref(), context),
            "right": expr_to_json_with_context(binary.right.as_ref(), context),
        }),
        Expr::Call(call) => {
            let callee = match call.func.as_ref() {
                Expr::Path(path) => emitted_path(&path.path),
                _ => "unsupported_callee".to_string(),
            };
            if let Expr::Path(path) = call.func.as_ref() {
                let segments = crate::names::path_segments(&path.path);
                if matches!(segments.first().map(String::as_str), Some("Self")) {
                    return unsupported_expr(format!(
                        "Self-qualified call expression is out of v0 skeleton scope: {}",
                        segments.join("::")
                    ));
                }
            }
            if context.is_tuple_struct_constructor(&callee) {
                return unsupported_expr(format!(
                    "tuple struct constructor expression is out of v0 skeleton scope: {callee}"
                ));
            }
            let mut value = json!({
                "kind": "Call",
                "callee": callee,
                "args": call
                    .args
                    .iter()
                    .map(|arg| expr_to_json_with_context(arg, context))
                    .collect::<Vec<_>>(),
            });
            if let Expr::Path(path) = call.func.as_ref() {
                let segments = crate::names::path_segments(&path.path);
                if segments.len() > 1 {
                    value["callee_source_path"] = json!(segments);
                }
            }
            value
        }
        Expr::MethodCall(call) => json!({
            "kind": "MethodCall",
            "receiver": expr_to_json_with_context(call.receiver.as_ref(), context),
            "method": call.method.to_string(),
            "args": call
                .args
                .iter()
                .map(|arg| expr_to_json_with_context(arg, context))
                .collect::<Vec<_>>(),
        }),
        Expr::ForLoop(_) => unsupported_expr("Rust for loop expression is out of v0 scope"),
        Expr::Match(_) => unsupported_expr("Rust match expression is out of v0 scope"),
        Expr::Macro(mac) if mac.mac.path.is_ident("vec") => vec_macro_to_json(mac, context),
        Expr::Paren(paren) => expr_to_json_with_context(paren.expr.as_ref(), context),
        Expr::Reference(reference) => expr_to_json_with_context(reference.expr.as_ref(), context),
        _ => unsupported_expr(format!("unsupported expression: {}", expr_kind(expr))),
    }
}

pub(crate) fn unsupported_expr(summary: impl Into<String>) -> Value {
    json!({
        "kind": "Unsupported",
        "reason": summary.into(),
    })
}

fn vec_macro_to_json(expr: &syn::ExprMacro, context: &ExprContext) -> Value {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let elements = parser
        .parse2(expr.mac.tokens.clone())
        .map(|items| {
            items
                .iter()
                .map(|item| expr_to_json_with_context(item, context))
                .collect::<Vec<_>>()
        })
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
