//! Lease-specific host-handle identity owner.
//!
//! The generic host-handle registry remains in the parent module.  This child
//! owns only the generation-branded identity and the one lock transition used
//! by the DynamicV2 End-authorized result; it does not define lease tokens.

use super::{slot_ref, HandlePayload, Registry};

/// Generation-branded identity owned by the reusable host-handle table.
///
/// This is separate from the legacy `BoxIdentity` projection: lease End must
/// distinguish a reused raw slot while the public object identity surface
/// remains compatibility-shaped.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct HostHandleLeaseIdentityV1 {
    pub(super) handle: u64,
    pub(super) generation: u64,
}

impl HostHandleLeaseIdentityV1 {
    #[inline(always)]
    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }
}

impl Registry {
    /// Allocate a text handle and return its generation identity while the
    /// same host-handle table write lock is still held by the allocation path.
    #[inline(always)]
    pub(super) fn alloc_text_with_lease_identity(
        &self,
        text: String,
    ) -> (u64, HostHandleLeaseIdentityV1) {
        let (handle, generation) =
            self.alloc_payload_with_generation(HandlePayload::StableText(text));
        (handle, HostHandleLeaseIdentityV1 { handle, generation })
    }

    #[inline(always)]
    fn capture_text_lease_identity(&self, h: u64) -> Option<HostHandleLeaseIdentityV1> {
        let table = self.table.read();
        let payload = slot_ref(&table, h)?;
        payload.as_str_fast()?;
        let idx = usize::try_from(h).ok()?;
        let generation = table.lease_generations.get(idx).copied()?;
        (generation != 0).then_some(HostHandleLeaseIdentityV1 {
            handle: h,
            generation,
        })
    }

    #[inline(always)]
    fn drop_if_lease_identity_matches(&self, identity: HostHandleLeaseIdentityV1) -> bool {
        let mut table = self.table.write();
        let Ok(idx) = usize::try_from(identity.handle) else {
            return false;
        };
        let matches = table.lease_generations.get(idx).copied() == Some(identity.generation)
            && table.slots.get(idx).is_some_and(Option::is_some);
        if !matches {
            return false;
        }
        table.slots[idx] = None;
        super::host_handles_policy::recycle_handle(
            self.alloc_policy_mode(),
            &mut table.free,
            identity.handle,
        );
        super::DROP_EPOCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        true
    }
}

#[inline(always)]
pub(crate) fn to_handle_text_with_lease_identity(
    text: impl Into<String>,
) -> (u64, HostHandleLeaseIdentityV1) {
    super::reg().alloc_text_with_lease_identity(text.into())
}

/// Capture the live identity used by the one-shot DynamicV2 lease owner.
#[inline(always)]
pub(crate) fn capture_text_lease_identity(h: u64) -> Option<HostHandleLeaseIdentityV1> {
    super::reg().capture_text_lease_identity(h)
}

/// Drop only if the raw slot still contains the captured lease identity.
/// The comparison and removal happen under one slot-table write lock.
#[inline(always)]
pub(crate) fn drop_if_lease_identity_matches(identity: HostHandleLeaseIdentityV1) -> bool {
    super::reg().drop_if_lease_identity_matches(identity)
}
