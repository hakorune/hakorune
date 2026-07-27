//! Candidate-only ingress for one selected pre-loop located MethodCall.
//!
//! This box consumes the exact source association, requires the existing
//! `Me -> Standard(Unified)` prepared route, and delegates ordinary child
//! descent to the existing Raw port. The selected Standard(Unified) terminal
//! alone uses the source-neutral receipt-required sibling and returns its exact
//! successful physical Call destination. Type publication remains outside.

use crate::ast::ASTNode;
use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::method_call_handlers::{
    prepare_me_call_execution_v1, PreparedMeCallExecutionV1, PreparedStandardMethodExecutionV1,
};
use crate::mir::builder::recursive_child_lowering::RawFunctionHeaderLookupPortV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::MirBuilder;

use super::member_route::MemberCallRoutePlan;
use super::method_call_terminal::emit_standard_value_terminal_with_receipt_v1;
use super::preloop_nested_result_receipt::ReachedPreloopNestedPhysicalCallV1;
use super::receiver_binding::ReceiverNormalizationPlan;
use super::unified_emitter::UnifiedValueCallReceiptErrorV1;
use super::{drive_call_arguments_v1, CallArgumentDescentPortV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopLocatedArgumentIngressStageV1 {
    MemberRoute,
    MePreparation,
    MeRoute,
    UnifiedCapability,
    ArgumentDescent,
    UnifiedTerminal,
    OuterTerminal,
    Completion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopObservedMemberRouteV1 {
    StaticReceiver,
    EnvMethod,
    StaticThis,
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopObservedMeRouteV1 {
    InlineRecord,
    InlineSetter,
    LoweredGlobal,
    StandardWeakLoad,
    StandardUpgradeRejected,
    StandardRecordHelper,
    StandardSetter,
    StandardUnified,
    StaticFallback,
    NotApplicable,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PreloopLocatedArgumentIngressErrorV1 {
    MemberRoutePreparation { detail: Box<str> },
    AlternateMemberRoute(PreloopObservedMemberRouteV1),
    MePreparation { detail: Box<str> },
    AlternateMeRoute(PreloopObservedMeRouteV1),
    UnifiedCallDisabled,
    ArgumentDescent { detail: Box<str> },
    PhysicalReceipt(UnifiedValueCallReceiptErrorV1),
    UnifiedTerminal { detail: Box<str> },
    OuterTerminal { detail: Box<str> },
    SelectedArgumentNotReached,
    SelectedArgumentNotCompleted,
    OuterTerminalNotCompleted,
}

/// Rejection retains the exact source owner in every state. A failure after a
/// successful inner generic Call additionally retains that physical receipt;
/// it is not reduced to an inferred destination or a String diagnostic.
#[derive(Debug)]
pub(super) enum RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    Source {
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        stage: PreloopLocatedArgumentIngressStageV1,
        cause: PreloopLocatedArgumentIngressErrorV1,
    },
    Physical {
        reached: ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>,
        stage: PreloopLocatedArgumentIngressStageV1,
        cause: PreloopLocatedArgumentIngressErrorV1,
    },
}

impl<'site, 'view, 'catalog> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    pub(super) fn outer_terminal(
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        detail: String,
    ) -> Self {
        reject(
            source,
            PreloopLocatedArgumentIngressStageV1::OuterTerminal,
            PreloopLocatedArgumentIngressErrorV1::OuterTerminal {
                detail: detail.into_boxed_str(),
            },
        )
    }

    pub(super) fn after_physical(
        reached: ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>,
        stage: PreloopLocatedArgumentIngressStageV1,
        cause: PreloopLocatedArgumentIngressErrorV1,
    ) -> Self {
        Self::Physical {
            reached,
            stage,
            cause,
        }
    }

    pub(super) fn completion(
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        cause: PreloopLocatedArgumentIngressErrorV1,
    ) -> Self {
        reject(
            source,
            PreloopLocatedArgumentIngressStageV1::Completion,
            cause,
        )
    }

    pub(super) const fn selected_index(&self) -> u32 {
        match self {
            Self::Source { source, .. } => source.selected().index(),
            Self::Physical { reached, .. } => reached.selected_index(),
        }
    }

    pub(super) fn selected_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        match self {
            Self::Source { source, .. } => source.selected().child().site(),
            Self::Physical { reached, .. } => reached.selected_site(),
        }
    }

    pub(super) const fn stage(&self) -> PreloopLocatedArgumentIngressStageV1 {
        match self {
            Self::Source { stage, .. } | Self::Physical { stage, .. } => *stage,
        }
    }

    pub(super) const fn cause(&self) -> &PreloopLocatedArgumentIngressErrorV1 {
        match self {
            Self::Source { cause, .. } | Self::Physical { cause, .. } => cause,
        }
    }

    #[cfg(test)]
    pub(super) const fn retained_physical_destination(&self) -> Option<crate::mir::ValueId> {
        match self {
            Self::Source { .. } => None,
            Self::Physical { reached, .. } => Some(reached.final_destination()),
        }
    }

    pub(super) fn bounded_report(&self) -> String {
        match self.cause() {
            PreloopLocatedArgumentIngressErrorV1::MemberRoutePreparation { detail } => {
                format!("[preloop-ingress/member-route-preparation] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::AlternateMemberRoute(observed) => {
                format!("[preloop-ingress/alternate-member-route] observed={observed:?}")
            }
            PreloopLocatedArgumentIngressErrorV1::MePreparation { detail } => {
                format!("[preloop-ingress/me-preparation] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::AlternateMeRoute(observed) => {
                format!("[preloop-ingress/alternate-me-route] observed={observed:?}")
            }
            PreloopLocatedArgumentIngressErrorV1::UnifiedCallDisabled => {
                "[preloop-ingress/unified-call-disabled] candidate requires the unified Call terminal"
                    .to_string()
            }
            PreloopLocatedArgumentIngressErrorV1::ArgumentDescent { detail } => {
                format!("[preloop-ingress/argument-descent] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::PhysicalReceipt(cause) => {
                format!("[preloop-ingress/physical-receipt] {cause:?}")
            }
            PreloopLocatedArgumentIngressErrorV1::UnifiedTerminal { detail } => {
                format!("[preloop-ingress/unified-terminal] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::OuterTerminal { detail } => {
                format!("[preloop-ingress/outer-terminal] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::SelectedArgumentNotReached => {
                "[preloop-ingress/completion] selected argument was not reached".to_string()
            }
            PreloopLocatedArgumentIngressErrorV1::SelectedArgumentNotCompleted => {
                "[preloop-ingress/completion] selected argument did not complete".to_string()
            }
            PreloopLocatedArgumentIngressErrorV1::OuterTerminalNotCompleted => {
                "[preloop-ingress/completion] outer terminal did not complete".to_string()
            }
        }
    }

    pub(super) fn discard(self) {
        match self {
            Self::Source { source, .. } => source.discard(),
            Self::Physical { reached, .. } => reached.discard(),
        }
    }
}

pub(super) fn lower_selected_preloop_located_argument_v1<'site, 'view, 'catalog, Port>(
    builder: &mut MirBuilder,
    ordinary: &mut Port,
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
) -> Result<
    ReachedPreloopNestedPhysicalCallV1<'site, 'view, 'catalog>,
    RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>,
>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    let route = {
        let input = source.association().input();
        builder.plan_member_call_route(input.receiver(), input.method())
    };
    let route = match route {
        Ok(route) => route,
        Err(detail) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::MemberRoute,
                PreloopLocatedArgumentIngressErrorV1::MemberRoutePreparation {
                    detail: detail.into_boxed_str(),
                },
            ))
        }
    };
    match route {
        MemberCallRoutePlan::ReceiverNormalized {
            plan: ReceiverNormalizationPlan::MeCall,
        } => {}
        MemberCallRoutePlan::StaticReceiver { .. } => {
            return Err(reject_alternate_member(
                source,
                PreloopObservedMemberRouteV1::StaticReceiver,
            ))
        }
        MemberCallRoutePlan::EnvMethod { .. } => {
            return Err(reject_alternate_member(
                source,
                PreloopObservedMemberRouteV1::EnvMethod,
            ))
        }
        MemberCallRoutePlan::ReceiverNormalized {
            plan: ReceiverNormalizationPlan::StaticThis { .. },
        } => {
            return Err(reject_alternate_member(
                source,
                PreloopObservedMemberRouteV1::StaticThis,
            ))
        }
        MemberCallRoutePlan::Standard => {
            return Err(reject_alternate_member(
                source,
                PreloopObservedMemberRouteV1::Standard,
            ))
        }
    }

    let prepared = {
        let input = source.association().input();
        prepare_me_call_execution_v1(builder, input.method(), input.arguments(), ordinary)
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(detail) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::MePreparation,
                PreloopLocatedArgumentIngressErrorV1::MePreparation {
                    detail: detail.into_boxed_str(),
                },
            ))
        }
    };
    let receiver = match prepared {
        PreparedMeCallExecutionV1::Standard {
            receiver,
            prepared: PreparedStandardMethodExecutionV1::Unified,
        } => receiver,
        alternate => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::MeRoute,
                PreloopLocatedArgumentIngressErrorV1::AlternateMeRoute(observe_me_route(alternate)),
            ))
        }
    };

    // The ordinary Raw terminal may intentionally use its compatibility Call
    // route when unified Call is disabled. This candidate cannot: its next
    // row needs the one generic physical Call seam, so reject before lowering
    // any inner argument or emitting any MIR.
    if !super::call_unified::is_unified_call_enabled() {
        return Err(reject(
            source,
            PreloopLocatedArgumentIngressStageV1::UnifiedCapability,
            PreloopLocatedArgumentIngressErrorV1::UnifiedCallDisabled,
        ));
    }

    let argument_values = {
        let input = source.association().input();
        drive_call_arguments_v1(builder, ordinary, input.arguments())
    };
    let argument_values = match argument_values {
        Ok(values) => values,
        Err(detail) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::ArgumentDescent,
                PreloopLocatedArgumentIngressErrorV1::ArgumentDescent {
                    detail: detail.into_boxed_str(),
                },
            ))
        }
    };
    let physical = {
        let input = source.association().input();
        emit_standard_value_terminal_with_receipt_v1(
            builder,
            ordinary,
            receiver,
            input.method().to_string(),
            argument_values,
        )
    };
    let physical = match physical {
        Ok(physical) => physical,
        Err(UnifiedValueCallReceiptErrorV1::UnifiedDisabled) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::UnifiedCapability,
                PreloopLocatedArgumentIngressErrorV1::UnifiedCallDisabled,
            ))
        }
        Err(cause) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::UnifiedTerminal,
                PreloopLocatedArgumentIngressErrorV1::PhysicalReceipt(cause),
            ))
        }
    };

    Ok(ReachedPreloopNestedPhysicalCallV1::prepare(
        source, physical,
    ))
}

fn reject_alternate_member<'site, 'view, 'catalog>(
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    observed: PreloopObservedMemberRouteV1,
) -> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    reject(
        source,
        PreloopLocatedArgumentIngressStageV1::MemberRoute,
        PreloopLocatedArgumentIngressErrorV1::AlternateMemberRoute(observed),
    )
}

fn reject<'site, 'view, 'catalog>(
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedArgumentIngressStageV1,
    cause: PreloopLocatedArgumentIngressErrorV1,
) -> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    RejectedPreloopLocatedArgumentIngressV1::Source {
        source,
        stage,
        cause,
    }
}

fn observe_me_route(prepared: PreparedMeCallExecutionV1) -> PreloopObservedMeRouteV1 {
    match prepared {
        PreparedMeCallExecutionV1::InlineRecord { .. } => PreloopObservedMeRouteV1::InlineRecord,
        PreparedMeCallExecutionV1::InlineSetter { .. } => PreloopObservedMeRouteV1::InlineSetter,
        PreparedMeCallExecutionV1::LoweredGlobal { .. } => PreloopObservedMeRouteV1::LoweredGlobal,
        PreparedMeCallExecutionV1::Standard { prepared, .. } => match prepared {
            PreparedStandardMethodExecutionV1::WeakLoad => {
                PreloopObservedMeRouteV1::StandardWeakLoad
            }
            PreparedStandardMethodExecutionV1::UpgradeRejected => {
                PreloopObservedMeRouteV1::StandardUpgradeRejected
            }
            PreparedStandardMethodExecutionV1::RecordHelper(_) => {
                PreloopObservedMeRouteV1::StandardRecordHelper
            }
            PreparedStandardMethodExecutionV1::Setter(_) => {
                PreloopObservedMeRouteV1::StandardSetter
            }
            PreparedStandardMethodExecutionV1::Unified => PreloopObservedMeRouteV1::StandardUnified,
        },
        PreparedMeCallExecutionV1::StaticFallback { .. } => {
            PreloopObservedMeRouteV1::StaticFallback
        }
        PreparedMeCallExecutionV1::NotApplicable => PreloopObservedMeRouteV1::NotApplicable,
    }
}
