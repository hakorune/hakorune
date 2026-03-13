use super::FAILFAST_TAG;

#[cfg(test)]
pub(super) fn source_to_program_and_mir_json(
    source_text: &str,
) -> Result<(String, String), String> {
    let program_json = source_to_program_json_for_current_authority(source_text)?;
    let mir_json = super::lowering::program_json_to_mir_json(&program_json)?;
    Ok((program_json, mir_json))
}

pub(super) fn source_to_mir_json(source_text: &str) -> Result<String, String> {
    let program_json = source_to_program_json_for_current_authority(source_text)?;
    super::user_box_decls::program_json_to_mir_json_with_user_box_decls(&program_json)
}

fn source_to_program_json_for_current_authority(source_text: &str) -> Result<String, String> {
    crate::stage1::program_json_v0::emit_program_json_v0_for_strict_authority_source(source_text)
        .map_err(|e| format!("{FAILFAST_TAG} {}", e))
}
