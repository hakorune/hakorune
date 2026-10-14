//! Checked expression String ABI. The compiler owns source permissions and the
//! Normal/Fault acquisition order; the existing lease owner owns exact End.
use super::{Diagnostic, FaultFrame, Status};
use nyash_rust::runtime::dynamic_v2_lease as lease;
use std::{ffi::c_void, io::Write, num::NonZeroU64};

const STATIC_TEXT: u32 = 1;
const OWNED_TEXT: u32 = 2;
const NORMAL_INTEGER: u32 = 3;
const WRITE_FAILED: u32 = 300;
const ALLOCATION_FAILED: u32 = 301;

/// Borrow-only operand descriptor. Inactive fields must be zero. Static bytes
/// are valid UTF-8 and remain live throughout the synchronous call.
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Operand {
    pub kind: u32,
    pub reserved: u32,
    pub bytes: *const u8,
    pub len: usize,
    pub handle: u64,
    pub token: u64,
    pub integer: i64,
}

/// Two scalar wire words, never an ordinary reusable Object handle.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct OwnedText {
    pub handle: u64,
    pub token: u64,
}

unsafe fn frame<'a>(storage: *mut c_void) -> Result<&'a mut FaultFrame, Status> {
    if storage.is_null() || !storage.cast::<FaultFrame>().is_aligned() {
        return Err(Status::InvalidContract);
    }
    // The caller guarantees live aligned exclusive FaultFrame storage.
    let frame = unsafe { &mut *storage.cast::<FaultFrame>() };
    if !frame.valid() {
        return Err(Status::InvalidContract);
    }
    Ok(frame)
}

fn fault(frame: &mut FaultFrame, site: u64, reason: u32) -> u32 {
    frame
        .record(Diagnostic::new(reason, site, [0, 0]))
        .unwrap_or(Status::InvalidContract) as u32
}

unsafe fn operand<'a>(value: *const Operand) -> Result<&'a Operand, Status> {
    if value.is_null() || !value.is_aligned() {
        return Err(Status::InvalidContract);
    }
    let value = unsafe { &*value };
    if value.reserved != 0 {
        return Err(Status::InvalidContract);
    }
    Ok(value)
}

/// Copy each borrowed operand in its own scope: publishing or ending while a
/// registry text callback holds its read lock would deadlock. No Box stringify.
unsafe fn text(value: &Operand) -> Result<String, Status> {
    let no_bytes = value.bytes.is_null() && value.len == 0;
    match value.kind {
        STATIC_TEXT if value.handle == 0 && value.token == 0 && value.integer == 0 => {
            if value.len > isize::MAX as usize || (value.len != 0 && value.bytes.is_null()) {
                return Err(Status::InvalidContract);
            }
            let bytes = if value.len == 0 {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(value.bytes, value.len) }
            };
            std::str::from_utf8(bytes)
                .map(str::to_owned)
                .map_err(|_| Status::InvalidContract)
        }
        OWNED_TEXT if no_bytes && value.integer == 0 => {
            let token = NonZeroU64::new(value.token).ok_or(Status::InvalidContract)?;
            lease::with_end_authorized_text_pair(value.handle, token, str::to_owned)
                .map_err(|_| Status::InvalidContract)
        }
        NORMAL_INTEGER if no_bytes && value.handle == 0 && value.token == 0 => {
            Ok(value.integer.to_string())
        }
        _ => Err(Status::InvalidContract),
    }
}

/// Caller guarantees live nonoverlapping frame/operands/out storage. Only
/// String+String or String+NormalInteger is this ABI; integer Add stays scalar.
/// Normal alone writes both result words and creates one fresh End obligation.
#[export_name = "nyash.string.checked_add_v1"]
pub unsafe extern "C" fn add(
    storage: *mut c_void,
    site: u64,
    left: *const Operand,
    right: *const Operand,
    out: *mut OwnedText,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let left = match unsafe { operand(left) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let right = match unsafe { operand(right) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    if !matches!(left.kind, STATIC_TEXT | OWNED_TEXT)
        || !matches!(right.kind, STATIC_TEXT | OWNED_TEXT | NORMAL_INTEGER)
    {
        return Status::InvalidContract as u32;
    }
    let mut result = match unsafe { text(left) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let suffix = match unsafe { text(right) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    if result.try_reserve(suffix.len()).is_err() {
        return fault(frame, site, ALLOCATION_FAILED);
    }
    result.push_str(&suffix);
    let owned = match lease::publish_end_authorized_text(result) {
        Ok(v) => v,
        Err(_) => return fault(frame, site, ALLOCATION_FAILED),
    };
    unsafe {
        out.write(OwnedText {
            handle: owned.handle(),
            token: owned.token().get(),
        })
    };
    Status::Normal as u32
}

pub(super) fn print_to(
    frame: &mut FaultFrame,
    site: u64,
    bytes: &[u8],
    sink: &mut impl Write,
) -> u32 {
    if sink
        .write_all(bytes)
        .and_then(|()| sink.write_all(b"\n"))
        .is_err()
    {
        fault(frame, site, WRITE_FAILED)
    } else {
        Status::Normal as u32
    }
}

/// Borrow the complete root value; Print never consumes a String lease.
#[export_name = "nyash.string.checked_print_v1"]
pub unsafe extern "C" fn print(storage: *mut c_void, site: u64, value: *const Operand) -> u32 {
    let frame = match unsafe { frame(storage) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let value = match unsafe { operand(value) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let value = match unsafe { text(value) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    print_to(frame, site, value.as_bytes(), &mut std::io::stdout().lock())
}

/// End runs after Print Normal or on a later Fault. Broken identity is a
/// physical InvalidContract, never a source Fault or permission to drop raw.
#[export_name = "nyash.string.checked_end_v1"]
pub unsafe extern "C" fn end(storage: *mut c_void, _site: u64, handle: u64, token: u64) -> u32 {
    if let Err(s) = unsafe { frame(storage) } {
        return s as u32;
    }
    let Some(token) = NonZeroU64::new(token) else {
        return Status::InvalidContract as u32;
    };
    match lease::end_authorized_text_pair(handle, token) {
        Ok(()) => Status::Normal as u32,
        Err(_) => Status::InvalidContract as u32,
    }
}
