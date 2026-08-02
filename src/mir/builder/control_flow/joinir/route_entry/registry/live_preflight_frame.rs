//! One-selection preflight observation with private legacy execution continuation.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::MirBuilder;

use super::{
    live_ordered_terminality::all_route_preflight::observe_selected_preflight_v1,
    try_execute_route_execution_witness, LoopRouteContext, RecipeFirstRouteSelectionV1,
    RouteExecutionWitnessV1, RouterEnv,
};

pub(crate) struct LivePreflightFrameV1<'frame> {
    _condition: &'frame ASTNode,
    _body: &'frame [ASTNode],
    facts: Option<&'frame CanonicalLoopFacts>,
    selection: RecipeFirstRouteSelectionV1,
    env: RouterEnv,
    recipe_contract_present: bool,
    recipe_first_allowed: bool,
}

pub(crate) fn issue_live_preflight_frame<'frame>(
    ctx: &LoopRouteContext<'frame>,
    facts: Option<&'frame CanonicalLoopFacts>,
    selection: RecipeFirstRouteSelectionV1,
    env: RouterEnv,
    recipe_contract_present: bool,
    recipe_first_allowed: bool,
) -> LivePreflightFrameV1<'frame> {
    LivePreflightFrameV1 {
        _condition: ctx.condition,
        _body: ctx.body,
        facts,
        selection,
        env,
        recipe_contract_present,
        recipe_first_allowed,
    }
}

pub(crate) struct PreflightObservationV1<'frame> {
    _disposition: super::loop_preflight::LoopPreflightDispositionV1,
    legacy: LegacyRouteExecutionContinuationV1<'frame>,
}

pub(crate) fn observe_all_route_preflight_v1(
    frame: LivePreflightFrameV1<'_>,
) -> PreflightObservationV1<'_> {
    let disposition =
        observe_selected_preflight_v1(frame.facts, frame.selection.raw_execution_routes());
    PreflightObservationV1 {
        _disposition: disposition,
        legacy: LegacyRouteExecutionContinuationV1 {
            facts: frame.facts,
            selection: frame.selection,
            env: frame.env,
            recipe_contract_present: frame.recipe_contract_present,
            recipe_first_allowed: frame.recipe_first_allowed,
        },
    }
}

impl<'frame> PreflightObservationV1<'frame> {
    pub(crate) fn into_legacy_execution(self) -> LegacyRouteExecutionContinuationV1<'frame> {
        self.legacy
    }
}

pub(crate) struct LegacyRouteExecutionContinuationV1<'frame> {
    facts: Option<&'frame CanonicalLoopFacts>,
    selection: RecipeFirstRouteSelectionV1,
    env: RouterEnv,
    recipe_contract_present: bool,
    recipe_first_allowed: bool,
}

impl<'frame> LegacyRouteExecutionContinuationV1<'frame> {
    pub(crate) fn try_execute_if_allowed(
        self,
        builder: &mut MirBuilder,
        ctx: &LoopRouteContext,
    ) -> Result<Option<super::types::LegacyRouteSuccess>, String> {
        if !self.recipe_first_allowed {
            return Ok(None);
        }
        let witness = RouteExecutionWitnessV1::issue(
            self.selection.raw_execution_routes(),
            &self.env,
            self.recipe_contract_present,
        );
        try_execute_route_execution_witness(builder, ctx, self.facts, witness)
    }
}

impl<'frame> LivePreflightFrameV1<'frame> {
    /// Test-only witness access bound to the exact production frame.
    ///
    /// The witness borrows the frame-owned raw schedule and environment.  A
    /// release-gated frame cannot be bypassed by an observer.
    #[cfg(test)]
    pub(crate) fn test_witness_if_allowed(&self) -> Option<RouteExecutionWitnessV1<'_>> {
        if !self.recipe_first_allowed {
            return None;
        }
        Some(RouteExecutionWitnessV1::issue(
            self.selection.raw_execution_routes(),
            &self.env,
            self.recipe_contract_present,
        ))
    }

    #[cfg(test)]
    pub(crate) fn test_env(&self) -> RouterEnv {
        self.env
    }

    #[cfg(test)]
    pub(crate) fn test_recipe_contract_present(&self) -> bool {
        self.recipe_contract_present
    }

    #[cfg(test)]
    pub(crate) fn test_recipe_first_allowed(&self) -> bool {
        self.recipe_first_allowed
    }

    #[cfg(test)]
    pub(crate) fn test_raw_schedule(&self) -> &[super::route_id::LoopRouteId] {
        self.selection.raw_execution_routes()
    }
}
