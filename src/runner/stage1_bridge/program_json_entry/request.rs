use crate::cli::CliGroups;
use crate::config::env::stage1;

#[derive(Debug)]
pub(super) struct ProgramJsonEmitRequest {
    pub(super) source_path: String,
    pub(super) out_path: String,
}

#[derive(Debug)]
pub(super) struct ProgramJsonEmitResponse {
    pub(super) out_path: String,
    pub(super) result: Result<(), String>,
}

impl ProgramJsonEmitRequest {
    pub(super) fn build(groups: &CliGroups) -> Result<Self, String> {
        Ok(Self {
            source_path: Self::resolve_source_path(groups)?,
            out_path: Self::resolve_out_path(groups),
        })
    }

    pub(super) fn execute(self) -> ProgramJsonEmitResponse {
        ProgramJsonEmitResponse {
            out_path: self.out_path.clone(),
            result: crate::runner::stage1_bridge::program_json::emit_program_json_v0(
                &self.source_path,
                &self.out_path,
            ),
        }
    }

    fn resolve_source_path(groups: &CliGroups) -> Result<String, String> {
        stage1::input_path()
            .or_else(|| groups.input.file.as_ref().cloned())
            .ok_or_else(|| "emit-program-json-v0 requires an input file".to_string())
    }

    fn resolve_out_path(groups: &CliGroups) -> String {
        emit_program_json_out_path_ref(groups)
            .expect("emit-program-json-v0 flag should be present")
            .clone()
    }
}

pub(super) fn emit_program_json_v0_requested(groups: &CliGroups) -> bool {
    emit_program_json_out_path_ref(groups).is_some()
}

fn emit_program_json_out_path_ref(groups: &CliGroups) -> Option<&String> {
    groups.emit.emit_program_json_v0.as_ref()
}

#[cfg(test)]
mod tests {
    use super::{emit_program_json_v0_requested, ProgramJsonEmitRequest};
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

        let request = ProgramJsonEmitRequest::build(&groups).expect("emit request");

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

        let request = ProgramJsonEmitRequest::build(&groups).expect("emit request");

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

        let request = ProgramJsonEmitRequest::build(&groups).expect("emit request");
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
        let error = ProgramJsonEmitRequest::build(&groups).expect_err("missing input must fail");
        assert_eq!(error, "emit-program-json-v0 requires an input file");
    }

    #[test]
    fn execute_preserves_exact_out_path_in_response() {
        let _guard = env_lock().lock().expect("env lock");
        std::env::remove_var("HAKO_STAGE1_INPUT");
        std::env::remove_var("NYASH_STAGE1_INPUT");
        std::env::remove_var("STAGE1_SOURCE");
        std::env::remove_var("STAGE1_INPUT");

        let unique = format!(
            "hakorune-stage1-bridge-entry-request-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let source_path = std::env::temp_dir().join(format!("{unique}.hako"));
        let out_path = std::env::temp_dir().join(format!("{unique}.json"));

        std::fs::write(
            &source_path,
            include_str!("../../../../lang/src/runner/stage1_cli_env.hako"),
        )
        .expect("write temp source");

        let mut groups = CliConfig::default().as_groups();
        groups.input.file = Some(source_path.to_string_lossy().into_owned());
        groups.emit.emit_program_json_v0 = Some(out_path.to_string_lossy().into_owned());

        let response = ProgramJsonEmitRequest::build(&groups)
            .expect("emit request")
            .execute();

        let _ = std::fs::remove_file(&source_path);
        let _ = std::fs::remove_file(&out_path);

        assert_eq!(response.out_path, out_path.to_string_lossy());
        assert!(response.result.is_ok());
    }
}
