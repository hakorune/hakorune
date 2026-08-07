//! Shared function-owned SSA session for canonical lowering profiles.
//!
//! Profiles decide which resolved source shape is admitted. This module owns
//! only the common identity, semantic, CFG, PHI, and completion machinery that
//! every admitted profile must share.

mod identity;
mod session;

pub(super) use identity::{CanonicalBindingReadReceiptV1, ResolvedSsaIdentityStateV2};
pub(super) use session::{finish_profile_close, CanonicalSsaFunctionSessionV2};
