//! Caller-zero façade for invocation-wide Text-formal call leases.
//!
//! The façade keeps the registry token opaque and move-only. It does not
//! decide callable signatures, actual origins, MIR lanes, or exit coverage.

use super::host_handles;
use super::text_formal_abi::TextFormalBorrowV1;

pub(crate) use host_handles::{TextFormalLeaseAcquireRejectV1, TextFormalLeaseFinishRejectV1};

/// One callee invocation's complete ExactText formal lease set.
///
/// Explicit consuming `finish` is the only release authority. There is no
/// implicit `Drop` finish because a forgotten token is an invariant failure,
/// not a recoverable cleanup policy.
#[must_use = "a Text formal call lease set must be explicitly finished"]
#[derive(Debug)]
pub(crate) struct TextFormalCallLeaseSetTokenV1 {
    inner: host_handles::RegistryTextFormalCallLeaseSetV1,
}

impl TextFormalCallLeaseSetTokenV1 {
    #[inline(always)]
    pub(crate) fn finish(self) -> Result<(), TextFormalLeaseFinishRejectV1> {
        host_handles::finish_text_formal_call_lease_set_v1(self.inner)
    }
}

/// Atomically validate and pin every ExactText formal occurrence.
///
/// Duplicate pairs remain duplicate occurrences and therefore add duplicate
/// pins. Empty sets are rejected; a signature with no ExactText formals uses
/// the separate no-lease disposition and never receives a fake token.
pub(crate) fn acquire_text_formal_call_leases_v1(
    formals: &[TextFormalBorrowV1],
) -> Result<TextFormalCallLeaseSetTokenV1, TextFormalLeaseAcquireRejectV1> {
    let pairs = formals
        .iter()
        .map(TextFormalBorrowV1::wire_pair)
        .collect::<Vec<_>>();
    host_handles::acquire_text_formal_call_lease_set_v1(&pairs)
        .map(|inner| TextFormalCallLeaseSetTokenV1 { inner })
}

#[cfg(test)]
#[path = "text_formal_call_lease_tests.rs"]
mod tests;
