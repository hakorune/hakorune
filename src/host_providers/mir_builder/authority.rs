pub(super) fn source_to_mir_json(source_text: &str) -> Result<String, String> {
    super::user_box_decls::source_to_mir_json_with_user_box_decls(source_text)
}
