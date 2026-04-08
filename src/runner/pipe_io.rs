/*!
 * Runner Pipe I/O helpers — JSON v0 handling
 *
 * Extracted from runner/mod.rs to keep the main runner slimmer.
 * Handles:
 *  - Reading JSON v0 from stdin or file
 *  - Optional Program(JSON v0) -> MIR(JSON) conversion
 *  - Unified execution via CoreExecutor
 */

use super::*;
use std::io::Write;

fn emit_program_json_to_mir_json_file(
    program_json: &str,
    out_path: &std::path::Path,
) -> Result<(), String> {
    let mir_json =
        crate::host_providers::mir_builder::program_json_to_mir_json_with_user_box_decls(
            program_json,
        )?;
    let file = std::fs::File::create(out_path).map_err(|e| format!("write mir json: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);
    writer
        .write_all(mir_json.as_bytes())
        .map_err(|e| format!("write mir json: {}", e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| format!("write mir json: {}", e))?;
    writer.flush().map_err(|e| format!("write mir json: {}", e))
}

impl NyashRunner {
    /// Try to handle `--ny-parser-pipe` / `--json-file` flow.
    /// Returns true if the request was handled (program should return early).
    pub(super) fn try_run_json_v0_pipe(&self) -> bool {
        let groups = self.config.as_groups();
        if !(groups.parser.ny_parser_pipe || groups.parser.json_file.is_some()) {
            return false;
        }
        let json = if let Some(path) = &groups.parser.json_file {
            match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ json-file read error: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            use std::io::Read;
            let mut buf = String::new();
            if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
                eprintln!("❌ stdin read error: {}", e);
                std::process::exit(1);
            }
            buf
        };
        // Optional: convert Program(JSON v0) → MIR(JSON) and exit when requested
        if let Some(out) = &groups.emit.program_json_to_mir {
            crate::runtime::deprecations::warn_program_json_to_mir_cli_once();
            let p = std::path::Path::new(out);
            if let Err(e) = emit_program_json_to_mir_json_file(&json, p) {
                eprintln!("❌ Program→MIR emit error: {}", e);
                std::process::exit(1);
            }
            std::process::exit(0);
        }

        // Unified: delegate to JSON artifact executor.
        let rc = crate::runner::core_executor::execute_json_artifact(self, &json);
        std::process::exit(rc);
    }
}

#[cfg(test)]
mod tests {
    use super::emit_program_json_to_mir_json_file;

    fn temp_output_path(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "hakorune-pipe-io-{}-{}-{}.json",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ))
    }

    #[test]
    fn program_json_to_mir_file_keeps_user_box_decls() {
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "defs": [
                {
                    "box": "HelperBox",
                    "name": "helper",
                    "params": [],
                    "body": {"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":1}}]}
                }
            ],
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Int", "value": 42}
                }
            ]
        }"#;
        let out = temp_output_path("program-json-to-mir");
        emit_program_json_to_mir_json_file(program_json, &out).expect("emit mir json");
        let mir_json = std::fs::read_to_string(&out).expect("read mir json");
        let _ = std::fs::remove_file(&out);

        assert!(mir_json.contains("\"user_box_decls\""));
        assert!(mir_json.contains("\"name\":\"Main\""));
        assert!(mir_json.contains("\"name\":\"HelperBox\""));
    }

    #[test]
    fn program_json_to_mir_file_prefers_explicit_user_box_decls() {
        let program_json = r#"{
            "version": 0,
            "kind": "Program",
            "user_box_decls": [
                {"name":"Main","fields":[]},
                {
                    "name":"PipeBox",
                    "fields":["slot"],
                    "field_decls":[
                        {"name":"slot","declared_type":"IntegerBox","is_weak":false}
                    ]
                }
            ],
            "defs": [
                {
                    "box": "CompatOnlyBox",
                    "name": "helper",
                    "params": [],
                    "body": {"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Int","value":1}}]}
                }
            ],
            "body": [
                {
                    "type": "Return",
                    "expr": {"type": "Int", "value": 42}
                }
            ]
        }"#;
        let out = temp_output_path("program-json-to-mir-explicit");
        emit_program_json_to_mir_json_file(program_json, &out).expect("emit mir json");
        let mir_json = std::fs::read_to_string(&out).expect("read mir json");
        let _ = std::fs::remove_file(&out);

        assert!(mir_json.contains("\"user_box_decls\""));
        assert!(mir_json.contains("\"name\":\"PipeBox\""));
        assert!(mir_json.contains("\"fields\":[\"slot\"]"));
        assert!(mir_json.contains("\"field_decls\":[{\"declared_type\":\"IntegerBox\",\"is_weak\":false,\"name\":\"slot\"}]"));
        assert!(!mir_json.contains("\"name\":\"CompatOnlyBox\""));
    }
}
