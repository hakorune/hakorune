"""Raw Rust source for the provider-backed global-allocator smoke probe."""

from __future__ import annotations


RUST_SOURCE = r"""
use std::alloc::{GlobalAlloc, Layout};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const RTLD_NOW: c_int = 2;
const RTLD_LOCAL: c_int = 0;
const API_MAGIC: u32 = 0x484B5241;
const API_MAJOR: u16 = 1;
const TRACK_CAP: usize = 65536;

#[repr(C)]
struct HakoHostAllocatorV0 {
    abi_major: u32,
    struct_size: u32,
    ctx: *mut c_void,
    malloc_fn: Option<unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void>,
    calloc_fn: Option<unsafe extern "C" fn(*mut c_void, usize, usize) -> *mut c_void>,
    realloc_fn: Option<unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> *mut c_void>,
    free_fn: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    usable_size_fn: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> usize>,
}

#[repr(C)]
struct HakoProviderApiV1 {
    magic: u32,
    abi_major: u16,
    abi_minor: u16,
    api_table_size: u32,
    ping: Option<unsafe extern "C" fn() -> c_int>,
    alloc: Option<unsafe extern "C" fn(usize, usize) -> *mut c_void>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
    owns: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    free_claim: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    usable_size_claim: Option<unsafe extern "C" fn(*mut c_void, *mut usize) -> c_int>,
    realloc_claim: Option<unsafe extern "C" fn(*mut c_void, usize, *mut *mut c_void) -> c_int>,
    init_host_allocator: Option<unsafe extern "C" fn(*const HakoHostAllocatorV0) -> c_int>,
}

type GetApi = unsafe extern "C" fn() -> *mut HakoProviderApiV1;

unsafe extern "C" {
    fn getenv(name: *const c_char) -> *mut c_char;
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(count: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn malloc_usable_size(ptr: *mut c_void) -> usize;
}

struct ProviderGlobalAlloc;

#[global_allocator]
static GLOBAL: ProviderGlobalAlloc = ProviderGlobalAlloc;

static PROVIDER_READY: AtomicBool = AtomicBool::new(false);
static PROVIDER_ATTEMPTED: AtomicBool = AtomicBool::new(false);
static IN_PROVIDER_INIT: AtomicBool = AtomicBool::new(false);
static PROVIDER_BIND_SUCCESS: AtomicUsize = AtomicUsize::new(0);
static PROVIDER_BIND_FAILURE: AtomicUsize = AtomicUsize::new(0);
static PROVIDER_ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static PROVIDER_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
static PROVIDER_REALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static RUNTIME_FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);
static INIT_FALLBACK_COUNT: AtomicUsize = AtomicUsize::new(0);
static POINTER_TABLE_OVERFLOW: AtomicUsize = AtomicUsize::new(0);
static mut API: *mut HakoProviderApiV1 = ptr::null_mut();
static mut HOST_ALLOCATOR: HakoHostAllocatorV0 = HakoHostAllocatorV0 {
    abi_major: 0,
    struct_size: std::mem::size_of::<HakoHostAllocatorV0>() as u32,
    ctx: ptr::null_mut(),
    malloc_fn: Some(host_malloc),
    calloc_fn: Some(host_calloc),
    realloc_fn: Some(host_realloc),
    free_fn: Some(host_free),
    usable_size_fn: Some(host_usable_size),
};
static mut TRACKED_PTRS: [usize; TRACK_CAP] = [0; TRACK_CAP];

unsafe extern "C" fn host_malloc(_ctx: *mut c_void, size: usize) -> *mut c_void {
    malloc(size)
}

unsafe extern "C" fn host_calloc(_ctx: *mut c_void, count: usize, size: usize) -> *mut c_void {
    calloc(count, size)
}

unsafe extern "C" fn host_realloc(_ctx: *mut c_void, ptr: *mut c_void, size: usize) -> *mut c_void {
    realloc(ptr, size)
}

unsafe extern "C" fn host_free(_ctx: *mut c_void, ptr: *mut c_void) {
    free(ptr)
}

unsafe extern "C" fn host_usable_size(_ctx: *mut c_void, ptr: *mut c_void) -> usize {
    malloc_usable_size(ptr)
}

unsafe fn track_ptr(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    let value = ptr as usize;
    let base = ptr::addr_of_mut!(TRACKED_PTRS) as *mut usize;
    for index in 0..TRACK_CAP {
        let slot = base.add(index);
        if *slot == 0 {
            *slot = value;
            return;
        }
    }
    POINTER_TABLE_OVERFLOW.fetch_add(1, Ordering::Relaxed);
}

unsafe fn untrack_ptr(ptr: *mut u8) -> bool {
    if ptr.is_null() {
        return false;
    }
    let value = ptr as usize;
    let base = ptr::addr_of_mut!(TRACKED_PTRS) as *mut usize;
    for index in 0..TRACK_CAP {
        let slot = base.add(index);
        if *slot == value {
            *slot = 0;
            return true;
        }
    }
    false
}

unsafe fn is_tracked(ptr: *mut u8) -> bool {
    if ptr.is_null() {
        return false;
    }
    let value = ptr as usize;
    let base = ptr::addr_of!(TRACKED_PTRS) as *const usize;
    for index in 0..TRACK_CAP {
        if *base.add(index) == value {
            return true;
        }
    }
    false
}

unsafe fn ensure_provider() -> bool {
    if PROVIDER_READY.load(Ordering::Acquire) {
        return true;
    }
    if PROVIDER_ATTEMPTED.swap(true, Ordering::AcqRel) {
        return false;
    }
    IN_PROVIDER_INIT.store(true, Ordering::Release);
    let path = getenv(b"HAKORUNE_PROVIDER_LIBRARY\0".as_ptr().cast());
    if path.is_null() {
        PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
        IN_PROVIDER_INIT.store(false, Ordering::Release);
        return false;
    }
    let handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if handle.is_null() {
        PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
        IN_PROVIDER_INIT.store(false, Ordering::Release);
        return false;
    }
    let sym = dlsym(handle, b"hakorune_provider_get_api_v1\0".as_ptr().cast());
    if sym.is_null() {
        PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
        IN_PROVIDER_INIT.store(false, Ordering::Release);
        return false;
    }
    let get_api: GetApi = std::mem::transmute(sym);
    let api = get_api();
    if api.is_null()
        || (*api).magic != API_MAGIC
        || (*api).abi_major != API_MAJOR
        || ((*api).api_table_size as usize) < std::mem::size_of::<HakoProviderApiV1>()
        || (*api).alloc.is_none()
        || (*api).free.is_none()
    {
        PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
        IN_PROVIDER_INIT.store(false, Ordering::Release);
        return false;
    }
    API = api;
    let host_allocator_required =
        !getenv(b"HAKORUNE_PROVIDER_HOST_ALLOCATOR_REQUIRED\0".as_ptr().cast()).is_null();
    if let Some(init_host_allocator) = (*api).init_host_allocator {
        let init_result = init_host_allocator(ptr::addr_of!(HOST_ALLOCATOR));
        if host_allocator_required && init_result != 1 {
            PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
            API = ptr::null_mut();
            IN_PROVIDER_INIT.store(false, Ordering::Release);
            return false;
        }
    } else if host_allocator_required {
        PROVIDER_BIND_FAILURE.fetch_add(1, Ordering::Relaxed);
        API = ptr::null_mut();
        IN_PROVIDER_INIT.store(false, Ordering::Release);
        return false;
    }
    PROVIDER_BIND_SUCCESS.fetch_add(1, Ordering::Relaxed);
    PROVIDER_READY.store(true, Ordering::Release);
    IN_PROVIDER_INIT.store(false, Ordering::Release);
    true
}

unsafe impl GlobalAlloc for ProviderGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if IN_PROVIDER_INIT.load(Ordering::Acquire) {
            INIT_FALLBACK_COUNT.fetch_add(1, Ordering::Relaxed);
            return malloc(layout.size()).cast();
        }
        if !ensure_provider() || API.is_null() {
            RUNTIME_FALLBACK_COUNT.fetch_add(1, Ordering::Relaxed);
            return malloc(layout.size()).cast();
        }
        let align = layout.align().max(1);
        let ptr = ((*API).alloc.unwrap())(layout.size(), align).cast::<u8>();
        if !ptr.is_null() {
            track_ptr(ptr);
            PROVIDER_ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if ptr.is_null() {
            return;
        }
        if is_tracked(ptr) && !API.is_null() {
            untrack_ptr(ptr);
            ((*API).free.unwrap())(ptr.cast());
            PROVIDER_FREE_COUNT.fetch_add(1, Ordering::Relaxed);
            return;
        }
        free(ptr.cast());
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if ptr.is_null() {
            return self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()));
        }
        if !is_tracked(ptr) {
            return realloc(ptr.cast(), new_size).cast();
        }
        let next = self.alloc(Layout::from_size_align_unchecked(new_size, layout.align()));
        if next.is_null() {
            return ptr::null_mut();
        }
        ptr::copy_nonoverlapping(ptr, next, layout.size().min(new_size));
        self.dealloc(ptr, layout);
        PROVIDER_REALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        next
    }
}

fn main() {
    let mut data = Vec::with_capacity(16);
    for i in 0..128u64 {
        data.push(i);
    }
    data.reserve(128);
    let sum: u64 = data.iter().copied().sum();
    drop(data);

    println!("global_allocator_smoke_sum={sum}");
    println!("rust_provider_bind_success={}", PROVIDER_BIND_SUCCESS.load(Ordering::Relaxed));
    println!("rust_provider_bind_failure={}", PROVIDER_BIND_FAILURE.load(Ordering::Relaxed));
    println!("rust_provider_alloc_count={}", PROVIDER_ALLOC_COUNT.load(Ordering::Relaxed));
    println!("rust_provider_free_count={}", PROVIDER_FREE_COUNT.load(Ordering::Relaxed));
    println!("rust_provider_realloc_count={}", PROVIDER_REALLOC_COUNT.load(Ordering::Relaxed));
    println!("rust_runtime_fallback_count={}", RUNTIME_FALLBACK_COUNT.load(Ordering::Relaxed));
    println!("rust_init_fallback_count={}", INIT_FALLBACK_COUNT.load(Ordering::Relaxed));
    println!("rust_pointer_table_overflow={}", POINTER_TABLE_OVERFLOW.load(Ordering::Relaxed));
}
"""
