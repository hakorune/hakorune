//! Clone-scrubbing candidate metadata slot for the selected Dynamic lane.
//!
//! This slot transports an already co-sealed AOT projection to the candidate
//! MIR JSON path.  It does not select a provider, issue a site plan, or own a
//! live executable.  Ordinary function metadata remains empty.

use crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1;

#[derive(Debug, PartialEq, Eq)]
enum State {
    Empty,
    Occupied(DynamicV2AotCallMetadataProjectionV1),
    Consumed,
}

/// Linear candidate-only storage.  Cloning metadata scrubs the projection so
/// a prepared clone cannot publish a second candidate admission.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DynamicV2AotMetadataSlotV1 {
    state: State,
}

impl Default for DynamicV2AotMetadataSlotV1 {
    fn default() -> Self {
        Self {
            state: State::Empty,
        }
    }
}

impl Clone for DynamicV2AotMetadataSlotV1 {
    fn clone(&self) -> Self {
        Self {
            state: match self.state {
                State::Empty => State::Empty,
                State::Occupied(_) | State::Consumed => State::Consumed,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicV2AotMetadataSlotRejectV1 {
    AlreadyOccupied,
    AlreadyConsumed,
}

impl DynamicV2AotMetadataSlotV1 {
    pub(crate) fn borrow(&self) -> Option<&DynamicV2AotCallMetadataProjectionV1> {
        match &self.state {
            State::Occupied(projection) => Some(projection),
            State::Empty | State::Consumed => None,
        }
    }

    pub(in crate::mir) fn install(
        &mut self,
        projection: DynamicV2AotCallMetadataProjectionV1,
    ) -> Result<(), DynamicV2AotMetadataSlotRejectV1> {
        match self.state {
            State::Empty => {
                self.state = State::Occupied(projection);
                Ok(())
            }
            State::Occupied(_) => Err(DynamicV2AotMetadataSlotRejectV1::AlreadyOccupied),
            State::Consumed => Err(DynamicV2AotMetadataSlotRejectV1::AlreadyConsumed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_scrubs_candidate_projection() {
        let slot = DynamicV2AotMetadataSlotV1::default();
        assert!(slot.borrow().is_none());
        let clone = slot.clone();
        assert!(clone.borrow().is_none());
    }
}
