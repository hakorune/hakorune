/*!
 * String corridor helper-name vocabulary.
 *
 * This module is a quarantine layer for legacy/helper/runtime-export names.
 * It does not infer corridor facts and it does not inspect MIR shape. Callers
 * must still decide whether a matched name is legal for their own demand.
 */

pub(crate) fn is_stringish_box_name(box_name: &str) -> bool {
    matches!(box_name, "StringBox" | "String" | "__str")
        || box_name.ends_with("StringBox")
        || box_name.ends_with("String")
}

pub(crate) fn is_len_method_name(method: &str) -> bool {
    matches!(method, "length" | "len")
}

pub(crate) fn is_slice_method_name(method: &str) -> bool {
    matches!(method, "substring" | "slice")
}

pub(crate) fn is_lowered_len_global(name: &str) -> bool {
    matches!(name, "str.len" | "__str.len")
}

pub(crate) fn is_runtime_len_export(name: &str) -> bool {
    matches!(
        name,
        "nyash.string.len_h"
            | "nyash.string.substring_len_hii"
            | "nyash.string.length_si"
            | "nyrt_string_length"
            | "nyrt.string.length"
    )
}

pub(crate) fn is_runtime_len_handle_export(name: &str) -> bool {
    name == "nyash.string.len_h"
}

pub(crate) fn is_runtime_slice_export(name: &str) -> bool {
    matches!(
        name,
        "nyash.string.substring_hii"
            | "nyash.string.substring_concat_hhii"
            | "nyash.string.substring_concat3_hhhii"
    )
}

pub(crate) fn is_runtime_substring_export(name: &str) -> bool {
    name == "nyash.string.substring_hii"
}

pub(crate) fn is_runtime_substring_len_export(name: &str) -> bool {
    name == "nyash.string.substring_len_hii"
}

pub(crate) fn is_runtime_substring_concat3_export(name: &str) -> bool {
    name == "nyash.string.substring_concat3_hhhii"
}

pub(crate) fn is_runtime_concat3_export(name: &str) -> bool {
    name == "nyash.string.concat3_hhh"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_keep_legacy_stringish_box_aliases() {
        assert!(is_stringish_box_name("StringBox"));
        assert!(is_stringish_box_name("__str"));
        assert!(is_stringish_box_name("MyStringBox"));
        assert!(!is_stringish_box_name("RuntimeDataBox"));
    }

    #[test]
    fn runtime_exports_split_len_from_slice() {
        assert!(is_runtime_len_export("nyash.string.substring_len_hii"));
        assert!(is_runtime_slice_export("nyash.string.substring_hii"));
        assert!(!is_runtime_len_export("nyash.string.substring_hii"));
        assert!(!is_runtime_slice_export("nyash.string.substring_len_hii"));
    }
}
