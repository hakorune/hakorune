//! The SafeMutex store owns program provenance and the transient receiver loan.

use super::{check_indexed_profile, handle_to_index, safe_mutex_objects, TypedObjectStoreBackend};
use std::num::NonZeroUsize;

pub(in crate::exports) struct DispatchReceiverPin {
    index: usize,
    type_id: i64,
}

impl DispatchReceiverPin {
    pub(in crate::exports) fn type_id(&self) -> i64 {
        self.type_id
    }
}

pub(in crate::exports) fn pin_dispatch_receiver(
    handle: i64,
    program: NonZeroUsize,
) -> Option<DispatchReceiverPin> {
    check_indexed_profile(TypedObjectStoreBackend::SafeMutex).ok()?;
    let index = handle_to_index(handle)?;
    let mut objects = safe_mutex_objects().lock().ok()?;
    let object = objects.get_mut(index)?.as_mut()?;
    let residence = object.dispatch.as_mut()?;
    if residence.stamp != program {
        return None;
    }
    residence.pins = residence.pins.checked_add(1)?;
    Some(DispatchReceiverPin {
        index,
        type_id: object.type_id,
    })
}

impl Drop for DispatchReceiverPin {
    fn drop(&mut self) {
        let mut objects = safe_mutex_objects()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let slot = objects[self.index].as_mut().expect("pinned object is live");
        let residence = slot.dispatch.as_mut().expect("pinned object has a program");
        residence.pins -= 1;
    }
}
