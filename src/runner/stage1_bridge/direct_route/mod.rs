/*!
 * Stage-1 bridge binary-only direct route facade.
 *
 * Keeps direct route entrypoints thin while delegating compile and emit-output
 * policy to focused helpers.
 */

mod compile;
mod emit;

use super::NyashRunner;
use crate::cli::CliGroups;

pub(super) fn emit_mir_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<(), String> {
    let module = compile::compile_and_maybe_dump(runner, groups)?;
    emit::emit_mir_json(&module, groups.emit.emit_mir_json.clone())
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
    let module = compile::compile_and_maybe_dump(runner, groups)?;
    Ok(runner.execute_mir_module_quiet_exit(&module))
}
