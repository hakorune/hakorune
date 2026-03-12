/*!
 * Weak Handle Registry (Phase 285LLVM-1)
 *
 * 目的:
 * - WeakRef のための LLVM handle 管理を提供。
 * - i64 handle (bit 63 = 1) → Weak<dyn NyashBox> をグローバルに保持。
 * - LLVM backend から FFI 経由でアクセス可能。
 *
 * Runtime 表現:
 * - Strong handle: 0x0000_0000_0000_0001 ~ 0x7FFF_FFFF_FFFF_FFFF (bit 63 = 0)
 * - Weak handle:   0x8000_0000_0000_0001 ~ 0xFFFF_FFFF_FFFF_FFFF (bit 63 = 1)
 */

use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock, Weak,
};

use crate::box_trait::NyashBox;

/// Weak handle marker (bit 63 = 1)
const WEAK_HANDLE_MARKER: u64 = 0x8000_0000_0000_0000;

/// Extract raw handle ID (clear bit 63)
#[inline]
fn extract_weak_id(handle: i64) -> u64 {
    (handle as u64) & !WEAK_HANDLE_MARKER
}

/// Mark handle as weak (set bit 63)
#[inline]
fn mark_weak_handle(id: u64) -> i64 {
    (id | WEAK_HANDLE_MARKER) as i64
}

/// Check if handle is weak (bit 63 = 1)
#[inline]
pub fn is_weak_handle(handle: i64) -> bool {
    (handle as u64 & WEAK_HANDLE_MARKER) != 0
}

struct WeakRegistry {
    next: AtomicU64,
    map: RwLock<HashMap<u64, Weak<dyn NyashBox>>>,
}

impl WeakRegistry {
    fn new() -> Self {
        Self {
            next: AtomicU64::new(1),
            map: RwLock::new(HashMap::new()),
        }
    }

    fn alloc(&self, weak: Weak<dyn NyashBox>) -> i64 {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut m) = self.map.write() {
            m.insert(id, weak);
        }
        mark_weak_handle(id)
    }

    fn get(&self, handle: i64) -> Option<Weak<dyn NyashBox>> {
        let id = extract_weak_id(handle);
        self.map.read().ok().and_then(|m| m.get(&id).cloned())
    }

    fn drop_handle(&self, handle: i64) {
        let id = extract_weak_id(handle);
        if let Ok(mut m) = self.map.write() {
            m.remove(&id);
        }
    }
}

static WEAK_REG: OnceCell<WeakRegistry> = OnceCell::new();
fn weak_reg() -> &'static WeakRegistry {
    WEAK_REG.get_or_init(WeakRegistry::new)
}

/// Weak<dyn NyashBox> → Weak Handle (i64, bit 63 = 1)
pub fn to_handle_weak(weak: Weak<dyn NyashBox>) -> i64 {
    weak_reg().alloc(weak)
}

/// Weak Handle (i64) → Weak<dyn NyashBox>
pub fn get_weak(handle: i64) -> Option<Weak<dyn NyashBox>> {
    weak_reg().get(handle)
}

/// Drop weak handle (release from registry)
pub fn drop_weak_handle(handle: i64) {
    weak_reg().drop_handle(handle)
}

/// Upgrade weak handle to strong handle
/// Returns: strong handle (>0) on success, 0 (Void) on failure
pub fn upgrade_weak_handle(weak_handle: i64) -> i64 {
    if let Some(weak) = get_weak(weak_handle) {
        if let Some(arc) = weak.upgrade() {
            return crate::runtime::host_handles::to_handle_arc(arc) as i64;
        }
    }
    0 // Void (null)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::StringBox;
    use std::sync::Arc;

    #[test]
    fn test_weak_handle_marker() {
        let strong_handle = 0x0000_0000_0000_0001i64;
        let weak_handle = 0x8000_0000_0000_0001u64 as i64;

        assert!(!is_weak_handle(strong_handle));
        assert!(is_weak_handle(weak_handle));
    }

    #[test]
    fn test_weak_handle_lifecycle() {
        let arc: Arc<dyn NyashBox> = Arc::new(StringBox::new("test"));
        let weak = Arc::downgrade(&arc);

        // Allocate weak handle
        let weak_handle = to_handle_weak(weak.clone());
        assert!(is_weak_handle(weak_handle));

        // Upgrade should succeed (arc is alive)
        let strong_handle = upgrade_weak_handle(weak_handle);
        assert!(strong_handle > 0);

        crate::runtime::host_handles::drop_handle(strong_handle as u64);

        // Drop arc
        drop(arc);

        // Upgrade should fail (arc is dead)
        let result = upgrade_weak_handle(weak_handle);
        assert_eq!(result, 0); // Void

        // Cleanup
        drop_weak_handle(weak_handle);
    }
}
