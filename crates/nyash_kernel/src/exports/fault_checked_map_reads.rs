//! Checked Map scalar, presence and indexed-borrow reads share admission.
//! No read acquires Home; source holder/slot support is a separate final proof.
use super::*;

unsafe fn admit_read<'a>(
    frame: *mut c_void, site: u64, map_ptr: *mut c_void,
    bytes: *const u8, len: usize, out: *mut i64,
) -> Result<(&'a CheckedMap, MapKeyDomain), u32> {
    if !out.is_aligned() || len > isize::MAX as usize
        || !separate(&[
            (frame as usize, size_of::<FaultFrame>()),
            (map_ptr as usize, size_of::<MapStorage>()),
            (bytes as usize, len),
            (out as usize, size_of::<i64>()),
        ])
        || !unsafe { valid_frame(frame) }
    {
        return Err(Status::InvalidContract as u32);
    }
    let map = match unsafe { admit::<CheckedMap>(map_ptr, MAP_TAG) } {
        Ok(v) => v,
        Err(s) => return Err(s as u32),
    };
    if map.require_live().is_err() {
        return Err(Status::InvalidContract as u32);
    }
    let slice = if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(bytes, len) }
    };
    let text = match std::str::from_utf8(slice) {
        Ok(text) => text,
        Err(_) => return Err(Status::InvalidContract as u32),
    };
    let key = match MapKeyDomain::try_from_text(text) {
        Ok(key) => key,
        Err(_) => return Err(unsafe { failed(frame, site, 100) }),
    };
    Ok((map, key))
}

/// Bounded scalar read for the borrowed-argument lane. The key arrives as
/// caller-supplied UTF-8 input (validated before use, like install_text) —
/// no key storage is prepared or consumed. Missing writes `0` and returns
/// Normal; a present non-i64 payload records Fault; `out` is written only
/// on Normal. The read consumes nothing and leaves the map live.
#[export_name = "nyash.map.checked_get_i64_v1"]
pub unsafe extern "C" fn get_i64(
    frame: *mut c_void,
    site: u64,
    map_ptr: *mut c_void,
    bytes: *const u8,
    len: usize,
    out: *mut i64,
) -> u32 {
    let (map, key) = match unsafe { admit_read(frame, site, map_ptr, bytes, len, out) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match map.read_i64(&key) {
        Ok(CheckedMapI64Read::Missing) => {
            unsafe { out.write(0) };
            Status::Normal as u32
        }
        Ok(CheckedMapI64Read::Value(value)) => {
            unsafe { out.write(value) };
            Status::Normal as u32
        }
        Ok(CheckedMapI64Read::NonScalar) => unsafe {
            failed(frame, site, MAP_GET_NON_SCALAR_REASON)
        },
        Err(error) => match root_reason(error) {
            Some(reason) => unsafe { failed(frame, site, reason) },
            None => Status::InvalidContract as u32,
        },
    }
}


// Scalar-read outcome reasons (mirrored in include/nyrt_fault_v1.h).
const MAP_ARRAY_LENGTH_MISSING_REASON: u32 = 111;
const MAP_ARRAY_LENGTH_NON_ARRAY_REASON: u32 = 112;

/// Read the length of an exact checked Array entry. Empty-array markers return
/// zero without allocating a residence; non-empty arrays return their sealed
/// residence length. Missing and non-array entries are named Faults.
#[export_name = "nyash.map.checked_array_length_v1"]
pub unsafe extern "C" fn array_length(
    frame: *mut c_void,
    site: u64,
    map_ptr: *mut c_void,
    bytes: *const u8,
    len: usize,
    out: *mut i64,
) -> u32 {
    let (map, key) = match unsafe { admit_read(frame, site, map_ptr, bytes, len, out) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match map.read_array_length(&key) {
        Ok(CheckedMapArrayLengthRead::Value(value)) => {
            let value = match i64::try_from(value) {
                Ok(value) => value,
                Err(_) => return unsafe { failed(frame, site, 100) },
            };
            unsafe { out.write(value) };
            Status::Normal as u32
        }
        Ok(CheckedMapArrayLengthRead::Missing) => unsafe {
            failed(frame, site, MAP_ARRAY_LENGTH_MISSING_REASON)
        },
        Ok(CheckedMapArrayLengthRead::NonArray) => unsafe {
            failed(frame, site, MAP_ARRAY_LENGTH_NON_ARRAY_REASON)
        },
        Err(error) => match root_reason(error) {
            Some(reason) => unsafe { failed(frame, site, reason) },
            None => Status::InvalidContract as u32,
        },
    }
}

/// Kind-independent presence. The result is exactly zero or one on Normal.
#[export_name = "nyash.map.checked_has_v1"]
pub unsafe extern "C" fn has(
    frame: *mut c_void, site: u64, map_ptr: *mut c_void,
    bytes: *const u8, len: usize, out: *mut i64,
) -> u32 {
    let (map, key) = match unsafe { admit_read(frame, site, map_ptr, bytes, len, out) } {
        Ok(value) => value, Err(status) => return status,
    };
    match map.contains_key(&key) {
        Ok(present) => { unsafe { out.write(i64::from(present)) }; Status::Normal as u32 },
        Err(error) => unsafe { read_error(frame, site, error) },
    }
}

/// Nullable nonowning indexed-object snapshot. Missing writes zero; wrong
/// payload/class Faults without touching out. The live identity check is not a
/// source lifetime proof: no Call/Return/claim or End authority is granted.
#[export_name = "nyash.map.checked_borrow_indexed_v1"]
pub unsafe extern "C" fn borrow_indexed(
    frame: *mut c_void, profile: u32, site: u64, map_ptr: *mut c_void,
    bytes: *const u8, len: usize, type_id: i64, out: *mut i64,
) -> u32 {
    if profile != 1 || type_id <= 0
        || store::check_indexed_profile(TypedObjectStoreBackend::SafeMutex).is_err() {
        return Status::InvalidContract as u32;
    }
    let (map, key) = match unsafe { admit_read(frame, site, map_ptr, bytes, len, out) } {
        Ok(value) => value, Err(status) => return status,
    };
    // read_indexed releases its Map guard before taking the object guard.
    // Never hold Map -> object locks against field installation's reverse order.
    match map.read_indexed(&key) {
        Ok(None) => { unsafe { out.write(0) }; Status::Normal as u32 },
        Ok(Some((handle, actual_type))) if actual_type == type_id => {
            match store::validate_checked_indexed_identity(TypedObjectStoreBackend::SafeMutex, handle, type_id) {
                Ok(()) => { unsafe { out.write(handle) }; Status::Normal as u32 },
                Err(CheckedStorageError::AllocationOrStorageUnavailable) => unsafe { failed(frame, site, 100) },
                Err(_) => Status::InvalidContract as u32,
            }
        },
        Ok(Some(_)) => unsafe { failed(frame, site, 101) },
        Err(error) => unsafe { read_error(frame, site, error) },
    }
}

unsafe fn read_error(frame: *mut c_void, site: u64, error: CheckedMapError) -> u32 {
    match error {
        CheckedMapError::ProjectionUnavailable => unsafe { failed(frame, site, 101) },
        error => match root_reason(error) {
            Some(reason) => unsafe { failed(frame, site, reason) },
            None => Status::InvalidContract as u32,
        },
    }
}
