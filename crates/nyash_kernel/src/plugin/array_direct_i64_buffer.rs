//! Stable direct i64 Array storage for future NativeDirect lowering.
//!
//! This module is storage-only. It does not change `ArrayBox`, does not export
//! helper ABI symbols, and does not open LLVM lowering. The goal is to prove a
//! compiler-consumable `repr(C)` header plus trailing i64 data layout.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::cell::RefCell;
use std::mem;
use std::ptr::{self, NonNull};
#[cfg(test)]
use std::sync::Arc;

#[cfg(test)]
use nyash_rust::box_trait::NyashBox;
#[cfg(test)]
use nyash_rust::boxes::array::ArrayBox;
#[cfg(test)]
use nyash_rust::runtime::host_handles;

pub(crate) const DIRECT_ARRAY_I64_KIND_V0: u32 = 1;
pub(crate) const DIRECT_ARRAY_ELEMENT_TAG_I64: u32 = 1;
const DIRECT_ARRAY_I64_HANDLE_TAG: usize = 3;
const DEFAULT_DIRECT_ARRAY_I64_CAPACITY: usize = 64;

thread_local! {
    static DIRECT_ARRAY_I64_OBJECTS: RefCell<Vec<DirectArrayI64BufferV0Box>> = const { RefCell::new(Vec::new()) };
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectArrayI64BufferV0 {
    kind: u32,
    flags: u32,
    generation: u32,
    element_tag: u32,
    len: u64,
    capacity: u64,
}

#[derive(Debug)]
pub(crate) struct DirectArrayI64BufferV0Box {
    ptr: NonNull<DirectArrayI64BufferV0>,
    layout: Layout,
}

impl DirectArrayI64BufferV0Box {
    pub(crate) fn new(generation: u32, capacity: usize) -> Option<Self> {
        if generation == 0 {
            return None;
        }
        let capacity_u64 = u64::try_from(capacity).ok()?;
        let layout = direct_array_i64_buffer_layout(capacity)?;
        let raw = unsafe { alloc(layout) };
        let Some(ptr) = NonNull::new(raw.cast::<DirectArrayI64BufferV0>()) else {
            handle_alloc_error(layout);
        };

        unsafe {
            ptr::write(
                ptr.as_ptr(),
                DirectArrayI64BufferV0 {
                    kind: DIRECT_ARRAY_I64_KIND_V0,
                    flags: 0,
                    generation,
                    element_tag: DIRECT_ARRAY_ELEMENT_TAG_I64,
                    len: 0,
                    capacity: capacity_u64,
                },
            );
        }

        Some(Self { ptr, layout })
    }

    #[cfg(test)]
    pub(crate) fn as_ptr(&self) -> *const DirectArrayI64BufferV0 {
        self.ptr.as_ptr()
    }

    pub(crate) fn handle(&self) -> Option<i64> {
        encode_direct_array_i64_handle(self.ptr)
    }

    pub(crate) fn from_handle(handle: i64) -> Option<NonNull<DirectArrayI64BufferV0>> {
        decode_direct_array_i64_handle(handle)
    }

    pub(crate) fn matches_handle(&self, handle: i64) -> bool {
        Self::from_handle(handle)
            .map(|ptr| ptr.as_ptr() == self.ptr.as_ptr())
            .unwrap_or(false)
    }

    pub(crate) fn data_ptr(&self) -> *const i64 {
        direct_array_i64_buffer_data_ptr(self.ptr.as_ptr())
    }

    fn data_mut_ptr(&mut self) -> *mut i64 {
        direct_array_i64_buffer_data_mut_ptr(self.ptr.as_ptr())
    }

    pub(crate) fn len(&self) -> usize {
        unsafe { (*self.ptr.as_ptr()).len as usize }
    }

    pub(crate) fn capacity(&self) -> usize {
        unsafe { (*self.ptr.as_ptr()).capacity as usize }
    }

    pub(crate) fn load(&self, index: usize) -> Option<i64> {
        if index >= self.len() {
            return None;
        }
        Some(unsafe { *self.data_ptr().add(index) })
    }

    pub(crate) fn store(&mut self, index: usize, value: i64) -> bool {
        let len = self.len();
        let capacity = self.capacity();
        if index > len || index >= capacity {
            return false;
        }
        unsafe {
            *self.data_mut_ptr().add(index) = value;
            if index == len {
                (*self.ptr.as_ptr()).len = (len + 1) as u64;
            }
        }
        true
    }

    #[cfg(test)]
    pub(crate) fn materialize_public_arraybox_snapshot(&self) -> Option<ArrayBox> {
        if !self.header_is_supported() || self.len() > self.capacity() {
            return None;
        }
        let snapshot = ArrayBox::new();
        for index in 0..self.len() {
            if !snapshot.slot_store_i64_raw(index as i64, self.load(index)?) {
                return None;
            }
        }
        Some(snapshot)
    }

    #[cfg(test)]
    pub(crate) fn materialize_public_arraybox_snapshot_handle(&self) -> Option<i64> {
        let snapshot: Arc<dyn NyashBox> = Arc::new(self.materialize_public_arraybox_snapshot()?);
        Some(host_handles::to_handle_arc(snapshot) as i64)
    }

    fn header_is_supported(&self) -> bool {
        let header = unsafe { &*self.ptr.as_ptr() };
        header.kind == DIRECT_ARRAY_I64_KIND_V0
            && header.generation != 0
            && header.element_tag == DIRECT_ARRAY_ELEMENT_TAG_I64
    }
}

impl Drop for DirectArrayI64BufferV0Box {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr().cast::<u8>(), self.layout);
        }
    }
}

pub(crate) fn direct_array_i64_buffer_header_size() -> usize {
    mem::size_of::<DirectArrayI64BufferV0>()
}

pub(crate) fn direct_array_i64_buffer_data_offset() -> usize {
    direct_array_i64_buffer_header_size()
}

fn direct_array_i64_buffer_layout(capacity: usize) -> Option<Layout> {
    let data_bytes = capacity.checked_mul(mem::size_of::<i64>())?;
    let size = direct_array_i64_buffer_header_size().checked_add(data_bytes)?;
    Layout::from_size_align(size, mem::align_of::<DirectArrayI64BufferV0>()).ok()
}

fn direct_array_i64_buffer_data_ptr(ptr: *const DirectArrayI64BufferV0) -> *const i64 {
    unsafe {
        ptr.cast::<u8>()
            .add(direct_array_i64_buffer_data_offset())
            .cast::<i64>()
    }
}

fn direct_array_i64_buffer_data_mut_ptr(ptr: *mut DirectArrayI64BufferV0) -> *mut i64 {
    unsafe {
        ptr.cast::<u8>()
            .add(direct_array_i64_buffer_data_offset())
            .cast::<i64>()
    }
}

fn encode_direct_array_i64_handle(ptr: NonNull<DirectArrayI64BufferV0>) -> Option<i64> {
    let raw = ptr.as_ptr() as usize;
    if raw & DIRECT_ARRAY_I64_HANDLE_TAG != 0 {
        return None;
    }
    i64::try_from(raw | DIRECT_ARRAY_I64_HANDLE_TAG).ok()
}

fn decode_direct_array_i64_handle(handle: i64) -> Option<NonNull<DirectArrayI64BufferV0>> {
    if handle <= 0 {
        return None;
    }
    let raw = usize::try_from(handle).ok()?;
    if raw & DIRECT_ARRAY_I64_HANDLE_TAG != DIRECT_ARRAY_I64_HANDLE_TAG {
        return None;
    }
    NonNull::new((raw & !DIRECT_ARRAY_I64_HANDLE_TAG) as *mut DirectArrayI64BufferV0)
}

fn with_registered_direct_array_i64<R>(
    handle: i64,
    f: impl FnOnce(&mut DirectArrayI64BufferV0Box) -> R,
) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    DIRECT_ARRAY_I64_OBJECTS.with(|objects| {
        let mut objects = objects.borrow_mut();
        let object = objects
            .iter_mut()
            .find(|object| object.matches_handle(handle))?;
        if !object.header_is_supported() || object.len() > object.capacity() {
            return None;
        }
        Some(f(object))
    })
}

pub(crate) fn direct_array_i64_store_i64(handle: i64, index: i64, value: i64) -> bool {
    if index < 0 {
        return false;
    }
    with_registered_direct_array_i64(handle, |object| object.store(index as usize, value))
        .unwrap_or(false)
}

pub(crate) fn direct_array_i64_load_i64(handle: i64, index: i64) -> Option<i64> {
    if index < 0 {
        return None;
    }
    with_registered_direct_array_i64(handle, |object| object.load(index as usize)).flatten()
}

pub(crate) fn direct_array_i64_push_i64(handle: i64, value: i64) -> Option<i64> {
    with_registered_direct_array_i64(handle, |object| {
        let len = object.len();
        if !object.store(len, value) {
            return None;
        }
        i64::try_from(len + 1).ok()
    })
    .flatten()
}

pub(crate) fn direct_array_i64_birth_handle_with_capacity(capacity: usize) -> Option<i64> {
    DIRECT_ARRAY_I64_OBJECTS.with(|objects| {
        let mut objects = objects.borrow_mut();
        let object = DirectArrayI64BufferV0Box::new(1, capacity)?;
        let handle = object.handle()?;
        objects.push(object);
        Some(handle)
    })
}

#[export_name = "nyash.array.direct_i64.birth_h"]
pub extern "C" fn nyash_array_direct_i64_birth_h_export() -> i64 {
    direct_array_i64_birth_handle_with_capacity(DEFAULT_DIRECT_ARRAY_I64_CAPACITY).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_array_i64_buffer_v0_header_and_data_offsets_are_stable() {
        assert_eq!(mem::size_of::<DirectArrayI64BufferV0>(), 32);
        assert_eq!(mem::align_of::<DirectArrayI64BufferV0>(), 8);
        assert_eq!(mem::align_of::<i64>(), 8);
        assert_eq!(direct_array_i64_buffer_header_size(), 32);
        assert_eq!(direct_array_i64_buffer_data_offset(), 32);
        let layout = direct_array_i64_buffer_layout(4).expect("layout");
        assert_eq!(layout.size(), 32 + 4 * 8);
        assert_eq!(layout.align(), 8);
    }

    #[test]
    fn direct_array_i64_buffer_v0_stores_and_loads_contiguous_i64_data() {
        let mut buffer = DirectArrayI64BufferV0Box::new(1, 3).expect("buffer");
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 3);
        assert!(buffer.store(0, 10));
        assert!(buffer.store(1, 20));
        assert!(buffer.store(2, 30));
        assert!(!buffer.store(3, 40));
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.load(0), Some(10));
        assert_eq!(buffer.load(1), Some(20));
        assert_eq!(buffer.load(2), Some(30));
        assert_eq!(buffer.load(3), None);
    }

    #[test]
    fn direct_array_i64_push_appends_raw_i64_values() {
        let handle = direct_array_i64_birth_handle_with_capacity(2).expect("handle");
        assert_eq!(direct_array_i64_push_i64(handle, 7), Some(1));
        assert_eq!(direct_array_i64_push_i64(handle, 8), Some(2));
        assert_eq!(direct_array_i64_push_i64(handle, 9), None);
        assert_eq!(direct_array_i64_load_i64(handle, 0), Some(7));
        assert_eq!(direct_array_i64_load_i64(handle, 1), Some(8));
        assert_eq!(direct_array_i64_load_i64(handle, 2), None);
    }

    #[test]
    fn direct_array_i64_runtime_ops_reject_public_arraybox_handles() {
        let array: Arc<dyn NyashBox> = Arc::new(ArrayBox::new());
        let public_handle = host_handles::to_handle_arc(array) as i64;

        assert_eq!(direct_array_i64_push_i64(public_handle, 7), None);
        assert!(!direct_array_i64_store_i64(public_handle, 0, 7));
        assert_eq!(direct_array_i64_load_i64(public_handle, 0), None);
    }

    #[test]
    fn direct_array_i64_buffer_v0_preserves_append_and_oob_policy() {
        let mut buffer = DirectArrayI64BufferV0Box::new(1, 2).expect("buffer");
        assert!(!buffer.store(1, 20));
        assert_eq!(buffer.len(), 0);
        assert!(buffer.store(0, 10));
        assert!(buffer.store(1, 20));
        assert!(!buffer.store(2, 30));
        assert_eq!(buffer.load(0), Some(10));
        assert_eq!(buffer.load(1), Some(20));
        assert_eq!(buffer.load(2), None);
    }

    #[test]
    fn direct_array_i64_buffer_v0_rejects_zero_generation() {
        assert!(DirectArrayI64BufferV0Box::new(0, 1).is_none());
    }

    #[test]
    fn direct_array_i64_buffer_v0_data_pointer_uses_header_offset() {
        let buffer = DirectArrayI64BufferV0Box::new(1, 1).expect("buffer");
        let base = buffer.as_ptr().cast::<u8>() as usize;
        let data = buffer.data_ptr().cast::<u8>() as usize;
        assert_eq!(data - base, 32);
    }

    #[test]
    fn direct_array_i64_buffer_v0_materializes_public_arraybox_snapshot() {
        let mut buffer = DirectArrayI64BufferV0Box::new(1, 3).expect("buffer");
        assert!(buffer.store(0, 11));
        assert!(buffer.store(1, 22));

        let snapshot = buffer
            .materialize_public_arraybox_snapshot()
            .expect("snapshot");
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot.slot_load_i64_raw(0), Some(11));
        assert_eq!(snapshot.slot_load_i64_raw(1), Some(22));
        assert_eq!(snapshot.slot_load_i64_raw(2), None);
    }

    #[test]
    fn direct_array_i64_buffer_v0_materializes_public_arraybox_host_handle() {
        let mut buffer = DirectArrayI64BufferV0Box::new(1, 2).expect("buffer");
        assert!(buffer.store(0, 7));
        assert!(buffer.store(1, 8));

        let handle = buffer
            .materialize_public_arraybox_snapshot_handle()
            .expect("snapshot handle");
        assert!(handle > 0);
        let source = host_handles::get(handle as u64).expect("host handle source");
        let snapshot = source
            .as_ref()
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("ArrayBox snapshot");
        assert_eq!(snapshot.len(), 2);
        assert_eq!(snapshot.slot_load_i64_raw(0), Some(7));
        assert_eq!(snapshot.slot_load_i64_raw(1), Some(8));
    }

    #[test]
    fn direct_array_i64_buffer_v0_handle_roundtrips_as_tagged_pointer() {
        let buffer = DirectArrayI64BufferV0Box::new(1, 1).expect("buffer");
        let handle = buffer.handle().expect("direct handle");
        assert_eq!(
            (handle as usize) & DIRECT_ARRAY_I64_HANDLE_TAG,
            DIRECT_ARRAY_I64_HANDLE_TAG
        );
        assert!(buffer.matches_handle(handle));
        assert!(DirectArrayI64BufferV0Box::from_handle(handle).is_some());
        assert!(DirectArrayI64BufferV0Box::from_handle(0).is_none());
        assert!(DirectArrayI64BufferV0Box::from_handle(2).is_none());
    }

    #[test]
    fn direct_array_i64_birth_handle_returns_direct_handle_without_public_arraybox_alias() {
        let handle = direct_array_i64_birth_handle_with_capacity(2).expect("birth handle");
        assert!(handle > 0);
        assert!(DirectArrayI64BufferV0Box::from_handle(handle).is_some());
        assert!(host_handles::get(handle as u64).is_none());

        DIRECT_ARRAY_I64_OBJECTS.with(|objects| {
            let objects = objects.borrow();
            assert!(objects.iter().any(|object| object.matches_handle(handle)));
        });
    }
}
