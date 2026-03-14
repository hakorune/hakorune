use crate::cli::CliGroups;
use crate::config::env::stage1;

#[derive(Debug)]
pub(super) struct ProgramJsonEmitRequest {
    pub(super) source_path: String,
    pub(super) out_path: String,
}

pub(super) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    emit_program_json_out_path_ref(groups).is_some()
}

pub(super) fn build_emit_request(groups: &CliGroups) -> Result<ProgramJsonEmitRequest, String> {
    let source_path = resolve_emit_program_json_source_path(groups)?;
    let out_path = resolve_emit_program_json_out_path(groups);
    Ok(ProgramJsonEmitRequest {
        source_path,
        out_path,
    })
}

fn resolve_emit_program_json_source_path(groups: &CliGroups) -> Result<String, String> {
    stage1::input_path()
        .or_else(|| groups.input.file.as_ref().cloned())
        .ok_or_else(|| "emit-program-json-v0 requires an input file".to_string())
}

fn resolve_emit_program_json_out_path(groups: &CliGroups) -> String {
    emit_program_json_out_path_ref(groups)
        .expect("emit-program-json-v0 flag should be present")
        .clone()
}

fn emit_program_json_out_path_ref(groups: &CliGroups) -> Option<&String> {
    groups.emit.emit_program_json_v0.as_ref()
}

#[cfg(test)]
mod tests {
    use super::{build_emit_request, emit_program_json_v0_requested};
    use crate::cli::CliConfig;
    use crate::runner::stage1_bridge::test_support::env_lock;

    #[test]
    fn emit_program_json_v0_requested_reports_exact_flag_presence() {
        let groups = CliConfig::default().as_groups();
        assert!(!emit_program_json_v0_requested(&groups));

        let mut groups = CliConfig::default().as_groups();
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());
        assert!(emit_program_json_v0_requested(&groups));
    }

    #[test]
    fn build_emit_request_captures_exact_out_path() {
        let _guard = env_lock().lock().expect("env lock");
        std::env::remove_var("HAKO_STAGE1_INPUT");
        std::env::remove_var("NYASH_STAGE1_INPUT");
        std::env::remove_var("STAGE1_SOURCE");
        std::env::remove_var("STAGE1_INPUT");

        let mut groups = CliConfig::default().as_groups();
        groups.input.file = Some("/tmp/source.hako".to_string());
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());

        let request = build_emit_request(&groups).expect("emit request");

        assert_eq!(request.source_path, "/tmp/source.hako");
        assert_eq!(request.out_path, "/tmp/out.json");
    }

    #[test]
    fn build_emit_request_prefers_stage1_env_aliases() {
        let _guard = env_lock().lock().expect("env lock");
        std::env::remove_var("HAKO_STAGE1_INPUT");
        std::env::remove_var("NYASH_STAGE1_INPUT");
        std::env::remove_var("STAGE1_SOURCE");
        std::env::remove_var("STAGE1_INPUT");
        std::env::set_var("NYASH_STAGE1_INPUT", "/tmp/from-env.hako");

        let mut groups = CliConfig::default().as_groups();
        groups.input.file = Some("/tmp/from-cli.hako".to_string());
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());

        let request = build_emit_request(&groups).expect("emit request");

        std::env::remove_var("NYASH_STAGE1_INPUT");
        assert_eq!(request.source_path, "/tmp/from-env.hako");
    }

    #[test]
    fn build_emit_request_falls_back_to_cli_input() {
        let _guard = env_lock().lock().expect("env lock");
        std::env::remove_var("HAKO_STAGE1_INPUT");
        std::env::remove_var("NYASH_STAGE1_INPUT");
        std::env::remove_var("STAGE1_SOURCE");
        std::env::remove_var("STAGE1_INPUT");

        let mut groups = CliConfig::default().as_groups();
        groups.input.file = Some("/tmp/from-cli.hako".to_string());
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());

        let request = build_emit_request(&groups).expect("emit request");
        assert_eq!(request.source_path, "/tmp/from-cli.hako");
    }

    #[test]
    fn build_emit_request_requires_input() {
        let _guard = env_lock().lock().expect("env lock");
        std::env::remove_var("HAKO_STAGE1_INPUT");
        std::env::remove_var("NYASH_STAGE1_INPUT");
        std::env::remove_var("STAGE1_SOURCE");
        std::env::remove_var("STAGE1_INPUT");

        let mut groups = CliConfig::default().as_groups();
        groups.emit.emit_program_json_v0 = Some("/tmp/out.json".to_string());
        let error = build_emit_request(&groups).expect_err("missing input must fail");
        assert_eq!(error, "emit-program-json-v0 requires an input file");
    }
}
