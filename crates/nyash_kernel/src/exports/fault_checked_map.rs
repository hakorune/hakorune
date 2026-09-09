//! Trusted opaque Map ABI. No host publication and no semantic ownership issuance.
//! Pointers name initialized, aligned, nonoverlapping storage with synchronous
//! access. Init additionally requires fresh storage; foreign pointers are UB.
use super::{Diagnostic, FaultFrame, Status};
use crate::exports::{
    checked_map_residence,
    typed_object_store_backend::{self as store, CheckedStorageError, TypedObjectStoreBackend},
};
use nyash_rust::boxes::{
    map_box::checked::{CheckedMap, CheckedMapError, CheckedMapPayload, MapEndError, MapEndReport},
    map_key_domain::MapKeyDomain,
};
use std::{ffi::c_void, mem::size_of, sync::Mutex};
#[path = "fault_checked_map_storage.rs"]
mod storage;
use storage::*;
pub(super) use storage::{KeyStorage, MapStorage, OutcomeStorage};

unsafe fn valid_frame(ptr: *mut c_void) -> bool {
    !ptr.is_null()
        && (ptr as usize) % std::mem::align_of::<FaultFrame>() == 0
        && unsafe { (&*ptr.cast::<FaultFrame>()).valid() }
}
unsafe fn failed(ptr: *mut c_void, site: u64, reason: u32) -> u32 {
    // Called only after callbacks/borrowed operation state have ended.
    unsafe { super::record_static(ptr, reason, site, 0, 0) }
}
fn root_reason(error: CheckedMapError) -> Option<u32> {
    match error {
        CheckedMapError::StorageUnavailable
        | CheckedMapError::CapacityUnavailable
        | CheckedMapError::OrderExhausted => Some(100),
        _ => None,
    }
}
fn end_reason(error: MapEndError) -> u32 {
    match error {
        MapEndError::StorageUnavailable => 100,
        _ => 101,
    }
}
unsafe fn report(ptr: *mut c_void, site: u64, result: MapEndReport) -> u32 {
    let Some(first) = result.first else {
        return Status::Normal as u32;
    };
    let frame = unsafe { &mut *ptr.cast::<FaultFrame>() };
    if !frame.valid() {
        return Status::InvalidContract as u32;
    }
    for error in std::iter::once(first).chain(result.suppressed.into_iter().flatten()) {
        if frame
            .record(Diagnostic::new(end_reason(error), site, [0; 2]))
            .is_err()
        {
            return Status::InvalidContract as u32;
        }
    }
    if result.suppressed_count > result.suppressed.len() {
        frame.omitted = 1;
    }
    Status::Fault as u32
}

#[export_name = "nyash.map.storage_init_v1"]
pub unsafe extern "C" fn map_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, MAP_TAG, CheckedMap::unissued()) }
}
#[export_name = "nyash.map.key_init_v1"]
pub unsafe extern "C" fn key_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, KEY_TAG, Mutex::new(KeyState::Empty)) }
}
#[export_name = "nyash.map.outcome_init_v1"]
pub unsafe extern "C" fn outcome_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, OUT_TAG, Mutex::new(OutcomeState::Unissued)) }
}
#[export_name = "nyash.map.checked_new_v1"]
pub unsafe extern "C" fn allocate(
    frame: *mut c_void,
    profile: u32,
    site: u64,
    ptr: *mut c_void,
) -> u32 {
    if !separate(&[
        (frame as usize, size_of::<FaultFrame>()),
        (ptr as usize, size_of::<MapStorage>()),
    ]) || !unsafe { valid_frame(frame) }
        || profile != 1
        || store::check_indexed_profile(TypedObjectStoreBackend::SafeMutex).is_err()
    {
        return Status::InvalidContract as u32;
    }
    let map = match unsafe { admit::<CheckedMap>(ptr, MAP_TAG) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    match map.acquire() {
        Ok(()) => Status::Normal as u32,
        Err(error) => match root_reason(error) {
            Some(r) => unsafe { failed(frame, site, r) },
            None => Status::InvalidContract as u32,
        },
    }
}
#[export_name = "nyash.map.key_prepare_utf8_v1"]
pub unsafe extern "C" fn key_prepare(
    frame: *mut c_void,
    site: u64,
    ptr: *mut c_void,
    bytes: *const u8,
    len: usize,
) -> u32 {
    if len > isize::MAX as usize
        || !separate(&[
            (frame as usize, size_of::<FaultFrame>()),
            (ptr as usize, size_of::<KeyStorage>()),
            (bytes as usize, len),
        ])
        || !unsafe { valid_frame(frame) }
    {
        return Status::InvalidContract as u32;
    }
    let key = match unsafe { admit::<Mutex<KeyState>>(ptr, KEY_TAG) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let mut key = match key.lock() {
        Ok(v) => v,
        Err(_) => return Status::InvalidContract as u32,
    };
    if !matches!(*key, KeyState::Empty) {
        return Status::InvalidContract as u32;
    }
    let bytes = if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(bytes, len) }
    };
    let text = match std::str::from_utf8(bytes) {
        Ok(v) => v,
        Err(_) => return Status::InvalidContract as u32,
    };
    match MapKeyDomain::try_from_text(text) {
        Ok(value) => {
            *key = KeyState::Ready(value);
            Status::Normal as u32
        }
        Err(_) => {
            drop(key);
            unsafe { failed(frame, site, 100) }
        }
    }
}
#[export_name = "nyash.map.checked_install_indexed_v1"]
pub unsafe extern "C" fn install(
    frame: *mut c_void,
    profile: u32,
    site: u64,
    map_ptr: *mut c_void,
    key_ptr: *mut c_void,
    handle: i64,
    type_id: i64,
    out_ptr: *mut c_void,
) -> u32 {
    unsafe {
        install_candidate(frame, profile, site, map_ptr, key_ptr, out_ptr, || {
            checked_map_residence::prepare(TypedObjectStoreBackend::SafeMutex, handle, type_id)
        })
    }
}

// Checked Map value ABI kinds, not object storage tags or source capabilities.
const MAP_VALUE_I64: u32 = 1;
const MAP_VALUE_BOOL: u32 = 2;
#[export_name = "nyash.map.checked_install_value_v1"]
pub unsafe extern "C" fn install_value(
    frame: *mut c_void,
    profile: u32,
    site: u64,
    map_ptr: *mut c_void,
    key_ptr: *mut c_void,
    kind: u32,
    payload: i64,
    out_ptr: *mut c_void,
) -> u32 {
    let value = match (kind, payload) {
        (MAP_VALUE_I64, value) => CheckedMapPayload::I64(value),
        (MAP_VALUE_BOOL, 0 | 1) => CheckedMapPayload::Bool(payload == 1),
        _ => return Status::InvalidContract as u32,
    };
    unsafe {
        install_candidate(frame, profile, site, map_ptr, key_ptr, out_ptr, || {
            Ok(value)
        })
    }
}

// Preflight precedes key consumption; candidate preparation follows it. Both
// exports use this one commit/outcome protocol, preserving Indexed fault order.
unsafe fn install_candidate(
    frame: *mut c_void,
    profile: u32,
    site: u64,
    map_ptr: *mut c_void,
    key_ptr: *mut c_void,
    out_ptr: *mut c_void,
    prepare: impl FnOnce() -> Result<CheckedMapPayload, CheckedStorageError>,
) -> u32 {
    if !separate(&[
        (frame as usize, size_of::<FaultFrame>()),
        (map_ptr as usize, size_of::<MapStorage>()),
        (key_ptr as usize, size_of::<KeyStorage>()),
        (out_ptr as usize, size_of::<OutcomeStorage>()),
    ]) || !unsafe { valid_frame(frame) }
        || profile != 1
        || store::check_indexed_profile(TypedObjectStoreBackend::SafeMutex).is_err()
    {
        return Status::InvalidContract as u32;
    }
    let map = match unsafe { admit::<CheckedMap>(map_ptr, MAP_TAG) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    if map.require_live().is_err() {
        return Status::InvalidContract as u32;
    }
    let outcome = match unsafe { admit::<Mutex<OutcomeState>>(out_ptr, OUT_TAG) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let mut outcome = match outcome.lock() {
        Ok(v) => v,
        Err(_) => return Status::InvalidContract as u32,
    };
    if !matches!(*outcome, OutcomeState::Unissued) {
        return Status::InvalidContract as u32;
    }
    let key = {
        let key = match unsafe { admit::<Mutex<KeyState>>(key_ptr, KEY_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        let mut key = match key.lock() {
            Ok(v) => v,
            Err(_) => return Status::InvalidContract as u32,
        };
        if !matches!(*key, KeyState::Ready(_)) {
            return Status::InvalidContract as u32;
        }
        match std::mem::replace(&mut *key, KeyState::Consumed) {
            KeyState::Ready(v) => v,
            _ => unreachable!(),
        }
    };
    let candidate = match prepare() {
        Ok(value) => value,
        Err(error) => {
            drop(outcome);
            let reason = match error {
                CheckedStorageError::AllocationOrStorageUnavailable => 100,
                _ => 101,
            };
            return unsafe { failed(frame, site, reason) };
        }
    };
    match map.install(key, candidate) {
        Ok(old) => {
            *outcome = OutcomeState::Ready(old);
            Status::Normal as u32
        }
        Err(error) => {
            drop(outcome);
            match root_reason(error.error) {
                Some(reason) => unsafe { failed(frame, site, reason) },
                None => Status::InvalidContract as u32,
            }
        }
    }
}
#[export_name = "nyash.map.outcome_end_v1"]
pub unsafe extern "C" fn outcome_end(frame: *mut c_void, site: u64, ptr: *mut c_void) -> u32 {
    if !separate(&[
        (frame as usize, size_of::<FaultFrame>()),
        (ptr as usize, size_of::<OutcomeStorage>()),
    ]) || !unsafe { valid_frame(frame) }
    {
        return Status::InvalidContract as u32;
    }
    let old = {
        let outcome = match unsafe { admit::<Mutex<OutcomeState>>(ptr, OUT_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        let mut outcome = match outcome.lock() {
            Ok(v) => v,
            Err(_) => return Status::InvalidContract as u32,
        };
        if !matches!(*outcome, OutcomeState::Ready(_)) {
            return Status::InvalidContract as u32;
        }
        match std::mem::replace(&mut *outcome, OutcomeState::Consumed) {
            OutcomeState::Ready(v) => v,
            _ => unreachable!(),
        }
    };
    match old.end() {
        Ok(()) => Status::Normal as u32,
        Err(error) => unsafe { failed(frame, site, end_reason(error)) },
    }
}
#[export_name = "nyash.map.checked_end_v1"]
pub unsafe extern "C" fn map_end(frame: *mut c_void, site: u64, ptr: *mut c_void) -> u32 {
    if !separate(&[
        (frame as usize, size_of::<FaultFrame>()),
        (ptr as usize, size_of::<MapStorage>()),
    ]) || !unsafe { valid_frame(frame) }
    {
        return Status::InvalidContract as u32;
    }
    let result = {
        let map = match unsafe { admit::<CheckedMap>(ptr, MAP_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        map.end()
    };
    match result {
        Ok(result) => unsafe { report(frame, site, result) },
        Err(_) => Status::InvalidContract as u32,
    }
}
#[export_name = "nyash.map.storage_dispose_v1"]
pub unsafe extern "C" fn map_dispose(ptr: *mut c_void) -> u32 {
    {
        let map = match unsafe { admit::<CheckedMap>(ptr, MAP_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        if map.require_disposable().is_err() {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<CheckedMap>(ptr) }
}
#[export_name = "nyash.map.key_dispose_v1"]
pub unsafe extern "C" fn key_dispose(ptr: *mut c_void) -> u32 {
    {
        let key = match unsafe { admit::<Mutex<KeyState>>(ptr, KEY_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        if key.lock().is_err() {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<Mutex<KeyState>>(ptr) }
}
#[export_name = "nyash.map.outcome_dispose_v1"]
pub unsafe extern "C" fn outcome_dispose(ptr: *mut c_void) -> u32 {
    {
        let out = match unsafe { admit::<Mutex<OutcomeState>>(ptr, OUT_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        let out = match out.lock() {
            Ok(v) => v,
            Err(_) => return Status::InvalidContract as u32,
        };
        if matches!(*out, OutcomeState::Ready(_)) {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<Mutex<OutcomeState>>(ptr) }
}
#[cfg(test)]
#[path = "fault_checked_map_tests.rs"]
mod tests;
