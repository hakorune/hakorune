//! Candidate-only ingress for one selected pre-loop located MethodCall.
//!
//! This box consumes the exact source association, requires the existing
//! `Me -> Standard(Unified)` prepared route, and delegates ordinary child
//! descent and terminal emission to the existing Raw port. The returned value
//! is the terminal-requested destination, not a physical Call receipt. The
//! existing terminal may emit a fixture-owned Call; this box issues no typed
//! receipt and claims no final physical destination.

use crate::ast::ASTNode;
use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::method_call_handlers::{
    prepare_me_call_execution_v1, PreparedMeCallExecutionV1, PreparedStandardMethodExecutionV1,
};
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::{MirBuilder, ValueId};

use super::member_route::MemberCallRoutePlan;
use super::receiver_binding::ReceiverNormalizationPlan;
use super::{drive_call_arguments_v1, CallArgumentDescentPortV1};

#[derive(Debug)]
pub(super) struct ReachedPreloopUnifiedMethodRequestV1<'site, 'view, 'catalog> {
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    requested_destination: ValueId,
    _seal: ReachedPreloopUnifiedMethodRequestSealV1,
}

#[derive(Debug)]
struct ReachedPreloopUnifiedMethodRequestSealV1(());

impl ReachedPreloopUnifiedMethodRequestSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopLocatedArgumentIngressStageV1 {
    MemberRoute,
    MePreparation,
    MeRoute,
    UnifiedCapability,
    ArgumentDescent,
    UnifiedTerminal,
    OuterTerminal,
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
    UnifiedTerminal { detail: Box<str> },
    OuterTerminal { detail: Box<str> },
}

#[derive(Debug)]
pub(super) struct RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedArgumentIngressStageV1,
    cause: PreloopLocatedArgumentIngressErrorV1,
}

impl<'site, 'view, 'catalog> ReachedPreloopUnifiedMethodRequestV1<'site, 'view, 'catalog> {
    fn new(
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        requested_destination: ValueId,
    ) -> Self {
        Self {
            source,
            requested_destination,
            _seal: ReachedPreloopUnifiedMethodRequestSealV1::new(),
        }
    }

    pub(super) const fn selected_index(&self) -> u32 {
        self.source.selected().index()
    }

    pub(super) fn selected_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.source.selected().child().site()
    }

    pub(super) const fn requested_destination(&self) -> ValueId {
        self.requested_destination
    }

    pub(super) fn reject_outer_terminal(
        self,
        detail: String,
    ) -> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
        reject(
            self.source,
            PreloopLocatedArgumentIngressStageV1::OuterTerminal,
            PreloopLocatedArgumentIngressErrorV1::OuterTerminal {
                detail: detail.into_boxed_str(),
            },
        )
    }

    pub(super) fn discard(self) {
        self.source.discard();
    }
}

impl<'site, 'view, 'catalog> RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog> {
    pub(super) const fn selected_index(&self) -> u32 {
        self.source.selected().index()
    }

    pub(super) fn selected_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.source.selected().child().site()
    }

    pub(super) const fn stage(&self) -> PreloopLocatedArgumentIngressStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> &PreloopLocatedArgumentIngressErrorV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> String {
        match &self.cause {
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
            PreloopLocatedArgumentIngressErrorV1::UnifiedTerminal { detail } => {
                format!("[preloop-ingress/unified-terminal] {detail}")
            }
            PreloopLocatedArgumentIngressErrorV1::OuterTerminal { detail } => {
                format!("[preloop-ingress/outer-terminal] {detail}")
            }
        }
    }

    pub(super) fn discard(self) {
        self.source.discard();
    }
}

pub(super) fn lower_selected_preloop_located_argument_v1<'site, 'view, 'catalog, Port>(
    builder: &mut MirBuilder,
    ordinary: &mut Port,
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
) -> Result<
    ReachedPreloopUnifiedMethodRequestV1<'site, 'view, 'catalog>,
    RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>,
>
where
    Port: MethodCallLoweringPortV1 + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>,
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
    let requested_destination = {
        let input = source.association().input();
        ordinary.emit_standard_value_terminal(
            builder,
            receiver,
            input.method().to_string(),
            argument_values,
        )
    };
    let requested_destination = match requested_destination {
        Ok(destination) => destination,
        Err(detail) => {
            return Err(reject(
                source,
                PreloopLocatedArgumentIngressStageV1::UnifiedTerminal,
                PreloopLocatedArgumentIngressErrorV1::UnifiedTerminal {
                    detail: detail.into_boxed_str(),
                },
            ))
        }
    };

    Ok(ReachedPreloopUnifiedMethodRequestV1::new(
        source,
        requested_destination,
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
    RejectedPreloopLocatedArgumentIngressV1 {
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
