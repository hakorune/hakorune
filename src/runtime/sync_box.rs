//! Reference runtime support for `sync box` serialized entry.
//!
//! This module owns the runtime-side reference shape only. It does not lower
//! `sync box` through MIR, Program JSON, or LLVM, and it does not expose raw
//! locks as source-level values.

use std::cell::RefCell;
use std::fmt;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncBoxError {
    ReentrantEntry {
        object_id: u64,
        method_name: String,
        active_object_id: u64,
        active_method_name: String,
    },
    Poisoned {
        object_id: u64,
        method_name: String,
    },
}

impl SyncBoxError {
    pub const fn tag(&self) -> &'static str {
        match self {
            Self::ReentrantEntry { .. } => "[syncbox/reentrant-entry]",
            Self::Poisoned { .. } => "[syncbox/poisoned]",
        }
    }
}

impl fmt::Display for SyncBoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReentrantEntry {
                object_id,
                method_name,
                active_object_id,
                active_method_name,
            } => write!(
                f,
                "{} object={} method={} active_object={} active_method={}",
                self.tag(),
                object_id,
                method_name,
                active_object_id,
                active_method_name
            ),
            Self::Poisoned {
                object_id,
                method_name,
            } => write!(
                f,
                "{} object={} method={}",
                self.tag(),
                object_id,
                method_name
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SyncEntry {
    object_id: u64,
    method_name: String,
}

thread_local! {
    static SYNC_ENTRY_STACK: RefCell<Vec<SyncEntry>> = const { RefCell::new(Vec::new()) };
}

/// Per-instance serialized-entry state for future `sync box` objects.
#[derive(Debug, Default)]
pub struct SyncState {
    mutex: Mutex<()>,
}

impl SyncState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter<'a>(
        &'a self,
        object_id: u64,
        method_name: &str,
    ) -> Result<SyncEntryGuard<'a>, SyncBoxError> {
        let active = SYNC_ENTRY_STACK.with(|stack| stack.borrow().last().cloned());
        if let Some(active) = active {
            return Err(SyncBoxError::ReentrantEntry {
                object_id,
                method_name: method_name.to_string(),
                active_object_id: active.object_id,
                active_method_name: active.method_name,
            });
        }

        let guard = self.mutex.lock().map_err(|_| SyncBoxError::Poisoned {
            object_id,
            method_name: method_name.to_string(),
        })?;

        SYNC_ENTRY_STACK.with(|stack| {
            stack.borrow_mut().push(SyncEntry {
                object_id,
                method_name: method_name.to_string(),
            });
        });

        Ok(SyncEntryGuard {
            object_id,
            method_name: method_name.to_string(),
            _guard: guard,
        })
    }

    pub fn current_entry_depth() -> usize {
        SYNC_ENTRY_STACK.with(|stack| stack.borrow().len())
    }
}

#[derive(Debug)]
pub struct SyncEntryGuard<'a> {
    object_id: u64,
    method_name: String,
    _guard: MutexGuard<'a, ()>,
}

impl Drop for SyncEntryGuard<'_> {
    fn drop(&mut self) {
        SYNC_ENTRY_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            let popped = stack.pop();
            debug_assert_eq!(
                popped,
                Some(SyncEntry {
                    object_id: self.object_id,
                    method_name: self.method_name.clone(),
                })
            );
        });
    }
}

pub fn sync_box_reference_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("sync_box_reference_runtime_enabled", "1"),
        ("sync_box_mir_lowering_enabled", "0"),
        ("sync_box_program_json_enabled", "0"),
        ("sync_box_llvm_enabled", "0"),
        ("sync_box_fairness_guarantee", "0"),
        ("sync_box_reentrancy_guarantee", "0"),
        ("sync_box_lock_order_verifier_enabled", "0"),
        ("sync_box_worker_pool_route_enabled", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sync_state_allows_no_contention_entry() {
        let state = SyncState::new();
        assert_eq!(SyncState::current_entry_depth(), 0);
        {
            let _guard = state.enter(1, "inc").expect("entry should succeed");
            assert_eq!(SyncState::current_entry_depth(), 1);
        }
        assert_eq!(SyncState::current_entry_depth(), 0);
    }

    #[test]
    fn sync_state_rejects_same_instance_reentry() {
        let state = SyncState::new();
        let _guard = state.enter(1, "inc").expect("entry should succeed");
        let error = state
            .enter(1, "get")
            .expect_err("reentrant sync call must fail-fast");

        assert_eq!(error.tag(), "[syncbox/reentrant-entry]");
        assert_eq!(
            error,
            SyncBoxError::ReentrantEntry {
                object_id: 1,
                method_name: "get".to_string(),
                active_object_id: 1,
                active_method_name: "inc".to_string(),
            }
        );
    }

    #[test]
    fn sync_state_rejects_nested_other_instance_entry_for_v0() {
        let first = SyncState::new();
        let second = SyncState::new();
        let _guard = first.enter(1, "inc").expect("entry should succeed");
        let error = second
            .enter(2, "get")
            .expect_err("nested sync entry must fail-fast in v0");

        assert_eq!(error.tag(), "[syncbox/reentrant-entry]");
        assert_eq!(
            error,
            SyncBoxError::ReentrantEntry {
                object_id: 2,
                method_name: "get".to_string(),
                active_object_id: 1,
                active_method_name: "inc".to_string(),
            }
        );
    }

    #[test]
    fn sync_box_reference_report_keeps_backends_closed() {
        assert_eq!(
            sync_box_reference_report_fields(),
            vec![
                ("sync_box_reference_runtime_enabled", "1"),
                ("sync_box_mir_lowering_enabled", "0"),
                ("sync_box_program_json_enabled", "0"),
                ("sync_box_llvm_enabled", "0"),
                ("sync_box_fairness_guarantee", "0"),
                ("sync_box_reentrancy_guarantee", "0"),
                ("sync_box_lock_order_verifier_enabled", "0"),
                ("sync_box_worker_pool_route_enabled", "0"),
            ]
        );
    }
}
