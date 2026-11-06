//! Shared helpers for the loop_cond_break_continue family.
//!
//! The former `lower_loop_cond_break_continue` oracle pipeline entry was
//! retired with the oracle-only chain; what remains here are the carrier /
//! accept-kind helpers still used by the live source lowerers.

use crate::mir::builder::control_flow::facts::loop_cond_break_continue::LoopCondBreakAcceptKind;

pub(super) const LOOP_COND_ERR: &str = "[normalizer] loop_cond_break_continue";

/// Facts->Lower contract pin: every issued accept kind must have a lowering
/// arm in both the raw and the located-source consumers. Keep exhaustive.
pub(super) const fn pin_accept_kind_contract(kind: LoopCondBreakAcceptKind) {
    match kind {
        LoopCondBreakAcceptKind::ExitIf => (),
        LoopCondBreakAcceptKind::ContinueIf => (),
        LoopCondBreakAcceptKind::ConditionalUpdate => (),
        LoopCondBreakAcceptKind::ReturnInExitIf => (),
        LoopCondBreakAcceptKind::ReturnOnlyBody => (),
        LoopCondBreakAcceptKind::ElseOnlyReturn => (),
        LoopCondBreakAcceptKind::ElseOnlyBreak => (),
        LoopCondBreakAcceptKind::MixedIf => (),
        LoopCondBreakAcceptKind::NestedLoopOnly => (),
        LoopCondBreakAcceptKind::ProgramBlockNoExit => (),
        LoopCondBreakAcceptKind::NoExitBody => (),
    }
}
