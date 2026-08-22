//! Callback-scoped structural view for the existing loop normalizer owner.
//!
//! This is a transport seam only.  It does not expose route decisions,
//! semantic plan authority, execution state, or physical allocation.

use super::route_entry::router::LoopRouteContext;
use crate::mir::builder::normal_callable_loop_handoff::CallableSemanticLoopHandoffPreEffectReceiptV1;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourcePathSegmentV1};

/// One borrowed structural view minted by the existing `cf_loop_joinir_impl`
/// Context owner.  The fields remain private so callers cannot rebuild or
/// re-pair a route context from independent inputs.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopStructuralPortV1<'view> {
    diagnostic_label: &'view str,
    debug: bool,
}

impl CallableLoopStructuralPortV1<'_> {
    /// Diagnostic-only label; it is not a route or semantic identity key.
    pub(in crate::mir::builder) fn diagnostic_label(&self) -> &str {
        self.diagnostic_label
    }

    /// Borrowed debug mode copied from the existing structural owner.
    pub(in crate::mir::builder) const fn debug_enabled(&self) -> bool {
        self.debug
    }
}

/// Lend the existing structural view for exactly one callback scope.
///
/// The higher-ranked callback prevents the borrowed port from becoming a
/// storable source product.  This helper is caller-zero infrastructure until
/// a separate named normalizer consumer is accepted.
pub(in crate::mir::builder) fn with_existing_structural_port<R>(
    ctx: &LoopRouteContext<'_>,
    use_port: impl for<'view> FnOnce(CallableLoopStructuralPortV1<'view>) -> R,
) -> R {
    use_port(CallableLoopStructuralPortV1 {
        diagnostic_label: ctx.func_name,
        debug: ctx.debug,
    })
}

/// Typed rejection for the route-neutral source-bound seed.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopStructuralLeaseRejectV1 {
    ParentNotLocated,
    ConditionNotLocated,
    BodyNotLocated,
    ForeignRootLineage,
    ConditionSiteMismatch,
    BodySiteMismatch,
    PreEffectOwnerMismatch,
    PreEffectSiteMismatch,
}

/// A structural-owner seal with no route or physical meaning.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopRouteNeutralStructuralSeedV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    _seal: CallableLoopStructuralOwnerSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct CallableLoopStructuralOwnerSealV1;

impl CallableLoopRouteNeutralStructuralSeedV1 {
    pub(in crate::mir::builder) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }
}

/// Issue one route-neutral seed from the already co-sealed source relation.
/// The source-facts lease issuer is the only caller; this function never
/// constructs `LoopRouteContext` or runs route classification.
pub(in crate::mir::builder) fn issue_route_neutral_structural_seed(
    owner: FunctionOwnerIdV1,
    parent_source: &RawInvocationSourceContextV1,
    condition_source: &RawInvocationSourceContextV1,
    body_source: &RawInvocationSourceContextV1,
    pre_effect: &CallableSemanticLoopHandoffPreEffectReceiptV1,
) -> Result<CallableLoopRouteNeutralStructuralSeedV1, CallableLoopStructuralLeaseRejectV1> {
    let parent_site = parent_source
        .site()
        .ok_or(CallableLoopStructuralLeaseRejectV1::ParentNotLocated)?;
    let condition_site = condition_source
        .site()
        .ok_or(CallableLoopStructuralLeaseRejectV1::ConditionNotLocated)?;
    let body_site = body_source
        .site()
        .ok_or(CallableLoopStructuralLeaseRejectV1::BodyNotLocated)?;
    if !parent_source.shares_root_lineage(condition_source)
        || !parent_source.shares_root_lineage(body_source)
    {
        return Err(CallableLoopStructuralLeaseRejectV1::ForeignRootLineage);
    }
    if !condition_source.is_exact_loop_condition()
        || !is_direct_child(
            parent_site,
            condition_site,
            SourcePathSegmentV1::LoopCondition,
        )
    {
        return Err(CallableLoopStructuralLeaseRejectV1::ConditionSiteMismatch);
    }
    if !body_source.is_exact_loop_body_root()
        || !is_direct_child(parent_site, body_site, SourcePathSegmentV1::LoopBodyRoot)
    {
        return Err(CallableLoopStructuralLeaseRejectV1::BodySiteMismatch);
    }
    if pre_effect.owner() != owner {
        return Err(CallableLoopStructuralLeaseRejectV1::PreEffectOwnerMismatch);
    }
    if pre_effect.loop_site() != parent_site {
        return Err(CallableLoopStructuralLeaseRejectV1::PreEffectSiteMismatch);
    }
    Ok(CallableLoopRouteNeutralStructuralSeedV1 {
        owner,
        loop_site: parent_site.clone(),
        _seal: CallableLoopStructuralOwnerSealV1,
    })
}

/// Borrowed route-neutral port tied to one private seed.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceBoundStructuralPortV1<'view> {
    seed: &'view CallableLoopRouteNeutralStructuralSeedV1,
}

impl CallableLoopSourceBoundStructuralPortV1<'_> {
    pub(in crate::mir::builder) fn from_seed(
        seed: &CallableLoopRouteNeutralStructuralSeedV1,
    ) -> CallableLoopSourceBoundStructuralPortV1<'_> {
        CallableLoopSourceBoundStructuralPortV1 { seed }
    }

    pub(in crate::mir::builder) fn owner(&self) -> FunctionOwnerIdV1 {
        self.seed.owner()
    }

    pub(in crate::mir::builder) fn loop_site(&self) -> &SourceNodeSiteV1 {
        self.seed.loop_site()
    }
}

fn is_direct_child(
    parent: &SourceNodeSiteV1,
    child: &SourceNodeSiteV1,
    expected: SourcePathSegmentV1,
) -> bool {
    let parent_segments = parent.segments();
    let child_segments = child.segments();
    child_segments.len() == parent_segments.len() + 1
        && child_segments.starts_with(parent_segments)
        && child_segments.last() == Some(&expected)
}

#[cfg(test)]
#[path = "structural_port_tests.rs"]
mod tests;
