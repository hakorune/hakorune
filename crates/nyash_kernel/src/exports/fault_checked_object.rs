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
    if storage.is_null() || !storage.cast::<FaultFrame>().is_aligned() {
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

use super::checked_map::storage::{
    Placement, init as placement_init, admit as placement_admit,
    dispose as placement_dispose, separate,
};
use crate::exports::typed_object::{
    DynamicFieldEndDescriptorV1, DynamicFieldStoredV1, DynamicFieldValueV1,
};
use std::{mem::size_of, sync::Mutex};

const FIELD_OUTCOME_TAG: u64 = 0x4649454c444f5501;
pub(super) enum DynamicFieldOutcomeV1 { Unissued, Ready(DynamicFieldStoredV1), Consumed }
pub(super) type DynamicFieldOutcomeStorageV1 = Placement<Mutex<DynamicFieldOutcomeV1>>;

const fn dynamic_field_abi() -> [u8; 32] {
    let words = [32u32, 1, size_of::<DynamicFieldValueV1>() as u32,
        std::mem::align_of::<DynamicFieldValueV1>() as u32,
        size_of::<DynamicFieldOutcomeStorageV1>() as u32,
        std::mem::align_of::<DynamicFieldOutcomeStorageV1>() as u32,
        size_of::<DynamicFieldEndDescriptorV1>() as u32,
        std::mem::align_of::<DynamicFieldEndDescriptorV1>() as u32];
    let mut out = [0; 32];
    let mut index = 0;
    while index < words.len() {
        let bytes = words[index].to_le_bytes();
        let mut byte = 0;
        while byte < 4 { out[index * 4 + byte] = bytes[byte]; byte += 1; }
        index += 1;
    }
    out
}

/// Optional field ABI extension, target-compiled independently of the unchanged
/// common runtime descriptor. Consumers must admit this version before use.
#[used]
#[link_section = ".nyash.object_field_abi.v1"]
#[export_name = "nyash_object_field_abi_descriptor_v1"]
pub static DYNAMIC_FIELD_ABI_V1: [u8; 32] = dynamic_field_abi();

fn dynamic_regions(frame: *mut c_void, outcome: *mut c_void) -> bool {
    separate(&[(frame as usize, size_of::<FaultFrame>()),
        (outcome as usize, size_of::<DynamicFieldOutcomeStorageV1>())])
}

#[export_name = "nyash.object.field_outcome_init_v1"]
pub unsafe extern "C" fn dynamic_outcome_init(ptr: *mut c_void) -> u32 {
    unsafe { placement_init(ptr, FIELD_OUTCOME_TAG, Mutex::new(DynamicFieldOutcomeV1::Unissued)) }
}

unsafe fn commit_dynamic(
    frame: *mut c_void, profile: u32, site: u64, outcome: *mut c_void,
    action: impl FnOnce(TypedObjectStoreBackend) -> Result<DynamicFieldStoredV1, CheckedStorageError>,
) -> u32 {
    if !dynamic_regions(frame, outcome) { return Status::InvalidContract as u32; }
    let profile = match unsafe { admit(frame, profile) } {
        Ok((_, profile)) => profile, Err(status) => return status as u32,
    };
    let result = {
        let outcome = match unsafe { placement_admit::<Mutex<DynamicFieldOutcomeV1>>(outcome, FIELD_OUTCOME_TAG) } {
            Ok(value) => value, Err(status) => return status as u32,
        };
        let mut outcome = match outcome.lock() { Ok(value) => value, Err(_) => return Status::InvalidContract as u32 };
        if !matches!(*outcome, DynamicFieldOutcomeV1::Unissued) { return Status::InvalidContract as u32; }
        match action(profile) {
            Ok(old) => {
                // No fallible work between the committed swap and publication.
                *outcome = DynamicFieldOutcomeV1::Ready(old);
                return Status::Normal as u32;
            }
            Err(error) => error,
        }
    };
    // No outcome/object guard survives into Fault recording or user cleanup.
    let frame = unsafe { &mut *frame.cast::<FaultFrame>() };
    failed(frame, result, site)
}

/// Candidate is caller-owned unless Normal publishes the detached-old outcome.
/// A null descriptor explicitly selects a no-Home scalar residence; it cannot
/// store Text/object/Array handles. Stored borrowed/Map profiles remain unadmitted.
#[export_name = "nyash.object.checked_dynamic_replace_v1"]
pub unsafe extern "C" fn dynamic_replace(
    frame: *mut c_void, profile: u32, site: u64, handle: i64, type_id: i64,
    slot: usize, value: *const DynamicFieldValueV1,
    descriptor: *const DynamicFieldEndDescriptorV1, outcome: *mut c_void,
) -> u32 {
    if value.is_null() || !value.is_aligned() || !separate(&[
        (frame as usize, size_of::<FaultFrame>()),
        (outcome as usize, size_of::<DynamicFieldOutcomeStorageV1>()),
        (value as usize, size_of::<DynamicFieldValueV1>()),
        (descriptor as usize, if descriptor.is_null() { 0 } else { size_of::<DynamicFieldEndDescriptorV1>() }),
    ]) { return Status::InvalidContract as u32; }
    let value = unsafe { *value };
    if !value.well_formed() || value.kind == 0 { return Status::InvalidContract as u32; }
    let end = if descriptor.is_null() { None } else {
        if !descriptor.is_aligned() { return Status::InvalidContract as u32; }
        // Published caller guarantees this immutable AOT constant remains live.
        Some(unsafe { &*descriptor })
    };
    match (value.kind, end) {
        (1..=3, None) => {}
        (4..=6, Some(end)) if end.kind == value.kind && end.profile == profile
            && end.end.is_some() && if value.kind == 5 { end.type_id > 0 } else { end.type_id == 0 } => {}
        _ => return Status::InvalidContract as u32,
    }
    unsafe { commit_dynamic(frame, profile, site, outcome, |profile|
        store::replace_dynamic_field_checked(profile, handle, type_id, slot, DynamicFieldStoredV1 { value, end })) }
}

/// Checked read is an observation supported by the still-live slot. It issues
/// neither a Home nor an independent Text lease and writes only on Normal.
/// Kind zero requests a tagged observation for an explicit source tag test;
/// EmptyField still Faults. It is not a borrowed Dynamic callable result.
#[export_name = "nyash.object.checked_dynamic_get_v1"]
pub unsafe extern "C" fn dynamic_get(
    frame: *mut c_void, profile: u32, site: u64, handle: i64, type_id: i64,
    slot: usize, kind: u32, out: *mut DynamicFieldValueV1,
) -> u32 {
    if out.is_null() || !out.is_aligned() || kind > 6
        || !separate(&[(frame as usize, size_of::<FaultFrame>()),
            (out as usize, size_of::<DynamicFieldValueV1>())]) { return Status::InvalidContract as u32; }
    let (frame, profile) = match unsafe { admit(frame, profile) } {
        Ok(value) => value, Err(status) => return status as u32,
    };
    match store::read_dynamic_field_checked(profile, handle, type_id, slot, kind) {
        Ok(value) => { unsafe { out.write(value); } Status::Normal as u32 }
        Err(error) => failed(frame, error, site),
    }
}

/// The source destructor selects reverse field order and parent fini. Taking a
/// field only detaches its current residence; End is a separate consumed outcome.
#[export_name = "nyash.object.checked_dynamic_take_v1"]
pub unsafe extern "C" fn dynamic_take(
    frame: *mut c_void, profile: u32, site: u64, handle: i64, type_id: i64,
    slot: usize, outcome: *mut c_void,
) -> u32 {
    unsafe { commit_dynamic(frame, profile, site, outcome, |profile|
        store::take_dynamic_field_checked(profile, handle, type_id, slot)) }
}

#[export_name = "nyash.object.field_outcome_end_v1"]
pub unsafe extern "C" fn dynamic_outcome_end(frame: *mut c_void, site: u64, ptr: *mut c_void) -> u32 {
    if !dynamic_regions(frame, ptr) || frame.is_null() || !frame.cast::<FaultFrame>().is_aligned()
        || !unsafe { (&*frame.cast::<FaultFrame>()).valid() } { return Status::InvalidContract as u32; }
    let old = {
        let outcome = match unsafe { placement_admit::<Mutex<DynamicFieldOutcomeV1>>(ptr, FIELD_OUTCOME_TAG) } {
            Ok(value) => value, Err(status) => return status as u32,
        };
        let mut outcome = match outcome.lock() { Ok(value) => value, Err(_) => return Status::InvalidContract as u32 };
        if !matches!(*outcome, DynamicFieldOutcomeV1::Ready(_)) { return Status::InvalidContract as u32; }
        match std::mem::replace(&mut *outcome, DynamicFieldOutcomeV1::Consumed) {
            DynamicFieldOutcomeV1::Ready(old) => old, _ => unreachable!(),
        }
    };
    // Consume before callback: reentrant End cannot repeat the same obligation.
    let status = unsafe { old.end(frame, site) };
    match status {
        0 => Status::Normal as u32,
        1 if unsafe { (&*frame.cast::<FaultFrame>()).valid() }
            && unsafe { (&*frame.cast::<FaultFrame>()).diagnostics().is_ok_and(|(first, _, _)| first.is_some()) } => Status::Fault as u32,
        _ => Status::InvalidContract as u32,
    }
}

#[export_name = "nyash.object.field_outcome_dispose_v1"]
pub unsafe extern "C" fn dynamic_outcome_dispose(ptr: *mut c_void) -> u32 {
    {
        let outcome = match unsafe { placement_admit::<Mutex<DynamicFieldOutcomeV1>>(ptr, FIELD_OUTCOME_TAG) } {
            Ok(value) => value, Err(status) => return status as u32,
        };
        let outcome = match outcome.lock() { Ok(value) => value, Err(_) => return Status::InvalidContract as u32 };
        if matches!(*outcome, DynamicFieldOutcomeV1::Ready(_)) { return Status::InvalidContract as u32; }
    }
    unsafe { placement_dispose::<Mutex<DynamicFieldOutcomeV1>>(ptr) }
}
