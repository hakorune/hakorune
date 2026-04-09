use super::extract::HelperMethod;
use super::record_payload::enum_variant_payload_type_name;
use crate::ast::{
    ASTNode, BinaryOperator, CatchClause, EnumVariantDecl, LiteralValue, UnaryOperator,
};
use std::collections::BTreeMap;

pub(super) fn program_json_v0_from_body(body: &[ASTNode]) -> Result<serde_json::Value, String> {
    program_json_v0_from_body_with_context(body, &ProgramJsonV0LoweringContext::default())
}

#[derive(Debug, Default, Clone)]
pub(super) struct ProgramJsonV0LoweringContext {
    known_enums: BTreeMap<String, Vec<EnumVariantDecl>>,
}

impl ProgramJsonV0LoweringContext {
    pub(super) fn with_known_enums(known_enums: BTreeMap<String, Vec<EnumVariantDecl>>) -> Self {
        Self { known_enums }
    }

    fn find_enum_variant(&self, enum_name: &str, variant_name: &str) -> Option<&EnumVariantDecl> {
        self.known_enums
            .get(enum_name)
            .and_then(|variants| variants.iter().find(|variant| variant.name == variant_name))
    }
}

pub(super) fn program_json_v0_from_body_with_context(
    body: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "version": 0,
        "kind": "Program",
        "body": statements_to_json_v0(body, context)?,
    }))
}

pub(super) fn defs_json_v0_from_methods(
    methods: &[HelperMethod<'_>],
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    let mut defs = Vec::with_capacity(methods.len());
    for method in methods {
        defs.push(function_def_json_v0(
            method.declaration,
            method.box_name,
            context,
        )?);
    }
    Ok(defs)
}

fn function_def_json_v0(
    declaration: &ASTNode,
    box_name: &str,
    context: &ProgramJsonV0LoweringContext,
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
        "body": program_json_v0_from_body_with_context(body, context)?,
        "box": box_name,
    }))
}

fn statements_to_json_v0(
    statements: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::new();
    for statement in statements {
        out.extend(statement_to_json_v0_many(statement, context)?);
    }
    Ok(out)
}

fn statement_to_json_v0_many(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    match statement {
        ASTNode::Program { statements, .. } => statements_to_json_v0(statements, context),
        ASTNode::ScopeBox { body, .. } => statements_to_json_v0(body, context),
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
                    .map(|value| expression_to_json_v0(value, context))
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
        _ => Ok(vec![statement_to_json_v0(statement, context)?]),
    }
}

fn statement_to_json_v0(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    match statement {
        ASTNode::Assignment { target, value, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return Err("unsupported assignment target".into());
            };
            Ok(serde_json::json!({
                "type": "Local",
                "name": name,
                "expr": expression_to_json_v0(value, context)?,
            }))
        }
        ASTNode::Print { expression, .. } => Ok(serde_json::json!({
            "type": "Expr",
            "expr": {
                "type": "Call",
                "name": "env.console.log",
                "args": [expression_to_json_v0(expression, context)?],
            },
        })),
        ASTNode::Return { value, .. } => {
            let return_value = value
                .as_deref()
                .map(|value| expression_to_json_v0(value, context))
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
            "cond": expression_to_json_v0(condition, context)?,
            "then": statements_to_json_v0(then_body, context)?,
            "else": else_body
                .as_ref()
                .map(|body| statements_to_json_v0(body, context))
                .transpose()?,
        })),
        ASTNode::Loop {
            condition, body, ..
        } => Ok(serde_json::json!({
            "type": "Loop",
            "cond": expression_to_json_v0(condition, context)?,
            "body": statements_to_json_v0(body, context)?,
        })),
        ASTNode::While {
            condition, body, ..
        } => Ok(serde_json::json!({
            "type": "Loop",
            "cond": expression_to_json_v0(condition, context)?,
            "body": statements_to_json_v0(body, context)?,
        })),
        ASTNode::Break { .. } => Ok(serde_json::json!({ "type": "Break" })),
        ASTNode::Continue { .. } => Ok(serde_json::json!({ "type": "Continue" })),
        ASTNode::Throw { expression, .. } => Ok(serde_json::json!({
            "type": "Throw",
            "expr": expression_to_json_v0(expression, context)?,
        })),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => Ok(serde_json::json!({
            "type": "Try",
            "try": statements_to_json_v0(try_body, context)?,
            "catches": catches_to_json_v0(catch_clauses, context)?,
            "finally": finally_body
                .as_ref()
                .map(|body| statements_to_json_v0(body, context))
                .transpose()?
                .unwrap_or_default(),
        })),
        _ => Ok(serde_json::json!({
            "type": "Expr",
            "expr": expression_to_json_v0(statement, context)?,
        })),
    }
}

fn catches_to_json_v0(
    catches: &[CatchClause],
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(catches.len());
    for catch_clause in catches {
        out.push(serde_json::json!({
            "param": catch_clause.variable_name,
            "typeHint": catch_clause.exception_type,
            "body": statements_to_json_v0(&catch_clause.body, context)?,
        }));
    }
    Ok(out)
}

fn expression_to_json_v0(
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
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
        } => binary_expr_to_json_v0(operator, left, right, context),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => unary_expr_to_json_v0(operator, operand),
        ASTNode::FunctionCall {
            name, arguments, ..
        } => Ok(serde_json::json!({
            "type": "Call",
            "name": name,
            "args": expressions_to_json_v0(arguments, context)?,
        })),
        ASTNode::Call {
            callee, arguments, ..
        } => {
            let call_name = static_path_from_expr(callee)
                .ok_or_else(|| "unsupported dynamic call callee in Main.main/0".to_string())?;
            Ok(serde_json::json!({
                "type": "Call",
                "name": call_name,
                "args": expressions_to_json_v0(arguments, context)?,
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
                    "args": expressions_to_json_v0(arguments, context)?,
                }));
            }
            Ok(serde_json::json!({
                "type": "Method",
                "recv": expression_to_json_v0(object, context)?,
                "method": method,
                "args": expressions_to_json_v0(arguments, context)?,
            }))
        }
        ASTNode::FromCall {
            parent,
            method,
            arguments,
            ..
        } => enum_ctor_to_json_v0(parent, method, arguments, context),
        ASTNode::FieldAccess { object, field, .. } => {
            if let Some(path) = static_path_from_expr(expression) {
                return Ok(serde_json::json!({
                    "type": "Var",
                    "name": path,
                }));
            }
            Ok(serde_json::json!({
                "type": "Field",
                "recv": expression_to_json_v0(object, context)?,
                "field": field,
            }))
        }
        ASTNode::New {
            class, arguments, ..
        } => Ok(serde_json::json!({
            "type": "New",
            "class": class,
            "args": expressions_to_json_v0(arguments, context)?,
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
            "prelude": statements_to_json_v0(prelude_stmts, context)?,
            "tail": {
                "type": "Expr",
                "expr": expression_to_json_v0(tail_expr, context)?,
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
                    "expr": expression_to_json_v0(value, context)?,
                }));
            }
            Ok(serde_json::json!({
                "type": "Match",
                "scrutinee": expression_to_json_v0(scrutinee, context)?,
                "arms": arm_values,
                "else": expression_to_json_v0(else_expr, context)?,
            }))
        }
        ASTNode::EnumMatchExpr {
            enum_name,
            scrutinee,
            arms,
            else_expr,
            ..
        } => enum_match_expr_to_json_v0(enum_name, scrutinee, arms, else_expr.as_deref(), context),
        other => Err(format!(
            "unsupported expression in Main.main/0: {:?}",
            other.node_type()
        )),
    }
}

fn enum_ctor_to_json_v0(
    enum_name: &str,
    variant_name: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    let variant = context
        .find_enum_variant(enum_name, variant_name)
        .ok_or_else(|| {
            format!(
                "unsupported qualified call in Main.main/0: {}::{}",
                enum_name, variant_name
            )
        })?;
    let expected_arity = if variant.is_record_payload() {
        variant.record_field_decls.len()
    } else {
        usize::from(variant.has_payload())
    };
    if arguments.len() != expected_arity {
        return Err(format!(
            "enum constructor arity mismatch in Main.main/0: {}::{} expects {} arg(s), got {}",
            enum_name,
            variant_name,
            expected_arity,
            arguments.len()
        ));
    }
    let payload_type = enum_variant_payload_type_name(enum_name, variant);
    let lowered_args = if variant.is_record_payload() {
        let payload_box = payload_type.clone().ok_or_else(|| {
            format!(
                "record enum payload box missing for {}::{}",
                enum_name, variant_name
            )
        })?;
        vec![serde_json::json!({
            "type": "New",
            "class": payload_box,
            "args": expressions_to_json_v0(arguments, context)?,
        })]
    } else {
        expressions_to_json_v0(arguments, context)?
    };

    Ok(serde_json::json!({
        "type": "EnumCtor",
        "enum": enum_name,
        "variant": variant_name,
        "payload_type": payload_type,
        "args": lowered_args,
    }))
}

fn enum_match_expr_to_json_v0(
    enum_name: &str,
    scrutinee: &ASTNode,
    arms: &[crate::ast::EnumMatchArm],
    else_expr: Option<&ASTNode>,
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    let variant_index = context.known_enums.get(enum_name).ok_or_else(|| {
        format!(
            "unsupported enum shorthand match in Main.main/0: unknown enum `{}`",
            enum_name
        )
    })?;
    let mut arm_values = Vec::with_capacity(arms.len());
    for arm in arms {
        let payload_type = variant_index
            .iter()
            .find(|variant| variant.name == arm.variant_name)
            .and_then(|variant| enum_variant_payload_type_name(enum_name, variant));
        arm_values.push(serde_json::json!({
            "variant": arm.variant_name,
            "bind": arm.binding_name,
            "payload_type": payload_type,
            "expr": expression_to_json_v0(&arm.body, context)?,
        }));
    }
    Ok(serde_json::json!({
        "type": "EnumMatch",
        "enum": enum_name,
        "scrutinee": expression_to_json_v0(scrutinee, context)?,
        "arms": arm_values,
        "else": else_expr
            .map(|expr| expression_to_json_v0(expr, context))
            .transpose()?,
    }))
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

fn expressions_to_json_v0(
    expressions: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(expressions.len());
    for expression in expressions {
        out.push(expression_to_json_v0(expression, context)?);
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
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    let lhs = expression_to_json_v0(left, context)?;
    let rhs = expression_to_json_v0(right, context)?;
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
