//! Phase 201: JoinValueSpace - Single source of truth for JoinIR ValueId allocation
//!
//! This module provides a unified ValueId allocator for JoinIR lowering to prevent
//! collisions between different allocation contexts (param vs local vs PHI).
//!
//! ## Problem Solved
//!
//! Before Phase 201, `loop_break` frontend used `alloc_join_value()` for env variables,
//! while JoinIR lowering used a separate `alloc_value()` starting from 0. Both could
//! produce the same ValueId for different purposes, causing PHI corruption after remapping.
//!
//! ## ValueId Space Layout
//!
//! ```text
//!  0          100        1000                     u32::MAX
//!  ├──────────┼──────────┼──────────────────────────┤
//!  │  PHI     │  Param   │       Local             │
//!  │  Reserved│  Region  │       Region            │
//!  └──────────┴──────────┴──────────────────────────┘
//! ```
//!
//! - **PHI Reserved (0-99)**: Pre-reserved for LoopHeader PHI dst
//! - **Param Region (100-999)**: For ConditionEnv, CarrierInfo.join_id, CapturedEnv
//! - **Local Region (1000+)**: For Const, BinOp, etc. in route lowerers
//!
//! ## Usage
//!
//! ```ignore
//! let mut space = JoinValueSpace::new();
//!
//! // Route frontend allocates param IDs
//! let i_param = space.alloc_param(); // ValueId(100)
//! let v_param = space.alloc_param(); // ValueId(101)
//!
//! // PHI builder reserves PHI dst
//! space.reserve_phi(ValueId(0)); // Mark as reserved
//!
//! // JoinIR lowerer allocates local IDs
//! let const_100 = space.alloc_local(); // ValueId(1000)
//! let binop_result = space.alloc_local(); // ValueId(1001)
//!
//! // No collision possible!
//! ```

use crate::mir::ValueId;
use std::collections::HashSet;

/// Region boundaries (can be tuned based on actual usage)
/// Phase 205: Explicit min/max constants for each region
pub const PHI_RESERVED_MIN: u32 = 0;
pub const PHI_RESERVED_MAX: u32 = 99;
pub const PARAM_MIN: u32 = 100;
pub const PARAM_MAX: u32 = 999;
pub const LOCAL_MIN: u32 = 1000;
pub const LOCAL_MAX: u32 = 100000;

// Legacy aliases for backward compatibility
const PHI_MAX: u32 = PHI_RESERVED_MAX;
const PARAM_BASE: u32 = PARAM_MIN;
const LOCAL_BASE: u32 = LOCAL_MIN;

/// Single source of truth for JoinIR ValueId allocation
///
/// All JoinIR ValueId allocation should go through this box to ensure
/// disjoint regions for Param, Local, and PHI dst IDs.
#[derive(Debug, Clone)]
pub struct JoinValueSpace {
    /// Next available param ID (starts at PARAM_BASE)
    next_param: u32,
    /// Next available local ID (starts at LOCAL_BASE)
    next_local: u32,
    /// Reserved PHI dst IDs (debug verification only)
    reserved_phi: HashSet<u32>,
    /// Phase 205: Track all allocated IDs for collision detection (debug-only)
    #[cfg(debug_assertions)]
    allocated_ids: HashSet<u32>,
}

/// Region classification for ValueIds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    /// PHI Reserved region (0-99)
    PhiReserved,
    /// Param region (100-999)
    Param,
    /// Local region (1000+)
    Local,
    /// Unknown/invalid region
    Unknown,
}

impl JoinValueSpace {
    /// Create a new JoinValueSpace with default regions
    pub fn new() -> Self {
        Self {
            next_param: PARAM_BASE,
            next_local: LOCAL_BASE,
            reserved_phi: HashSet::new(),
            #[cfg(debug_assertions)]
            allocated_ids: HashSet::new(),
        }
    }

    /// Phase 205: Check for ValueId collision (debug-only)
    ///
    /// Panics if the given ValueId has already been allocated.
    /// This is a fail-fast mechanism to detect bugs in JoinIR lowering.
    #[cfg(debug_assertions)]
    fn check_collision(&self, id: ValueId, role: &str) {
        if self.allocated_ids.contains(&id.0) {
            panic!(
                "[JoinValueSpace] ValueId collision detected!\n\
                 ID: {:?}\n\
                 Role: {}\n\
                 This indicates a bug in JoinIR lowering - contact maintainer",
                id, role
            );
        }
    }

    /// Allocate a parameter ValueId (for ConditionEnv, CarrierInfo, etc.)
    ///
    /// Returns ValueId in Param Region (100-999).
    /// Panics in debug mode if param region overflows.
    pub fn alloc_param(&mut self) -> ValueId {
        let id = self.next_param;
        debug_assert!(
            id < LOCAL_BASE,
            "Param region overflow: {} >= {}",
            id,
            LOCAL_BASE
        );

        // Phase 205: Collision detection (debug-only)
        #[cfg(debug_assertions)]
        self.check_collision(ValueId(id), "param");

        #[cfg(debug_assertions)]
        self.allocated_ids.insert(id);

        self.next_param += 1;
        ValueId(id)
    }

    /// Allocate a local ValueId (for Const, BinOp, etc. in lowerers)
    ///
    /// Returns ValueId in Local Region (1000+).
    pub fn alloc_local(&mut self) -> ValueId {
        let id = self.next_local;

        // Phase 205: Collision detection (debug-only)
        #[cfg(debug_assertions)]
        self.check_collision(ValueId(id), "local");

        #[cfg(debug_assertions)]
        self.allocated_ids.insert(id);

        self.next_local += 1;
        ValueId(id)
    }

    /// Phase 286 P1+: JoinIR function parameter allocation
    ///
    /// Use this for: function parameters, loop variables, condition variables
    /// Wrapper around `alloc_param()` with explicit "JoinIR" context
    pub fn alloc_join_param(&mut self) -> ValueId {
        self.alloc_param()
    }

    /// Phase 286 P1+: JoinIR local variable allocation
    ///
    /// Use this for: temporary values, intermediate computations, local variables
    /// Wrapper around `alloc_local()` with explicit "JoinIR" context
    pub fn alloc_join_local(&mut self) -> ValueId {
        self.alloc_local()
    }

    /// Reserve a PHI dst ValueId (called by PHI builder before allocation)
    ///
    /// No allocation - just marks the ID as reserved for PHI use.
    /// This is for debug verification only; the actual PHI dst comes from
    /// MirBuilder (host side), not JoinValueSpace.
    pub fn reserve_phi(&mut self, id: ValueId) {
        debug_assert!(
            id.0 <= PHI_MAX,
            "PHI reservation out of range: {} > {}",
            id.0,
            PHI_MAX
        );
        self.reserved_phi.insert(id.0);
    }

    /// Check if a ValueId is reserved as PHI dst
    pub fn is_phi_reserved(&self, id: ValueId) -> bool {
        self.reserved_phi.contains(&id.0)
    }

    /// Determine which region a ValueId belongs to
    pub fn region_of(&self, id: ValueId) -> Region {
        if id.0 <= PHI_MAX {
            Region::PhiReserved
        } else if id.0 < LOCAL_BASE {
            Region::Param
        } else {
            Region::Local
        }
    }

    /// Phase 205: Verify that a ValueId is in the expected region (debug-only)
    ///
    /// Returns Ok(()) if the ValueId is in the expected region.
    /// Returns Err(message) if the region doesn't match.
    ///
    /// This is a fail-fast verification mechanism for debugging.
    #[cfg(debug_assertions)]
    pub fn verify_region(&self, id: ValueId, expected_region: Region) -> Result<(), String> {
        let actual = self.region_of(id);
        if actual != expected_region {
            return Err(format!(
                "ValueId {:?} is in {:?} region, expected {:?}\n\
                 Hint: Use alloc_param() for loop arguments, alloc_local() for JoinIR values",
                id, actual, expected_region
            ));
        }
        Ok(())
    }

    /// Get the current param counter (for debugging)
    pub fn param_count(&self) -> u32 {
        self.next_param - PARAM_BASE
    }

    /// Get the current local counter (for debugging)
    pub fn local_count(&self) -> u32 {
        self.next_local - LOCAL_BASE
    }

    /// Get the number of reserved PHI IDs (for debugging)
    pub fn phi_reserved_count(&self) -> usize {
        self.reserved_phi.len()
    }

    /// Verify no overlap between regions (debug assertion)
    ///
    /// This checks that:
    /// 1. Param region hasn't overflowed into Local region
    /// 2. Reserved PHI IDs are within PHI region
    ///
    /// Returns Ok(()) if valid, Err(message) if invalid.
    #[cfg(debug_assertions)]
    pub fn verify_no_overlap(&self) -> Result<(), String> {
        // Check param region hasn't overflowed
        if self.next_param >= LOCAL_BASE {
            return Err(format!(
                "Param region overflow: next_param={} >= LOCAL_BASE={}",
                self.next_param, LOCAL_BASE
            ));
        }

        // Check all reserved PHI IDs are in PHI region
        for &phi_id in &self.reserved_phi {
            if phi_id > PHI_MAX {
                return Err(format!(
                    "PHI ID {} is out of PHI region (max={})",
                    phi_id, PHI_MAX
                ));
            }
        }

        Ok(())
    }

    /// Create an allocator closure for local IDs
    ///
    /// This is a convenience method to create a closure compatible with
    /// existing lowerer signatures that expect `FnMut() -> ValueId`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut space = JoinValueSpace::new();
    /// let mut alloc_local = space.local_allocator();
    /// let id1 = alloc_local(); // ValueId(1000)
    /// let id2 = alloc_local(); // ValueId(1001)
    /// ```
    pub fn local_allocator(&mut self) -> impl FnMut() -> ValueId + '_ {
        move || self.alloc_local()
    }

    /// Create an allocator closure for param IDs
    ///
    /// Similar to local_allocator(), but for param region.
    pub fn param_allocator(&mut self) -> impl FnMut() -> ValueId + '_ {
        move || self.alloc_param()
    }
}

impl Default for JoinValueSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_space_has_correct_initial_values() {
        let space = JoinValueSpace::new();
        assert_eq!(space.next_param, PARAM_BASE);
        assert_eq!(space.next_local, LOCAL_BASE);
        assert!(space.reserved_phi.is_empty());
    }

    #[test]
    fn test_alloc_param_returns_correct_ids() {
        let mut space = JoinValueSpace::new();
        let id1 = space.alloc_param();
        let id2 = space.alloc_param();
        let id3 = space.alloc_param();

        assert_eq!(id1, ValueId(100));
        assert_eq!(id2, ValueId(101));
        assert_eq!(id3, ValueId(102));
    }

    #[test]
    fn test_alloc_local_returns_correct_ids() {
        let mut space = JoinValueSpace::new();
        let id1 = space.alloc_local();
        let id2 = space.alloc_local();
        let id3 = space.alloc_local();

        assert_eq!(id1, ValueId(1000));
        assert_eq!(id2, ValueId(1001));
        assert_eq!(id3, ValueId(1002));
    }

    #[test]
    fn test_param_and_local_do_not_overlap() {
        let mut space = JoinValueSpace::new();

        // Allocate many params
        for _ in 0..100 {
            space.alloc_param();
        }

        // Allocate many locals
        for _ in 0..100 {
            space.alloc_local();
        }

        // Param should be in range [100, 200)
        assert_eq!(space.next_param, 200);
        // Local should be in range [1000, 1100)
        assert_eq!(space.next_local, 1100);

        // No overlap possible
        assert!(space.next_param < LOCAL_BASE);
    }

    #[test]
    fn test_reserve_phi() {
        let mut space = JoinValueSpace::new();
        space.reserve_phi(ValueId(0));
        space.reserve_phi(ValueId(5));
        space.reserve_phi(ValueId(10));

        assert!(space.is_phi_reserved(ValueId(0)));
        assert!(space.is_phi_reserved(ValueId(5)));
        assert!(space.is_phi_reserved(ValueId(10)));
        assert!(!space.is_phi_reserved(ValueId(1)));
        assert!(!space.is_phi_reserved(ValueId(100)));
    }

    #[test]
    fn test_region_of() {
        let space = JoinValueSpace::new();

        // PHI region
        assert_eq!(space.region_of(ValueId(0)), Region::PhiReserved);
        assert_eq!(space.region_of(ValueId(50)), Region::PhiReserved);
        assert_eq!(space.region_of(ValueId(99)), Region::PhiReserved);

        // Param region
        assert_eq!(space.region_of(ValueId(100)), Region::Param);
        assert_eq!(space.region_of(ValueId(500)), Region::Param);
        assert_eq!(space.region_of(ValueId(999)), Region::Param);

        // Local region
        assert_eq!(space.region_of(ValueId(1000)), Region::Local);
        assert_eq!(space.region_of(ValueId(5000)), Region::Local);
        assert_eq!(space.region_of(ValueId(u32::MAX)), Region::Local);
    }

    #[test]
    fn test_counters() {
        let mut space = JoinValueSpace::new();

        assert_eq!(space.param_count(), 0);
        assert_eq!(space.local_count(), 0);
        assert_eq!(space.phi_reserved_count(), 0);

        space.alloc_param();
        space.alloc_param();
        space.alloc_local();
        space.reserve_phi(ValueId(0));

        assert_eq!(space.param_count(), 2);
        assert_eq!(space.local_count(), 1);
        assert_eq!(space.phi_reserved_count(), 1);
    }

    #[cfg(debug_assertions)]
    #[test]
    fn test_verify_no_overlap_success() {
        let mut space = JoinValueSpace::new();
        space.alloc_param();
        space.alloc_local();
        space.reserve_phi(ValueId(0));

        assert!(space.verify_no_overlap().is_ok());
    }

    #[test]
    fn test_local_allocator_closure() {
        let mut space = JoinValueSpace::new();
        let id1;
        let id2;
        {
            let mut alloc = space.local_allocator();
            id1 = alloc();
            id2 = alloc();
        }

        assert_eq!(id1, ValueId(1000));
        assert_eq!(id2, ValueId(1001));
    }

    #[test]
    fn test_param_allocator_closure() {
        let mut space = JoinValueSpace::new();
        let id1;
        let id2;
        {
            let mut alloc = space.param_allocator();
            id1 = alloc();
            id2 = alloc();
        }

        assert_eq!(id1, ValueId(100));
        assert_eq!(id2, ValueId(101));
    }

    /// Phase 201-A scenario: Verify that the bug case is impossible
    ///
    /// Previously: env['v'] = ValueId(7), const 100 dst = ValueId(7) -> collision
    /// Now: env['v'] = alloc_param() -> ValueId(100+), const 100 = alloc_local() -> ValueId(1000+)
    #[test]
    fn test_phase201a_scenario_no_collision() {
        let mut space = JoinValueSpace::new();

        // loop_break frontend allocates param for carrier 'v'
        let v_param = space.alloc_param(); // ValueId(100)

        // JoinIR lowering allocates local for const 100
        let const_100 = space.alloc_local(); // ValueId(1000)

        // They are in different regions - no collision!
        assert_ne!(v_param, const_100);
        assert_eq!(space.region_of(v_param), Region::Param);
        assert_eq!(space.region_of(const_100), Region::Local);
    }
}
