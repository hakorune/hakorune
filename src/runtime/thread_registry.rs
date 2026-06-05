//! Runtime thread registry substrate.
//!
//! This registry tracks runtime worker threads for diagnostics and future
//! root cleanup. It does not authorize Box movement/sharing and is not a
//! source-level thread surface.

use crate::runtime::ring0::HostThreadId;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkerId(u64);

impl WorkerId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn id(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadRegistryRole {
    RuntimeWorker,
    HostThread,
    Test,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadRegistration {
    pub worker_id: WorkerId,
    pub host_thread_id: HostThreadId,
    pub role: ThreadRegistryRole,
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadRegistryError {
    RegistryPoisoned,
}

#[derive(Debug, Default)]
pub struct ThreadRegistrySnapshot {
    pub registrations: Vec<ThreadRegistration>,
}

impl ThreadRegistrySnapshot {
    pub fn len(&self) -> usize {
        self.registrations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.registrations.is_empty()
    }
}

#[derive(Debug)]
pub struct ThreadRegistry {
    next_worker_id: AtomicU64,
    registrations: Mutex<HashMap<HostThreadId, ThreadRegistration>>,
}

impl Default for ThreadRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadRegistry {
    pub fn new() -> Self {
        Self {
            next_worker_id: AtomicU64::new(1),
            registrations: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_current_thread(
        &self,
        host_thread_id: HostThreadId,
        role: ThreadRegistryRole,
        name: Option<String>,
    ) -> Result<WorkerId, ThreadRegistryError> {
        let mut registrations = self
            .registrations
            .lock()
            .map_err(|_| ThreadRegistryError::RegistryPoisoned)?;
        if let Some(existing) = registrations.get(&host_thread_id) {
            return Ok(existing.worker_id);
        }

        let worker_id = WorkerId::new(self.next_worker_id.fetch_add(1, Ordering::Relaxed));
        registrations.insert(
            host_thread_id,
            ThreadRegistration {
                worker_id,
                host_thread_id,
                role,
                name,
            },
        );
        Ok(worker_id)
    }

    pub fn unregister_current_thread(
        &self,
        host_thread_id: HostThreadId,
    ) -> Result<Option<ThreadRegistration>, ThreadRegistryError> {
        let mut registrations = self
            .registrations
            .lock()
            .map_err(|_| ThreadRegistryError::RegistryPoisoned)?;
        Ok(registrations.remove(&host_thread_id))
    }

    pub fn snapshot(&self) -> Result<ThreadRegistrySnapshot, ThreadRegistryError> {
        let registrations = self
            .registrations
            .lock()
            .map_err(|_| ThreadRegistryError::RegistryPoisoned)?;
        let mut registrations: Vec<_> = registrations.values().cloned().collect();
        registrations.sort_by_key(|registration| registration.worker_id.id());
        Ok(ThreadRegistrySnapshot { registrations })
    }
}

static GLOBAL_THREAD_REGISTRY: OnceLock<Arc<ThreadRegistry>> = OnceLock::new();

pub fn global_thread_registry() -> Arc<ThreadRegistry> {
    GLOBAL_THREAD_REGISTRY
        .get_or_init(|| Arc::new(ThreadRegistry::new()))
        .clone()
}

#[cfg(test)]
pub(crate) fn reset_global_thread_registry_for_tests() {
    if let Some(registry) = GLOBAL_THREAD_REGISTRY.get() {
        if let Ok(mut registrations) = registry.registrations.lock() {
            registrations.clear();
        }
        registry.next_worker_id.store(1, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_registry_registers_and_unregisters_thread() {
        let registry = ThreadRegistry::new();
        let worker_id = registry
            .register_current_thread(
                100,
                ThreadRegistryRole::RuntimeWorker,
                Some("worker-0".to_string()),
            )
            .expect("register should succeed");

        assert_eq!(worker_id.id(), 1);
        let snapshot = registry.snapshot().expect("snapshot should succeed");
        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot.registrations[0].host_thread_id, 100);

        let removed = registry
            .unregister_current_thread(100)
            .expect("unregister should succeed");
        assert!(removed.is_some());
        assert!(registry
            .snapshot()
            .expect("snapshot should succeed")
            .is_empty());
    }

    #[test]
    fn thread_registry_keeps_worker_id_stable_for_same_host_thread() {
        let registry = ThreadRegistry::new();

        let first = registry
            .register_current_thread(100, ThreadRegistryRole::RuntimeWorker, None)
            .expect("first register should succeed");
        let second = registry
            .register_current_thread(100, ThreadRegistryRole::RuntimeWorker, None)
            .expect("second register should succeed");

        assert_eq!(first, second);
        assert_eq!(
            registry.snapshot().expect("snapshot should succeed").len(),
            1
        );
    }
}
