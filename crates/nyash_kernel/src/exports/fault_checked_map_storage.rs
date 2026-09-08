//! Initialized opaque placement. Headers never validate arbitrary pointers.
use super::Status;
use nyash_rust::boxes::{
    map_box::checked::{CheckedMap, DetachedMapEntry},
    map_key_domain::MapKeyDomain,
};
use std::{ffi::c_void, mem::MaybeUninit, sync::Mutex};
pub(super) const MAP_TAG: u64 = 0x4d415001;
pub(super) const KEY_TAG: u64 = 0x4b455901;
pub(super) const OUT_TAG: u64 = 0x4f555401;
#[repr(C)]
pub(in crate::exports::fault) struct Placement<T> {
    pub magic: u64,
    pub value: MaybeUninit<T>,
}
pub(in crate::exports::fault) type MapStorage = Placement<CheckedMap>;
pub(in crate::exports::fault) type KeyStorage = Placement<Mutex<KeyState>>;
pub(in crate::exports::fault) type OutcomeStorage = Placement<Mutex<OutcomeState>>;
pub(in crate::exports::fault) enum KeyState {
    Empty,
    Ready(MapKeyDomain),
    Consumed,
}
pub(in crate::exports::fault) enum OutcomeState {
    Unissued,
    Ready(DetachedMapEntry),
    Consumed,
}

pub(super) unsafe fn init<T>(ptr: *mut c_void, magic: u64, value: T) -> u32 {
    if ptr.is_null() || (ptr as usize) % std::mem::align_of::<Placement<T>>() != 0 {
        return Status::InvalidContract as u32;
    }
    // Fresh writable storage is a caller obligation, not a header inference.
    unsafe {
        ptr.cast::<Placement<T>>().write(Placement {
            magic,
            value: MaybeUninit::new(value),
        });
    }
    Status::Normal as u32
}
pub(super) unsafe fn admit<'a, T>(ptr: *mut c_void, magic: u64) -> Result<&'a T, Status> {
    if ptr.is_null() || (ptr as usize) % std::mem::align_of::<Placement<T>>() != 0 {
        return Err(Status::InvalidContract);
    }
    let place = ptr.cast::<Placement<T>>();
    if unsafe { std::ptr::addr_of!((*place).magic).read() } != magic {
        return Err(Status::InvalidContract);
    }
    Ok(unsafe { (&*place).value.assume_init_ref() })
}
/// Only after live obligations and all borrows have ended. No semantic callback.
pub(super) unsafe fn dispose<T>(ptr: *mut c_void) -> u32 {
    let place = ptr.cast::<Placement<T>>();
    unsafe {
        std::ptr::addr_of_mut!((*place).magic).write(0);
        (*place).value.assume_init_drop();
    }
    Status::Normal as u32
}
pub(super) fn separate(regions: &[(usize, usize)]) -> bool {
    for (i, &(start, size)) in regions.iter().enumerate() {
        let Some(end) = start.checked_add(size) else {
            return false;
        };
        if size != 0 && start == 0 {
            return false;
        }
        for &(other, count) in &regions[..i] {
            let Some(other_end) = other.checked_add(count) else {
                return false;
            };
            if size != 0 && count != 0 && start < other_end && other < end {
                return false;
            }
        }
    }
    true
}
