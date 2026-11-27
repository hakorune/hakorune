//! Shared cleanup exit admission.

mod exit_admission;

pub(in crate::mir::builder) use exit_admission::{
    ensure_cleanup_exit_allowed_v1, CleanupExitKindV1, CleanupExitPolicyV1,
};
