//! Checked physical owner for Array slots containing opaque Dynamic carriers.
//! The carrier obligation moves only after Normal append; End detaches before
//! invoking token cleanup so neither Array nor host-registry locks are held.
use super::*;
use nyash_rust::boxes::array::{ArrayCarrierSlotDescriptor, ArrayOwnedSlotError};
use std::ffi::c_void;

const ARRAY_CARRIER_APPEND_FAULT: u32 = 205;

#[repr(C)]
pub(super) struct Descriptor {
    version: u32,
    reserved: u32,
}

unsafe fn descriptor(
    raw: *const Descriptor,
) -> Result<ArrayCarrierSlotDescriptor, Status> {
    if raw.is_null() || !raw.is_aligned() {
        return Err(Status::InvalidContract);
    }
    // The caller supplies an immutable descriptor retained through this call.
    let descriptor = unsafe { &*raw };
    if descriptor.version != 1 || descriptor.reserved != 0 {
        return Err(Status::InvalidContract);
    }
    Ok(ArrayCarrierSlotDescriptor::new(raw as usize))
}

/// Install the carrier-only profile on an empty builtin Array.
#[export_name = "nyash.array.checked_carrier_claim_v1"]
pub unsafe extern "C" fn claim(
    storage: *mut c_void,
    _site: u64,
    handle: i64,
    raw: *const Descriptor,
) -> u32 {
    if let Err(status) = unsafe { super::admit(storage) } {
        return status as u32;
    }
    let descriptor = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match super::with_array_box_direct(handle, |array| {
        array.claim_dynamic_carrier_slots(descriptor)
    }) {
        Some(Ok(())) => Status::Normal as u32,
        _ => Status::InvalidContract as u32,
    }
}

/// The candidate remains caller-owned on every returned failure. A Normal
/// result is the only point where the Array takes the single End obligation.
#[export_name = "nyash.array.checked_carrier_append_v1"]
pub unsafe extern "C" fn append(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    raw: *const Descriptor,
    candidate: u64,
) -> u32 {
    if let Err(status) = unsafe { super::admit(storage) } {
        return status as u32;
    }
    let descriptor = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    match super::with_array_box_direct(handle, |array| {
        array.append_dynamic_carrier_slot(descriptor, candidate)
    }) {
        Some(Ok(_)) => Status::Normal as u32,
        Some(Err(ArrayOwnedSlotError::ReservationFailed)) => {
            let frame = match unsafe { super::admit(storage) } {
                Ok(value) => value,
                Err(status) => return status as u32,
            };
            super::record(
                frame,
                Ok(Diagnostic::new(ARRAY_CARRIER_APPEND_FAULT, site, [0, 0])),
            )
        }
        _ => Status::InvalidContract as u32,
    }
}

/// Consume the carrier Array. Successful tokens end in reverse commit order;
/// each End runs outside Array/registry locks, and later cleanup is attempted
/// even when an earlier carrier reports Fault.
#[export_name = "nyash.array.checked_carrier_end_v1"]
pub unsafe extern "C" fn end(
    storage: *mut c_void,
    site: u64,
    handle: i64,
    raw: *const Descriptor,
) -> u32 {
    if let Err(status) = unsafe { super::admit(storage) } {
        return status as u32;
    }
    let descriptor = match unsafe { descriptor(raw) } {
        Ok(value) => value,
        Err(status) => return status as u32,
    };
    let (owner_identity, carriers) = match super::with_array_box_direct(handle, |array| {
        array.detach_dynamic_carriers_for_end(descriptor)
    }) {
        Some(Ok(detached)) => detached,
        _ => return Status::InvalidContract as u32,
    };

    let mut fault = false;
    let mut invalid = false;
    for carrier in carriers.into_iter().rev() {
        match unsafe {
            super::super::checked_dynamic::end_array_slot(storage, site, carrier, owner_identity)
        } {
            value if value == Status::Normal as u32 => {}
            value if value == Status::Fault as u32 => fault = true,
            _ => invalid = true,
        }
    }

    if !matches!(
        super::with_array_box_direct(handle, |array| {
            array.finish_dynamic_carriers_end(descriptor)
        }),
        Some(Ok(()))
    ) {
        return Status::InvalidContract as u32;
    }
    crate::nyrt_handle_release_h(handle);
    if invalid {
        Status::InvalidContract as u32
    } else if fault {
        Status::Fault as u32
    } else {
        Status::Normal as u32
    }
}
