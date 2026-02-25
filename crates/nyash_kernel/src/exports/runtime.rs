// Runtime/GC exports.

// Exported as: nyash.rt.checkpoint
#[export_name = "nyash.rt.checkpoint"]
pub extern "C" fn nyash_rt_checkpoint_export() -> i64 {
    if std::env::var("NYASH_RUNTIME_CHECKPOINT_TRACE")
        .ok()
        .as_deref()
        == Some("1")
    {
        eprintln!("[nyrt] nyash.rt.checkpoint reached");
    }
    0
}

// Exported as: nyash.gc.barrier_write
#[export_name = "nyash.gc.barrier_write"]
pub extern "C" fn nyash_gc_barrier_write_export(handle_or_ptr: i64) -> i64 {
    let _ = handle_or_ptr;
    if std::env::var("NYASH_GC_BARRIER_TRACE").ok().as_deref() == Some("1") {
        eprintln!("[nyrt] nyash.gc.barrier_write h=0x{:x}", handle_or_ptr);
    }
    // Forward to runtime GC hooks when available (Write barrier)
    nyash_rust::runtime::global_hooks::gc_barrier(nyash_rust::runtime::BarrierKind::Write);
    0
}

// LLVM safepoint exports (llvmlite harness)
// export: ny_safepoint(live_count: i64, live_values: i64*) -> void
#[no_mangle]
pub extern "C" fn ny_safepoint(_live_count: i64, _live_values: *const i64) {
    // For now we ignore live-values; runtime uses cooperative safepoint + poll
    nyash_rust::runtime::global_hooks::safepoint_and_poll();
}

// export: ny_check_safepoint() -> void
#[no_mangle]
pub extern "C" fn ny_check_safepoint() {
    nyash_rust::runtime::global_hooks::safepoint_and_poll();
}
