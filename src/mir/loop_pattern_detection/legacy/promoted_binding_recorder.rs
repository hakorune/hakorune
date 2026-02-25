//! PromotedBindingRecorder - Type-safe BindingId recording for promoted variables
//!
//! This box centralizes the logic for recording promoted variable mappings
//! (original BindingId → promoted BindingId) in the promoted_bindings map.
//!
//! Replaces scattered binding_map wiring across 8 files with a single,
//! testable, reusable box.

#[cfg(feature = "normalized_dev")]
use crate::mir::binding_id::BindingId;
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
#[cfg(feature = "normalized_dev")]
use std::collections::BTreeMap;

/// Records promoted variable bindings in a type-safe manner.
///
/// Example:
/// ```ignore
/// let recorder = PromotedBindingRecorder::new(Some(&builder.binding_map));
/// recorder.record_promotion(&mut carrier_info, "digit_pos", "is_digit_pos")?;
/// ```
#[cfg(feature = "normalized_dev")]
pub struct PromotedBindingRecorder<'a> {
    binding_map: Option<&'a BTreeMap<String, BindingId>>,
}

/// Non-dev version (zero-sized, no lifetime)
#[cfg(not(feature = "normalized_dev"))]
pub struct PromotedBindingRecorder;

#[derive(Debug)]
pub enum BindingRecordError {
    OriginalNotFound(String),
    PromotedNotFound(String),
}

#[cfg(feature = "normalized_dev")]
impl<'a> PromotedBindingRecorder<'a> {
    /// Create a new recorder with optional binding map.
    pub fn new(binding_map: Option<&'a BTreeMap<String, BindingId>>) -> Self {
        Self { binding_map }
    }

    /// Record a promotion mapping (original_name → promoted_name).
    pub fn record_promotion(
        &self,
        carrier_info: &mut CarrierInfo,
        original_name: &str,
        promoted_name: &str,
    ) -> Result<(), BindingRecordError> {
        if let Some(binding_map) = self.binding_map {
            let original_bid = binding_map
                .get(original_name)
                .copied()
                .ok_or_else(|| BindingRecordError::OriginalNotFound(original_name.to_string()))?;

            let promoted_bid = binding_map
                .get(promoted_name)
                .copied()
                .ok_or_else(|| BindingRecordError::PromotedNotFound(promoted_name.to_string()))?;

            carrier_info.record_promoted_binding(original_bid, promoted_bid);
            Ok(())
        } else {
            Ok(()) // No binding map available
        }
    }
}

#[cfg(not(feature = "normalized_dev"))]
impl PromotedBindingRecorder {
    /// Create a new recorder (no-op in non-dev builds).
    pub fn new() -> Self {
        Self
    }

    /// Record a promotion mapping (no-op in non-dev builds).
    pub fn record_promotion(
        &self,
        _carrier_info: &mut CarrierInfo,
        _original_name: &str,
        _promoted_name: &str,
    ) -> Result<(), BindingRecordError> {
        Ok(()) // No-op in non-dev mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    fn make_test_carrier_info() -> CarrierInfo {
        CarrierInfo::with_carriers("test_var".to_string(), ValueId(0), vec![])
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_record_promotion_success() {
        use crate::mir::binding_id::BindingId;
        use std::collections::BTreeMap;

        let mut binding_map = BTreeMap::new();
        binding_map.insert("digit_pos".to_string(), BindingId::new(10));
        binding_map.insert("is_digit_pos".to_string(), BindingId::new(20));

        let recorder = PromotedBindingRecorder::new(Some(&binding_map));
        let mut carrier_info = make_test_carrier_info();

        assert!(recorder
            .record_promotion(&mut carrier_info, "digit_pos", "is_digit_pos")
            .is_ok());
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_record_promotion_missing_original() {
        use std::collections::BTreeMap;

        let binding_map = BTreeMap::new();
        let recorder = PromotedBindingRecorder::new(Some(&binding_map));
        let mut carrier_info = make_test_carrier_info();

        match recorder.record_promotion(&mut carrier_info, "digit_pos", "is_digit_pos") {
            Err(BindingRecordError::OriginalNotFound(name)) => {
                assert_eq!(name, "digit_pos");
            }
            _ => panic!("Expected OriginalNotFound error"),
        }
    }

    #[cfg(feature = "normalized_dev")]
    #[test]
    fn test_record_promotion_missing_promoted() {
        use crate::mir::binding_id::BindingId;
        use std::collections::BTreeMap;

        let mut binding_map = BTreeMap::new();
        binding_map.insert("digit_pos".to_string(), BindingId::new(10));

        let recorder = PromotedBindingRecorder::new(Some(&binding_map));
        let mut carrier_info = make_test_carrier_info();

        match recorder.record_promotion(&mut carrier_info, "digit_pos", "is_digit_pos") {
            Err(BindingRecordError::PromotedNotFound(name)) => {
                assert_eq!(name, "is_digit_pos");
            }
            _ => panic!("Expected PromotedNotFound error"),
        }
    }

    #[test]
    fn test_record_promotion_no_binding_map() {
        #[cfg(feature = "normalized_dev")]
        let recorder = PromotedBindingRecorder::new(None);
        #[cfg(not(feature = "normalized_dev"))]
        let recorder = PromotedBindingRecorder::new();

        let mut carrier_info = make_test_carrier_info();

        // Should be no-op
        assert!(recorder
            .record_promotion(&mut carrier_info, "digit_pos", "is_digit_pos")
            .is_ok());
    }
}
