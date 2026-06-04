// Runtime/GC exports.

const REQUIRE_TYPED_OBJECT_DIRECT_SLOT_EXACT: i64 = 1;
const REQUIRE_ARRAY_DIRECT_I64_EXACT: i64 = 2;

fn backend_mode_env_error(context: &str, key: &str, expected: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(actual) if actual == expected => None,
        Ok(actual) => Some(format!(
            "[freeze:contract][direct-exact/runtime-mode] {context} expected {key}={expected} got={actual}"
        )),
        Err(_) => Some(format!(
            "[freeze:contract][direct-exact/runtime-mode] {context} expected {key}={expected} got=<unset>"
        )),
    }
}

// Exported as: nyash.runtime.require_backend_modes_i
#[export_name = "nyash.runtime.require_backend_modes_i"]
pub extern "C" fn nyash_runtime_require_backend_modes_i(flags: i64) -> i64 {
    if let Some(message) = backend_modes_error(flags) {
        eprintln!("{message}");
        std::process::exit(70);
    }
    1
}

#[cfg(test)]
fn require_backend_modes(flags: i64) -> i64 {
    if let Some(message) = backend_modes_error(flags) {
        panic!("{message}");
    }
    1
}

fn backend_modes_error(flags: i64) -> Option<String> {
    if flags & REQUIRE_TYPED_OBJECT_DIRECT_SLOT_EXACT != 0 {
        if let Some(message) = backend_mode_env_error(
            "typed_object_store",
            "HAKO_TYPED_OBJECT_STORE",
            "direct_slot_exact",
        ) {
            return Some(message);
        }
    }
    if flags & REQUIRE_ARRAY_DIRECT_I64_EXACT != 0 {
        if let Some(message) = backend_mode_env_error(
            "array_slot_store",
            "HAKO_ARRAY_SLOT_STORE",
            "direct_array_i64_exact",
        ) {
            return Some(message);
        }
    }
    None
}

// Exported as: nyash.rt.checkpoint
#[export_name = "nyash.rt.checkpoint"]
pub extern "C" fn nyash_rt_checkpoint_export() -> i64 {
    if crate::env_flags::flag_on("NYASH_RUNTIME_CHECKPOINT_TRACE") {
        eprintln!("[nyrt] nyash.rt.checkpoint reached");
    }
    0
}

// Exported as: nyash.gc.barrier_write
#[export_name = "nyash.gc.barrier_write"]
pub extern "C" fn nyash_gc_barrier_write_export(handle_or_ptr: i64) -> i64 {
    let _ = handle_or_ptr;
    if crate::env_flags::flag_on("NYASH_GC_BARRIER_TRACE") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvRestore {
        key: &'static str,
        old: Option<String>,
    }

    impl EnvRestore {
        fn unset(key: &'static str) -> Self {
            let old = std::env::var(key).ok();
            std::env::remove_var(key);
            Self { key, old }
        }

        fn set(key: &'static str, value: &'static str) -> Self {
            let old = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, old }
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            if let Some(value) = &self.old {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn direct_exact_runtime_mode_accepts_matching_env() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _typed = EnvRestore::set("HAKO_TYPED_OBJECT_STORE", "direct_slot_exact");
        let _array = EnvRestore::set("HAKO_ARRAY_SLOT_STORE", "direct_array_i64_exact");

        assert_eq!(
            require_backend_modes(
                REQUIRE_TYPED_OBJECT_DIRECT_SLOT_EXACT | REQUIRE_ARRAY_DIRECT_I64_EXACT,
            ),
            1
        );
    }

    #[test]
    #[should_panic(expected = "[freeze:contract][direct-exact/runtime-mode]")]
    fn direct_exact_runtime_mode_rejects_missing_typed_env() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _typed = EnvRestore::unset("HAKO_TYPED_OBJECT_STORE");
        let _array = EnvRestore::set("HAKO_ARRAY_SLOT_STORE", "direct_array_i64_exact");

        let _ = require_backend_modes(REQUIRE_TYPED_OBJECT_DIRECT_SLOT_EXACT);
    }

    #[test]
    #[should_panic(expected = "[freeze:contract][direct-exact/runtime-mode]")]
    fn direct_exact_runtime_mode_rejects_missing_array_env() {
        let _guard = ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _typed = EnvRestore::set("HAKO_TYPED_OBJECT_STORE", "direct_slot_exact");
        let _array = EnvRestore::unset("HAKO_ARRAY_SLOT_STORE");

        let _ = require_backend_modes(REQUIRE_ARRAY_DIRECT_I64_EXACT);
    }
}
