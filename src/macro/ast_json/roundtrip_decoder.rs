use nyash_rust::ast::{ASTNode, LiteralValue, Span};
use serde_json::Value;

use super::shared::{self, json_to_attrs, json_to_local_declared_type_names};

mod declarations;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DecodeMode {
    Legacy,
    RoundtripV2,
}

pub(super) struct AstJsonDecoder {
    mode: DecodeMode,
    pub(super) nested_failure: std::cell::Cell<bool>,
}

impl AstJsonDecoder {
    pub(super) fn new(mode: DecodeMode) -> Self {
        Self {
            mode,
            nested_failure: std::cell::Cell::new(false),
        }
    }

    pub(super) fn decode(&self, v: &Value) -> Option<ASTNode> {
        // JSON null denotes an absent optional field; malformed objects are
        // recorded so collection helpers cannot silently drop nested errors.
        if v.is_null() {
            return None;
        }
        let result = self.decode_inner(v);
        if result.is_none() {
            self.nested_failure.set(true);
        }
        result
    }

    pub(super) fn decode_inner(&self, v: &Value) -> Option<ASTNode> {
        let k = v.get("kind")?.as_str()?;
        Some(match k {
            "Program" => {
                let stmts = v
                    .get("statements")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
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
                    .filter_map(|node| self.decode(node))
                    .collect::<Vec<_>>(),
                tail_expr: Box::new(self.decode(v.get("tail_expr")?)?),
                span: Span::unknown(),
            },
            "ScopeBox" => ASTNode::ScopeBox {
                body: v
                    .get("body")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect(),
                span: Span::unknown(),
            },
            "BoxDeclaration" | "EnumDeclaration" | "BrandDeclaration" | "TypeAliasDeclaration" => {
                declarations::decode(self, k, v)?
            }
            "Loop" => ASTNode::Loop {
                condition: Box::new(self.decode(v.get("condition")?)?),
                body: v
                    .get("body")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect::<Vec<_>>(),
                span: Span::unknown(),
            },
            "LoopRange" | "ForRange" => ASTNode::LoopRange {
                var_name: v.get("var_name")?.as_str()?.to_string(),
                start: Box::new(self.decode(v.get("start")?)?),
                end: Box::new(self.decode(v.get("end")?)?),
                body: v
                    .get("body")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
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
                    .filter_map(|node| self.decode(node))
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
                value: Box::new(self.decode(v.get("value")?)?),
                body: v
                    .get("body")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect::<Vec<_>>(),
                span: Span::unknown(),
            },
            "FastMemRegion" => ASTNode::FastMemRegion {
                contract: v.get("contract")?.as_str()?.to_string(),
                body: v
                    .get("body")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect::<Vec<_>>(),
                span: Span::unknown(),
            },
            "Print" => ASTNode::Print {
                expression: Box::new(self.decode(v.get("expression")?)?),
                span: Span::unknown(),
            },
            "Return" => ASTNode::Return {
                value: v
                    .get("value")
                    .and_then(|node| self.decode(node))
                    .map(Box::new),
                span: Span::unknown(),
            },
            "Break" => ASTNode::Break {
                span: Span::unknown(),
            },
            "Continue" => ASTNode::Continue {
                span: Span::unknown(),
            },
            "Release" => ASTNode::Release {
                root: v.get("root")?.as_str()?.to_string(),
                span: Span::unknown(),
            },
            "Assignment" => ASTNode::Assignment {
                target: Box::new(
                    if let Some(lhs) = v.get("lhs").and_then(|node| self.decode(node)) {
                        lhs
                    } else if let Some(name) = v.get("target").and_then(|t| t.as_str()) {
                        ASTNode::Variable {
                            name: name.to_string(),
                            span: Span::unknown(),
                        }
                    } else {
                        self.decode(v.get("target")?)?
                    },
                ),
                value: Box::new(self.decode(v.get("value")?)?),
                span: Span::unknown(),
            },
            "CompoundAssignment" => ASTNode::CompoundAssignment {
                target: Box::new(self.decode(v.get("target")?)?),
                operator: shared::str_to_bin(v.get("op")?.as_str()?)?,
                value: Box::new(self.decode(v.get("value")?)?),
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
                            self.decode(initv).map(Box::new)
                        }
                    })
                    .collect();
                let declared_type_names = json_to_local_declared_type_names(v, vars.len());
                ASTNode::Local {
                    variables: vars,
                    initial_values: inits,
                    declared_type_names,
                    span: Span::unknown(),
                }
            }
            "If" => ASTNode::If {
                condition: Box::new(self.decode(v.get("condition")?)?),
                then_body: v
                    .get("then")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect::<Vec<_>>(),
                else_body: v.get("else").and_then(|a| {
                    a.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|node| self.decode(node))
                            .collect::<Vec<_>>()
                    })
                }),
                span: Span::unknown(),
            },
            "FunctionDeclaration" => {
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
                ASTNode::FunctionDeclaration {
                    name: v.get("name")?.as_str()?.to_string(),
                    param_decls,
                    params,
                    return_type_name,
                    body: v
                        .get("body")?
                        .as_array()?
                        .iter()
                        .filter_map(|node| self.decode(node))
                        .collect(),
                    uses: shared::json_to_string_array(v.get("uses")).unwrap_or_default(),
                    contracts: shared::json_to_contract_clauses_with(v.get("contracts"), |node| {
                        self.decode(node)
                    })
                    .unwrap_or_default(),
                    is_static: v.get("static").and_then(|b| b.as_bool()).unwrap_or(false),
                    is_override: v.get("override").and_then(|b| b.as_bool()).unwrap_or(false),
                    attrs: json_to_attrs(v.get("attrs")),
                    span: Span::unknown(),
                }
            }
            "Variable" => ASTNode::Variable {
                name: v.get("name")?.as_str()?.to_string(),
                span: Span::unknown(),
            },
            "Me" => ASTNode::Me {
                span: Span::unknown(),
            },
            "Literal" => {
                let value = if let Some(nested) = v.get("value").filter(|vv| vv.is_object()) {
                    // Older/lit_to_json format: { kind:"Literal", value:{type:"int",...} }
                    shared::json_to_lit(nested)?
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
                operator: shared::str_to_bin(v.get("op")?.as_str()?)?,
                left: Box::new(self.decode(v.get("left")?)?),
                right: Box::new(self.decode(v.get("right")?)?),
                span: Span::unknown(),
            },
            "UnaryOp" => ASTNode::UnaryOp {
                operator: shared::str_to_un(v.get("op")?.as_str()?)?,
                operand: Box::new(self.decode(v.get("operand")?)?),
                span: Span::unknown(),
            },
            "MethodCall" => ASTNode::MethodCall {
                object: Box::new(self.decode(v.get("object").or_else(|| v.get("receiver"))?)?),
                method: v.get("method")?.as_str()?.to_string(),
                arguments: v
                    .get("arguments")
                    .or_else(|| v.get("args"))?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect(),
                span: Span::unknown(),
            },
            "FieldAccess" => ASTNode::FieldAccess {
                object: Box::new(self.decode(v.get("object")?)?),
                field: v.get("field")?.as_str()?.to_string(),
                span: Span::unknown(),
            },
            "Index" => ASTNode::Index {
                target: Box::new(self.decode(v.get("target")?)?),
                index: Box::new(self.decode(v.get("index")?)?),
                span: Span::unknown(),
            },
            "FunctionCall" => ASTNode::FunctionCall {
                name: v.get("name")?.as_str()?.to_string(),
                arguments: v
                    .get("arguments")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect(),
                span: Span::unknown(),
            },
            "ExplicitExternCall" => ASTNode::ExplicitExternCall {
                target: v.get("target")?.as_str()?.to_string(),
                arguments: v
                    .get("arguments")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect(),
                span: Span::unknown(),
            },
            "Array" => ASTNode::ArrayLiteral {
                elements: v
                    .get("elements")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
                    .collect(),
                span: Span::unknown(),
            },
            "Map" => ASTNode::MapLiteral {
                entries: v
                    .get("entries")?
                    .as_array()?
                    .iter()
                    .filter_map(|e| {
                        Some((e.get("k")?.as_str()?.to_string(), self.decode(e.get("v")?)?))
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
                            self.decode(field.get("value")?)?,
                        ))
                    })
                    .collect(),
                span: Span::unknown(),
            },
            "RecordUpdate" => ASTNode::RecordUpdate {
                base: Box::new(self.decode(v.get("base")?)?),
                updates: v
                    .get("updates")?
                    .as_array()?
                    .iter()
                    .filter_map(|field| {
                        Some((
                            field.get("name")?.as_str()?.to_string(),
                            self.decode(field.get("value")?)?,
                        ))
                    })
                    .collect(),
                span: Span::unknown(),
            },
            "MatchExpr" => {
                let scr = self.decode(v.get("scrutinee")?)?;
                let arms_json = v.get("arms")?.as_array()?.iter();
                let mut arms = Vec::new();
                for arm_v in arms_json {
                    let lit_val = arm_v.get("literal")?.get("value")?;
                    let lit = shared::json_to_lit(lit_val)?;
                    let body = self.decode(arm_v.get("body")?)?;
                    arms.push((lit, body));
                }
                let else_expr = self.decode(v.get("else")?)?;
                ASTNode::MatchExpr {
                    scrutinee: Box::new(scr),
                    arms,
                    else_expr: Box::new(else_expr),
                    span: Span::unknown(),
                }
            }
            "EnumMatchExpr" => {
                let scr = self.decode(v.get("scrutinee")?)?;
                let arms_json = v.get("arms")?.as_array()?.iter();
                let mut arms = Vec::new();
                for arm_v in arms_json {
                    arms.push(crate::ast::EnumMatchArm {
                        variant_name: arm_v.get("variant_name")?.as_str()?.to_string(),
                        binding_name: arm_v
                            .get("binding_name")
                            .and_then(|value| value.as_str())
                            .map(str::to_string),
                        body: self.decode(arm_v.get("body")?)?,
                    });
                }
                ASTNode::EnumMatchExpr {
                    enum_name: v.get("enum_name")?.as_str()?.to_string(),
                    scrutinee: Box::new(scr),
                    arms,
                    else_expr: v
                        .get("else")
                        .and_then(|node| self.decode(node))
                        .map(Box::new),
                    span: Span::unknown(),
                }
            }
            "TryCatch" => {
                let try_b = v
                    .get("try")?
                    .as_array()?
                    .iter()
                    .filter_map(|node| self.decode(node))
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
                            .filter_map(|node| self.decode(node))
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
                    cl.as_array().map(|arr| {
                        arr.iter()
                            .filter_map(|node| self.decode(node))
                            .collect::<Vec<_>>()
                    })
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
}
