//! Candidate-only completion for the exact located outer static MethodCall.
//!
//! The outer syntax comes from the catalog-backed source view. This box calls
//! the existing member planner once, accepts only `StaticReceiver`, and then
//! reuses the one static handler and ordered argument driver. It creates no
//! outer physical Call receipt and publishes no type fact.

use crate::ast::ASTNode;
use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::recursive_child_lowering::RawFunctionHeaderLookupPortV1;
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::{MirBuilder, ValueId};

use super::member_route::MemberCallRoutePlan;
use super::method_call_descent::MethodCallArgumentDescentV1;
use super::method_call_terminal::StaticMethodCallCompletionV1;
use super::preloop_located_argument_ingress::{
    PreloopLocatedArgumentIngressErrorV1, PreloopLocatedArgumentIngressStageV1,
    RejectedPreloopLocatedArgumentIngressV1,
};
use super::preloop_located_argument_port::PreloopLocatedArgumentPortV1;
use super::preloop_nested_result_receipt::ReachedPreloopOuterPhysicalCallV1;
use super::{drive_call_arguments_v1, lower_call_argument_v1, CallArgumentDescentPortV1};

#[derive(Debug)]
struct VerifiedPreloopLocatedOuterSyntaxProjectionV1<'catalog> {
    receiver: &'catalog ASTNode,
    method: &'catalog str,
    arguments: &'catalog [ASTNode],
    _seal: VerifiedPreloopLocatedOuterSyntaxProjectionSealV1,
}

#[derive(Debug)]
struct VerifiedPreloopLocatedOuterSyntaxProjectionSealV1;

impl<'catalog> VerifiedPreloopLocatedOuterSyntaxProjectionV1<'catalog> {
    fn from_source<'site, 'view>(
        source: &PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    ) -> Self {
        let outer = source.selected().parent();
        Self {
            receiver: outer.receiver(),
            method: outer.method(),
            arguments: outer.arguments(),
            _seal: VerifiedPreloopLocatedOuterSyntaxProjectionSealV1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopLocatedOuterCompletionStageV1 {
    RoutePreparation,
    RouteSelection,
    StaticCompletion,
    SelectedCompletion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopLocatedOuterObservedRouteV1 {
    EnvMethod,
    ReceiverNormalized,
    Standard,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PreloopLocatedOuterCompletionErrorV1 {
    RoutePreparation { detail: Box<str> },
    AlternateRoute(PreloopLocatedOuterObservedRouteV1),
    StaticCompletion { detail: Box<str> },
    SelectedCompletion { detail: Box<str> },
}

#[derive(Debug)]
enum RetainedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog> {
    Source(PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>),
    Ingress(RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>),
}

#[derive(Debug)]
pub(super) struct RejectedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog> {
    owner: RetainedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedOuterCompletionStageV1,
    cause: PreloopLocatedOuterCompletionErrorV1,
}

impl RejectedPreloopLocatedOuterCompletionV1<'_, '_, '_> {
    pub(super) const fn stage(&self) -> PreloopLocatedOuterCompletionStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> &PreloopLocatedOuterCompletionErrorV1 {
        &self.cause
    }

    pub(super) fn bounded_report(&self) -> String {
        format!("[preloop-located-outer/{:?}] {:?}", self.stage, self.cause)
    }

    pub(super) fn discard(self) {
        match self.owner {
            RetainedPreloopLocatedOuterCompletionV1::Source(source) => source.discard(),
            RetainedPreloopLocatedOuterCompletionV1::Ingress(rejected) => rejected.discard(),
        }
    }
}

/// Exact inner source/physical authority plus the successful containing
/// physical Call receipt.
#[derive(Debug)]
pub(super) struct CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog> {
    physical: ReachedPreloopOuterPhysicalCallV1<'site, 'view, 'catalog>,
    _seal: CompletedPreloopLocatedOuterRequestSealV1,
}

#[derive(Debug)]
struct CompletedPreloopLocatedOuterRequestSealV1;

impl<'site, 'view, 'catalog>
    CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>
{
    pub(super) const fn inner_destination(&self) -> ValueId {
        self.physical.inner().final_destination()
    }

    pub(super) fn caller(
        &self,
    ) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        self.physical.inner().caller()
    }

    pub(super) fn outer_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.physical.inner().outer_site()
    }

    pub(super) const fn selected_index(&self) -> u32 {
        self.physical.inner().selected_index()
    }

    pub(super) fn inner_site(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
        self.physical.inner().selected_site()
    }

    pub(super) const fn outer_destination(&self) -> ValueId {
        self.physical.outer_destination()
    }

    pub(super) fn discard(self) {
        self.physical.discard();
    }
}

struct PreloopLocatedStaticCompletionV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    arguments: &'catalog [ASTNode],
    port: PreloopLocatedArgumentPortV1<'site, 'view, 'catalog, Port>,
}

impl<'site, 'view, 'catalog, Port> PreloopLocatedStaticCompletionV1<'site, 'view, 'catalog, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    fn new(
        ordinary: Port,
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        arguments: &'catalog [ASTNode],
    ) -> Self {
        Self {
            arguments,
            port: PreloopLocatedArgumentPortV1::new(ordinary, source),
        }
    }

    fn into_reached(
        self,
    ) -> Result<
        ReachedPreloopOuterPhysicalCallV1<'site, 'view, 'catalog>,
        RejectedPreloopLocatedArgumentIngressV1<'site, 'view, 'catalog>,
    > {
        self.port.into_reached_outer_physical()
    }
}

impl<Port> MethodCallArgumentDescentV1 for PreloopLocatedStaticCompletionV1<'_, '_, '_, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    fn lower_all(&mut self, builder: &mut MirBuilder) -> Result<Vec<ValueId>, String> {
        drive_call_arguments_v1(builder, &mut self.port, self.arguments)
    }

    fn lower_index(&mut self, builder: &mut MirBuilder, index: usize) -> Result<ValueId, String> {
        lower_call_argument_v1(builder, &mut self.port, self.arguments, index)
    }
}

impl<Port> StaticMethodCallCompletionV1 for PreloopLocatedStaticCompletionV1<'_, '_, '_, Port>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    fn finish_static_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.port.finish_outer_static_request_v1(
            builder,
            owner,
            method,
            checked_source_arity,
            arguments,
        )
    }
}

/// Lower one exact located outer call through the existing StaticReceiver
/// policy and return only a requested outer destination.
pub(super) fn complete_preloop_located_outer_request_v1<'site, 'view, 'catalog, Port>(
    builder: &mut MirBuilder,
    ordinary: Port,
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
) -> Result<
    CompletedPreloopLocatedOuterRequestV1<'site, 'view, 'catalog>,
    RejectedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog>,
>
where
    Port: MethodCallLoweringPortV1
        + CallArgumentDescentPortV1<ArgumentsInput = [ASTNode]>
        + RawFunctionHeaderLookupPortV1,
{
    let syntax = VerifiedPreloopLocatedOuterSyntaxProjectionV1::from_source(&source);
    let route = match builder.plan_member_call_route(syntax.receiver, syntax.method) {
        Ok(route) => route,
        Err(detail) => {
            return Err(reject_source(
                source,
                PreloopLocatedOuterCompletionStageV1::RoutePreparation,
                PreloopLocatedOuterCompletionErrorV1::RoutePreparation {
                    detail: detail.into_boxed_str(),
                },
            ))
        }
    };
    let box_name = match route {
        MemberCallRoutePlan::StaticReceiver { box_name } => box_name,
        MemberCallRoutePlan::EnvMethod { .. } => {
            return Err(reject_alternate(
                source,
                PreloopLocatedOuterObservedRouteV1::EnvMethod,
            ))
        }
        MemberCallRoutePlan::ReceiverNormalized { .. } => {
            return Err(reject_alternate(
                source,
                PreloopLocatedOuterObservedRouteV1::ReceiverNormalized,
            ))
        }
        MemberCallRoutePlan::Standard => {
            return Err(reject_alternate(
                source,
                PreloopLocatedOuterObservedRouteV1::Standard,
            ))
        }
    };

    let mut completion = PreloopLocatedStaticCompletionV1::new(ordinary, source, syntax.arguments);
    let outer_destination = match builder.handle_static_method_call_with_descent(
        &box_name,
        syntax.method,
        syntax.arguments,
        &mut completion,
    ) {
        Ok(destination) => destination,
        Err(detail) => {
            let retained = completion.into_reached().map_or_else(
                RetainedPreloopLocatedOuterCompletionV1::Ingress,
                |reached| {
                    RetainedPreloopLocatedOuterCompletionV1::Ingress(
                        RejectedPreloopLocatedArgumentIngressV1::after_outer_physical(
                            reached,
                            PreloopLocatedArgumentIngressStageV1::OuterTerminal,
                            PreloopLocatedArgumentIngressErrorV1::OuterTerminal {
                                detail: detail.clone().into_boxed_str(),
                            },
                        ),
                    )
                },
            );
            return Err(RejectedPreloopLocatedOuterCompletionV1 {
                owner: retained,
                stage: PreloopLocatedOuterCompletionStageV1::StaticCompletion,
                cause: PreloopLocatedOuterCompletionErrorV1::StaticCompletion {
                    detail: detail.into_boxed_str(),
                },
            });
        }
    };

    let physical = completion.into_reached().map_err(|rejected| {
        let detail = rejected.bounded_report();
        RejectedPreloopLocatedOuterCompletionV1 {
            owner: RetainedPreloopLocatedOuterCompletionV1::Ingress(rejected),
            stage: PreloopLocatedOuterCompletionStageV1::SelectedCompletion,
            cause: PreloopLocatedOuterCompletionErrorV1::SelectedCompletion {
                detail: detail.into_boxed_str(),
            },
        }
    })?;
    debug_assert_eq!(outer_destination, physical.outer_destination());
    Ok(CompletedPreloopLocatedOuterRequestV1 {
        physical,
        _seal: CompletedPreloopLocatedOuterRequestSealV1,
    })
}

fn reject_alternate<'site, 'view, 'catalog>(
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    observed: PreloopLocatedOuterObservedRouteV1,
) -> RejectedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog> {
    reject_source(
        source,
        PreloopLocatedOuterCompletionStageV1::RouteSelection,
        PreloopLocatedOuterCompletionErrorV1::AlternateRoute(observed),
    )
}

fn reject_source<'site, 'view, 'catalog>(
    source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
    stage: PreloopLocatedOuterCompletionStageV1,
    cause: PreloopLocatedOuterCompletionErrorV1,
) -> RejectedPreloopLocatedOuterCompletionV1<'site, 'view, 'catalog> {
    RejectedPreloopLocatedOuterCompletionV1 {
        owner: RetainedPreloopLocatedOuterCompletionV1::Source(source),
        stage,
        cause,
    }
}
