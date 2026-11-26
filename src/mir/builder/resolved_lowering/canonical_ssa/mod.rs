//! Shared function-owned SSA session for canonical lowering profiles.
//!
//! Profiles decide which resolved source shape is admitted. This module owns
//! only the common identity, semantic, CFG, PHI, and completion machinery that
//! every admitted profile must share.

mod function_control;
pub(in crate::mir::builder) use function_control::SharedFunctionControlV1;

mod identity;
mod session;

#[cfg(test)]
pub(in crate::mir) use identity::CanonicalBindingReadReceiptV1;
pub(super) use identity::ResolvedSsaIdentityStateV2;
pub(super) use session::{finish_profile_close, CanonicalSsaFunctionSessionV2};
