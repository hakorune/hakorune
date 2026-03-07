//! Phase 252: User-Defined Method Policy Box
//!
//! This box provides a Single Source of Truth (SSOT) for determining whether
//! user-defined static box methods are allowed in JoinIR contexts.
//!
//! ## Design Philosophy
//!
//! **Box-First Design**: UserMethodPolicy is a single-responsibility box that
//! answers one question: "Can this static box method be safely lowered to JoinIR?"
//!
//! **Metadata-Driven**: Uses a policy table to determine allowed methods.
//! NO method name hardcoding in lowering logic - all decisions made here.
//!
//! **Fail-Fast**: If a method is not in the policy table, immediately returns false.
//! No silent fallbacks or guessing.
//!
//! **Future Extension**: This SSOT can be moved to .hako annotations or nyash.toml
//! in the future without breaking lowering logic.
//!
//! ## Supported Static Boxes
//!
//! - **StringUtils**: String utility functions (trim, character checks, etc.)
//!
//! ## Example Usage
//!
//! ```ignore
//! // Check if StringUtils.is_whitespace is allowed in condition
//! if UserMethodPolicy::allowed_in_condition("StringUtils", "is_whitespace") {
//!     // Lower this.is_whitespace(...) to JoinIR
//! }
//! ```

/// Phase 252: User-Defined Method Policy Box
///
/// Provides metadata for user-defined static box methods to determine
/// their eligibility for JoinIR lowering in different contexts.
pub struct UserMethodPolicy;

impl UserMethodPolicy {
    /// Check if a user-defined method is allowed in loop condition context
    ///
    /// # Requirements for Condition Context
    ///
    /// - Method must be pure (no side effects)
    /// - Method should return boolean (for use in conditions)
    /// - Method should be deterministic (same inputs → same outputs)
    ///
    /// # Arguments
    ///
    /// * `box_name` - Name of the static box (e.g., "StringUtils")
    /// * `method_name` - Name of the method (e.g., "is_whitespace")
    ///
    /// # Returns
    ///
    /// * `true` - Method is whitelisted for condition context
    /// * `false` - Method is not whitelisted or unknown
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Loop condition: loop(i < n && not this.is_whitespace(ch))
    /// assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_whitespace"));
    /// assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "trim_start"));
    /// ```
    pub fn allowed_in_condition(box_name: &str, method_name: &str) -> bool {
        match box_name {
            "StringUtils" => Self::stringutils_allowed_in_condition(method_name),
            _ => false, // Unknown static box - fail-fast
        }
    }

    /// Check if a user-defined method is allowed in LoopBodyLocal init context
    ///
    /// # Requirements for Init Context
    ///
    /// - Method must be pure (no side effects)
    /// - Method can return any type (strings, integers, etc.)
    /// - Method should be deterministic
    ///
    /// # Arguments
    ///
    /// * `box_name` - Name of the static box (e.g., "StringUtils")
    /// * `method_name` - Name of the method (e.g., "trim_start")
    ///
    /// # Returns
    ///
    /// * `true` - Method is whitelisted for init context
    /// * `false` - Method is not whitelisted or unknown
    ///
    /// # Example
    ///
    /// ```ignore
    /// // LoopBodyLocal init: local ch = s.substring(i, i + 1)
    /// // (substring is allowed in init but not in condition)
    /// assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_start"));
    /// ```
    pub fn allowed_in_init(box_name: &str, method_name: &str) -> bool {
        match box_name {
            "StringUtils" => Self::stringutils_allowed_in_init(method_name),
            _ => false, // Unknown static box - fail-fast
        }
    }

    // ========================================================================
    // StringUtils Policy Table
    // ========================================================================

    /// StringUtils methods allowed in condition context
    ///
    /// All methods here are pure boolean-returning functions suitable for
    /// use in loop conditions and conditional expressions.
    ///
    /// # StringUtils Source
    ///
    /// See: `apps/lib/json_native/utils/string.hako`
    fn stringutils_allowed_in_condition(method_name: &str) -> bool {
        matches!(
            method_name,
            // Character classification (pure boolean functions)
            "is_whitespace"      // ch == " " or ch == "\t" or ...
            | "is_digit"         // ch == "0" or ch == "1" or ...
            | "is_hex_digit"     // is_digit(ch) or ch == "a" or ...
            | "is_alpha"         // (ch >= "a" and ch <= "z") or ...
            | "is_alphanumeric"  // is_alpha(ch) or is_digit(ch)

            // String validation (pure boolean functions)
            | "is_integer"       // Checks if string represents an integer
            | "is_empty_or_whitespace"  // trim(s).length() == 0

            // String matching (pure boolean functions)
            | "starts_with"      // s.substring(0, prefix.length()) == prefix
            | "ends_with"        // s.substring(s.length() - suffix.length(), ...) == suffix
            | "contains"         // index_of_string(s, substr) != -1
        )
    }

    /// StringUtils methods allowed in init context
    ///
    /// All methods here are pure functions but may return non-boolean types
    /// (strings, integers). Suitable for LoopBodyLocal initialization.
    ///
    /// # StringUtils Source
    ///
    /// See: `apps/lib/json_native/utils/string.hako`
    fn stringutils_allowed_in_init(method_name: &str) -> bool {
        matches!(
            method_name,
            // Whitespace handling (pure string functions)
            "trim"               // s.trim() (VM StringBox method)
            | "trim_start"       // Remove leading whitespace
            | "trim_end"         // Remove trailing whitespace

            // String search (pure integer-returning functions)
            | "index_of"         // First occurrence of character (-1 if not found)
            | "last_index_of"    // Last occurrence of character (-1 if not found)
            | "index_of_string"  // First occurrence of substring (-1 if not found)

            // String transformation (pure string functions)
            | "to_upper"         // Convert string to uppercase
            | "to_lower"         // Convert string to lowercase
            | "char_to_upper"    // Convert single character to uppercase
            | "char_to_lower"    // Convert single character to lowercase

            // String manipulation (pure functions)
            | "join"             // Join array elements with separator
            | "split"            // Split string by separator

            // Numeric parsing (pure functions)
            | "parse_float"      // Parse floating-point number (currently identity)
            | "parse_integer"    // Parse integer from string

            // Character classification (also allowed in init)
            | "is_whitespace"
            | "is_digit"
            | "is_hex_digit"
            | "is_alpha"
            | "is_alphanumeric"
            | "is_integer"
            | "is_empty_or_whitespace"
            | "starts_with"
            | "ends_with"
            | "contains"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Condition Context Tests =====

    #[test]
    fn test_stringutils_character_classification_in_condition() {
        // Pure boolean character classification methods should be allowed
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_whitespace"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_digit"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_hex_digit"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_alpha"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_alphanumeric"));
    }

    #[test]
    fn test_stringutils_validation_in_condition() {
        // Pure boolean validation methods should be allowed
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_integer"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_empty_or_whitespace"));
    }

    #[test]
    fn test_stringutils_matching_in_condition() {
        // Pure boolean matching methods should be allowed
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "starts_with"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "ends_with"));
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "contains"));
    }

    #[test]
    fn test_stringutils_string_functions_not_in_condition() {
        // String-returning functions should NOT be allowed in condition
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "trim"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "trim_start"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "trim_end"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "to_upper"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "to_lower"));
    }

    #[test]
    fn test_stringutils_search_not_in_condition() {
        // Integer-returning search functions should NOT be allowed in condition
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "index_of"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "last_index_of"));
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "index_of_string"));
    }

    #[test]
    fn test_unknown_static_box_in_condition() {
        // Unknown static boxes should fail-fast
        assert!(!UserMethodPolicy::allowed_in_condition("UnknownBox", "some_method"));
        assert!(!UserMethodPolicy::allowed_in_condition("MathUtils", "abs"));
    }

    // ===== Init Context Tests =====

    #[test]
    fn test_stringutils_all_pure_methods_in_init() {
        // All pure methods should be allowed in init (more permissive than condition)
        // Character classification
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "is_whitespace"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "is_digit"));

        // String manipulation
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_start"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_end"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "to_upper"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "to_lower"));

        // String search
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "index_of"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "last_index_of"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "index_of_string"));

        // Numeric parsing
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "parse_integer"));
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "parse_float"));
    }

    #[test]
    fn test_unknown_static_box_in_init() {
        // Unknown static boxes should fail-fast
        assert!(!UserMethodPolicy::allowed_in_init("UnknownBox", "some_method"));
        assert!(!UserMethodPolicy::allowed_in_init("MathUtils", "sqrt"));
    }

    // ===== Real-World Pattern Tests =====

    #[test]
    fn test_trim_end_pattern() {
        // Phase 252 P0: StringUtils.trim_end/1 pattern
        // loop(i >= 0) { if not this.is_whitespace(s.substring(i, i + 1)) { break } ... }

        // is_whitespace should be allowed in condition (boolean check)
        assert!(UserMethodPolicy::allowed_in_condition("StringUtils", "is_whitespace"));

        // trim_end itself should NOT be allowed in condition (string function)
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "trim_end"));

        // But trim_end should be allowed in init
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "trim_end"));
    }

    #[test]
    fn test_index_of_pattern() {
        // Case: local pos = this.index_of(s, ch)
        // index_of returns integer (-1 or index), not boolean

        // Should NOT be allowed in condition
        assert!(!UserMethodPolicy::allowed_in_condition("StringUtils", "index_of"));

        // But should be allowed in init
        assert!(UserMethodPolicy::allowed_in_init("StringUtils", "index_of"));
    }
}
