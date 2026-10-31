//! Thread-confined Shared cells. The immutable descriptor is issued by the
//! published module; a handle never supplies a payload type or End function.
use super::{Diagnostic, FaultFrame, Status};
use crate::exports::typed_object_store_backend::{
    check_indexed_profile, validate_checked_indexed_identity, TypedObjectStoreBackend,
};
use std::{cell::RefCell, collections::HashMap, ffi::c_void, sync::atomic::{AtomicU64, Ordering}};

type End = unsafe extern "C" fn(*mut c_void, i64, u64) -> u32;

#[repr(C)]
pub(super) struct Descriptor {
    pub abi: u32,
    pub profile: u32,
    pub type_id: i64,
    pub end: Option<End>,
}

struct Cell {
    descriptor: usize,
    payload: i64,
    count: usize,
}

#[derive(Default)]
struct Slab {
    cells: HashMap<i64, Cell>,
    handles: HashMap<i64, i64>,
}

thread_local! { static SLAB: RefCell<Slab> = RefCell::new(Slab::default()); }

// IDs are never reused, including across threads. RC remains non-atomic and
// thread local; this counter supplies only process-unique handle names.
static NEXT_HANDLE: AtomicU64 = AtomicU64::new(1);

fn fresh_handle() -> Option<i64> {
    NEXT_HANDLE.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id|
        (id < i64::MAX as u64).then_some(id + 1)).ok().map(|id| id as i64)
}

unsafe fn admit(storage: *mut c_void) -> Result<(), Status> {
    if storage.is_null() || !storage.cast::<FaultFrame>().is_aligned() {
        return Err(Status::InvalidContract);
    }
    if !unsafe { &*storage.cast::<FaultFrame>() }.valid() {
        return Err(Status::InvalidContract);
    }
    Ok(())
}

unsafe fn descriptor(raw: *const Descriptor) -> Result<(TypedObjectStoreBackend, i64, End), Status> {
    if raw.is_null() || !raw.is_aligned() { return Err(Status::InvalidContract); }
    // The generated module owns this aligned immutable constant for its lifetime.
    let desc = unsafe { &*raw };
    if desc.abi != 1 || desc.type_id <= 0 { return Err(Status::InvalidContract); }
    let profile = match desc.profile {
        1 => TypedObjectStoreBackend::SafeMutex,
        2 => TypedObjectStoreBackend::SingleThreadExact,
        _ => return Err(Status::InvalidContract),
    };
    check_indexed_profile(profile).map_err(|_| Status::InvalidContract)?;
    Ok((profile, desc.type_id, desc.end.ok_or(Status::InvalidContract)?))
}

unsafe fn allocation_fault(storage: *mut c_void, site: u64) -> u32 {
    let frame = unsafe { &mut *storage.cast::<FaultFrame>() };
    frame.record(Diagnostic::new(205, site, [0, 0])).unwrap_or(Status::InvalidContract) as u32
}

#[export_name = "nyash.shared.checked_new_v1"]
pub unsafe extern "C" fn new(storage: *mut c_void, site: u64, raw: *const Descriptor, payload: i64, out: *mut i64) -> u32 {
    if let Err(status) = unsafe { admit(storage) } { return status as u32; }
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    let (profile, ty, _) = match unsafe { descriptor(raw) } { Ok(x) => x, Err(s) => return s as u32 };
    if validate_checked_indexed_identity(profile, payload, ty).is_err() { return Status::InvalidContract as u32; }
    let result = SLAB.with(|slot| {
        let mut slab = slot.borrow_mut();
        slab.cells.try_reserve(1).ok()?;
        slab.handles.try_reserve(1).ok()?;
        let id = fresh_handle()?;
        slab.cells.insert(id, Cell { descriptor: raw as usize, payload, count: 1 });
        slab.handles.insert(id, id);
        Some(id)
    });
    match result {
        Some(id) => { unsafe { out.write(id) }; Status::Normal as u32 }
        None => unsafe { allocation_fault(storage, site) },
    }
}

#[export_name = "nyash.shared.checked_clone_v1"]
pub unsafe extern "C" fn clone_handle(storage: *mut c_void, site: u64, raw: *const Descriptor, handle: i64, out: *mut i64) -> u32 {
    if let Err(status) = unsafe { admit(storage) } { return status as u32; }
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    if let Err(status) = unsafe { descriptor(raw) } { return status as u32; }
    let result = SLAB.with(|slot| {
        let mut slab = slot.borrow_mut();
        let cell_id = *slab.handles.get(&handle).ok_or(Status::InvalidContract)?;
        let cell = slab.cells.get(&cell_id).ok_or(Status::InvalidContract)?;
        if cell.descriptor != raw as usize || cell.count == 0 { return Err(Status::InvalidContract); }
        let next_count = cell.count.checked_add(1).ok_or(Status::Fault)?;
        slab.handles.try_reserve(1).map_err(|_| Status::Fault)?;
        let new_handle = fresh_handle().ok_or(Status::Fault)?;
        slab.cells.get_mut(&cell_id).ok_or(Status::InvalidContract)?.count = next_count;
        slab.handles.insert(new_handle, cell_id);
        Ok(new_handle)
    });
    match result {
        Ok(id) => { unsafe { out.write(id) }; Status::Normal as u32 }
        Err(Status::Fault) => unsafe { allocation_fault(storage, site) },
        Err(status) => status as u32,
    }
}

#[export_name = "nyash.shared.checked_borrow_v1"]
pub unsafe extern "C" fn borrow(storage: *mut c_void, _site: u64, raw: *const Descriptor, handle: i64, out: *mut i64) -> u32 {
    if let Err(status) = unsafe { admit(storage) } { return status as u32; }
    if out.is_null() || !out.is_aligned() { return Status::InvalidContract as u32; }
    if let Err(status) = unsafe { descriptor(raw) } { return status as u32; }
    let payload = SLAB.with(|slot| {
        let slab = slot.borrow();
        let cell = slab.cells.get(slab.handles.get(&handle)?)?;
        (cell.descriptor == raw as usize && cell.count != 0).then_some(cell.payload)
    });
    match payload {
        Some(value) => { unsafe { out.write(value) }; Status::Normal as u32 }
        None => Status::InvalidContract as u32,
    }
}

#[export_name = "nyash.shared.checked_release_v1"]
pub unsafe extern "C" fn release(storage: *mut c_void, site: u64, raw: *const Descriptor, handle: i64) -> u32 {
    if let Err(status) = unsafe { admit(storage) } { return status as u32; }
    let (_, _, end) = match unsafe { descriptor(raw) } { Ok(x) => x, Err(s) => return s as u32 };
    let result = SLAB.with(|slot| {
        let mut slab = slot.borrow_mut();
        let cell_id = *slab.handles.get(&handle).ok_or(Status::InvalidContract)?;
        let cell = slab.cells.get(&cell_id).ok_or(Status::InvalidContract)?;
        if cell.descriptor != raw as usize || cell.count == 0 { return Err(Status::InvalidContract); }
        let last = cell.count == 1;
        slab.handles.remove(&handle);
        if last {
            Ok(Some(slab.cells.remove(&cell_id).ok_or(Status::InvalidContract)?.payload))
        } else {
            slab.cells.get_mut(&cell_id).ok_or(Status::InvalidContract)?.count -= 1;
            Ok(None)
        }
    });
    match result {
        Err(status) => status as u32,
        Ok(None) => Status::Normal as u32,
        // The slab and FaultFrame borrows ended before the potentially reentrant End.
        Ok(Some(payload)) => match unsafe { end(storage, payload, site) } {
            0 => Status::Normal as u32,
            1 if unsafe { (&*storage.cast::<FaultFrame>()).valid() }
                && unsafe { (&*storage.cast::<FaultFrame>()).diagnostics().is_ok_and(|(first, _, _)| first.is_some()) } => Status::Fault as u32,
            _ => Status::InvalidContract as u32,
        },
    }
}
