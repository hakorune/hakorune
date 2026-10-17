//! Physical source-issued object-slot descriptor. The published caller owns
//! transfer authorization; identity validation here cannot invent that authority.
use super::*;
use crate::exports::typed_object_store_backend::{
    check_indexed_profile, validate_checked_indexed_identity, TypedObjectStoreBackend,
};
use nyash_rust::boxes::array::{ArrayOwnedSlotDescriptor, ArrayOwnedSlotError};

type End = unsafe extern "C" fn(*mut c_void, i64, u64) -> u32;

/// Immutable, live AOT constant. The address and every field are bound at claim
/// and rechecked at append/read/End. C must not mutate or retire this constant.
#[repr(C)]
pub(super) struct Descriptor {
    pub profile: u32,
    pub nullable: u32,
    pub type_id: i64,
    pub end: Option<End>,
}

unsafe fn descriptor(
    raw: *const Descriptor,
) -> Result<(ArrayOwnedSlotDescriptor, TypedObjectStoreBackend, End), Status> {
    if raw.is_null() {
        return Err(Status::InvalidContract);
    }
    // Trusted aligned immutable constant, not an untrusted address validator.
    let value = unsafe { &*raw };
    let profile = match value.profile {
        1 => TypedObjectStoreBackend::SafeMutex,
        2 => TypedObjectStoreBackend::SingleThreadExact,
        _ => return Err(Status::InvalidContract),
    };
    check_indexed_profile(profile).map_err(|_| Status::InvalidContract)?;
    if value.nullable > 1 || value.type_id <= 0 {
        return Err(Status::InvalidContract);
    }
    let end = value.end.ok_or(Status::InvalidContract)?;
    Ok((
        ArrayOwnedSlotDescriptor {
            identity: raw as usize,
            profile: value.profile,
            type_id: value.type_id,
            end_address: end as usize,
            nullable: value.nullable == 1,
        },
        profile,
        end,
    ))
}

/// Same live/exclusive frame and immutable descriptor contract for all entries.
#[export_name = "nyash.array.checked_owned_claim_v1"]
pub unsafe extern "C" fn claim(
    storage: *mut c_void,
    _site: u64,
    handle: i64,
    raw: *const Descriptor,
) -> u32 {
    if let Err(status) = unsafe { admit(storage) } {
        return status as u32;
    }
    let (key, _, _) = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match with_array_box_direct(handle, |array| array.claim_owned_slots(key)) {
        Some(Ok(())) => Status::Normal as u32,
        _ => Status::InvalidContract as u32,
    }
}

/// Candidate stays caller-owned unless this returns Normal. No clone or retain.
#[export_name = "nyash.array.checked_owned_append_v1"]
pub unsafe extern "C" fn append(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    raw: *const Descriptor,
    candidate: i64,
) -> u32 {
    if let Err(status) = unsafe { admit(storage) } {
        return status as u32;
    }
    let (key, profile, _) = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    if candidate != 0 && validate_checked_indexed_identity(profile, candidate, key.type_id).is_err()
    {
        return Status::InvalidContract as u32;
    }
    match with_array_box_direct(handle, |array| array.append_owned_slot(key, candidate)) {
        Some(Ok(_)) => Status::Normal as u32,
        Some(Err(ArrayOwnedSlotError::ReservationFailed)) => {
            let frame = match unsafe { admit(storage) } {
                Ok(value) => value,
                Err(status) => return status as u32,
            };
            record(frame, Ok(Diagnostic::new(205, site, [0, 0])))
        }
        _ => Status::InvalidContract as u32,
    }
}

/// Only Normal publishes the borrowed nullable handle. Invalid state/descriptor
/// is never projected into missing/null; there is no new Home or host handle.
#[export_name = "nyash.array.checked_owned_get_v1"]
pub unsafe extern "C" fn get(
    storage: *mut c_void,
    _site: u64,
    handle: i64,
    raw: *const Descriptor,
    index: i64,
    out: *mut i64,
) -> u32 {
    if out.is_null() {
        return Status::InvalidContract as u32;
    }
    if let Err(status) = unsafe { admit(storage) } {
        return status as u32;
    }
    let (key, profile, _) = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    let value = match with_array_box_direct(handle, |array| array.borrow_owned_slot(key, index)) {
        Some(Ok(value)) => value,
        _ => return Status::InvalidContract as u32,
    };
    if value != 0 && validate_checked_indexed_identity(profile, value, key.type_id).is_err() {
        return Status::InvalidContract as u32;
    }
    unsafe { out.write(value) };
    Status::Normal as u32
}

#[export_name = "nyash.array.checked_owned_end_v1"]
pub unsafe extern "C" fn end(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    raw: *const Descriptor,
) -> u32 {
    // Scope this borrow before any thunk re-enters the very same FaultFrame.
    if let Err(status) = unsafe { admit(storage) } {
        return status as u32;
    }
    let (key, _, child_end) = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    let slots = match with_array_box_direct(handle, |array| array.detach_owned_slots_for_end(key)) {
        Some(Ok(slots)) => slots,
        _ => return Status::InvalidContract as u32,
    };
    // Both ArrayStateCell and host registry locks have been released here.
    let mut fault = false;
    for child in slots.into_iter().rev().filter(|child| *child != 0) {
        match unsafe { child_end(storage, child, site) } {
            0 => {}
            1 => fault = true,
            _ => return Status::InvalidContract as u32,
        }
    }
    if !matches!(
        with_array_box_direct(handle, |array| array.finish_owned_slots_end(key)),
        Some(Ok(()))
    ) {
        return Status::InvalidContract as u32;
    }
    crate::nyrt_handle_release_h(handle);
    if fault {
        Status::Fault as u32
    } else {
        Status::Normal as u32
    }
}
