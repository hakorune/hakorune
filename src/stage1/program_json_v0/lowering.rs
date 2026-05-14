use super::extract::HelperMethod;
use super::record_payload::enum_variant_payload_type_name;
use crate::ast::{
    ASTNode, BinaryOperator, CatchClause, ContractClause, ContractKind, EnumVariantDecl, FieldDecl,
    LiteralValue, ParamDecl, UnaryOperator,
};
use crate::semantics::option_contract::{nullish_payload_error, requires_non_nullish_payload};
use std::collections::{BTreeMap, BTreeSet};

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
        } => {
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
                        declared_type_name
                            .filter(|type_name| context.find_record(type_name).is_some())
                    })
                    .map(str::to_string);
                let array_element_type = declared_type_name
                    .and_then(array_type_element_type)
                    .map(str::to_string);
                let initializer = match initializer_node {
                    Some(ASTNode::ArrayLiteral { elements, .. }) => {
                        let declared_type_name = declared_type_name.ok_or_else(|| {
                            "[array/literal-context] array literal requires local typed context"
                                .to_string()
                        })?;
                        array_literal_to_json_v0(
                            declared_type_name,
                            elements,
                            context,
                            local_types,
                        )?
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
        _ => Ok(vec![statement_to_json_v0(statement, context, local_types)?]),
    }
}

fn statement_to_json_v0(
    statement: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<serde_json::Value, String> {
    match statement {
        ASTNode::Assignment { target, value, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
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
        ASTNode::Print { expression, .. } => Ok(serde_json::json!({
            "type": "Expr",
            "expr": {
                "type": "Call",
                "name": "env.console.log",
                "args": [expression_to_json_v0(expression, context, local_types)?],
            },
        })),
        ASTNode::Return { value, .. } => {
            let return_value = value
                .as_deref()
                .map(|value| expression_to_json_v0(value, context, local_types))
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
        } => {
            let cond = expression_to_json_v0(condition, context, local_types)?;
            let mut then_types = local_types.clone();
            let then_json = statements_to_json_v0(then_body, context, &mut then_types)?;
            let else_json = else_body
                .as_ref()
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
        ASTNode::Loop {
            condition, body, ..
        } => {
            let cond = expression_to_json_v0(condition, context, local_types)?;
            let mut body_types = local_types.clone();
            let body_json = statements_to_json_v0(body, context, &mut body_types)?;
            Ok(serde_json::json!({
                "type": "Loop",
                "cond": cond,
                "body": body_json,
            }))
        }
        ASTNode::ForRange {
            var_name,
            start,
            end,
            body,
            ..
        } => {
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
        ASTNode::Break { .. } => Ok(serde_json::json!({ "type": "Break" })),
        ASTNode::Continue { .. } => Ok(serde_json::json!({ "type": "Continue" })),
        ASTNode::Throw { expression, .. } => Ok(serde_json::json!({
            "type": "Throw",
            "expr": expression_to_json_v0(expression, context, local_types)?,
        })),
        ASTNode::TryCatch {
            try_body,
            catch_clauses,
            finally_body,
            ..
        } => {
            let mut try_types = local_types.clone();
            let try_json = statements_to_json_v0(try_body, context, &mut try_types)?;
            let catches_json = catches_to_json_v0(catch_clauses, context, local_types)?;
            let finally_json = finally_body
                .as_ref()
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
        _ => Ok(serde_json::json!({
            "type": "Expr",
            "expr": expression_to_json_v0(statement, context, local_types)?,
        })),
    }
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
        } => {
            if let Some(underlying_type) = context.brand_underlying_type(name) {
                return brand_construct_to_json_v0(
                    name,
                    underlying_type,
                    arguments,
                    context,
                    local_types,
                );
            }
            Ok(serde_json::json!({
                "type": "Call",
                "name": name,
                "args": expressions_to_json_v0(arguments, context, local_types)?,
            }))
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            let call_name = static_path_from_expr(callee)
                .ok_or_else(|| "unsupported dynamic call callee in Main.main/0".to_string())?;
            Ok(serde_json::json!({
                "type": "Call",
                "name": call_name,
                "args": expressions_to_json_v0(arguments, context, local_types)?,
            }))
        }
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
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
            if let ASTNode::Variable { name, .. } = object.as_ref() {
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
        ASTNode::FromCall {
            parent,
            method,
            arguments,
            ..
        } => enum_ctor_to_json_v0(parent, method, arguments, context, local_types),
        ASTNode::FieldAccess { object, field, .. } => {
            if let Some(static_receiver) = static_path_from_expr(object) {
                if context
                    .find_enum_variant(&static_receiver, field)
                    .is_some()
                {
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
                let (field_index, field_decl) =
                    record_field_decl(context, record_type_name, field)?;
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
        ASTNode::New {
            class, arguments, ..
        } => Ok(serde_json::json!({
            "type": "New",
            "class": class,
            "args": expressions_to_json_v0(arguments, context, local_types)?,
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
        } => {
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
        ASTNode::RecordLiteral {
            record_type_name,
            fields,
            ..
        } => {
            validate_record_literal_fields(context, record_type_name, fields)?;
            let mut lowered_fields = Vec::with_capacity(fields.len());
            for (name, value) in fields {
                let (field_index, field_decl) = record_field_decl(context, record_type_name, name)?;
                lowered_fields.push(serde_json::json!({
                    "name": name,
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
        ASTNode::RecordUpdate { base, updates, .. } => {
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

fn array_literal_to_json_v0(
    declared_type_name: &str,
    elements: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &mut ProgramJsonV0LocalTypes,
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
        "elements": expressions_to_json_v0(elements, context, local_types)?,
    }))
}

fn array_literal_element_type_for_context(declared_type_name: &str) -> Result<&str, String> {
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

fn array_type_element_type(type_name: &str) -> Option<&str> {
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

fn validate_array_element_type_supported(
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

fn array_element_type_has_unresolved_generic(type_name: &str) -> bool {
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

fn validate_typed_array_method_contract(
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

fn validate_typed_array_method_value(
    element_type: &str,
    method: &str,
    arguments: &[ASTNode],
    context: &ProgramJsonV0LoweringContext,
    local_types: &ProgramJsonV0LocalTypes,
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

fn validate_array_element_expr(
    element_type: &str,
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &ProgramJsonV0LocalTypes,
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

fn array_element_direct_type_name(
    expression: &ASTNode,
    context: &ProgramJsonV0LoweringContext,
    local_types: &ProgramJsonV0LocalTypes,
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
            record_type_name_for_expr(expression, local_types).map(str::to_string)
        }
        _ => None,
    }
}

fn literal_direct_type_name(value: &LiteralValue) -> Option<String> {
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

fn array_element_type_accepts(expected: &str, actual: &str) -> bool {
    if expected == actual {
        return true;
    }
    is_builtin_integer_type(expected) && actual == "i64"
}

fn array_element_type_is_enforced(
    expected: &str,
    context: &ProgramJsonV0LoweringContext,
) -> bool {
    is_builtin_scalar_type(expected)
        || context.brand_underlying_type(expected).is_some()
        || context.find_record(expected).is_some()
        || context.known_enums.contains_key(expected)
}

fn is_builtin_scalar_type(type_name: &str) -> bool {
    is_builtin_integer_type(type_name)
        || matches!(type_name, "String" | "str" | "bool" | "f32" | "f64")
}

fn is_builtin_integer_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize"
    )
}

fn record_type_name_for_expr<'a>(
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

fn validate_record_literal_fields(
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
        if !actual.contains(declared_field.name.as_str()) {
            return Err(format!(
                "[record/literal-shape] {} missing field `{}`",
                record_type_name, declared_field.name
            ));
        }
    }
    Ok(())
}

fn validate_record_update_fields(
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

fn record_field_decl<'a>(
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

fn brand_construct_to_json_v0(
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

fn brand_static_method_to_json_v0(
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

fn enum_ctor_to_json_v0(
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

fn ast_expr_is_statically_nullish(node: &ASTNode) -> bool {
    match node {
        ASTNode::Literal {
            value: LiteralValue::Null | LiteralValue::Void,
            ..
        } => true,
        ASTNode::BlockExpr { tail_expr, .. } => ast_expr_is_statically_nullish(tail_expr),
        _ => false,
    }
}

fn enum_match_expr_to_json_v0(
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
    local_types: &mut ProgramJsonV0LocalTypes,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::with_capacity(expressions.len());
    for expression in expressions {
        out.push(expression_to_json_v0(expression, context, local_types)?);
    }
    Ok(out)
}

fn literal_to_json_v0(literal: &LiteralValue) -> Result<serde_json::Value, String> {
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

fn binary_expr_to_json_v0(
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
