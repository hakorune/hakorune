use super::type_ref::{parse_type_ref_text, TypeRef};
use crate::ast::{ASTNode, FieldDecl, ParamDecl};
use std::collections::{BTreeMap, BTreeSet};

const PACKED_ARRAY_TYPE: &str = "PackedArray";

#[derive(Debug, Clone)]
struct RecordInfo {
    type_parameters: Vec<String>,
    fields: Vec<FieldDecl>,
}

#[derive(Debug, Default)]
struct PackedArrayEligibilityContext {
    records: BTreeMap<String, RecordInfo>,
    ordinary_boxes: BTreeSet<String>,
    brands: BTreeMap<String, String>,
    aliases: BTreeMap<String, String>,
}

pub(super) fn check_packed_array_eligibility(ast: &ASTNode) -> Result<(), String> {
    let context = collect_context(ast);
    check_node(ast, &context)
}

fn collect_context(ast: &ASTNode) -> PackedArrayEligibilityContext {
    let mut context = PackedArrayEligibilityContext::default();
    let ASTNode::Program { statements, .. } = ast else {
        return context;
    };

    for statement in statements {
        match statement {
            ASTNode::BoxDeclaration {
                name,
                is_record,
                type_parameters,
                field_decls,
                ..
            } => {
                if *is_record {
                    context.records.insert(
                        name.clone(),
                        RecordInfo {
                            type_parameters: type_parameters.clone(),
                            fields: field_decls.clone(),
                        },
                    );
                } else {
                    context.ordinary_boxes.insert(name.clone());
                }
            }
            ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } => {
                context
                    .brands
                    .insert(name.clone(), underlying_type_name.clone());
            }
            ASTNode::TypeAliasDeclaration {
                name,
                target_type_name,
                ..
            } => {
                context
                    .aliases
                    .insert(name.clone(), target_type_name.clone());
            }
            _ => {}
        }
    }

    context
}

fn check_node(node: &ASTNode, context: &PackedArrayEligibilityContext) -> Result<(), String> {
    match node {
        ASTNode::Program { statements, .. } => check_statements(statements, context),
        ASTNode::BoxDeclaration {
            field_decls,
            methods,
            constructors,
            static_init,
            ..
        } => {
            check_field_decls(field_decls, context)?;
            for method in methods.values() {
                check_node(method, context)?;
            }
            for constructor in constructors.values() {
                check_node(constructor, context)?;
            }
            if let Some(static_init) = static_init {
                check_statements(static_init, context)?;
            }
            Ok(())
        }
        ASTNode::EnumDeclaration { variants, .. } => {
            for variant in variants {
                if let Some(payload_type_name) = variant.payload_type_name.as_deref() {
                    check_type_text(payload_type_name, context)?;
                }
                for payload_type_name in &variant.tuple_payload_type_names {
                    check_type_text(payload_type_name, context)?;
                }
                check_field_decls(&variant.record_field_decls, context)?;
            }
            Ok(())
        }
        ASTNode::BrandDeclaration {
            underlying_type_name,
            ..
        } => check_type_text(underlying_type_name, context),
        ASTNode::TypeAliasDeclaration {
            target_type_name, ..
        } => check_type_text(target_type_name, context),
        ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            ..
        } => {
            for decl in ParamDecl::with_name_fallback(param_decls, params).iter() {
                if let Some(type_name) = decl.declared_type_name.as_deref() {
                    check_type_text(type_name, context)?;
                }
            }
            if let Some(return_type_name) = return_type_name.as_deref() {
                check_type_text(return_type_name, context)?;
            }
            check_statements(body, context)
        }
        _ => Ok(()),
    }
}

fn check_statements(
    statements: &[ASTNode],
    context: &PackedArrayEligibilityContext,
) -> Result<(), String> {
    for statement in statements {
        check_node(statement, context)?;
    }
    Ok(())
}

fn check_field_decls(
    field_decls: &[FieldDecl],
    context: &PackedArrayEligibilityContext,
) -> Result<(), String> {
    for decl in field_decls {
        if let Some(type_name) = decl.declared_type_name.as_deref() {
            check_type_text(type_name, context)?;
        }
    }
    Ok(())
}

fn check_type_text(
    type_text: &str,
    context: &PackedArrayEligibilityContext,
) -> Result<(), String> {
    let type_ref = parse_type_ref_text(type_text)?;
    check_type_ref(&type_ref, context)
}

fn check_type_ref(
    type_ref: &TypeRef,
    context: &PackedArrayEligibilityContext,
) -> Result<(), String> {
    if type_ref.name == PACKED_ARRAY_TYPE {
        if let Some(element) = type_ref.args.first() {
            check_packed_array_element(type_ref, element, context)?;
        }
    }
    for arg in &type_ref.args {
        check_type_ref(arg, context)?;
    }
    Ok(())
}

fn check_packed_array_element(
    packed_type: &TypeRef,
    element_type: &TypeRef,
    context: &PackedArrayEligibilityContext,
) -> Result<(), String> {
    if !element_type.args.is_empty() {
        return Err(packed_error(
            "generic-element",
            packed_type,
            format!(" element={}", format_type_ref(element_type)),
        ));
    }

    let Some(record) = context.records.get(&element_type.name) else {
        let reason = if context.ordinary_boxes.contains(&element_type.name) {
            "ordinary-box-element"
        } else {
            "unknown-element"
        };
        return Err(packed_error(
            reason,
            packed_type,
            format!(" element={}", element_type.name),
        ));
    };

    if !record.type_parameters.is_empty() {
        return Err(packed_error(
            "generic-record",
            packed_type,
            format!(" record={}", element_type.name),
        ));
    }
    if record.fields.is_empty() {
        return Err(packed_error(
            "empty-record",
            packed_type,
            format!(" record={}", element_type.name),
        ));
    }

    for field in &record.fields {
        if field.is_weak {
            return Err(packed_error(
                "weak-field",
                packed_type,
                format!(" record={} field={}", element_type.name, field.name),
            ));
        }
        let Some(field_type) = field.declared_type_name.as_deref() else {
            return Err(packed_error(
                "untyped-field",
                packed_type,
                format!(" record={} field={}", element_type.name, field.name),
            ));
        };
        if !is_integer_lane_type(field_type, context, 0) {
            return Err(packed_error(
                "unsupported-field-storage",
                packed_type,
                format!(
                    " record={} field={} field_type={}",
                    element_type.name, field.name, field_type
                ),
            ));
        }
    }

    Ok(())
}

fn is_integer_lane_type(
    type_text: &str,
    context: &PackedArrayEligibilityContext,
    depth: usize,
) -> bool {
    if depth > 16 || type_text.contains("[]") {
        return false;
    }
    let Ok(type_ref) = parse_type_ref_text(type_text) else {
        return false;
    };
    if !type_ref.args.is_empty() {
        return false;
    }
    if is_exact_integer_lane_type_name(&type_ref.name) {
        return true;
    }
    if let Some(target) = context.aliases.get(&type_ref.name) {
        return is_integer_lane_type(target, context, depth + 1);
    }
    if let Some(underlying) = context.brands.get(&type_ref.name) {
        return is_integer_lane_type(underlying, context, depth + 1);
    }
    false
}

fn is_exact_integer_lane_type_name(type_name: &str) -> bool {
    matches!(
        type_name,
        "i8" | "i16" | "i32" | "i64" | "isize" | "u8" | "u16" | "u32" | "u64" | "usize"
    )
}

fn packed_error(reason: &str, packed_type: &TypeRef, detail: String) -> String {
    format!(
        "[packed/eligibility] reason={} type={}{}",
        reason,
        format_type_ref(packed_type),
        detail
    )
}

fn format_type_ref(type_ref: &TypeRef) -> String {
    if type_ref.args.is_empty() {
        return type_ref.name.clone();
    }
    let args = type_ref
        .args
        .iter()
        .map(format_type_ref)
        .collect::<Vec<_>>()
        .join(",");
    format!("{}<{}>", type_ref.name, args)
}
