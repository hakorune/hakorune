/*!
 * Stage-1 bridge binary-only direct route - MIR compilation helper.
 */

use crate::cli::CliGroups;
use crate::config::env::stage1;
use crate::mir::{MirModule, MirPrinter};
use crate::runner::NyashRunner;

pub(super) fn compile_and_maybe_dump(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<MirModule, String> {
    let module = compile_mir_binary_only_direct(runner, groups)?;
    dump_mir_if_requested(groups, &module);
    Ok(module)
}

fn compile_mir_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<MirModule, String> {
    let source = stage1::input_path()
        .or_else(|| groups.input.file.as_ref().cloned())
        .ok_or_else(|| "input file is required".to_string())?;

    let code = std::fs::read_to_string(&source)
        .map_err(|error| format!("read error: {}: {}", source, error))?;

    let ast = crate::parser::NyashParser::parse_from_string(&code)
        .map_err(|error| format!("parse error: {}", error))?;
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

    let mut mir_compiler = crate::mir::MirCompiler::with_options(!runner.config.no_optimize);
    let compile_result = crate::runner::modes::common_util::source_hint::compile_with_source_hint(
        &mut mir_compiler,
        ast,
        Some(&source),
    )
    .map_err(|error| format!("MIR compilation error: {}", error))?;

    Ok(compile_result.module)
}

fn dump_mir_if_requested(groups: &CliGroups, module: &MirModule) {
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
