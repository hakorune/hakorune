/*!
 * Stage-A route orchestration helper.
 *
 * Purpose:
 * - Keep `selfhost.rs` focused on high-level route sequencing.
 * - Keep Stage-A child spawn/setup and captured payload handoff under one thin owner.
 */

use super::{
    child, stage0_capture, stage0_capture_route, stage_a_compat_bridge, stage_a_policy,
    stage_a_spawn,
};

const STAGE_A_COMPILER_ENTRY: &str = "lang/src/compiler/entry/compiler.hako";

pub(crate) fn try_capture_stage_a_module(
    exe: &std::path::Path,
    source_name: &str,
    raw_source: &str,
    timeout_ms: u64,
    verbose_level: u8,
) -> Option<stage_a_compat_bridge::ProgramCompatMir> {
    let parser_prog = std::path::Path::new(STAGE_A_COMPILER_ENTRY);
    if !parser_prog.exists() {
        return None;
    }

    child::emit_runtime_route_mode(child::ROUTE_MODE_STAGE_A, source_name);

    // Non-strict Stage-A compat lanes remain explicit-only.
    stage_a_policy::enforce_stage_a_compat_policy_or_exit(source_name);

    if verbose_level >= 2 {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.info(&format!(
            "[selfhost/ny] spawning Ny compiler child process: {}",
            parser_prog.display()
        ));
    }

    let extra_owned = stage_a_spawn::build_stage_a_child_extra_args();
    let extra: Vec<&str> = extra_owned.iter().map(|s| s.as_str()).collect();
    let child_env_owned = stage_a_spawn::build_stage_a_child_env(raw_source);
    let child_env: Vec<(&str, &str)> = child_env_owned
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    // Stage-A source path is now direct/core-first.
    // Keep the compat-only Program(JSON) bridge for fallback, but do not
    // anchor the mainline capture on `--backend vm`.
    let cmd = stage0_capture_route::build_stage0_non_vm_capture_command(
        exe,
        parser_prog,
        &extra,
        &["NYASH_USE_NY_COMPILER", "NYASH_CLI_VERBOSE"],
        &child_env,
    );

    let captured = stage0_capture::run_captured_json_v0_command(cmd, timeout_ms)?;

    stage_a_compat_bridge::resolve_captured_payload_to_mir(
        exe,
        source_name,
        timeout_ms,
        verbose_level,
        captured.mir_line.as_deref(),
        captured.program_line.as_deref(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiler_entry_path_is_stable() {
        assert_eq!(
            super::STAGE_A_COMPILER_ENTRY,
            "lang/src/compiler/entry/compiler.hako"
        );
    }
}
