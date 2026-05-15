// hako.mem native memory exports for pure-first EXE lowering.
//
// These exports own raw native allocation for the runtime/kernel link target.
// They intentionally do not publish noalias/nonnull/dereferenceable facts; MIR
// return-proof rows decide when those facts may be exposed to backends.
//
// Thread-safety contract: these leaves may be called concurrently for distinct
// allocations. They do not serialize double-free, use-after-free, or concurrent
// mutation of the same allocation; allocator policy owners must preserve single
// logical ownership for each pointer.

use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_void;
use std::ptr;

const HEADER_BYTES: usize = 16;
const MIN_ALIGN: usize = 16;

fn payload_layout(payload_size: usize) -> Option<Layout> {
    let total = HEADER_BYTES.checked_add(payload_size)?;
    Layout::from_size_align(total, MIN_ALIGN).ok()
}

fn normalized_payload_size(size: u64) -> Option<usize> {
    let requested = usize::try_from(size).ok()?;
    Some(requested.max(1))
}

#[no_mangle]
pub extern "C" fn hako_mem_alloc(size: u64) -> *mut c_void {
    let Some(payload_size) = normalized_payload_size(size) else {
        return std::ptr::null_mut();
    };
    let Some(layout) = payload_layout(payload_size) else {
        return std::ptr::null_mut();
    };

    unsafe {
        let base = alloc(layout);
        if base.is_null() {
            return std::ptr::null_mut();
        }
        base.cast::<usize>().write(payload_size);
        base.add(HEADER_BYTES).cast::<c_void>()
    }
}

#[no_mangle]
pub extern "C" fn hako_mem_realloc(ptr: *mut c_void, new_size: u64) -> *mut c_void {
    if ptr.is_null() {
        return hako_mem_alloc(new_size);
    }

    let Some(new_payload_size) = normalized_payload_size(new_size) else {
        return ptr::null_mut();
    };
    let Some(new_layout) = payload_layout(new_payload_size) else {
        return ptr::null_mut();
    };

    unsafe {
        let old_base = ptr.cast::<u8>().sub(HEADER_BYTES);
        let old_payload_size = old_base.cast::<usize>().read();
        let Some(old_layout) = payload_layout(old_payload_size) else {
            return ptr::null_mut();
        };

        let new_base = alloc(new_layout);
        if new_base.is_null() {
            return ptr::null_mut();
        }
        new_base.cast::<usize>().write(new_payload_size);
        let new_ptr = new_base.add(HEADER_BYTES);
        ptr::copy_nonoverlapping(
            ptr.cast::<u8>(),
            new_ptr,
            old_payload_size.min(new_payload_size),
        );
        dealloc(old_base, old_layout);
        new_ptr.cast::<c_void>()
    }
}

#[no_mangle]
pub extern "C" fn hako_mem_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let base = ptr.cast::<u8>().sub(HEADER_BYTES);
        let payload_size = base.cast::<usize>().read();
        if let Some(layout) = payload_layout(payload_size) {
            dealloc(base, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hako_mem_alloc_returns_aligned_payload_and_free_accepts_null() {
        let ptr = hako_mem_alloc(64);
        assert!(!ptr.is_null());
        assert_eq!((ptr as usize) % MIN_ALIGN, 0);
        hako_mem_free(ptr);
        hako_mem_free(std::ptr::null_mut());
    }

    #[test]
    fn hako_mem_realloc_preserves_payload_prefix() {
        let ptr = hako_mem_alloc(8);
        assert!(!ptr.is_null());
        unsafe {
            let bytes = ptr.cast::<u8>();
            for i in 0..8 {
                bytes.add(i).write((i + 1) as u8);
            }
        }

        let grown = hako_mem_realloc(ptr, 32);
        assert!(!grown.is_null());
        unsafe {
            let bytes = grown.cast::<u8>();
            for i in 0..8 {
                assert_eq!(bytes.add(i).read(), (i + 1) as u8);
            }
        }

        let shrunk = hako_mem_realloc(grown, 4);
        assert!(!shrunk.is_null());
        unsafe {
            let bytes = shrunk.cast::<u8>();
            for i in 0..4 {
                assert_eq!(bytes.add(i).read(), (i + 1) as u8);
            }
        }
        hako_mem_free(shrunk);
    }

    #[test]
    fn hako_mem_alloc_realloc_free_are_thread_safe_for_distinct_allocations() {
        let mut threads = Vec::new();
        for tid in 0..4usize {
            threads.push(std::thread::spawn(move || {
                for iter in 0..64usize {
                    let ptr = hako_mem_alloc((16 + iter) as u64);
                    assert!(!ptr.is_null());
                    unsafe {
                        ptr.cast::<u8>().write((tid + iter) as u8);
                    }
                    let next = hako_mem_realloc(ptr, (32 + iter) as u64);
                    assert!(!next.is_null());
                    unsafe {
                        assert_eq!(next.cast::<u8>().read(), (tid + iter) as u8);
                    }
                    hako_mem_free(next);
                }
            }));
        }

        for thread in threads {
            thread.join().expect("hako_mem worker thread must finish");
        }
    }
}
