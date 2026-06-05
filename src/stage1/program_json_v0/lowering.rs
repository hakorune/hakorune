use super::extract::HelperMethod;
use crate::ast::{
    ASTNode, CatchClause, ContractClause, ContractKind, EnumVariantDecl, FieldDecl, LiteralValue,
    ParamDecl,
};
use crate::semantics::option_contract::requires_non_nullish_payload;
use std::collections::{BTreeMap, BTreeSet};

#[path = "lowering/expr_support.rs"]
mod expr_support;
#[path = "lowering/typed_array.rs"]
mod typed_array;

use self::expr_support::{
    ast_expr_is_statically_nullish, binary_expr_to_json_v0, brand_construct_to_json_v0,
    brand_static_method_to_json_v0, enum_ctor_to_json_v0, enum_match_expr_to_json_v0,
    expressions_to_json_v0, literal_to_json_v0, match_label_from_literal, record_field_decl,
    record_type_name_for_expr, static_path_from_expr, unary_expr_to_json_v0,
    validate_record_literal_fields, validate_record_update_fields,
};
use self::typed_array::{
    array_literal_to_json_v0, array_type_element_type, validate_array_element_type_supported,
    validate_typed_array_method_contract, validate_typed_array_method_value,
};

#[cfg(test)]
pub(super) fn program_json_v0_from_body(body: &[ASTNode]) -> Result<serde_json::Value, String> {
    program_json_v0_from_body_with_context(body, &ProgramJsonV0LoweringContext::default())
}

#[derive(Debug, Default, Clone)]
pub(super) struct ProgramJsonV0LoweringContext {
    known_enums: BTreeMap<String, Vec<EnumVariantDecl>>,
    known_brands: BTreeMap<String, String>,
    known_records: BTreeMap<String, Vec<FieldDecl>>,
    source_enum_names: BTreeSet<String>,
}

impl ProgramJsonV0LoweringContext {
    pub(super) fn with_known_enums_brands_and_records(
        known_enums: BTreeMap<String, Vec<EnumVariantDecl>>,
        known_brands: BTreeMap<String, String>,
        known_records: BTreeMap<String, Vec<FieldDecl>>,
        source_enum_names: BTreeSet<String>,
    ) -> Self {
        Self {
            known_enums,
            known_brands,
            known_records,
            source_enum_names,
        }
    }

    fn find_enum_variant(&self, enum_name: &str, variant_name: &str) -> Option<&EnumVariantDecl> {
        self.known_enums
            .get(enum_name)
            .and_then(|variants| variants.iter().find(|variant| variant.name == variant_name))
    }

    fn brand_underlying_type(&self, brand_name: &str) -> Option<&str> {
        self.known_brands.get(brand_name).map(String::as_str)
    }

    fn find_record(&self, record_name: &str) -> Option<&[FieldDecl]> {
        self.known_records.get(record_name).map(Vec::as_slice)
    }

    fn is_prelude_result_option_enum(&self, enum_name: &str) -> bool {
        matches!(enum_name, "Option" | "Result") && !self.source_enum_names.contains(enum_name)
    }
}

#[derive(Debug, Default, Clone)]
struct ProgramJsonV0LocalTypes {
    record_locals: BTreeMap<String, String>,
    array_locals: BTreeMap<String, String>,
}

pub(super) fn program_json_v0_from_body_with_context(
    body: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
) -> Result<serde_json::Value, String> {
    let mut local_types = ProgramJsonV0LocalTypes::default();
    Ok(serde_json::json!({
        "version": 0,
        "kind": "Program",
        "body": statements_to_json_v0(body, context, &mut local_types)?,
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
        name,
        params,
        param_decls,
        return_type_name,
        body,
        uses,
        contracts,
        ..
    } = declaration
    else {
        return Err("expected FunctionDeclaration in helper defs".to_string());
    };

    Ok(serde_json::json!({
        "name": name,
        "params": params,
        "param_decls": param_decls_json_v0(params, param_decls),
        "return_type": return_type_name,
        "uses": uses,
        "contracts": contract_clauses_json_v0(contracts, context)?,
        "body": program_json_v0_from_body_with_context(body, context)?,
        "box": box_name,
    }))
}

fn contract_clauses_json_v0(
    contracts: &[ContractClause],
    context: &ProgramJsonV0LoweringContext,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(contracts.len());
    let mut local_types = ProgramJsonV0LocalTypes::default();
    for clause in contracts {
        let kind = match clause.kind {
            ContractKind::Requires => "requires",
            ContractKind::Ensures => "ensures",
        };
        out.push(serde_json::json!({
            "kind": kind,
            "condition": expression_to_json_v0(&clause.condition, context, &mut local_types)?,
        }));
    }
    Ok(out)
}

fn param_decls_json_v0(params: &[String], param_decls: &[ParamDecl]) -> Vec<serde_json::Value> {
    ParamDecl::with_name_fallback(param_decls, params)
        .iter()
        .map(|decl| {
            serde_json::json!({
                "name": decl.name,
                "declared_type": decl.declared_type_name,
            })
        })
        .collect()
}

fn statements_to_json_v0(
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

fn expression_to_json_v0(
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
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
        } => binary_expr_to_json_v0(operator, left, right, context, local_types),
        ASTNode::UnaryOp {
            operator, operand, ..
        } => unary_expr_to_json_v0(operator, operand),
        ASTNode::FunctionCall {
            name, arguments, ..
        } => function_call_expr_to_json_v0(name, arguments, context, local_types),
        ASTNode::Call {
            callee, arguments, ..
        } => call_expr_to_json_v0(callee, arguments, context, local_types),
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => method_call_expr_to_json_v0(object, method, arguments, context, local_types),
        ASTNode::FromCall {
            parent,
            method,
            arguments,
            ..
        } => enum_ctor_to_json_v0(parent, method, arguments, context, local_types),
        ASTNode::FieldAccess { object, field, .. } => {
            field_access_expr_to_json_v0(expression, object, field, context, local_types)
        }
        ASTNode::New {
            class,
            arguments,
            field_initializers,
            ..
        } => Ok(serde_json::json!({
            "type": "New",
            "class": class,
            "args": expressions_to_json_v0(arguments, context, local_types)?,
            "field_initializers": field_initializers
                .iter()
                .map(|(name, expr)| {
                    Ok(serde_json::json!({
                        "field": name,
                        "value": expression_to_json_v0(expr, context, local_types)?,
                    }))
                })
                .collect::<Result<Vec<_>, String>>()?,
        })),
        ASTNode::ArrayLiteral { .. } => {
            Err("[array/literal-context] array literal requires local typed context".to_string())
        }
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
        } => block_expr_to_json_v0(prelude_stmts, tail_expr, context, local_types),
        ASTNode::RecordLiteral {
            record_type_name,
            fields,
            ..
        } => record_literal_to_json_v0(record_type_name, fields, context, local_types),
        ASTNode::RecordUpdate { base, updates, .. } => {
            record_update_to_json_v0(base, updates, context, local_types)
        }
        ASTNode::MatchExpr {
            scrutinee,
            arms,
            else_expr,
            ..
        } => match_expr_to_json_v0(scrutinee, arms, else_expr, context, local_types),
        ASTNode::EnumMatchExpr {
            enum_name,
            scrutinee,
            arms,
            else_expr,
            ..
        } => enum_match_expr_to_json_v0(
            enum_name,
            scrutinee,
            arms,
            else_expr.as_deref(),
            context,
            local_types,
        ),
        other => Err(format!(
            "unsupported expression in Main.main/0: {:?}",
            other.node_type()
        )),
    }
}

fn function_call_expr_to_json_v0(
    name: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    if let Some(underlying_type) = context.brand_underlying_type(name) {
        return brand_construct_to_json_v0(name, underlying_type, arguments, context, local_types);
    }
    Ok(serde_json::json!({
        "type": "Call",
        "name": name,
        "args": expressions_to_json_v0(arguments, context, local_types)?,
    }))
}

fn call_expr_to_json_v0(
    callee: &ASTNode,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let call_name = static_path_from_expr(callee)
        .ok_or_else(|| "unsupported dynamic call callee in Main.main/0".to_string())?;
    Ok(serde_json::json!({
        "type": "Call",
        "name": call_name,
        "args": expressions_to_json_v0(arguments, context, local_types)?,
    }))
}

fn method_call_expr_to_json_v0(
    object: &ASTNode,
    method: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    if let Some(static_receiver) = static_path_from_expr(object) {
        if context
            .find_enum_variant(&static_receiver, method)
            .is_some()
        {
            return Err(format!(
                "[enum/variant-surface] use `{}::{}` for enum variants; `{}.{}` is object/member syntax",
                static_receiver, method, static_receiver, method
            ));
        }
        if let Some(underlying_type) = context.brand_underlying_type(&static_receiver) {
            return brand_static_method_to_json_v0(
                &static_receiver,
                underlying_type,
                method,
                arguments,
                context,
                local_types,
            );
        }
        return Ok(serde_json::json!({
            "type": "Call",
            "name": format!("{}.{}", static_receiver, method),
            "args": expressions_to_json_v0(arguments, context, local_types)?,
        }));
    }
    if let ASTNode::Variable { name, .. } = object {
        if let Some(element_type) = local_types.array_locals.get(name).cloned() {
            validate_typed_array_method_contract(name, method, arguments.len())?;
            validate_typed_array_method_value(
                &element_type,
                method,
                arguments,
                context,
                local_types,
            )?;
        }
    }
    Ok(serde_json::json!({
        "type": "Method",
        "recv": expression_to_json_v0(object, context, local_types)?,
        "method": method,
        "args": expressions_to_json_v0(arguments, context, local_types)?,
    }))
}

fn field_access_expr_to_json_v0(
    expression: &ASTNode,
    object: &ASTNode,
    field: &str,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    if let Some(static_receiver) = static_path_from_expr(object) {
        if context.find_enum_variant(&static_receiver, field).is_some() {
            return Err(format!(
                "[enum/variant-surface] use `{}::{}` for enum variants; `{}.{}` is object/member syntax",
                static_receiver, field, static_receiver, field
            ));
        }
    }
    if let Some(path) = static_path_from_expr(expression) {
        return Ok(serde_json::json!({
            "type": "Var",
            "name": path,
        }));
    }
    if let Some(record_type_name) = record_type_name_for_expr(object, local_types) {
        let (field_index, field_decl) = record_field_decl(context, record_type_name, field)?;
        return Ok(serde_json::json!({
            "type": "RecordField",
            "record": record_type_name,
            "recv": expression_to_json_v0(object, context, local_types)?,
            "field": field,
            "field_index": field_index,
            "declared_type": field_decl.declared_type_name.clone(),
        }));
    }
    Ok(serde_json::json!({
        "type": "Field",
        "recv": expression_to_json_v0(object, context, local_types)?,
        "field": field,
    }))
}

fn block_expr_to_json_v0(
    prelude_stmts: &[ASTNode],
    tail_expr: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let mut block_types = local_types.clone();
    Ok(serde_json::json!({
        "type": "BlockExpr",
        "prelude": statements_to_json_v0(prelude_stmts, context, &mut block_types)?,
        "tail": {
            "type": "Expr",
            "expr": expression_to_json_v0(tail_expr, context, &mut block_types)?,
        },
    }))
}

fn record_literal_to_json_v0(
    record_type_name: &str,
    fields: &[(String, ASTNode)],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    validate_record_literal_fields(context, record_type_name, fields)?;
    let declared_fields = context.find_record(record_type_name).ok_or_else(|| {
        format!(
            "[record/literal-shape] unknown record `{}`",
            record_type_name
        )
    })?;
    let mut provided_fields = BTreeMap::new();
    for (name, value) in fields {
        provided_fields.insert(name.as_str(), value);
    }
    let mut lowered_fields = Vec::with_capacity(declared_fields.len());
    for (field_index, field_decl) in declared_fields.iter().enumerate() {
        let value = match provided_fields.get(field_decl.name.as_str()) {
            Some(value) => *value,
            None => field_decl.default_value.as_deref().ok_or_else(|| {
                format!(
                    "[record/literal-shape] {} missing field `{}`",
                    record_type_name, field_decl.name
                )
            })?,
        };
        lowered_fields.push(serde_json::json!({
            "name": field_decl.name,
            "field_index": field_index,
            "declared_type": field_decl.declared_type_name.clone(),
            "value": expression_to_json_v0(value, context, local_types)?,
        }));
    }
    Ok(serde_json::json!({
        "type": "RecordLiteral",
        "record": record_type_name,
        "fields": lowered_fields,
    }))
}

fn record_update_to_json_v0(
    base: &ASTNode,
    updates: &[(String, ASTNode)],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let record_type_name = record_type_name_for_expr(base, local_types)
        .ok_or_else(|| "[record/update] base expression is not a tracked record".to_string())?
        .to_string();
    validate_record_update_fields(context, &record_type_name, updates)?;
    let mut lowered_updates = Vec::with_capacity(updates.len());
    for (name, value) in updates {
        let (field_index, field_decl) = record_field_decl(context, &record_type_name, name)?;
        lowered_updates.push(serde_json::json!({
            "name": name,
            "field_index": field_index,
            "declared_type": field_decl.declared_type_name.clone(),
            "value": expression_to_json_v0(value, context, local_types)?,
        }));
    }
    Ok(serde_json::json!({
        "type": "RecordUpdate",
        "record": record_type_name,
        "base": expression_to_json_v0(base, context, local_types)?,
        "updates": lowered_updates,
    }))
}

fn match_expr_to_json_v0(
    scrutinee: &ASTNode,
    arms: &[(LiteralValue, ASTNode)],
    else_expr: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    let mut arm_values = Vec::new();
    for (label, value) in arms {
        arm_values.push(serde_json::json!({
            "label": match_label_from_literal(label),
            "expr": expression_to_json_v0(value, context, local_types)?,
        }));
    }
    Ok(serde_json::json!({
        "type": "Match",
        "scrutinee": expression_to_json_v0(scrutinee, context, local_types)?,
        "arms": arm_values,
        "else": expression_to_json_v0(else_expr, context, local_types)?,
    }))
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
