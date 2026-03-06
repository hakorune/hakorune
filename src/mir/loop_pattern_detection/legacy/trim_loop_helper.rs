//! Phase 171-C-5: TrimLoopHelper - trim route lowering helper
//!
//! This module provides a helper struct for trim-route lowering.
//! It encapsulates route-specific logic for converting body-local
//! conditions to carrier-based conditions.
//!
//! ## Purpose
//!
//! When a trim route shape is detected (e.g., trim leading/trailing whitespace),
//! the LoopBodyCarrierPromoter promotes the body-local variable (like `ch`)
//! to a bool carrier (like `is_whitespace`).
//!
//! TrimLoopHelper stores the pattern information needed to:
//! 1. Generate carrier initialization code
//! 2. Generate carrier update code
//! 3. Map the promoted carrier back to the original variable semantics
//!
//! ## Example Use Case
//!
//! **Original pattern**:
//! ```nyash
//! loop(start < end) {
//!     local ch = s.substring(start, start+1)
//!     if ch == " " || ch == "\t" { start = start + 1 } else { break }
//! }
//! ```
//!
//! **After promotion**:
//! - Original variable: `ch`
//! - Promoted carrier: `is_whitespace` (bool)
//! - Comparison literals: `[" ", "\t"]`
//!
//! **TrimLoopHelper usage**:
//! ```rust
//! let helper = TrimLoopHelper {
//!     original_var: "ch".to_string(),
//!     carrier_name: "is_whitespace".to_string(),
//!     whitespace_chars: vec![" ".to_string(), "\t".to_string()],
//! };
//!
//! // Generate carrier initialization: is_whitespace = true
//! let init_value = helper.initial_value(); // true
//!
//! // Generate carrier update: is_whitespace = (ch == " " || ch == "\t")
//! let carrier_type = helper.carrier_type(); // "Bool"
//! ```

use super::loop_body_carrier_promoter::TrimPatternInfo;

/// Helper for trim-route lowering
///
/// Encapsulates route-specific logic for converting
/// body-local conditions to carrier-based conditions.
///
/// # Fields
///
/// * `original_var` - The original body-local variable name (e.g., "ch")
/// * `carrier_name` - The promoted carrier name (e.g., "is_whitespace")
/// * `whitespace_chars` - The whitespace characters to compare against (e.g., [" ", "\t", "\n", "\r"])
///
/// # Design Philosophy
///
/// This struct follows Box Theory principles:
/// - **Single Responsibility**: Only handles trim-route lowering logic
/// - **Reusability**: Can be used by both break-route and continue-route lowerers
/// - **Testability**: Pure data structure with simple accessors
#[derive(Debug, Clone)]
pub struct TrimLoopHelper {
    /// The original variable name (e.g., "ch")
    pub original_var: String,

    /// The promoted carrier name (e.g., "is_whitespace")
    pub carrier_name: String,

    /// Whitespace characters to compare against (e.g., [" ", "\t", "\n", "\r"])
    pub whitespace_chars: Vec<String>,
}

impl TrimLoopHelper {
    /// Create TrimLoopHelper from TrimPatternInfo
    ///
    /// # Arguments
    ///
    /// * `info` - The TrimPatternInfo from LoopBodyCarrierPromoter
    ///
    /// # Returns
    ///
    /// A new TrimLoopHelper with the same information
    ///
    /// # Example
    ///
    /// ```ignore
    /// let trim_info = TrimPatternInfo {
    ///     var_name: "ch".to_string(),
    ///     comparison_literals: vec![" ".to_string(), "\t".to_string()],
    ///     carrier_name: "is_whitespace".to_string(),
    /// };
    ///
    /// let helper = TrimLoopHelper::from_pattern_info(&trim_info);
    /// assert_eq!(helper.original_var, "ch");
    /// assert_eq!(helper.carrier_name, "is_whitespace");
    /// ```
    pub fn from_pattern_info(info: &TrimPatternInfo) -> Self {
        TrimLoopHelper {
            original_var: info.var_name.clone(),
            carrier_name: info.carrier_name.clone(),
            whitespace_chars: info.comparison_literals.clone(),
        }
    }

    /// Get the carrier type (always Bool for Trim pattern)
    ///
    /// Trim patterns always use bool carriers to represent
    /// "does the character match the whitespace set?"
    ///
    /// # Returns
    ///
    /// "Bool" - the carrier type name
    pub fn carrier_type(&self) -> &str {
        "Bool"
    }

    /// Get initial carrier value (true = continue looping)
    ///
    /// The carrier is initialized to `true` to represent
    /// "keep looping initially". When the character doesn't match
    /// whitespace, the carrier becomes `false` and the loop breaks.
    ///
    /// # Returns
    ///
    /// `true` - initial value for the carrier
    pub fn initial_value(&self) -> bool {
        true
    }

    /// Get the number of whitespace characters in the comparison set
    ///
    /// Useful for diagnostics and code generation.
    ///
    /// # Returns
    ///
    /// The count of whitespace characters (e.g., 4 for [" ", "\t", "\n", "\r"])
    pub fn whitespace_count(&self) -> usize {
        self.whitespace_chars.len()
    }

    /// Check if a specific character is in the whitespace set
    ///
    /// # Arguments
    ///
    /// * `ch` - The character to check (as a string slice)
    ///
    /// # Returns
    ///
    /// `true` if the character is in the whitespace set, `false` otherwise
    ///
    /// # Example
    ///
    /// ```ignore
    /// let helper = TrimLoopHelper {
    ///     whitespace_chars: vec![" ".to_string(), "\t".to_string()],
    ///     ..Default::default()
    /// };
    ///
    /// assert!(helper.is_whitespace(" "));
    /// assert!(helper.is_whitespace("\t"));
    /// assert!(!helper.is_whitespace("a"));
    /// ```
    pub fn is_whitespace(&self, ch: &str) -> bool {
        self.whitespace_chars.iter().any(|wc| wc == ch)
    }

    /// Check if this is a safe trim route shape that can bypass body-local restrictions
    ///
    /// A safe Trim pattern must:
    /// 1. Have a valid carrier name
    /// 2. Have at least one whitespace character to compare
    /// 3. Have the expected structure (substring + OR chain + break)
    ///
    /// # Returns
    ///
    /// `true` if this is a safe Trim pattern, `false` otherwise
    ///
    /// # Example
    ///
    /// ```ignore
    /// let helper = TrimLoopHelper {
    ///     original_var: "ch".to_string(),
    ///     carrier_name: "is_whitespace".to_string(),
    ///     whitespace_chars: vec![" ".to_string(), "\t".to_string()],
    /// };
    /// assert!(helper.is_safe_trim());
    /// ```
    pub fn is_safe_trim(&self) -> bool {
        // Basic validation
        !self.carrier_name.is_empty() && !self.whitespace_chars.is_empty()
    }

    /// Alias for is_safe_trim() - checks if this follows the Trim-like pattern
    ///
    /// This method provides a semantic alias for safety checks.
    ///
    /// # Returns
    ///
    /// `true` if this pattern is Trim-like, `false` otherwise
    pub fn is_trim_like(&self) -> bool {
        self.is_safe_trim()
    }

    /// Check if this pattern has the expected Trim structure:
    /// - substring() method call
    /// - OR chain of equality comparisons
    /// - break on non-match
    ///
    /// # Returns
    ///
    /// `true` if the pattern has valid structure, `false` otherwise
    ///
    /// # Implementation Note
    ///
    /// For now, just check basic requirements.
    /// The full structure was already validated by LoopBodyCarrierPromoter.
    pub fn has_valid_structure(&self) -> bool {
        !self.original_var.is_empty()
            && !self.carrier_name.is_empty()
            && !self.whitespace_chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_pattern_info() {
        let trim_info = TrimPatternInfo {
            var_name: "ch".to_string(),
            comparison_literals: vec![" ".to_string(), "\t".to_string()],
            carrier_name: "is_ch_match".to_string(),
        };

        let helper = TrimLoopHelper::from_pattern_info(&trim_info);

        assert_eq!(helper.original_var, "ch");
        assert_eq!(helper.carrier_name, "is_ch_match");
        assert_eq!(helper.whitespace_chars.len(), 2);
        assert!(helper.whitespace_chars.contains(&" ".to_string()));
        assert!(helper.whitespace_chars.contains(&"\t".to_string()));
    }

    #[test]
    fn test_carrier_type() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![],
        };

        assert_eq!(helper.carrier_type(), "Bool");
    }

    #[test]
    fn test_initial_value() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![],
        };

        assert_eq!(helper.initial_value(), true);
    }

    #[test]
    fn test_whitespace_count() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![
                " ".to_string(),
                "\t".to_string(),
                "\n".to_string(),
                "\r".to_string(),
            ],
        };

        assert_eq!(helper.whitespace_count(), 4);
    }

    #[test]
    fn test_is_whitespace() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![" ".to_string(), "\t".to_string()],
        };

        assert!(helper.is_whitespace(" "));
        assert!(helper.is_whitespace("\t"));
        assert!(!helper.is_whitespace("\n"));
        assert!(!helper.is_whitespace("a"));
    }

    #[test]
    fn test_is_safe_trim() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![" ".to_string(), "\t".to_string()],
        };
        assert!(helper.is_safe_trim());
        assert!(helper.is_trim_like());
    }

    #[test]
    fn test_is_safe_trim_empty_carrier() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "".to_string(), // Empty!
            whitespace_chars: vec![" ".to_string()],
        };
        assert!(!helper.is_safe_trim());
    }

    #[test]
    fn test_is_safe_trim_no_whitespace() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![], // Empty!
        };
        assert!(!helper.is_safe_trim());
    }

    #[test]
    fn test_has_valid_structure() {
        let helper = TrimLoopHelper {
            original_var: "ch".to_string(),
            carrier_name: "is_whitespace".to_string(),
            whitespace_chars: vec![" ".to_string()],
        };
        assert!(helper.has_valid_structure());
    }
}
