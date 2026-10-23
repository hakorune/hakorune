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

/// Independent profile for opaque Dynamic carriers. Its identity is the
/// immutable physical descriptor address; it cannot be confused with an
/// object End thunk or a Text-copy residence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrayCarrierSlotDescriptor {
    pub identity: usize,
}

impl ArrayCarrierSlotDescriptor {
    pub fn new(identity: usize) -> Self {
        Self { identity }
    }
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

pub(super) struct ArrayDynamicCarrierStorage {
    descriptor: Option<ArrayCarrierSlotDescriptor>,
    owner_identity: usize,
    slots: Vec<u64>,
    phase: Phase,
}

/// One exclusive storage representation with disjoint, typed profiles.
/// Generic Array operations reject the outer variant; each profile's claim,
/// append and End methods still validate its own descriptor type.
pub(super) enum ArrayOwnedSlots {
    Object(ArrayOwnedObjectStorage),
    DynamicCarrier(ArrayDynamicCarrierStorage),
}

impl ArrayOwnedSlots {
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Object(slots) => slots.len(),
            Self::DynamicCarrier(slots) => slots.slots.len(),
        }
    }

    pub(super) fn capacity(&self) -> usize {
        match self {
            Self::Object(slots) => slots.capacity(),
            Self::DynamicCarrier(slots) => slots.slots.capacity(),
        }
    }
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
        state.storage = ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(ArrayOwnedObjectStorage {
            descriptor: None,
            slots: Vec::new(),
            phase: Phase::Ended,
        }));
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
            ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(ArrayOwnedObjectStorage {
                descriptor: None, slots: Vec::new(), phase: Phase::Ended,
            })));
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
            ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(slots)) => return slots.live(descriptor),
            ArrayStorage::Boxed(items) if items.is_empty() => {}
            _ => return Err(ArrayOwnedSlotError::InvalidContract),
        }
        state.storage = ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(ArrayOwnedObjectStorage {
            descriptor: Some(descriptor),
            slots: Vec::new(),
            phase: Phase::Live,
        }));
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
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(slots)) = &mut state.storage else {
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
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(slots)) = &state.storage else {
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
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(slots)) = &mut state.storage else {
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
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::Object(slots)) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        if slots.descriptor != Some(descriptor) || slots.phase != Phase::Ending || !slots.slots.is_empty()
        {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots.phase = Phase::Ended;
        Ok(())
    }

    /// Claim a source-selected carrier profile on the same empty residence.
    /// This creates no child Home; its element obligations are separate opaque
    /// carrier tokens committed only by `append_dynamic_carrier_slot`.
    pub fn claim_dynamic_carrier_slots(
        &self,
        descriptor: ArrayCarrierSlotDescriptor,
    ) -> Result<(), ArrayOwnedSlotError> {
        if descriptor.identity == 0 {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        let mut state = self.items.state.write();
        if state.element_contract.is_some() {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        match &state.storage {
            ArrayStorage::OwnedSlots(ArrayOwnedSlots::DynamicCarrier(slots)) => {
                if slots.descriptor == Some(descriptor) && slots.phase == Phase::Live {
                    return Ok(());
                }
                return Err(ArrayOwnedSlotError::InvalidContract);
            }
            ArrayStorage::Boxed(items) if items.is_empty() => {}
            _ => return Err(ArrayOwnedSlotError::InvalidContract),
        }
        state.storage = ArrayStorage::OwnedSlots(ArrayOwnedSlots::DynamicCarrier(
            ArrayDynamicCarrierStorage {
                descriptor: Some(descriptor),
                owner_identity: std::sync::Arc::as_ptr(&self.items) as usize,
                slots: Vec::new(),
                phase: Phase::Live,
            },
        ));
        Ok(())
    }

    /// Commit one already-published carrier token. Every rejecting path leaves
    /// the candidate with its caller, and successful commit stores its raw
    /// identity exactly once without interpreting its payload.
    pub fn append_dynamic_carrier_slot(
        &self,
        descriptor: ArrayCarrierSlotDescriptor,
        candidate: u64,
    ) -> Result<usize, ArrayOwnedSlotError> {
        let token = crate::runtime::dynamic_invocation::DynamicCarrierTokenV1::from_raw_v1(candidate)
            .map_err(|_| ArrayOwnedSlotError::InvalidContract)?;
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::DynamicCarrier(slots)) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        if slots.phase != Phase::Live || slots.descriptor != Some(descriptor) {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots.slots.try_reserve(1).map_err(|_| ArrayOwnedSlotError::ReservationFailed)?;
        token
            .attach_to_array_v1(slots.owner_identity)
            .map_err(|_| ArrayOwnedSlotError::InvalidContract)?;
        slots.slots.push(candidate);
        Ok(slots.slots.len())
    }

    /// Detach carrier obligations under the Array gate. The caller runs
    /// Dynamic End outside both Array storage and host-registry locks.
    pub fn detach_dynamic_carriers_for_end(
        &self,
        descriptor: ArrayCarrierSlotDescriptor,
    ) -> Result<(usize, Vec<u64>), ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::DynamicCarrier(slots)) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        if slots.phase != Phase::Live || slots.descriptor != Some(descriptor) {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots.phase = Phase::Ending;
        Ok((slots.owner_identity, std::mem::take(&mut slots.slots)))
    }

    /// Complete End after every detached carrier has been attempted.
    pub fn finish_dynamic_carriers_end(
        &self,
        descriptor: ArrayCarrierSlotDescriptor,
    ) -> Result<(), ArrayOwnedSlotError> {
        let mut state = self.items.state.write();
        let ArrayStorage::OwnedSlots(ArrayOwnedSlots::DynamicCarrier(slots)) = &mut state.storage else {
            return Err(ArrayOwnedSlotError::InvalidContract);
        };
        if slots.descriptor != Some(descriptor) || slots.phase != Phase::Ending || !slots.slots.is_empty() {
            return Err(ArrayOwnedSlotError::InvalidContract);
        }
        slots.phase = Phase::Ended;
        Ok(())
    }
}
