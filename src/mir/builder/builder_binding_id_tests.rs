//! Test-only binding/value allocator and lexical-shadowing coverage.
//!
//! The parent `builder` module remains the production barrel. Keeping this
//! cluster in a path child preserves the logical `binding_id_tests` module
//! while removing test-only bodies from the production entry file.

use super::*;

#[test]
fn test_binding_map_initialization() {
    let builder = MirBuilder::new();
    assert_eq!(builder.core_ctx.next_binding_id, 0);
    // Phase 2-6: binding_ctx is now SSOT (legacy field removed)
    assert!(builder.function_state.binding_ctx.is_empty());
}

#[test]
fn test_binding_allocation_sequential() {
    let mut builder = MirBuilder::new();
    let bid0 = builder.allocate_binding_id().unwrap();
    let bid1 = builder.allocate_binding_id().unwrap();
    let bid2 = builder.allocate_binding_id().unwrap();

    assert_eq!(bid0.raw(), 0);
    assert_eq!(bid1.raw(), 1);
    assert_eq!(bid2.raw(), 2);
    assert_eq!(builder.core_ctx.next_binding_id, 3);
}

#[test]
fn test_shadowing_binding_restore() {
    let mut builder = MirBuilder::new();

    // Simulate function entry scope
    builder.push_lexical_scope_for_test();

    // Declare outer x
    // Phase 136 P0: Use SSOT allocator for function scope simulation
    let outer_vid = builder.next_value_id();
    builder
        .declare_local_in_current_scope("x", outer_vid)
        .unwrap();
    // Phase 2-6: Check binding_ctx (SSOT)
    let outer_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
    assert_eq!(outer_bid.raw(), 0);

    // Enter inner scope and shadow x
    builder.push_lexical_scope_for_test();
    // Phase 136 P0: Use SSOT allocator for function scope simulation
    let inner_vid = builder.next_value_id();
    builder
        .declare_local_in_current_scope("x", inner_vid)
        .unwrap();
    // Phase 2-6: Check binding_ctx (SSOT)
    let inner_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
    assert_eq!(inner_bid.raw(), 1);

    // Exit inner scope - should restore outer binding
    builder.pop_lexical_scope_for_test();
    // Phase 2-6: Check binding_ctx (SSOT)
    let restored_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
    assert_eq!(restored_bid, outer_bid);
    assert_eq!(restored_bid.raw(), 0);

    // Cleanup
    builder.pop_lexical_scope_for_test();
}

#[test]
fn test_valueid_binding_parallel_allocation() {
    let mut builder = MirBuilder::new();

    // Phase 136 P0: Use SSOT allocator (next_value_id)
    // Note: Without current_function, next_value_id() falls back to value_gen.next()
    // so this test still validates ValueId/BindingId independence
    // Allocate ValueIds and BindingIds in parallel
    let vid0 = builder.next_value_id();
    let bid0 = builder.allocate_binding_id().unwrap();
    let vid1 = builder.next_value_id();
    let bid1 = builder.allocate_binding_id().unwrap();

    // ValueId and BindingId should be independent
    assert_eq!(vid0.0, 0);
    assert_eq!(bid0.raw(), 0);
    assert_eq!(vid1.0, 1);
    assert_eq!(bid1.raw(), 1);

    // Allocating more ValueIds should not affect BindingId counter
    let _ = builder.next_value_id();
    let _ = builder.next_value_id();
    let bid2 = builder.allocate_binding_id().unwrap();
    assert_eq!(bid2.raw(), 2); // Still sequential

    // Allocating more BindingIds should not affect ValueId counter
    let _ = builder.allocate_binding_id().unwrap();
    let _ = builder.allocate_binding_id().unwrap();
    let vid2 = builder.next_value_id();
    assert_eq!(vid2.0, 4); // Continues from where we left off
}
