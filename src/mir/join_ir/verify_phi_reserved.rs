//! Phase 72: PHI Reserved Region Verifier
//!
//! This module observes and verifies that PHI dst ValueIds stay within
//! the reserved region (0-99) as documented in join_value_space.rs.
//!
//! ## Goal
//!
//! According to the doc, PHI dst should be in the "PHI Reserved (0-99)" region.
//! However, the actual PHI dst allocation comes from `builder.next_value_id()`
//! in loop_header_phi_builder.rs, not from JoinValueSpace.
//!
//! This verifier collects PHI dst values during tests and checks if they
//! fall within the expected reserved region.

use crate::mir::ValueId;
use std::collections::BTreeSet;
use std::sync::Mutex;

/// Global collector for PHI dst observations (debug-only)
static PHI_DST_OBSERVATIONS: Mutex<Option<BTreeSet<u32>>> = Mutex::new(None);

/// Enable PHI dst observation (call this before tests)
#[cfg(debug_assertions)]
pub fn enable_observation() {
    let mut obs = PHI_DST_OBSERVATIONS.lock().unwrap();
    *obs = Some(BTreeSet::new());
}

/// Observe a PHI dst ValueId
#[cfg(debug_assertions)]
pub fn observe_phi_dst(dst: ValueId) {
    if let Ok(mut obs) = PHI_DST_OBSERVATIONS.lock() {
        if let Some(ref mut set) = *obs {
            set.insert(dst.0);
        }
    }
}

/// Get observation results and reset
#[cfg(debug_assertions)]
pub fn get_observations() -> Vec<u32> {
    let mut obs = PHI_DST_OBSERVATIONS.lock().unwrap();
    if let Some(ref mut set) = *obs {
        let result = set.iter().copied().collect();
        set.clear();
        result
    } else {
        vec![]
    }
}

/// Disable observation
#[cfg(debug_assertions)]
pub fn disable_observation() {
    let mut obs = PHI_DST_OBSERVATIONS.lock().unwrap();
    *obs = None;
}

/// Analyze PHI dst distribution
#[cfg(debug_assertions)]
pub fn analyze_distribution(observations: &[u32]) -> PhiDistributionReport {
    use crate::mir::join_ir::lowering::join_value_space::{LOCAL_MIN, PARAM_MIN, PHI_RESERVED_MAX};

    let mut in_reserved = 0;
    let mut in_param = 0;
    let mut in_local = 0;
    let mut min_val = u32::MAX;
    let mut max_val = 0;

    for &dst in observations {
        min_val = min_val.min(dst);
        max_val = max_val.max(dst);

        if dst <= PHI_RESERVED_MAX {
            in_reserved += 1;
        } else if dst < PARAM_MIN {
            // Between PHI_RESERVED_MAX+1 and PARAM_MIN-1 (gap region)
            in_param += 1; // Count as "leaked into param region vicinity"
        } else if dst < LOCAL_MIN {
            in_param += 1;
        } else {
            in_local += 1;
        }
    }

    PhiDistributionReport {
        total: observations.len(),
        in_reserved,
        in_param,
        in_local,
        min_val: if observations.is_empty() {
            None
        } else {
            Some(min_val)
        },
        max_val: if observations.is_empty() {
            None
        } else {
            Some(max_val)
        },
    }
}

/// Report of PHI dst distribution
#[derive(Debug, Clone)]
pub struct PhiDistributionReport {
    pub total: usize,
    pub in_reserved: usize,
    pub in_param: usize,
    pub in_local: usize,
    pub min_val: Option<u32>,
    pub max_val: Option<u32>,
}

impl PhiDistributionReport {
    /// Check if all PHI dsts are within reserved region (0-99)
    pub fn is_all_reserved(&self) -> bool {
        self.total > 0 && self.in_reserved == self.total
    }

    /// Get human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "PHI dst distribution: total={}, reserved(0-99)={}, param(100-999)={}, local(1000+)={}, range=[{}-{}]",
            self.total,
            self.in_reserved,
            self.in_param,
            self.in_local,
            self.min_val.map_or("N/A".to_string(), |v| v.to_string()),
            self.max_val.map_or("N/A".to_string(), |v| v.to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_distribution_empty() {
        let report = analyze_distribution(&[]);
        assert_eq!(report.total, 0);
        assert_eq!(report.in_reserved, 0);
        assert!(report.min_val.is_none());
        assert!(report.max_val.is_none());
    }

    #[test]
    fn test_analyze_distribution_all_reserved() {
        let observations = vec![0, 5, 10, 50, 99];
        let report = analyze_distribution(&observations);
        assert_eq!(report.total, 5);
        assert_eq!(report.in_reserved, 5);
        assert_eq!(report.in_param, 0);
        assert_eq!(report.in_local, 0);
        assert!(report.is_all_reserved());
        assert_eq!(report.min_val, Some(0));
        assert_eq!(report.max_val, Some(99));
    }

    #[test]
    fn test_analyze_distribution_mixed() {
        let observations = vec![0, 50, 100, 500, 1000, 2000];
        let report = analyze_distribution(&observations);
        assert_eq!(report.total, 6);
        assert_eq!(report.in_reserved, 2); // 0, 50
        assert_eq!(report.in_param, 2); // 100, 500
        assert_eq!(report.in_local, 2); // 1000, 2000
        assert!(!report.is_all_reserved());
        assert_eq!(report.min_val, Some(0));
        assert_eq!(report.max_val, Some(2000));
    }

    #[test]
    fn test_analyze_distribution_all_local() {
        let observations = vec![1000, 1500, 2000];
        let report = analyze_distribution(&observations);
        assert_eq!(report.total, 3);
        assert_eq!(report.in_reserved, 0);
        assert_eq!(report.in_param, 0);
        assert_eq!(report.in_local, 3);
        assert!(!report.is_all_reserved());
    }
}

#[cfg(test)]
mod observation_tests {
    use super::*;
    use crate::runtime::get_global_ring0;

    /// Phase 72-1: Observe PHI dst distribution in existing tests
    ///
    /// This test runs BEFORE strengthening the verifier. It collects actual
    /// PHI dst values and reports their distribution.
    ///
    /// Expected outcomes:
    /// - If all PHI dst in reserved (0-99) → proceed to strengthen verifier
    /// - If some PHI dst outside reserved → document why and skip verifier
    #[test]
    fn test_phase72_observe_phi_dst_distribution() {
        // Enable observation
        enable_observation();

        // The observations will come from loop_header_phi_builder
        // when it allocates PHI dst during other tests running in parallel
        // For this initial test, we just verify the mechanism works

        let observations = get_observations();
        let report = analyze_distribution(&observations);

        let ring0 = get_global_ring0();
        ring0
            .log
            .debug("\n[Phase 72] PHI dst observation report (initial):");
        ring0.log.debug(&format!("  {}", report.summary()));
        ring0.log.debug(&format!(
            "  All in reserved region: {}",
            report.is_all_reserved()
        ));

        // This test always passes - it's for observation only
        disable_observation();
    }
}
