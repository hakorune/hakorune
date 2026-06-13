use super::{OwnedText, TextRef};
use crate::c_string::c_string_bytes;
use crate::plugin::value_demand::{
    DemandSet, PUBLISH_EXPLICIT_API, PUBLISH_EXTERNAL_BOUNDARY, PUBLISH_GENERIC_FALLBACK,
    PUBLISH_NEED_STABLE_OBJECT,
};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{mem::ManuallyDrop, sync::Arc};

#[derive(Clone, Copy)]
pub(crate) enum PublishReason {
    ExternalBoundary,
    GenericFallback,
    ExplicitApi,
    NeedStableObject,
}

impl PublishReason {
    #[inline(always)]
    const fn demand(self) -> DemandSet {
        match self {
            Self::ExternalBoundary => PUBLISH_EXTERNAL_BOUNDARY,
            Self::GenericFallback => PUBLISH_GENERIC_FALLBACK,
            Self::ExplicitApi => PUBLISH_EXPLICIT_API,
            Self::NeedStableObject => PUBLISH_NEED_STABLE_OBJECT,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum StringPublishSite {
    Generic,
    StringConcatHh,
    StringSubstringConcatHhii,
    ConstSuffix,
    FreezeTextPlanPieces3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum KernelTextSlotState {
    Empty = 0,
    OwnedBytes = 1,
    Published = 2,
    DeferredConstSuffix = 3,
}

/// Runtime-private string slot exported for AOT/LLVM lowering. The text-slot
/// publication boundary stays local here and must not be treated as a general
/// public string API. Future `TextCell` work must stay separate from
/// `KernelTextSlot`.
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
            3 => KernelTextSlotState::DeferredConstSuffix,
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
    pub(crate) fn replace_owned_bytes(&mut self, bytes: OwnedText) {
        self.clear();
        let bytes = bytes.into_string().into_bytes();
        let mut bytes = ManuallyDrop::new(bytes);
        self.ptr = bytes.as_mut_ptr();
        self.len = bytes.len();
        self.cap = bytes.capacity();
        self.state = KernelTextSlotState::OwnedBytes as u8;
    }

    #[inline(always)]
    pub(crate) fn take_materialized_owned_bytes(&mut self) -> Option<OwnedText> {
        match self.state() {
            KernelTextSlotState::OwnedBytes => {
                let value = unsafe { String::from_raw_parts(self.ptr, self.len, self.cap) };
                self.reset_empty();
                Some(OwnedText::from_string(value))
            }
            KernelTextSlotState::DeferredConstSuffix => {
                let source_h = self.len as i64;
                let suffix_ptr = self.ptr as *const i8;
                self.reset_empty();
                let source = crate::exports::string::to_owned_string_handle_arg(source_h);
                deferred_const_suffix_string(source.as_str(), suffix_ptr)
                    .map(OwnedText::from_string)
            }
            KernelTextSlotState::Empty | KernelTextSlotState::Published => None,
        }
    }
}

impl Drop for KernelTextSlot {
    #[inline(always)]
    fn drop(&mut self) {
        self.clear();
    }
}

#[inline(always)]
fn deferred_const_suffix_string(source: &str, suffix_ptr: *const i8) -> Option<String> {
    if suffix_ptr.is_null() {
        return None;
    }
    let bytes = c_string_bytes(suffix_ptr);
    let suffix = unsafe { std::str::from_utf8_unchecked(bytes) };
    let mut out = String::with_capacity(source.len().saturating_add(suffix.len()));
    out.push_str(source);
    out.push_str(suffix);
    Some(out)
}

#[inline(always)]
fn deferred_const_suffix_text(source_h: i64, suffix_ptr: *const i8) -> Option<String> {
    handles::with_text_read_session_ready(|session| {
        session.str_handle(source_h as u64, |source| {
            deferred_const_suffix_string(source, suffix_ptr)
        })
    })
    .flatten()
    .flatten()
    .or_else(|| {
        let source = crate::exports::string::to_owned_string_handle_arg(source_h);
        deferred_const_suffix_string(source.as_str(), suffix_ptr)
    })
}

#[inline(always)]
pub(crate) fn with_kernel_text_slot_text<R>(
    slot: &KernelTextSlot,
    f: impl FnOnce(TextRef<'_>) -> R,
) -> Option<R> {
    match slot.state() {
        KernelTextSlotState::OwnedBytes => {
            let bytes = unsafe { std::slice::from_raw_parts(slot.ptr as *const u8, slot.len) };
            let text = unsafe { std::str::from_utf8_unchecked(bytes) };
            Some(f(TextRef::new(text)))
        }
        KernelTextSlotState::DeferredConstSuffix => {
            let source_h = slot.len as i64;
            let suffix_ptr = slot.ptr as *const i8;
            let text = deferred_const_suffix_text(source_h, suffix_ptr)?;
            Some(f(TextRef::new(text.as_str())))
        }
        KernelTextSlotState::Empty | KernelTextSlotState::Published => None,
    }
}

#[inline(always)]
pub(crate) fn issue_fresh_handle(arc: Arc<dyn NyashBox>) -> i64 {
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::record_birth_backend_handle_issue();
        crate::observe::record_birth_backend_issue_fresh_handle();
        crate::observe::record_birth_backend_carrier_kind_handle();
    }
    let handle = handles::to_handle_arc(arc) as i64;
    handles::perf_observe_mark_latest_fresh_handle(handle as u64);
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::mark_latest_fresh_handle(handle);
    }
    handle
}

#[inline(always)]
fn issue_fresh_text_handle(text: String) -> i64 {
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::record_birth_backend_handle_issue();
        crate::observe::record_birth_backend_issue_fresh_handle();
        crate::observe::record_birth_backend_carrier_kind_handle();
    }
    let handle = handles::to_handle_text(text) as i64;
    handles::perf_observe_mark_latest_fresh_handle(handle as u64);
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::mark_latest_fresh_handle(handle);
    }
    handle
}

#[inline(always)]
pub(crate) fn freeze_owned_bytes(value: String) -> OwnedText {
    crate::observe::record_birth_backend_materialize_owned(value.len());
    crate::observe::record_birth_backend_carrier_kind_owned_bytes();
    if crate::observe::bypass_gc_alloc_enabled() {
        crate::observe::record_birth_backend_gc_alloc_skipped();
    } else {
        crate::observe::record_birth_backend_gc_alloc(value.len());
        nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    }
    OwnedText::from_string(value)
}

#[inline(always)]
pub(crate) fn freeze_owned_string_into_slot(slot: &mut KernelTextSlot, value: String) {
    slot.replace_owned_bytes(freeze_owned_bytes(value));
}

#[inline(always)]
pub(crate) fn freeze_owned_bytes_with_site(value: String, site: StringPublishSite) -> OwnedText {
    match site {
        StringPublishSite::Generic => {}
        StringPublishSite::StringConcatHh => {
            crate::observe::record_birth_backend_site_string_concat_hh_materialize_owned(
                value.len(),
            );
        }
        StringPublishSite::StringSubstringConcatHhii => {
            crate::observe::record_birth_backend_site_string_substring_concat_hhii_materialize_owned(
                value.len(),
            );
        }
        StringPublishSite::ConstSuffix => {
            crate::observe::record_birth_backend_site_const_suffix_materialize_owned(value.len());
        }
        StringPublishSite::FreezeTextPlanPieces3 => {
            crate::observe::record_birth_backend_site_freeze_text_plan_pieces3_materialize_owned(
                value.len(),
            );
        }
    }
    freeze_owned_bytes(value)
}

#[inline(always)]
fn record_publish_reason(reason: PublishReason) {
    let _demand = reason.demand();
    match reason {
        PublishReason::ExternalBoundary => {
            crate::observe::record_birth_backend_publish_reason_external_boundary();
        }
        PublishReason::NeedStableObject => {
            crate::observe::record_birth_backend_publish_reason_need_stable_object();
        }
        PublishReason::GenericFallback => {
            crate::observe::record_birth_backend_publish_reason_generic_fallback();
        }
        PublishReason::ExplicitApi => {
            crate::observe::record_birth_backend_publish_reason_explicit_api();
        }
    }
}

#[inline(always)]
fn record_publish_site_objectize(site: StringPublishSite) {
    match site {
        StringPublishSite::Generic => {}
        StringPublishSite::StringConcatHh => {
            crate::observe::record_birth_backend_site_string_concat_hh_objectize_box();
        }
        StringPublishSite::StringSubstringConcatHhii => {
            crate::observe::record_birth_backend_site_string_substring_concat_hhii_objectize_box();
        }
        StringPublishSite::ConstSuffix => {
            crate::observe::record_birth_backend_site_const_suffix_objectize_box();
        }
        StringPublishSite::FreezeTextPlanPieces3 => {
            crate::observe::record_birth_backend_site_freeze_text_plan_pieces3_objectize_box();
        }
    }
}

#[inline(always)]
fn record_publish_site_handle(site: StringPublishSite) {
    match site {
        StringPublishSite::Generic => {}
        StringPublishSite::StringConcatHh => {
            crate::observe::record_birth_backend_site_string_concat_hh_publish_handle();
        }
        StringPublishSite::StringSubstringConcatHhii => {
            crate::observe::record_birth_backend_site_string_substring_concat_hhii_publish_handle();
        }
        StringPublishSite::ConstSuffix => {
            crate::observe::record_birth_backend_site_const_suffix_publish_handle();
        }
        StringPublishSite::FreezeTextPlanPieces3 => {
            crate::observe::record_birth_backend_site_freeze_text_plan_pieces3_publish_handle();
        }
    }
}

#[cold]
#[inline(never)]
pub(crate) fn publish_owned_bytes_with_reason_and_site(
    bytes: OwnedText,
    reason: PublishReason,
    site: StringPublishSite,
) -> i64 {
    record_publish_reason(reason);
    record_publish_site_objectize(site);
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::record_birth_backend_string_box_new(bytes.as_str().len());
        crate::observe::record_birth_backend_objectize_stable_box_now(bytes.as_str().len());
        crate::observe::record_birth_backend_carrier_kind_stable_box();
    }
    let string_box = {
        let value = bytes.into_string();
        #[cfg(feature = "perf-observe")]
        {
            crate::observe::record_birth_backend_string_box_ctor(value.len());
            StringBox::perf_observe_from_owned(value)
        }
        #[cfg(not(feature = "perf-observe"))]
        {
            StringBox::new(value)
        }
    };
    #[cfg(feature = "perf-observe")]
    {
        crate::observe::record_birth_backend_arc_wrap();
    }
    let arc: Arc<dyn NyashBox> = Arc::new(string_box);
    record_publish_site_handle(site);
    issue_fresh_handle(arc)
}

#[cold]
#[inline(never)]
pub(crate) fn publish_owned_text_handle_with_reason_and_site(
    bytes: OwnedText,
    reason: PublishReason,
    site: StringPublishSite,
) -> i64 {
    record_publish_reason(reason);
    record_publish_site_handle(site);
    issue_fresh_text_handle(bytes.into_string())
}

#[inline(always)]
pub(crate) fn publish_kernel_text_slot(slot: &mut KernelTextSlot) -> Option<i64> {
    let state = slot.state();
    match state {
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix => {
            crate::observe::record_birth_backend_publish_boundary_slot_publish_handle();
        }
        KernelTextSlotState::Empty => {
            crate::observe::record_birth_backend_publish_boundary_slot_empty();
        }
        KernelTextSlotState::Published => {
            crate::observe::record_birth_backend_publish_boundary_slot_already_published();
        }
    }
    if state == KernelTextSlotState::Published {
        debug_assert!(
            slot.ptr.is_null() && slot.len == 0 && slot.cap == 0,
            "published KernelTextSlot must not retain owned bytes"
        );
    }
    let bytes = if matches!(
        state,
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix
    ) {
        slot.take_materialized_owned_bytes()
    } else {
        None
    }?;
    let handle = publish_owned_bytes_with_reason_and_site(
        bytes,
        PublishReason::ExternalBoundary,
        StringPublishSite::Generic,
    );
    slot.reset_empty();
    slot.state = KernelTextSlotState::Published as u8;
    Some(handle)
}

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    publish_owned_bytes_with_reason_and_site(
        freeze_owned_bytes(value),
        PublishReason::ExplicitApi,
        StringPublishSite::Generic,
    )
}
