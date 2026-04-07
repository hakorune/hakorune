use super::borrowed_handle::{maybe_borrow_string_handle_with_epoch, SourceLifetimeKeep};
use super::decode::int_arg_to_box;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StringHandleSourceKind {
    StringLike,
    OtherObject,
    Missing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StringLikeProof {
    StringBox,
    StringView,
}

#[derive(Clone)]
pub(crate) enum ArrayStoreStrSource {
    StringLike {
        proof: StringLikeProof,
        keep: SourceLifetimeKeep,
    },
    OtherObject(Arc<dyn NyashBox>),
    Missing,
}

impl ArrayStoreStrSource {
    #[inline(always)]
    pub(crate) fn stable_object_fallback_ref(&self) -> Option<&Arc<dyn NyashBox>> {
        match self {
            Self::StringLike { keep, .. } => Some(keep.stable_box_ref()),
            Self::OtherObject(obj) => Some(obj),
            Self::Missing => None,
        }
    }

    #[inline(always)]
    pub(crate) fn source_kind(&self) -> StringHandleSourceKind {
        match self {
            Self::StringLike { .. } => StringHandleSourceKind::StringLike,
            Self::OtherObject(_) => StringHandleSourceKind::OtherObject,
            Self::Missing => StringHandleSourceKind::Missing,
        }
    }

    #[inline(always)]
    pub(crate) fn record_observe_source_kind(&self) {
        match self {
            Self::StringLike {
                proof: StringLikeProof::StringBox,
                ..
            } => crate::observe::record_store_array_str_source_string_box(),
            Self::StringLike {
                proof: StringLikeProof::StringView,
                ..
            } => crate::observe::record_store_array_str_source_string_view(),
            Self::OtherObject(_) => {}
            Self::Missing => crate::observe::record_store_array_str_source_missing(),
        }
    }
}

struct OwnedBytes(String);

impl OwnedBytes {
    #[inline(always)]
    fn from_string(value: String) -> Self {
        Self(value)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn into_string(self) -> String {
        self.0
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
    crate::observe::record_birth_backend_string_box_new(bytes.len());
    crate::observe::record_birth_backend_objectize_stable_box_now(bytes.len());
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

#[cfg(feature = "perf-observe")]
#[inline(never)]
fn materialize_owned_bytes(value: String) -> OwnedBytes {
    crate::observe::record_birth_backend_materialize_owned(value.len());
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
fn materialize_owned_bytes(value: String) -> OwnedBytes {
    crate::observe::record_birth_backend_materialize_owned(value.len());
    if crate::observe::bypass_gc_alloc_enabled() {
        crate::observe::record_birth_backend_gc_alloc_skipped();
    } else {
        crate::observe::record_birth_backend_gc_alloc(value.len());
        nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    }
    OwnedBytes::from_string(value)
}

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    let bytes = materialize_owned_bytes(value);
    let arc = objectize_stable_string_box(bytes);
    issue_fresh_handle(arc)
}

#[inline(always)]
pub(crate) fn store_string_box_from_source(
    source_handle: i64,
    source_obj: Option<&Arc<dyn NyashBox>>,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = source_obj else {
        return int_arg_to_box(source_handle);
    };
    if obj.as_any().downcast_ref::<StringBox>().is_some()
        || obj
            .as_any()
            .downcast_ref::<crate::exports::string_view::StringViewBox>()
            .is_some()
    {
        crate::observe::record_birth_placement_store_from_source();
        return maybe_borrow_string_handle_with_epoch(
            obj.clone(),
            source_handle,
            source_drop_epoch,
        );
    }
    int_arg_to_box(source_handle)
}

#[inline(always)]
pub(crate) fn classify_string_handle_source(
    source_obj: Option<&Arc<dyn NyashBox>>,
) -> StringHandleSourceKind {
    match classify_string_like_proof(source_obj) {
        Some(_) => StringHandleSourceKind::StringLike,
        None => {
            if source_obj.is_some() {
                StringHandleSourceKind::OtherObject
            } else {
                StringHandleSourceKind::Missing
            }
        }
    }
}

#[inline(always)]
pub(crate) fn classify_string_like_proof(
    source_obj: Option<&Arc<dyn NyashBox>>,
) -> Option<StringLikeProof> {
    let source_obj = source_obj?;
    if source_obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Some(StringLikeProof::StringBox);
    }
    if source_obj
        .as_any()
        .downcast_ref::<crate::exports::string_view::StringViewBox>()
        .is_some()
    {
        return Some(StringLikeProof::StringView);
    }
    None
}

#[inline(always)]
pub(crate) fn with_array_store_str_source<R>(
    source_handle: i64,
    f: impl FnOnce(ArrayStoreStrSource) -> R,
) -> R {
    handles::with_handle_caller(
        source_handle as u64,
        handles::PerfObserveObjectWithHandleCaller::ArrayStoreStrSource,
        |source_obj| {
            let source = match classify_string_like_proof(source_obj) {
                Some(proof) => ArrayStoreStrSource::StringLike {
                    proof,
                    keep: SourceLifetimeKeep::stable_box(
                        source_obj.expect("string-like source object").clone(),
                    ),
                },
                None if source_obj.is_some() => {
                    ArrayStoreStrSource::OtherObject(source_obj.expect("object source").clone())
                }
                None => ArrayStoreStrSource::Missing,
            };
            f(source)
        },
    )
}

#[inline(always)]
pub(crate) fn is_string_handle_source(source_obj: &Arc<dyn NyashBox>) -> bool {
    matches!(
        classify_string_handle_source(Some(source_obj)),
        StringHandleSourceKind::StringLike
    )
}

#[inline(always)]
pub(crate) fn store_string_box_from_string_source(
    source_handle: i64,
    source_obj: &Arc<dyn NyashBox>,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    debug_assert!(source_handle > 0);
    debug_assert!(is_string_handle_source(source_obj));
    crate::observe::record_birth_placement_store_from_source();
    maybe_borrow_string_handle_with_epoch(source_obj.clone(), source_handle, source_drop_epoch)
}

#[inline(always)]
pub(crate) fn maybe_store_string_box_from_verified_source(
    source_handle: i64,
    source_obj: Option<&Arc<dyn NyashBox>>,
    source_drop_epoch: u64,
    source_is_string: bool,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = source_obj else {
        return int_arg_to_box(source_handle);
    };
    if source_is_string {
        crate::observe::record_birth_placement_store_from_source();
        return maybe_borrow_string_handle_with_epoch(
            obj.clone(),
            source_handle,
            source_drop_epoch,
        );
    }
    int_arg_to_box(source_handle)
}
