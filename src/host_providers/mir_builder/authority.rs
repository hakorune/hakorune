use super::FAILFAST_TAG;

pub(super) fn source_to_mir_json(source_text: &str) -> Result<String, String> {
    let program_json =
        crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(
            source_text,
        )
        .map_err(|e| format!("{FAILFAST_TAG} {}", e))?;
    super::user_box_decls::program_json_to_mir_json_with_user_box_decls(&program_json)
}
