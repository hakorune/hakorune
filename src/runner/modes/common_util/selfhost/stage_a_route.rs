/*!
 * mode-A compatibility route orchestration helper.
 *
 * Purpose:
 * - Keep `selfhost.rs` focused on high-level route sequencing.
 * - Keep mode-A compatibility child spawn/setup local while leaving Program(JSON) fallback
 *   in the explicit compat bridge only.
 */

use super::{
    child, json, runtime_route_contract, stage0_capture, stage0_capture_route,
    stage_a_compat_bridge, stage_a_policy, stage_a_spawn,
};

const STAGE_A_COMPILER_ENTRY: &str = "lang/src/compiler/entry/compiler.hako";

fn resolve_captured_mir_line(
    mir_line: &str,
) -> Result<stage_a_compat_bridge::ProgramCompatMir, String> {
    let module = json::parse_mir_json_v0_line(mir_line)
        .map_err(|e| format!("[stage-a][mir-rejected] {}", e))?;
    Ok(stage_a_compat_bridge::ProgramCompatMir {
        module,
        lane: runtime_route_contract::LANE_DIRECT,
    })
}

pub(crate) fn try_capture_stage_a_module(
    exe: &std::path::Path,
    source_name: &str,
    raw_source: &str,
    timeout_ms: u64,
    verbose_level: u8,
) -> Result<Option<stage_a_compat_bridge::ProgramCompatMir>, String> {
    let parser_prog = std::path::Path::new(STAGE_A_COMPILER_ENTRY);
    if !parser_prog.exists() {
        return Ok(None);
    }

    child::emit_runtime_route_mode(child::ROUTE_MODE_COMPAT, source_name);

    // Non-strict mode-A compatibility lanes remain explicit-only.
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

    // mode-A compatibility keeps an explicit compat-only Program(JSON) bridge for fallback.
    // Do not widen this branch into a day-to-day owner or re-anchor it on `--backend vm`.
    let cmd = stage0_capture_route::build_stage0_non_vm_capture_command(
        exe,
        parser_prog,
        &extra,
        &["NYASH_USE_NY_COMPILER", "NYASH_CLI_VERBOSE"],
        &child_env,
    );

    let Some(captured) = stage0_capture::run_captured_json_v0_command(cmd, timeout_ms) else {
        return Ok(None);
    };

    if let Some(mir_line) = captured.mir_line.as_deref() {
        return resolve_captured_mir_line(mir_line).map(Some);
    }

    match captured.program_line.as_deref() {
        Some(program_line) => stage_a_compat_bridge::resolve_program_payload_to_mir(
            exe,
            source_name,
            timeout_ms,
            verbose_level,
            program_line,
        ),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compiler_entry_path_is_stable() {
        assert_eq!(
            super::STAGE_A_COMPILER_ENTRY,
            "lang/src/compiler/entry/compiler.hako"
        );
        // Keep the route-level negative witness in this established test so
        // the fixed lib-test inventory remains stable.
        let result = super::resolve_captured_mir_line("{invalid");
        let error = match result {
            Ok(_) => panic!("captured MIR rejection must propagate"),
            Err(error) => error,
        };
        assert!(error.starts_with("[stage-a][mir-rejected]"), "{error}");
    }
}
