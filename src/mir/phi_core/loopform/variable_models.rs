//! Phase 191: LoopForm Variable Models
//!
//! Responsibility: Type definitions for LoopForm variable representations
//! - CarrierVariable: Loop-carried dependencies (modified in loop)
//! - PinnedVariable: Loop-invariant parameters
//! - LoopBypassFlags: Bypass control flags

use crate::mir::ValueId;

/// A carrier variable: modified within the loop (loop-carried dependency)
///
/// Represents a variable that is updated in the loop body.
/// Flow: init_value → preheader_copy → header_phi → latch_value
#[derive(Debug, Clone)]
pub struct CarrierVariable {
    pub name: String,
    pub init_value: ValueId, // Initial value from preheader (local variable)
    pub preheader_copy: ValueId, // Copy allocated in preheader block
    pub header_phi: ValueId, // PHI node allocated in header block
    pub latch_value: ValueId, // Updated value computed in latch (set during sealing)
}

/// A pinned variable: not modified in loop body (loop-invariant, typically parameters)
///
/// Represents a variable that remains constant throughout loop iterations.
/// Flow: param_value → preheader_copy → header_phi (with same value)
#[derive(Debug, Clone)]
pub struct PinnedVariable {
    pub name: String,
    pub param_value: ValueId, // Original parameter or loop-invariant value
    pub preheader_copy: ValueId, // Copy allocated in preheader block
    pub header_phi: ValueId,  // PHI node allocated in header block
}

/// Phase 27.4-C Refactor: JoinIR Loop φ bypass flags
///
/// Controls bypass behavior for Header and Exit PHI nodes.
/// Future: Will be integrated with JoinIR Exit φ bypass (Phase 27.6-2).
#[derive(Debug, Clone, Copy, Default)]
pub struct LoopBypassFlags {
    /// Header φ bypass enabled
    pub header: bool,
    // Phase 30: exit field removed (completely unused, will be replaced by JoinIR)
}

impl LoopBypassFlags {
    /// Create new bypass flags with all disabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if all bypass flags are enabled
    pub fn all_enabled(&self) -> bool {
        // Currently only header flag exists
        self.header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bypass_flags_default() {
        let flags = LoopBypassFlags::new();
        assert!(!flags.all_enabled());
        assert!(!flags.header);
    }

    #[test]
    fn test_bypass_flags_all_enabled() {
        let mut flags = LoopBypassFlags::new();
        flags.header = true;
        assert!(flags.all_enabled());
    }

    #[test]
    fn test_carrier_variable_creation() {
        let carrier = CarrierVariable {
            name: "x".to_string(),
            init_value: ValueId(0),
            preheader_copy: ValueId(1),
            header_phi: ValueId(2),
            latch_value: ValueId(3),
        };
        assert_eq!(carrier.name, "x");
        assert_eq!(carrier.init_value.0, 0);
    }

    #[test]
    fn test_pinned_variable_creation() {
        let pinned = PinnedVariable {
            name: "param".to_string(),
            param_value: ValueId(0),
            preheader_copy: ValueId(1),
            header_phi: ValueId(2),
        };
        assert_eq!(pinned.name, "param");
        assert_eq!(pinned.param_value.0, 0);
    }
}
