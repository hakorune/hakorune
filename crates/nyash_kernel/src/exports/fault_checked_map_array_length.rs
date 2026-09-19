//! Checked Array length export for the bounded map-array read lane.
use super::*;

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
    if len > isize::MAX as usize
        || !separate(&[
            (frame as usize, size_of::<FaultFrame>()),
            (map_ptr as usize, size_of::<MapStorage>()),
            (bytes as usize, len),
            (out as usize, size_of::<i64>()),
        ])
        || !unsafe { valid_frame(frame) }
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
    let slice = if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(bytes, len) }
    };
    let text = match std::str::from_utf8(slice) {
        Ok(text) => text,
        Err(_) => return Status::InvalidContract as u32,
    };
    let key = match MapKeyDomain::try_from_text(text) {
        Ok(key) => key,
        Err(_) => return unsafe { failed(frame, site, 100) },
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
