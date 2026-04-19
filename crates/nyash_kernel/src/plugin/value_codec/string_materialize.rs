use crate::plugin::value_demand::{
    DemandSet, KERNEL_TEXT_SLOT_DEFERRED_CONST_SUFFIX, KERNEL_TEXT_SLOT_EMPTY,
    KERNEL_TEXT_SLOT_OWNED_BYTES, KERNEL_TEXT_SLOT_PUBLISHED, PUBLISH_EXPLICIT_API,
    PUBLISH_EXTERNAL_BOUNDARY, PUBLISH_GENERIC_FALLBACK, PUBLISH_NEED_STABLE_OBJECT,
};
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::{ffi::CStr, mem::ManuallyDrop, sync::Arc};

#[derive(Clone, Copy)]
enum PublishReason {
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
fn record_publish_site_materialize_owned(site: StringPublishSite, bytes: usize) {
    match site {
        StringPublishSite::Generic => {}
        StringPublishSite::StringConcatHh => {
            crate::observe::record_birth_backend_site_string_concat_hh_materialize_owned(bytes);
        }
        StringPublishSite::StringSubstringConcatHhii => {
            crate::observe::record_birth_backend_site_string_substring_concat_hhii_materialize_owned(
                bytes,
            );
        }
        StringPublishSite::ConstSuffix => {
            crate::observe::record_birth_backend_site_const_suffix_materialize_owned(bytes);
        }
        StringPublishSite::FreezeTextPlanPieces3 => {
            crate::observe::record_birth_backend_site_freeze_text_plan_pieces3_materialize_owned(
                bytes,
            );
        }
    }
}

#[inline(always)]
fn record_publish_site_objectize_box(site: StringPublishSite) {
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
fn record_publish_site_publish_handle(site: StringPublishSite) {
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
    DeferredConstSuffix = 3,
}

impl KernelTextSlotState {
    #[inline(always)]
    const fn demand(self) -> DemandSet {
        match self {
            Self::Empty => KERNEL_TEXT_SLOT_EMPTY,
            Self::OwnedBytes => KERNEL_TEXT_SLOT_OWNED_BYTES,
            Self::Published => KERNEL_TEXT_SLOT_PUBLISHED,
            Self::DeferredConstSuffix => KERNEL_TEXT_SLOT_DEFERRED_CONST_SUFFIX,
        }
    }
}

#[derive(Clone, Copy)]
enum KernelTextSlotBoundary {
    PublishHandle,
    ObjectizeStableBox,
}

impl KernelTextSlotBoundary {
    #[inline(always)]
    const fn demand(self) -> DemandSet {
        match self {
            Self::PublishHandle => PUBLISH_EXTERNAL_BOUNDARY,
            Self::ObjectizeStableBox => PUBLISH_NEED_STABLE_OBJECT,
        }
    }
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
    pub(crate) fn replace_owned_string(&mut self, value: String) {
        self.replace_owned_bytes(OwnedBytes::from_string(value));
    }

    #[inline(always)]
    pub(crate) fn replace_deferred_const_suffix(&mut self, source_h: i64, suffix_ptr: *const i8) {
        self.clear();
        if source_h <= 0 || suffix_ptr.is_null() {
            return;
        }
        self.ptr = suffix_ptr as *mut u8;
        self.len = source_h as usize;
        self.cap = 0;
        self.state = KernelTextSlotState::DeferredConstSuffix as u8;
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
    pub(crate) fn deferred_const_suffix(&self) -> Option<(i64, *const i8)> {
        if self.state() != KernelTextSlotState::DeferredConstSuffix {
            return None;
        }
        Some((self.len as i64, self.ptr as *const i8))
    }

    #[inline(always)]
    pub(crate) fn take_deferred_const_suffix(&mut self) -> Option<(i64, *const i8)> {
        let value = self.deferred_const_suffix()?;
        self.reset_empty();
        Some(value)
    }

    #[inline(always)]
    pub(crate) fn take_materialized_owned_bytes(&mut self) -> Option<OwnedBytes> {
        match self.state() {
            KernelTextSlotState::OwnedBytes => self.take_owned_bytes(),
            KernelTextSlotState::DeferredConstSuffix => {
                let (source_h, suffix_ptr) = self.take_deferred_const_suffix()?;
                let source = crate::exports::string::to_owned_string_handle_arg(source_h);
                materialize_const_suffix_text(source.as_str(), suffix_ptr)
                    .map(OwnedBytes::from_string)
            }
            KernelTextSlotState::Empty | KernelTextSlotState::Published => None,
        }
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
fn record_kernel_text_slot_boundary(boundary: KernelTextSlotBoundary, state: KernelTextSlotState) {
    let _state_demand = state.demand();
    let _boundary_demand = boundary.demand();
    match state {
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix => {
            match boundary {
                KernelTextSlotBoundary::PublishHandle => {
                    crate::observe::record_birth_backend_publish_boundary_slot_publish_handle();
                }
                KernelTextSlotBoundary::ObjectizeStableBox => {
                    crate::observe::record_birth_backend_publish_boundary_slot_objectize_stable_box(
                    );
                }
            }
        }
        KernelTextSlotState::Empty => {
            crate::observe::record_birth_backend_publish_boundary_slot_empty();
        }
        KernelTextSlotState::Published => {
            crate::observe::record_birth_backend_publish_boundary_slot_already_published();
        }
    }
}

#[inline(always)]
fn take_kernel_text_slot_boundary_owned_bytes(
    slot: &mut KernelTextSlot,
    boundary: KernelTextSlotBoundary,
) -> Option<OwnedBytes> {
    let state = slot.state();
    record_kernel_text_slot_boundary(boundary, state);
    if state == KernelTextSlotState::Published {
        debug_assert!(
            slot.ptr.is_null() && slot.len == 0 && slot.cap == 0,
            "published KernelTextSlot must not retain owned bytes"
        );
    }
    if !matches!(
        state,
        KernelTextSlotState::OwnedBytes | KernelTextSlotState::DeferredConstSuffix
    ) {
        return None;
    }
    slot.take_materialized_owned_bytes()
}

#[inline(always)]
pub(crate) fn with_const_suffix_ptr_text<R>(
    suffix_ptr: *const i8,
    f: impl FnOnce(&str) -> R,
) -> Option<R> {
    if suffix_ptr.is_null() {
        return None;
    }
    let bytes = unsafe { CStr::from_ptr(suffix_ptr) }.to_bytes();
    let suffix = unsafe { std::str::from_utf8_unchecked(bytes) };
    Some(f(suffix))
}

#[inline(always)]
pub(crate) fn materialize_const_suffix_text(source: &str, suffix_ptr: *const i8) -> Option<String> {
    with_const_suffix_ptr_text(suffix_ptr, |suffix| {
        let mut out = String::with_capacity(source.len().saturating_add(suffix.len()));
        out.push_str(source);
        out.push_str(suffix);
        out
    })
}

#[inline(always)]
pub(crate) fn with_kernel_text_slot_text<R>(
    slot: &KernelTextSlot,
    f: impl FnOnce(&str) -> R,
) -> Option<R> {
    match slot.state() {
        KernelTextSlotState::OwnedBytes => {
            let bytes = unsafe { std::slice::from_raw_parts(slot.ptr as *const u8, slot.len) };
            let text = unsafe { std::str::from_utf8_unchecked(bytes) };
            Some(f(text))
        }
        KernelTextSlotState::DeferredConstSuffix => {
            let (source_h, suffix_ptr) = slot.deferred_const_suffix()?;
            let text = handles::with_text_read_session_ready(|session| {
                session.str_handle(source_h as u64, |source| {
                    materialize_const_suffix_text(source, suffix_ptr)
                })
            })
            .flatten()
            .flatten()
            .or_else(|| {
                let source = crate::exports::string::to_owned_string_handle_arg(source_h);
                materialize_const_suffix_text(source.as_str(), suffix_ptr)
            })?;
            Some(f(text.as_str()))
        }
        KernelTextSlotState::Empty | KernelTextSlotState::Published => None,
    }
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
fn freeze_owned_bytes_with_site(value: String, site: StringPublishSite) -> OwnedBytes {
    record_publish_site_materialize_owned(site, value.len());
    freeze_owned_bytes(value)
}

#[inline(always)]
fn publish_owned_bytes_with_reason(bytes: OwnedBytes, reason: PublishReason) -> i64 {
    publish_owned_bytes_with_reason_and_site(bytes, reason, StringPublishSite::Generic)
}

#[inline(always)]
fn publish_owned_bytes_with_reason_and_site(
    bytes: OwnedBytes,
    reason: PublishReason,
    site: StringPublishSite,
) -> i64 {
    publish_owned_bytes_with_reason_and_site_cold(bytes, reason, site)
}

#[cold]
#[inline(never)]
fn publish_owned_bytes_with_reason_and_site_cold(
    bytes: OwnedBytes,
    reason: PublishReason,
    site: StringPublishSite,
) -> i64 {
    record_publish_reason(reason);
    record_publish_site_objectize_box(site);
    let arc = objectize_stable_string_box(bytes);
    record_publish_site_publish_handle(site);
    issue_fresh_handle(arc)
}

fn publish_owned_bytes_explicit_api_boundary(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason(bytes, PublishReason::ExplicitApi)
}

#[cold]
#[inline(never)]
fn publish_owned_bytes_external_boundary(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason(bytes, PublishReason::ExternalBoundary)
}

#[cold]
#[inline(never)]
fn publish_owned_bytes_generic_fallback_boundary(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason(bytes, PublishReason::GenericFallback)
}

#[cfg(feature = "perf-observe")]
#[cold]
#[inline(never)]
fn publish_owned_bytes_string_concat_hh_generic_fallback_boundary(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason_and_site(
        bytes,
        PublishReason::GenericFallback,
        StringPublishSite::StringConcatHh,
    )
}

#[cfg(feature = "perf-observe")]
#[cold]
#[inline(never)]
fn publish_owned_bytes_string_substring_concat_hhii_generic_fallback_boundary(
    bytes: OwnedBytes,
) -> i64 {
    publish_owned_bytes_with_reason_and_site(
        bytes,
        PublishReason::GenericFallback,
        StringPublishSite::StringSubstringConcatHhii,
    )
}

#[cfg(feature = "perf-observe")]
#[cold]
#[inline(never)]
fn publish_owned_bytes_const_suffix_generic_fallback_boundary(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_with_reason_and_site(
        bytes,
        PublishReason::GenericFallback,
        StringPublishSite::ConstSuffix,
    )
}

#[cfg(feature = "perf-observe")]
#[cold]
#[inline(never)]
fn publish_owned_bytes_freeze_text_plan_pieces3_generic_fallback_boundary(
    bytes: OwnedBytes,
) -> i64 {
    publish_owned_bytes_with_reason_and_site(
        bytes,
        PublishReason::GenericFallback,
        StringPublishSite::FreezeTextPlanPieces3,
    )
}

#[inline(always)]
fn publish_owned_bytes_generic_fallback_boundary_for_site(
    bytes: OwnedBytes,
    site: StringPublishSite,
) -> i64 {
    #[cfg(feature = "perf-observe")]
    {
        match site {
            StringPublishSite::Generic => publish_owned_bytes_generic_fallback_boundary(bytes),
            StringPublishSite::StringConcatHh => {
                publish_owned_bytes_string_concat_hh_generic_fallback_boundary(bytes)
            }
            StringPublishSite::StringSubstringConcatHhii => {
                publish_owned_bytes_string_substring_concat_hhii_generic_fallback_boundary(bytes)
            }
            StringPublishSite::ConstSuffix => {
                publish_owned_bytes_const_suffix_generic_fallback_boundary(bytes)
            }
            StringPublishSite::FreezeTextPlanPieces3 => {
                publish_owned_bytes_freeze_text_plan_pieces3_generic_fallback_boundary(bytes)
            }
        }
    }
    #[cfg(not(feature = "perf-observe"))]
    {
        publish_owned_bytes_with_reason_and_site(bytes, PublishReason::GenericFallback, site)
    }
}

#[inline(always)]
pub(crate) fn publish_owned_bytes(bytes: OwnedBytes) -> i64 {
    publish_owned_bytes_explicit_api_boundary(bytes)
}

#[inline(always)]
pub(crate) fn publish_kernel_text_slot(slot: &mut KernelTextSlot) -> Option<i64> {
    let bytes =
        take_kernel_text_slot_boundary_owned_bytes(slot, KernelTextSlotBoundary::PublishHandle)?;
    let handle = publish_owned_bytes_external_boundary(bytes);
    slot.mark_published();
    Some(handle)
}

#[cold]
#[inline(never)]
pub(crate) fn objectize_kernel_text_slot_stable_box(
    slot: &mut KernelTextSlot,
) -> Option<Arc<dyn NyashBox>> {
    let bytes = take_kernel_text_slot_boundary_owned_bytes(
        slot,
        KernelTextSlotBoundary::ObjectizeStableBox,
    )?;
    record_publish_reason(PublishReason::NeedStableObject);
    Some(objectize_stable_string_box(bytes))
}

#[inline(always)]
pub(crate) fn materialize_owned_string_generic_fallback(value: String) -> i64 {
    publish_owned_bytes_generic_fallback_boundary(freeze_owned_bytes(value))
}

#[inline(always)]
pub(crate) fn materialize_owned_string_generic_fallback_for_site(
    value: String,
    site: StringPublishSite,
) -> i64 {
    publish_owned_bytes_generic_fallback_boundary_for_site(
        freeze_owned_bytes_with_site(value, site),
        site,
    )
}

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    publish_owned_bytes_explicit_api_boundary(freeze_owned_bytes(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_reason_maps_to_runtime_private_demand_set() {
        assert_eq!(
            PublishReason::ExternalBoundary.demand(),
            PUBLISH_EXTERNAL_BOUNDARY
        );
        assert_eq!(
            PublishReason::GenericFallback.demand(),
            PUBLISH_GENERIC_FALLBACK
        );
        assert_eq!(PublishReason::ExplicitApi.demand(), PUBLISH_EXPLICIT_API);
        assert_eq!(
            PublishReason::NeedStableObject.demand(),
            PUBLISH_NEED_STABLE_OBJECT
        );
    }

    #[test]
    fn kernel_text_slot_state_maps_to_runtime_private_demand_set() {
        assert_eq!(KernelTextSlotState::Empty.demand(), KERNEL_TEXT_SLOT_EMPTY);
        assert_eq!(
            KernelTextSlotState::OwnedBytes.demand(),
            KERNEL_TEXT_SLOT_OWNED_BYTES
        );
        assert_eq!(
            KernelTextSlotState::Published.demand(),
            KERNEL_TEXT_SLOT_PUBLISHED
        );
        assert_eq!(
            KernelTextSlotState::DeferredConstSuffix.demand(),
            KERNEL_TEXT_SLOT_DEFERRED_CONST_SUFFIX
        );
    }

    #[test]
    fn kernel_text_slot_boundary_maps_to_publish_demand_set() {
        assert_eq!(
            KernelTextSlotBoundary::PublishHandle.demand(),
            PUBLISH_EXTERNAL_BOUNDARY
        );
        assert_eq!(
            KernelTextSlotBoundary::ObjectizeStableBox.demand(),
            PUBLISH_NEED_STABLE_OBJECT
        );
    }
}
