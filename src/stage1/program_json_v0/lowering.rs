use super::extract::HelperMethod;
use crate::ast::{ASTNode, BinaryOperator, CatchClause, LiteralValue, UnaryOperator};

pub(super) fn program_json_v0_from_body(body: &[ASTNode]) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "version": 0,
        "kind": "Program",
        "body": statements_to_json_v0(body)?,
    }))
}

pub(super) fn defs_json_v0_from_methods(
    methods: &[HelperMethod<'_>],
) -> Result<Vec<serde_json::Value>, String> {
    let mut defs = Vec::with_capacity(methods.len());
    for method in methods {
        defs.push(function_def_json_v0(method.declaration, method.box_name)?);
    }
    Ok(defs)
}

fn function_def_json_v0(
    declaration: &ASTNode,
    box_name: &str,
) -> Result<serde_json::Value, String> {
    let ASTNode::FunctionDeclaration {
        name, params, body, ..
    } = declaration
    else {
        return Err("expected FunctionDeclaration in helper defs".to_string());
    };

    Ok(serde_json::json!({
        "name": name,
        "params": params,
        "body": program_json_v0_from_body(body)?,
        "box": box_name,
    }))
}

fn statements_to_json_v0(statements: &[ASTNode]) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::new();
    for statement in statements {
        out.extend(statement_to_json_v0_many(statement)?);
    }
    Ok(out)
}

fn statement_to_json_v0_many(statement: &ASTNode) -> Result<Vec<serde_json::Value>, String> {
    match statement {
        ASTNode::Program { statements, .. } => statements_to_json_v0(statements),
        ASTNode::ScopeBox { body, .. } => statements_to_json_v0(body),
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            let mut out = Vec::new();
            for (index, name) in variables.iter().enumerate() {
                let initializer = initial_values
                    .get(index)
                    .and_then(|value| value.as_deref())
                    .map(expression_to_json_v0)
                    .transpose()?
                    .unwrap_or_else(|| serde_json::json!({ "type": "Null" }));
                out.push(serde_json::json!({
                    "type": "Local",
                    "name": name,
                    "expr": initializer,
                }));
            }
            Ok(out)
        }
        _ => Ok(vec![statement_to_json_v0(statement)?]),
    }
}

fn statement_to_json_v0(statement: &ASTNode) -> Result<serde_json::Value, String> {
    match statement {
        ASTNode::Assignment { target, value, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return Err("unsupported assignment target".into());
            };
            Ok(serde_json::json!({
                "type": "Local",
                "name": name,
                "expr": expression_to_json_v0(value)?,
            }))
        }
        ASTNode::Print { expression, .. } => Ok(serde_json::json!({
            "type": "Expr",
            "expr": {
                "type": "Call",
                "name": "env.console.log",
                "args": [expression_to_json_v0(expression)?],
            },
        })),
        ASTNode::Return { value, .. } => {
            let return_value = value
                .as_deref()
                .map(expression_to_json_v0)
                .transpose()?
                .unwrap_or_else(|| serde_json::json!({ "type": "Int", "value": 0 }));
            Ok(serde_json::json!({
                "type": "Return",
                "expr": return_value,
            }))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => Ok(serde_json::json!({
            "type": "If",
            "cond": expression_to_json_v0(condition)?,
            "then": statements_to_json_v0(then_body)?,
            "else": else_body
                .as_ref()
                .map(|body| statements_to_json_v0(body))
                .transpose()?,
        })),
        ASTNode::Loop {
            condition, body, ..
        } => Ok(serde_json::json!({
            "type": "Loop",
            "cond": expression_to_json_v0(condition)?,
            "body": statements_to_json_v0(body)?,
        })),
        ASTNode::While {
            condition, body, ..
        } => Ok(serde_json::json!({
            "type": "Loop",
            "cond": expression_to_json_v0(condition)?,
            "body": statements_to_json_v0(body)?,
        })),
        ASTNode::Break { .. } => Ok(serde_json::json!({ "type": "Break" })),
        ASTNode::Continue { .. } => Ok(serde_json::json!({ "type": "Continue" })),
        ASTNode::Throw { expression, .. } => Ok(serde_json::json!({
            "type": "Throw",
            "expr": expression_to_json_v0(expression)?,
        })),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => Ok(serde_json::json!({
            "type": "Try",
            "try": statements_to_json_v0(try_body)?,
            "catches": catches_to_json_v0(catch_clauses)?,
            "finally": finally_body
                .as_ref()
                .map(|body| statements_to_json_v0(body))
                .transpose()?
                .unwrap_or_default(),
        })),
        _ => Ok(serde_json::json!({
            "type": "Expr",
            "expr": expression_to_json_v0(statement)?,
        })),
    }
}

fn catches_to_json_v0(catches: &[CatchClause]) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(catches.len());
    for catch_clause in catches {
        out.push(serde_json::json!({
            "param": catch_clause.variable_name,
            "typeHint": catch_clause.exception_type,
            "body": statements_to_json_v0(&catch_clause.body)?,
        }));
    }
    Ok(out)
}

fn expression_to_json_v0(expression: &ASTNode) -> Result<serde_json::Value, String> {
    match expression {
        ASTNode::Literal { value, .. } => literal_to_json_v0(value),
        ASTNode::Variable { name, .. } => Ok(serde_json::json!({
            "type": "Var",
            "name": name,
        })),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => binary_expr_to_json_v0(operator, left, right),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => unary_expr_to_json_v0(operator, operand),
        ASTNode::FunctionCall {
            name, arguments, ..
        } => Ok(serde_json::json!({
            "type": "Call",
            "name": name,
            "args": expressions_to_json_v0(arguments)?,
        })),
        ASTNode::Call {
            callee, arguments, ..
        } => {
            let call_name = static_path_from_expr(callee)
                .ok_or_else(|| "unsupported dynamic call callee in Main.main/0".to_string())?;
            Ok(serde_json::json!({
                "type": "Call",
                "name": call_name,
                "args": expressions_to_json_v0(arguments)?,
            }))
        }
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            if let Some(static_receiver) = static_path_from_expr(object) {
                return Ok(serde_json::json!({
                    "type": "Call",
                    "name": format!("{}.{}", static_receiver, method),
                    "args": expressions_to_json_v0(arguments)?,
                }));
            }
            Ok(serde_json::json!({
                "type": "Method",
                "recv": expression_to_json_v0(object)?,
                "method": method,
                "args": expressions_to_json_v0(arguments)?,
            }))
        }
        ASTNode::FieldAccess { .. } => {
            let path = static_path_from_expr(expression)
                .ok_or_else(|| "unsupported field access in Main.main/0".to_string())?;
            Ok(serde_json::json!({
                "type": "Var",
                "name": path,
            }))
        }
        ASTNode::New {
            class, arguments, ..
        } => Ok(serde_json::json!({
            "type": "New",
            "class": class,
            "args": expressions_to_json_v0(arguments)?,
        })),
        ASTNode::This { .. } => Ok(serde_json::json!({
            "type": "Var",
            "name": "this",
        })),
        ASTNode::Me { .. } => Ok(serde_json::json!({
            "type": "Var",
            "name": "me",
        })),
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => Ok(serde_json::json!({
            "type": "BlockExpr",
            "prelude": statements_to_json_v0(prelude_stmts)?,
            "tail": {
                "type": "Expr",
                "expr": expression_to_json_v0(tail_expr)?,
            },
        })),
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => {
            let mut arm_values = Vec::new();
            for (label, value) in arms {
                arm_values.push(serde_json::json!({
                    "label": match_label_from_literal(label),
                    "expr": expression_to_json_v0(value)?,
                }));
            }
            Ok(serde_json::json!({
                "type": "Match",
                "scrutinee": expression_to_json_v0(scrutinee)?,
                "arms": arm_values,
                "else": expression_to_json_v0(else_expr)?,
            }))
        }
        other => Err(format!(
            "unsupported expression in Main.main/0: {:?}",
            other.node_type()
        )),
    }
}

fn unary_expr_to_json_v0(
    operator: &UnaryOperator,
    operand: &ASTNode,
) -> Result<serde_json::Value, String> {
    match (operator, operand) {
        (
            UnaryOperator::Minus,
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            },
        ) => Ok(serde_json::json!({
            "type": "Int",
            "value": -value,
        })),
        (
            UnaryOperator::Minus,
            ASTNode::Literal {
                value: LiteralValue::Float(value),
                ..
            },
        ) => Ok(serde_json::json!({
            "type": "Float",
            "value": -value,
        })),
        _ => Err(format!(
            "unsupported expression in Main.main/0: {:?}",
            ASTNode::UnaryOp {
                operator: operator.clone(),
                operand: Box::new(operand.clone()),
                span: crate::ast::Span::unknown(),
            }
            .node_type()
        )),
    }
}

fn expressions_to_json_v0(expressions: &[ASTNode]) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(expressions.len());
    for expression in expressions {
        out.push(expression_to_json_v0(expression)?);
    }
    Ok(out)
}

fn literal_to_json_v0(literal: &LiteralValue) -> Result<serde_json::Value, String> {
    match literal {
        LiteralValue::Integer(integer_value) => Ok(serde_json::json!({
            "type": "Int",
            "value": integer_value,
        })),
        LiteralValue::String(string_value) => Ok(serde_json::json!({
            "type": "Str",
            "value": string_value,
        })),
        LiteralValue::Bool(bool_value) => Ok(serde_json::json!({
            "type": "Bool",
            "value": bool_value,
        })),
        LiteralValue::Null | LiteralValue::Void => Ok(serde_json::json!({
            "type": "Null",
        })),
        LiteralValue::Float(float_value) => Ok(serde_json::json!({
            "type": "Float",
            "value": float_value,
        })),
    }
}

fn binary_expr_to_json_v0(
    operator: &BinaryOperator,
    left: &ASTNode,
    right: &ASTNode,
) -> Result<serde_json::Value, String> {
    let lhs = expression_to_json_v0(left)?;
    let rhs = expression_to_json_v0(right)?;
    match operator {
        BinaryOperator::Add
        | BinaryOperator::Subtract
        | BinaryOperator::Multiply
        | BinaryOperator::Divide
        | BinaryOperator::Modulo
        | BinaryOperator::BitAnd
        | BinaryOperator::BitOr
        | BinaryOperator::BitXor
        | BinaryOperator::Shl
        | BinaryOperator::Shr => Ok(serde_json::json!({
            "type": "Binary",
            "op": binary_operator_symbol(operator),
            "lhs": lhs,
            "rhs": rhs,
        })),
        BinaryOperator::Equal
        | BinaryOperator::NotEqual
        | BinaryOperator::Less
        | BinaryOperator::Greater
        | BinaryOperator::LessEqual
        | BinaryOperator::GreaterEqual => Ok(serde_json::json!({
            "type": "Compare",
            "op": binary_operator_symbol(operator),
            "lhs": lhs,
            "rhs": rhs,
        })),
        BinaryOperator::And | BinaryOperator::Or => Ok(serde_json::json!({
            "type": "Logical",
            "op": binary_operator_symbol(operator),
            "lhs": lhs,
            "rhs": rhs,
        })),
    }
}

fn binary_operator_symbol(operator: &BinaryOperator) -> &'static str {
    match operator {
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

fn static_path_from_expr(expression: &ASTNode) -> Option<String> {
    match expression {
        ASTNode::Variable { name, .. } if looks_like_static_symbol(name) => Some(name.clone()),
        ASTNode::FieldAccess { object, field, .. } => {
            let base = static_path_from_expr(object)?;
            Some(format!("{}.{}", base, field))
        }
        _ => None,
    }
}

fn looks_like_static_symbol(name: &str) -> bool {
    name.chars()
        .next()
        .map(|ch| ch.is_ascii_uppercase())
        .unwrap_or(false)
}

fn match_label_from_literal(literal: &LiteralValue) -> String {
    match literal {
        LiteralValue::String(value) => value.clone(),
        LiteralValue::Integer(value) => value.to_string(),
        LiteralValue::Float(value) => value.to_string(),
        LiteralValue::Bool(value) => value.to_string(),
        LiteralValue::Null => "null".to_string(),
        LiteralValue::Void => "void".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::program_json_v0_from_body;
    use crate::ast::{ASTNode, LiteralValue, Span, UnaryOperator};
    use serde_json::json;

    fn float_lit(value: f64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Float(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn program_json_v0_from_body_preserves_float_return_literal() {
        let body = vec![ASTNode::Return {
            value: Some(Box::new(float_lit(2.5))),
            span: Span::unknown(),
        }];

        let program = program_json_v0_from_body(&body).expect("float return literal should lower");

        assert_eq!(
            program,
            json!({
                "version": 0,
                "kind": "Program",
                "body": [{
                    "type": "Return",
                    "expr": {
                        "type": "Float",
                        "value": 2.5
                    }
                }],
            })
        );
    }

    #[test]
    fn program_json_v0_from_body_preserves_negative_float_return_literal() {
        let body = vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(float_lit(1.25)),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }];

        let program =
            program_json_v0_from_body(&body).expect("negative float return literal should lower");

        assert_eq!(
            program,
            json!({
                "version": 0,
                "kind": "Program",
                "body": [{
                    "type": "Return",
                    "expr": {
                        "type": "Float",
                        "value": -1.25
                    }
                }],
            })
        );
    }
}
