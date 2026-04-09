use nyash_rust::ast::{ASTNode, EnumVariantDecl, FieldDecl, LiteralValue, Span};
use serde_json::{json, Value};
use std::collections::HashMap;

use super::shared;
mod helpers;
use helpers::{
    attrs_to_json, json_to_attrs, json_to_lit, literal_to_joinir_json, str_to_bin, str_to_un,
};

pub fn ast_to_json(ast: &ASTNode) -> Value {
    match ast.clone() {
        ASTNode::Program { statements, .. } => json!({
            "kind": "Program",
            "statements": statements.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => json!({
            "kind": "BlockExpr",
            "prelude_stmts": prelude_stmts.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "tail_expr": ast_to_json(&tail_expr),
        }),
        ASTNode::BoxDeclaration {
            name,
            fields,
            field_decls,
            public_fields,
            private_fields,
            methods,
            constructors,
            init_fields,
            weak_fields,
            is_interface,
            extends,
            implements,
            type_parameters,
            is_static,
            static_init,
            attrs,
            ..
        } => json!({
            "kind": "BoxDeclaration",
            "name": name,
            "fields": fields,
            "field_decls": field_decls.into_iter().map(|decl| json!({
                "name": decl.name,
                "declared_type": decl.declared_type_name,
                "is_weak": decl.is_weak,
            })).collect::<Vec<_>>(),
            "public_fields": public_fields,
            "private_fields": private_fields,
            "methods": methods
                .into_iter()
                .map(|(key, decl)| json!({"key": key, "decl": ast_to_json(&decl)}))
                .collect::<Vec<_>>(),
            "constructors": constructors
                .into_iter()
                .map(|(key, decl)| json!({"key": key, "decl": ast_to_json(&decl)}))
                .collect::<Vec<_>>(),
            "init_fields": init_fields,
            "weak_fields": weak_fields,
            "is_interface": is_interface,
            "extends": extends,
            "implements": implements,
            "type_parameters": type_parameters,
            "is_static": is_static,
            "static_init": static_init.map(|stmts| stmts.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()),
            "attrs": attrs_to_json(&attrs),
        }),
        ASTNode::EnumDeclaration {
            name,
            variants,
            type_parameters,
            attrs,
            ..
        } => json!({
            "kind": "EnumDeclaration",
            "name": name,
            "variants": variants.into_iter().map(|variant| json!({
                "name": variant.name,
                "payload_type": variant.payload_type_name,
                "record_fields": variant.record_field_decls.into_iter().map(|field| json!({
                    "name": field.name,
                    "declared_type": field.declared_type_name,
                    "is_weak": field.is_weak,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "type_parameters": type_parameters,
            "attrs": attrs_to_json(&attrs),
        }),
        // Phase 54: Loop with JoinIR-compatible fields
        ASTNode::Loop {
            condition, body, ..
        } => json!({
            "kind": "Loop",
            "type": "Loop",  // JoinIR Frontend expects "type"
            "condition": ast_to_json(&condition),
            "cond": ast_to_json(&condition),  // JoinIR expects "cond"
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
        }),
        // Phase 54: Print with JoinIR-compatible fields
        ASTNode::Print { expression, .. } => json!({
            "kind": "Print",
            "type": "Print",  // JoinIR Frontend expects "type"
            "expression": ast_to_json(&expression),
            "expr": ast_to_json(&expression),  // JoinIR expects "expr"
        }),
        // Phase 54: Return with JoinIR-compatible fields
        ASTNode::Return { value, .. } => json!({
            "kind": "Return",
            "type": "Return",  // JoinIR Frontend expects "type"
            "value": value.as_ref().map(|v| ast_to_json(v)),
        }),
        // Phase 56: Break with JoinIR-compatible type field
        ASTNode::Break { .. } => json!({
            "kind": "Break",
            "type": "Break"  // JoinIR Frontend expects "type"
        }),
        // Phase 56: Continue with JoinIR-compatible type field
        ASTNode::Continue { .. } => json!({
            "kind": "Continue",
            "type": "Continue"  // JoinIR Frontend expects "type"
        }),
        // Phase 54: Assignment with JoinIR-compatible fields
        ASTNode::Assignment { target, value, .. } => {
            // Extract variable name if target is a simple Variable
            let target_str = match target.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => "complex".to_string(), // FieldAccess or other complex target
            };
            json!({
                "kind": "Assignment",
                "type": "Assignment",  // JoinIR Frontend expects "type"
                "target": target_str,  // JoinIR expects string variable name
                "lhs": ast_to_json(&target),  // Keep full AST for complex cases
                "value": ast_to_json(&value),
                "expr": ast_to_json(&value),  // JoinIR expects "expr"
            })
        }
        // Phase 54: Local with JoinIR-compatible fields
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            // For single-variable declarations, add "name" and "expr" for JoinIR compatibility
            let (name, expr) = if variables.len() == 1 {
                let n = variables[0].clone();
                let e = initial_values
                    .get(0)
                    .and_then(|opt| opt.as_ref())
                    .map(|v| ast_to_json(v));
                (Some(n), e)
            } else {
                (None, None)
            };

            let inits: Vec<_> = initial_values
                .into_iter()
                .map(|opt| opt.map(|v| ast_to_json(&v)))
                .collect();

            json!({
                "kind": "Local",
                "type": "Local",  // JoinIR Frontend expects "type"
                "name": name,  // Single variable name for JoinIR (null if multiple)
                "expr": expr,  // Single variable init for JoinIR (null if multiple)
                "variables": variables,
                "inits": inits
            })
        }
        // Phase 54: If with JoinIR-compatible fields
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => json!({
            "kind": "If",
            "type": "If",  // JoinIR Frontend expects "type"
            "condition": ast_to_json(&condition),
            "cond": ast_to_json(&condition),  // JoinIR expects "cond"
            "then": then_body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "else": else_body.map(|v| v.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()),
        }),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => json!({
            "kind": "TryCatch",
            "try": try_body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "catch": catch_clauses.into_iter().map(|cc| json!({
                "type": cc.exception_type,
                "var": cc.variable_name,
                "body": cc.body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>()
            })).collect::<Vec<_>>(),
            "cleanup": finally_body.map(|v| v.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>())
        }),
        ASTNode::FunctionDeclaration {
            name,
            params,
            body,
            is_static,
            is_override,
            attrs,
            ..
        } => json!({
            "kind": "FunctionDeclaration",
            "name": name,
            "params": params,
            "body": body.into_iter().map(|s| ast_to_json(&s)).collect::<Vec<_>>(),
            "static": is_static,
            "override": is_override,
            "attrs": attrs_to_json(&attrs),
        }),
        // Phase 52: Variable → Var ノード（JoinIR Frontend 互換）
        ASTNode::Variable { name, .. } => json!({
            "kind": "Variable",
            "type": "Var",  // JoinIR Frontend expects "type": "Var"
            "name": name
        }),
        // Phase 52: Literal → Int/Bool/String ノード（JoinIR Frontend 互換）
        ASTNode::Literal { value, .. } => literal_to_joinir_json(&value),
        // Phase 52: BinaryOp → Binary/Compare ノード（JoinIR Frontend 互換）
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let op_str = shared::bin_to_str(&operator);
            // JoinIR Frontend distinguishes between Binary (arithmetic) and Compare
            let type_str = if shared::is_compare_op(&operator) {
                "Compare"
            } else {
                "Binary"
            };
            json!({
                "kind": "BinaryOp",
                "type": type_str,
                "op": op_str,
                // JoinIR Frontend expects "lhs"/"rhs" not "left"/"right"
                "lhs": ast_to_json(&left),
                "rhs": ast_to_json(&right),
                // Also keep "left"/"right" for backward compatibility
                "left": ast_to_json(&left),
                "right": ast_to_json(&right),
            })
        }
        // Phase 56: UnaryOp → Unary ノード（JoinIR Frontend 互換）
        ASTNode::UnaryOp {
            operator, operand, ..
        } => json!({
            "kind": "UnaryOp",
            "type": "Unary",  // Phase 56: JoinIR Frontend expects "type" field
            "op": shared::un_to_str(&operator),
            "operand": ast_to_json(&operand),
        }),
        // Phase 52: MethodCall → Method ノード（JoinIR Frontend 互換）
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => json!({
            "kind": "MethodCall",
            "type": "Method",  // JoinIR Frontend expects "type": "Method"
            // JoinIR Frontend expects "receiver" not "object"
            "receiver": ast_to_json(&object),
            "object": ast_to_json(&object),  // Keep for backward compatibility
            "method": method,
            // JoinIR Frontend expects "args" not "arguments"
            "args": arguments.iter().map(|a| ast_to_json(a)).collect::<Vec<_>>(),
            "arguments": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>()  // Keep for backward compatibility
        }),
        // Phase 56: FunctionCall with JoinIR-compatible type field
        ASTNode::FunctionCall {
            name, arguments, ..
        } => json!({
            "kind": "FunctionCall",
            "type": "Call",  // JoinIR Frontend expects "type": "Call"
            "name": name,
            "func": name.clone(),  // JoinIR expects "func" for function name
            "args": arguments.iter().map(|a| ast_to_json(a)).collect::<Vec<_>>(),  // JoinIR expects "args"
            "arguments": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>()  // Keep for backward compatibility
        }),
        // Phase 56: ArrayLiteral with JoinIR-compatible type field
        ASTNode::ArrayLiteral { elements, .. } => json!({
            "kind": "Array",
            "type": "Array",  // JoinIR Frontend expects "type"
            "elements": elements.into_iter().map(|e| ast_to_json(&e)).collect::<Vec<_>>()
        }),
        // Phase 56: MapLiteral with JoinIR-compatible type field
        ASTNode::MapLiteral { entries, .. } => json!({
            "kind": "Map",
            "type": "Map",  // JoinIR Frontend expects "type"
            "entries": entries.into_iter().map(|(k,v)| json!({"k":k,"v":ast_to_json(&v)})).collect::<Vec<_>>()
        }),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => json!({
        "kind":"MatchExpr",
        "scrutinee": ast_to_json(&scrutinee),
            "arms": arms.into_iter().map(|(lit, body)| json!({
                "literal": {
                    "kind": "Literal",
                    "value": shared::lit_to_json(&lit)
                },
                "body": ast_to_json(&body)
            })).collect::<Vec<_>>(),
            "else": ast_to_json(&else_expr),
        }),
        ASTNode::EnumMatchExpr {
            enum_name,
            scrutinee,
            arms,
            else_expr,
            ..
        } => json!({
            "kind":"EnumMatchExpr",
            "enum_name": enum_name,
            "scrutinee": ast_to_json(&scrutinee),
            "arms": arms.into_iter().map(|arm| json!({
                "variant_name": arm.variant_name,
                "binding_name": arm.binding_name,
                "body": ast_to_json(&arm.body)
            })).collect::<Vec<_>>(),
            "else": else_expr.as_ref().map(|expr| ast_to_json(expr)),
        }),
        // Phase 52: FieldAccess → Field ノード（JoinIR Frontend 互換）
        ASTNode::FieldAccess { object, field, .. } => json!({
            "kind": "FieldAccess",
            "type": "Field",  // JoinIR Frontend expects "type": "Field"
            "object": ast_to_json(&object),
            "field": field
        }),
        // Phase 52: Me → Var("me") ノード（JoinIR Frontend 互換）
        ASTNode::Me { .. } => json!({
            "kind": "Me",
            "type": "Var",  // JoinIR Frontend expects "type": "Var"
            "name": "me"
        }),
        // Phase 52: New → NewBox ノード（JoinIR Frontend 互換）
        ASTNode::New {
            class, arguments, ..
        } => json!({
            "kind": "New",
            "type": "NewBox",  // JoinIR Frontend expects "type": "NewBox"
            "box_name": class,
            "args": arguments.into_iter().map(|a| ast_to_json(&a)).collect::<Vec<_>>()
        }),
        other => json!({"kind":"Unsupported","debug": format!("{:?}", other)}),
    }
}

#[allow(dead_code)]
pub(crate) fn json_to_ast(v: &Value) -> Option<ASTNode> {
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
            let fields: Vec<String> = v
                .get("fields")?
                .as_array()?
                .iter()
                .filter_map(|s| s.as_str().map(|x| x.to_string()))
                .collect();
            let weak_fields: Vec<String> = v
                .get("weak_fields")
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(|x| x.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let field_decls = v
                .get("field_decls")
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            Some(FieldDecl {
                                name: item.get("name")?.as_str()?.to_string(),
                                declared_type_name: item
                                    .get("declared_type")
                                    .and_then(|s| s.as_str())
                                    .map(|s| s.to_string()),
                                is_weak: item
                                    .get("is_weak")
                                    .and_then(|b| b.as_bool())
                                    .unwrap_or(false),
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| {
                    fields
                        .iter()
                        .cloned()
                        .map(|name| FieldDecl {
                            is_weak: weak_fields.contains(&name),
                            name,
                            declared_type_name: None,
                        })
                        .collect()
                });

            ASTNode::BoxDeclaration {
                name: v.get("name")?.as_str()?.to_string(),
                fields,
                field_decls,
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
                weak_fields,
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
        "EnumDeclaration" => ASTNode::EnumDeclaration {
            name: v.get("name")?.as_str()?.to_string(),
            variants: v
                .get("variants")?
                .as_array()?
                .iter()
                .filter_map(|item| {
                    Some(EnumVariantDecl {
                        name: item.get("name")?.as_str()?.to_string(),
                        payload_type_name: item
                            .get("payload_type")
                            .and_then(|value| value.as_str())
                            .map(str::to_string),
                        record_field_decls: item
                            .get("record_fields")
                            .and_then(|value| value.as_array())
                            .map(|fields| {
                                fields
                                    .iter()
                                    .filter_map(|field| {
                                        Some(FieldDecl {
                                            name: field.get("name")?.as_str()?.to_string(),
                                            declared_type_name: field
                                                .get("declared_type")
                                                .and_then(|value| value.as_str())
                                                .map(str::to_string),
                                            is_weak: field
                                                .get("is_weak")
                                                .and_then(|value| value.as_bool())
                                                .unwrap_or(false),
                                        })
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect(),
            type_parameters: v
                .get("type_parameters")
                .and_then(|a| a.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(str::to_string))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
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
                json_to_lit(nested)?
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
            operator: str_to_bin(v.get("op")?.as_str()?)?,
            left: Box::new(json_to_ast(v.get("left")?)?),
            right: Box::new(json_to_ast(v.get("right")?)?),
            span: Span::unknown(),
        },
        "UnaryOp" => ASTNode::UnaryOp {
            operator: str_to_un(v.get("op")?.as_str()?)?,
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
                let lit = json_to_lit(lit_val)?;
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
        "EnumMatchExpr" => {
            let scr = json_to_ast(v.get("scrutinee")?)?;
            let arms_json = v.get("arms")?.as_array()?.iter();
            let mut arms = Vec::new();
            for arm_v in arms_json {
                arms.push(crate::ast::EnumMatchArm {
                    variant_name: arm_v.get("variant_name")?.as_str()?.to_string(),
                    binding_name: arm_v
                        .get("binding_name")
                        .and_then(|value| value.as_str())
                        .map(str::to_string),
                    body: json_to_ast(arm_v.get("body")?)?,
                });
            }
            ASTNode::EnumMatchExpr {
                enum_name: v.get("enum_name")?.as_str()?.to_string(),
                scrutinee: Box::new(scr),
                arms,
                else_expr: v.get("else").and_then(json_to_ast).map(Box::new),
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
