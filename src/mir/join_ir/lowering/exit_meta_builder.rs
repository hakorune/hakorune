//! Phase 118: IfSumExitMetaBuilderBox - Exit metadata builder for if-sum pattern
//!
//! This module provides a box-based abstraction for building ExitMeta structures
//! in the if-sum pattern lowering process.
//!
//! # Design Philosophy
//!
//! - **Single Responsibility**: Only builds ExitMeta from carrier bindings
//! - **Fail-Fast**: Validates carrier names immediately
//! - **Box Theory**: Encapsulates ExitMeta construction logic
//!
//! # Usage
//!
//! ```ignore
//! let builder = IfSumExitMetaBuilderBox::new();
//! let exit_meta = builder.build_single(carrier_name, exit_value_id)?;
//! ```

use crate::mir::join_ir::lowering::carrier_info::ExitMeta;
use crate::mir::ValueId;

/// Phase 118: Box for building ExitMeta in if-sum pattern
///
/// This box separates the concern of "which carriers should be in exit_bindings"
/// from the JoinIR generation logic.
///
/// # Fail-Fast Contract
///
/// - Carrier name must be non-empty
/// - ValueId must be valid (no validation possible at this level)
///
/// # Example
///
/// ```ignore
/// let builder = IfSumExitMetaBuilderBox::new();
/// let exit_meta = builder.build_single("sum", sum_final)?;
/// // Creates: ExitMeta { exit_values: [("sum", sum_final)] }
/// ```
pub struct IfSumExitMetaBuilderBox;

impl IfSumExitMetaBuilderBox {
    /// Create a new IfSumExitMetaBuilderBox
    pub fn new() -> Self {
        Self
    }

    /// Build ExitMeta for a single carrier
    ///
    /// # Arguments
    ///
    /// * `carrier_name` - Name of the carrier variable (e.g., "sum")
    /// * `exit_value` - JoinIR-local ValueId for the exit value (e.g., sum_final from k_exit)
    ///
    /// # Returns
    ///
    /// * `Ok(ExitMeta)` - Successfully built ExitMeta
    /// * `Err(String)` - Carrier name validation failed (Fail-Fast)
    ///
    /// # Fail-Fast Guarantee
    ///
    /// This method immediately returns an error if the carrier name is empty,
    /// preventing downstream issues from undefined carrier bindings.
    pub fn build_single(&self, carrier_name: String, exit_value: ValueId) -> Result<ExitMeta, String> {
        // Fail-Fast: Validate carrier name
        if carrier_name.is_empty() {
            return Err("[IfSumExitMetaBuilderBox] Carrier name cannot be empty".to_string());
        }

        // Build ExitMeta with single carrier
        let mut exit_values = vec![];
        exit_values.push((carrier_name.clone(), exit_value));

        Ok(ExitMeta::multiple(exit_values))
    }

    /// Build ExitMeta for multiple carriers
    ///
    /// # Arguments
    ///
    /// * `carrier_bindings` - Vector of (carrier_name, exit_value) tuples
    ///
    /// # Returns
    ///
    /// * `Ok(ExitMeta)` - Successfully built ExitMeta
    /// * `Err(String)` - Validation failed (Fail-Fast)
    ///
    /// # Fail-Fast Guarantee
    ///
    /// This method immediately returns an error if:
    /// - Any carrier name is empty
    /// - No carrier bindings provided
    pub fn build_multiple(&self, carrier_bindings: Vec<(String, ValueId)>) -> Result<ExitMeta, String> {
        // Fail-Fast: Validate at least one carrier
        if carrier_bindings.is_empty() {
            return Err("[IfSumExitMetaBuilderBox] At least one carrier binding required".to_string());
        }

        // Fail-Fast: Validate all carrier names
        for (name, _) in &carrier_bindings {
            if name.is_empty() {
                return Err("[IfSumExitMetaBuilderBox] Carrier name cannot be empty".to_string());
            }
        }

        Ok(ExitMeta::multiple(carrier_bindings))
    }
}

impl Default for IfSumExitMetaBuilderBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_single_success() {
        let builder = IfSumExitMetaBuilderBox::new();
        let result = builder.build_single("sum".to_string(), ValueId(42));

        assert!(result.is_ok());
        let exit_meta = result.unwrap();
        assert_eq!(exit_meta.exit_values.len(), 1);
        assert_eq!(exit_meta.exit_values[0].0, "sum");
        assert_eq!(exit_meta.exit_values[0].1, ValueId(42));
    }

    #[test]
    fn test_build_single_empty_name_fails() {
        let builder = IfSumExitMetaBuilderBox::new();
        let result = builder.build_single("".to_string(), ValueId(42));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_build_multiple_success() {
        let builder = IfSumExitMetaBuilderBox::new();
        let carriers = vec![
            ("sum".to_string(), ValueId(10)),
            ("count".to_string(), ValueId(20)),
        ];
        let result = builder.build_multiple(carriers);

        assert!(result.is_ok());
        let exit_meta = result.unwrap();
        assert_eq!(exit_meta.exit_values.len(), 2);
    }

    #[test]
    fn test_build_multiple_empty_list_fails() {
        let builder = IfSumExitMetaBuilderBox::new();
        let result = builder.build_multiple(vec![]);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("At least one carrier"));
    }

    #[test]
    fn test_build_multiple_empty_name_fails() {
        let builder = IfSumExitMetaBuilderBox::new();
        let carriers = vec![
            ("sum".to_string(), ValueId(10)),
            ("".to_string(), ValueId(20)),
        ];
        let result = builder.build_multiple(carriers);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }
}
