use super::type_ref::{parse_type_ref_text, TypeRef};
use crate::ast::{ASTNode, FieldDecl, ParamDecl};
use std::collections::BTreeMap;

pub(super) fn check_generic_arities(ast: &ASTNode) -> Result<(), String> {
    let arities = collect_generic_arities(ast);
    check_node(ast, &arities)
}

fn collect_generic_arities(ast: &ASTNode) -> BTreeMap<String, usize> {
    let mut arities = builtin_generic_arities();
    let ASTNode::Program { statements, .. } = ast else {
        return arities;
    };

    for statement in statements {
        match statement {
            ASTNode::BoxDeclaration {
                name,
                type_parameters,
                ..
            } => {
                arities.insert(name.clone(), type_parameters.len());
            }
            ASTNode::EnumDeclaration {
                name,
                type_parameters,
                ..
            } => {
                arities.insert(name.clone(), type_parameters.len());
            }
            _ => {}
        }
    }
    arities
}

fn builtin_generic_arities() -> BTreeMap<String, usize> {
    [
        ("Array", 1),
        ("PackedArray", 1),
        ("Span", 1),
        ("Option", 1),
        ("Result", 2),
    ]
    .into_iter()
    .map(|(name, arity)| (name.to_string(), arity))
    .collect()
}

fn check_node(node: &ASTNode, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    match node {
        ASTNode::Program { statements, .. } => check_statements(statements, arities),
        ASTNode::BoxDeclaration {
            field_decls,
            methods,
            constructors,
            static_init,
            ..
        } => {
            check_field_decls(field_decls, arities)?;
            for method in methods.values() {
                check_node(method, arities)?;
            }
            for constructor in constructors.values() {
                check_node(constructor, arities)?;
            }
            if let Some(static_init) = static_init {
                check_statements(static_init, arities)?;
            }
            Ok(())
        }
        ASTNode::EnumDeclaration { variants, .. } => {
            for variant in variants {
                if let Some(payload_type_name) = variant.payload_type_name.as_deref() {
                    check_type_text(payload_type_name, arities)?;
                }
                for payload_type_name in &variant.tuple_payload_type_names {
                    check_type_text(payload_type_name, arities)?;
                }
                check_field_decls(&variant.record_field_decls, arities)?;
            }
            Ok(())
        }
        ASTNode::BrandDeclaration {
            underlying_type_name,
            ..
        } => check_type_text(underlying_type_name, arities),
        ASTNode::TypeAliasDeclaration {
            target_type_name, ..
        } => check_type_text(target_type_name, arities),
        ASTNode::FunctionDeclaration {
            params,
            param_decls,
            return_type_name,
            body,
            ..
        } => {
            for decl in ParamDecl::with_name_fallback(param_decls, params).iter() {
                if let Some(type_name) = decl.declared_type_name.as_deref() {
                    check_type_text(type_name, arities)?;
                }
            }
            if let Some(return_type_name) = return_type_name.as_deref() {
                check_type_text(return_type_name, arities)?;
            }
            check_statements(body, arities)
        }
        _ => Ok(()),
    }
}

fn check_statements(
    statements: &[ASTNode],
    arities: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for statement in statements {
        check_node(statement, arities)?;
    }
    Ok(())
}

fn check_field_decls(
    field_decls: &[FieldDecl],
    arities: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for decl in field_decls {
        if let Some(type_name) = decl.declared_type_name.as_deref() {
            check_type_text(type_name, arities)?;
        }
    }
    Ok(())
}

fn check_type_text(type_text: &str, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    let type_ref = parse_type_ref_text(type_text)?;
    check_type_ref(&type_ref, arities)
}

fn check_type_ref(type_ref: &TypeRef, arities: &BTreeMap<String, usize>) -> Result<(), String> {
    if let Some(expected) = arities.get(&type_ref.name) {
        let actual = type_ref.args.len();
        if actual != *expected {
            return Err(format!(
                "[generic/arity] type={} expected={} actual={}",
                type_ref.name, expected, actual
            ));
        }
    }
    for arg in &type_ref.args {
        check_type_ref(arg, arities)?;
    }
    Ok(())
}
