use super::{borrowed_handle::SourceLifetimeKeep, TextRef};
use crate::observe;
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
pub(crate) struct VerifiedTextSource {
    proof: StringLikeProof,
    keep: SourceLifetimeKeep,
}

impl VerifiedTextSource {
    #[inline(always)]
    pub(crate) fn new(proof: StringLikeProof, keep: SourceLifetimeKeep) -> Self {
        Self { proof, keep }
    }

    #[inline(always)]
    pub(crate) fn proof(&self) -> StringLikeProof {
        self.proof
    }

    #[inline(always)]
    pub(crate) fn with_text<R>(&self, f: impl FnOnce(TextRef<'_>) -> R) -> Option<R> {
        self.keep.with_text(f)
    }

    #[inline(always)]
    pub(crate) fn with_text_and_proof<R>(
        &self,
        f: impl FnOnce(TextRef<'_>, StringLikeProof) -> R,
    ) -> Option<R> {
        let proof = self.proof;
        self.keep.with_text(|text| f(text, proof))
    }

    #[inline(always)]
    pub(crate) fn copy_owned_text_cold(&self) -> String {
        self.keep.copy_owned_text_cold()
    }

    #[inline(always)]
    pub(crate) fn into_keep(self) -> SourceLifetimeKeep {
        self.keep
    }
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn materialize_verified_text_source(
    source_obj: &Arc<dyn NyashBox>,
    proof: StringLikeProof,
) -> VerifiedTextSource {
    VerifiedTextSource::new(
        proof,
        match proof {
            StringLikeProof::StringBox => SourceLifetimeKeep::string_box(source_obj.clone()),
            StringLikeProof::StringView => SourceLifetimeKeep::string_view(source_obj.clone()),
        },
    )
}

#[derive(Clone)]
pub(crate) enum ArrayStoreStrSource {
    StringLike(VerifiedTextSource),
    OtherObject,
    Missing,
}

#[inline(always)]
#[allow(dead_code)] // Phase 291x-126: compatibility classifier kept for direct handle-source probes.
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

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
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

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn lookup_array_store_str_source_obj<R>(
    source_handle: i64,
    f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R,
) -> R {
    let mut f = Some(f);
    if source_handle > 0 && observe::len_route_matches_latest_fresh_handle(source_handle) {
        if let Some(result) =
            handles::with_latest_fresh_stable_box(source_handle as u64, |source_obj| {
                lookup_array_store_str_source_caller_latest_fresh_tag(source_handle);
                let f = f
                    .take()
                    .expect("array store latest-fresh callback should run once");
                f(Some(source_obj))
            })
        {
            return result;
        }
    }
    lookup_array_store_str_source_registry_slot(source_handle, |source_obj| {
        if source_obj.is_some() {
            lookup_array_store_str_source_caller_latest_fresh_tag(source_handle);
        }
        let f = f
            .take()
            .expect("array store source callback should run once");
        f(source_obj)
    })
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn lookup_array_store_str_source_registry_slot<R>(
    source_handle: i64,
    f: impl FnOnce(Option<&Arc<dyn NyashBox>>) -> R,
) -> R {
    crate::observe::record_store_array_str_lookup_registry_slot_read();
    handles::with_handle(source_handle as u64, f)
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn lookup_array_store_str_source_caller_latest_fresh_tag(source_handle: i64) {
    crate::observe::record_store_array_str_lookup_caller_latest_fresh_tag();
    handles::perf_observe_object_with_handle_caller(
        source_handle as u64,
        handles::PerfObserveObjectWithHandleCaller::ArrayStoreStrSource,
    );
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn classify_array_store_str_source_proof(
    source_obj: Option<&Arc<dyn NyashBox>>,
) -> Option<StringLikeProof> {
    classify_string_like_proof(source_obj)
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
fn shape_array_store_str_verified_source(
    source_obj: &Arc<dyn NyashBox>,
    proof: StringLikeProof,
) -> ArrayStoreStrSource {
    ArrayStoreStrSource::StringLike(materialize_verified_text_source(source_obj, proof))
}

#[inline(always)]
pub(crate) fn with_array_store_str_source<R>(
    source_handle: i64,
    f: impl FnOnce(StringHandleSourceKind, ArrayStoreStrSource) -> R,
) -> R {
    lookup_array_store_str_source_obj(source_handle, |source_obj| {
        let (source_kind, source) = match classify_array_store_str_source_proof(source_obj) {
            Some(proof) => {
                let source_obj = source_obj.expect("string-like source object");
                (
                    StringHandleSourceKind::StringLike,
                    shape_array_store_str_verified_source(source_obj, proof),
                )
            }
            None if source_obj.is_some() => (
                StringHandleSourceKind::OtherObject,
                ArrayStoreStrSource::OtherObject,
            ),
            None => (
                StringHandleSourceKind::Missing,
                ArrayStoreStrSource::Missing,
            ),
        };
        f(source_kind, source)
    })
}

#[inline(always)]
#[allow(dead_code)] // Phase 291x-126: compatibility predicate retained for old value-codec callers.
pub(crate) fn is_string_handle_source(source_obj: &Arc<dyn NyashBox>) -> bool {
    matches!(
        classify_string_handle_source(Some(source_obj)),
        StringHandleSourceKind::StringLike
    )
}
