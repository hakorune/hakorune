use super::super::record_payload::enum_variant_payload_type_name;
use super::{expression_to_json_v0, ProgramJsonV0LocalTypes, ProgramJsonV0LoweringContext};
use crate::ast::{ASTNode, BinaryOperator, FieldDecl, LiteralValue, UnaryOperator};
use crate::semantics::option_contract::{nullish_payload_error, requires_non_nullish_payload};
use std::collections::BTreeSet;

pub(super) fn record_type_name_for_expr<'a>(
    expression: &'a ASTNode,
    local_types: &'a ProgramJsonV0LocalTypes,
) -> Option<&'a str> {
    match expression {
        ASTNode::RecordLiteral {
            record_type_name, ..
        } => Some(record_type_name.as_str()),
        ASTNode::Variable { name, .. } => local_types.record_locals.get(name).map(String::as_str),
        ASTNode::RecordUpdate { base, .. } => record_type_name_for_expr(base, local_types),
        ASTNode::BlockExpr { tail_expr, .. } => record_type_name_for_expr(tail_expr, local_types),
        _ => None,
    }
}

pub(super) fn validate_record_literal_fields(
    context: &ProgramJsonV0LoweringContext,
    record_type_name: &str,
    fields: &[(String, ASTNode)],
) -> Result<(), String> {
    let declared_fields = context.find_record(record_type_name).ok_or_else(|| {
        format!(
            "[record/literal-shape] unknown record `{}`",
            record_type_name
        )
    })?;
    let mut actual = BTreeSet::new();
    for (field_name, _) in fields {
        if !actual.insert(field_name.as_str()) {
            return Err(format!(
                "[record/literal-shape] {} duplicate field `{}`",
                record_type_name, field_name
            ));
        }
        if !declared_fields
            .iter()
            .any(|decl| decl.name.as_str() == field_name.as_str())
        {
            return Err(format!(
                "[record/literal-shape] {} extra field `{}`",
                record_type_name, field_name
            ));
        }
    }
    for declared_field in declared_fields {
        if !actual.contains(declared_field.name.as_str()) && declared_field.default_value.is_none()
        {
            return Err(format!(
                "[record/literal-shape] {} missing field `{}`",
                record_type_name, declared_field.name
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_record_update_fields(
    context: &ProgramJsonV0LoweringContext,
    record_type_name: &str,
    updates: &[(String, ASTNode)],
) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for (field_name, _) in updates {
        if !seen.insert(field_name.as_str()) {
            return Err(format!(
                "[record/update] {} duplicate field `{}`",
                record_type_name, field_name
            ));
        }
        record_field_decl(context, record_type_name, field_name)?;
    }
    Ok(())
}

pub(super) fn record_field_decl<'a>(
    context: &'a ProgramJsonV0LoweringContext,
    record_type_name: &str,
    field_name: &str,
) -> Result<(usize, &'a FieldDecl), String> {
    let declared_fields = context
        .find_record(record_type_name)
        .ok_or_else(|| format!("[record/field-read] unknown record `{}`", record_type_name))?;
    declared_fields
        .iter()
        .enumerate()
        .find(|(_, decl)| decl.name == field_name)
        .ok_or_else(|| {
            format!(
                "[record/field-read] {} has no field `{}`",
                record_type_name, field_name
            )
        })
}

pub(super) fn brand_construct_to_json_v0(
    brand_name: &str,
    underlying_type: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    if arguments.len() != 1 {
        return Err(format!(
            "[brand/constructor-arity] {} expects 1 arg, got {}",
            brand_name,
            arguments.len()
        ));
    }
    Ok(serde_json::json!({
        "type": "BrandConstruct",
        "brand": brand_name,
        "underlying_type": underlying_type,
        "value": expression_to_json_v0(&arguments[0], context, local_types)?,
    }))
}

pub(super) fn brand_static_method_to_json_v0(
    brand_name: &str,
    underlying_type: &str,
    method: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    if method != "unwrap" {
        return Err(format!(
            "[brand/unsupported-static-method] {}.{}",
            brand_name, method
        ));
    }
    if arguments.len() != 1 {
        return Err(format!(
            "[brand/unwrap-arity] {}.unwrap expects 1 arg, got {}",
            brand_name,
            arguments.len()
        ));
    }
    Ok(serde_json::json!({
        "type": "BrandUnwrap",
        "brand": brand_name,
        "underlying_type": underlying_type,
        "value": expression_to_json_v0(&arguments[0], context, local_types)?,
    }))
}

pub(super) fn enum_ctor_to_json_v0(
    enum_name: &str,
    variant_name: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let variant = context
        .find_enum_variant(enum_name, variant_name)
        .ok_or_else(|| {
            format!(
                "unsupported qualified call in Main.main/0: {}::{}",
                enum_name, variant_name
            )
        })?;
    let expected_arity = variant.payload_arity();
    if arguments.len() != expected_arity {
        if context.is_prelude_result_option_enum(enum_name) {
            return Err(format!(
                "[enum/payload][prelude] {}::{} expects {} payload arg(s), got {}",
                enum_name,
                variant_name,
                expected_arity,
                arguments.len()
            ));
        }
        return Err(format!(
            "enum constructor arity mismatch in Main.main/0: {}::{} expects {} arg(s), got {}",
            enum_name,
            variant_name,
            expected_arity,
            arguments.len()
        ));
    }
    if requires_non_nullish_payload(enum_name, variant_name)
        && arguments.iter().any(ast_expr_is_statically_nullish)
    {
        return Err(nullish_payload_error("stage1/program_json_v0"));
    }
    let payload_type = enum_variant_payload_type_name(enum_name, variant);
    let lowered_args = if variant.requires_compat_payload_box() {
        let payload_box = payload_type.clone().ok_or_else(|| {
            format!(
                "compat enum payload box missing for {}::{}",
                enum_name, variant_name
            )
        })?;
        vec![serde_json::json!({
            "type": "New",
            "class": payload_box,
            "args": expressions_to_json_v0(arguments, context, local_types)?,
        })]
    } else {
        expressions_to_json_v0(arguments, context, local_types)?
    };

    Ok(serde_json::json!({
        "type": "EnumCtor",
        "enum": enum_name,
        "variant": variant_name,
        "payload_type": payload_type,
        "args": lowered_args,
    }))
}

pub(super) fn ast_expr_is_statically_nullish(node: &ASTNode) -> bool {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Null | LiteralValue::Void,
            ..
        } => true,
        ASTNode::BlockExpr { tail_expr, .. } => ast_expr_is_statically_nullish(tail_expr),
        _ => false,
    }
}

pub(super) fn enum_match_expr_to_json_v0(
    enum_name: &str,
    scrutinee: &ASTNode,
    arms: &[crate::ast::EnumMatchArm],
    else_expr: Option<&ASTNode>,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
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
            "expr": expression_to_json_v0(&arm.body, context, local_types)?,
        }));
    }
    Ok(serde_json::json!({
        "type": "EnumMatch",
        "enum": enum_name,
        "scrutinee": expression_to_json_v0(scrutinee, context, local_types)?,
        "arms": arm_values,
        "else": else_expr
            .map(|expr| expression_to_json_v0(expr, context, local_types))
            .transpose()?,
    }))
}

pub(super) fn unary_expr_to_json_v0(
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

pub(super) fn expressions_to_json_v0(
    expressions: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(expressions.len());
    for expression in expressions {
        out.push(expression_to_json_v0(expression, context, local_types)?);
    }
    Ok(out)
}

pub(super) fn literal_to_json_v0(literal: &LiteralValue) -> Result<serde_json::Value, String> {
    match literal {
        LiteralValue::Integer(integer_value) => Ok(serde_json::json!({
            "type": "Int",
            "value": integer_value,
        })),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => Ok(serde_json::json!({
            "type": "Int",
            "value": value,
            "declared_type": declared_type_name,
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

pub(super) fn binary_expr_to_json_v0(
    operator: &BinaryOperator,
    left: &ASTNode,
    right: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let lhs = expression_to_json_v0(left, context, local_types)?;
    let rhs = expression_to_json_v0(right, context, local_types)?;
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

pub(super) fn binary_operator_symbol(operator: &BinaryOperator) -> &'static str {
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

pub(super) fn static_path_from_expr(expression: &ASTNode) -> Option<String> {
    match expression {
        ASTNode::Variable { name, .. } if looks_like_static_symbol(name) => Some(name.clone()),
        ASTNode::FieldAccess { object, field, .. } => {
            let base = static_path_from_expr(object)?;
            Some(format!("{}.{}", base, field))
        }
        _ => None,
    }
}

pub(super) fn looks_like_static_symbol(name: &str) -> bool {
    name.chars()
        .next()
        .map(|ch| ch.is_ascii_uppercase())
        .unwrap_or(false)
}

pub(super) fn match_label_from_literal(literal: &LiteralValue) -> String {
    match literal {
        LiteralValue::String(value) => value.clone(),
        LiteralValue::Integer(value) => value.to_string(),
        LiteralValue::TypedInteger {
            value,
            declared_type_name,
        } => format!("{}{}", value, declared_type_name),
        LiteralValue::Float(value) => value.to_string(),
        LiteralValue::Bool(value) => value.to_string(),
        LiteralValue::Null => "null".to_string(),
        LiteralValue::Void => "void".to_string(),
    }
}
