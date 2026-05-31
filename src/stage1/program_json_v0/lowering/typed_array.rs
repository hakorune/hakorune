use crate::ast::{ASTNode, LiteralValue};

pub(super) fn array_literal_to_json_v0(
    declared_type_name: &str,
    elements: &[ASTNode],
    context: &super::ProgramJsonV0LoweringContext,
    local_types: &mut super::ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let element_type = array_literal_element_type_for_context(declared_type_name)?;
    validate_array_element_type_supported(element_type, declared_type_name)?;
    for element in elements {
        validate_array_element_expr(
            element_type,
            element,
            context,
            local_types,
            "array literal element",
        )?;
    }
    Ok(serde_json::json!({
        "type": "ArrayLiteral",
        "declared_type": declared_type_name,
        "element_type": element_type,
        "elements": super::expressions_to_json_v0(elements, context, local_types)?,
    }))
}

pub(super) fn array_literal_element_type_for_context(
    declared_type_name: &str,
) -> Result<&str, String> {
    let type_name = declared_type_name.trim();
    if let Some(inner) = array_type_element_type(type_name) {
        return Ok(inner);
    }
    if type_name.starts_with("Array<") {
        return Err(format!(
            "[array/literal-context] invalid Array<T> context `{}`",
            declared_type_name
        ));
    }
    if type_name.starts_with("PackedArray<") {
        return Err(
            "[array/literal-context] PackedArray literal lowering is deferred; no Array<T> fallback"
                .to_string(),
        );
    }
    Err(format!(
        "[array/literal-context] array literal requires Array<T> typed context, got `{}`",
        declared_type_name
    ))
}

pub(super) fn array_type_element_type(type_name: &str) -> Option<&str> {
    let inner = type_name
        .trim()
        .strip_prefix("Array<")
        .and_then(|rest| rest.strip_suffix('>'))?
        .trim();
    if inner.is_empty() {
        return None;
    }
    Some(inner)
}

pub(super) fn validate_array_element_type_supported(
    element_type: &str,
    declared_type_name: &str,
) -> Result<(), String> {
    if array_element_type_has_unresolved_generic(element_type) {
        return Err(format!(
            "[array/inference] `{}` uses unresolved Array element type `{}`; use a concrete `Array<T>` element type",
            declared_type_name, element_type
        ));
    }
    Ok(())
}

pub(super) fn array_element_type_has_unresolved_generic(type_name: &str) -> bool {
    type_name
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .any(|ident| {
            ident.len() == 1
                && ident
                    .chars()
                    .next()
                    .map(|ch| ch.is_ascii_uppercase())
                    .unwrap_or(false)
        })
}

pub(super) fn validate_typed_array_method_contract(
    receiver_name: &str,
    method: &str,
    arg_count: usize,
) -> Result<(), String> {
    let expected = match method {
        "push" => 1,
        "get" => 1,
        "set" => 2,
        "length" => 0,
        _ => {
            return Err(format!(
                "[array/method-contract] Array<T> local `{}` supports push/get/set/length; got `{}`",
                receiver_name, method
            ));
        }
    };
    if arg_count != expected {
        return Err(format!(
            "[array/method-contract] Array<T>.{} on local `{}` expects {} arg(s), got {}",
            method, receiver_name, expected, arg_count
        ));
    }
    Ok(())
}

pub(super) fn validate_typed_array_method_value(
    element_type: &str,
    method: &str,
    arguments: &[ASTNode],
    context: &super::ProgramJsonV0LoweringContext,
    local_types: &super::ProgramJsonV0LocalTypes,
) -> Result<(), String> {
    match method {
        "push" => validate_array_element_expr(
            element_type,
            &arguments[0],
            context,
            local_types,
            "push value",
        ),
        "set" => validate_array_element_expr(
            element_type,
            &arguments[1],
            context,
            local_types,
            "set value",
        ),
        _ => Ok(()),
    }
}

pub(super) fn validate_array_element_expr(
    element_type: &str,
    expression: &ASTNode,
    context: &super::ProgramJsonV0LoweringContext,
    local_types: &super::ProgramJsonV0LocalTypes,
    position: &str,
) -> Result<(), String> {
    let expected = element_type.trim();
    let Some(actual) = array_element_direct_type_name(expression, context, local_types) else {
        return Ok(());
    };
    if array_element_type_accepts(expected, &actual) {
        return Ok(());
    }
    if !array_element_type_is_enforced(expected, context) {
        return Ok(());
    }
    Err(format!(
        "[array/element-type] {} expects `{}`, got `{}`",
        position, expected, actual
    ))
}

pub(super) fn array_element_direct_type_name(
    expression: &ASTNode,
    context: &super::ProgramJsonV0LoweringContext,
    local_types: &super::ProgramJsonV0LocalTypes,
) -> Option<String> {
    match expression {
        ASTNode::Literal { value, .. } => literal_direct_type_name(value),
        ASTNode::FunctionCall { name, .. } if context.brand_underlying_type(name).is_some() => {
            Some(name.clone())
        }
        ASTNode::FromCall { parent, .. } if context.known_enums.contains_key(parent) => {
            Some(parent.clone())
        }
        ASTNode::RecordLiteral {
            record_type_name, ..
        } => Some(record_type_name.clone()),
        ASTNode::Variable { .. } | ASTNode::RecordUpdate { .. } | ASTNode::BlockExpr { .. } => {
            super::expr_support::record_type_name_for_expr(expression, local_types)
                .map(str::to_string)
        }
        _ => None,
    }
}

pub(super) fn literal_direct_type_name(value: &LiteralValue) -> Option<String> {
    match value {
        LiteralValue::String(_) => Some("String".to_string()),
        LiteralValue::Integer(_) => Some("i64".to_string()),
        LiteralValue::TypedInteger {
            declared_type_name, ..
        } => Some(declared_type_name.clone()),
        LiteralValue::Float(_) => Some("f64".to_string()),
        LiteralValue::Bool(_) => Some("bool".to_string()),
        LiteralValue::Null => Some("null".to_string()),
        LiteralValue::Void => Some("void".to_string()),
    }
}

pub(super) fn array_element_type_accepts(expected: &str, actual: &str) -> bool {
    if expected == actual {
        return true;
    }
    is_builtin_integer_type(expected) && actual == "i64"
}

pub(super) fn array_element_type_is_enforced(
    expected: &str,
    context: &super::ProgramJsonV0LoweringContext,
) -> bool {
    is_builtin_scalar_type(expected)
        || context.brand_underlying_type(expected).is_some()
        || context.find_record(expected).is_some()
        || context.known_enums.contains_key(expected)
}

pub(super) fn is_builtin_scalar_type(type_name: &str) -> bool {
    is_builtin_integer_type(type_name)
        || matches!(type_name, "String" | "str" | "bool" | "f32" | "f64")
}

pub(super) fn is_builtin_integer_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize"
    )
}
