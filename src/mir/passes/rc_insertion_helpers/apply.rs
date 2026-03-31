#[cfg(feature = "rc-insertion-minimal")]
use crate::ast::Span;
#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::MirInstruction;

#[cfg(feature = "rc-insertion-minimal")]
use super::types::{DropPoint, DropReason, DropSite, RcPlan};
#[cfg(feature = "rc-insertion-minimal")]
use super::util::sorted_release_values;
#[cfg(feature = "rc-insertion-minimal")]
use super::RcInsertionStats;

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn apply_rc_plan(
    insts: Vec<MirInstruction>,
    spans: Vec<Span>,
    terminator: Option<MirInstruction>,
    terminator_span: Option<Span>,
    plan: RcPlan,
    stats: &mut RcInsertionStats,
) -> (
    Vec<MirInstruction>,
    Vec<Span>,
    Option<MirInstruction>,
    Option<Span>,
) {
    let mut drops_before_instr: Vec<Vec<DropSite>> = vec![Vec::new(); insts.len()];
    let mut drops_before_terminator: Vec<DropSite> = Vec::new();

    for drop_site in plan.drops {
        match drop_site.at {
            DropPoint::BeforeInstr(idx) => {
                if idx < drops_before_instr.len() {
                    drops_before_instr[idx].push(drop_site);
                } else {
                    debug_assert!(
                        idx < drops_before_instr.len(),
                        "rc_insertion plan references out-of-range instruction index"
                    );
                }
            }
            DropPoint::BeforeTerminator => {
                drops_before_terminator.push(drop_site);
            }
        }
    }

    let mut new_insts = Vec::with_capacity(insts.len() * 2);
    let mut new_spans = Vec::with_capacity(spans.len() * 2);

    for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
        for drop_site in drops_before_instr[idx].drain(..) {
            let _ = drop_site.reason;
            new_insts.push(MirInstruction::ReleaseStrong {
                values: sorted_release_values(drop_site.values),
            });
            new_spans.push(span.clone());
            stats.release_inserted += 1;
        }

        new_insts.push(inst);
        new_spans.push(span);
    }

    if !drops_before_terminator.is_empty() {
        let span = terminator_span.clone().unwrap_or_else(Span::unknown);
        for drop_site in drops_before_terminator {
            match drop_site.reason {
                DropReason::ReturnCleanup => {
                    debug_assert!(
                        matches!(terminator, Some(MirInstruction::Return { .. })),
                        "rc_insertion: ReturnCleanup planned for non-Return terminator"
                    );
                }
                DropReason::BreakCleanup => {
                    debug_assert!(
                        matches!(terminator, Some(MirInstruction::Jump { .. })),
                        "rc_insertion: BreakCleanup planned for non-Jump terminator"
                    );
                }
                DropReason::ContinueCleanup => {
                    debug_assert!(
                        matches!(terminator, Some(MirInstruction::Jump { .. })),
                        "rc_insertion: ContinueCleanup planned for non-Jump terminator"
                    );
                }
                _ => {
                    debug_assert!(
                        false,
                        "rc_insertion: non-cleanup reason planned before terminator"
                    );
                }
            }
            new_insts.push(MirInstruction::ReleaseStrong {
                values: sorted_release_values(drop_site.values),
            });
            new_spans.push(span.clone());
            stats.release_inserted += 1;
        }
    }

    (new_insts, new_spans, terminator, terminator_span)
}
