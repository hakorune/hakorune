pub const OPTION_SOME_NULLISH_TAG: &str = "[freeze:contract][option/some_nullish]";

pub fn requires_non_nullish_payload(enum_name: &str, variant_name: &str) -> bool {
    enum_name == "Option" && variant_name == "Some"
}

pub fn nullish_payload_error(surface: &str) -> String {
    format!(
        "{} Option::Some payload must not be null or void ({})",
        OPTION_SOME_NULLISH_TAG, surface
    )
}
