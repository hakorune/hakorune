//! Caller-zero pinned Text residence.
//!
//! This is the narrow runtime substrate for an already-published
//! `{slot,generation}` lane set.  It owns the lease set and the occurrence-
//! ordered root descriptors together; it does not issue source bindings,
//! MIR values, call edges, or Text execution plans.

use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::ptr;
use std::slice;

use super::host_handles;
use super::text_formal_abi::{
    issue_text_formal_borrows_from_published_wires_v1, TextFormalBorrowStatusV1,
    TextFormalBorrowV1, TextFormalWirePairV1,
};

pub(crate) use host_handles::{TextFormalLeaseAcquireRejectV1, TextFormalLeaseFinishRejectV1};

const RESIDENCE_FRAME_REVISION_V1: u32 = 1;
const RESIDENCE_FRAME_HEADER_SIZE_V1: u32 = 32;
const RESIDENCE_ROOT_ROW_SIZE_V1: u32 = 16;
const RESIDENCE_FRAME_HEADER_ALIGNMENT_V1: u32 = 8;
const RESIDENCE_ROOT_ROW_ALIGNMENT_V1: u32 = 8;
const RESIDENCE_MAX_ROOT_COUNT_V1: u32 = 1024;
const RESIDENCE_MAX_FRAME_BYTES_V1: u32 = 65_536;
const RESIDENCE_ABI_REVISION_V1: &str = "text-formal-residence-v1";

/// Compile-time ABI facts owned by the Residence implementation.
///
/// This view intentionally contains no pointer, lease token, or invocation
/// state.  The backend-frame binder may compare it with the explicit compile
/// target capability, but may not recreate it from host layout observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResidenceAbiLayoutV1 {
    revision: &'static str,
    frame_revision: u32,
    header_size: u32,
    root_row_size: u32,
    header_alignment: u32,
    root_row_alignment: u32,
    max_root_count: u32,
    max_frame_bytes: u32,
}

impl ResidenceAbiLayoutV1 {
    pub(crate) const fn revision(self) -> &'static str {
        self.revision
    }

    pub(crate) const fn frame_revision(self) -> u32 {
        self.frame_revision
    }

    pub(crate) const fn header_size(self) -> u32 {
        self.header_size
    }

    pub(crate) const fn root_row_size(self) -> u32 {
        self.root_row_size
    }

    pub(crate) const fn header_alignment(self) -> u32 {
        self.header_alignment
    }

    pub(crate) const fn root_row_alignment(self) -> u32 {
        self.root_row_alignment
    }

    pub(crate) const fn max_root_count(self) -> u32 {
        self.max_root_count
    }

    pub(crate) const fn max_frame_bytes(self) -> u32 {
        self.max_frame_bytes
    }

    pub(crate) const fn frame_size_for_roots(self, root_count: u32) -> Option<u32> {
        if root_count > self.max_root_count {
            return None;
        }
        match self.root_row_size.checked_mul(root_count) {
            Some(rows) => match self.header_size.checked_add(rows) {
                Some(size) if size <= self.max_frame_bytes => Some(size),
                _ => None,
            },
            None => None,
        }
    }
}

/// Sole issuer for the Residence-owned compile-time ABI view.
pub(crate) const fn residence_abi_layout_v1() -> ResidenceAbiLayoutV1 {
    ResidenceAbiLayoutV1 {
        revision: RESIDENCE_ABI_REVISION_V1,
        frame_revision: RESIDENCE_FRAME_REVISION_V1,
        header_size: RESIDENCE_FRAME_HEADER_SIZE_V1,
        root_row_size: RESIDENCE_ROOT_ROW_SIZE_V1,
        header_alignment: RESIDENCE_FRAME_HEADER_ALIGNMENT_V1,
        root_row_alignment: RESIDENCE_ROOT_ROW_ALIGNMENT_V1,
        max_root_count: RESIDENCE_MAX_ROOT_COUNT_V1,
        max_frame_bytes: RESIDENCE_MAX_FRAME_BYTES_V1,
    }
}

/// Private compiler/runtime frame header.  It is not the callable Text wire;
/// the latter remains the separate slot/generation pair.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextFormalResidenceFrameHeaderV1 {
    pub revision: u32,
    pub header_size: u32,
    pub total_size: u32,
    pub root_count: u32,
    pub lease_token: u64,
    pub reserved: u64,
}

/// One occurrence-ordered root row in the private backend frame.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextFormalResidenceRootRowV1 {
    pub ptr: *const u8,
    pub byte_len: i64,
}

/// Fixed status projection for the private caller-zero frame bridge.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextFormalResidenceCStatusV1 {
    Valid = 0,
    NullArgument = 1,
    EmptyInput = 2,
    UnsupportedTarget = 3,
    MisalignedArgument = 4,
    PairFrameOverlap = 5,
    FrameTooSmall = 6,
    FrameSizeOverflow = 7,
    LeaseZeroOrOutOfRange = 16,
    LeaseMissingSlot = 17,
    LeaseGenerationMismatch = 18,
    LeaseNonTextPayload = 19,
    LeaseRetirementPending = 20,
    LeasePinCountOverflow = 21,
    LeaseByteLengthOutOfRange = 22,
    LeaseTokenExhausted = 23,
    RollbackFailed = 24,
    InvalidFrame = 32,
    UnknownOrAlreadyFinished = 33,
    FinishMissingPinnedSlot = 34,
    FinishGenerationMismatch = 35,
    FinishPinCountUnderflow = 36,
    FinishStateMismatch = 37,
}

impl TextFormalResidenceCStatusV1 {
    #[inline(always)]
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalResidenceAcquireRejectV1 {
    Lease(TextFormalLeaseAcquireRejectV1),
    FrameSizeOverflow { root_count: usize },
    RollbackFailed(TextFormalLeaseFinishRejectV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextFormalResidenceIngressRejectV1 {
    Borrow(TextFormalBorrowStatusV1),
    Residence(TextFormalResidenceAcquireRejectV1),
}

type PinnedTextResidenceFrameHeaderV1 = TextFormalResidenceFrameHeaderV1;

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

    /// Move the runtime-owned token and root rows into the private C frame
    /// projection.  The caller must publish all rows or finish the raw token
    /// for rollback; no partial owner is retained here.
    pub(crate) fn into_raw_parts(
        self,
    ) -> (
        u64,
        PinnedTextResidenceFrameHeaderV1,
        Box<[host_handles::TextFormalRootDescriptorV1]>,
    ) {
        let Self { inner, header } = self;
        let (lease_token, roots) = inner.into_raw_parts();
        (lease_token, header, roots)
    }
}

/// Atomically validate and pin already-published pairs, then project the
/// occurrence-ordered StableText roots into one private frame owner.
pub(crate) fn acquire_text_formal_residence_v1(
    pairs: &[TextFormalWirePairV1],
) -> Result<TextFormalCallResidenceV1, TextFormalResidenceAcquireRejectV1> {
    if pairs.len() > RESIDENCE_MAX_ROOT_COUNT_V1 as usize {
        return Err(TextFormalResidenceAcquireRejectV1::FrameSizeOverflow {
            root_count: pairs.len(),
        });
    }
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
            lease_token: 0,
            reserved: 0,
        },
    })
}

/// Connect published ExactText entry lanes to the existing invocation
/// Residence owner.  The adapter is runtime-private and consumes the borrow
/// batch immediately; all pinning and root publication still happen in the
/// single atomic Residence transaction below.
pub(crate) fn acquire_text_formal_residence_from_published_wires_v1(
    wires: &[(u64, u64)],
) -> Result<TextFormalCallResidenceV1, TextFormalResidenceIngressRejectV1> {
    let borrows = issue_text_formal_borrows_from_published_wires_v1(wires)
        .map_err(TextFormalResidenceIngressRejectV1::Borrow)?;
    let pairs = borrows
        .iter()
        .map(TextFormalBorrowV1::wire_pair)
        .collect::<Vec<_>>();
    acquire_text_formal_residence_v1(&pairs).map_err(TextFormalResidenceIngressRejectV1::Residence)
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

#[inline(always)]
fn frame_total_size(root_count: u32) -> Option<u32> {
    residence_abi_layout_v1().frame_size_for_roots(root_count)
}

#[inline(always)]
fn target_layout_supported() -> bool {
    size_of::<*const u8>() == 8
        && align_of::<*const u8>() == 8
        && size_of::<TextFormalResidenceFrameHeaderV1>() == RESIDENCE_FRAME_HEADER_SIZE_V1 as usize
        && align_of::<TextFormalResidenceFrameHeaderV1>() == 8
        && size_of::<TextFormalResidenceRootRowV1>() == RESIDENCE_ROOT_ROW_SIZE_V1 as usize
        && align_of::<TextFormalResidenceRootRowV1>() == 8
}

#[inline(always)]
fn checked_region(ptr: *const u8, len: usize) -> Option<(usize, usize)> {
    let start = ptr as usize;
    Some((start, start.checked_add(len)?))
}

#[inline(always)]
fn regions_overlap(lhs: (usize, usize), rhs: (usize, usize)) -> bool {
    lhs.0 < rhs.1 && rhs.0 < lhs.1
}

fn map_acquire_status(error: TextFormalResidenceAcquireRejectV1) -> TextFormalResidenceCStatusV1 {
    match error {
        TextFormalResidenceAcquireRejectV1::Lease(error) => match error {
            TextFormalLeaseAcquireRejectV1::EmptyLeaseSet => {
                TextFormalResidenceCStatusV1::EmptyInput
            }
            TextFormalLeaseAcquireRejectV1::ZeroOrOutOfRangeSlot { .. } => {
                TextFormalResidenceCStatusV1::LeaseZeroOrOutOfRange
            }
            TextFormalLeaseAcquireRejectV1::MissingSlot { .. } => {
                TextFormalResidenceCStatusV1::LeaseMissingSlot
            }
            TextFormalLeaseAcquireRejectV1::GenerationMismatch { .. } => {
                TextFormalResidenceCStatusV1::LeaseGenerationMismatch
            }
            TextFormalLeaseAcquireRejectV1::NonTextPayload { .. } => {
                TextFormalResidenceCStatusV1::LeaseNonTextPayload
            }
            TextFormalLeaseAcquireRejectV1::RetirementPending { .. } => {
                TextFormalResidenceCStatusV1::LeaseRetirementPending
            }
            TextFormalLeaseAcquireRejectV1::PinCountOverflow { .. } => {
                TextFormalResidenceCStatusV1::LeasePinCountOverflow
            }
            TextFormalLeaseAcquireRejectV1::ByteLengthOutOfRange { .. } => {
                TextFormalResidenceCStatusV1::LeaseByteLengthOutOfRange
            }
            TextFormalLeaseAcquireRejectV1::TokenExhausted => {
                TextFormalResidenceCStatusV1::LeaseTokenExhausted
            }
        },
        TextFormalResidenceAcquireRejectV1::FrameSizeOverflow { .. } => {
            TextFormalResidenceCStatusV1::FrameSizeOverflow
        }
        TextFormalResidenceAcquireRejectV1::RollbackFailed(_) => {
            TextFormalResidenceCStatusV1::RollbackFailed
        }
    }
}

fn map_finish_status(error: TextFormalLeaseFinishRejectV1) -> TextFormalResidenceCStatusV1 {
    match error {
        TextFormalLeaseFinishRejectV1::UnknownOrAlreadyFinished => {
            TextFormalResidenceCStatusV1::UnknownOrAlreadyFinished
        }
        TextFormalLeaseFinishRejectV1::MissingPinnedSlot => {
            TextFormalResidenceCStatusV1::FinishMissingPinnedSlot
        }
        TextFormalLeaseFinishRejectV1::PinnedGenerationMismatch => {
            TextFormalResidenceCStatusV1::FinishGenerationMismatch
        }
        TextFormalLeaseFinishRejectV1::PinCountUnderflow => {
            TextFormalResidenceCStatusV1::FinishPinCountUnderflow
        }
        TextFormalLeaseFinishRejectV1::CallLifetimeStateMismatch => {
            TextFormalResidenceCStatusV1::FinishStateMismatch
        }
    }
}

/// Caller-zero C projection for the private residence frame.
///
/// The pair array and frame buffer are caller-owned for this invocation.  The
/// runtime never retains either pointer; it retains only the move-only lease
/// record behind the opaque token written into the frame.
pub unsafe fn enter_text_formal_residence_c_v1(
    pairs: *const TextFormalBorrowV1,
    pair_count: u32,
    frame: *mut TextFormalResidenceFrameHeaderV1,
    frame_bytes: u32,
) -> u32 {
    if !target_layout_supported() {
        return TextFormalResidenceCStatusV1::UnsupportedTarget.as_u32();
    }
    if pair_count == 0 {
        return TextFormalResidenceCStatusV1::EmptyInput.as_u32();
    }
    if pairs.is_null() || frame.is_null() {
        return TextFormalResidenceCStatusV1::NullArgument.as_u32();
    }

    let pair_count_usize = pair_count as usize;
    if pair_count > RESIDENCE_MAX_ROOT_COUNT_V1 {
        return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32();
    }
    let pair_bytes = match size_of::<TextFormalBorrowV1>().checked_mul(pair_count_usize) {
        Some(bytes) => bytes,
        None => return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32(),
    };
    let required_bytes = match frame_total_size(pair_count) {
        Some(bytes) => bytes,
        None => return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32(),
    };
    if frame_bytes < required_bytes {
        return TextFormalResidenceCStatusV1::FrameTooSmall.as_u32();
    }
    if (pairs as usize) % align_of::<TextFormalBorrowV1>() != 0
        || (frame as usize) % align_of::<TextFormalResidenceFrameHeaderV1>() != 0
    {
        return TextFormalResidenceCStatusV1::MisalignedArgument.as_u32();
    }
    let Some(pair_region) = checked_region(pairs.cast::<u8>(), pair_bytes) else {
        return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32();
    };
    let Some(frame_region) = checked_region(frame.cast::<u8>(), frame_bytes as usize) else {
        return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32();
    };
    if regions_overlap(pair_region, frame_region) {
        return TextFormalResidenceCStatusV1::PairFrameOverlap.as_u32();
    }

    let pair_slice = slice::from_raw_parts(pairs, pair_count_usize);
    let wire_pairs = pair_slice
        .iter()
        .map(TextFormalBorrowV1::wire_pair)
        .collect::<Vec<_>>();
    let residence = match acquire_text_formal_residence_v1(&wire_pairs) {
        Ok(residence) => residence,
        Err(error) => return map_acquire_status(error).as_u32(),
    };
    let (lease_token, header, roots) = residence.into_raw_parts();
    if header.root_count != pair_count || header.total_size != required_bytes {
        let _ = host_handles::finish_text_formal_call_lease_set_raw_v1(lease_token);
        return TextFormalResidenceCStatusV1::FrameSizeOverflow.as_u32();
    }
    let rows_ptr = frame
        .cast::<u8>()
        .add(size_of::<TextFormalResidenceFrameHeaderV1>())
        .cast::<TextFormalResidenceRootRowV1>();
    for (index, root) in roots.iter().enumerate() {
        let Ok(byte_len) = i64::try_from(root.byte_len) else {
            let _ = host_handles::finish_text_formal_call_lease_set_raw_v1(lease_token);
            return TextFormalResidenceCStatusV1::LeaseByteLengthOutOfRange.as_u32();
        };
        ptr::write(
            rows_ptr.add(index),
            TextFormalResidenceRootRowV1 {
                ptr: root.ptr,
                byte_len,
            },
        );
    }
    ptr::write(
        frame,
        TextFormalResidenceFrameHeaderV1 {
            revision: header.revision,
            header_size: header.header_size,
            total_size: header.total_size,
            root_count: header.root_count,
            lease_token,
            reserved: 0,
        },
    );
    TextFormalResidenceCStatusV1::Valid.as_u32()
}

/// Consume the private frame's move-only residence token exactly once.
unsafe fn finish_text_formal_residence_c_v1(frame: *mut TextFormalResidenceFrameHeaderV1) -> u32 {
    if !target_layout_supported() {
        return TextFormalResidenceCStatusV1::UnsupportedTarget.as_u32();
    }
    if frame.is_null() {
        return TextFormalResidenceCStatusV1::NullArgument.as_u32();
    }
    if (frame as usize) % align_of::<TextFormalResidenceFrameHeaderV1>() != 0 {
        return TextFormalResidenceCStatusV1::MisalignedArgument.as_u32();
    }
    let header = &*frame;
    if header.revision != RESIDENCE_FRAME_REVISION_V1
        || header.header_size != RESIDENCE_FRAME_HEADER_SIZE_V1
        || header.lease_token == 0
        || frame_total_size(header.root_count) != Some(header.total_size)
    {
        return TextFormalResidenceCStatusV1::InvalidFrame.as_u32();
    }
    match host_handles::finish_text_formal_call_lease_set_raw_v1(header.lease_token) {
        Ok(()) => {
            ptr::write(
                frame,
                TextFormalResidenceFrameHeaderV1 {
                    revision: header.revision,
                    header_size: header.header_size,
                    total_size: header.total_size,
                    root_count: header.root_count,
                    lease_token: 0,
                    reserved: header.reserved,
                },
            );
            TextFormalResidenceCStatusV1::Valid.as_u32()
        }
        Err(error) => map_finish_status(error).as_u32(),
    }
}

/// Terminal C-facing finish projection.
///
/// The status-returning core remains the single Residence state-transition
/// owner. This wrapper deliberately exposes no status to a caller: a valid
/// finish returns after consuming the token, while every nonzero status
/// fail-stops inside the runtime. The wrapper itself is not `noreturn`; only
/// the failure path is.
#[cold]
#[inline(never)]
fn abort_text_formal_residence_finish_v1(status: u32) -> ! {
    debug_assert_ne!(status, TextFormalResidenceCStatusV1::Valid.as_u32());
    std::process::abort()
}

/// Consume the private frame exactly once, returning only on success.
pub unsafe fn finish_text_formal_residence_or_abort_v1(
    frame: *mut TextFormalResidenceFrameHeaderV1,
) {
    let status = finish_text_formal_residence_c_v1(frame);
    if status != TextFormalResidenceCStatusV1::Valid.as_u32() {
        abort_text_formal_residence_finish_v1(status);
    }
}

#[cfg(test)]
#[path = "text_formal_residence_tests.rs"]
mod tests;
