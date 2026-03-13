use crate::mir::MirModule;
use serde_json::Value as JsonValue;

pub(super) fn lower_ast_json_to_module(parsed: &JsonValue) -> Result<MirModule, String> {
    // Route B (Phase-0 bridge): AST JSON (from `--emit-ast-json`, legacy: `--emit-program-json`)
    // -> MIR Module. This remains a compat keep while the current `.hako` authority still
    // flows through Program(JSON v0).
    let Some(ast) = crate::r#macro::ast_json::json_to_ast(parsed) else {
        return Err(super::super::failfast_error(
            "unsupported JSON input (expected Program(JSON v0) or AST JSON)",
        ));
    };

    let mut builder = crate::mir::builder::MirBuilder::new();
    builder
        .build_module(ast)
        .map_err(super::super::failfast_error)
}
