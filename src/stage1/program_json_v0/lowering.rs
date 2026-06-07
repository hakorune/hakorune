use super::extract::HelperMethod;
use crate::ast::{
    ASTNode, ContractClause, ContractKind, EnumVariantDecl, FieldDecl, LiteralValue, ParamDecl,
};
use std::collections::{BTreeMap, BTreeSet};

#[path = "lowering/expr_support.rs"]
mod expr_support;
#[path = "lowering/statements.rs"]
mod statements;
#[path = "lowering/typed_array.rs"]
mod typed_array;

use self::expr_support::{
    binary_expr_to_json_v0, brand_construct_to_json_v0, brand_static_method_to_json_v0,
    enum_ctor_to_json_v0, enum_match_expr_to_json_v0, expressions_to_json_v0, literal_to_json_v0,
    match_label_from_literal, record_field_decl, record_type_name_for_expr, static_path_from_expr,
    unary_expr_to_json_v0, validate_record_literal_fields, validate_record_update_fields,
};
use self::statements::statements_to_json_v0;
use self::typed_array::{validate_typed_array_method_contract, validate_typed_array_method_value};

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
#[path = "lowering/tests.rs"]
mod tests;
