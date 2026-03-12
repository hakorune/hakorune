/*!
 * Stage-1 CLI bridge — delegate to Hako Stage1 stub when explicitly enabled.
 *
 * - Entry: NYASH_USE_STAGE1_CLI=1 (default OFF).
 * - Toggle guard for child recursion: NYASH_STAGE1_CLI_CHILD=1 (set by bridge).
 * - Entry path override: STAGE1_CLI_ENTRY or HAKORUNE_STAGE1_ENTRY (optional).
 * - Mode toggles:
 *     - STAGE1_EMIT_PROGRAM_JSON=1 : emit program-json <source.hako>
 *     - STAGE1_EMIT_MIR_JSON=1     : emit mir-json (<source.hako> or STAGE1_PROGRAM_JSON)
 *     - STAGE1_BACKEND={vm|llvm} hint for run path (default: CLI backend)
 *
 * Notes
 * - This bridge aims to keep Rust Stage0 thin: it only invokes the Stage1 stub
 *   (lang/src/runner/stage1_cli.hako) with script args and exits with the stub's code.
 * - Exact execution plan selection lives in `plan.rs`.
 * - Bridge entry child/enable guard + trace logging live in `entry_guard.rs`.
 * - Stub capture-vs-delegate execution contract lives in `args.rs::Stage1StubExecPlan`.
 * - Route execution facade lives in `route_exec.rs`.
 * - Binary-only direct route execution + exit-code mapping live in `route_exec/direct.rs`.
 * - Stage1 stub route facade lives in `route_exec/stub.rs`.
 * - Binary-only direct-route facade/compile/emit helpers live in `direct_route/`.
 * - Bridge-local emit output-path policy lives in `emit_paths.rs`.
 * - Stage1 stub entry resolution + child command/env assembly + prepare-failure mapping live in `stub_child.rs`.
 * - Stage1 stub plain delegate-status execution + child-spawn-failure mapping live in `stub_delegate.rs`.
 * - Stub emit facade/parse/writeback helpers live in `stub_emit.rs` + `stub_emit/`.
 * - Child env section policy lives behind `env.rs` and the `env/` helper modules.
 * - Stage1 stub emit stdout parse / validation / writeback live behind `stub_emit.rs` + `stub_emit/`.
 * - Bridge-local Program(JSON v0) file I/O lives in `program_json/mod.rs`.
 * - Bridge-local Program(JSON v0) entry facade lives in `program_json_entry.rs`.
 * - When toggles are unset or this is a child invocation, the bridge is a no-op.
 */

mod args;
mod direct_route;
mod emit_paths;
mod entry_guard;
mod env;
mod modules;
mod plan;
mod program_json;
pub(super) mod program_json_entry;
mod route_exec;
mod stub_child;
mod stub_delegate;
mod stub_emit;
#[cfg(test)]
mod test_support;

use super::NyashRunner;
use crate::cli::CliGroups;

impl NyashRunner {
    /// If enabled, run the Stage-1 CLI stub as a child process and return its exit code.
    /// Returns None when the bridge is not engaged.
    pub(crate) fn maybe_run_stage1_cli_stub(&self, groups: &CliGroups) -> Option<i32> {
        if !entry_guard::should_engage() {
            return None;
        }

        // Build args
        let args_result = args::build_stage1_args(groups);
        Some(route_exec::execute(
            self,
            groups,
            &args_result,
            plan::decide(&args_result),
        ))
    }
}
