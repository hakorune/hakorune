// Weak reference FFI functions
// Phase 287 P4: Extracted from lib.rs for modularization

/// nyrt_weak_new: Create weak reference from strong handle
///
/// # Arguments
/// * `strong_handle` - Strong Box handle (>0)
///
/// # Returns
/// * Weak handle (bit 63 = 1) on success
/// * 0 on failure (invalid handle)
///
/// # SSOT
/// - docs/reference/language/lifecycle.md:179
/// - docs/development/current/main/phases/phase-285/phase-285llvm-1-design.md
#[no_mangle]
pub extern "C" fn nyrt_weak_new(strong_handle: i64) -> i64 {
    use nyash_rust::runtime::host_handles as handles;
    use nyash_rust::runtime::weak_handles;

    eprintln!("[nyrt_weak_new] called with handle: {}", strong_handle);

    if strong_handle <= 0 {
        eprintln!("[nyrt_weak_new] invalid handle (<=0), returning 0");
        return 0;
    }

    // Get Arc from strong handle
    if let Some(arc) = handles::get(strong_handle as u64) {
        // Downgrade to Weak and allocate weak handle
        let weak = std::sync::Arc::downgrade(&arc);
        let weak_handle = weak_handles::to_handle_weak(weak);
        eprintln!(
            "[nyrt_weak_new] success: strong {} → weak {}",
            strong_handle, weak_handle
        );
        return weak_handle;
    }

    eprintln!(
        "[nyrt_weak_new] handle {} not found in registry, returning 0",
        strong_handle
    );
    0 // Invalid handle
}

/// nyrt_weak_to_strong: Upgrade weak reference to strong handle
///
/// # Arguments
/// * `weak_handle` - Weak handle (bit 63 = 1)
///
/// # Returns
/// * Strong handle (>0) on success (target is alive)
/// * 0 (Void/null) on failure (target is dead or invalid handle)
///
/// # SSOT
/// - docs/reference/language/lifecycle.md:179
#[no_mangle]
pub extern "C" fn nyrt_weak_to_strong(weak_handle: i64) -> i64 {
    use nyash_rust::runtime::weak_handles;

    eprintln!(
        "[nyrt_weak_to_strong] called with weak_handle: {}",
        weak_handle
    );

    // Upgrade weak handle to strong handle (0 on failure)
    let result = weak_handles::upgrade_weak_handle(weak_handle);

    eprintln!(
        "[nyrt_weak_to_strong] result: {} (0=null/failed, >0=success)",
        result
    );
    result
}

/// nyrt_weak_drop: Release weak reference
///
/// # Arguments
/// * `weak_handle` - Weak handle (bit 63 = 1)
///
/// # Note
/// Called when WeakRef goes out of scope (LLVM backend cleanup)
#[no_mangle]
pub extern "C" fn nyrt_weak_drop(weak_handle: i64) {
    use nyash_rust::runtime::weak_handles;

    if weak_handle != 0 {
        weak_handles::drop_weak_handle(weak_handle);
    }
}
