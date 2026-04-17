use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{mem::ManuallyDrop, sync::Arc};

#[derive(Clone, Copy)]
enum PublishReason {
    ExternalBoundary,
    GenericFallback,
    ExplicitApi,
}

#[inline(always)]
fn record_publish_reason(reason: PublishReason) {
    match reason {
        PublishReason::ExternalBoundary => {
            crate::observe::record_birth_backend_publish_reason_external_boundary();
        }
        PublishReason::GenericFallback => {
            crate::observe::record_birth_backend_publish_reason_generic_fallback();
        }
        PublishReason::ExplicitApi => {
            crate::observe::record_birth_backend_publish_reason_explicit_api();
        }
    }
}

pub(crate) struct OwnedBytes(String);

impl OwnedBytes {
    #[inline(always)]
    fn from_string(value: String) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline(always)]
    pub(crate) fn into_string(self) -> String {
        self.0
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KernelTextSlotState {
    Empty = 0,
    OwnedBytes = 1,
    Published = 2,
}

/// Runtime-private direct-kernel string carrier passed through exported C ABI seams.
/// The symbol is exported for AOT/LLVM lowering, but semantic ownership stays local
/// to the active corridor and must not be treated as a general public string API.
#[repr(C)]
pub struct KernelTextSlot {
    pub(crate) state: u8,
    pub(crate) ptr: *mut u8,
    pub(crate) len: usize,
    pub(crate) cap: usize,
}

impl KernelTextSlot {
    #[inline(always)]
    pub(crate) const fn empty() -> Self {
        Self {
            state: KernelTextSlotState::Empty as u8,
            ptr: std::ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    #[inline(always)]
    pub(crate) fn state(&self) -> KernelTextSlotState {
        match self.state {
            1 => KernelTextSlotState::OwnedBytes,
            2 => KernelTextSlotState::Published,
            _ => KernelTextSlotState::Empty,
        }
    }

    #[inline(always)]
    fn reset_empty(&mut self) {
        self.state = KernelTextSlotState::Empty as u8;
        self.ptr = std::ptr::null_mut();
        self.len = 0;
        self.cap = 0;
    }

    #[inline(always)]
    pub(crate) fn clear(&mut self) {
        if self.state() == KernelTextSlotState::OwnedBytes {
            unsafe {
                drop(String::from_raw_parts(self.ptr, self.len, self.cap));
            }
        }
        self.reset_empty();
    }

    #[inline(always)]
    pub(crate) fn replace_owned_bytes(&mut self, bytes: OwnedBytes) {
        self.clear();
        let bytes = bytes.into_string().into_bytes();
        let mut bytes = ManuallyDrop::new(bytes);
        self.ptr = bytes.as_mut_ptr();
        self.len = bytes.len();
        self.cap = bytes.capacity();
        self.state = KernelTextSlotState::OwnedBytes as u8;
    }

    #[inline(always)]
    pub(crate) fn take_owned_bytes(&mut self) -> Option<OwnedBytes> {
        if self.state() != KernelTextSlotState::OwnedBytes {
            return None;
        }
        let value = unsafe { String::from_raw_parts(self.ptr, self.len, self.cap) };
        self.reset_empty();
        Some(OwnedBytes::from_string(value))
    }

    #[inline(always)]
    pub(crate) fn mark_published(&mut self) {
        self.reset_empty();
        self.state = KernelTextSlotState::Published as u8;
    }
}

impl Drop for KernelTextSlot {
    #[inline(always)]
    fn drop(&mut self) {
        self.clear();
    }
}

#[inline(always)]
pub(crate) fn with_kernel_text_slot_text<R>(
    slot: &KernelTextSlot,
    f: impl FnOnce(&str) -> R,
) -> Option<R> {
    if slot.state() != KernelTextSlotState::OwnedBytes {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(slot.ptr as *const u8, slot.len) };
    let text = unsafe { std::str::from_utf8_unchecked(bytes) };
    Some(f(text))
}

#[cfg(feature = "perf-observe")]
#[inline(never)]
fn birth_string_box_from_owned(value: String) -> StringBox {
    crate::observe::record_birth_backend_string_box_ctor(value.len());
    StringBox::perf_observe_from_owned(value)
}

#[cfg(not(feature = "perf-observe"))]
#[inline(always)]
fn birth_string_box_from_owned(value: String) -> StringBox {
    StringBox::new(value)
}

#[cfg(feature = "perf-observe")]
#[inline(never)]
fn wrap_string_box_in_arc(string_box: StringBox) -> Arc<dyn NyashBox> {
    crate::observe::record_birth_backend_arc_wrap();
    Arc::new(string_box)
}

#[cfg(not(feature = "perf-observe"))]
#[inline(always)]
fn wrap_string_box_in_arc(string_box: StringBox) -> Arc<dyn NyashBox> {
    Arc::new(string_box)
}

#[cfg(feature = "perf-observe")]
#[inline(never)]
fn objectize_stable_string_box(bytes: OwnedBytes) -> Arc<dyn NyashBox> {
    crate::observe::record_birth_backend_string_box_new(bytes.0.len());
    crate::observe::record_birth_backend_objectize_stable_box_now(bytes.0.len());
    crate::observe::record_birth_backend_carrier_kind_stable_box();
    let string_box = birth_string_box_from_owned(bytes.into_string());
    wrap_string_box_in_arc(string_box)
}

#[cfg(not(feature = "perf-observe"))]
#[inline(always)]
fn objectize_stable_string_box(bytes: OwnedBytes) -> Arc<dyn NyashBox> {
    let string_box = birth_string_box_from_owned(bytes.into_string());
    wrap_string_box_in_arc(string_box)
}

#[cfg(feature = "perf-observe")]
#[inline(never)]
fn issue_fresh_handle(arc: Arc<dyn NyashBox>) -> i64 {
    crate::observe::record_birth_backend_handle_issue();
    crate::observe::record_birth_backend_issue_fresh_handle();
    crate::observe::record_birth_backend_carrier_kind_handle();
    let handle = handles::to_handle_arc(arc) as i64;
    handles::perf_observe_mark_latest_fresh_handle(handle as u64);
    crate::observe::mark_latest_fresh_handle(handle);
    handle
}

#[cfg(not(feature = "perf-observe"))]
#[inline(always)]
fn issue_fresh_handle(arc: Arc<dyn NyashBox>) -> i64 {
    let handle = handles::to_handle_arc(arc) as i64;
    handles::perf_observe_mark_latest_fresh_handle(handle as u64);
    handle
}

#[inline(always)]
pub(crate) fn issue_fresh_handle_from_arc(arc: Arc<dyn NyashBox>) -> i64 {
    issue_fresh_handle(arc)
}

#[cfg(feature = "perf-observe")]
#[inline(never)]
pub(crate) fn freeze_owned_bytes(value: String) -> OwnedBytes {
    crate::observe::record_birth_backend_materialize_owned(value.len());
    crate::observe::record_birth_backend_carrier_kind_owned_bytes();
    if crate::observe::bypass_gc_alloc_enabled() {
        crate::observe::record_birth_backend_gc_alloc_skipped();
    } else {
        crate::observe::record_birth_backend_gc_alloc(value.len());
        nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    }
    OwnedBytes::from_string(value)
}

#[cfg(not(feature = "perf-observe"))]
#[inline(always)]
pub(crate) fn freeze_owned_bytes(value: String) -> OwnedBytes {
    crate::observe::record_birth_backend_materialize_owned(value.len());
    crate::observe::record_birth_backend_carrier_kind_owned_bytes();
    if crate::observe::bypass_gc_alloc_enabled() {
        crate::observe::record_birth_backend_gc_alloc_skipped();
    } else {
        crate::observe::record_birth_backend_gc_alloc(value.len());
        nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    }
    OwnedBytes::from_string(value)
}

#[inline(always)]
pub(crate) fn freeze_owned_string_into_slot(slot: &mut KernelTextSlot, value: String) {
    slot.replace_owned_bytes(freeze_owned_bytes(value));
}

#[inline(always)]
fn publish_owned_bytes_with_reason(bytes: OwnedBytes, reason: PublishReason) -> i64 {
    record_publish_reason(reason);
    let arc = objectize_stable_string_box(bytes);
    issue_fresh_handle(arc)
}

#[inline(always)]
pub(crate) fn publish_owned_bytes(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason(bytes, PublishReason::ExplicitApi)
}

#[inline(always)]
pub(crate) fn publish_kernel_text_slot(slot: &mut KernelTextSlot) -> Option<i64> {
    let bytes = slot.take_owned_bytes()?;
    let handle = publish_owned_bytes_with_reason(bytes, PublishReason::ExternalBoundary);
    slot.mark_published();
    Some(handle)
}

#[inline(always)]
pub(crate) fn objectize_kernel_text_slot_stable_box(
    slot: &mut KernelTextSlot,
) -> Option<Arc<dyn NyashBox>> {
    let bytes = slot.take_owned_bytes()?;
    Some(objectize_stable_string_box(bytes))
}

#[inline(always)]
pub(crate) fn materialize_owned_string_generic_fallback(value: String) -> i64 {
    publish_owned_bytes_with_reason(freeze_owned_bytes(value), PublishReason::GenericFallback)
}

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    publish_owned_bytes_with_reason(freeze_owned_bytes(value), PublishReason::ExplicitApi)
}
