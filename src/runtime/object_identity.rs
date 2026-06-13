//! Object identity contract for future Arc retirement.
//!
//! This module is intentionally representation-only.  It does not own storage,
//! dispatch, refcounting, finalization, or plugin invocation.  The current
//! runtime may still back object identity with `Arc<dyn NyashBox>`; these types
//! define the stable seam that a later ownership substrate can implement.

use std::num::NonZeroU64;

/// Stable runtime object handle.
///
/// `0` remains the external null/void sentinel and is not a valid object.
/// The raw value is deliberately opaque: callers must not infer table index,
/// generation, object kind, or ownership state from it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ObjectHandle(NonZeroU64);

impl ObjectHandle {
    /// Construct from a raw runtime handle, returning `None` for the null
    /// sentinel.
    #[inline]
    pub const fn new(raw: u64) -> Option<Self> {
        match NonZeroU64::new(raw) {
            Some(raw) => Some(Self(raw)),
            None => None,
        }
    }

    /// Construct from a raw runtime handle, failing fast on the null sentinel.
    #[inline]
    pub fn new_or_panic(raw: u64) -> Self {
        Self::new(raw).expect("ObjectHandle cannot be zero")
    }

    /// Return the opaque raw handle used at C ABI / host-handle boundaries.
    #[inline]
    pub const fn raw(self) -> u64 {
        self.0.get()
    }
}

/// Monotonic slot generation for stale-handle / weak-handle checks.
///
/// Generation `0` is reserved for legacy unversioned handles.  New substrate
/// slots should start at `FIRST`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ObjectGeneration(u32);

impl ObjectGeneration {
    pub const LEGACY_UNVERSIONED: Self = Self(0);
    pub const FIRST: Self = Self(1);

    #[inline]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    #[inline]
    pub const fn is_versioned(self) -> bool {
        self.0 != 0
    }
}

/// Stable identity of one object slot.
///
/// A `BoxIdentity` is stronger than a raw handle because it can distinguish
/// reused slots once the ownership substrate adds generations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoxIdentity {
    handle: ObjectHandle,
    generation: ObjectGeneration,
}

impl BoxIdentity {
    #[inline]
    pub const fn new(handle: ObjectHandle, generation: ObjectGeneration) -> Self {
        Self { handle, generation }
    }

    #[inline]
    pub const fn legacy(handle: ObjectHandle) -> Self {
        Self::new(handle, ObjectGeneration::LEGACY_UNVERSIONED)
    }

    #[inline]
    pub const fn handle(self) -> ObjectHandle {
        self.handle
    }

    #[inline]
    pub const fn generation(self) -> ObjectGeneration {
        self.generation
    }
}

/// Non-owning identity token.
///
/// Upgrading a weak handle must verify both handle and generation.  Current
/// `Weak<dyn NyashBox>` support can project into this shape later without
/// changing language-level semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WeakObjectHandle {
    identity: BoxIdentity,
}

impl WeakObjectHandle {
    #[inline]
    pub const fn new(identity: BoxIdentity) -> Self {
        Self { identity }
    }

    #[inline]
    pub const fn identity(self) -> BoxIdentity {
        self.identity
    }
}

/// Object family that owns semantic identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ObjectIdentityKind {
    Builtin,
    UserBox,
    Plugin,
    HostBridge,
    Unknown,
}

/// Runtime root visibility for an object identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RootVisibility {
    /// Strong runtime root; object must remain alive while visible.
    StrongRoot,
    /// Non-owning weak visibility only.
    WeakOnly,
    /// Borrowed under another owner or registry lock.
    Borrowed,
    /// Not visible to root enumeration.
    Unrooted,
}

/// Owner responsible for finalization/fini behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FiniOwner {
    None,
    Scope,
    ObjectDrop,
    Plugin {
        type_id: u32,
        instance_id: u32,
        fini_method_id: Option<u32>,
    },
    Host,
}

/// Builtin object identity payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuiltinIdentity {
    pub box_id: u64,
    pub type_name: &'static str,
}

/// Plugin object identity payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PluginInstanceIdentity {
    pub type_id: u32,
    pub instance_id: u32,
    pub fini_method_id: Option<u32>,
}

/// Read-only descriptor that ties identity to root and finalization ownership.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectIdentityDescriptor {
    pub identity: BoxIdentity,
    pub kind: ObjectIdentityKind,
    pub root_visibility: RootVisibility,
    pub fini_owner: FiniOwner,
}

impl ObjectIdentityDescriptor {
    #[inline]
    pub const fn new(
        identity: BoxIdentity,
        kind: ObjectIdentityKind,
        root_visibility: RootVisibility,
        fini_owner: FiniOwner,
    ) -> Self {
        Self {
            identity,
            kind,
            root_visibility,
            fini_owner,
        }
    }
}

/// Stable report fields for ARC-RETIRE-003.
pub fn object_identity_contract_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("object_handle_contract_defined", "1"),
        ("object_handle_zero_is_invalid", "1"),
        ("box_identity_generation_defined", "1"),
        ("weak_object_handle_generation_check_required", "1"),
        ("object_identity_root_visibility_defined", "1"),
        ("object_identity_fini_owner_defined", "1"),
        ("typeabi_identity_truth_count", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_handle_rejects_zero() {
        assert_eq!(ObjectHandle::new(0), None);
        assert_eq!(ObjectHandle::new(7).map(ObjectHandle::raw), Some(7));
    }

    #[test]
    fn box_identity_distinguishes_reused_generation() {
        let handle = ObjectHandle::new_or_panic(9);
        let a = BoxIdentity::new(handle, ObjectGeneration::new(1));
        let b = BoxIdentity::new(handle, ObjectGeneration::new(2));

        assert_ne!(a, b);
        assert_eq!(a.handle().raw(), 9);
        assert!(a.generation().is_versioned());
    }

    #[test]
    fn report_contract_stays_explicit() {
        let fields = object_identity_contract_report_fields();
        assert!(fields.contains(&("object_handle_contract_defined", "1")));
        assert!(fields.contains(&("typeabi_identity_truth_count", "0")));
    }
}
