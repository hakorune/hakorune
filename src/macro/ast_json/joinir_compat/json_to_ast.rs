use nyash_rust::ast::{ASTNode, LiteralValue, Span};
use serde_json::Value;

use super::constructors;
use super::helpers::{json_to_lit, str_to_bin, str_to_un};

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
        "BoxDeclaration" => constructors::box_declaration_from_json(v, json_to_ast)?,
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
        "LoopRange" | "ForRange" => ASTNode::LoopRange {
            var_name: v.get("var_name")?.as_str()?.to_string(),
            start: Box::new(json_to_ast(v.get("start")?)?),
            end: Box::new(json_to_ast(v.get("end")?)?),
            body: v
                .get("body")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            span: Span::unknown(),
        },
        "TaskScope" => ASTNode::TaskScope {
            source_keyword: v
                .get("spelling")
                .and_then(|value| value.as_str())
                .unwrap_or("co")
                .to_string(),
            body: v
                .get("body")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            span: Span::unknown(),
        },
        "ContextScope" => ASTNode::ContextScope {
            source_keyword: v
                .get("spelling")
                .and_then(|value| value.as_str())
                .unwrap_or("context")
                .to_string(),
            name: v.get("name")?.as_str()?.to_string(),
            declared_type_name: v
                .get("declared_type")
                .and_then(|value| value.as_str())
                .map(str::to_string),
            value: Box::new(json_to_ast(v.get("value")?)?),
            body: v
                .get("body")?
                .as_array()?
                .iter()
                .filter_map(json_to_ast)
                .collect::<Vec<_>>(),
            span: Span::unknown(),
        },
        "FastMemRegion" => ASTNode::FastMemRegion {
            contract: v.get("contract")?.as_str()?.to_string(),
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
            let vars: Vec<String> = v
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
            let declared_type_names =
                constructors::json_to_local_declared_type_names(v, vars.len());
            ASTNode::Local {
                variables: vars,
                initial_values: inits,
                declared_type_names,
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
        "FunctionDeclaration" => constructors::function_declaration_from_json(v, json_to_ast)?,
        "EnumDeclaration" => constructors::enum_declaration_from_json(v, json_to_ast)?,
        "BrandDeclaration" => ASTNode::BrandDeclaration {
            name: v.get("name")?.as_str()?.to_string(),
            underlying_type_name: v.get("underlying_type")?.as_str()?.to_string(),
            span: Span::unknown(),
        },
        "TypeAliasDeclaration" => ASTNode::TypeAliasDeclaration {
            name: v.get("name")?.as_str()?.to_string(),
            target_type_name: v.get("target_type")?.as_str()?.to_string(),
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
                    "Int" => {
                        let value = v.get("value")?.as_i64()?;
                        if let Some(declared_type_name) =
                            v.get("declared_type").and_then(|value| value.as_str())
                        {
                            LiteralValue::TypedInteger {
                                value,
                                declared_type_name: declared_type_name.to_string(),
                            }
                        } else {
                            LiteralValue::Integer(value)
                        }
                    }
                    "TypedInt" => LiteralValue::TypedInteger {
                        value: v.get("value")?.as_i64()?,
                        declared_type_name: v.get("declared_type")?.as_str()?.to_string(),
                    },
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
        "RecordLiteral" => ASTNode::RecordLiteral {
            record_type_name: v.get("record_type")?.as_str()?.to_string(),
            fields: v
                .get("fields")?
                .as_array()?
                .iter()
                .filter_map(|field| {
                    Some((
                        field.get("name")?.as_str()?.to_string(),
                        json_to_ast(field.get("value")?)?,
                    ))
                })
                .collect(),
            span: Span::unknown(),
        },
        "RecordUpdate" => ASTNode::RecordUpdate {
            base: Box::new(json_to_ast(v.get("base")?)?),
            updates: v
                .get("updates")?
                .as_array()?
                .iter()
                .filter_map(|field| {
                    Some((
                        field.get("name")?.as_str()?.to_string(),
                        json_to_ast(field.get("value")?)?,
                    ))
                })
                .collect(),
            span: Span::unknown(),
        },
        "MatchExpr" => constructors::match_expr_from_json(v, json_to_ast)?,
        "EnumMatchExpr" => constructors::enum_match_expr_from_json(v, json_to_ast)?,
        "TryCatch" => constructors::try_catch_from_json(v, json_to_ast)?,
        _ => return None,
    })
}
