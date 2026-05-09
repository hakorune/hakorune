// hako.mem native memory exports for pure-first EXE lowering.
//
// These exports own raw native allocation for the runtime/kernel link target.
// They intentionally do not publish noalias/nonnull/dereferenceable facts; MIR
// return-proof rows decide when those facts may be exposed to backends.

use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_void;

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
}
