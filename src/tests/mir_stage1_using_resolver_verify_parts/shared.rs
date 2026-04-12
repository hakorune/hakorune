pub(crate) fn ensure_stage3_env() {
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_PARSER_ALLOW_SEMICOLON", "1");
    std::env::set_var("NYASH_ENABLE_USING", "1");
    std::env::set_var("HAKO_ENABLE_USING", "1");
}
