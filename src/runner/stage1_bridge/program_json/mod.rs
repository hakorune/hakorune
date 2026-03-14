//! Stage1 bridge Program(JSON v0) emit facade.
//!
//! Keep the bridge root focused on routing while `program_json/` owns
//! source-text read, bridge-local read->emit->write orchestration, and
//! writeback policy for the future-retire lane. Source-path precedence
//! stays in the bridge-entry owner (`program_json_entry/request.rs`).

mod read_input;
mod payload;
mod writeback;

pub(super) fn emit_program_json_v0(source_path: &str, out_path: &str) -> Result<(), String> {
    let out = load_program_json_output(source_path)?;
    writeback::write_program_json_output(out_path, &out)
}

fn load_program_json_output(source_path: &str) -> Result<String, String> {
    let code = read_input::read_source_text(source_path)?;
    payload::emit_program_json_payload(&code)
}

#[cfg(test)]
mod tests {
    use super::{emit_program_json_v0, load_program_json_output};

    #[test]
    fn load_program_json_output_preserves_read_error_prefix() {
        let unique = format!(
            "/tmp/hakorune-stage1-bridge-program-json-missing-{}-{}.hako",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("unix epoch")
                .as_nanos()
        );
        let error = load_program_json_output(&unique).expect_err("missing path must fail");
        assert!(error.starts_with(&format!(
            "emit-program-json-v0 read error: {}",
            unique
        )));
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
}
