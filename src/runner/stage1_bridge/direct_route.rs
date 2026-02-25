/*!
 * Stage-1 bridge binary-only direct route executor.
 *
 * Keeps direct route logic isolated from the main Stage-1 stub orchestration.
 */

use super::NyashRunner;
use crate::cli::CliGroups;
use crate::config::env::stage1;
use crate::mir::MirPrinter;

fn compile_mir_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<crate::mir::MirModule, String> {
    let source = stage1::input_path()
        .or_else(|| groups.input.file.as_ref().cloned())
        .ok_or_else(|| "input file is required".to_string())?;

    let code =
        std::fs::read_to_string(&source).map_err(|e| format!("read error: {}: {}", source, e))?;

    let ast = crate::parser::NyashParser::parse_from_string(&code)
        .map_err(|e| format!("parse error: {}", e))?;
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

    let mut mir_compiler = crate::mir::MirCompiler::with_options(!runner.config.no_optimize);
    let compile_result = crate::runner::modes::common_util::source_hint::compile_with_source_hint(
        &mut mir_compiler,
        ast,
        Some(&source),
    )
    .map_err(|e| format!("MIR compilation error: {}", e))?;

    Ok(compile_result.module)
}

fn dump_mir_if_requested(groups: &CliGroups, module: &crate::mir::MirModule) {
    if groups.debug.dump_mir {
        let mut printer = if groups.debug.mir_verbose {
            MirPrinter::verbose()
        } else {
            MirPrinter::new()
        };
        if groups.debug.mir_verbose_effects {
            printer.set_show_effects_inline(true);
        }
        println!("{}", printer.print_module(module));
    }
}

pub(super) fn emit_mir_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<(), String> {
    let module = compile_mir_binary_only_direct(runner, groups)?;
    dump_mir_if_requested(groups, &module);

    let out_path = groups.emit.emit_mir_json.clone().or_else(|| {
        std::env::var("NYASH_STAGE1_EMIT_MIR_OUT")
            .ok()
            .or_else(|| std::env::var("HAKO_STAGE1_EMIT_MIR_OUT").ok())
    });
    let Some(path) = out_path else {
        return Err("output path is required".to_string());
    };

    let p = std::path::Path::new(&path);
    crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(&module, p)
        .map_err(|e| format!("MIR JSON emit error: {}", e))?;
    println!("MIR JSON written: {}", p.display());
    Ok(())
}

pub(super) fn run_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<i32, String> {
    if groups.backend.backend != "vm" {
        return Err(format!(
            "unsupported backend for run binary-only direct route: {}",
            groups.backend.backend
        ));
    }
    let module = compile_mir_binary_only_direct(runner, groups)?;
    dump_mir_if_requested(groups, &module);
    Ok(runner.execute_mir_module_quiet_exit(&module))
}
