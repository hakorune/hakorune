use super::borrowed_handle::SourceLifetimeKeep;
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
    pub(crate) fn into_keep(self) -> SourceLifetimeKeep {
        self.keep
    }
}

#[inline(always)]
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
    f: impl FnOnce(StringHandleSourceKind, ArrayStoreStrSource) -> R,
) -> R {
    handles::with_handle_caller(
        source_handle as u64,
        handles::PerfObserveObjectWithHandleCaller::ArrayStoreStrSource,
        |source_obj| {
            let (source_kind, source) = match classify_string_like_proof(source_obj) {
                Some(proof) => {
                    let source_obj = source_obj.expect("string-like source object");
                    (
                        StringHandleSourceKind::StringLike,
                        ArrayStoreStrSource::StringLike(materialize_verified_text_source(
                            source_obj, proof,
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
            f(source_kind, source)
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
