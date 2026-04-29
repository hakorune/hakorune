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
//! // JoinIR lowerer allocates local IDs
//! let const_100 = space.alloc_local(); // ValueId(1000)
//! let binop_result = space.alloc_local(); // ValueId(1001)
//!
//! // No collision possible!
//! ```

use crate::mir::ValueId;
#[cfg(debug_assertions)]
use std::collections::HashSet;

/// Region boundaries (can be tuned based on actual usage)
/// Phase 205: Explicit min/max constants for each region
pub const PHI_RESERVED_MIN: u32 = 0;
pub const PHI_RESERVED_MAX: u32 = 99;
pub const PARAM_MIN: u32 = 100;
pub const PARAM_MAX: u32 = 999;
pub const LOCAL_MIN: u32 = 1000;
pub const LOCAL_MAX: u32 = 100000;

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
    /// Phase 205: Track all allocated IDs for collision detection (debug-only)
    #[cfg(debug_assertions)]
    allocated_ids: HashSet<u32>,
}

impl JoinValueSpace {
    /// Create a new JoinValueSpace with default regions
    pub fn new() -> Self {
        Self {
            next_param: PARAM_BASE,
            next_local: LOCAL_BASE,
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
        assert!((PARAM_MIN..=PARAM_MAX).contains(&v_param.0));
        assert!(const_100.0 >= LOCAL_MIN);
    }
}
