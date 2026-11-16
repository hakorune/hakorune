//! Explicit independent builder selection. No retry or old source preflight.
use super::super::NyashRunner;
use nyash_rust::{mir, parser::NyashParser};
use std::{fs, process};

pub(super) fn selected() -> Result<bool, String> {
    match std::env::var("HAKO_MIR_BUILDER").ok().as_deref() {
        None | Some("current") => Ok(false),
        Some("blueprint") => Ok(true),
        Some(other) => Err(format!("[blueprint/config/UnknownBuilder] {other}")),
    }
}

pub(super) fn execute(runner: &NyashRunner, filename: &str) {
    if let Err(error) = run(runner, filename) {
        eprintln!("{error}");
        process::exit(1);
    }
}

fn run(runner: &NyashRunner, filename: &str) -> Result<(), String> {
    let code = fs::read_to_string(filename).map_err(|e| format!("[blueprint/input] {e}"))?;
    let ast = NyashParser::parse_from_string_with_build_config(&code, runner.parser_build_config())
        .map_err(|e| format!("[blueprint/parse] {e}"))?;
    let mut module = mir::blueprint_builder::compile(ast)?;
    let options = runner.config.as_groups();
    if options.debug.dump_mir {
        println!("{}", mir::MirPrinter::new().print_module(&module));
    }
    if options.debug.verify_mir {
        println!("blueprint: structural and source correspondence verification passed");
    }
    // Existing backend-only projections. They do not issue source facts or
    // call the default builder, semantic refresh, or family preflight.
    mir::global_call_route_plan::refresh_module_global_call_routes(&mut module);
    mir::same_module_definition_plan::refresh_module_same_module_definition_plans(&mut module);
    if let Some(path) = &options.emit.emit_mir_json {
        return crate::runner::mir_json_emit::emit_mir_json_for_selected_dynamic_candidate(
            &module,
            std::path::Path::new(path),
        );
    }
    if let Some(path) = &options.emit.emit_exe {
        return crate::runner::modes::common_util::exec::emit_json_and_run_ny_llvmc_emit_exe(
            |json| {
                crate::runner::mir_json_emit::emit_mir_json_for_selected_dynamic_candidate(
                    &module, json,
                )
            },
            path,
            options.emit.emit_exe_nyrt.as_deref(),
            options.emit.emit_exe_libs.as_deref(),
            None,
            None,
        );
    }
    if options.debug.dump_mir || options.debug.verify_mir {
        return Ok(());
    }
    Err(
        "[blueprint/config/ArtifactRequired] select --emit-exe, --emit-mir-json or --dump-mir"
            .into(),
    )
}
