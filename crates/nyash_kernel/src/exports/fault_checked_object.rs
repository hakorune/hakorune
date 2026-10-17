//! Trusted typed lifecycle ABI. Published definitions authorize operations;
//! this module only validates physical operands and records runtime failures.
use super::{Diagnostic, FaultFrame, Status};
use crate::exports::typed_object::ObjectResidenceOutcomeV1;
use crate::exports::typed_object_store_backend::{
    self as store, CheckedStorageError, TypedObjectStoreBackend,
};
use std::ffi::c_void;

/// A physical residence pair, never a dynamically inferred ownership claim.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ObjectResidenceV1 {
    pub(super) value: i64,
    pub(super) outcome: u32,
    pub(super) reserved: u32,
}

fn indexed_profile(profile: u32) -> Result<TypedObjectStoreBackend, Status> {
    match profile {
        1 => Ok(TypedObjectStoreBackend::SafeMutex),
        2 => Ok(TypedObjectStoreBackend::SingleThreadExact),
        _ => Err(Status::InvalidContract),
    }
}

unsafe fn admit<'a>(
    storage: *mut c_void,
    profile: u32,
) -> Result<(&'a mut FaultFrame, TypedObjectStoreBackend), Status> {
    if storage.is_null() {
        return Err(Status::InvalidContract);
    }
    // Caller guarantees a live aligned frame and unique synchronous borrow.
    let frame = unsafe { &mut *storage.cast::<FaultFrame>() };
    if !frame.valid() {
        return Err(Status::InvalidContract);
    }
    let profile = indexed_profile(profile)?;
    store::check_indexed_profile(profile).map_err(|_| Status::InvalidContract)?;
    Ok((frame, profile))
}

/// Exact nonfallible source read: failure is a broken physical contract, not
/// a source Fault. The caller supplies a valid aligned nonoverlapping out-slot.
/// It is written only on Normal; zero is a value, never failure substitution.
#[export_name = "nyash.object.checked_field_get_i64_v1"]
pub unsafe extern "C" fn field_get(
    profile: u32,
    handle: i64,
    type_id: i64,
    slot: usize,
    out: *mut i64,
) -> u32 {
    if out.is_null() {
        return Status::InvalidContract as u32;
    }
    let profile = match indexed_profile(profile) {
        Ok(profile) => profile,
        Err(status) => return status as u32,
    };
    match store::get_checked_indexed(profile, handle, type_id, slot) {
        Ok(value) => {
            unsafe {
                out.write(value);
            }
            Status::Normal as u32
        }
        Err(_) => Status::InvalidContract as u32,
    }
}

/// Fallible exact received-object read. The original source and final consumers
/// authorize field identity; this entry records the existing checked-object
/// failure and writes the out-slot exclusively on Normal.
#[export_name = "nyash.object.checked_field_get_fallible_i64_v1"]
pub unsafe extern "C" fn field_get_fallible(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    handle: i64,
    type_id: i64,
    slot: usize,
    out: *mut i64,
) -> u32 {
    if out.is_null() { return Status::InvalidContract as u32; }
    let (frame, profile) = match unsafe { admit(storage, profile) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match store::get_checked_indexed(profile, handle, type_id, slot) {
        Ok(value) => {
            unsafe { out.write(value); }
            Status::Normal as u32
        }
        Err(error) => failed(frame, error, site),
    }
}

fn failed(frame: &mut FaultFrame, error: CheckedStorageError, site: u64) -> u32 {
    let reason = match error {
        CheckedStorageError::AllocationOrStorageUnavailable => 100,
        CheckedStorageError::ObjectOrFieldMismatch => 101,
        CheckedStorageError::InvalidLayout | CheckedStorageError::ProfileMismatch => {
            return Status::InvalidContract as u32
        }
    };
    match frame.record(Diagnostic::new(reason, site, [0; 2])) {
        Ok(status) => status as u32,
        Err(_) => Status::InvalidContract as u32,
    }
}

/// Non-null pointers must denote valid aligned nonoverlapping storage. A zero
/// count permits a null layout pointer. The out-slot is written only on Normal.
#[export_name = "nyash.object.checked_new_v1"]
pub unsafe extern "C" fn allocate(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    type_id: i64,
    layout: *const u32,
    count: usize,
    out: *mut i64,
) -> u32 {
    if out.is_null()
        || (count != 0 && layout.is_null())
        || i64::try_from(count)
            .ok()
            .and_then(crate::exports::typed_object::normalize_field_count)
            .is_none()
    {
        return Status::InvalidContract as u32;
    }
    let (frame, profile) = match unsafe { admit(storage, profile) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    let layout = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(layout, count) }
    };
    match store::new_checked_wire_indexed(profile, type_id, layout) {
        Ok(handle) => {
            unsafe {
                out.write(handle);
            }
            Status::Normal as u32
        }
        Err(error) => failed(frame, error, site),
    }
}

#[export_name = "nyash.object.checked_field_set_v1"]
pub unsafe extern "C" fn field_set(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    handle: i64,
    type_id: i64,
    slot: usize,
    value: i64,
) -> u32 {
    let (frame, profile) = match unsafe { admit(storage, profile) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match store::set_checked_indexed(profile, handle, type_id, slot, value) {
        Ok(()) => Status::Normal as u32,
        Err(error) => failed(frame, error, site),
    }
}

/// Published actual/formal/store correspondence authorizes this one move.
/// The runtime validates both columns before changing either one.
#[export_name = "nyash.object.checked_residence_set_v1"]
pub unsafe extern "C" fn residence_set(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    handle: i64,
    type_id: i64,
    slot: usize,
    child_type: i64,
    value: i64,
    outcome: u32,
) -> u32 {
    let outcome = match outcome {
        0 => ObjectResidenceOutcomeV1::Null,
        1 => ObjectResidenceOutcomeV1::Fresh,
        2 => ObjectResidenceOutcomeV1::Forward,
        _ => return Status::InvalidContract as u32,
    };
    if !outcome.accepts_payload(value) {
        return Status::InvalidContract as u32;
    }
    let (frame, profile) = match unsafe { admit(storage, profile) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match store::install_object_residence_checked(
        profile, handle, type_id, slot, child_type, value, outcome,
    ) {
        Ok(()) => Status::Normal as u32,
        Err(error) => failed(frame, error, site),
    }
}

/// Exact read: neither a source Fault nor a zero/default substitution.
/// Caller provides aligned nonoverlapping storage, written only on Normal.
#[export_name = "nyash.object.checked_residence_get_v1"]
pub unsafe extern "C" fn residence_get(
    profile: u32,
    handle: i64,
    type_id: i64,
    slot: usize,
    child_type: i64,
    out: *mut ObjectResidenceV1,
) -> u32 {
    if out.is_null() {
        return Status::InvalidContract as u32;
    }
    let profile = match indexed_profile(profile) {
        Ok(profile) => profile,
        Err(status) => return status as u32,
    };
    match store::get_object_residence_checked(profile, handle, type_id, slot, child_type) {
        Ok((value, outcome)) => {
            unsafe {
                out.write(ObjectResidenceV1 {
                    value,
                    outcome: outcome as u32,
                    reserved: 0,
                });
            }
            Status::Normal as u32
        }
        Err(_) => Status::InvalidContract as u32,
    }
}

unsafe fn release(storage: *mut c_void, profile: u32, site: u64, handle: i64, type_id: i64) -> u32 {
    let (frame, profile) = match unsafe { admit(storage, profile) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match store::reclaim_checked_indexed(profile, handle, type_id) {
        Ok(()) => Status::Normal as u32,
        Err(error) => failed(frame, error, site),
    }
}

/// Caller authorizes unpublished outer storage only, never parent fini.
#[export_name = "nyash.object.reclaim_unpublished_v1"]
pub unsafe extern "C" fn reclaim(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    handle: i64,
    type_id: i64,
) -> u32 {
    unsafe { release(storage, profile, site, handle, type_id) }
}

/// Caller must carry published PlainI64NoHook destruction admission. Slot tags
/// and absent runtime hooks never establish that semantic permission.
#[export_name = "nyash.object.home_release_plain_i64_v1"]
pub unsafe extern "C" fn home_release(
    storage: *mut c_void,
    profile: u32,
    site: u64,
    handle: i64,
    type_id: i64,
) -> u32 {
    unsafe { release(storage, profile, site, handle, type_id) }
}
