//! Physical Array ABI only. Source/Recipe selects operations and owns cleanup.
//! ArrayStateCell alone validates and commits; FaultFrame alone records failures.
use super::{Diagnostic, FaultFrame, Status};
use crate::plugin::with_array_box_direct;
use nyash_rust::boxes::array::{
    ArrayBox, ArrayPrimitiveWriteError, TypedArrayRuntimeContractError,
};
use nyash_rust::runtime::host_handles as handles;
use nyash_rust::typed_array_contract_spec::{ArrayElementContractSpec, ExactArrayElementType};
use std::{ffi::c_void, sync::Arc};

const ELEMENT_I8: u32 = 1;
const ELEMENT_I16: u32 = 2;
const ELEMENT_I32: u32 = 3;
const ELEMENT_I64: u32 = 4;
const ELEMENT_U8: u32 = 5;
const ELEMENT_U16: u32 = 6;
const ELEMENT_U32: u32 = 7;
const ARRAY_CLAIM_CONFLICT: u32 = 200;
const ARRAY_EXISTING_ELEMENT_MISMATCH: u32 = 201;
const ARRAY_APPEND_ELEMENT_MISMATCH: u32 = 202;
const TYPE_MISMATCH: i64 = 1;
const NEGATIVE_TO_UNSIGNED: i64 = 2;
const OUT_OF_RANGE: i64 = 3;

fn element_spec(tag: u32) -> Result<ArrayElementContractSpec, Status> {
    let element = match tag {
        ELEMENT_I8 => ExactArrayElementType::I8,
        ELEMENT_I16 => ExactArrayElementType::I16,
        ELEMENT_I32 => ExactArrayElementType::I32,
        ELEMENT_I64 => ExactArrayElementType::I64,
        ELEMENT_U8 => ExactArrayElementType::U8,
        ELEMENT_U16 => ExactArrayElementType::U16,
        ELEMENT_U32 => ExactArrayElementType::U32,
        _ => return Err(Status::InvalidContract),
    };
    Ok(ArrayElementContractSpec { element })
}

fn subtype(reason: &str) -> Result<i64, Status> {
    match reason {
        "runtime-type-mismatch" => Ok(TYPE_MISMATCH),
        "negative-to-unsigned" => Ok(NEGATIVE_TO_UNSIGNED),
        "out-of-range" => Ok(OUT_OF_RANGE),
        _ => Err(Status::InvalidContract),
    }
}

unsafe fn admit<'a>(storage: *mut c_void) -> Result<&'a mut FaultFrame, Status> {
    if storage.is_null() {
        return Err(Status::InvalidContract);
    }
    // SAFETY: caller supplies live aligned storage, exclusively borrowed here.
    let frame = unsafe { &mut *storage.cast::<FaultFrame>() };
    if !frame.valid() {
        return Err(Status::InvalidContract);
    }
    Ok(frame)
}

fn record(frame: &mut FaultFrame, diagnostic: Result<Diagnostic, Status>) -> u32 {
    match diagnostic {
        Ok(diagnostic) => frame.record(diagnostic).unwrap_or(Status::InvalidContract) as u32,
        Err(status) => status as u32,
    }
}

fn claim_diagnostic(
    error: TypedArrayRuntimeContractError,
    site: u64,
    requested: u32,
) -> Result<Diagnostic, Status> {
    let (reason, details) = match error {
        TypedArrayRuntimeContractError::StateConflict => {
            (ARRAY_CLAIM_CONFLICT, [i64::from(requested), 0])
        }
        TypedArrayRuntimeContractError::ExistingElementMismatch { index, reason } => (
            ARRAY_EXISTING_ELEMENT_MISMATCH,
            [
                i64::try_from(index).map_err(|_| Status::InvalidContract)?,
                subtype(reason)?,
            ],
        ),
    };
    Ok(Diagnostic::new(reason, site, details))
}

/// Trusted synchronous pointers: live aligned frame, exclusive borrow and a
/// writable nonoverlapping out-slot. Null/header rejection is not pointer proof.
/// Only Normal publishes out. Fatal allocator termination is not returned Fault.
#[export_name = "nyash.array.checked_new_v1"]
pub unsafe extern "C" fn allocate(storage: *mut c_void, _site: u64, out: *mut i64) -> u32 {
    if out.is_null() {
        return Status::InvalidContract as u32;
    }
    if let Err(status) = unsafe { admit(storage) } {
        return status as u32;
    }
    let raw = handles::to_handle_arc(Arc::new(ArrayBox::new()));
    let handle = match i64::try_from(raw) {
        Ok(handle) if handle > 0 => handle,
        _ => {
            handles::drop_handle(raw);
            return Status::InvalidContract as u32;
        }
    };
    // SAFETY: validated nonnull out; remaining pointer validity is caller-owned.
    unsafe { out.write(handle) };
    Status::Normal as u32
}

/// Same trusted frame contract as allocate. The issued spec is a physical tag;
/// no source spelling lookup, storage inference or alternate backend selection.
#[export_name = "nyash.array.checked_claim_v1"]
pub unsafe extern "C" fn claim(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    element_tag: u32,
) -> u32 {
    let frame = match unsafe { admit(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let spec = match element_spec(element_tag) {
        Ok(spec) => spec,
        Err(status) => return status as u32,
    };
    match with_array_box_direct(handle, |array| array.claim_element_contract(spec)) {
        Some(Ok(())) => Status::Normal as u32,
        Some(Err(error)) => record(frame, claim_diagnostic(error, site, element_tag)),
        None => Status::InvalidContract as u32,
    }
}

fn append_result(
    frame: &mut FaultFrame,
    site: u64,
    result: Option<Result<usize, ArrayPrimitiveWriteError>>,
) -> u32 {
    match result {
        Some(Ok(_)) => Status::Normal as u32,
        Some(Err(ArrayPrimitiveWriteError::ElementContract { reason })) => record(
            frame,
            subtype(reason)
                .map(|kind| Diagnostic::new(ARRAY_APPEND_ELEMENT_MISMATCH, site, [kind, 0])),
        ),
        None
        | Some(Err(
            ArrayPrimitiveWriteError::InvalidIndex | ArrayPrimitiveWriteError::UnsupportedStorage,
        )) => Status::InvalidContract as u32,
    }
}

/// Explicit integer payload, never a handle-or-integer carrier. Same frame contract.
#[export_name = "nyash.array.checked_append_i64_v1"]
pub unsafe extern "C" fn append_i64(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    value: i64,
) -> u32 {
    let frame = match unsafe { admit(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    append_result(
        frame,
        site,
        with_array_box_direct(handle, |array| array.slot_append_i64_result(value)),
    )
}

/// Bool uses exactly 0/1 on the wire. Same trusted frame contract.
#[export_name = "nyash.array.checked_append_bool_v1"]
pub unsafe extern "C" fn append_bool(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    value: u32,
) -> u32 {
    let frame = match unsafe { admit(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let value = match value {
        0 => false,
        1 => true,
        _ => return Status::InvalidContract as u32,
    };
    append_result(
        frame,
        site,
        with_array_box_direct(handle, |array| array.slot_append_bool_result(value)),
    )
}

/// F64 uses a double payload without integer conversion. Same frame contract.
#[export_name = "nyash.array.checked_append_f64_v1"]
pub unsafe extern "C" fn append_f64(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    value: f64,
) -> u32 {
    let frame = match unsafe { admit(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    append_result(
        frame,
        site,
        with_array_box_direct(handle, |array| array.slot_append_f64_result(value)),
    )
}

#[cfg(test)]
#[path = "fault_checked_array_tests.rs"]
mod tests;
