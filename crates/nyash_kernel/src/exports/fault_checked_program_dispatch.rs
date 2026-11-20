//! Program-scoped user-box dispatch. The published image owns descriptors and
//! thunk code for the entire process. Incoming typed borrows never use tag 3.

use super::{checked_dynamic as dynamic, Status};
use crate::exports::typed_object_store_backend::pin_dispatch_receiver;
use nyash_rust::runtime::dynamic_invocation::{
    select_unique_provider_v1, DynamicCarrierReservationV1, DynamicInvocationFailureV1 as Failure,
};
use std::{ffi::c_void, mem::size_of, num::NonZeroUsize};

const PROGRAM_VERSION: u32 = 1;
const TYPED_STORE_BORROW: u32 = 5;
const FORMAL_DYNAMIC: u32 = 0;
const FORMAL_I64: u32 = 1;

type Thunk =
    unsafe extern "C" fn(*mut c_void, i64, *const dynamic::Operand, usize, *mut i64) -> u32;

#[repr(C)]
pub(super) struct DispatchTypeV1 {
    type_id: i64,
    layout: *const u32,
    layout_count: usize,
}

#[repr(C)]
pub(super) struct DispatchProviderV1 {
    type_id: i64,
    selector: *const u8,
    selector_len: usize,
    arity: usize,
    parameters: *const u32,
    thunk: Option<Thunk>,
}

#[repr(C)]
pub(super) struct DispatchProgramV1 {
    version: u32,
    reserved: u32,
    types: *const DispatchTypeV1,
    type_count: usize,
    rows: *const DispatchProviderV1,
    row_count: usize,
}

/// Wire storage is trusted immutable image memory or synchronous stack
/// storage. These checks reject malformed sizes and alignment at the boundary.
unsafe fn wire_slice<'a, T>(ptr: *const T, len: usize) -> Option<&'a [T]> {
    if len == 0 {
        return Some(&[]);
    }
    if ptr.is_null() || !ptr.is_aligned() || len > (isize::MAX as usize) / size_of::<T>() {
        return None;
    }
    Some(unsafe { std::slice::from_raw_parts(ptr, len) })
}

unsafe fn read_program<'a>(program: *const DispatchProgramV1) -> Option<&'a DispatchProgramV1> {
    if program.is_null() || !program.is_aligned() {
        return None;
    }
    let program = unsafe { &*program };
    (program.version == PROGRAM_VERSION && program.reserved == 0).then_some(program)
}

pub(super) unsafe fn program_type_stamp(
    program: *const DispatchProgramV1,
    type_id: i64,
    layout: *const u32,
    layout_count: usize,
) -> Option<NonZeroUsize> {
    let stamp = NonZeroUsize::new(program.addr())?;
    let program = unsafe { read_program(program) }?;
    let types = unsafe { wire_slice(program.types, program.type_count) }?;
    let row = select_unique_provider_v1(types.iter().filter(|row| row.type_id == type_id)).ok()?;
    let layout = unsafe { wire_slice(layout, layout_count) }?;
    let expected = unsafe { wire_slice(row.layout, row.layout_count) }?;
    (type_id > 0 && layout == expected && layout.iter().all(|tag| *tag == 1)).then_some(stamp)
}

fn record(storage: *mut c_void, site: u64, failure: Failure) -> u32 {
    // The export validated the frame and no other kernel borrow crosses here.
    match unsafe { dynamic::frame(storage) } {
        Ok(frame) => dynamic::record_failure(frame, site, failure),
        Err(status) => status as u32,
    }
}

fn is_integer(op: &dynamic::Operand) -> bool {
    (op.kind == dynamic::INTEGER && op.tag == 0) || (op.kind == dynamic::TAGGED && op.tag == 1)
}

/// The site belongs to the original Invoke. A thunk may synchronously invoke
/// this entry again, so only the transient pin and vacant result survive it.
#[export_name = "nyash.dynamic.checked_invoke_program_v1"]
pub unsafe extern "C" fn invoke(
    storage: *mut c_void,
    site: u64,
    program: *const DispatchProgramV1,
    receiver: *const dynamic::Operand,
    selector: *const u8,
    selector_len: usize,
    arguments: *const dynamic::Operand,
    argument_count: usize,
    out: *mut u64,
) -> u32 {
    if out.is_null() || !out.is_aligned() || unsafe { dynamic::frame(storage) }.is_err() {
        return Status::InvalidContract as u32;
    }
    let Some(selector_bytes) = (unsafe { wire_slice(selector, selector_len) }) else {
        return Status::InvalidContract as u32;
    };
    if std::str::from_utf8(selector_bytes).is_err() {
        return Status::InvalidContract as u32;
    }
    let Some(args) = (unsafe { wire_slice(arguments, argument_count) }) else {
        return Status::InvalidContract as u32;
    };
    let Ok(receiver_row) = (unsafe { dynamic::operand(receiver) }) else {
        return Status::InvalidContract as u32;
    };
    let Some(descriptor) = (unsafe { read_program(program) }) else {
        return Status::InvalidContract as u32;
    };

    // The disjoint host/weak domain selects only its existing builtin rows.
    // Failed typed-store selection never falls through to the host registry.
    if receiver_row.kind != dynamic::TAGGED || receiver_row.tag != TYPED_STORE_BORROW {
        return unsafe {
            dynamic::invoke(
                storage,
                site,
                receiver,
                selector,
                selector_len,
                arguments,
                argument_count,
                out,
            )
        };
    }

    let stamp = NonZeroUsize::new(program.addr()).expect("validated program");
    let handle = receiver_row.payload as i64;
    let Some(pin) = pin_dispatch_receiver(handle, stamp) else {
        return record(storage, site, Failure::InvalidCarrier);
    };
    let Some(rows) = (unsafe { wire_slice(descriptor.rows, descriptor.row_count) }) else {
        return Status::InvalidContract as u32;
    };
    let mut malformed_selector = false;
    let selected = select_unique_provider_v1(rows.iter().filter(|row| {
        if row.type_id != pin.type_id() || row.arity != argument_count {
            return false;
        }
        match unsafe { wire_slice(row.selector, row.selector_len) } {
            Some(bytes) => bytes == selector_bytes,
            None => {
                malformed_selector = true;
                false
            }
        }
    }));
    if malformed_selector {
        return Status::InvalidContract as u32;
    }
    let row = match selected {
        Ok(row) => row,
        Err(failure) => return record(storage, site, failure),
    };
    let Some(parameters) = (unsafe { wire_slice(row.parameters, row.arity) }) else {
        return Status::InvalidContract as u32;
    };
    if parameters
        .iter()
        .any(|formal| !matches!(*formal, FORMAL_DYNAMIC | FORMAL_I64))
    {
        return Status::InvalidContract as u32;
    }
    if args.iter().any(|arg| !is_integer(arg)) {
        return record(storage, site, Failure::ArgumentTypeMismatch);
    }
    let Some(thunk) = row.thunk else {
        return Status::InvalidContract as u32;
    };
    let reservation = match DynamicCarrierReservationV1::reserve_v1() {
        Ok(reservation) => reservation,
        Err(failure) => return record(storage, site, failure),
    };
    let mut result = 0i64;
    let status = unsafe { thunk(storage, handle, arguments, argument_count, &mut result) };
    drop(pin);
    if status == Status::Normal as u32 {
        let token = reservation.publish_integer_v1(result);
        unsafe { out.write(token.raw_v1()) };
        Status::Normal as u32
    } else if status == Status::Fault as u32 {
        Status::Fault as u32
    } else {
        Status::InvalidContract as u32
    }
}

#[cfg(test)]
#[path = "fault_checked_program_dispatch_tests.rs"]
mod tests;
