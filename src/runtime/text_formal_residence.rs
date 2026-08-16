//! Caller-zero pinned Text residence.
//!
//! This is the narrow runtime substrate for an already-published
//! `{slot,generation}` lane set.  It owns the lease set and the occurrence-
//! ordered root descriptors together; it does not issue source bindings,
//! MIR values, call edges, or Text execution plans.

use std::marker::PhantomData;

use super::host_handles;
use super::text_formal_abi::TextFormalWirePairV1;

pub(crate) use host_handles::{TextFormalLeaseAcquireRejectV1, TextFormalLeaseFinishRejectV1};

const RESIDENCE_FRAME_REVISION_V1: u32 = 1;
const RESIDENCE_FRAME_HEADER_SIZE_V1: u32 = 32;
const RESIDENCE_ROOT_ROW_SIZE_V1: u32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalResidenceAcquireRejectV1 {
    Lease(TextFormalLeaseAcquireRejectV1),
    FrameSizeOverflow { root_count: usize },
    RollbackFailed(TextFormalLeaseFinishRejectV1),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PinnedTextResidenceFrameHeaderV1 {
    revision: u32,
    header_size: u32,
    total_size: u32,
    root_count: u32,
    reserved_a: u64,
    reserved_b: u64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PinnedTextRootViewRef<'residence> {
    ptr: *const u8,
    byte_len: u64,
    _residence: PhantomData<&'residence ()>,
}

impl<'residence> PinnedTextRootViewRef<'residence> {
    #[inline(always)]
    pub(crate) const fn byte_len(self) -> u64 {
        self.byte_len
    }
}

/// Move-only residence owner for one invocation's occurrence-ordered roots.
#[must_use = "a pinned Text residence must be explicitly finished"]
#[derive(Debug)]
pub(crate) struct TextFormalCallResidenceV1 {
    inner: host_handles::RegistryTextFormalCallResidenceV1,
    header: PinnedTextResidenceFrameHeaderV1,
}

impl TextFormalCallResidenceV1 {
    #[inline(always)]
    pub(crate) fn root_count(&self) -> usize {
        self.header.root_count as usize
    }

    #[inline(always)]
    pub(crate) fn frame_revision(&self) -> u32 {
        self.header.revision
    }

    #[inline(always)]
    pub(crate) fn frame_size(&self) -> u32 {
        self.header.total_size
    }

    /// Lend one root descriptor only for the duration of this closure.
    /// The pointer is a backend-private projection; the residence remains the
    /// only lifetime owner and no descriptor is returned as a storable pair.
    #[inline(always)]
    pub(crate) fn with_root<R>(
        &self,
        index: usize,
        f: impl for<'residence> FnOnce(PinnedTextRootViewRef<'residence>) -> R,
    ) -> Option<R> {
        self.inner.root(index).map(|root| {
            f(PinnedTextRootViewRef {
                ptr: root.ptr,
                byte_len: root.byte_len,
                _residence: PhantomData,
            })
        })
    }

    /// Consume the residence and release every invocation pin exactly once.
    #[inline(always)]
    pub(crate) fn finish(self) -> Result<(), TextFormalLeaseFinishRejectV1> {
        self.inner.finish()
    }
}

/// Atomically validate and pin already-published pairs, then project the
/// occurrence-ordered StableText roots into one private frame owner.
pub(crate) fn acquire_text_formal_residence_v1(
    pairs: &[TextFormalWirePairV1],
) -> Result<TextFormalCallResidenceV1, TextFormalResidenceAcquireRejectV1> {
    let inner = host_handles::acquire_text_formal_call_residence_v1(pairs)
        .map_err(TextFormalResidenceAcquireRejectV1::Lease)?;
    let root_count = inner.root_count();
    let root_count_u32 = match u32::try_from(root_count) {
        Ok(value) => value,
        Err(_) => {
            return Err(rollback_residence(
                inner,
                TextFormalResidenceAcquireRejectV1::FrameSizeOverflow { root_count },
            ));
        }
    };
    let total_size = match RESIDENCE_FRAME_HEADER_SIZE_V1
        .checked_add(RESIDENCE_ROOT_ROW_SIZE_V1.saturating_mul(root_count_u32))
    {
        Some(value) => value,
        None => {
            return Err(rollback_residence(
                inner,
                TextFormalResidenceAcquireRejectV1::FrameSizeOverflow { root_count },
            ));
        }
    };
    Ok(TextFormalCallResidenceV1 {
        inner,
        header: PinnedTextResidenceFrameHeaderV1 {
            revision: RESIDENCE_FRAME_REVISION_V1,
            header_size: RESIDENCE_FRAME_HEADER_SIZE_V1,
            total_size,
            root_count: root_count_u32,
            reserved_a: 0,
            reserved_b: 0,
        },
    })
}

fn rollback_residence(
    inner: host_handles::RegistryTextFormalCallResidenceV1,
    error: TextFormalResidenceAcquireRejectV1,
) -> TextFormalResidenceAcquireRejectV1 {
    match inner.finish() {
        Ok(()) => error,
        Err(finish_error) => TextFormalResidenceAcquireRejectV1::RollbackFailed(finish_error),
    }
}

#[cfg(test)]
#[path = "text_formal_residence_tests.rs"]
mod tests;
