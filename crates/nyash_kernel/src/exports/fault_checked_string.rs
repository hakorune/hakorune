//! Checked expression String ABI. The compiler owns source permissions and the
//! Normal/Fault acquisition order; the existing lease owner owns exact End.
use super::{Diagnostic, FaultFrame, Status};
use nyash_rust::runtime::dynamic_v2_lease as lease;
use nyash_rust::boxes::string_ops::{self, StringIndexMode};
use std::{ffi::c_void, io::Write, num::NonZeroU64};

const STATIC_TEXT: u32 = 1;
const OWNED_TEXT: u32 = 2;
const NORMAL_INTEGER: u32 = 3;
const TAGGED_DYNAMIC: u32 = 4;
const DYNAMIC_CARRIER: u32 = 5;
const WRITE_FAILED: u32 = 300;
const ALLOCATION_FAILED: u32 = 301;
const TYPE_ERROR: u32 = 302;

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
    if (value.kind == TAGGED_DYNAMIC && value.reserved > 3)
        || (value.kind != TAGGED_DYNAMIC && value.reserved != 0) {
        return Err(Status::InvalidContract);
    }
    Ok(value)
}

enum IntegerOperandFailure { TypeError, InvalidContract }

/// Decode only the exact signed Integer carrier. The Dynamic tag remains
/// authoritative: payload shape never upgrades another source type to I64.
unsafe fn integer_operand(value: *const Operand) -> Result<i64, IntegerOperandFailure> {
    let value = unsafe { operand(value) }.map_err(|_| IntegerOperandFailure::InvalidContract)?;
    let no_bytes = value.bytes.is_null() && value.len == 0;
    if value.kind == NORMAL_INTEGER {
        return if no_bytes && value.handle == 0 && value.token == 0 {
            Ok(value.integer)
        } else {
            Err(IntegerOperandFailure::InvalidContract)
        };
    }
    if value.kind != TAGGED_DYNAMIC || !no_bytes || value.handle != 0 || value.token != 0 {
        return Err(IntegerOperandFailure::TypeError);
    }
    match value.reserved {
        1 => Ok(value.integer),
        0 if value.integer == 0 => Err(IntegerOperandFailure::TypeError),
        2 if value.integer == 0 || value.integer == 1 => Err(IntegerOperandFailure::TypeError),
        3 if value.integer != 0 => Err(IntegerOperandFailure::TypeError),
        _ => Err(IntegerOperandFailure::InvalidContract),
    }
}

enum DynamicDisplayFailure { TypeError, InvalidContract }

/// Decode one borrowed original tag/payload pair. The wire tag is not inferred
/// from the payload's MIR type; only tag 3 consults the Box handle registry.
unsafe fn dynamic_text(value: &Operand) -> Result<String, DynamicDisplayFailure> {
    use nyash_rust::runtime::semantics::{display_string_add_value, StringAddDisplayValue as DisplayValue};
    let no_bytes = value.bytes.is_null() && value.len == 0;
    if value.kind != TAGGED_DYNAMIC || !no_bytes || value.handle != 0 || value.token != 0 {
        return Err(DynamicDisplayFailure::InvalidContract);
    }
    let displayed = match value.reserved {
        0 if value.integer == 0 => return Err(DynamicDisplayFailure::TypeError),
        1 => display_string_add_value(DisplayValue::Integer(value.integer)),
        2 if value.integer == 0 || value.integer == 1 =>
            display_string_add_value(DisplayValue::Bool(value.integer == 1)),
        3 => {
            if value.integer == 0 { return Err(DynamicDisplayFailure::InvalidContract); }
            if nyash_rust::runtime::weak_handles::is_weak_handle(value.integer) {
                let Some(weak) = nyash_rust::runtime::weak_handles::get_weak(value.integer) else {
                    return Err(DynamicDisplayFailure::InvalidContract);
                };
                display_string_add_value(DisplayValue::WeakBox { usable_target: weak.upgrade().is_some() })
            } else {
                let Some(object) = nyash_rust::runtime::host_handles::get(value.integer as u64) else {
                    return Err(DynamicDisplayFailure::InvalidContract);
                };
                display_string_add_value(DisplayValue::BoxRef(object.as_ref()))
            }
        }
        _ => return Err(DynamicDisplayFailure::InvalidContract),
    };
    displayed.map_err(|_| DynamicDisplayFailure::TypeError)
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

pub(super) unsafe fn text_operand(value: *const Operand) -> Result<String, Status> {
    let value = unsafe { operand(value) }?;
    if !matches!(value.kind, STATIC_TEXT | OWNED_TEXT) { return Err(Status::InvalidContract); }
    unsafe { text(value) }
}

/// Every successful acquisition, including full/empty slices, is independent.
unsafe fn publish(frame: &mut FaultFrame, site: u64, result: String, out: *mut OwnedText) -> u32 {
    let owned = match lease::publish_end_authorized_text(result) {
        Ok(v) => v,
        Err(_) => return fault(frame, site, ALLOCATION_FAILED),
    };
    unsafe { out.write(OwnedText { handle: owned.handle(), token: owned.token().get() }) };
    Status::Normal as u32
}

/// Exact Text comparison. The wire opcode preserves the original comparison;
/// malformed descriptors/opcodes never publish a result or consume a lease.
#[export_name = "nyash.string.checked_compare_v1"]
pub unsafe extern "C" fn compare(storage: *mut c_void, site: u64, op: u32,
    left: *const Operand, right: *const Operand, out: *mut i64) -> u32 {
    if out.is_null() || !out.is_aligned() || op > 5 { return Status::InvalidContract as u32; }
    let frame = match unsafe { frame(storage) } { Ok(v) => v, Err(s) => return s as u32 };
    let left = match unsafe { comparable(left) } { Ok(v) => v, Err(status) => return status };
    let right = match unsafe { comparable(right) } { Ok(v) => v, Err(status) => return status };
    let both_null = matches!((&left, &right), (Comparable::Null, Comparable::Null));
    let descriptors = [left.text_descriptor(), right.text_descriptor()];
    let result = unsafe { with_text_operands(descriptors.each_ref().map(|value|
        value.as_ref().map(std::ptr::from_ref)), |[left, right]| match (left, right) {
            (Some(left), Some(right)) => Some(compare_text_value(op, left, right)),
            _ if op < 2 => Some(i64::from(both_null == (op == 0))),
            _ => None,
        }) };
    match result {
        Ok(Some(result)) => { unsafe { out.write(result) }; Status::Normal as u32 }
        Ok(None) => fault(frame, site, TYPE_ERROR),
        Err(status) => status as u32,
    }
}

/// Separate valid non-Text payloads from null without assigning them Text equality.
enum Comparable {
    Text(Operand),
    Snapshot(String),
    Null,
    Other,
}
impl Comparable {
    /// A snapshot keeps the carrier's existing copy boundary; it is borrowed
    /// here without another copy and remains alive until the helper returns.
    fn text_descriptor(&self) -> Option<Operand> {
        match self {
            Self::Text(value) => Some(*value),
            Self::Snapshot(value) => Some(Operand { kind: STATIC_TEXT, reserved: 0,
                bytes: value.as_ptr(), len: value.len(), handle: 0, token: 0, integer: 0 }),
            Self::Null | Self::Other => None,
        }
    }
}

/// One descriptor/token decoder for every opaque String carrier consumer.
fn dynamic_carrier(value: &Operand) -> Result<nyash_rust::runtime::dynamic_invocation::DynamicCarrierTokenV1, u32> {
    if value.kind != DYNAMIC_CARRIER || !value.bytes.is_null() || value.len != 0
        || value.handle != 0 || value.integer != 0 || value.token == 0 || value.reserved != 0 {
        return Err(Status::InvalidContract as u32);
    }
    nyash_rust::runtime::dynamic_invocation::DynamicCarrierTokenV1::from_raw_v1(value.token)
        .map_err(|_| Status::InvalidContract as u32)
}

/// Decode both operands before applying an operator. Invalid transport is never
/// a language TypeError, and a valid non-Text payload need not Fault for equality.
unsafe fn comparable(value: *const Operand) -> Result<Comparable, u32> {
    let descriptor = unsafe { operand(value) }.map_err(|status| status as u32)?;
    if descriptor.kind != DYNAMIC_CARRIER {
        if !matches!(descriptor.kind, STATIC_TEXT | OWNED_TEXT) { return Err(Status::InvalidContract as u32); }
        return Ok(Comparable::Text(*descriptor));
    }
    let carrier = dynamic_carrier(descriptor)?;
    match carrier.is_null_v1() {
        Ok(true) => return Ok(Comparable::Null),
        Ok(false) => {}
        Err(_) => return Err(Status::InvalidContract as u32),
    }
    match carrier.text_snapshot_v1() {
        Ok(text) => Ok(Comparable::Snapshot(text)),
        Err(nyash_rust::runtime::dynamic_invocation::DynamicInvocationFailureV1::ArgumentTypeMismatch) => {
            Ok(Comparable::Other)
        }
        Err(_) => Err(Status::InvalidContract as u32),
    }
}

/// Validate one or two Text descriptors, then borrow all leased texts at once.
/// Arrays avoid allocation; no registry/lease callback is nested or reentered.
unsafe fn with_text_operands<const N: usize, R>(
    values: [Option<*const Operand>; N],
    callback: impl FnOnce([Option<&str>; N]) -> R,
) -> Result<R, Status> {
    let mut literals = [None; N];
    let mut pairs = [None; N];
    for (i, value) in values.into_iter().enumerate() {
        let Some(value) = value else { continue; };
        let value = unsafe { operand(value) }?;
        match value.kind {
            STATIC_TEXT if value.handle == 0 && value.token == 0 && value.integer == 0 => {
                if value.len > isize::MAX as usize || (value.len != 0 && value.bytes.is_null()) {
                    return Err(Status::InvalidContract);
                }
                let bytes = if value.len == 0 { &[] } else {
                    unsafe { std::slice::from_raw_parts(value.bytes, value.len) }
                };
                literals[i] = Some(std::str::from_utf8(bytes).map_err(|_| Status::InvalidContract)?);
            }
            OWNED_TEXT if value.bytes.is_null() && value.len == 0 && value.integer == 0 => {
                pairs[i] = Some((value.handle, NonZeroU64::new(value.token).ok_or(Status::InvalidContract)?));
            }
            _ => return Err(Status::InvalidContract),
        }
    }
    if pairs.iter().all(Option::is_none) {
        return Ok(callback(literals));
    }
    lease::with_end_authorized_texts(pairs, |texts| {
        callback(std::array::from_fn(|i| texts[i].or(literals[i])))
    }).map_err(|_| Status::InvalidContract)
}

/// Count a live Text borrow without allocation or a language Fault frame.
/// Invalid descriptors leave the caller's output unchanged.
#[export_name = "nyash.string.code_point_count_v1"]
pub unsafe extern "C" fn code_point_count(value: *const Operand, out: *mut i64) -> u32 {
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let count = unsafe { with_text_operands([Some(value)], |[text]| text.map(|text| text.chars().count())) };
    let Some(count) = count.ok().flatten().and_then(|count| i64::try_from(count).ok()) else {
        return Status::InvalidContract as u32;
    };
    unsafe { out.write(count) }; // Both identity guards have been released.
    Status::Normal as u32
}

/// Search two exact live Text borrows without copies, acquisition or a Fault.
#[export_name = "nyash.string.code_point_index_of_v1"]
pub unsafe extern "C" fn code_point_index_of(
    value: *const Operand, needle: *const Operand, out: *mut i64,
) -> u32 {
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let index = unsafe { with_text_operands([Some(value), Some(needle)], |[value, needle]|
        value.zip(needle).map(|(value, needle)| string_ops::index_of(value, needle, None, StringIndexMode::CodePoint))) };
    let Ok(Some(index)) = index else { return Status::InvalidContract as u32; };
    unsafe { out.write(index) }; // No identity guard is held here.
    Status::Normal as u32
}

/// Shared exact scalar-sequence law. Every caller validates the wire predicate.
fn compare_text_value(op: u32, left: &str, right: &str) -> i64 {
    let order = left.chars().cmp(right.chars());
    i64::from(match op {
        0 => order.is_eq(), 1 => !order.is_eq(), 2 => order.is_lt(),
        3 => !order.is_gt(), 4 => order.is_gt(), 5 => !order.is_lt(),
        _ => unreachable!("validated comparison opcode"),
    })
}

/// Exact Text comparison is a read; release all identity guards before output.
#[export_name = "nyash.string.compare_text_v1"]
pub unsafe extern "C" fn compare_text(op: u32, left: *const Operand,
    right: *const Operand, out: *mut i64) -> u32 {
    if out.is_null() || !out.is_aligned() || op > 5 { return Status::InvalidContract as u32; }
    let result = unsafe { with_text_operands([Some(left), Some(right)], |[left, right]|
        left.zip(right).map(|(left, right)| compare_text_value(op, left, right))) };
    let Ok(Some(result)) = result else { return Status::InvalidContract as u32; };
    unsafe { out.write(result) };
    Status::Normal as u32
}

/// Copy a borrowed Text descriptor into one independent End-authorized pair.
/// Operand validation and publication use the same owners as the other reads.
#[export_name = "nyash.string.checked_materialize_v1"]
pub unsafe extern "C" fn materialize(storage: *mut c_void, site: u64, value: *const Operand,
    out: *mut OwnedText) -> u32 {
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let frame = match unsafe { frame(storage) } { Ok(v) => v, Err(s) => return s as u32 };
    let descriptor = match unsafe { operand(value) } { Ok(v) => v, Err(s) => return s as u32 };
    let value = if descriptor.kind == DYNAMIC_CARRIER {
        let carrier = match dynamic_carrier(descriptor) { Ok(value) => value, Err(status) => return status };
        match carrier.text_snapshot_v1() {
            Ok(value) => value,
            Err(nyash_rust::runtime::dynamic_invocation::DynamicInvocationFailureV1::ArgumentTypeMismatch) => {
                return fault(frame, site, TYPE_ERROR);
            }
            Err(_) => return Status::InvalidContract as u32,
        }
    } else {
        match unsafe { text_operand(value) } { Ok(v) => v, Err(s) => return s as u32 }
    };
    unsafe { publish(frame, site, value, out) }
}

#[export_name = "nyash.string.checked_substring_v1"]
pub unsafe extern "C" fn substring(storage: *mut c_void, site: u64, value: *const Operand,
    start: i64, end: i64, out: *mut OwnedText) -> u32 {
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let frame = match unsafe { frame(storage) } { Ok(v) => v, Err(s) => return s as u32 };
    let value = match unsafe { text_operand(value) } { Ok(v) => v, Err(s) => return s as u32 };
    let result = string_ops::substring(&value, start, Some(end), StringIndexMode::CodePoint);
    unsafe { publish(frame, site, result, out) }
}

/// Checked form for source expressions whose indices may retain a Dynamic
/// carrier. Both indices are decoded before publishing; wrong tags Fault with
/// TypeError and leave the caller's result storage unchanged.
#[export_name = "nyash.string.checked_substring_operands_v1"]
pub unsafe extern "C" fn substring_operands(storage: *mut c_void, site: u64,
    value: *const Operand, start: *const Operand, end: *const Operand, out: *mut OwnedText) -> u32 {
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let frame = match unsafe { frame(storage) } { Ok(v) => v, Err(s) => return s as u32 };
    let value = match unsafe { text_operand(value) } { Ok(v) => v, Err(s) => return s as u32 };
    let start = match unsafe { integer_operand(start) } {
        Ok(v) => v,
        Err(IntegerOperandFailure::TypeError) => return fault(frame, site, TYPE_ERROR),
        Err(IntegerOperandFailure::InvalidContract) => return Status::InvalidContract as u32,
    };
    let end = match unsafe { integer_operand(end) } {
        Ok(v) => v,
        Err(IntegerOperandFailure::TypeError) => return fault(frame, site, TYPE_ERROR),
        Err(IntegerOperandFailure::InvalidContract) => return Status::InvalidContract as u32,
    };
    let result = string_ops::substring(&value, start, Some(end), StringIndexMode::CodePoint);
    unsafe { publish(frame, site, result, out) }
}

/// Caller guarantees live nonoverlapping frame/operands/out storage. The
/// tagged Dynamic operand preserves its original source tag and payload.
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
        || !matches!(right.kind, STATIC_TEXT | OWNED_TEXT | NORMAL_INTEGER | TAGGED_DYNAMIC | DYNAMIC_CARRIER)
    {
        return Status::InvalidContract as u32;
    }
    let mut result = match unsafe { text(left) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    let suffix = match right.kind {
        TAGGED_DYNAMIC => match unsafe { dynamic_text(right) } {
            Ok(value) => value,
            Err(DynamicDisplayFailure::TypeError) => return fault(frame, site, TYPE_ERROR),
            Err(DynamicDisplayFailure::InvalidContract) => return Status::InvalidContract as u32,
        },
        DYNAMIC_CARRIER => {
            let carrier = match dynamic_carrier(right) { Ok(value) => value, Err(status) => return status };
            match carrier.string_add_suffix_v1() {
                Ok(value) => value,
                Err(nyash_rust::runtime::dynamic_invocation::DynamicInvocationFailureV1::ArgumentTypeMismatch) => {
                    return fault(frame, site, TYPE_ERROR);
                }
                Err(_) => return Status::InvalidContract as u32,
            }
        }
        _ => match unsafe { text(right) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        },
    };
    if result.try_reserve(suffix.len()).is_err() {
        return fault(frame, site, ALLOCATION_FAILED);
    }
    result.push_str(&suffix);
    unsafe { publish(frame, site, result, out) }
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
