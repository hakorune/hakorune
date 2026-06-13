use super::*;

// ===== Condition Context Tests =====

#[test]
fn test_stringutils_character_classification_in_condition() {
    // Pure boolean character classification methods should be allowed
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_whitespace"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_digit"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_hex_digit"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_alpha"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_alphanumeric"
    ));
}

#[test]
fn test_stringutils_validation_in_condition() {
    // Pure boolean validation methods should be allowed
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_integer"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_empty_or_whitespace"
    ));
}

#[test]
fn test_stringutils_matching_in_condition() {
    // Pure boolean matching methods should be allowed
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "starts_with"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "ends_with"
    ));
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "contains"
    ));
}

#[test]
fn test_stringutils_string_functions_not_in_condition() {
    // String-returning functions should NOT be allowed in condition
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "trim"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "trim_start"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "trim_end"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "to_upper"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "to_lower"
    ));
}

#[test]
fn test_stringutils_search_not_in_condition() {
    // Integer-returning search functions should NOT be allowed in condition
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "index_of"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "last_index_of"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "index_of_string"
    ));
}

#[test]
fn test_unknown_static_box_in_condition() {
    // Unknown static boxes should fail-fast
    assert!(!UserMethodPolicy::allowed_in_condition(
        "UnknownBox",
        "some_method"
    ));
    assert!(!UserMethodPolicy::allowed_in_condition("MathUtils", "abs"));
}

// ===== Init Context Tests =====

#[test]
fn test_stringutils_all_pure_methods_in_init() {
    // All pure methods should be allowed in init (more permissive than condition)
    // Character classification
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "is_whitespace"
    ));
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "is_digit"));

    // String manipulation
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim"));
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "trim_start"
    ));
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_end"));
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "to_upper"));
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "to_lower"));

    // String search
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "index_of"));
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "last_index_of"
    ));
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "index_of_string"
    ));

    // Numeric parsing
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "parse_integer"
    ));
    assert!(UserMethodPolicy::allowed_in_init(
        "StringUtils",
        "parse_float"
    ));
}

#[test]
fn test_unknown_static_box_in_init() {
    // Unknown static boxes should fail-fast
    assert!(!UserMethodPolicy::allowed_in_init(
        "UnknownBox",
        "some_method"
    ));
    assert!(!UserMethodPolicy::allowed_in_init("MathUtils", "sqrt"));
}

// ===== Real-World Pattern Tests =====

#[test]
fn test_trim_end_pattern() {
    // Phase 252 P0: StringUtils.trim_end/1 pattern
    // loop(i >= 0) { if not this.is_whitespace(s.substring(i, i + 1)) { break } ... }

    // is_whitespace should be allowed in condition (boolean check)
    assert!(UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "is_whitespace"
    ));

    // trim_end itself should NOT be allowed in condition (string function)
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "trim_end"
    ));

    // But trim_end should be allowed in init
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_end"));
}

#[test]
fn test_index_of_pattern() {
    // Case: local pos = this.index_of(s, ch)
    // index_of returns integer (-1 or index), not boolean

    // Should NOT be allowed in condition
    assert!(!UserMethodPolicy::allowed_in_condition(
        "StringUtils",
        "index_of"
    ));

    // But should be allowed in init
    assert!(UserMethodPolicy::allowed_in_init("StringUtils", "index_of"));
}
