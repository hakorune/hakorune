//! Stage1 bridge Program(JSON v0) emit facade.
//!
//! Keep `mod.rs` focused on routing while `program_json/` owns
//! source-path precedence, source-text read, bridge-local payload emission,
//! and writeback policy for the future-retire lane.

use crate::cli::CliGroups;
mod emit_payload;
mod read_input;
mod source;
mod writeback;

pub(super) fn emit_program_json_v0(groups: &CliGroups, out_path: &str) -> Result<(), String> {
    let source = source::resolve_source_path(groups)?;
    let code = read_input::read_source_text(&source)?;
    let out = emit_payload::emit_program_json_payload(&code)?;
    writeback::write_program_json_output(out_path, &out)
}
