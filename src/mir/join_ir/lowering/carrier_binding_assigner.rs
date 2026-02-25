//! CarrierBindingAssigner - Assigns BindingIds to promoted carriers
//!
//! Solves the "synthetic carrier has no BindingId" problem by allocating
//! dedicated BindingIds for promoted carriers (e.g., is_digit_pos, is_ch_match).
//!
//! # Problem
//!
//! Phase 77's PromotedBindingRecorder expects both original and promoted names
//! to exist in builder.binding_map, but promoted names are synthetic and don't exist.
//!
//! # Solution (Phase 78)
//!
//! This box allocates BindingIds for synthetic carriers and records the mapping:
//!
//! 1. Get original BindingId from builder.binding_map (Fail-Fast if missing)
//! 2. Allocate new BindingId for promoted carrier
//! 3. Record mapping in carrier_info.promoted_bindings
//! 4. Set binding_id field on the promoted CarrierVar
//!
//! # Integration Point (Important!)
//!
//! **DO NOT call this from DigitPosPromoter or TrimPromoter**. Those run during
//! pattern detection phase and don't have access to MirBuilder.
//!
//! **INSTEAD, call this from Pattern2/4 lowering code** after receiving the
//! promoted CarrierInfo:
//!
//! ```ignore
//! // In Pattern2WithBreak::lower() or similar lowering code that has builder access:
//!
//! // After receiving promoted CarrierInfo from DigitPosPromoter
//! if let Some(promoted_var_name) = /* extract from promotion result */ {
//!     // Assign BindingId for promoted carrier
//!     CarrierBindingAssigner::assign_promoted_binding(
//!         builder,
//!         &mut carrier_info,
//!         "digit_pos",       // original_name (from promotion result)
//!         "is_digit_pos"     // promoted_carrier_name (from carrier_info)
//!     )?;
//! }
//!
//! // Continue with normal lowering...
//! ```
//!
//! # Example
//!
//! ```ignore
//! // DigitPos promotion: "digit_pos" → "is_digit_pos"
//! CarrierBindingAssigner::assign_promoted_binding(
//!     builder,
//!     carrier_info,
//!     "digit_pos",       // original_name
//!     "is_digit_pos"     // promoted_carrier_name
//! )?;
//! // Result:
//! // - carrier_info.promoted_bindings[BindingId(5)] = BindingId(10)
//! // - carrier_info.carriers["is_digit_pos"].binding_id = Some(BindingId(10))
//! ```

use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::MirBuilder;

#[derive(Debug)]
pub enum CarrierBindingError {
    CarrierNotFound(String),
}

pub struct CarrierBindingAssigner;

impl CarrierBindingAssigner {
    /// Assign BindingIds to promoted carriers and record in promoted_bindings.
    ///
    /// For DigitPos: original_name="digit_pos", promoted_carrier_name="is_digit_pos"
    /// For Trim: original_name="ch", promoted_carrier_name="is_ch_match"
    #[cfg(feature = "normalized_dev")]
    pub fn assign_promoted_binding(
        builder: &mut MirBuilder,
        carrier_info: &mut CarrierInfo,
        original_name: &str,
        promoted_carrier_name: &str,
    ) -> Result<(), CarrierBindingError> {
        let prev_original = builder.binding_map.get(original_name).copied();
        let prev_promoted = builder.binding_map.get(promoted_carrier_name).copied();

        // Ensure original BindingId exists (LoopBodyLocal promotion may happen before `local` is lowered).
        let original_bid = match prev_original {
            Some(bid) => bid,
            None => {
                let bid = builder.allocate_binding_id();
                builder.binding_map.insert(original_name.to_string(), bid);
                bid
            }
        };

        // Ensure promoted carrier BindingId exists (synthetic names are never in source binding_map).
        let promoted_bid = match prev_promoted {
            Some(bid) => bid,
            None => {
                let bid = builder.allocate_binding_id();
                builder
                    .binding_map
                    .insert(promoted_carrier_name.to_string(), bid);
                bid
            }
        };

        carrier_info.record_promoted_binding(original_bid, promoted_bid);

        let Some(carrier) = carrier_info
            .carriers
            .iter_mut()
            .find(|c| c.name == promoted_carrier_name)
        else {
            // Restore builder.binding_map before returning.
            match prev_original {
                Some(prev) => {
                    builder.binding_map.insert(original_name.to_string(), prev);
                }
                None => {
                    builder.binding_map.remove(original_name);
                }
            }
            match prev_promoted {
                Some(prev) => {
                    builder
                        .binding_map
                        .insert(promoted_carrier_name.to_string(), prev);
                }
                None => {
                    builder.binding_map.remove(promoted_carrier_name);
                }
            }
            return Err(CarrierBindingError::CarrierNotFound(
                promoted_carrier_name.to_string(),
            ));
        };
        carrier.binding_id = Some(promoted_bid);

        use super::debug_output_box::DebugOutputBox;
        let debug = DebugOutputBox::new("phase78/carrier_assigner");
        debug.log_if_enabled(|| {
            format!(
                "'{}' (BindingId({})) → '{}' (BindingId({}))",
                original_name, original_bid.0, promoted_carrier_name, promoted_bid.0
            )
        });

        // Restore builder.binding_map to avoid leaking synthetic names into the source map.
        match prev_original {
            Some(prev) => {
                builder.binding_map.insert(original_name.to_string(), prev);
            }
            None => {
                builder.binding_map.remove(original_name);
            }
        }
        match prev_promoted {
            Some(prev) => {
                builder
                    .binding_map
                    .insert(promoted_carrier_name.to_string(), prev);
            }
            None => {
                builder.binding_map.remove(promoted_carrier_name);
            }
        }

        Ok(())
    }

    /// No-op in non-dev builds
    #[cfg(not(feature = "normalized_dev"))]
    pub fn assign_promoted_binding(
        _builder: &mut MirBuilder,
        _carrier_info: &mut CarrierInfo,
        _original_name: &str,
        _promoted_carrier_name: &str,
    ) -> Result<(), CarrierBindingError> {
        Ok(())
    }
}

#[cfg(all(test, feature = "normalized_dev"))]
mod tests {
    use super::*;
    use crate::mir::ValueId;

    #[test]
    fn test_assign_promoted_binding_success() {
        use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};

        // Create a MirBuilder with binding_map
        let mut builder = MirBuilder::new();

        // Add original binding to builder.binding_map
        let original_bid = builder.allocate_binding_id();
        builder
            .binding_map
            .insert("digit_pos".to_string(), original_bid);

        // Create CarrierInfo with promoted carrier
        let mut carrier_info = CarrierInfo::with_carriers(
            "test_loop".to_string(),
            ValueId(0),
            vec![CarrierVar {
                name: "is_digit_pos".to_string(),
                host_id: ValueId(0),
                join_id: None,
                role: CarrierRole::ConditionOnly,
                init: CarrierInit::BoolConst(false),
                binding_id: None, // Should be set by assigner
            }],
        );

        // Assign promoted binding
        let result = CarrierBindingAssigner::assign_promoted_binding(
            &mut builder,
            &mut carrier_info,
            "digit_pos",
            "is_digit_pos",
        );

        assert!(result.is_ok());

        // Verify: promoted_bindings contains the mapping
        let promoted_bid = carrier_info.resolve_promoted_with_binding(original_bid);
        assert!(promoted_bid.is_some());

        // Verify: carrier.binding_id was set
        let carrier = carrier_info
            .carriers
            .iter()
            .find(|c| c.name == "is_digit_pos")
            .unwrap();
        assert_eq!(carrier.binding_id, promoted_bid);
    }

    #[test]
    fn test_assign_promoted_binding_missing_original() {
        use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};

        // Create MirBuilder WITHOUT adding "digit_pos" to binding_map
        let mut builder = MirBuilder::new();

        let mut carrier_info = CarrierInfo::with_carriers(
            "test_loop".to_string(),
            ValueId(0),
            vec![CarrierVar {
                name: "is_digit_pos".to_string(),
                host_id: ValueId(0),
                join_id: None,
                role: CarrierRole::ConditionOnly,
                init: CarrierInit::BoolConst(false),
                binding_id: None,
            }],
        );

        // Should succeed (original/promoted BindingId are allocated on demand).
        let result = CarrierBindingAssigner::assign_promoted_binding(
            &mut builder,
            &mut carrier_info,
            "digit_pos",
            "is_digit_pos",
        );

        assert!(result.is_ok());
        assert_eq!(carrier_info.promoted_bindings.len(), 1);

        let carrier = carrier_info
            .carriers
            .iter()
            .find(|c| c.name == "is_digit_pos")
            .unwrap();
        assert!(carrier.binding_id.is_some());
    }

    #[test]
    fn test_assign_promoted_binding_multiple_carriers() {
        use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};
        let mut builder = MirBuilder::new();

        // Add two original bindings
        let digit_pos_bid = builder.allocate_binding_id();
        builder
            .binding_map
            .insert("digit_pos".to_string(), digit_pos_bid);

        let ch_bid = builder.allocate_binding_id();
        builder.binding_map.insert("ch".to_string(), ch_bid);

        let mut carrier_info = CarrierInfo::with_carriers(
            "test_loop".to_string(),
            ValueId(0),
            vec![
                CarrierVar {
                    name: "is_digit_pos".to_string(),
                    host_id: ValueId(0),
                    join_id: None,
                    role: CarrierRole::ConditionOnly,
                    init: CarrierInit::BoolConst(false),
                    binding_id: None,
                },
                CarrierVar {
                    name: "is_ch_match".to_string(),
                    host_id: ValueId(0),
                    join_id: None,
                    role: CarrierRole::ConditionOnly,
                    init: CarrierInit::BoolConst(false),
                    binding_id: None,
                },
            ],
        );

        // Assign both promoted bindings
        CarrierBindingAssigner::assign_promoted_binding(
            &mut builder,
            &mut carrier_info,
            "digit_pos",
            "is_digit_pos",
        )
        .unwrap();

        CarrierBindingAssigner::assign_promoted_binding(
            &mut builder,
            &mut carrier_info,
            "ch",
            "is_ch_match",
        )
        .unwrap();

        // Verify both mappings exist
        assert!(carrier_info
            .resolve_promoted_with_binding(digit_pos_bid)
            .is_some());
        assert!(carrier_info.resolve_promoted_with_binding(ch_bid).is_some());

        // Verify both carriers have binding_id set
        let is_digit_pos = carrier_info
            .carriers
            .iter()
            .find(|c| c.name == "is_digit_pos")
            .unwrap();
        assert!(is_digit_pos.binding_id.is_some());

        let is_ch_match = carrier_info
            .carriers
            .iter()
            .find(|c| c.name == "is_ch_match")
            .unwrap();
        assert!(is_ch_match.binding_id.is_some());
    }
}
