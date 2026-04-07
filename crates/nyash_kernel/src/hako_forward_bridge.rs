// Host-service contract for hookable `.hako` entrypoints.
// This file belongs to the Rust host microkernel glue layer.

pub(crate) type HakoFutureSpawnInstanceFn = extern "C" fn(i64, i64, i64, i64) -> i64;
pub(crate) type HakoStringDispatchFn = extern "C" fn(i64, i64, i64, i64) -> i64;
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(test))]
use std::sync::OnceLock;

static FUTURE_SPAWN_INSTANCE_FN: AtomicUsize = AtomicUsize::new(0);
static STRING_DISPATCH_FN: AtomicUsize = AtomicUsize::new(0);

mod ffi {
    use super::{HakoFutureSpawnInstanceFn, HakoStringDispatchFn};

    unsafe extern "C" {
        pub fn nyrt_hako_register_future_spawn_instance(
            f: Option<HakoFutureSpawnInstanceFn>,
        ) -> i64;
        pub fn nyrt_hako_register_string_dispatch(f: Option<HakoStringDispatchFn>) -> i64;
    }
}

pub(crate) mod string_ops {
    pub const LEN_H: i64 = 1;
    pub const CHARCODE_AT_H: i64 = 2;
    pub const CONCAT_HH: i64 = 3;
    pub const CONCAT3_HHH: i64 = 4;
    pub const EQ_HH: i64 = 5;
    pub const SUBSTRING_HII: i64 = 6;
    pub const INDEXOF_HH: i64 = 7;
    pub const LASTINDEXOF_HH: i64 = 8;
    pub const LT_HH: i64 = 9;
    pub const FROM_U64X2: i64 = 10;
}

fn stage1_string_dispatch_trace_enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var("STAGE1_CLI_DEBUG").ok().as_deref() == Some("1")
            || std::env::var("HAKO_STAGE1_MODULE_DISPATCH_TRACE")
                .ok()
                .as_deref()
                == Some("1")
    }
    #[cfg(not(test))]
    {
        static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
        *TRACE_ENABLED.get_or_init(|| {
            std::env::var("STAGE1_CLI_DEBUG").ok().as_deref() == Some("1")
                || std::env::var("HAKO_STAGE1_MODULE_DISPATCH_TRACE")
                    .ok()
                    .as_deref()
                    == Some("1")
        })
    }
}

fn string_op_name(op: i64) -> &'static str {
    match op {
        string_ops::LEN_H => "len_h",
        string_ops::CHARCODE_AT_H => "charcode_at_h",
        string_ops::CONCAT_HH => "concat_hh",
        string_ops::CONCAT3_HHH => "concat3_hhh",
        string_ops::EQ_HH => "eq_hh",
        string_ops::SUBSTRING_HII => "substring_hii",
        string_ops::INDEXOF_HH => "indexof_hh",
        string_ops::LASTINDEXOF_HH => "lastindexof_hh",
        string_ops::LT_HH => "lt_hh",
        string_ops::FROM_U64X2 => "from_u64x2",
        _ => "unknown",
    }
}

pub(crate) fn call_future_spawn_instance(a0: i64, a1: i64, a2: i64, argc: i64) -> Option<i64> {
    let raw = FUTURE_SPAWN_INSTANCE_FN.load(Ordering::Acquire);
    if raw == 0 {
        return None;
    }
    let spawn: HakoFutureSpawnInstanceFn = unsafe { std::mem::transmute(raw) };
    Some(spawn(a0, a1, a2, argc))
}

pub(crate) fn call_string_dispatch(op: i64, a0: i64, a1: i64, a2: i64) -> Option<i64> {
    let dispatch = string_dispatch_fn()?;
    let out = dispatch(op, a0, a1, a2);
    if stage1_string_dispatch_trace_enabled() {
        eprintln!(
            "[stage1/string_dispatch] op={}({}) a0={} a1={} a2={} out={}",
            string_op_name(op),
            op,
            a0,
            a1,
            a2,
            out
        );
    }
    Some(out)
}

#[inline(always)]
pub(crate) fn string_dispatch_fn() -> Option<HakoStringDispatchFn> {
    let raw = STRING_DISPATCH_FN.load(Ordering::Acquire);
    if raw == 0 {
        None
    } else {
        Some(unsafe { std::mem::transmute(raw) })
    }
}

pub(crate) fn register_future_spawn_instance(f: Option<HakoFutureSpawnInstanceFn>) -> i64 {
    let raw = f.map(|fp| fp as usize).unwrap_or(0);
    FUTURE_SPAWN_INSTANCE_FN.store(raw, Ordering::Release);
    // SAFETY: function pointer is passed through to C registry as an opaque callback.
    unsafe { ffi::nyrt_hako_register_future_spawn_instance(f) }
}

pub(crate) fn register_string_dispatch(f: Option<HakoStringDispatchFn>) -> i64 {
    let raw = f.map(|fp| fp as usize).unwrap_or(0);
    STRING_DISPATCH_FN.store(raw, Ordering::Release);
    // SAFETY: function pointer is passed through to C registry as an opaque callback.
    unsafe { ffi::nyrt_hako_register_string_dispatch(f) }
}

/// Mainline host-service fallback policy shared by hookable entrypoints.
///
/// `NYASH_VM_USE_FALLBACK=0` means "do not execute Rust fallback routes"
/// when a `.hako` hook is not registered.
#[inline]
pub(crate) fn rust_fallback_allowed() -> bool {
    nyash_rust::config::env::vm_compat_fallback_allowed()
}

#[inline]
fn trace_hook_miss(route: &str, policy: &str) {
    if nyash_rust::config::env::vm_route_trace() {
        eprintln!("[hako-forward/hook-miss] route={} policy={}", route, policy);
    }
}

/// Canonical error code when a hook-capable scalar route misses registration
/// while `NYASH_VM_USE_FALLBACK=0`.
pub(crate) const NYRT_E_HOOK_MISS: i64 = -0x4E59_0001;

#[inline]
pub(crate) fn hook_miss_error_code(route: &str) -> i64 {
    trace_hook_miss(route, "error_code_on_fallback_off");
    NYRT_E_HOOK_MISS
}

#[inline]
pub(crate) fn hook_miss_freeze_handle(route: &str) -> i64 {
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::runtime::host_handles;
    use std::sync::Arc;

    trace_hook_miss(route, "freeze_handle_on_fallback_off");
    let msg = format!(
        "[freeze:contract][hako_forward/hook_miss] route={} require=hook_registered_or_NYASH_VM_USE_FALLBACK=1",
        route
    );
    let boxed: Arc<dyn NyashBox> = Arc::new(StringBox::new(msg));
    host_handles::to_handle_arc(boxed) as i64
}

#[cfg(test)]
pub(crate) fn reset_for_tests() {
    let _ = register_future_spawn_instance(None);
    let _ = register_string_dispatch(None);
    FUTURE_SPAWN_INSTANCE_FN.store(0, Ordering::Release);
    STRING_DISPATCH_FN.store(0, Ordering::Release);
}

#[cfg(test)]
static TEST_FORWARD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
pub(crate) fn with_test_reset<F: FnOnce()>(f: F) {
    let _guard = TEST_FORWARD_LOCK.lock().expect("forward lock");
    reset_for_tests();
    f();
    reset_for_tests();
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn future_stub(a0: i64, a1: i64, a2: i64, argc: i64) -> i64 {
        a0 + a1 + a2 + argc
    }

    extern "C" fn string_stub(op: i64, a0: i64, a1: i64, a2: i64) -> i64 {
        op * 1000 + a0 + a1 + a2
    }

    #[test]
    fn hako_forward_registration_and_call_contract() {
        with_test_reset(|| {
            assert!(call_future_spawn_instance(1, 2, 3, 4).is_none());
            assert!(call_string_dispatch(1, 2, 3, 4).is_none());

            assert_eq!(register_future_spawn_instance(Some(future_stub)), 1);
            assert_eq!(register_string_dispatch(Some(string_stub)), 1);

            assert_eq!(call_future_spawn_instance(1, 2, 3, 4), Some(10));
            assert_eq!(
                call_string_dispatch(string_ops::CONCAT_HH, 1, 2, 3),
                Some(3006)
            );
        });
    }
}
