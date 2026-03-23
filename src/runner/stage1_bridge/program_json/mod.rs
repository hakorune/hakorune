//! Stage1 bridge Program(JSON v0) emit facade.
//!
//! Keep the bridge root focused on routing while `program_json/orchestrator.rs`
//! owns source-text read, bridge-local read->emit->write orchestration, and
//! writeback policy for the future-retire lane. Source-path precedence stays in
//! the bridge-entry owner (`program_json_entry/request.rs`).

mod orchestrator;
mod payload;
mod read_input;
mod writeback;

pub(super) fn emit_program_json_v0(source_path: &str, out_path: &str) -> Result<(), String> {
    orchestrator::emit_program_json_v0(source_path, out_path)
}
