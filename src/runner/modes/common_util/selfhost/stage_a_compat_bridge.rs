/*!
 * Stage-A Program(JSON v0) compatibility ladder.
 *
 * Purpose:
 * - Keep `selfhost.rs` focused on route sequencing.
 * - Keep Program(JSON v0) -> MIR(JSON v0) fallback ownership in one box.
 */

use crate::mir::MirModule;

use super::{child, json, runtime_route_contract, stage_a_policy};

const MIR_BUILDER_PROGRAM_PATH: &str =
    "lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako";
const CHILD_ENV_REMOVE: &[&str] = &["NYASH_USE_NY_COMPILER", "NYASH_CLI_VERBOSE"];

pub(crate) struct ProgramCompatMir {
    pub(crate) module: MirModule,
    pub(crate) lane: &'static str,
}

// Captured Stage-A payload-family resolution lives here, not in `selfhost.rs`.
// Keep direct MIR acceptance and Program(JSON v0) compat fallback under one thin owner.
pub(crate) fn resolve_captured_payload_to_mir(
    exe: &std::path::Path,
    source_name: &str,
    timeout_ms: u64,
    verbose_level: u8,
    mir_line: Option<&str>,
    program_line: Option<&str>,
) -> Option<ProgramCompatMir> {
    let resolved = json::resolve_stage_a_payload(mir_line, program_line);
    if let Some(error) = resolved.mir_parse_error.as_deref() {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.error(&format!(
            "[ny-compiler] mir json parse error (child): {}",
            error
        ));
    }

    match resolved.payload {
        json::StageAPayload::MirModule(module) => Some(ProgramCompatMir {
            module,
            lane: runtime_route_contract::LANE_DIRECT,
        }),
        json::StageAPayload::ProgramJson(program_line) => resolve_program_payload_to_mir(
            exe,
            source_name,
            timeout_ms,
            verbose_level,
            program_line.as_str(),
        ),
        json::StageAPayload::Empty => None,
    }
}

pub(crate) fn resolve_program_payload_to_mir(
    exe: &std::path::Path,
    source_name: &str,
    timeout_ms: u64,
    verbose_level: u8,
    program_line: &str,
) -> Option<ProgramCompatMir> {
    // Phase D5-min1 contract:
    // strict/dev(+planner_required) must reject Program(JSON v0) at runtime route boundary.
    stage_a_policy::enforce_stage_a_program_payload_policy_or_exit(source_name);

    if verbose_level >= 2 {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.info(&format!(
            "[selfhost/ny] stage-a compat lane: Program(JSON v0) -> MIR(JSON v0) via .hako mirbuilder (size={} bytes)",
            program_line.len()
        ));
    }

    let mir_builder_prog = std::path::Path::new(MIR_BUILDER_PROGRAM_PATH);
    if !mir_builder_prog.exists() {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.error(&format!(
            "[ny-compiler] mirbuilder entry missing: {}",
            mir_builder_prog.display()
        ));
        return None;
    }

    let envs = [("HAKO_PROGRAM_JSON", program_line)];
    if let Some(mir_line) = child::run_ny_program_capture_mir_json(
        exe,
        mir_builder_prog,
        timeout_ms,
        &[],
        CHILD_ENV_REMOVE,
        &envs,
    ) {
        match json::parse_mir_json_v0_line(&mir_line) {
            Ok(module) => {
                return Some(ProgramCompatMir {
                    module,
                    lane: runtime_route_contract::LANE_COMPAT_PROGRAM_TO_MIR,
                });
            }
            Err(e) => {
                let ring0 = crate::runtime::ring0::get_global_ring0();
                ring0.log.error(&format!(
                    "[ny-compiler] mir json parse error (.hako mirbuilder): {}",
                    e
                ));
            }
        }
    } else {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0
            .log
            .error("[ny-compiler] stage-a compat lane produced no MIR(JSON v0)");
    }

    // RDM-1-min1 contract:
    // Rust Program(JSON v0)->MIR fallback is explicit compat opt-in only.
    // Mainline default must fail-fast when .hako mirbuilder does not produce MIR(JSON v0).
    stage_a_policy::enforce_stage_a_rust_json_bridge_guard_or_exit(source_name);

    // Explicit compat lane: keep runtime alive via existing Rust bridge
    // only when NYASH_VM_USE_FALLBACK=1 is set.
    match json::parse_json_v0_line(program_line) {
        Ok(module) => Some(ProgramCompatMir {
            module,
            lane: runtime_route_contract::LANE_COMPAT_RUST_JSON_V0_BRIDGE,
        }),
        Err(e) => {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0.log.error(&format!(
                "[ny-compiler] json parse error (stage-a compat fallback): {}",
                e
            ));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_captured_payload_to_mir;

    #[test]
    fn mir_builder_program_path_is_stable() {
        assert_eq!(
            super::MIR_BUILDER_PROGRAM_PATH,
            "lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako"
        );
    }

    #[test]
    fn resolve_captured_payload_to_mir_prefers_direct_lane() {
        let resolved = resolve_captured_payload_to_mir(
            std::path::Path::new("/tmp/unused-nyash"),
            "inline.ny",
            1000,
            0,
            Some(
                r#"{"kind":"MIR","schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[{"op":"const","dst":1,"value":{"type":"i64","value":7}},{"op":"ret","value":1}]}]}]}"#,
            ),
            Some(r#"{"version":0,"kind":"Program","body":[]}"#),
        )
        .expect("direct MIR payload must resolve");

        assert_eq!(resolved.lane, super::runtime_route_contract::LANE_DIRECT);
    }

    #[test]
    fn resolve_captured_payload_to_mir_returns_none_when_empty() {
        let resolved = resolve_captured_payload_to_mir(
            std::path::Path::new("/tmp/unused-nyash"),
            "inline.ny",
            1000,
            0,
            None,
            None,
        );

        assert!(resolved.is_none());
    }
}
