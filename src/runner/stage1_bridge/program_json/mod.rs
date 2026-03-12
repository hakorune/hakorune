//! Stage1 bridge Program(JSON v0) emit facade.
//!
//! Keep the bridge root focused on routing while `program_json/` owns
//! source-text read, bridge-local payload emission, and writeback policy
//! for the future-retire lane. Source-path precedence stays in the
//! bridge-entry owner (`program_json_entry/request.rs`).

mod emit_payload;
mod read_input;
mod writeback;

pub(super) fn emit_program_json_v0(source_path: &str, out_path: &str) -> Result<(), String> {
    let code = read_input::read_source_text(source_path)?;
    let out = emit_payload::emit_program_json_payload(&code)?;
    writeback::write_program_json_output(out_path, &out)
}
