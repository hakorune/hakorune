use super::{borrowed_handle::SourceLifetimeKeep, TextRef};
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
        self.keep
            .backing
            .stable_box
            .as_ref()
            .as_str_fast()
            .map(TextRef::new)
            .map(f)
    }

    #[cfg(test)]
    #[inline(always)]
    pub(crate) fn into_keep(self) -> SourceLifetimeKeep {
        self.keep
    }
}

#[derive(Clone)]
pub(crate) enum ArrayStoreStrSource {
    StringLike(VerifiedTextSource),
    OtherObject,
    Missing,
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

#[inline(always)]
pub(crate) fn with_array_store_str_source<R>(
    source_handle: i64,
    f: impl FnOnce(StringHandleSourceKind, ArrayStoreStrSource) -> R,
) -> R {
    let mut f = Some(f);
    let mut dispatch = |source_obj: Option<&Arc<dyn NyashBox>>| {
        let (source_kind, source) = match classify_string_like_proof(source_obj) {
            Some(proof) => {
                let source_obj = source_obj.expect("string-like source object");
                (
                    StringHandleSourceKind::StringLike,
                    ArrayStoreStrSource::StringLike(VerifiedTextSource::new(
                        proof,
                        match proof {
                            StringLikeProof::StringBox => {
                                SourceLifetimeKeep::string_box(source_obj.clone())
                            }
                            StringLikeProof::StringView => {
                                SourceLifetimeKeep::string_view(source_obj.clone())
                            }
                        },
                    )),
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
        let f = f
            .take()
            .expect("array store source callback should run once");
        f(source_kind, source)
    };
    // Hot string-store paths favor one registry lookup over the latest-fresh
    // TLS shortcut. The latest-fresh fast path stays available for other
    // string corridors, but this route keeps the object lookup shape uniform.
    crate::observe::record_store_array_str_lookup_registry_slot_read();
    handles::with_handle(source_handle as u64, |source_obj| {
        if source_obj.is_some() {
            handles::perf_observe_object_with_handle_caller(
                source_handle as u64,
                handles::PerfObserveObjectWithHandleCaller::ArrayStoreStrSource,
            );
        }
        dispatch(source_obj)
    })
}
