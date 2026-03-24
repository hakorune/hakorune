use super::NyashRunner;
use crate::cli::CliGroups;
use crate::runner::stage1_bridge::program_json_entry;

impl NyashRunner {
    pub(super) fn maybe_emit_and_exit(&self, groups: &CliGroups) -> bool {
        // RVP-0-min1:
        // hako-prefixed routes are stage1-only; rust emit path must not absorb them implicitly.
        if groups.emit.hako_emit_program_json
            || groups.emit.hako_emit_mir_json
            || groups.emit.hako_run
        {
            eprintln!(
                "[freeze:contract][stage1-route/hako-cli] expected=stage1-stub got=rust-emit"
            );
            std::process::exit(1);
        }

        // Emit Program(JSON v0) via Stage-1 bridge entry and exit (explicit SSOT flag).
        if program_json_entry::emit_program_json_v0_requested(groups) {
            crate::runtime::deprecations::warn_stage1_bridge_program_json_route_once();
            program_json_entry::emit_program_json_v0_and_exit(groups);
        }

        // Emit AST JSON and exit (direct Rust parser route).
        if let Some(path) = groups.emit.emit_ast_json.as_ref() {
            let Some(file) = groups.input.file.as_ref() else {
                eprintln!("❌ --emit-ast-json requires an input file");
                std::process::exit(1);
            };

            // Phase 90-A: fs 系移行
            let ring0 = crate::runtime::ring0::get_global_ring0();
            let code = match ring0.fs.read_to_string(std::path::Path::new(file)) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("❌ Error reading file {}: {}", file, e);
                    std::process::exit(1);
                }
            };

            let ast = match crate::parser::NyashParser::parse_from_string(&code) {
                Ok(ast) => ast,
                Err(e) => {
                    crate::runner::modes::common_util::diag::print_parse_error_with_context(
                        file, &code, &e,
                    );
                    std::process::exit(1);
                }
            };

            let prog = crate::r#macro::ast_json::ast_to_json_roundtrip(&ast);
            let out_path = std::path::Path::new(path);
            if let Err(e) = std::fs::write(out_path, prog.to_string()) {
                eprintln!("❌ emit-ast-json write error: {}", e);
                std::process::exit(1);
            }
            println!("AST JSON written: {}", out_path.display());
            std::process::exit(0);
        }

        false
    }
}
