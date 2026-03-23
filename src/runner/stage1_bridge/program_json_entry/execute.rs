use super::request::ProgramJsonEmitRequest;

#[derive(Debug)]
pub(super) struct ProgramJsonEmitResponse {
    pub(super) out_path: String,
    pub(super) result: Result<(), String>,
}

impl ProgramJsonEmitRequest {
    pub(super) fn execute(self) -> ProgramJsonEmitResponse {
        let ProgramJsonEmitRequest {
            source_path,
            out_path,
        } = self;
        let result = crate::runner::stage1_bridge::program_json::emit_program_json_v0(
            &source_path,
            &out_path,
        );
        ProgramJsonEmitResponse {
            out_path,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProgramJsonEmitResponse;
    use crate::cli::CliConfig;
    use crate::runner::stage1_bridge::program_json_entry::request::ProgramJsonEmitRequest;
    use crate::runner::stage1_bridge::test_support::env_lock;

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

        let response: ProgramJsonEmitResponse = ProgramJsonEmitRequest::build(&groups)
            .expect("emit request")
            .execute();

        let _ = std::fs::remove_file(&source_path);
        let _ = std::fs::remove_file(&out_path);

        assert_eq!(response.out_path, out_path.to_string_lossy());
        assert!(response.result.is_ok());
    }
}
