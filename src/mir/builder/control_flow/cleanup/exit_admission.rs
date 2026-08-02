//! Sole cleanup-block admission policy for Return and Throw exits.

use crate::mir::builder::function_lowering_state::FunctionLoweringStateV1;

/// Immutable cleanup-exit policy captured before a TryCatch region lowers.
///
/// The policy is an input to the protected-region operation, never an ambient
/// lower-side environment read. Function state receives these booleans only
/// while the region's cleanup body is active.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::mir::builder) struct CleanupExitPolicyV1 {
    allow_return: bool,
    allow_throw: bool,
}

impl CleanupExitPolicyV1 {
    pub(in crate::mir::builder) fn capture_from_environment() -> Self {
        Self {
            allow_return: crate::config::env::cleanup_allow_return(),
            allow_throw: crate::config::env::cleanup_allow_throw(),
        }
    }

    pub(in crate::mir::builder) const fn allows_return(self) -> bool {
        self.allow_return
    }

    pub(in crate::mir::builder) const fn allows_throw(self) -> bool {
        self.allow_throw
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CleanupExitKindV1 {
    Return,
    Throw,
}

pub(in crate::mir::builder) fn ensure_cleanup_exit_allowed_v1(
    state: &FunctionLoweringStateV1,
    kind: CleanupExitKindV1,
) -> Result<(), String> {
    if !state.protected_region.cleanup.active {
        return Ok(());
    }
    let (allowed, diagnostic) = match kind {
        CleanupExitKindV1::Return => (
            state.protected_region.cleanup.allow_return,
            "return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)",
        ),
        CleanupExitKindV1::Throw => (
            state.protected_region.cleanup.allow_throw,
            "throw is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_THROW=1 to permit)",
        ),
    };
    allowed.then_some(()).ok_or_else(|| diagnostic.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_exit_policy_is_immutable_after_capture() {
        let policy = crate::test_support::with_env_vars(
            &[
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("1")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("0")),
            ],
            CleanupExitPolicyV1::capture_from_environment,
        );
        crate::test_support::with_env_vars(
            &[
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("0")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("1")),
            ],
            || {
                assert!(policy.allows_return());
                assert!(!policy.allows_throw());
            },
        );
    }

    #[test]
    fn cleanup_exit_admission_matrix_is_exact() {
        let mut state = FunctionLoweringStateV1::default();
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Return),
            Ok(())
        );
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Throw),
            Ok(())
        );

        state.protected_region.cleanup.active = true;
        let return_error =
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Return).unwrap_err();
        let throw_error =
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Throw).unwrap_err();
        assert_eq!(
            return_error,
            "return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)"
        );
        assert_eq!(
            throw_error,
            "throw is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_THROW=1 to permit)"
        );

        state.protected_region.cleanup.allow_return = true;
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Return),
            Ok(())
        );
        state.protected_region.cleanup.allow_throw = true;
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Throw),
            Ok(())
        );
    }
}
