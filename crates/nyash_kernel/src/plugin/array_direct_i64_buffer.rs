//! Stable direct i64 Array storage substrate for future NativeDirect lowering.
//!
//! This module is storage-only. It does not change `ArrayBox`, does not export
//! helper ABI symbols, and does not open LLVM lowering. The goal is to prove a
//! compiler-consumable `repr(C)` header plus trailing i64 data layout.

#![allow(dead_code)]

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::mem;
use std::ptr::{self, NonNull};

pub(crate) const DIRECT_ARRAY_I64_KIND_V0: u32 = 1;
pub(crate) const DIRECT_ARRAY_ELEMENT_TAG_I64: u32 = 1;

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

    pub(crate) fn as_ptr(&self) -> *const DirectArrayI64BufferV0 {
        self.ptr.as_ptr()
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
}
