use nyash_rust::ast::{ASTNode, DeclarationAttrs, LiteralValue, RuneAttr, Span};
use serde_json::Value;
use std::collections::HashMap;

use super::shared;

pub const SCHEMA: &str = "ast_json_roundtrip_v1";
pub const SCHEMA_VERSION: u32 = 1;

/// Encode AST JSON for tooling/macro pipelines (schema-tagged).
///
/// Note: This currently reuses the JoinIR-compatible exporter for node shapes,
/// and adds a schema tag at the root so callers can distinguish it quickly.
pub fn ast_to_json_roundtrip(ast: &ASTNode) -> Value {
    let mut v = super::joinir_compat::ast_to_json(ast);
    if let Value::Object(ref mut m) = v {
        if m.get("kind").and_then(Value::as_str) == Some("Program") {
            m.insert("schema".to_string(), Value::from(SCHEMA));
            m.insert("schema_version".to_string(), Value::from(SCHEMA_VERSION));
        }
    }
    v
}

/// Decode AST JSON into `ASTNode`.
///
/// Accepts both schema-tagged roundtrip JSON and legacy JoinIR-compatible shapes.
pub fn json_to_ast(v: &Value) -> Option<ASTNode> {
    let k = v.get("kind")?.as_str()?;
    Some(match k {
        "Program" => {
            let stmts = v
                .get("statements")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>();
            ASTNode::Program {
                statements: stmts,
                span: Span::unknown(),
            }
        }
        "BlockExpr" => ASTNode::BlockExpr {
            prelude_stmts: v
                .get("prelude_stmts")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            tail_expr: Box::new(json_to_ast(v.get("tail_expr")?)?),
            span: Span::unknown(),
        },
        "BoxDeclaration" => {
            let methods = v
                .get("methods")?
                .as_array()?
                .iter()
                .filter_map(|m| {
                    Some((
                        m.get("key")?.as_str()?.to_string(),
                        json_to_ast(m.get("decl")?)?,
                    ))
                })
                .collect::<HashMap<String, ASTNode>>();
            let constructors = v
                .get("constructors")?
                .as_array()?
                .iter()
                .filter_map(|c| {
                    Some((
                        c.get("key")?.as_str()?.to_string(),
                        json_to_ast(c.get("decl")?)?,
                    ))
                })
                .collect::<HashMap<String, ASTNode>>();
            let static_init = v.get("static_init").and_then(|s| {
                s.as_array()
                    .map(|arr| arr.iter().filter_map(json_to_ast).collect::<Vec<ASTNode>>())
            });

            ASTNode::BoxDeclaration {
                name: v.get("name")?.as_str()?.to_string(),
                fields: v
                    .get("fields")?
                    .as_array()?
                    .iter()
                    .filter_map(|s| s.as_str().map(|x| x.to_string()))
                    .collect(),
                public_fields: v
                    .get("public_fields")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                private_fields: v
                    .get("private_fields")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                methods,
                constructors,
                init_fields: v
                    .get("init_fields")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                weak_fields: v
                    .get("weak_fields")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                is_interface: v
                    .get("is_interface")
                    .and_then(|b| b.as_bool())
                    .unwrap_or(false),
                extends: v
                    .get("extends")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                implements: v
                    .get("implements")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                type_parameters: v
                    .get("type_parameters")
                    .and_then(|a| a.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|x| x.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default(),
                is_static: v
                    .get("is_static")
                    .and_then(|b| b.as_bool())
                    .unwrap_or(false),
                static_init,
                attrs: json_to_attrs(v.get("attrs")),
                span: Span::unknown(),
            }
        }
        "Loop" => ASTNode::Loop {
            condition: Box::new(json_to_ast(v.get("condition")?)?),
            body: v
                .get("body")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            span: Span::unknown(),
        },
        "Print" => ASTNode::Print {
            expression: Box::new(json_to_ast(v.get("expression")?)?),
            span: Span::unknown(),
        },
        "Return" => ASTNode::Return {
            value: v.get("value").and_then(json_to_ast).map(Box::new),
            span: Span::unknown(),
        },
        "Break" => ASTNode::Break {
            span: Span::unknown(),
        },
        "Continue" => ASTNode::Continue {
            span: Span::unknown(),
        },
        "Assignment" => ASTNode::Assignment {
            target: Box::new(if let Some(lhs) = v.get("lhs").and_then(json_to_ast) {
                lhs
            } else if let Some(name) = v.get("target").and_then(|t| t.as_str()) {
                ASTNode::Variable {
                    name: name.to_string(),
                    span: Span::unknown(),
                }
            } else {
                json_to_ast(v.get("target")?)?
            }),
            value: Box::new(json_to_ast(v.get("value")?)?),
            span: Span::unknown(),
        },
        "Local" => {
            let vars = v
                .get("variables")?
                .as_array()?
                .iter()
                .filter_map(|s| s.as_str().map(|x| x.to_string()))
                .collect();
            let inits = v
                .get("inits")?
                .as_array()?
                .iter()
                .map(|initv| {
                    if initv.is_null() {
                        None
                    } else {
                        json_to_ast(initv).map(Box::new)
                    }
                })
                .collect();
            ASTNode::Local {
                variables: vars,
                initial_values: inits,
                span: Span::unknown(),
            }
        }
        "If" => ASTNode::If {
            condition: Box::new(json_to_ast(v.get("condition")?)?),
            then_body: v
                .get("then")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            else_body: v.get("else").and_then(|a| {
                a.as_array()
                    .map(|arr| arr.iter().filter_map(json_to_ast).collect::<Vec<_>>())
            }),
            span: Span::unknown(),
        },
        "FunctionDeclaration" => ASTNode::FunctionDeclaration {
            name: v.get("name")?.as_str()?.to_string(),
            params: v
                .get("params")?
                .as_array()?
                .iter()
                .filter_map(|s| s.as_str().map(|x| x.to_string()))
                .collect(),
            body: v
                .get("body")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect(),
            is_static: v.get("static").and_then(|b| b.as_bool()).unwrap_or(false),
            is_override: v.get("override").and_then(|b| b.as_bool()).unwrap_or(false),
            attrs: json_to_attrs(v.get("attrs")),
            span: Span::unknown(),
        },
        "Variable" => ASTNode::Variable {
            name: v.get("name")?.as_str()?.to_string(),
            span: Span::unknown(),
        },
        "Literal" => {
            let value = if let Some(nested) = v.get("value").filter(|vv| vv.is_object()) {
                // Older/lit_to_json format: { kind:"Literal", value:{type:"int",...} }
                shared::json_to_lit(nested)?
            } else if let Some(t) = v.get("type").and_then(|t| t.as_str()) {
                // JoinIR-compatible format: { kind:"Literal", type:"Int", value:42 }
                match t {
                    "Int" => LiteralValue::Integer(v.get("value")?.as_i64()?),
                    "Float" => LiteralValue::Float(v.get("value")?.as_f64()?),
                    "Bool" => LiteralValue::Bool(v.get("value")?.as_bool()?),
                    "String" => LiteralValue::String(v.get("value")?.as_str()?.to_string()),
                    "Null" => LiteralValue::Null,
                    "Void" => LiteralValue::Void,
                    _ => return None,
                }
            } else {
                return None;
            };
            ASTNode::Literal {
                value,
                span: Span::unknown(),
            }
        }
        "BinaryOp" => ASTNode::BinaryOp {
            operator: shared::str_to_bin(v.get("op")?.as_str()?)?,
            left: Box::new(json_to_ast(v.get("left")?)?),
            right: Box::new(json_to_ast(v.get("right")?)?),
            span: Span::unknown(),
        },
        "UnaryOp" => ASTNode::UnaryOp {
            operator: shared::str_to_un(v.get("op")?.as_str()?)?,
            operand: Box::new(json_to_ast(v.get("operand")?)?),
            span: Span::unknown(),
        },
        "MethodCall" => ASTNode::MethodCall {
            object: Box::new(json_to_ast(v.get("object")?)?),
            method: v.get("method")?.as_str()?.to_string(),
            arguments: v
                .get("arguments")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect(),
            span: Span::unknown(),
        },
        "FunctionCall" => ASTNode::FunctionCall {
            name: v.get("name")?.as_str()?.to_string(),
            arguments: v
                .get("arguments")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect(),
            span: Span::unknown(),
        },
        "Array" => ASTNode::ArrayLiteral {
            elements: v
                .get("elements")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect(),
            span: Span::unknown(),
        },
        "Map" => ASTNode::MapLiteral {
            entries: v
                .get("entries")?
                .as_array()?
                .iter()
                .filter_map(|e| {
                    Some((e.get("k")?.as_str()?.to_string(), json_to_ast(e.get("v")?)?))
                })
                .collect(),
            span: Span::unknown(),
        },
        "MatchExpr" => {
            let scr = json_to_ast(v.get("scrutinee")?)?;
            let arms_json = v.get("arms")?.as_array()?.iter();
            let mut arms = Vec::new();
            for arm_v in arms_json {
                let lit_val = arm_v.get("literal")?.get("value")?;
                let lit = shared::json_to_lit(lit_val)?;
                let body = json_to_ast(arm_v.get("body")?)?;
                arms.push((lit, body));
            }
            let else_expr = json_to_ast(v.get("else")?)?;
            ASTNode::MatchExpr {
                scrutinee: Box::new(scr),
                arms,
                else_expr: Box::new(else_expr),
                span: Span::unknown(),
            }
        }
        "TryCatch" => {
            let try_b = v
                .get("try")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>();
            let mut catches = Vec::new();
            if let Some(arr) = v.get("catch").and_then(|x| x.as_array()) {
                for c in arr.iter() {
                    let exc_t = match c.get("type") {
                        Some(t) if !t.is_null() => t.as_str().map(|s| s.to_string()),
                        _ => None,
                    };
                    let var = match c.get("var") {
                        Some(vv) if !vv.is_null() => vv.as_str().map(|s| s.to_string()),
                        _ => None,
                    };
                    let body = c
                        .get("body")?
                        .as_array()?
                        .iter()
                        .filter_map(json_to_ast)
                        .collect::<Vec<_>>();
                    catches.push(nyash_rust::ast::CatchClause {
                        exception_type: exc_t,
                        variable_name: var,
                        body,
                        span: Span::unknown(),
                    });
                }
            }
            let cleanup = v.get("cleanup").and_then(|cl| {
                cl.as_array()
                    .map(|arr| arr.iter().filter_map(json_to_ast).collect::<Vec<_>>())
            });
            ASTNode::TryCatch {
                try_body: try_b,
                catch_clauses: catches,
                finally_body: cleanup,
                span: Span::unknown(),
            }
        }
        _ => return None,
    })
}

fn json_to_attrs(value: Option<&Value>) -> DeclarationAttrs {
    let runes = value
        .and_then(|attrs| attrs.get("runes"))
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    Some(RuneAttr {
                        name: entry.get("name")?.as_str()?.to_string(),
                        args: entry
                            .get("args")
                            .and_then(Value::as_array)
                            .map(|args| {
                                args.iter()
                                    .filter_map(|arg| arg.as_str().map(|s| s.to_string()))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    DeclarationAttrs { runes }
}
