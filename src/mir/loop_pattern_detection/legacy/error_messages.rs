//! Phase 170-ultrathink: Error Message Utilities
//!
//! Common error message formatting functions for loop pattern validation.
//! This module centralizes error messages to ensure consistency across
//! Pattern 2, Pattern 4, and future patterns.

use super::loop_condition_scope::CondVarScope;

/// Format an error message for unsupported loop-body-local variables in conditions
///
/// # Arguments
///
/// * `pattern_name` - Name of the pattern (e.g., "pattern2", "pattern4")
/// * `body_local_names` - Names of loop-body-local variables found in condition
///
/// # Returns
///
/// Formatted error string with:
/// - Clear identification of the problem
/// - List of problematic variable names
/// - Explanation of what is supported
/// - Suggestion for alternative patterns
///
/// # Example
///
/// ```
/// let err = format_unsupported_condition_error(
///     "pattern2",
///     &["ch", "temp"]
/// );
/// // Returns:
/// // "[joinir/pattern2] Unsupported condition: uses loop-body-local variables: ["ch", "temp"].
/// //  Pattern 2 supports only loop parameters and outer-scope variables.
/// //  Consider using Pattern 5+ for complex loop conditions."
/// ```
pub fn format_unsupported_condition_error(
    pattern_name: &str,
    body_local_names: &[&String],
) -> String {
    format!(
        "[joinir/{}] Unsupported condition: uses loop-body-local variables: {:?}. \
         Pattern {} supports only loop parameters and outer-scope variables. \
         Consider using Pattern 5+ for complex loop conditions.",
        pattern_name,
        body_local_names,
        pattern_name
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
    )
}

/// Extract loop-body-local variable names from a LoopConditionScope
///
/// Helper function to filter variables by LoopBodyLocal scope.
///
/// # Arguments
///
/// * `vars` - Slice of CondVarInfo to filter
///
/// # Returns
///
/// Vector of references to variable names that are LoopBodyLocal
pub fn extract_body_local_names(vars: &[super::loop_condition_scope::CondVarInfo]) -> Vec<&String> {
    vars.iter()
        .filter(|v| v.scope == CondVarScope::LoopBodyLocal)
        .map(|v| &v.name)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_error_pattern2() {
        let names = vec!["ch".to_string(), "temp".to_string()];
        let refs: Vec<&String> = names.iter().collect();
        let err = format_unsupported_condition_error("pattern2", &refs);

        assert!(err.contains("[joinir/pattern2]"));
        assert!(err.contains("loop-body-local variables"));
        assert!(err.contains("ch"));
        assert!(err.contains("temp"));
        assert!(err.contains("Pattern 2 supports"));
        assert!(err.contains("Pattern 5+"));
    }

    #[test]
    fn test_format_error_pattern4() {
        let names = vec!["x".to_string()];
        let refs: Vec<&String> = names.iter().collect();
        let err = format_unsupported_condition_error("pattern4", &refs);

        assert!(err.contains("[joinir/pattern4]"));
        assert!(err.contains("loop-body-local variables"));
        assert!(err.contains("x"));
        assert!(err.contains("Pattern 4 supports"));
    }
}

// ========================================================================
// Pattern 4 Specific Error Messages
// ========================================================================

/// Pattern4: Cannot promote LoopBodyLocal variables in condition
///
/// Used when LoopBodyCondPromoter fails to promote a LoopBodyLocal variable
/// used in loop conditions to a bool carrier.
///
/// # Arguments
///
/// * `vars` - Names of LoopBodyLocal variables that failed promotion
/// * `reason` - Human-readable reason for the failure
///
/// # Example
///
/// ```
/// let err = format_error_pattern4_promotion_failed(&["ch"], "not a Trim pattern");
/// // Returns: "[cf_loop/pattern4] Cannot promote LoopBodyLocal variables ["ch"]: not a Trim pattern"
/// ```
pub fn format_error_pattern4_promotion_failed(vars: &[String], reason: &str) -> String {
    format!(
        "[cf_loop/pattern4] Cannot promote LoopBodyLocal variables {:?}: {}",
        vars, reason
    )
}

/// Pattern4: Trim pattern detected but not safe
///
/// Used when a Trim pattern is detected but does not meet safety criteria
/// (e.g., too many whitespace characters, unsafe structure).
///
/// # Arguments
///
/// * `carrier_name` - Name of the carrier variable
/// * `whitespace_count` - Number of whitespace characters detected
///
/// # Example
///
/// ```
/// let err = format_error_pattern4_trim_not_safe("is_whitespace", 5);
/// // Returns: "[cf_loop/pattern4] Trim pattern detected but not safe: carrier='is_whitespace', whitespace_count=5"
/// ```
pub fn format_error_pattern4_trim_not_safe(carrier_name: &str, whitespace_count: usize) -> String {
    format!(
        "[cf_loop/pattern4] Trim pattern detected but not safe: carrier='{}', whitespace_count={}",
        carrier_name, whitespace_count
    )
}

/// Pattern4: Lowering failed
///
/// Generic error wrapper for Pattern 4 lowering failures.
///
/// # Arguments
///
/// * `cause` - The underlying error message
///
/// # Example
///
/// ```
/// let err = format_error_pattern4_lowering_failed("JoinIR conversion error");
/// // Returns: "[cf_loop/pattern4] Lowering failed: JoinIR conversion error"
/// ```
pub fn format_error_pattern4_lowering_failed(cause: &str) -> String {
    format!("[cf_loop/pattern4] Lowering failed: {}", cause)
}

/// Pattern4: Carrier not found in variable_map
///
/// Used when an expected carrier variable is missing from variable_map,
/// indicating an exit binding is not properly generated.
///
/// # Arguments
///
/// * `carrier_name` - Name of the missing carrier variable
///
/// # Example
///
/// ```
/// let err = format_error_pattern4_carrier_not_found("sum");
/// // Returns: "[cf_loop/pattern4] Carrier 'sum' not found in variable_map - exit binding missing"
/// ```
pub fn format_error_pattern4_carrier_not_found(carrier_name: &str) -> String {
    format!(
        "[cf_loop/pattern4] Carrier '{}' not found in variable_map - exit binding missing",
        carrier_name
    )
}

// ========================================================================
// Pattern 2 Specific Error Messages
// ========================================================================

/// Pattern2: Cannot promote LoopBodyLocal variables in condition
///
/// Used when LoopBodyCondPromoter fails to promote a LoopBodyLocal variable
/// used in loop/break conditions to a bool carrier.
///
/// # Arguments
///
/// * `vars` - Names of LoopBodyLocal variables that failed promotion
/// * `reason` - Human-readable reason for the failure
///
/// # Example
///
/// ```
/// let err = format_error_pattern2_promotion_failed(&["ch"], "not a Trim pattern");
/// // Returns: "[cf_loop/pattern2] Cannot promote LoopBodyLocal variables ["ch"]: not a Trim pattern"
/// ```
pub fn format_error_pattern2_promotion_failed(vars: &[String], reason: &str) -> String {
    format!(
        "[cf_loop/pattern2] Cannot promote LoopBodyLocal variables {:?}: {}",
        vars, reason
    )
}

/// Pattern2: Trim pattern detected but not safe
///
/// Used when a Trim pattern is detected but does not meet safety criteria.
///
/// # Arguments
///
/// * `carrier_name` - Name of the carrier variable
/// * `whitespace_count` - Number of whitespace characters detected
pub fn format_error_pattern2_trim_not_safe(carrier_name: &str, whitespace_count: usize) -> String {
    format!(
        "[cf_loop/pattern2] Trim pattern detected but not safe: carrier='{}', whitespace_count={}",
        carrier_name, whitespace_count
    )
}

#[cfg(test)]
mod pattern2_tests {
    use super::*;

    #[test]
    fn test_format_error_pattern2_promotion_failed() {
        let vars = vec!["ch".to_string(), "digit_pos".to_string()];
        let err = format_error_pattern2_promotion_failed(&vars, "cascading LoopBodyLocal");

        assert!(err.contains("[cf_loop/pattern2]"));
        assert!(err.contains("Cannot promote"));
        assert!(err.contains("ch"));
        assert!(err.contains("digit_pos"));
        assert!(err.contains("cascading LoopBodyLocal"));
    }

    #[test]
    fn test_format_error_pattern2_trim_not_safe() {
        let err = format_error_pattern2_trim_not_safe("is_digit", 3);

        assert!(err.contains("[cf_loop/pattern2]"));
        assert!(err.contains("Trim pattern"));
        assert!(err.contains("not safe"));
        assert!(err.contains("is_digit"));
        assert!(err.contains("whitespace_count=3"));
    }
}

#[cfg(test)]
mod pattern4_tests {
    use super::*;

    #[test]
    fn test_format_error_pattern4_promotion_failed() {
        let vars = vec!["ch".to_string(), "temp".to_string()];
        let err = format_error_pattern4_promotion_failed(&vars, "not a Trim pattern");

        assert!(err.contains("[cf_loop/pattern4]"));
        assert!(err.contains("Cannot promote"));
        assert!(err.contains("ch"));
        assert!(err.contains("temp"));
        assert!(err.contains("not a Trim pattern"));
    }

    #[test]
    fn test_format_error_pattern4_trim_not_safe() {
        let err = format_error_pattern4_trim_not_safe("is_whitespace", 5);

        assert!(err.contains("[cf_loop/pattern4]"));
        assert!(err.contains("Trim pattern"));
        assert!(err.contains("not safe"));
        assert!(err.contains("is_whitespace"));
        assert!(err.contains("whitespace_count=5"));
    }

    #[test]
    fn test_format_error_pattern4_lowering_failed() {
        let err = format_error_pattern4_lowering_failed("JoinIR error");

        assert!(err.contains("[cf_loop/pattern4]"));
        assert!(err.contains("Lowering failed"));
        assert!(err.contains("JoinIR error"));
    }

    #[test]
    fn test_format_error_pattern4_carrier_not_found() {
        let err = format_error_pattern4_carrier_not_found("sum");

        assert!(err.contains("[cf_loop/pattern4]"));
        assert!(err.contains("Carrier 'sum'"));
        assert!(err.contains("not found"));
        assert!(err.contains("exit binding missing"));
    }
}
