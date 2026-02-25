//! Phase 29y.1 Task 1: Handle ABI shim for lifecycle operations
//!
//! This module provides unified handle lifecycle FFI functions.
//! The naming uses `_h` suffix to distinguish from NyBox structure ABI.
//!
//! SSOT: docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
//!
//! Handle ABI contract:
//! - retain_h(h): h == 0 → return 0 (no-op), else return new handle to same object
//! - release_h(h): h == 0 → no-op, else decrement reference count

/// nyrt_handle_retain_h: Retain strong reference (increment ref count)
///
/// # Arguments
/// * `handle` - Strong Box handle (i64)
///
/// # Returns
/// * New handle to same object on success (reference count +1)
/// * 0 if input was 0 or invalid
///
/// # Contract (10-ABI-SSOT.md)
/// - h == 0 → return 0 (no-op)
/// - h != 0 → return new handle to same object
#[no_mangle]
pub extern "C" fn nyrt_handle_retain_h(handle: i64) -> i64 {
    use nyash_rust::runtime::host_handles;

    if handle <= 0 {
        return 0;
    }

    // Get Arc from handle (this clones the Arc, incrementing ref count)
    if let Some(arc) = host_handles::get(handle as u64) {
        // Allocate new handle for the cloned Arc
        let new_handle = host_handles::to_handle_arc(arc);
        return new_handle as i64;
    }

    0 // Invalid handle
}

/// nyrt_handle_release_h: Release strong reference (decrement ref count)
///
/// # Arguments
/// * `handle` - Strong Box handle (i64)
///
/// # Contract (10-ABI-SSOT.md)
/// - h == 0 → no-op
/// - h != 0 → decrement reference count (may trigger deallocation)
#[no_mangle]
pub extern "C" fn nyrt_handle_release_h(handle: i64) {
    use nyash_rust::runtime::host_handles;

    if handle > 0 {
        host_handles::drop_handle(handle as u64);
    }
}

// ============================================================================
// Legacy name (backward compatibility)
// ============================================================================

/// ny_release_strong: Legacy name (backward compatibility)
///
/// Original name from Phase 287. Kept for backward compatibility with existing
/// LLVM lowering code. Equivalent to nyrt_handle_release_h.
///
/// New code should prefer nyrt_handle_release_h for clarity.
#[no_mangle]
pub extern "C" fn ny_release_strong(handle: i64) {
    nyrt_handle_release_h(handle);
}
