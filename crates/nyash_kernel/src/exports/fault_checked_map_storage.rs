//! Initialized opaque placement. Headers never validate arbitrary pointers.
use super::Status;
use nyash_rust::boxes::{
    map_box::checked::{CheckedMap, DetachedMapEntry},
    map_key_domain::MapKeyDomain,
};
use std::{
    ffi::c_void,
    mem::{size_of, MaybeUninit},
    sync::Mutex,
};
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
pub(super) const VIEW_TAG: u64 = 0x564945570001;
pub(super) const TEXT_VIEW_TAG: u64 = 0x544558540001;
pub(super) const VIEW_LIVE: u32 = 1;
pub(super) const VIEW_CONSUMED: u32 = 2;
#[repr(C)]
pub(in crate::exports::fault) struct MapView {
    pub(in crate::exports::fault) state: u32,
    pub(in crate::exports::fault) reserved: u32,
    pub(in crate::exports::fault) parent_map: *mut c_void,
    pub(in crate::exports::fault) child_map: *const CheckedMap,
}
#[repr(C)]
pub(in crate::exports::fault) struct TextView {
    pub(in crate::exports::fault) state: u32,
    pub(in crate::exports::fault) reserved: u32,
    pub(in crate::exports::fault) bytes: *const u8,
    pub(in crate::exports::fault) len: usize,
}
pub(in crate::exports::fault) type MapViewStorage = Placement<MapView>;
pub(in crate::exports::fault) type TextViewStorage = Placement<TextView>;
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

pub(in crate::exports::fault) unsafe fn init<T>(ptr: *mut c_void, magic: u64, value: T) -> u32 {
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
pub(in crate::exports::fault) unsafe fn admit<'a, T>(
    ptr: *mut c_void,
    magic: u64,
) -> Result<&'a T, Status> {
    if ptr.is_null() || (ptr as usize) % std::mem::align_of::<Placement<T>>() != 0 {
        return Err(Status::InvalidContract);
    }
    let place = ptr.cast::<Placement<T>>();
    if unsafe { std::ptr::addr_of!((*place).magic).read() } != magic {
        return Err(Status::InvalidContract);
    }
    Ok(unsafe { (&*place).value.assume_init_ref() })
}
pub(super) unsafe fn admit_mut<'a, T>(ptr: *mut c_void, magic: u64) -> Result<&'a mut T, Status> {
    if ptr.is_null() || (ptr as usize) % std::mem::align_of::<Placement<T>>() != 0 {
        return Err(Status::InvalidContract);
    }
    let place = ptr.cast::<Placement<T>>();
    if unsafe { std::ptr::addr_of!((*place).magic).read() } != magic {
        return Err(Status::InvalidContract);
    }
    Ok(unsafe { (&mut *place).value.assume_init_mut() })
}

/// A source-exclusive live Map loan and a fresh, fallibly allocated destination.
/// Dropping preparation only frees uninitialized bytes; ownership has not moved.
pub(in crate::exports) struct PreparedMapMoveV1<'a> {
    source: std::ptr::NonNull<MapStorage>,
    destination: Box<MaybeUninit<MapStorage>>,
    loan: std::marker::PhantomData<&'a mut MapStorage>,
}
impl<'a> PreparedMapMoveV1<'a> {
    pub(in crate::exports) fn source_ptr(&self) -> *const c_void {
        self.source.as_ptr().cast()
    }
    pub(in crate::exports) unsafe fn prepare(source: *mut c_void) -> Result<Self, Status> {
        let map = unsafe { admit::<CheckedMap>(source, MAP_TAG) }?;
        map.require_live().map_err(|_| Status::InvalidContract)?;
        let layout = std::alloc::Layout::new::<MaybeUninit<MapStorage>>();
        let allocation = unsafe { std::alloc::alloc(layout) }.cast::<MaybeUninit<MapStorage>>();
        if allocation.is_null() { return Err(Status::Fault); }
        Ok(Self {
            source: std::ptr::NonNull::new(source.cast::<MapStorage>()).expect("admitted source"),
            destination: unsafe { Box::from_raw(allocation) },
            loan: std::marker::PhantomData,
        })
    }

    /// Call only after destination slot validation under its retained guard.
    /// No allocation, validation, callback or lock acquisition remains here.
    pub(in crate::exports) fn commit(mut self) -> OwnedMapPlacementV1 {
        // No exclusive reference was formed before the same-slot refusal.
        let source = unsafe { self.source.as_mut() };
        let value = unsafe { source.value.assume_init_read() };
        source.value = MaybeUninit::new(CheckedMap::unissued());
        self.destination.write(Placement { magic: MAP_TAG, value: MaybeUninit::new(value) });
        OwnedMapPlacementV1(std::mem::ManuallyDrop::new(self.destination))
    }
}

/// Native ownership, never a numeric handle or another semantic registry.
/// InvalidContract must not free bytes still backing a live Map obligation.
#[derive(Debug)]
pub(crate) struct OwnedMapPlacementV1(
    std::mem::ManuallyDrop<Box<MaybeUninit<MapStorage>>>,
);
impl OwnedMapPlacementV1 {
    pub(in crate::exports) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr().cast_mut().cast()
    }
    pub(in crate::exports) unsafe fn end(mut self, frame: *mut c_void, site: u64) -> u32 {
        let ptr = self.as_ptr();
        let status = unsafe { super::map_end(frame, site, ptr) };
        if status > 1 || unsafe { map_dispose(ptr) } != Status::Normal as u32 {
            return Status::InvalidContract as u32;
        }
        // Disposal has consumed native bookkeeping; no semantic Drop occurs.
        unsafe { std::mem::ManuallyDrop::drop(&mut self.0) };
        status
    }
}
/// Only after live obligations and all borrows have ended. No semantic callback.
pub(in crate::exports::fault) unsafe fn dispose<T>(ptr: *mut c_void) -> u32 {
    let place = ptr.cast::<Placement<T>>();
    unsafe {
        std::ptr::addr_of_mut!((*place).magic).write(0);
        (*place).value.assume_init_drop();
    }
    Status::Normal as u32
}
pub(in crate::exports::fault) fn separate(regions: &[(usize, usize)]) -> bool {
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

// Fresh placement, explicit move and native disposal never run semantic End.
#[export_name = "nyash.map.storage_init_v1"]
pub unsafe extern "C" fn map_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, MAP_TAG, CheckedMap::unissued()) }
}
#[export_name = "nyash.map.storage_move_v1"]
pub unsafe extern "C" fn map_move(dst: *mut c_void, src: *mut c_void) -> u32 {
    if !separate(&[
        (dst as usize, size_of::<MapStorage>()),
        (src as usize, size_of::<MapStorage>()),
    ]) {
        return Status::InvalidContract as u32;
    }
    let map = match unsafe { admit::<CheckedMap>(src, MAP_TAG) } {
        Ok(v) => v,
        Err(s) => return s as u32,
    };
    if map.require_live().is_err() {
        return Status::InvalidContract as u32;
    }
    // dst null/alignment failures precede the move; init cannot fail below.
    if dst.is_null() || (dst as usize) % std::mem::align_of::<Placement<CheckedMap>>() != 0 {
        return Status::InvalidContract as u32;
    }
    let place = src.cast::<Placement<CheckedMap>>();
    let moved = unsafe { (*place).value.assume_init_read() };
    unsafe { (*place).value = MaybeUninit::new(CheckedMap::unissued()) };
    unsafe { init(dst, MAP_TAG, moved) }
}
#[export_name = "nyash.map.storage_dispose_v1"]
pub unsafe extern "C" fn map_dispose(ptr: *mut c_void) -> u32 {
    {
        let map = match unsafe { admit::<CheckedMap>(ptr, MAP_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        if map.require_disposable().is_err() {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<CheckedMap>(ptr) }
}

#[export_name = "nyash.map.key_init_v1"]
pub unsafe extern "C" fn key_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, KEY_TAG, Mutex::new(KeyState::Empty)) }
}
#[export_name = "nyash.map.outcome_init_v1"]
pub unsafe extern "C" fn outcome_init(ptr: *mut c_void) -> u32 {
    unsafe { init(ptr, OUT_TAG, Mutex::new(OutcomeState::Unissued)) }
}
#[export_name = "nyash.map.key_dispose_v1"]
pub unsafe extern "C" fn key_dispose(ptr: *mut c_void) -> u32 {
    {
        let key = match unsafe { admit::<Mutex<KeyState>>(ptr, KEY_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        if key.lock().is_err() {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<Mutex<KeyState>>(ptr) }
}
#[export_name = "nyash.map.outcome_dispose_v1"]
pub unsafe extern "C" fn outcome_dispose(ptr: *mut c_void) -> u32 {
    {
        let out = match unsafe { admit::<Mutex<OutcomeState>>(ptr, OUT_TAG) } {
            Ok(v) => v,
            Err(s) => return s as u32,
        };
        let out = match out.lock() {
            Ok(v) => v,
            Err(_) => return Status::InvalidContract as u32,
        };
        if matches!(*out, OutcomeState::Ready(_)) {
            return Status::InvalidContract as u32;
        }
    }
    unsafe { dispose::<Mutex<OutcomeState>>(ptr) }
}
