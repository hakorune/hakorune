pub(super) fn emit_program_json_v0_for_stage1_bridge_emit_program_json(
    source_text: &str,
) -> Result<String, String> {
    super::authority::source_to_program_json_v0_strict(source_text)
        .map_err(|error_text| format!("emit-program-json-v0: {}", error_text))
}
