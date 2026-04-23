use crate::mir::definitions::call_unified::TypeCertainty;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Unified,
    BoxCall,
}

/// Decide routing policy for a method call (Unified vs BoxCall) without changing behavior.
/// Rules (behavior-preserving):
/// - UnknownBox → BoxCall (unified is unstable for unknown receivers)
/// - Core boxes: StringBox/ArrayBox/MapBox → BoxCall unless a catalog-backed
///   method family has an explicit Unified value-path proof
/// - User boxes: names not ending with "Box" → BoxCall
/// - Otherwise Unified
pub fn choose_route(box_name: &str, method: &str, certainty: TypeCertainty, arity: usize) -> Route {
    let mut reason = "unified";
    let route = if box_name == "UnknownBox" {
        reason = "unknown_recv";
        Route::BoxCall
    } else if is_stringbox_unified_value_path(method, arity) && box_name == "StringBox" {
        reason = "stringbox_value_path";
        Route::Unified
    } else if is_arraybox_unified_value_path(method, arity) && box_name == "ArrayBox" {
        reason = "arraybox_value_path";
        Route::Unified
    } else if is_mapbox_unified_value_path(method, arity) && box_name == "MapBox" {
        reason = "mapbox_value_path";
        Route::Unified
    } else if matches!(box_name, "StringBox" | "ArrayBox" | "MapBox") {
        reason = "core_box";
        Route::BoxCall
    } else if !box_name.ends_with("Box") {
        reason = "user_instance";
        Route::BoxCall
    } else {
        Route::Unified
    };

    if router_trace_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[router] route={:?} reason={} recv={} method={} arity={} certainty={:?}",
            route, reason, box_name, method, arity, certainty
        ));
    }

    route
}

#[inline]
fn is_stringbox_unified_value_path(method: &str, arity: usize) -> bool {
    matches!(
        crate::boxes::basic::StringMethodId::from_name_and_arity(method, arity),
            Some(
                crate::boxes::basic::StringMethodId::Length
                    | crate::boxes::basic::StringMethodId::Substring
                    | crate::boxes::basic::StringMethodId::Concat
                    | crate::boxes::basic::StringMethodId::Trim
                    | crate::boxes::basic::StringMethodId::Upper
                    | crate::boxes::basic::StringMethodId::Lower
                    | crate::boxes::basic::StringMethodId::Contains
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

#[inline]
fn router_trace_enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(crate::config::env::builder_router_trace)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn route(box_name: &str, method: &str, arity: usize) -> Route {
        choose_route(box_name, method, TypeCertainty::Known, arity)
    }

    #[test]
    fn unknown_and_user_instance_stay_boxcall() {
        assert_eq!(route("UnknownBox", "length", 0), Route::BoxCall);
        assert_eq!(route("UserThing", "length", 0), Route::BoxCall);
    }

    #[test]
    fn string_length_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "length", 0), Route::Unified);
        assert_eq!(route("StringBox", "len", 0), Route::Unified);
        assert_eq!(route("StringBox", "size", 0), Route::Unified);
    }

    #[test]
    fn string_substring_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "substring", 2), Route::Unified);
        assert_eq!(route("StringBox", "substr", 2), Route::Unified);
    }

    #[test]
    fn string_concat_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "concat", 1), Route::Unified);
    }

    #[test]
    fn string_trim_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "trim", 0), Route::Unified);
    }

    #[test]
    fn string_case_conversion_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "toUpper", 0), Route::Unified);
        assert_eq!(route("StringBox", "toLower", 0), Route::Unified);
        assert_eq!(route("StringBox", "toUpperCase", 0), Route::Unified);
        assert_eq!(route("StringBox", "toLowerCase", 0), Route::Unified);
    }

    #[test]
    fn string_contains_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "contains", 1), Route::Unified);
    }

    #[test]
    fn string_last_index_of_one_arg_uses_unified_value_path() {
        assert_eq!(route("StringBox", "lastIndexOf", 1), Route::Unified);
    }

    #[test]
    fn string_last_index_of_two_arg_uses_unified_value_path() {
        assert_eq!(route("StringBox", "lastIndexOf", 2), Route::Unified);
    }

    #[test]
    fn string_replace_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "replace", 2), Route::Unified);
    }

    #[test]
    fn string_index_of_family_uses_unified_value_path() {
        assert_eq!(route("StringBox", "indexOf", 1), Route::Unified);
        assert_eq!(route("StringBox", "indexOf", 2), Route::Unified);
        assert_eq!(route("StringBox", "find", 1), Route::Unified);
        assert_eq!(route("StringBox", "find", 2), Route::Unified);
    }

    #[test]
    fn array_length_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "length", 0), Route::Unified);
        assert_eq!(route("ArrayBox", "size", 0), Route::Unified);
        assert_eq!(route("ArrayBox", "len", 0), Route::Unified);
    }

    #[test]
    fn array_push_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "push", 1), Route::Unified);
    }

    #[test]
    fn array_slice_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "slice", 2), Route::Unified);
    }

    #[test]
    fn array_get_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "get", 1), Route::Unified);
    }

    #[test]
    fn array_pop_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "pop", 0), Route::Unified);
    }

    #[test]
    fn array_set_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "set", 2), Route::Unified);
    }

    #[test]
    fn array_remove_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "remove", 1), Route::Unified);
    }

    #[test]
    fn array_insert_family_uses_unified_value_path() {
        assert_eq!(route("ArrayBox", "insert", 2), Route::Unified);
    }

    #[test]
    fn map_size_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "size", 0), Route::Unified);
    }

    #[test]
    fn map_len_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "len", 0), Route::Unified);
    }

    #[test]
    fn map_length_alias_uses_unified_value_path() {
        assert_eq!(route("MapBox", "length", 0), Route::Unified);
    }

    #[test]
    fn map_has_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "has", 1), Route::Unified);
    }

    #[test]
    fn map_get_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "get", 1), Route::Unified);
    }

    #[test]
    fn map_set_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "set", 2), Route::Unified);
    }

    #[test]
    fn map_delete_remove_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "delete", 1), Route::Unified);
        assert_eq!(route("MapBox", "remove", 1), Route::Unified);
    }

    #[test]
    fn map_values_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "values", 0), Route::Unified);
    }

    #[test]
    fn map_keys_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "keys", 0), Route::Unified);
    }

    #[test]
    fn map_clear_row_uses_unified_value_path() {
        assert_eq!(route("MapBox", "clear", 0), Route::Unified);
    }

    #[test]
    fn non_allowlisted_corebox_methods_stay_boxcall() {
        assert_eq!(route("StringBox", "length", 1), Route::BoxCall);
        assert_eq!(route("StringBox", "substring", 1), Route::BoxCall);
        assert_eq!(route("StringBox", "concat", 0), Route::BoxCall);
        assert_eq!(route("StringBox", "trim", 1), Route::BoxCall);
        assert_eq!(route("StringBox", "contains", 0), Route::BoxCall);
        assert_eq!(route("StringBox", "lastIndexOf", 3), Route::BoxCall);
        assert_eq!(route("StringBox", "replace", 1), Route::BoxCall);
        assert_eq!(route("StringBox", "indexOf", 0), Route::BoxCall);
        assert_eq!(route("StringBox", "indexOf", 3), Route::BoxCall);
        assert_eq!(route("StringBox", "find", 0), Route::BoxCall);
        assert_eq!(route("StringBox", "find", 3), Route::BoxCall);
        assert_eq!(route("ArrayBox", "length", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "get", 0), Route::BoxCall);
        assert_eq!(route("ArrayBox", "get", 2), Route::BoxCall);
        assert_eq!(route("ArrayBox", "set", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "set", 3), Route::BoxCall);
        assert_eq!(route("ArrayBox", "push", 0), Route::BoxCall);
        assert_eq!(route("ArrayBox", "push", 2), Route::BoxCall);
        assert_eq!(route("ArrayBox", "pop", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "slice", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "slice", 3), Route::BoxCall);
        assert_eq!(route("ArrayBox", "remove", 0), Route::BoxCall);
        assert_eq!(route("ArrayBox", "remove", 2), Route::BoxCall);
        assert_eq!(route("ArrayBox", "insert", 1), Route::BoxCall);
        assert_eq!(route("ArrayBox", "insert", 3), Route::BoxCall);
        assert_eq!(route("MapBox", "size", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "len", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "length", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "has", 0), Route::BoxCall);
        assert_eq!(route("MapBox", "has", 2), Route::BoxCall);
        assert_eq!(route("MapBox", "get", 0), Route::BoxCall);
        assert_eq!(route("MapBox", "get", 2), Route::BoxCall);
        assert_eq!(route("MapBox", "set", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "set", 3), Route::BoxCall);
        assert_eq!(route("MapBox", "delete", 0), Route::BoxCall);
        assert_eq!(route("MapBox", "delete", 2), Route::BoxCall);
        assert_eq!(route("MapBox", "remove", 0), Route::BoxCall);
        assert_eq!(route("MapBox", "remove", 2), Route::BoxCall);
        assert_eq!(route("MapBox", "keys", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "values", 1), Route::BoxCall);
        assert_eq!(route("MapBox", "clear", 1), Route::BoxCall);
    }

    #[test]
    fn non_core_box_names_keep_unified_route() {
        assert_eq!(route("FileBox", "read", 0), Route::Unified);
        assert_eq!(route("ConsoleBox", "log", 1), Route::Unified);
    }
}
