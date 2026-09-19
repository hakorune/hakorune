//! Source-owned route token for the bounded LoopCond handoff.
//!
//! The planner remains the Facts/Recipe authority.  This module only co-seals
//! its already selected route with the resolver forest projection that the
//! callable scope owns.  It does not lower, allocate Builder state, or select
//! a compatibility fallback.

use crate::mir::builder::control_flow::joinir::route_entry::registry::RecipeFirstRouteSelectionV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionOriginV1, FunctionOwnerIdV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceRouteRejectV1 {
    FactsMissing,
    LoopCondFactsMissing,
    RouteNotExclusive { routes: Box<[LoopRouteId]> },
    ProjectionMissing,
    ProjectionOwnerMismatch,
    ProjectionIdentityMismatch,
    SourceItemsMissing,
    SourceItemForeign,
    SourceItemOutsideLoop,
    SourceTargetMissing,
    SourceTargetSiteMismatch,
    SourceTargetMultiple,
    SourceIdentityMissing,
    SourceParentMissing,
}

/// Resolver-owned source item relation retained for the future LoopCond
/// physical port. This copies only source sites and selector metadata; it
/// does not reopen AST dispatch or issue a Recipe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceItemBindingV1 {
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    selector: Box<str>,
    arity: u32,
}

impl CallableLoopSourceItemBindingV1 {
    pub(in crate::mir::builder) fn from_resolved(
        owner: FunctionOwnerIdV1,
        call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if call.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        let argument_sites = call
            .arguments()
            .iter()
            .map(|argument| argument.site().clone())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        if argument_sites.len() != call.arity() as usize {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemForeign);
        }
        Ok(Self {
            call_site: call.site().clone(),
            receiver_site: call.receiver_site().clone(),
            argument_sites,
            result_site: call.result_site().clone(),
            selector: call.selector().into(),
            arity: call.arity(),
        })
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(in crate::mir::builder) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(in crate::mir::builder) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(in crate::mir::builder) fn selector(&self) -> &str {
        &self.selector
    }

    pub(in crate::mir::builder) const fn arity(&self) -> u32 {
        self.arity
    }
}

/// Exact target relation issued by the same invocation's source-target
/// authority. The route token never derives a target from selector text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct CallableLoopSourceTargetRelationV1 {
    call_site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
}

impl CallableLoopSourceTargetRelationV1 {
    pub(in crate::mir::builder) fn new(
        call_site: SourceExprSiteV1,
        target: CanonicalSameModuleCallableKeyV1,
    ) -> Self {
        Self { call_site, target }
    }

    pub(in crate::mir::builder) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir::builder) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }
}

/// One move-only source route token.  The planner outcome and resolver forest
/// are consumed together; no later consumer may pair them by AST shape or
/// source line.
#[derive(Debug)]
pub(in crate::mir::builder) struct CallableLoopSourceRouteTokenV1 {
    owner: FunctionOwnerIdV1,
    parent_site: SourceNodeSiteV1,
    outcome: PlanBuildOutcome,
    selection: RecipeFirstRouteSelectionV1,
    projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    source_items: Box<[CallableLoopSourceItemBindingV1]>,
    source_target: Option<CallableLoopSourceTargetRelationV1>,
}

impl CallableLoopSourceRouteTokenV1 {
    pub(in crate::mir::builder) fn issue(
        owner: FunctionOwnerIdV1,
        parent_site: SourceNodeSiteV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        outcome: PlanBuildOutcome,
        selection: RecipeFirstRouteSelectionV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        let facts = outcome
            .facts
            .as_ref()
            .ok_or(CallableLoopSourceRouteRejectV1::FactsMissing)?;
        if facts.facts.loop_cond_break_continue().is_none() {
            return Err(CallableLoopSourceRouteRejectV1::LoopCondFactsMissing);
        }
        if selection.raw_execution_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.raw_execution_routes().into(),
            });
        }
        let projection = projection.ok_or(CallableLoopSourceRouteRejectV1::ProjectionMissing)?;
        if projection.owner() != owner {
            return Err(CallableLoopSourceRouteRejectV1::ProjectionOwnerMismatch);
        }
        let parent_stmt_site = SourceStmtSiteV1::from_node(parent_site.clone());
        if !projection.matches_source_identity(function_origin, source_kind, &parent_stmt_site) {
            return Err(CallableLoopSourceRouteRejectV1::ProjectionIdentityMismatch);
        }
        Ok(Self {
            owner,
            parent_site,
            outcome,
            selection,
            projection,
            source_items: Box::new([]),
            source_target: None,
        })
    }

    pub(in crate::mir::builder) fn issue_with_source_relations(
        owner: FunctionOwnerIdV1,
        parent_site: SourceNodeSiteV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        outcome: PlanBuildOutcome,
        selection: RecipeFirstRouteSelectionV1,
        projection: Option<VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
        source_items: Box<[CallableLoopSourceItemBindingV1]>,
        source_target: Option<CallableLoopSourceTargetRelationV1>,
    ) -> Result<Self, CallableLoopSourceRouteRejectV1> {
        if source_items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        if source_items
            .iter()
            .any(|item| !is_under_parent(item.call_site().node(), &parent_site))
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemOutsideLoop);
        }
        let source_target =
            source_target.ok_or(CallableLoopSourceRouteRejectV1::SourceTargetMissing)?;
        if !source_items
            .iter()
            .any(|item| item.call_site() == source_target.call_site())
        {
            return Err(CallableLoopSourceRouteRejectV1::SourceTargetSiteMismatch);
        }
        let mut token = Self::issue(
            owner,
            parent_site,
            function_origin,
            source_kind,
            outcome,
            selection,
            projection,
        )?;
        token.source_items = source_items;
        token.source_target = Some(source_target);
        Ok(token)
    }

    /// Collect source item relations from the resolver ledger for one exact
    /// loop root. The target relation is intentionally supplied separately by
    /// the same-invocation target authority.
    pub(in crate::mir::builder) fn source_items_for_loop(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        owner: FunctionOwnerIdV1,
        parent_site: &SourceNodeSiteV1,
    ) -> Result<Box<[CallableLoopSourceItemBindingV1]>, CallableLoopSourceRouteRejectV1> {
        let items = ledger
            .method_calls()
            .filter(|(site, _)| is_under_parent(site.node(), parent_site))
            .map(|(_, call)| CallableLoopSourceItemBindingV1::from_resolved(owner, call))
            .collect::<Result<Vec<_>, _>>()?
            .into_boxed_slice();
        if items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        Ok(items)
    }

    pub(in crate::mir::builder) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn parent_site(&self) -> &SourceNodeSiteV1 {
        &self.parent_site
    }

    pub(in crate::mir::builder) fn outcome(&self) -> &PlanBuildOutcome {
        &self.outcome
    }

    pub(in crate::mir::builder) fn selection(&self) -> &RecipeFirstRouteSelectionV1 {
        &self.selection
    }

    pub(in crate::mir::builder) fn projection(
        &self,
    ) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.projection
    }

    pub(in crate::mir::builder) fn source_items(&self) -> &[CallableLoopSourceItemBindingV1] {
        &self.source_items
    }

    pub(in crate::mir::builder) fn source_target(
        &self,
    ) -> Option<&CallableLoopSourceTargetRelationV1> {
        self.source_target.as_ref()
    }

    /// Move the already co-sealed route product into its sole physical
    /// consumer.  The Facts/Recipe remains owned by `outcome`; callers must
    /// consume this tuple exactly once and may not reconstruct it from AST or
    /// source names.
    pub(in crate::mir::builder) fn into_physical_parts(
        self,
    ) -> Result<
        (
            FunctionOwnerIdV1,
            SourceNodeSiteV1,
            PlanBuildOutcome,
            RecipeFirstRouteSelectionV1,
            VerifiedLoopCondBreakContinueSourceForestProjectionV1,
            Box<[CallableLoopSourceItemBindingV1]>,
            CallableLoopSourceTargetRelationV1,
        ),
        CallableLoopSourceRouteRejectV1,
    > {
        let Self {
            owner,
            parent_site,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
        } = self;
        let source_target =
            source_target.ok_or(CallableLoopSourceRouteRejectV1::SourceTargetMissing)?;
        if source_items.is_empty() {
            return Err(CallableLoopSourceRouteRejectV1::SourceItemsMissing);
        }
        if selection.raw_execution_routes() != [LoopRouteId::LoopCondBreakContinue] {
            return Err(CallableLoopSourceRouteRejectV1::RouteNotExclusive {
                routes: selection.raw_execution_routes().into(),
            });
        }
        Ok((
            owner,
            parent_site,
            outcome,
            selection,
            projection,
            source_items,
            source_target,
        ))
    }
}

fn is_under_parent(site: &SourceNodeSiteV1, parent: &SourceNodeSiteV1) -> bool {
    site.segments().starts_with(parent.segments())
}

#[cfg(test)]
mod tests {
    use super::{CallableLoopSourceRouteRejectV1, CallableLoopSourceRouteTokenV1};
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::select_recipe_first_routes;
    use crate::mir::builder::control_flow::plan::single_planner::{
        self, CallableLoopFactsPlannerInputV1,
    };
    use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
    use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_forest_projection_v1;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    fn route_fixture() -> ASTNode {
        let variable = |name: &str| ASTNode::Variable {
            name: name.into(),
            span: Span::unknown(),
        };
        let integer = |value: i64| ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        };
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(variable("flag")),
            right: Box::new(integer(2)),
            span: Span::unknown(),
        };
        let branch_condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Equal,
            left: Box::new(variable("flag")),
            right: Box::new(integer(1)),
            span: Span::unknown(),
        };
        let increment = ASTNode::Assignment {
            target: Box::new(variable("flag")),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("flag")),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        ASTNode::FunctionDeclaration {
            name: "loop_cond_route_fixture".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["flag".into()],
                    initial_values: vec![Some(Box::new(integer(0)))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(condition),
                    body: vec![ASTNode::If {
                        condition: Box::new(branch_condition),
                        then_body: vec![increment],
                        else_body: Some(vec![ASTNode::Break {
                            span: Span::unknown(),
                        }]),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                },
            ],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn source_loop_cond_route_token_requires_exclusive_registry_route() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(route_fixture())
            .expect("loop-cond fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let root = input.source().body_stmt(&body, 1).expect("root loop");
        let crate::ast::ASTNode::Loop {
            condition, body, ..
        } = root.node()
        else {
            panic!("fixture root must be a loop")
        };
        let policy =
            GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);
        let outcome =
            single_planner::try_build_source_outcome(CallableLoopFactsPlannerInputV1::new(
                condition,
                body,
                policy,
                "route-fixture".into(),
                false,
            ))
            .expect("planner outcome");
        let selection = select_recipe_first_routes(outcome.facts.as_ref());
        assert_eq!(
            selection.raw_execution_routes(),
            [crate::mir::loop_recipe_contract::route_id::LoopRouteId::LoopCondBreakContinue]
        );
        let projection = issue_loop_cond_break_continue_source_forest_projection_v1(input, &root)
            .expect("forest projection");
        let token = CallableLoopSourceRouteTokenV1::issue(
            input.owner(),
            root.site().node().clone(),
            input.function().function_origin(),
            input.function().source_kind(),
            outcome,
            selection,
            Some(projection),
        )
        .expect("exclusive source route token");
        assert_eq!(token.owner(), input.owner());
        assert_eq!(token.parent_site(), &root.site().node().clone());
        assert_eq!(token.projection().member_sites().len(), 1);
        assert_eq!(token.selection().raw_execution_routes().len(), 1);
        assert!(token.outcome().facts.is_some());
    }

    #[test]
    fn source_loop_cond_route_token_rejects_missing_projection() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(route_fixture())
            .expect("loop-cond fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let root = input.source().body_stmt(&body, 1).expect("root loop");
        let crate::ast::ASTNode::Loop {
            condition, body, ..
        } = root.node()
        else {
            panic!("fixture root must be a loop")
        };
        let policy =
            GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);
        let outcome =
            single_planner::try_build_source_outcome(CallableLoopFactsPlannerInputV1::new(
                condition,
                body,
                policy,
                "route-fixture".into(),
                false,
            ))
            .expect("planner outcome");
        let selection = select_recipe_first_routes(outcome.facts.as_ref());
        let reject = CallableLoopSourceRouteTokenV1::issue(
            input.owner(),
            root.site().node().clone(),
            input.function().function_origin(),
            input.function().source_kind(),
            outcome,
            selection,
            None,
        )
        .expect_err("source route must not mint without resolver projection");
        assert_eq!(reject, CallableLoopSourceRouteRejectV1::ProjectionMissing);
    }

    #[test]
    fn source_loop_cond_route_token_rejects_physical_transfer_without_target() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(route_fixture())
            .expect("loop-cond fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let root = input.source().body_stmt(&body, 1).expect("root loop");
        let crate::ast::ASTNode::Loop {
            condition, body, ..
        } = root.node()
        else {
            panic!("fixture root must be a loop")
        };
        let policy =
            GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true);
        let outcome =
            single_planner::try_build_source_outcome(CallableLoopFactsPlannerInputV1::new(
                condition,
                body,
                policy,
                "route-fixture".into(),
                false,
            ))
            .expect("planner outcome");
        let selection = select_recipe_first_routes(outcome.facts.as_ref());
        let projection = issue_loop_cond_break_continue_source_forest_projection_v1(input, &root)
            .expect("forest projection");
        let token = CallableLoopSourceRouteTokenV1::issue(
            input.owner(),
            root.site().node().clone(),
            input.function().function_origin(),
            input.function().source_kind(),
            outcome,
            selection,
            Some(projection),
        )
        .expect("route token before physical transfer");
        let reject = token
            .into_physical_parts()
            .expect_err("physical transfer must require the exact target relation");
        assert_eq!(reject, CallableLoopSourceRouteRejectV1::SourceTargetMissing);
    }

    #[test]
    fn source_loop_item_inventory_rejects_a_loop_without_resolver_method_rows() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(route_fixture())
            .expect("loop-cond fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let root = input.source().body_stmt(&body, 1).expect("root loop");
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .expect("callable ledger");
        let error = CallableLoopSourceRouteTokenV1::source_items_for_loop(
            &ledger,
            input.owner(),
            root.site().node(),
        )
        .expect_err("item inventory must be explicit when no method row exists");
        assert_eq!(error, CallableLoopSourceRouteRejectV1::SourceItemsMissing);
    }
}
