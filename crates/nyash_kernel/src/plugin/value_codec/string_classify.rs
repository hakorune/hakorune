use super::borrowed_handle::SourceLifetimeKeep;
use nyash_rust::{box_trait::{NyashBox, StringBox}, runtime::host_handles as handles};
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
                    keep: match proof {
                        StringLikeProof::StringBox => SourceLifetimeKeep::string_box(
                            source_obj.expect("string-like source object").clone(),
                        ),
                        StringLikeProof::StringView => SourceLifetimeKeep::string_view(
                            source_obj.expect("string-like source object").clone(),
                        ),
                    },
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
