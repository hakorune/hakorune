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
            match super::json_v0_bridge::parse_json_v0_to_module(&json) {
                Ok(module) => {
                    let p = std::path::Path::new(out);
                    if let Err(e) = super::mir_json_emit::emit_mir_json_for_harness_bin(&module, p)
                    {
                        eprintln!("❌ Program→MIR emit error: {}", e);
                        std::process::exit(1);
                    }
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("❌ Program(JSON v0) parse error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Unified: delegate to CoreExecutor (boxed)
        let rc = crate::runner::core_executor::run_json_v0(self, &json);
        std::process::exit(rc);
    }
}
