/*!
 * Host Handle Registry (global)
 *
 * 目的:
 * - C ABI(TLV)でユーザー/内蔵Boxを渡すためのホスト管理ハンドルを提供。
 * - u64ハンドルID → Arc<dyn NyashBox> をグローバルに保持し、VM/PluginHost/JITから参照可能にする。
 */

#[path = "host_handles/perf_observe.rs"]
mod perf_observe;
pub use perf_observe::PerfObserveSnapshot;
#[path = "host_handles/text_read.rs"]
mod text_read;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::cell::RefCell;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use super::host_handles_policy;
use crate::box_trait::NyashBox;
use crate::config::env::HostHandleAllocPolicyMode;
pub use perf_observe::ObjectWithHandleCaller as PerfObserveObjectWithHandleCaller;
pub use text_read::TextReadSession;

#[derive(Clone)]
struct LatestFreshStableBox {
    handle: u64,
    drop_epoch: u64,
    object: Arc<dyn NyashBox>,
}

thread_local! {
    static LATEST_FRESH_STABLE_BOX: RefCell<Option<LatestFreshStableBox>> = const { RefCell::new(None) };
}

enum HandlePayload {
    StableBox(Arc<dyn NyashBox>),
}

impl HandlePayload {
    #[inline(always)]
    fn stable_box_ref(&self) -> &Arc<dyn NyashBox> {
        match self {
            Self::StableBox(obj) => obj,
        }
    }

    #[inline(always)]
    fn cloned_stable_box(&self) -> Arc<dyn NyashBox> {
        self.stable_box_ref().clone()
    }

    #[inline(always)]
    fn as_str_fast(&self) -> Option<&str> {
        self.stable_box_ref().as_ref().as_str_fast()
    }
}

struct SlotTable {
    // Fresh handle counter. Updated only under table write lock.
    next: u64,
    // Dense slot table: handle ID is the direct index.
    // index 0 is reserved as empty to keep handle=0 invalid.
    slots: Vec<Option<HandlePayload>>,
    // Reusable handle IDs released via drop_handle().
    // Reuse keeps slot table growth bounded under churn.
    free: Vec<u64>,
}

struct Registry {
    // slots/free are updated together under one write lock to avoid
    // double-lock overhead on alloc/drop hot paths.
    table: RwLock<SlotTable>,
    // In non-test builds policy mode is process-static (env is OnceLock-cached),
    // so keep a local copy to avoid repeated lookup on alloc/drop hot paths.
    #[cfg(not(test))]
    alloc_policy_mode: HostHandleAllocPolicyMode,
}

#[inline(always)]
fn slot_ref(table: &SlotTable, h: u64) -> Option<&HandlePayload> {
    let idx = usize::try_from(h).ok()?;
    table.slots.get(idx).and_then(|slot| slot.as_ref())
}

#[cold]
#[inline(never)]
fn host_handle_panic(message: &'static str) -> ! {
    panic!("{}", message);
}

#[inline(always)]
fn handle_index_or_panic(handle: u64, overflow_message: &'static str) -> usize {
    usize::try_from(handle).unwrap_or_else(|_| host_handle_panic(overflow_message))
}

#[inline(always)]
fn ensure_slot_vacant_or_panic(
    table: &SlotTable,
    idx: usize,
    range_message: &'static str,
    occupied_message: &'static str,
) {
    if idx >= table.slots.len() {
        host_handle_panic(range_message);
    }
    if table.slots[idx].is_some() {
        host_handle_panic(occupied_message);
    }
}

impl Registry {
    fn new() -> Self {
        #[cfg(not(test))]
        let alloc_policy_mode = host_handles_policy::active_host_handle_alloc_policy_mode();
        // Perf lane notes:
        // string-heavy kernels allocate/drop many transient handles.
        // Start denser to reduce growth realloc spikes on hot paths.
        let mut slots = Vec::with_capacity(131072);
        slots.push(None);
        Self {
            table: RwLock::new(SlotTable {
                next: 1,
                slots,
                free: Vec::with_capacity(65536),
            }),
            #[cfg(not(test))]
            alloc_policy_mode,
        }
    }

    #[inline(always)]
    fn alloc_policy_mode(&self) -> HostHandleAllocPolicyMode {
        #[cfg(test)]
        {
            host_handles_policy::active_host_handle_alloc_policy_mode()
        }
        #[cfg(not(test))]
        {
            self.alloc_policy_mode
        }
    }

    #[inline(always)]
    fn alloc(&self, obj: Arc<dyn NyashBox>) -> u64 {
        let policy_mode = self.alloc_policy_mode();
        let mut table = self.table.write();
        let payload = HandlePayload::StableBox(obj);

        if let Some(h) = host_handles_policy::take_reusable_handle(policy_mode, &mut table.free) {
            let idx = handle_index_or_panic(h, "[host_handles] reusable handle overflow");
            ensure_slot_vacant_or_panic(
                &table,
                idx,
                "[host_handles] reusable handle out of slots range",
                "[host_handles] reusable handle points to occupied slot",
            );
            table.slots[idx] = Some(payload);
            return h;
        }

        let h = host_handles_policy::issue_fresh_handle(policy_mode, &mut table.next);
        let idx = handle_index_or_panic(h, "[host_handles] fresh handle overflow");
        if idx == table.slots.len() {
            table.slots.push(Some(payload));
        } else {
            ensure_slot_vacant_or_panic(
                &table,
                idx,
                "[host_handles] fresh handle out of slots range",
                "[host_handles] fresh handle points to occupied slot",
            );
            table.slots[idx] = Some(payload);
        }
        h
    }
    #[inline(always)]
    fn get(&self, h: u64) -> Option<Arc<dyn NyashBox>> {
        let table = self.table.read();
        let out = slot_ref(&table, h).map(HandlePayload::cloned_stable_box);
        if out.is_some() {
            perf_observe::object_get(h);
        }
        out
    }

    #[inline(always)]
    fn with_handle<R>(&self, h: u64, f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R) -> R {
        let table = self.table.read();
        let obj = slot_ref(&table, h).map(HandlePayload::stable_box_ref);
        if obj.is_some() {
            perf_observe::object_with_handle(h, PerfObserveObjectWithHandleCaller::Generic);
        }
        f(obj)
    }
    #[inline(always)]
    fn get_pair(&self, a: u64, b: u64) -> (Option<Arc<dyn NyashBox>>, Option<Arc<dyn NyashBox>>) {
        let table = self.table.read();
        let a_obj = slot_ref(&table, a).map(HandlePayload::cloned_stable_box);
        let b_obj = slot_ref(&table, b).map(HandlePayload::cloned_stable_box);
        if a_obj.is_some() || b_obj.is_some() {
            perf_observe::object_pair(a, b);
        }
        (a_obj, b_obj)
    }

    #[inline(always)]
    fn with_pair<R>(
        &self,
        a: u64,
        b: u64,
        f: impl FnOnce(Option<&Arc<dyn NyashBox>>, Option<&Arc<dyn NyashBox>>) -> R,
    ) -> R {
        let table = self.table.read();
        let a_obj = slot_ref(&table, a).map(HandlePayload::stable_box_ref);
        let b_obj = slot_ref(&table, b).map(HandlePayload::stable_box_ref);
        if a_obj.is_some() || b_obj.is_some() {
            perf_observe::object_pair(a, b);
        }
        f(a_obj, b_obj)
    }

    #[inline(always)]
    fn with3<R>(
        &self,
        a: u64,
        b: u64,
        c: u64,
        f: impl FnOnce(
            Option<&Arc<dyn NyashBox>>,
            Option<&Arc<dyn NyashBox>>,
            Option<&Arc<dyn NyashBox>>,
        ) -> R,
    ) -> R {
        let table = self.table.read();
        let a_obj = slot_ref(&table, a).map(HandlePayload::stable_box_ref);
        let b_obj = slot_ref(&table, b).map(HandlePayload::stable_box_ref);
        let c_obj = slot_ref(&table, c).map(HandlePayload::stable_box_ref);
        if a_obj.is_some() || b_obj.is_some() || c_obj.is_some() {
            perf_observe::object_triple(a, b, c);
        }
        f(a_obj, b_obj, c_obj)
    }

    #[inline(always)]
    fn with_str_pair<R>(&self, a: u64, b: u64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
        self.with_text_read_session(|session| session.str_pair(a, b, f))
    }

    #[inline(always)]
    fn with_str_handle<R>(&self, h: u64, f: impl FnOnce(&str) -> R) -> Option<R> {
        self.with_text_read_session(|session| session.str_handle(h, f))
    }

    #[inline(always)]
    fn with_text_read_session<R>(&self, f: impl FnOnce(TextReadSession<'_>) -> R) -> R {
        let table = self.table.read();
        f(TextReadSession::new(&table))
    }

    #[inline(always)]
    fn with_str3<R>(
        &self,
        a: u64,
        b: u64,
        c: u64,
        f: impl FnOnce(&str, &str, &str) -> R,
    ) -> Option<R> {
        self.with_text_read_session(|session| session.str3(a, b, c, f))
    }

    #[inline(always)]
    fn get3(
        &self,
        a: u64,
        b: u64,
        c: u64,
    ) -> (
        Option<Arc<dyn NyashBox>>,
        Option<Arc<dyn NyashBox>>,
        Option<Arc<dyn NyashBox>>,
    ) {
        let table = self.table.read();
        let a_obj = slot_ref(&table, a).map(HandlePayload::cloned_stable_box);
        let b_obj = slot_ref(&table, b).map(HandlePayload::cloned_stable_box);
        let c_obj = slot_ref(&table, c).map(HandlePayload::cloned_stable_box);
        if a_obj.is_some() || b_obj.is_some() || c_obj.is_some() {
            perf_observe::object_triple(a, b, c);
        }
        (a_obj, b_obj, c_obj)
    }
    #[inline(always)]
    fn snapshot(&self) -> Vec<Arc<dyn NyashBox>> {
        let table = self.table.read();
        table
            .slots
            .iter()
            .filter_map(|slot| slot.as_ref().map(HandlePayload::cloned_stable_box))
            .collect()
    }
    #[inline(always)]
    fn drop_handle(&self, h: u64) {
        let mut table = self.table.write();
        let removed = if let Ok(idx) = usize::try_from(h) {
            table
                .slots
                .get_mut(idx)
                .and_then(|slot| slot.take())
                .is_some()
        } else {
            false
        };
        if removed {
            host_handles_policy::recycle_handle(self.alloc_policy_mode(), &mut table.free, h);
            DROP_EPOCH.fetch_add(1, Ordering::Relaxed);
        }
    }
}

static DROP_EPOCH: AtomicU64 = AtomicU64::new(0);
static REG: OnceCell<Registry> = OnceCell::new();
#[inline(always)]
fn reg() -> &'static Registry {
    REG.get_or_init(Registry::new)
}

#[inline(always)]
fn remember_latest_fresh_stable_box(handle: u64, object: Arc<dyn NyashBox>) {
    let drop_epoch = DROP_EPOCH.load(Ordering::Relaxed);
    LATEST_FRESH_STABLE_BOX.with(|slot| {
        *slot.borrow_mut() = Some(LatestFreshStableBox {
            handle,
            drop_epoch,
            object,
        });
    });
}

/// Box<dyn NyashBox> → HostHandle (u64)
#[inline(always)]
pub fn to_handle_box(bx: Box<dyn NyashBox>) -> u64 {
    to_handle_arc(Arc::from(bx))
}
/// Arc<dyn NyashBox> → HostHandle (u64)
#[inline(always)]
pub fn to_handle_arc(arc: Arc<dyn NyashBox>) -> u64 {
    let handle = reg().alloc(arc.clone());
    remember_latest_fresh_stable_box(handle, arc);
    handle
}

#[inline(always)]
pub fn perf_observe_mark_latest_fresh_handle(h: u64) {
    perf_observe::mark_latest_fresh_handle(h);
}

#[inline(always)]
pub fn perf_observe_snapshot() -> PerfObserveSnapshot {
    perf_observe::snapshot()
}

#[inline(always)]
pub fn perf_observe_object_with_handle_caller(h: u64, caller: PerfObserveObjectWithHandleCaller) {
    perf_observe::object_with_handle(h, caller);
}
/// HostHandle(u64) → Arc<dyn NyashBox>
#[inline(always)]
pub fn get(h: u64) -> Option<Arc<dyn NyashBox>> {
    reg().get(h)
}

/// Borrow handle under one registry read lock and run `f`.
/// Use this on read-only decode paths to avoid Arc clone cost.
#[inline(always)]
pub fn with_handle<R>(h: u64, f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R) -> R {
    reg().with_handle(h, f)
}

#[inline(always)]
pub fn with_handle_caller<R>(
    h: u64,
    caller: PerfObserveObjectWithHandleCaller,
    f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R,
) -> R {
    let table = reg().table.read();
    let obj = slot_ref(&table, h).map(HandlePayload::stable_box_ref);
    if obj.is_some() {
        perf_observe::object_with_handle(h, caller);
    }
    f(obj)
}

#[inline(always)]
pub fn with_latest_fresh_stable_box<R>(
    h: u64,
    f: impl FnOnce(&Arc<dyn NyashBox>) -> R,
) -> Option<R> {
    let current_epoch = DROP_EPOCH.load(Ordering::Relaxed);
    LATEST_FRESH_STABLE_BOX.with(|slot| {
        let cached = slot.borrow();
        let entry = cached
            .as_ref()
            .filter(|entry| entry.handle == h && entry.drop_epoch == current_epoch)?;
        Some(f(&entry.object))
    })
}

/// HostHandle(u64) -> borrowed &str under one registry read lock.
#[inline(always)]
pub fn with_str_handle<R>(h: u64, f: impl FnOnce(&str) -> R) -> Option<R> {
    reg().with_str_handle(h, f)
}

/// Borrow a read-only string session under one registry read lock.
/// Use this when a pure string consumer needs multiple string-like handle reads
/// without escalating into object-world APIs.
#[inline(always)]
pub fn with_text_read_session<R>(f: impl FnOnce(TextReadSession<'_>) -> R) -> R {
    reg().with_text_read_session(|session| f(session))
}

/// Borrow a read-only string session only when the registry is already initialized.
/// This avoids the `get_or_init` slow path on hot pure-string corridors.
#[inline(always)]
pub fn with_text_read_session_ready<R>(f: impl FnOnce(TextReadSession<'_>) -> R) -> Option<R> {
    let reg = REG.get()?;
    Some(reg.with_text_read_session(|session| f(session)))
}

/// HostHandle(u64)x2 -> Arc<dyn NyashBox>x2.
/// Uses a single registry read-lock acquisition for paired lookups.
#[inline(always)]
pub fn get_pair(a: u64, b: u64) -> (Option<Arc<dyn NyashBox>>, Option<Arc<dyn NyashBox>>) {
    reg().get_pair(a, b)
}

/// Borrow pair handles under one registry read lock and run `f`.
/// Use this to avoid Arc clone cost on hot read-only routes.
#[inline(always)]
pub fn with_pair<R>(
    a: u64,
    b: u64,
    f: impl FnOnce(Option<&Arc<dyn NyashBox>>, Option<&Arc<dyn NyashBox>>) -> R,
) -> R {
    reg().with_pair(a, b, f)
}

/// Borrow triple handles under one registry read lock and run `f`.
/// Use this to avoid Arc clone cost on hot read-only routes.
#[inline(always)]
pub fn with3<R>(
    a: u64,
    b: u64,
    c: u64,
    f: impl FnOnce(
        Option<&Arc<dyn NyashBox>>,
        Option<&Arc<dyn NyashBox>>,
        Option<&Arc<dyn NyashBox>>,
    ) -> R,
) -> R {
    reg().with3(a, b, c, f)
}

/// Borrow pair handles and run string-only closure when both sides are string-like.
/// Returns None when either handle is missing or not string-like.
#[inline(always)]
pub fn with_str_pair<R>(a: u64, b: u64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
    reg().with_str_pair(a, b, f)
}

/// Borrow triple handles and run string-only closure when all sides are string-like.
/// Returns None when any handle is missing or not string-like.
#[inline(always)]
pub fn with_str3<R>(a: u64, b: u64, c: u64, f: impl FnOnce(&str, &str, &str) -> R) -> Option<R> {
    reg().with_str3(a, b, c, f)
}

/// HostHandle(u64)x3 -> Arc<dyn NyashBox>x3.
/// Uses a single registry read-lock acquisition for triple lookups.
#[inline(always)]
pub fn get3(
    a: u64,
    b: u64,
    c: u64,
) -> (
    Option<Arc<dyn NyashBox>>,
    Option<Arc<dyn NyashBox>>,
    Option<Arc<dyn NyashBox>>,
) {
    reg().get3(a, b, c)
}

/// Snapshot all current handles as Arc<dyn NyashBox> roots for diagnostics/GC traversal.
#[inline(always)]
pub fn snapshot() -> Vec<Arc<dyn NyashBox>> {
    reg().snapshot()
}

/// Drop a handle from the registry, decrementing its reference count
#[inline(always)]
pub fn drop_handle(h: u64) {
    reg().drop_handle(h)
}

/// Monotonic epoch incremented when any handle is dropped.
/// Consumers can use this to invalidate per-thread fast caches safely.
#[inline(always)]
pub fn drop_epoch() -> u64 {
    DROP_EPOCH.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::IntegerBox;
    use std::sync::Mutex;

    static HOST_HANDLE_POLICY_ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_host_handle_policy_env<F: FnOnce()>(value: &str, f: F) {
        let _guard = HOST_HANDLE_POLICY_ENV_LOCK.lock().expect("env lock");
        let prev = std::env::var("NYASH_HOST_HANDLE_ALLOC_POLICY").ok();
        std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", value);
        f();
        if let Some(v) = prev {
            std::env::set_var("NYASH_HOST_HANDLE_ALLOC_POLICY", v);
        } else {
            std::env::remove_var("NYASH_HOST_HANDLE_ALLOC_POLICY");
        }
    }

    fn int_box(value: i64) -> Arc<dyn NyashBox> {
        Arc::new(IntegerBox::new(value))
    }

    #[test]
    fn host_handles_registry_lifo_reuses_dropped_handle() {
        with_host_handle_policy_env("lifo", || {
            let registry = Registry::new();
            let first = registry.alloc(int_box(1));
            registry.drop_handle(first);
            let second = registry.alloc(int_box(2));
            assert_eq!(second, first);
        });
    }

    #[test]
    fn host_handles_registry_none_issues_fresh_handle_after_drop() {
        with_host_handle_policy_env("none", || {
            let registry = Registry::new();
            let first = registry.alloc(int_box(1));
            registry.drop_handle(first);
            let second = registry.alloc(int_box(2));
            assert!(second > first);
            assert_ne!(second, first);
        });
    }

    #[test]
    fn latest_fresh_stable_box_returns_current_object() {
        let handle = to_handle_arc(int_box(41));
        let got = with_latest_fresh_stable_box(handle, |obj| {
            obj.as_any()
                .downcast_ref::<IntegerBox>()
                .expect("integer latest fresh object")
                .value
        });
        assert_eq!(got, Some(41));
    }

    #[test]
    fn latest_fresh_stable_box_invalidates_after_drop_epoch_changes() {
        let handle = to_handle_arc(int_box(52));
        assert!(with_latest_fresh_stable_box(handle, |_| ()).is_some());
        drop_handle(handle);
        assert!(with_latest_fresh_stable_box(handle, |_| ()).is_none());
    }
}
