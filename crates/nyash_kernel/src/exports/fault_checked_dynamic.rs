//! Checked C boundary for source-selected ordinary Dynamic invocation.
//! Provider selection and opaque carrier ownership remain in Nyash runtime.
use super::{Diagnostic, FaultFrame, Status};
use nyash_rust::runtime::dynamic_invocation::{
    self as invocation, DynamicCarrierTokenV1, DynamicInvocationFailureV1, DynamicInvocationValueV1,
};
use std::{ffi::c_void, mem::size_of};

pub(super) const TAGGED: u32 = 1;
pub(super) const INTEGER: u32 = 2;
pub(super) const BOOL: u32 = 3;
pub(super) const CARRIER: u32 = 4;
pub(super) const BORROWED_TEXT_TAG: u32 = 4;
const DYNAMIC_INVOCATION_FAULT: u32 = 400;

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Operand {
    pub kind: u32,
    pub tag: u32,
    pub payload: u64,
}

pub(super) unsafe fn frame<'a>(storage: *mut c_void) -> Result<&'a mut FaultFrame, Status> {
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

pub(super) fn record_failure(frame: &mut FaultFrame, site: u64, failure: DynamicInvocationFailureV1) -> u32 {
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

pub(super) unsafe fn decode(value: &Operand) -> Result<DynamicInvocationValueV1, Status> {
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

pub(super) unsafe fn operand<'a>(value: *const Operand) -> Result<&'a Operand, Status> {
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

/// Evaluate one checked ArrayBox Get and publish its language value as an
/// independent opaque carrier. ArrayBox retains its own OOB/null semantics.
#[export_name = "nyash.dynamic.checked_array_get_v1"]
pub unsafe extern "C" fn array_get(
    storage: *mut c_void,
    site: u64,
    array: i64,
    index: i64,
    out: *mut u64,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let Some(value) = crate::plugin::with_array_box_direct(array, |array| array.get_index_i64(index)) else {
        return Status::InvalidContract as u32;
    };
    match invocation::publish_array_get_v1(value) {
        Ok(carrier) => {
            unsafe { out.write(carrier.raw_v1()) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Return 0/1 for one Dynamic/scalar equality and 2 for malformed transport.
#[export_name = "nyash.dynamic.scalar_equal_v1"]
pub unsafe extern "C" fn scalar_equal(
    program: *const super::checked_program_dispatch::DispatchProgramV1,
    left_kind: u32, left_tag: u32, left_payload: u64,
    right_kind: u32, right_tag: u32, right_payload: u64,
) -> u32 {
    use invocation::DynamicEqualityScalarV1 as Scalar;
    let invalid = Status::InvalidContract as u32;
    let left_dynamic = matches!(left_kind, TAGGED | CARRIER);
    let right_dynamic = matches!(right_kind, TAGGED | CARRIER);
    if left_dynamic == right_dynamic { return invalid; }

    let read = |kind, tag, payload| {
        if kind == TAGGED && tag == BORROWED_TEXT_TAG {
            unsafe { super::checked_string::validate_text_operand(payload as *const _) }?;
            return Ok(None);
        }
        if kind == TAGGED && tag == super::checked_program_dispatch::TYPED_STORE_BORROW {
            let stamp = std::num::NonZeroUsize::new(program.addr()).ok_or(Status::InvalidContract)?;
            crate::exports::typed_object_store_backend::pin_dispatch_receiver(payload as i64, stamp)
                .ok_or(Status::InvalidContract)?;
            return Ok(None);
        }
        match unsafe { decode(&Operand { kind, tag, payload }) }? {
            DynamicInvocationValueV1::Integer(value) => Ok(Some(Scalar::Integer(value))),
            DynamicInvocationValueV1::Bool(value) => Ok(Some(Scalar::Bool(value))),
            DynamicInvocationValueV1::Carrier(token) => token.scalar_equality_operand_v1()
                .map_err(|_| Status::InvalidContract),
            _ => Ok(None),
        }
    };

    match (read(left_kind, left_tag, left_payload), read(right_kind, right_tag, right_payload)) {
        (Ok(left), Ok(right)) => u32::from(left == right),
        _ => invalid,
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
        Err(DynamicInvocationFailureV1::ArgumentTypeMismatch) =>
            super::checked_string::comparison_type_error(frame, site),
        Err(_) => Status::InvalidContract as u32,
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

/// Acquire an independent result from the admitted, still-live borrowed value.
#[export_name = "nyash.dynamic.checked_materialize_borrowed_v1"]
pub unsafe extern "C" fn materialize_borrowed(
    storage: *mut c_void,
    site: u64,
    tag: u32,
    payload: u64,
    out: *mut u64,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let published = match tag {
        0..=2 => {
            let operand = Operand { kind: TAGGED, tag, payload };
            let value = match unsafe { decode(&operand) } {
                Ok(value) => value,
                Err(status) => return status as u32,
            };
            invocation::publish_immediate_v1(value)
        }
        BORROWED_TEXT_TAG => {
            match unsafe { super::checked_string::snapshot_borrowed_text(payload as *const _) } {
                Ok(Some(text)) => invocation::publish_string_v1(text),
                Ok(None) => Err(DynamicInvocationFailureV1::CarrierSpaceExhausted),
                Err(status) => return status as u32,
            }
        }
        _ => return Status::InvalidContract as u32,
    };
    match published {
        Ok(token) => {
            unsafe { out.write(token.raw_v1()) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Move one owned String pair, or the null sentinel pair (0, 0), into a fresh
/// carrier. A String pair is ended only after the carrier exists; any failure
/// leaves the pair for its Fault cleanup.
#[export_name = "nyash.dynamic.checked_publish_text_or_null_v1"]
pub unsafe extern "C" fn publish_text_or_null(
    storage: *mut c_void,
    site: u64,
    handle: u64,
    token: u64,
    out: *mut u64,
) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    let frame = match unsafe { frame(storage) } {
        Ok(frame) => frame,
        Err(status) => return status as u32,
    };
    let published = if handle == 0 && token == 0 {
        invocation::publish_null_v1()
    } else {
        let Some(token) = std::num::NonZeroU64::new(token) else {
            return Status::InvalidContract as u32;
        };
        let Ok(text) = nyash_rust::runtime::dynamic_v2_lease::with_end_authorized_text_pair(
            handle, token, str::to_owned,
        ) else {
            return Status::InvalidContract as u32;
        };
        match invocation::publish_string_v1(text) {
            Ok(carrier) => {
                if nyash_rust::runtime::dynamic_v2_lease::end_authorized_text_pair(handle, token).is_err() {
                    // Publication has not transferred the carrier to the caller.
                    // Retire it even if the original pair became invalid after
                    // the snapshot; the output remains untouched on failure.
                    let _ = carrier.end_v1();
                    return Status::InvalidContract as u32;
                }
                Ok(carrier)
            }
            Err(failure) => Err(failure),
        }
    };
    match published {
        Ok(carrier) => {
            unsafe { out.write(carrier.raw_v1()) };
            Status::Normal as u32
        }
        Err(failure) => record_failure(frame, site, failure),
    }
}

/// Borrow an opaque carrier and report whether it holds null (1) or not (0).
/// This neither consumes the carrier nor records a Fault; a stale or malformed
/// token is an invalid physical contract.
#[export_name = "nyash.dynamic.checked_is_null_v1"]
pub unsafe extern "C" fn is_null(storage: *mut c_void, _site: u64, raw: u64, out: *mut i64) -> u32 {
    if out.is_null() || !out.is_aligned() {
        return Status::InvalidContract as u32;
    }
    if let Err(status) = unsafe { frame(storage) } {
        return status as u32;
    }
    let token = match DynamicCarrierTokenV1::from_raw_v1(raw) {
        Ok(token) => token,
        Err(_) => return Status::InvalidContract as u32,
    };
    match token.is_null_v1() {
        Ok(null) => {
            unsafe { out.write(i64::from(null)) };
            Status::Normal as u32
        }
        Err(_) => Status::InvalidContract as u32,
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::checked_string::{self as text, Operand as TextOperand, OwnedText};

    #[test]
    fn scalar_equality_borrows_exactly_typed_values_and_refuses_stale_tokens() {
        let mut frame = FaultFrame::new();
        let storage = (&mut frame as *mut FaultFrame).cast();
        let mut token = 0;
        let program = std::ptr::null();
        unsafe {
            assert_eq!(materialize_borrowed(storage, 7, 2, 1, &mut token), 0);
            assert_eq!(scalar_equal(program, TAGGED, 2, 1, INTEGER, 0, 1), 0);
            assert_eq!(scalar_equal(program, CARRIER, 0, token, INTEGER, 0, 1), 0);
            assert_eq!(scalar_equal(program, BOOL, 0, 1, CARRIER, 0, token), 1);
            assert_eq!(scalar_equal(program, TAGGED, 2, 2, BOOL, 0, 1), 2);
            assert_eq!(scalar_equal(program, TAGGED, u32::MAX, 0, INTEGER, 0, 0), 2);
            assert_eq!(end(storage, 8, token), 0);
            assert_eq!(scalar_equal(program, CARRIER, 0, token, BOOL, 0, 1), 2);
        }
    }

    #[test]
    fn borrowed_result_owns_exact_tag_and_text_after_source_end() {
        let mut frame = FaultFrame::new();
        let storage = (&mut frame as *mut FaultFrame).cast();
        let mut token = 17;
        for (tag, payload) in [(0, 1), (2, 2), (3, 1), (4, 0)] {
            assert_eq!(unsafe { materialize_borrowed(storage, 7, tag, payload, &mut token) }, 2);
            assert_eq!(token, 17);
        }

        for (tag, payload, expected) in [(0, 0, None), (1, (-7i64) as u64, Some("-7")), (2, 0, Some("false")), (2, 1, Some("true"))] {
            assert_eq!(unsafe { materialize_borrowed(storage, 7, tag, payload, &mut token) }, 0);
            let carrier = DynamicCarrierTokenV1::from_raw_v1(token).unwrap();
            assert_eq!(carrier.is_null_v1().unwrap(), expected.is_none());
            if let Some(expected) = expected {
                assert_eq!(carrier.string_add_suffix_v1().unwrap(), expected);
            }
            assert_eq!(carrier.integer_payload_v1().ok(), (tag == 1).then_some(-7));
            assert_eq!(unsafe { end(storage, 8, token) }, 0);
            assert!(carrier.is_null_v1().is_err());
        }

        let bytes = "猫é".as_bytes();
        let literal = TextOperand {
            kind: 1, reserved: 0, bytes: bytes.as_ptr(), len: bytes.len(),
            handle: 0, token: 0, integer: 0,
        };
        let mut pair = OwnedText { handle: 0, token: 0 };
        assert_eq!(unsafe { text::materialize(storage, 7, &literal, &mut pair) }, 0);
        let borrowed = TextOperand {
            kind: 2, bytes: std::ptr::null(), len: 0, handle: pair.handle, token: pair.token, ..literal
        };
        assert_eq!(unsafe {
            materialize_borrowed(storage, 7, BORROWED_TEXT_TAG, (&borrowed as *const TextOperand) as u64, &mut token)
        }, 0);
        assert_eq!(unsafe { text::end(storage, 8, pair.handle, pair.token) }, 0);
        let carrier = DynamicCarrierTokenV1::from_raw_v1(token).unwrap();
        assert_eq!(carrier.text_snapshot_v1().unwrap(), "猫é");
        assert_eq!(unsafe { end(storage, 8, token) }, 0);
        assert!(carrier.text_snapshot_v1().is_err());
    }
}
