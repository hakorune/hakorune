// hako.osvm native virtual-memory exports for pure-first EXE lowering.
//
// These exports own only the narrow OSVM ABI used by substrate proof rows.
// MIR owns route facts; allocator policy and pointer proof remain above this layer.

use std::ffi::{c_int, c_void};

const HAKO_OK: i64 = 0;
#[cfg(unix)]
const HAKO_OOM: i64 = 4;
#[cfg(not(unix))]
const HAKO_UNSUPPORTED: i64 = 5;
const HAKO_VALIDATION: i64 = 6;
const DEFAULT_PAGE_BYTES: usize = 4096;

fn page_bytes() -> usize {
    DEFAULT_PAGE_BYTES
}

fn round_up_bytes(size: usize, page: usize) -> Option<usize> {
    let adjusted = size.checked_add(page.checked_sub(1)?)?;
    Some((adjusted / page) * page)
}

fn normalize_len(len_bytes: i64) -> Option<usize> {
    if len_bytes <= 0 {
        return None;
    }
    let size = usize::try_from(len_bytes).ok()?;
    round_up_bytes(size, page_bytes())
}

#[cfg(not(unix))]
fn region_input_is_positive(base: i64, len_bytes: i64) -> bool {
    base > 0 && len_bytes > 0
}

#[cfg(unix)]
fn normalize_region_len(base: i64, len_bytes: i64) -> Option<usize> {
    if base <= 0 {
        return None;
    }
    normalize_len(len_bytes)
}

#[no_mangle]
pub extern "C" fn hako_osvm_page_size_i64() -> i64 {
    page_bytes() as i64
}

#[cfg(unix)]
mod platform {
    use super::*;

    const PROT_NONE: c_int = 0x0;
    const PROT_READ: c_int = 0x1;
    const PROT_WRITE: c_int = 0x2;
    const MAP_PRIVATE: c_int = 0x02;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    const MAP_ANON_FLAG: c_int = 0x20;

    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    const MAP_ANON_FLAG: c_int = 0x1000;

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "netbsd"
    )))]
    const MAP_ANON_FLAG: c_int = 0x20;

    unsafe extern "C" {
        fn mmap(
            addr: *mut c_void,
            length: usize,
            prot: c_int,
            flags: c_int,
            fd: c_int,
            offset: isize,
        ) -> *mut c_void;
        fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
        fn munmap(addr: *mut c_void, length: usize) -> c_int;
    }

    pub(super) fn reserve(len_bytes: i64) -> i64 {
        let Some(len) = normalize_len(len_bytes) else {
            return 0;
        };
        unsafe {
            let ptr = mmap(
                std::ptr::null_mut(),
                len,
                PROT_NONE,
                MAP_PRIVATE | MAP_ANON_FLAG,
                -1,
                0,
            );
            if (ptr as isize) == -1 {
                0
            } else {
                ptr as isize as i64
            }
        }
    }

    pub(super) fn commit(base: i64, len_bytes: i64) -> i64 {
        let Some(len) = normalize_region_len(base, len_bytes) else {
            return HAKO_VALIDATION;
        };
        let rc = unsafe { mprotect(base as isize as *mut c_void, len, PROT_READ | PROT_WRITE) };
        if rc == 0 {
            HAKO_OK
        } else {
            HAKO_OOM
        }
    }

    pub(super) fn decommit(base: i64, len_bytes: i64) -> i64 {
        let Some(len) = normalize_region_len(base, len_bytes) else {
            return HAKO_VALIDATION;
        };
        let rc = unsafe { mprotect(base as isize as *mut c_void, len, PROT_NONE) };
        if rc == 0 {
            HAKO_OK
        } else {
            HAKO_VALIDATION
        }
    }

    pub(super) fn unreserve(base: i64, len_bytes: i64) -> i64 {
        let Some(len) = normalize_region_len(base, len_bytes) else {
            return HAKO_VALIDATION;
        };
        let rc = unsafe { munmap(base as isize as *mut c_void, len) };
        if rc == 0 {
            HAKO_OK
        } else {
            HAKO_VALIDATION
        }
    }
}

#[cfg(not(unix))]
mod platform {
    use super::*;

    pub(super) fn reserve(_len_bytes: i64) -> i64 {
        0
    }

    pub(super) fn commit(base: i64, len_bytes: i64) -> i64 {
        if !region_input_is_positive(base, len_bytes) {
            HAKO_VALIDATION
        } else {
            HAKO_UNSUPPORTED
        }
    }

    pub(super) fn decommit(base: i64, len_bytes: i64) -> i64 {
        if !region_input_is_positive(base, len_bytes) {
            HAKO_VALIDATION
        } else {
            HAKO_UNSUPPORTED
        }
    }

    pub(super) fn unreserve(base: i64, len_bytes: i64) -> i64 {
        if !region_input_is_positive(base, len_bytes) {
            HAKO_VALIDATION
        } else {
            HAKO_UNSUPPORTED
        }
    }
}

#[no_mangle]
pub extern "C" fn hako_osvm_reserve_bytes_i64(len_bytes: i64) -> i64 {
    platform::reserve(len_bytes)
}

#[no_mangle]
pub extern "C" fn hako_osvm_commit_bytes_i64(base: i64, len_bytes: i64) -> i64 {
    platform::commit(base, len_bytes)
}

#[no_mangle]
pub extern "C" fn hako_osvm_decommit_bytes_i64(base: i64, len_bytes: i64) -> i64 {
    platform::decommit(base, len_bytes)
}

#[no_mangle]
pub extern "C" fn hako_osvm_unreserve_bytes_i64(base: i64, len_bytes: i64) -> i64 {
    platform::unreserve(base, len_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_size_is_stable_positive_i64() {
        assert_eq!(hako_osvm_page_size_i64(), DEFAULT_PAGE_BYTES as i64);
    }

    #[cfg(unix)]
    #[test]
    fn reserve_commit_decommit_one_page() {
        let base = hako_osvm_reserve_bytes_i64(DEFAULT_PAGE_BYTES as i64);
        assert_ne!(base, 0);
        assert_eq!(
            hako_osvm_commit_bytes_i64(base, DEFAULT_PAGE_BYTES as i64),
            HAKO_OK
        );
        assert_eq!(
            hako_osvm_decommit_bytes_i64(base, DEFAULT_PAGE_BYTES as i64),
            HAKO_OK
        );
        assert_eq!(
            hako_osvm_unreserve_bytes_i64(base, DEFAULT_PAGE_BYTES as i64),
            HAKO_OK
        );
    }
}
