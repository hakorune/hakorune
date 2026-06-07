use super::expr_support::{ast_expr_is_statically_nullish, record_type_name_for_expr};
use super::typed_array::{
    array_literal_to_json_v0, array_type_element_type, validate_array_element_type_supported,
};
use super::{expression_to_json_v0, ProgramJsonV0LocalTypes, ProgramJsonV0LoweringContext};
use crate::ast::{ASTNode, CatchClause};
use crate::semantics::option_contract::requires_non_nullish_payload;

pub(super) fn statements_to_json_v0(
    statements: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::new();
    for statement in statements {
        out.extend(statement_to_json_v0_many(statement, context, local_types)?);
    }
    Ok(out)
}

fn statement_to_json_v0_many(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    match statement {
        ASTNode::Program { statements, .. } => {
            let mut scoped_types = local_types.clone();
            statements_to_json_v0(statements, context, &mut scoped_types)
        }
        ASTNode::ScopeBox { body, .. } => {
            let mut scoped_types = local_types.clone();
            statements_to_json_v0(body, context, &mut scoped_types)
        }
        ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        } => local_statement_to_json_v0_many(
            variables,
            initial_values,
            declared_type_names,
            context,
            local_types,
        ),
        _ => Ok(vec![statement_to_json_v0(statement, context, local_types)?]),
    }
}

fn local_statement_to_json_v0_many(
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    declared_type_names: &[Option<String>],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::new();
    for (index, name) in variables.iter().enumerate() {
        let declared_type_name = declared_type_names
            .get(index)
            .and_then(|value| value.as_deref());
        let initializer_node = initial_values.get(index).and_then(|value| value.as_deref());
        validate_prelude_enum_expected_type_context(
            name,
            declared_type_name,
            initializer_node,
            context,
        )?;
        let record_type = initializer_node
            .and_then(|value| record_type_name_for_expr(value, local_types))
            .or_else(|| {
                declared_type_name.filter(|type_name| context.find_record(type_name).is_some())
            })
            .map(str::to_string);
        let array_element_type = declared_type_name
            .and_then(array_type_element_type)
            .map(str::to_string);
        let initializer = match initializer_node {
            Some(ASTNode::ArrayLiteral { elements, .. }) => {
                let declared_type_name = declared_type_name.ok_or_else(|| {
                    "[array/literal-context] array literal requires local typed context".to_string()
                })?;
                array_literal_to_json_v0(declared_type_name, elements, context, local_types)?
            }
            Some(value) => expression_to_json_v0(value, context, local_types)?,
            None => serde_json::json!({ "type": "Null" }),
        };
        if let Some(record_type) = record_type {
            local_types
                .record_locals
                .insert(name.clone(), record_type.to_string());
        } else {
            local_types.record_locals.remove(name);
        }
        if let Some(array_element_type) = array_element_type {
            let declared_type_name = declared_type_name.expect("array type has declaration");
            validate_array_element_type_supported(&array_element_type, declared_type_name)?;
            local_types
                .array_locals
                .insert(name.clone(), array_element_type);
        } else if declared_type_name.is_some() {
            local_types.array_locals.remove(name);
        }
        out.push(serde_json::json!({
            "type": "Local",
            "name": name,
            "declared_type": declared_type_name,
            "expr": initializer,
        }));
    }
    Ok(out)
}

fn validate_prelude_enum_expected_type_context(
    local_name: &str,
    declared_type_name: Option<&str>,
    initializer_node: Option<&ASTNode>,
    context: &ProgramJsonV0LoweringContext,
) -> Result<(), String> {
    if declared_type_name.is_some() {
        return Ok(());
    }
    let Some(ASTNode::FromCall {
        parent,
        method,
        arguments,
        ..
    }) = initializer_node
    else {
        return Ok(());
    };
    if !context.is_prelude_result_option_enum(parent) {
        return Ok(());
    }
    let Some(variant) = context.find_enum_variant(parent, method) else {
        return Ok(());
    };
    if arguments.len() != variant.payload_arity() {
        return Ok(());
    }
    if requires_non_nullish_payload(parent, method)
        && arguments.iter().any(ast_expr_is_statically_nullish)
    {
        return Ok(());
    }
    let type_hint = match parent.as_str() {
        "Option" => "Option<T>",
        "Result" => "Result<T,E>",
        _ => unreachable!("prelude enum gate only accepts Option/Result"),
    };
    let ctor_hint = if variant.payload_arity() == 0 {
        format!("{}::{}", parent, method)
    } else {
        format!("{}::{}(...)", parent, method)
    };
    Err(format!(
        "[enum/expected-type][prelude] {}::{} for local `{}` requires explicit expected type; add `local {}: {} = {}`",
        parent, method, local_name, local_name, type_hint, ctor_hint
    ))
}

fn statement_to_json_v0(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    match statement {
        ASTNode::Assignment { target, value, .. } => {
            assignment_statement_to_json_v0(target, value, context, local_types)
        }
        ASTNode::Print { expression, .. } => {
            print_statement_to_json_v0(expression, context, local_types)
        }
        ASTNode::Return { value, .. } => {
            return_statement_to_json_v0(value.as_deref(), context, local_types)
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => if_statement_to_json_v0(
            condition,
            then_body,
            else_body.as_deref(),
            context,
            local_types,
        ),
        ASTNode::Loop {
            condition, body, ..
        } => loop_statement_to_json_v0(condition, body, context, local_types),
        ASTNode::LoopRange {
            var_name,
            start,
            end,
            body,
            ..
        } => loop_range_statement_to_json_v0(var_name, start, end, body, context, local_types),
        ASTNode::TaskScope {
            body,
            source_keyword,
            ..
        } => task_scope_statement_to_json_v0(body, source_keyword, context, local_types),
        ASTNode::FastMemRegion { contract, body, .. } => {
            fastmem_region_statement_to_json_v0(contract, body, context, local_types)
        }
        ASTNode::ContextScope {
            source_keyword,
            name,
            ..
        } => Err(format!(
            "[freeze:contract][program_json_v0/context_scope_not_supported] spelling={} name={} context propagation is owned by CONC-CONTEXT-002",
            source_keyword, name
        )),
        ASTNode::Break { .. } => Ok(serde_json::json!({ "type": "Break" })),
        ASTNode::Continue { .. } => Ok(serde_json::json!({ "type": "Continue" })),
        ASTNode::Throw { expression, .. } => {
            throw_statement_to_json_v0(expression, context, local_types)
        }
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => try_catch_statement_to_json_v0(
            try_body,
            catch_clauses,
            finally_body.as_deref(),
            context,
            local_types,
        ),
        _ => expression_statement_to_json_v0(statement, context, local_types),
    }
}

fn fastmem_region_statement_to_json_v0(
    contract: &str,
    body: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "type": "FastMemRegion",
        "contract": contract,
        "body": statements_to_json_v0(body, context, local_types)?,
    }))
}

fn assignment_statement_to_json_v0(
    target: &ASTNode,
    value: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let ASTNode::Variable { name, .. } = target else {
        return Err("unsupported assignment target".into());
    };
    let record_type = record_type_name_for_expr(value, local_types).map(str::to_string);
    let lowered_value = expression_to_json_v0(value, context, local_types)?;
    if let Some(record_type) = record_type {
        local_types.record_locals.insert(name.clone(), record_type);
    } else {
        local_types.record_locals.remove(name);
    }
    Ok(serde_json::json!({
        "type": "Local",
        "name": name,
        "expr": lowered_value,
    }))
}

fn print_statement_to_json_v0(
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "type": "Expr",
        "expr": {
            "type": "Call",
            "name": "env.console.log",
            "args": [expression_to_json_v0(expression, context, local_types)?],
        },
    }))
}

fn return_statement_to_json_v0(
    value: Option<&ASTNode>,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let return_value = value
        .map(|value| expression_to_json_v0(value, context, local_types))
        .transpose()?
        .unwrap_or_else(|| serde_json::json!({ "type": "Int", "value": 0 }));
    Ok(serde_json::json!({
        "type": "Return",
        "expr": return_value,
    }))
}

fn if_statement_to_json_v0(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&[ASTNode]>,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let cond = expression_to_json_v0(condition, context, local_types)?;
    let mut then_types = local_types.clone();
    let then_json = statements_to_json_v0(then_body, context, &mut then_types)?;
    let else_json = else_body
        .map(|body| {
            let mut else_types = local_types.clone();
            statements_to_json_v0(body, context, &mut else_types)
        })
        .transpose()?;
    Ok(serde_json::json!({
        "type": "If",
        "cond": cond,
        "then": then_json,
        "else": else_json,
    }))
}

fn loop_statement_to_json_v0(
    condition: &ASTNode,
    body: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let cond = expression_to_json_v0(condition, context, local_types)?;
    let mut body_types = local_types.clone();
    let body_json = statements_to_json_v0(body, context, &mut body_types)?;
    Ok(serde_json::json!({
        "type": "Loop",
        "cond": cond,
        "body": body_json,
    }))
}

fn loop_range_statement_to_json_v0(
    var_name: &str,
    start: &ASTNode,
    end: &ASTNode,
    body: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let start_json = expression_to_json_v0(start, context, local_types)?;
    let end_json = expression_to_json_v0(end, context, local_types)?;
    let mut body_types = local_types.clone();
    let body_json = statements_to_json_v0(body, context, &mut body_types)?;
    Ok(serde_json::json!({
        "type": "LoopRange",
        "var_name": var_name,
        "start": start_json,
        "end": end_json,
        "body": body_json,
    }))
}

fn task_scope_statement_to_json_v0(
    body: &[ASTNode],
    source_keyword: &str,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let mut body_types = local_types.clone();
    let body_json = statements_to_json_v0(body, context, &mut body_types)?;
    Ok(serde_json::json!({
        "type": "TaskScope",
        "spelling": source_keyword,
        "body": body_json,
    }))
}

fn throw_statement_to_json_v0(
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "type": "Throw",
        "expr": expression_to_json_v0(expression, context, local_types)?,
    }))
}

fn try_catch_statement_to_json_v0(
    try_body: &[ASTNode],
    catch_clauses: &[CatchClause],
    finally_body: Option<&[ASTNode]>,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let mut try_types = local_types.clone();
    let try_json = statements_to_json_v0(try_body, context, &mut try_types)?;
    let catches_json = catches_to_json_v0(catch_clauses, context, local_types)?;
    let finally_json = finally_body
        .map(|body| {
            let mut finally_types = local_types.clone();
            statements_to_json_v0(body, context, &mut finally_types)
        })
        .transpose()?
        .unwrap_or_default();
    Ok(serde_json::json!({
        "type": "Try",
        "try": try_json,
        "catches": catches_json,
        "finally": finally_json,
    }))
}

fn expression_statement_to_json_v0(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "type": "Expr",
        "expr": expression_to_json_v0(statement, context, local_types)?,
    }))
}

fn catches_to_json_v0(
    catches: &[CatchClause],
    context: &ProgramJsonV0LoweringContext,
    local_types: &ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(catches.len());
    for catch_clause in catches {
        let mut catch_types = local_types.clone();
        out.push(serde_json::json!({
            "param": catch_clause.variable_name,
            "typeHint": catch_clause.exception_type,
            "body": statements_to_json_v0(&catch_clause.body, context, &mut catch_types)?,
        }));
    }
    Ok(out)
}
