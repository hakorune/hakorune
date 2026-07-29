//! Sole cleanup-block admission policy for Return and Throw exits.

use crate::mir::builder::function_lowering_state::FunctionLoweringStateV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CleanupExitKindV1 {
    Return,
    Throw,
}

pub(in crate::mir::builder) fn ensure_cleanup_exit_allowed_v1(
    state: &FunctionLoweringStateV1,
    kind: CleanupExitKindV1,
) -> Result<(), String> {
    if !state.in_cleanup_block {
        return Ok(());
    }
    let (allowed, diagnostic) = match kind {
        CleanupExitKindV1::Return => (
            state.cleanup_allow_return,
            "return is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_RETURN=1 to permit)",
        ),
        CleanupExitKindV1::Throw => (
            state.cleanup_allow_throw,
            "throw is not allowed inside cleanup block (enable NYASH_CLEANUP_ALLOW_THROW=1 to permit)",
        ),
    };
    allowed.then_some(()).ok_or_else(|| diagnostic.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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

        state.in_cleanup_block = true;
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

        state.cleanup_allow_return = true;
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Return),
            Ok(())
        );
        state.cleanup_allow_throw = true;
        assert_eq!(
            ensure_cleanup_exit_allowed_v1(&state, CleanupExitKindV1::Throw),
            Ok(())
        );
    }
}
