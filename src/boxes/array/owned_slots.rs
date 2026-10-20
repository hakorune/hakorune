//! Physical append-only ownership state. Source/final owners authorize its use;
//! neither a numeric handle nor this descriptor establishes source ownership.
use super::{ArrayBox, ArrayStorage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayOwnedSlotDescriptor {
    pub identity: usize,
    pub profile: u32,
    pub type_id: i64,
    pub end_address: usize,
    pub nullable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayOwnedSlotError {
    InvalidContract,
    ReservationFailed,
}

#[derive(Debug, PartialEq, Eq)]
enum Phase {
    Live,
    Ending,
    Ended,
}

pub(super) struct ArrayOwnedObjectStorage {
    // Live/Ending always have a real child contract. None is terminal-only.
    descriptor: Option<ArrayOwnedSlotDescriptor>,
    slots: Vec<i64>,
    phase: Phase,
}

impl ArrayOwnedObjectStorage {
    pub(super) fn len(&self) -> usize {
        self.slots.len()
    }
    pub(super) fn capacity(&self) -> usize {
        self.slots.capacity()
    }
    fn live(&self, descriptor: ArrayOwnedSlotDescriptor) -> Result<(), ArrayOwnedSlotError> {
        if self.phase != Phase::Live || self.descriptor != Some(descriptor) {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        Ok(())
    }
}

impl ArrayBox {
    /// Consume a source-proved unclaimed empty residence under the SAME gate.
    /// This physical check grants no source ownership or content/End contract.
    pub fn retire_empty_residence(&self) -> Result<(), ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        if state.element_contract.is_some()
            || !matches!(&state.storage, ArrayStorage::Boxed(items) if items.is_empty())
        {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        // Reuse the existing terminal representation, never a fabricated live
        // child descriptor. All claimed and compatibility mutations stay closed.
        state.storage = ArrayStorage::OwnedObject(ArrayOwnedObjectStorage {
            descriptor: None,
            slots: Vec::new(),
            phase: Phase::Ended,
        });
        Ok(())
    }

    /// Retire a source-selected Text-value residence. Canonical String values
    /// own copies, not caller leases or callback-bearing object Homes.
    pub fn retire_text_residence(&self) -> Result<(), ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        if state.element_contract.is_some() || !match &state.storage {
            ArrayStorage::Boxed(items) => items.iter().all(|value|
                value.as_any().is::<crate::box_trait::StringBox>()),
            ArrayStorage::Text(_) => true,
            _ => false,
        } {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        let contents = std::mem::replace(&mut state.storage,
            ArrayStorage::OwnedObject(ArrayOwnedObjectStorage {
                descriptor: None, slots: Vec::new(), phase: Phase::Ended,
            }));
        drop(state);
        // Release detached Text storage outside the state gate.
        drop(contents);
        Ok(())
    }

    /// Only an empty, uncontracted builtin Array may receive this representation.
    pub fn claim_owned_slots(
        &self,
        descriptor: ArrayOwnedSlotDescriptor,
    ) -> Result<(), ArrayOwnedSlotError> {
        if descriptor.identity == 0
            || descriptor.end_address == 0
            || !matches!(descriptor.profile, 1 | 2)
            || descriptor.type_id <= 0
        {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        let mut state = self.items.state.write();
        if state.element_contract.is_some() {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        match &state.storage {
            ArrayStorage::OwnedObject(slots) => return slots.live(descriptor),
            ArrayStorage::Boxed(items) if items.is_empty() => {}
            _ => return Err(ArrayOwnedSlotError::InvalidContract),
        }
        state.storage = ArrayStorage::OwnedObject(ArrayOwnedObjectStorage {
            descriptor: Some(descriptor),
            slots: Vec::new(),
            phase: Phase::Live,
        });
        Ok(())
    }

    /// Normal is the single transfer point. Returned failure changes no slot.
    /// The ABI caller validates the candidate's physical identity beforehand.
    pub fn append_owned_slot(
        &self,
        descriptor: ArrayOwnedSlotDescriptor,
        candidate: i64,
    ) -> Result<usize, ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedObject(slots) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        slots.live(descriptor)?;
        // Checked indexed object handles are negative; host registry handles
        // are positive and must never be interpreted as this representation.
        if candidate > 0
            || (candidate == 0 && !descriptor.nullable)
            || (candidate != 0 && slots.slots.contains(&candidate))
        {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots
            .slots
            .try_reserve(1)
            .map_err(|_| ArrayOwnedSlotError::ReservationFailed)?;
        slots.slots.push(candidate);
        Ok(slots.slots.len())
    }

    /// A borrowed numeric identity, never a boxed clone, retain or new Home.
    pub fn borrow_owned_slot(
        &self,
        descriptor: ArrayOwnedSlotDescriptor,
        index: i64,
    ) -> Result<i64, ArrayOwnedSlotError> {
        let state = self.items.state.read();
        let ArrayStorage::OwnedObject(slots) = &state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        slots.live(descriptor)?;
        Ok(usize::try_from(index)
            .ok()
            .and_then(|index| slots.slots.get(index))
            .copied()
            .unwrap_or(0))
    }

    /// Return the detached obligations to the ABI caller. It must leave the
    /// host registry closure before invoking any child End callback.
    pub fn detach_owned_slots_for_end(
        &self,
        descriptor: ArrayOwnedSlotDescriptor,
    ) -> Result<Vec<i64>, ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedObject(slots) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        slots.live(descriptor)?;
        slots.phase = Phase::Ending;
        Ok(std::mem::take(&mut slots.slots))
    }

    pub fn finish_owned_slots_end(
        &self,
        descriptor: ArrayOwnedSlotDescriptor,
    ) -> Result<(), ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedObject(slots) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        if slots.descriptor != Some(descriptor) || slots.phase != Phase::Ending || !slots.slots.is_empty()
        {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots.phase = Phase::Ended;
        Ok(())
    }
}
