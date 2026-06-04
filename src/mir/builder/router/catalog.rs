//! Catalog-backed Unified value-path rows for RouterPolicy.
//!
//! This module owns method-family membership only. `policy.rs` owns the route
//! decision and logging.

pub(super) fn unified_value_path_reason(
    box_name: &str,
    method: &str,
    arity: usize,
) -> Option<&'static str> {
    match box_name {
        "StringBox" if is_stringbox_unified_value_path(method, arity) => {
            Some("stringbox_value_path")
        }
        "ArrayBox" if is_arraybox_unified_value_path(method, arity) => Some("arraybox_value_path"),
        "MapBox" if is_mapbox_unified_value_path(method, arity) => Some("mapbox_value_path"),
        _ => None,
    }
}

pub(super) fn is_core_box(box_name: &str) -> bool {
    matches!(box_name, "StringBox" | "ArrayBox" | "MapBox")
}

#[inline]
fn is_stringbox_unified_value_path(method: &str, arity: usize) -> bool {
    matches!(
        crate::boxes::basic::StringMethodId::from_name_and_arity(method, arity),
        Some(
            crate::boxes::basic::StringMethodId::Length
                | crate::boxes::basic::StringMethodId::SubstringFrom
                | crate::boxes::basic::StringMethodId::Substring
                | crate::boxes::basic::StringMethodId::Concat
                | crate::boxes::basic::StringMethodId::Trim
                | crate::boxes::basic::StringMethodId::Upper
                | crate::boxes::basic::StringMethodId::Lower
                | crate::boxes::basic::StringMethodId::Contains
                | crate::boxes::basic::StringMethodId::StartsWith
                | crate::boxes::basic::StringMethodId::LastIndexOf
                | crate::boxes::basic::StringMethodId::LastIndexOfFrom
                | crate::boxes::basic::StringMethodId::Replace
                | crate::boxes::basic::StringMethodId::IndexOf
                | crate::boxes::basic::StringMethodId::IndexOfFrom
        )
    )
}

#[inline]
fn is_arraybox_unified_value_path(method: &str, arity: usize) -> bool {
    matches!(
        crate::boxes::array::ArrayMethodId::from_name_and_arity(method, arity),
        Some(
            crate::boxes::array::ArrayMethodId::Length
                | crate::boxes::array::ArrayMethodId::Push
                | crate::boxes::array::ArrayMethodId::Slice
                | crate::boxes::array::ArrayMethodId::Get
                | crate::boxes::array::ArrayMethodId::Pop
                | crate::boxes::array::ArrayMethodId::Clear
                | crate::boxes::array::ArrayMethodId::Contains
                | crate::boxes::array::ArrayMethodId::IndexOf
                | crate::boxes::array::ArrayMethodId::Join
                | crate::boxes::array::ArrayMethodId::Sort
                | crate::boxes::array::ArrayMethodId::Reverse
                | crate::boxes::array::ArrayMethodId::Set
                | crate::boxes::array::ArrayMethodId::Remove
                | crate::boxes::array::ArrayMethodId::Insert
        )
    )
}

#[inline]
fn is_mapbox_unified_value_path(method: &str, arity: usize) -> bool {
    matches!(
        crate::boxes::MapMethodId::from_name_and_arity(method, arity),
        Some(
            crate::boxes::MapMethodId::Size
                | crate::boxes::MapMethodId::Len
                | crate::boxes::MapMethodId::Has
                | crate::boxes::MapMethodId::Get
                | crate::boxes::MapMethodId::Set
                | crate::boxes::MapMethodId::Delete
                | crate::boxes::MapMethodId::Keys
                | crate::boxes::MapMethodId::Values
                | crate::boxes::MapMethodId::Clear
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn accepts(box_name: &str, method: &str, arity: usize) {
        assert!(
            unified_value_path_reason(box_name, method, arity).is_some(),
            "{box_name}.{method}/{arity} should be catalog-backed"
        );
    }

    fn rejects(box_name: &str, method: &str, arity: usize) {
        assert!(
            unified_value_path_reason(box_name, method, arity).is_none(),
            "{box_name}.{method}/{arity} should not be catalog-backed"
        );
    }

    #[test]
    fn string_catalog_rows_accept_known_value_paths() {
        for (method, arity) in [
            ("length", 0),
            ("len", 0),
            ("size", 0),
            ("substring", 2),
            ("substr", 2),
            ("concat", 1),
            ("trim", 0),
            ("toUpper", 0),
            ("toLower", 0),
            ("toUpperCase", 0),
            ("toLowerCase", 0),
            ("contains", 1),
            ("startsWith", 1),
            ("lastIndexOf", 1),
            ("lastIndexOf", 2),
            ("replace", 2),
            ("indexOf", 1),
            ("indexOf", 2),
            ("find", 1),
            ("find", 2),
        ] {
            accepts("StringBox", method, arity);
        }
    }

    #[test]
    fn array_catalog_rows_accept_known_value_paths() {
        for (method, arity) in [
            ("length", 0),
            ("size", 0),
            ("len", 0),
            ("push", 1),
            ("slice", 2),
            ("get", 1),
            ("pop", 0),
            ("set", 2),
            ("clear", 0),
            ("contains", 1),
            ("indexOf", 1),
            ("join", 1),
            ("reverse", 0),
            ("sort", 0),
            ("remove", 1),
            ("insert", 2),
        ] {
            accepts("ArrayBox", method, arity);
        }
    }

    #[test]
    fn map_catalog_rows_accept_known_value_paths() {
        for (method, arity) in [
            ("size", 0),
            ("len", 0),
            ("length", 0),
            ("has", 1),
            ("get", 1),
            ("set", 2),
            ("delete", 1),
            ("remove", 1),
            ("values", 0),
            ("keys", 0),
            ("clear", 0),
        ] {
            accepts("MapBox", method, arity);
        }
    }

    #[test]
    fn non_catalog_core_box_rows_reject() {
        for (box_name, method, arity) in [
            ("StringBox", "length", 1),
            ("StringBox", "concat", 0),
            ("StringBox", "trim", 1),
            ("StringBox", "contains", 0),
            ("StringBox", "startsWith", 0),
            ("StringBox", "startsWith", 2),
            ("StringBox", "lastIndexOf", 3),
            ("StringBox", "replace", 1),
            ("StringBox", "indexOf", 0),
            ("StringBox", "indexOf", 3),
            ("StringBox", "find", 0),
            ("StringBox", "find", 3),
            ("ArrayBox", "length", 1),
            ("ArrayBox", "get", 0),
            ("ArrayBox", "get", 2),
            ("ArrayBox", "set", 1),
            ("ArrayBox", "set", 3),
            ("ArrayBox", "push", 0),
            ("ArrayBox", "push", 2),
            ("ArrayBox", "pop", 1),
            ("ArrayBox", "clear", 1),
            ("ArrayBox", "contains", 0),
            ("ArrayBox", "contains", 2),
            ("ArrayBox", "indexOf", 0),
            ("ArrayBox", "indexOf", 2),
            ("ArrayBox", "join", 0),
            ("ArrayBox", "join", 2),
            ("ArrayBox", "reverse", 1),
            ("ArrayBox", "sort", 1),
            ("ArrayBox", "slice", 1),
            ("ArrayBox", "slice", 3),
            ("ArrayBox", "remove", 0),
            ("ArrayBox", "remove", 2),
            ("ArrayBox", "insert", 1),
            ("ArrayBox", "insert", 3),
            ("MapBox", "size", 1),
            ("MapBox", "len", 1),
            ("MapBox", "length", 1),
            ("MapBox", "has", 0),
            ("MapBox", "has", 2),
            ("MapBox", "get", 0),
            ("MapBox", "get", 2),
            ("MapBox", "set", 1),
            ("MapBox", "set", 3),
            ("MapBox", "delete", 0),
            ("MapBox", "delete", 2),
            ("MapBox", "remove", 0),
            ("MapBox", "remove", 2),
            ("MapBox", "keys", 1),
            ("MapBox", "values", 1),
            ("MapBox", "clear", 1),
        ] {
            rejects(box_name, method, arity);
        }
    }
}
