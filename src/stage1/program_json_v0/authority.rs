use crate::ast::ASTNode;
use crate::parser::NyashParser;
use std::collections::BTreeMap;

use super::extract::{collect_using_imports, find_static_main_box, preexpand_dev_local_aliases};
use super::lowering::{defs_json_v0_from_methods, program_json_v0_from_body};
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
    let ast = NyashParser::parse_from_string(&normalized_source)
        .map_err(|primary_error| format!("parse error (Rust parser, v0 subset): {}", primary_error))?;
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
    let mut program = program_json_v0_from_body(main_box.body)?;
    let defs = defs_json_v0_from_methods(&main_box.helper_methods)?;
    if super::trace_enabled() {
        eprintln!("[stage1/program_json_v0] serialized_defs={}", defs.len());
    }
    if !defs.is_empty() {
        let object = program
            .as_object_mut()
            .ok_or_else(|| "program json root must be object".to_string())?;
        object.insert("defs".to_string(), serde_json::Value::Array(defs));
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
