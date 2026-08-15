//! Lease-specific host-handle identity owner.
//!
//! The generic host-handle registry remains in the parent module.  This child
//! owns only the generation-branded identity and the one lock transition used
//! by the DynamicV2 End-authorized result; it does not define lease tokens.

use super::{slot_ref, HandlePayload, Registry};

/// Mechanical rejection codes for the strict callable Text formal lane.
///
/// The semantic owner lives in `runtime::text_formal_abi`; this enum only
/// reports the slot-table facts needed by that owner and never becomes a
/// public ABI or a fallback route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalLookupRejectV1 {
    ZeroOrOutOfRangeSlot,
    MissingSlot,
    GenerationMismatch,
    NonTextPayload,
}

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

#[inline(always)]
fn exact_text_ref(payload: &HandlePayload) -> Option<&str> {
    match payload {
        // StableText is the registry's own canonical text payload.
        HandlePayload::StableText(text) => Some(text.as_str()),
        // Only the language's admitted StringBox representation is accepted
        // here.  Do not widen this to every `as_str_fast` plugin/helper.
        HandlePayload::StableBox(object) if object.type_name() == "StringBox" => {
            object.as_ref().as_str_fast()
        }
        HandlePayload::StableBox(_) => None,
    }
}

impl Registry {
    /// Capture a generation-branded Text formal identity without exposing the
    /// registry or a raw-handle-only capability.
    #[inline(always)]
    pub(super) fn capture_text_formal_pair(
        &self,
        handle: u64,
    ) -> Result<(u64, u64), TextFormalLookupRejectV1> {
        if handle == 0 {
            return Err(TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot);
        }
        let idx =
            usize::try_from(handle).map_err(|_| TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot)?;
        let table = self.table.read();
        if idx >= table.slots.len() {
            return Err(TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot);
        }
        let payload = table.slots[idx]
            .as_ref()
            .ok_or(TextFormalLookupRejectV1::MissingSlot)?;
        let generation = table.lease_generations[idx];
        if generation == 0 {
            return Err(TextFormalLookupRejectV1::MissingSlot);
        }
        exact_text_ref(payload).ok_or(TextFormalLookupRejectV1::NonTextPayload)?;
        Ok((handle, generation))
    }

    /// Validate the captured generation and lend the exact Text payload under
    /// one registry read lock.  The callback cannot outlive this borrow.
    #[inline(always)]
    pub(super) fn with_text_formal_identity<R>(
        &self,
        identity: &HostHandleLeaseIdentityV1,
        f: impl FnOnce(&str) -> R,
    ) -> Result<R, TextFormalLookupRejectV1> {
        let idx = usize::try_from(identity.handle)
            .map_err(|_| TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot)?;
        let table = self.table.read();
        if idx >= table.slots.len() {
            return Err(TextFormalLookupRejectV1::ZeroOrOutOfRangeSlot);
        }
        let payload = table.slots[idx]
            .as_ref()
            .ok_or(TextFormalLookupRejectV1::MissingSlot)?;
        if table.lease_generations[idx] != identity.generation {
            return Err(TextFormalLookupRejectV1::GenerationMismatch);
        }
        let text = exact_text_ref(payload).ok_or(TextFormalLookupRejectV1::NonTextPayload)?;
        Ok(f(text))
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

/// Capture the strict callable Text formal identity.  Unlike the DynamicV2
/// text lease helper this accepts only StableText/StringBox payloads.
#[inline(always)]
pub(crate) fn capture_text_formal_pair(
    handle: u64,
) -> Result<(u64, u64), TextFormalLookupRejectV1> {
    super::reg().capture_text_formal_pair(handle)
}

/// Validate a published wire pair without allowing callers to reconstruct a
/// generation from the raw slot.  Construction stays inside this owner.
#[inline(always)]
pub(crate) fn with_text_formal_wire<R>(
    handle: u64,
    generation: u64,
    f: impl FnOnce(&str) -> R,
) -> Result<R, TextFormalLookupRejectV1> {
    let identity = HostHandleLeaseIdentityV1 { handle, generation };
    super::reg().with_text_formal_identity(&identity, f)
}

/// Drop only if the raw slot still contains the captured lease identity.
/// The comparison and removal happen under one slot-table write lock.
#[inline(always)]
pub(crate) fn drop_if_lease_identity_matches(identity: HostHandleLeaseIdentityV1) -> bool {
    super::reg().drop_if_lease_identity_matches(identity)
}
