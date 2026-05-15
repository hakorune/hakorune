use crate::ast::{ASTNode, EnumVariantDecl, FieldDecl};
use crate::parser::NyashParser;
use std::collections::{BTreeMap, BTreeSet};

use super::brand_checker;
use super::extract::{collect_using_imports, find_static_main_box, preexpand_dev_local_aliases};
use super::generic_arity_checker;
use super::lowering::{
    defs_json_v0_from_methods, program_json_v0_from_body_with_context, ProgramJsonV0LoweringContext,
};
use super::packed_array_eligibility_checker;
use super::record_payload::{
    collect_enum_record_payload_box_decls, enum_variant_payload_type_name,
};
use super::routing;

pub(super) fn source_to_program_json_v0_relaxed(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0_impl(source_text, true)
}

pub(super) fn source_to_program_json_v0_strict(source_text: &str) -> Result<String, String> {
    source_to_program_json_v0_impl(source_text, false)
}

pub(super) fn emit_program_json_v0_for_strict_authority_source(
    source_text: &str,
) -> Result<String, String> {
    if let Some(detail) =
        routing::strict_authority_program_json_v0_source_rejection(source_text, "source route")
    {
        return Err(detail);
    }
    source_to_program_json_v0_strict(source_text)
}

fn source_to_program_json_v0_impl(
    source_text: &str,
    allow_dev_local_alias_sugar: bool,
) -> Result<String, String> {
    let imports = collect_using_imports(source_text);
    let normalized_source = if allow_dev_local_alias_sugar {
        preexpand_dev_local_aliases(source_text)
    } else {
        source_text.to_string()
    };
    let ast = NyashParser::parse_from_string(&normalized_source).map_err(|primary_error| {
        format!("parse error (Rust parser, v0 subset): {}", primary_error)
    })?;
    ast_to_program_json_v0_with_imports(&ast, imports)
}

fn ast_to_program_json_v0_with_imports(
    ast: &ASTNode,
    imports: BTreeMap<String, String>,
) -> Result<String, String> {
    let main_box = find_static_main_box(ast)
        .ok_or_else(|| "expected `static box Main { main() { ... } }`".to_string())?;
    if super::trace_enabled() {
        eprintln!(
            "[stage1/program_json_v0] main_body_stmts={} helper_defs={} imports={}",
            main_box.body.len(),
            main_box.helper_methods.len(),
            imports.len()
        );
    }
    reject_sync_box_decls(ast)?;
    generic_arity_checker::check_generic_arities(ast)?;
    packed_array_eligibility_checker::check_packed_array_eligibility(ast)?;
    let brand_decl_index = collect_brand_decl_index(ast);
    brand_checker::check_brand_mismatches(ast, &brand_decl_index)?;
    let lowering_context = ProgramJsonV0LoweringContext::with_known_enums_brands_and_records(
        collect_enum_decl_index(ast),
        brand_decl_index,
        collect_record_decl_index(ast),
        collect_source_enum_decl_names(ast),
    );
    let mut program = program_json_v0_from_body_with_context(main_box.body, &lowering_context)?;
    let defs = defs_json_v0_from_methods(&main_box.helper_methods, &lowering_context)?;
    if super::trace_enabled() {
        eprintln!("[stage1/program_json_v0] serialized_defs={}", defs.len());
    }
    if !defs.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert("defs".to_string(), serde_json::Value::Array(defs));
    }
    let user_box_decls = collect_user_box_decls(ast);
    if !user_box_decls.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "user_box_decls".to_string(),
            serde_json::Value::Array(user_box_decls),
        );
    }
    let record_decls = collect_record_decls(ast);
    if !record_decls.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "record_decls".to_string(),
            serde_json::Value::Array(record_decls),
        );
    }
    let enum_decls = collect_enum_decls(ast);
    if !enum_decls.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "enum_decls".to_string(),
            serde_json::Value::Array(enum_decls),
        );
    }
    let brand_decls = collect_brand_decls(ast);
    if !brand_decls.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "brand_decls".to_string(),
            serde_json::Value::Array(brand_decls),
        );
    }
    let type_alias_decls = collect_type_alias_decls(ast);
    if !type_alias_decls.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "type_alias_decls".to_string(),
            serde_json::Value::Array(type_alias_decls),
        );
    }
    let static_data_plans = collect_static_data_plans(ast);
    if !static_data_plans.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "static_data_plans".to_string(),
            serde_json::Value::Array(static_data_plans),
        );
    }
    if !imports.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert(
            "imports".to_string(),
            serde_json::to_value(imports)
                .map_err(|error| format!("imports serialize error: {}", error))?,
        );
    }
    serde_json::to_string(&program).map_err(|error| format!("serialize error: {}", error))
}

fn reject_sync_box_decls(ast: &ASTNode) -> Result<(), String> {
    let ASTNode::Program { statements, .. } = ast else {
        return Ok(());
    };
    for statement in statements {
        if let ASTNode::BoxDeclaration {
            name,
            is_sync: true,
            ..
        } = statement
        {
            return Err(format!(
                "[freeze:contract][program_json_v0/sync_box_not_supported] box={} sync box Program JSON lowering is owned by CONC-SYNCBOX runtime rows",
                name
            ));
        }
    }
    Ok(())
}

fn collect_user_box_decls(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    let mut decls = statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BoxDeclaration {
                name,
                fields,
                field_decls,
                delegates,
                invariants,
                transitions,
                is_record,
                type_parameters,
                ..
            } = statement
            else {
                return None;
            };
            if *is_record {
                return None;
            }
            Some(serde_json::json!({
                "name": name,
                "fields": fields,
                "type_parameters": type_parameters,
                "invariants": invariants.iter().map(crate::r#macro::ast_json::ast_to_json).collect::<Vec<_>>(),
                "transitions": transitions.iter().map(|decl| serde_json::json!({
                    "from": decl.from_state,
                    "to": decl.to_state,
                    "method": decl.method_name,
                })).collect::<Vec<_>>(),
                "field_decls": field_decls.iter().map(|decl| serde_json::json!({
                    "name": decl.name,
                    "declared_type": decl.declared_type_name,
                    "is_weak": decl.is_weak,
                })).collect::<Vec<_>>(),
                "delegates": delegates.iter().map(|decl| serde_json::json!({
                    "field_name": decl.field_name,
                    "exposes": decl.exposes.iter().map(|expose| serde_json::json!({
                        "source_name": expose.source_name,
                        "exposed_name": expose.exposed_name,
                    })).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            }))
        })
        .collect::<Vec<_>>();
    decls.extend(collect_enum_record_payload_box_decls(statements));
    decls
}

fn collect_record_decls(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BoxDeclaration {
                name,
                fields,
                field_decls,
                invariants,
                is_record,
                type_parameters,
                ..
            } = statement
            else {
                return None;
            };
            if !*is_record {
                return None;
            }
            Some(serde_json::json!({
                "name": name,
                "fields": fields,
                "type_parameters": type_parameters,
                "invariants": invariants.iter().map(crate::r#macro::ast_json::ast_to_json).collect::<Vec<_>>(),
                "field_decls": field_decls.iter().enumerate().map(|(index, decl)| serde_json::json!({
                    "name": decl.name,
                    "declared_type": decl.declared_type_name,
                    "is_weak": decl.is_weak,
                    "field_index": index,
                })).collect::<Vec<_>>(),
            }))
        })
        .collect()
}

fn collect_record_decl_index(ast: &ASTNode) -> BTreeMap<String, Vec<FieldDecl>> {
    let ASTNode::Program { statements, .. } = ast else {
        return BTreeMap::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BoxDeclaration {
                name,
                field_decls,
                is_record,
                ..
            } = statement
            else {
                return None;
            };
            if !*is_record {
                return None;
            }
            Some((name.clone(), field_decls.clone()))
        })
        .collect()
}

fn collect_enum_decl_index(ast: &ASTNode) -> BTreeMap<String, Vec<EnumVariantDecl>> {
    let mut index = crate::semantics::result_option_prelude::result_option_prelude_enum_decls();
    let ASTNode::Program { statements, .. } = ast else {
        return index;
    };

    index.extend(statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::EnumDeclaration { name, variants, .. } = statement else {
                return None;
            };
            Some((name.clone(), variants.clone()))
        })
    );
    index
}

fn collect_source_enum_decl_names(ast: &ASTNode) -> BTreeSet<String> {
    let ASTNode::Program { statements, .. } = ast else {
        return BTreeSet::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::EnumDeclaration { name, .. } = statement else {
                return None;
            };
            Some(name.clone())
        })
        .collect()
}

fn collect_brand_decl_index(ast: &ASTNode) -> BTreeMap<String, String> {
    let ASTNode::Program { statements, .. } = ast else {
        return BTreeMap::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } = statement
            else {
                return None;
            };
            Some((name.clone(), underlying_type_name.clone()))
        })
        .collect()
}

fn collect_enum_decls(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::EnumDeclaration {
                name,
                variants,
                type_parameters,
                ..
            } = statement
            else {
                return None;
            };
            Some(serde_json::json!({
                "name": name,
                "type_parameters": type_parameters,
                "variants": variants.iter().map(|variant| serde_json::json!({
                    "name": variant.name,
                    "payload_type": enum_variant_payload_type_name(name, variant),
                    "record_fields": variant.record_field_decls.iter().map(|field| serde_json::json!({
                        "name": field.name,
                        "declared_type": field.declared_type_name,
                    })).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
            }))
        })
        .collect()
}

fn collect_brand_decls(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } = statement
            else {
                return None;
            };
            Some(serde_json::json!({
                "name": name,
                "underlying_type": underlying_type_name,
            }))
        })
        .collect()
}

fn collect_type_alias_decls(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::TypeAliasDeclaration {
                name,
                target_type_name,
                ..
            } = statement
            else {
                return None;
            };
            Some(serde_json::json!({
                "name": name,
                "target_type": target_type_name,
            }))
        })
        .collect()
}

fn collect_static_data_plans(ast: &ASTNode) -> Vec<serde_json::Value> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::StaticConstTable {
                name,
                element_type,
                values,
                ..
            } = statement
            else {
                return None;
            };
            Some(serde_json::json!({
                "source_name": name,
                "symbol": format!(".hako.static.{}", name),
                "element": element_type,
                "align": static_data_alignment(element_type),
                "linkage": "private",
                "unnamed_addr": true,
                "values": values,
            }))
        })
        .collect()
}

fn static_data_alignment(element_type: &str) -> u32 {
    match element_type {
        "u8" => 1,
        "u16" => 2,
        "u32" => 4,
        "u64" => 8,
        _ => 1,
    }
}
