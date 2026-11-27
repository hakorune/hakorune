//! Strict Match-return source diagnostics.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RejectReason {
    MatchReturnScrutineeNotSupported,
    MatchReturnTooFewArms,
    MatchReturnElseNotLiteral,
    MatchReturnArmNotLiteral,
}

impl RejectReason {
    fn tag(self) -> &'static str {
        match self {
            Self::MatchReturnScrutineeNotSupported => "match_return_scrutinee_not_supported",
            Self::MatchReturnTooFewArms => "match_return_too_few_arms",
            Self::MatchReturnElseNotLiteral => "match_return_else_not_literal",
            Self::MatchReturnArmNotLiteral => "match_return_arm_not_literal",
        }
    }

    pub(super) fn as_freeze_message(self) -> &'static str {
        match self {
            Self::MatchReturnScrutineeNotSupported => {
                "match return scrutinee must be var or int literal"
            }
            Self::MatchReturnTooFewArms => "match return requires >= 2 arms",
            Self::MatchReturnElseNotLiteral => "match return else must be literal",
            Self::MatchReturnArmNotLiteral => "match return arm must be literal",
        }
    }
}

pub(super) fn log_reject(reason: RejectReason) {
    if crate::config::env::is_joinir_debug() {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[plan/reject] box=match_return_facts reason={} handoff=out_of_scope",
            reason.tag()
        ));
    }
}
