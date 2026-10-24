//! Checked C boundary for source-selected ordinary Dynamic invocation.
//! Provider selection and opaque carrier ownership remain in Nyash runtime.
use super::{Diagnostic, FaultFrame, Status};
use nyash_rust::runtime::dynamic_invocation::{
    self as invocation, DynamicCarrierTokenV1, DynamicInvocationFailureV1, DynamicInvocationValueV1,
};
use std::{ffi::c_void, mem::size_of};

const TAGGED: u32 = 1;
const INTEGER: u32 = 2;
const BOOL: u32 = 3;
const CARRIER: u32 = 4;
const DYNAMIC_INVOCATION_FAULT: u32 = 400;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Operand {
    pub kind: u32,
    pub tag: u32,
    pub payload: u64,
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

fn record_failure(frame: &mut FaultFrame, site: u64, failure: DynamicInvocationFailureV1) -> u32 {
    let code = match failure {
        DynamicInvocationFailureV1::InvalidCarrier => 1,
        DynamicInvocationFailureV1::MissingProvider => 2,
        DynamicInvocationFailureV1::AmbiguousProvider => 3,
        DynamicInvocationFailureV1::ArgumentTypeMismatch => 4,
        DynamicInvocationFailureV1::CarrierSpaceExhausted => 5,
    };
    frame
        .record(Diagnostic::new(DYNAMIC_INVOCATION_FAULT, site, [code, 0]))
        .unwrap_or(Status::InvalidContract) as u32
}

unsafe fn decode(value: &Operand) -> Result<DynamicInvocationValueV1, Status> {
    match value.kind {
        TAGGED => match value.tag {
            0 if value.payload == 0 => Ok(DynamicInvocationValueV1::Null),
            1 => Ok(DynamicInvocationValueV1::Integer(value.payload as i64)),
            2 if value.payload <= 1 => Ok(DynamicInvocationValueV1::Bool(value.payload == 1)),
            3 if value.payload != 0 => {
                let handle = value.payload;
                if nyash_rust::runtime::weak_handles::is_weak_handle(handle as i64) {
                    let weak = nyash_rust::runtime::weak_handles::get_weak(handle as i64)
                        .ok_or(Status::InvalidContract)?;
                    Ok(DynamicInvocationValueV1::Weak(weak))
                } else {
                    let object = nyash_rust::runtime::host_handles::get(handle)
                        .ok_or(Status::InvalidContract)?;
                    Ok(DynamicInvocationValueV1::Object(object))
                }
            }
            _ => Err(Status::InvalidContract),
        },
        INTEGER if value.tag == 0 => Ok(DynamicInvocationValueV1::Integer(value.payload as i64)),
        BOOL if value.tag == 0 && value.payload <= 1 => {
            Ok(DynamicInvocationValueV1::Bool(value.payload == 1))
        }
        CARRIER if value.tag == 0 => {
            let token = DynamicCarrierTokenV1::from_raw_v1(value.payload)
                .map_err(|_| Status::InvalidContract)?;
            Ok(DynamicInvocationValueV1::Carrier(token))
        }
        _ => Err(Status::InvalidContract),
    }
}

unsafe fn operand<'a>(value: *const Operand) -> Result<&'a Operand, Status> {
    if value.is_null() || !value.is_aligned() {
        return Err(Status::InvalidContract);
    }
    // The caller guarantees one live aligned descriptor.
    Ok(unsafe { &*value })
}

/// Execute exactly one provider selected by runtime class, source selector,
/// and exact source arity. No output carrier is written before Normal.
#[export_name = "nyash.dynamic.checked_invoke_v1"]
pub unsafe extern "C" fn invoke(
    storage: *mut c_void,
    site: u64,
    receiver: *const Operand,
    selector_bytes: *const u8,
    selector_len: usize,
    arguments: *const Operand,
    argument_count: usize,
    out: *mut u64,
) -> u32 {
    if out.is_null()
        || !out.is_aligned()
        || selector_len > isize::MAX as usize
        || argument_count > (isize::MAX as usize) / size_of::<Operand>()
        || (selector_len != 0 && selector_bytes.is_null())
        || (argument_count != 0 && (arguments.is_null() || !arguments.is_aligned()))
    {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let receiver = match unsafe { operand(receiver) }.and_then(|value| unsafe { decode(value) }) {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    let selector = if selector_len == 0 {
        ""
    } else {
        let bytes = unsafe { std::slice::from_raw_parts(selector_bytes, selector_len) };
        match std::str::from_utf8(bytes) {
            Ok(selector) => selector,
            Err(_) => return Status::InvalidContract as u32,
        }
    };
    let argument_rows = if argument_count == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(arguments, argument_count) }
    };
    let mut decoded = Vec::with_capacity(argument_count);
    for argument in argument_rows {
        match unsafe { decode(argument) } {
            Ok(value) => decoded.push(value),
            Err(status) => return status as u32,
        }
    }
    let plan = match invocation::plan_dynamic_invocation_v1(receiver, selector, decoded) {
        Ok(plan) => plan,
        Err(failure) => return record_failure(frame, site, failure),
    };
    match plan.execute_once_v1() {
        Ok(carrier) => {
            unsafe { out.write(carrier.raw_v1()) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Borrow an opaque carrier as an integer for a separately admitted Dynamic
/// operator. A non-integer payload records TypeError and leaves `out`
/// untouched; this read does not consume the carrier.
#[export_name = "nyash.dynamic.checked_integer_v1"]
pub unsafe extern "C" fn integer_payload(
    storage: *mut c_void,
    site: u64,
    raw: u64,
    out: *mut i64,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let token = match DynamicCarrierTokenV1::from_raw_v1(raw) {
        Ok(token) => token,
        Err(_) => return Status::InvalidContract as u32,
    };
    match token.integer_payload_v1() {
        Ok(value) => {
            unsafe { out.write(value) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Borrow an integer carrier and publish one fresh subtraction result. The
/// input remains live on both Normal and Fault; the output is untouched on
/// Fault.
#[export_name = "nyash.dynamic.checked_sub_i64_v1"]
pub unsafe extern "C" fn subtract_i64(
    storage: *mut c_void,
    site: u64,
    raw: u64,
    rhs: i64,
    out: *mut u64,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let token = match DynamicCarrierTokenV1::from_raw_v1(raw) {
        Ok(token) => token,
        Err(_) => return Status::InvalidContract as u32,
    };
    match token.subtract_i64_v1(rhs) {
        Ok(result) => {
            unsafe { out.write(result.raw_v1()) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Consume one opaque carrier. A second End is a source Fault; malformed
/// tokens are rejected as invalid physical contracts before table access.
#[export_name = "nyash.dynamic.checked_end_v1"]
pub unsafe extern "C" fn end(storage: *mut c_void, site: u64, raw: u64) -> u32 {
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let token = match DynamicCarrierTokenV1::from_raw_v1(raw) {
        Ok(token) => token,
        Err(_) => return Status::InvalidContract as u32,
    };
    match token.end_v1() {
        Ok(()) => Status::Normal as u32,
        Err(failure) => record_failure(frame, site, failure),
    }
}

pub(super) unsafe fn end_array_slot(
    storage: *mut c_void,
    site: u64,
    raw: u64,
    owner_identity: usize,
) -> u32 {
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let token = match DynamicCarrierTokenV1::from_raw_v1(raw) {
        Ok(token) => token,
        Err(_) => return Status::InvalidContract as u32,
    };
    match token.end_array_slot_v1(owner_identity) {
        Ok(()) => Status::Normal as u32,
        Err(failure) => record_failure(frame, site, failure),
    }
}
