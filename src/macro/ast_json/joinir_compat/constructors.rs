use nyash_rust::ast::{
    ASTNode, CatchClause, DelegateDecl, DelegateExposeDecl, EnumVariantDecl, FieldDecl, Span,
};
use serde_json::Value;
use std::collections::HashMap;

use super::helpers::{json_to_attrs, json_to_lit};
use super::shared;

pub(super) fn box_declaration_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
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
                        default_value: item
                            .get("default_value")
                            .and_then(json_to_ast)
                            .map(Box::new),
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
                    default_value: None,
                })
                .collect()
        });

    Some(ASTNode::BoxDeclaration {
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
        delegates: v
            .get("delegates")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(DelegateDecl {
                            field_name: item.get("field_name")?.as_str()?.to_string(),
                            exposes: item
                                .get("exposes")
                                .and_then(|value| value.as_array())
                                .map(|exposes| {
                                    exposes
                                        .iter()
                                        .filter_map(|expose| {
                                            Some(DelegateExposeDecl {
                                                source_name: expose
                                                    .get("source_name")?
                                                    .as_str()?
                                                    .to_string(),
                                                exposed_name: expose
                                                    .get("exposed_name")?
                                                    .as_str()?
                                                    .to_string(),
                                            })
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default(),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        invariants: v
            .get("invariants")
            .and_then(|a| a.as_array())
            .map(|arr| arr.iter().filter_map(json_to_ast).collect::<Vec<_>>())
            .unwrap_or_default(),
        transitions: shared::json_to_transition_decls(v.get("transitions")).unwrap_or_default(),
        is_interface: v
            .get("is_interface")
            .and_then(|b| b.as_bool())
            .unwrap_or(false),
        is_record: v
            .get("is_record")
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
        is_sync: v.get("is_sync").and_then(|b| b.as_bool()).unwrap_or(false),
        is_static: v
            .get("is_static")
            .and_then(|b| b.as_bool())
            .unwrap_or(false),
        static_init,
        attrs: json_to_attrs(v.get("attrs")),
        span: Span::unknown(),
    })
}

pub(super) fn enum_declaration_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
    Some(ASTNode::EnumDeclaration {
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
                    tuple_payload_type_names: item
                        .get("tuple_payload_types")
                        .and_then(|value| value.as_array())
                        .map(|types| {
                            types
                                .iter()
                                .filter_map(|value| value.as_str().map(str::to_string))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default(),
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
                                        default_value: field
                                            .get("default_value")
                                            .and_then(json_to_ast)
                                            .map(Box::new),
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
    })
}

pub(super) fn function_declaration_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
    let params = v
        .get("params")?
        .as_array()?
        .iter()
        .filter_map(|s| s.as_str().map(|x| x.to_string()))
        .collect::<Vec<_>>();
    let param_decls = shared::json_to_param_decls(v, &params)?;
    let return_type_name = v
        .get("return_type")
        .and_then(|value| value.as_str())
        .map(str::to_string);
    Some(ASTNode::FunctionDeclaration {
        name: v.get("name")?.as_str()?.to_string(),
        param_decls,
        params,
        return_type_name,
        body: v
            .get("body")?
            .as_array()?
            .iter()
            .filter_map(json_to_ast)
            .collect(),
        uses: shared::json_to_string_array(v.get("uses")).unwrap_or_default(),
        contracts: shared::json_to_contract_clauses_with(v.get("contracts"), json_to_ast)
            .unwrap_or_default(),
        is_static: v.get("static").and_then(|b| b.as_bool()).unwrap_or(false),
        is_override: v.get("override").and_then(|b| b.as_bool()).unwrap_or(false),
        attrs: json_to_attrs(v.get("attrs")),
        span: Span::unknown(),
    })
}

pub(super) fn match_expr_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
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
    Some(ASTNode::MatchExpr {
        scrutinee: Box::new(scr),
        arms,
        else_expr: Box::new(else_expr),
        span: Span::unknown(),
    })
}

pub(super) fn enum_match_expr_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
    let scr = json_to_ast(v.get("scrutinee")?)?;
    let arms_json = v.get("arms")?.as_array()?.iter();
    let mut arms = Vec::new();
    for arm_v in arms_json {
        arms.push(nyash_rust::ast::EnumMatchArm {
            variant_name: arm_v.get("variant_name")?.as_str()?.to_string(),
            binding_name: arm_v
                .get("binding_name")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            body: json_to_ast(arm_v.get("body")?)?,
        });
    }
    Some(ASTNode::EnumMatchExpr {
        enum_name: v.get("enum_name")?.as_str()?.to_string(),
        scrutinee: Box::new(scr),
        arms,
        else_expr: v.get("else").and_then(json_to_ast).map(Box::new),
        span: Span::unknown(),
    })
}

pub(super) fn try_catch_from_json(
    v: &Value,
    json_to_ast: fn(&Value) -> Option<ASTNode>,
) -> Option<ASTNode> {
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
            catches.push(CatchClause {
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
    Some(ASTNode::TryCatch {
        try_body: try_b,
        catch_clauses: catches,
        finally_body: cleanup,
        span: Span::unknown(),
    })
}

pub(super) fn json_to_local_declared_type_names(v: &Value, len: usize) -> Vec<Option<String>> {
    if let Some(values) = v.get("declared_type_names").and_then(Value::as_array) {
        return values
            .iter()
            .map(|value| {
                if value.is_null() {
                    None
                } else {
                    value.as_str().map(str::to_string)
                }
            })
            .collect();
    }
    if len == 1 {
        if let Some(value) = v.get("declared_type") {
            return vec![value.as_str().map(str::to_string)];
        }
    }
    vec![None; len]
}
