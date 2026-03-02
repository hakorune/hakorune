/*!
 * Host Handle Registry (global)
 *
 * 目的:
 * - C ABI(TLV)でユーザー/内蔵Boxを渡すためのホスト管理ハンドルを提供。
 * - u64ハンドルID → Arc<dyn NyashBox> をグローバルに保持し、VM/PluginHost/JITから参照可能にする。
 */

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use crate::box_trait::NyashBox;
use crate::config::env::HostHandleAllocPolicyMode;

trait HostHandleAllocPolicy {
    fn take_reusable_handle(free: &mut Vec<u64>) -> Option<u64>;
    fn issue_fresh_handle(next: &mut u64) -> u64;
    fn recycle_handle(free: &mut Vec<u64>, handle: u64);
}

struct DefaultHostHandleAllocPolicy;

impl HostHandleAllocPolicy for DefaultHostHandleAllocPolicy {
    #[inline(always)]
    fn take_reusable_handle(free: &mut Vec<u64>) -> Option<u64> {
        free.pop()
    }

    #[inline(always)]
    fn issue_fresh_handle(next: &mut u64) -> u64 {
        let handle = *next;
        *next = next
            .checked_add(1)
            .expect("[host_handles] fresh handle counter overflow");
        handle
    }

    #[inline(always)]
    fn recycle_handle(free: &mut Vec<u64>, handle: u64) {
        free.push(handle);
    }
}

struct NoReuseHostHandleAllocPolicy;

impl HostHandleAllocPolicy for NoReuseHostHandleAllocPolicy {
    #[inline(always)]
    fn take_reusable_handle(_free: &mut Vec<u64>) -> Option<u64> {
        None
    }

    #[inline(always)]
    fn issue_fresh_handle(next: &mut u64) -> u64 {
        DefaultHostHandleAllocPolicy::issue_fresh_handle(next)
    }

    #[inline(always)]
    fn recycle_handle(_free: &mut Vec<u64>, _handle: u64) {}
}

#[inline(always)]
fn active_host_handle_alloc_policy_mode() -> HostHandleAllocPolicyMode {
    crate::config::env::host_handle_alloc_policy_mode()
}

#[inline(always)]
fn take_reusable_handle(mode: HostHandleAllocPolicyMode, free: &mut Vec<u64>) -> Option<u64> {
    match mode {
        HostHandleAllocPolicyMode::Lifo => DefaultHostHandleAllocPolicy::take_reusable_handle(free),
        HostHandleAllocPolicyMode::None => NoReuseHostHandleAllocPolicy::take_reusable_handle(free),
    }
}

#[inline(always)]
fn issue_fresh_handle(mode: HostHandleAllocPolicyMode, next: &mut u64) -> u64 {
    match mode {
        HostHandleAllocPolicyMode::Lifo => DefaultHostHandleAllocPolicy::issue_fresh_handle(next),
        HostHandleAllocPolicyMode::None => NoReuseHostHandleAllocPolicy::issue_fresh_handle(next),
    }
}

#[inline(always)]
fn recycle_handle(mode: HostHandleAllocPolicyMode, free: &mut Vec<u64>, handle: u64) {
    match mode {
        HostHandleAllocPolicyMode::Lifo => {
            DefaultHostHandleAllocPolicy::recycle_handle(free, handle)
        }
        HostHandleAllocPolicyMode::None => {
            NoReuseHostHandleAllocPolicy::recycle_handle(free, handle)
        }
    }
}

struct SlotTable {
    // Fresh handle counter. Updated only under table write lock.
    next: u64,
    // Dense slot table: handle ID is the direct index.
    // index 0 is reserved as empty to keep handle=0 invalid.
    slots: Vec<Option<Arc<dyn NyashBox>>>,
    // Reusable handle IDs released via drop_handle().
    // Reuse keeps slot table growth bounded under churn.
    free: Vec<u64>,
}

struct Registry {
    drop_epoch: AtomicU64,
    // slots/free are updated together under one write lock to avoid
    // double-lock overhead on alloc/drop hot paths.
    table: RwLock<SlotTable>,
    // In non-test builds policy mode is process-static (env is OnceLock-cached),
    // so keep a local copy to avoid repeated lookup on alloc/drop hot paths.
    #[cfg(not(test))]
    alloc_policy_mode: HostHandleAllocPolicyMode,
}

#[inline(always)]
fn slot_ref(table: &SlotTable, h: u64) -> Option<&Arc<dyn NyashBox>> {
    let idx = usize::try_from(h).ok()?;
    table.slots.get(idx).and_then(|slot| slot.as_ref())
}

#[inline(always)]
fn slot_clone(table: &SlotTable, h: u64) -> Option<Arc<dyn NyashBox>> {
    slot_ref(table, h).cloned()
}

#[inline(always)]
fn slot_str_ref<'a>(table: &'a SlotTable, h: u64) -> Option<&'a str> {
    slot_ref(table, h).and_then(|obj| obj.as_ref().as_str_fast())
}

impl Registry {
    fn new() -> Self {
        #[cfg(not(test))]
        let alloc_policy_mode = active_host_handle_alloc_policy_mode();
        // Perf lane notes:
        // string-heavy kernels allocate/drop many transient handles.
        // Start denser to reduce growth realloc spikes on hot paths.
        let mut slots = Vec::with_capacity(131072);
        slots.push(None);
        Self {
            drop_epoch: AtomicU64::new(0),
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
            active_host_handle_alloc_policy_mode()
        }
        #[cfg(not(test))]
        {
            self.alloc_policy_mode
        }
    }

    fn alloc(&self, obj: Arc<dyn NyashBox>) -> u64 {
        let mut table = self.table.write();
        let policy_mode = self.alloc_policy_mode();
        if let Some(h) = take_reusable_handle(policy_mode, &mut table.free) {
            let idx = usize::try_from(h).expect("[host_handles] reusable handle overflow");
            assert!(
                idx < table.slots.len(),
                "[host_handles] reusable handle out of slots range"
            );
            assert!(
                table.slots[idx].is_none(),
                "[host_handles] reusable handle points to occupied slot"
            );
            table.slots[idx] = Some(obj);
            return h;
        }

        let h = issue_fresh_handle(policy_mode, &mut table.next);
        let idx = usize::try_from(h).expect("[host_handles] fresh handle overflow");
        if idx == table.slots.len() {
            table.slots.push(Some(obj));
        } else {
            assert!(
                idx < table.slots.len(),
                "[host_handles] fresh handle out of slots range"
            );
            assert!(
                table.slots[idx].is_none(),
                "[host_handles] fresh handle points to occupied slot"
            );
            table.slots[idx] = Some(obj);
        }
        h
    }
    fn get(&self, h: u64) -> Option<Arc<dyn NyashBox>> {
        let table = self.table.read();
        slot_clone(&table, h)
    }

    fn with_handle<R>(&self, h: u64, f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R) -> R {
        let table = self.table.read();
        let obj = slot_ref(&table, h);
        f(obj)
    }
    fn get_pair(&self, a: u64, b: u64) -> (Option<Arc<dyn NyashBox>>, Option<Arc<dyn NyashBox>>) {
        let table = self.table.read();
        let a_obj = slot_clone(&table, a);
        let b_obj = slot_clone(&table, b);
        (a_obj, b_obj)
    }

    fn with_pair<R>(
        &self,
        a: u64,
        b: u64,
        f: impl FnOnce(Option<&Arc<dyn NyashBox>>, Option<&Arc<dyn NyashBox>>) -> R,
    ) -> R {
        let table = self.table.read();
        let a_obj = slot_ref(&table, a);
        let b_obj = slot_ref(&table, b);
        f(a_obj, b_obj)
    }

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
        f(slot_ref(&table, a), slot_ref(&table, b), slot_ref(&table, c))
    }

    fn with_str_pair<R>(&self, a: u64, b: u64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
        let table = self.table.read();
        let a = slot_str_ref(&table, a)?;
        let b = slot_str_ref(&table, b)?;
        Some(f(a, b))
    }

    fn with_str3<R>(
        &self,
        a: u64,
        b: u64,
        c: u64,
        f: impl FnOnce(&str, &str, &str) -> R,
    ) -> Option<R> {
        let table = self.table.read();
        let a = slot_str_ref(&table, a)?;
        let b = slot_str_ref(&table, b)?;
        let c = slot_str_ref(&table, c)?;
        Some(f(a, b, c))
    }

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
        let a_obj = slot_clone(&table, a);
        let b_obj = slot_clone(&table, b);
        let c_obj = slot_clone(&table, c);
        (a_obj, b_obj, c_obj)
    }
    fn snapshot(&self) -> Vec<Arc<dyn NyashBox>> {
        let table = self.table.read();
        table.slots.iter().filter_map(|slot| slot.clone()).collect()
    }
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
            recycle_handle(self.alloc_policy_mode(), &mut table.free, h);
            self.drop_epoch.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn drop_epoch(&self) -> u64 {
        self.drop_epoch.load(Ordering::Relaxed)
    }
}

static REG: OnceCell<Registry> = OnceCell::new();
fn reg() -> &'static Registry {
    REG.get_or_init(Registry::new)
}

/// Box<dyn NyashBox> → HostHandle (u64)
pub fn to_handle_box(bx: Box<dyn NyashBox>) -> u64 {
    reg().alloc(Arc::from(bx))
}
/// Arc<dyn NyashBox> → HostHandle (u64)
pub fn to_handle_arc(arc: Arc<dyn NyashBox>) -> u64 {
    reg().alloc(arc)
}
/// HostHandle(u64) → Arc<dyn NyashBox>
pub fn get(h: u64) -> Option<Arc<dyn NyashBox>> {
    reg().get(h)
}

/// Borrow handle under one registry read lock and run `f`.
/// Use this on read-only decode paths to avoid Arc clone cost.
pub fn with_handle<R>(h: u64, f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R) -> R {
    reg().with_handle(h, f)
}

/// HostHandle(u64)x2 -> Arc<dyn NyashBox>x2.
/// Uses a single registry read-lock acquisition for paired lookups.
pub fn get_pair(a: u64, b: u64) -> (Option<Arc<dyn NyashBox>>, Option<Arc<dyn NyashBox>>) {
    reg().get_pair(a, b)
}

/// Borrow pair handles under one registry read lock and run `f`.
/// Use this to avoid Arc clone cost on hot read-only routes.
pub fn with_pair<R>(
    a: u64,
    b: u64,
    f: impl FnOnce(Option<&Arc<dyn NyashBox>>, Option<&Arc<dyn NyashBox>>) -> R,
) -> R {
    reg().with_pair(a, b, f)
}

/// Borrow triple handles under one registry read lock and run `f`.
/// Use this to avoid Arc clone cost on hot read-only routes.
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
pub fn with_str_pair<R>(a: u64, b: u64, f: impl FnOnce(&str, &str) -> R) -> Option<R> {
    reg().with_str_pair(a, b, f)
}

/// Borrow triple handles and run string-only closure when all sides are string-like.
/// Returns None when any handle is missing or not string-like.
pub fn with_str3<R>(a: u64, b: u64, c: u64, f: impl FnOnce(&str, &str, &str) -> R) -> Option<R> {
    reg().with_str3(a, b, c, f)
}

/// HostHandle(u64)x3 -> Arc<dyn NyashBox>x3.
/// Uses a single registry read-lock acquisition for triple lookups.
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
pub fn snapshot() -> Vec<Arc<dyn NyashBox>> {
    reg().snapshot()
}

/// Drop a handle from the registry, decrementing its reference count
pub fn drop_handle(h: u64) {
    reg().drop_handle(h)
}

/// Monotonic epoch incremented when any handle is dropped.
/// Consumers can use this to invalidate per-thread fast caches safely.
pub fn drop_epoch() -> u64 {
    reg().drop_epoch()
}
