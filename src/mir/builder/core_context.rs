/*!
 * CoreContext - Core ID generation management for MirBuilder
 *
 * Phase 136 follow-up (Step 2/7): Extract ID generation fields from MirBuilder
 * to improve code organization and enable centralized ID allocation.
 *
 * Consolidates:
 * - value_gen: ValueIdGenerator for SSA values
 * - block_gen: BasicBlockIdGenerator for basic blocks
 * - next_binding_id: BindingId allocation counter
 * - temp_slot_counter: Temporary pin slot counter
 * - debug_join_counter: Debug scope join ID counter
 */

use crate::mir::{BasicBlockId, BasicBlockIdGenerator, BindingId, ValueId, ValueIdGenerator};

/// Core ID generation context for MIR builder
///
/// Provides centralized allocation for all MIR entity IDs.
/// All ID generators are collected here for better organization and SSOT compliance.
#[derive(Debug)]
pub(crate) struct CoreContext {
    /// Primary ValueId generator for SSA values
    pub value_gen: ValueIdGenerator,

    /// BasicBlockId generator for control flow graph
    pub block_gen: BasicBlockIdGenerator,

    /// Phase 74: BindingId allocation counter (parallel to ValueId)
    /// Monotonically increasing counter for lexical variable binding IDs.
    pub next_binding_id: u32,

    /// Internal counter for temporary pin slots (block-crossing ephemeral values)
    pub temp_slot_counter: u32,

    /// Phase 136: Debug scope join ID counter (deterministic region tracking)
    pub debug_join_counter: u32,
}

impl CoreContext {
    /// Create a new CoreContext with default-initialized generators
    pub fn new() -> Self {
        Self {
            value_gen: ValueIdGenerator::new(),
            block_gen: BasicBlockIdGenerator::new(),
            next_binding_id: 0,
            temp_slot_counter: 0,
            debug_join_counter: 0,
        }
    }

    /// Allocate the next ValueId from the primary generator
    ///
    /// Note: MirBuilder::next_value_id() provides higher-level allocation
    /// with function context and reserved ID skipping.
    pub fn next_value(&mut self) -> ValueId {
        self.value_gen.next()
    }

    /// Allocate the next BasicBlockId
    pub fn next_block(&mut self) -> BasicBlockId {
        self.block_gen.next()
    }

    /// Allocate the next BindingId
    ///
    /// Phase 74: Independent from ValueId allocation to support stable binding
    /// identity across SSA transformations.
    pub fn next_binding(&mut self) -> BindingId {
        let id = BindingId::new(self.next_binding_id);
        self.next_binding_id = self.next_binding_id.saturating_add(1);
        debug_assert!(
            self.next_binding_id < u32::MAX,
            "BindingId counter overflow: {}",
            self.next_binding_id
        );
        id
    }

    /// Allocate the next temporary pin slot counter value
    pub fn next_temp_slot(&mut self) -> u32 {
        let id = self.temp_slot_counter;
        self.temp_slot_counter = self.temp_slot_counter.saturating_add(1);
        id
    }

    /// Allocate the next debug join counter value
    pub fn next_debug_join(&mut self) -> u32 {
        let id = self.debug_join_counter;
        self.debug_join_counter = self.debug_join_counter.saturating_add(1);
        id
    }

    /// Peek at the next ValueId without consuming it
    #[allow(dead_code)]
    pub fn peek_next_value(&self) -> ValueId {
        self.value_gen.peek_next()
    }

    /// Peek at the next BasicBlockId without consuming it
    #[allow(dead_code)]
    pub fn peek_next_block(&self) -> BasicBlockId {
        self.block_gen.peek_next()
    }
}

impl Default for CoreContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_context_creation() {
        let ctx = CoreContext::new();
        assert_eq!(ctx.peek_next_value().as_u32(), 0);
        assert_eq!(ctx.peek_next_block().as_u32(), 0);
        assert_eq!(ctx.next_binding_id, 0);
        assert_eq!(ctx.temp_slot_counter, 0);
        assert_eq!(ctx.debug_join_counter, 0);
    }

    #[test]
    fn test_value_allocation() {
        let mut ctx = CoreContext::new();
        let v0 = ctx.next_value();
        let v1 = ctx.next_value();
        let v2 = ctx.next_value();
        assert_eq!(v0.as_u32(), 0);
        assert_eq!(v1.as_u32(), 1);
        assert_eq!(v2.as_u32(), 2);
        assert_eq!(ctx.peek_next_value().as_u32(), 3);
    }

    #[test]
    fn test_block_allocation() {
        let mut ctx = CoreContext::new();
        let b0 = ctx.next_block();
        let b1 = ctx.next_block();
        let b2 = ctx.next_block();
        assert_eq!(b0.as_u32(), 0);
        assert_eq!(b1.as_u32(), 1);
        assert_eq!(b2.as_u32(), 2);
        assert_eq!(ctx.peek_next_block().as_u32(), 3);
    }

    #[test]
    fn test_binding_allocation() {
        let mut ctx = CoreContext::new();
        let bid0 = ctx.next_binding();
        let bid1 = ctx.next_binding();
        let bid2 = ctx.next_binding();
        assert_eq!(bid0.raw(), 0);
        assert_eq!(bid1.raw(), 1);
        assert_eq!(bid2.raw(), 2);
        assert_eq!(ctx.next_binding_id, 3);
    }

    #[test]
    fn test_temp_slot_allocation() {
        let mut ctx = CoreContext::new();
        let t0 = ctx.next_temp_slot();
        let t1 = ctx.next_temp_slot();
        let t2 = ctx.next_temp_slot();
        assert_eq!(t0, 0);
        assert_eq!(t1, 1);
        assert_eq!(t2, 2);
    }

    #[test]
    fn test_debug_join_allocation() {
        let mut ctx = CoreContext::new();
        let d0 = ctx.next_debug_join();
        let d1 = ctx.next_debug_join();
        let d2 = ctx.next_debug_join();
        assert_eq!(d0, 0);
        assert_eq!(d1, 1);
        assert_eq!(d2, 2);
    }

    #[test]
    fn test_independent_counters() {
        let mut ctx = CoreContext::new();
        let v0 = ctx.next_value();
        let b0 = ctx.next_block();
        let bid0 = ctx.next_binding();
        let v1 = ctx.next_value();
        let b1 = ctx.next_block();
        let bid1 = ctx.next_binding();

        // All counters are independent
        assert_eq!(v0.as_u32(), 0);
        assert_eq!(v1.as_u32(), 1);
        assert_eq!(b0.as_u32(), 0);
        assert_eq!(b1.as_u32(), 1);
        assert_eq!(bid0.raw(), 0);
        assert_eq!(bid1.raw(), 1);
    }
}
