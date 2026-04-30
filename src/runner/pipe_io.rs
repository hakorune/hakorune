/*!
 * Runner Pipe I/O helpers — JSON v0 handling
 *
 * Extracted from runner/mod.rs to keep the main runner slimmer.
 * Handles:
 *  - Reading JSON v0 from stdin or file
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

        // Unified: delegate to JSON artifact executor.
        let rc = crate::runner::core_executor::execute_json_artifact(self, &json);
        std::process::exit(rc);
    }
}
