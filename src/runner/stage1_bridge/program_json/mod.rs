//! Stage1 bridge Program(JSON v0) emit facade.
//!
//! Keep the bridge root focused on routing while `program_json/` owns
//! source-text read, bridge-local read->emit->write orchestration, and
//! writeback policy for the future-retire lane. Source-path precedence
//! stays in the bridge-entry owner (`program_json_entry/request.rs`).

mod payload;
mod read_input;
mod writeback;

pub(super) fn emit_program_json_v0(source_path: &str, out_path: &str) -> Result<(), String> {
    ProgramJsonOutput::from_source_path(source_path)?.emit_to_path(out_path)
}

#[derive(Debug)]
struct ProgramJsonOutput {
    payload: String,
}

impl ProgramJsonOutput {
    fn from_source_path(source_path: &str) -> Result<Self, String> {
        Self::from_source_text(&read_input::read_source_text(source_path)?)
    }

    fn from_source_text(source_text: &str) -> Result<Self, String> {
        Ok(Self {
            payload: payload::emit_program_json_payload(source_text)?,
        })
    }

    #[cfg(test)]
    fn into_payload(self) -> String {
        self.payload
    }

    fn emit_to_path(self, out_path: &str) -> Result<(), String> {
        writeback::write_program_json_output(out_path, &self.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::{emit_program_json_v0, ProgramJsonOutput};

    #[test]
    fn program_json_output_preserves_read_error_prefix() {
        let unique = format!(
            "/tmp/hakorune-stage1-bridge-program-json-missing-{}-{}.hako",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let error =
            ProgramJsonOutput::from_source_path(&unique).expect_err("missing path must fail");
        assert!(error.starts_with(&format!("emit-program-json-v0 read error: {}", unique)));
    }

    #[test]
    fn emit_program_json_v0_round_trips_source_file_into_program_json_file() {
        let unique = format!(
            "hakorune-stage1-bridge-mod-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let source_path = std::env::temp_dir().join(format!("{unique}.hako"));
        let out_path = std::env::temp_dir().join(format!("{unique}.json"));
        let source_path_str = source_path.to_string_lossy().into_owned();
        let out_path_str = out_path.to_string_lossy().into_owned();

        std::fs::write(
            &source_path,
            include_str!("../../../../lang/src/runner/stage1_cli_env.hako"),
        )
        .expect("write temp source");

        emit_program_json_v0(&source_path_str, &out_path_str).expect("emit program json");
        let written = std::fs::read_to_string(&out_path).expect("read written program json");

        let _ = std::fs::remove_file(&source_path);
        let _ = std::fs::remove_file(&out_path);

        assert!(written.contains("\"kind\":\"Program\""));
        assert!(written.contains("\"version\":0"));
    }

    #[test]
    fn program_json_output_reads_payload_from_source_path() {
        let unique = format!(
            "hakorune-stage1-bridge-payload-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let source_path = std::env::temp_dir().join(format!("{unique}.hako"));
        let source_path_str = source_path.to_string_lossy().into_owned();

        std::fs::write(
            &source_path,
            include_str!("../../../../lang/src/runner/stage1_cli_env.hako"),
        )
        .expect("write temp source");

        let out = ProgramJsonOutput::from_source_path(&source_path_str)
            .expect("program json output")
            .into_payload();

        let _ = std::fs::remove_file(&source_path);

        assert!(out.contains("\"kind\":\"Program\""));
        assert!(out.contains("\"version\":0"));
    }

    #[test]
    fn program_json_output_builds_payload_from_source_text() {
        let out = ProgramJsonOutput::from_source_text(include_str!(
            "../../../../lang/src/runner/stage1_cli_env.hako"
        ))
        .expect("program json output")
        .into_payload();

        assert!(out.contains("\"kind\":\"Program\""));
        assert!(out.contains("\"version\":0"));
    }
}
