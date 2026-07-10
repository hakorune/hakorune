use crate::BindingId;

/// Domain-specific identity for a source local slot.
///
/// Allocation remains owned by the existing `BindingId` allocator. This
/// wrapper prevents local-contract APIs from accidentally accepting SSA
/// `ValueId`s or introducing a second lexical identity namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalSlotId(BindingId);

impl LocalSlotId {
    pub const fn from_binding_id(binding_id: BindingId) -> Self {
        Self(binding_id)
    }

    pub const fn binding_id(self) -> BindingId {
        self.0
    }
}

impl From<BindingId> for LocalSlotId {
    fn from(binding_id: BindingId) -> Self {
        Self::from_binding_id(binding_id)
    }
}

impl From<LocalSlotId> for BindingId {
    fn from(local_slot_id: LocalSlotId) -> Self {
        local_slot_id.binding_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_existing_binding_identity_without_allocating() {
        let binding_id = BindingId::new(7);
        let slot_id = LocalSlotId::from(binding_id);

        assert_eq!(slot_id.binding_id(), binding_id);
        assert_eq!(BindingId::from(slot_id), binding_id);
    }
}
