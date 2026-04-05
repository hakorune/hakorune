/*!
 * Stage-1 bridge route executor - binary-only direct route helper.
 */

use crate::cli::CliGroups;
use crate::runner::NyashRunner;

pub(super) fn execute_emit_mir(
    runner: &NyashRunner,
    groups: &CliGroups,
    reason: &'static str,
) -> i32 {
    crate::runtime::get_global_ring0().log.warn(&format!(
        "[stage1-cli] emit-mir: binary-only direct route engaged (legacy keep, {})",
        reason
    ));
    match super::super::direct_route::emit_mir_binary_only_direct(runner, groups) {
        Ok(()) => 0,
        Err(error) => {
            crate::runtime::get_global_ring0()
                .log
                .error(&format!("[stage1-cli] emit-mir(binary-only): {}", error));
            98
        }
    }
}

pub(super) fn execute_run(runner: &NyashRunner, groups: &CliGroups, reason: &'static str) -> i32 {
    crate::runtime::get_global_ring0().log.warn(&format!(
        "[stage1-cli] run: binary-only direct route engaged (legacy keep, {})",
        reason
    ));
    match super::super::direct_route::run_binary_only_direct(runner, groups) {
        Ok(rc) => rc,
        Err(error) => {
            crate::runtime::get_global_ring0()
                .log
                .error(&format!("[stage1-cli] run(binary-only): {}", error));
            98
        }
    }
}
