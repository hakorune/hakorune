//! Source-owned route token for the bounded LoopCond handoff.
//!
//! The planner remains the Facts/Recipe authority.  This module only co-seals
//! its already selected route with the resolver forest projection that the
//! callable scope owns.  It does not lower, allocate Builder state, or select
//! a compatibility fallback.

use crate::mir::builder::control_flow::joinir::route_entry::registry::RecipeFirstRouteSelectionV1;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::loop_recipe_contract::route_id::LoopRouteId;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, SemanticOwnerSourceKindV1, SourceNodeSiteV1,
    SourceStmtSiteV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableLoopSourceRouteRejectV1 {
    FactsMissing,
    LoopCondFactsMissing,
    RouteNotExclusive { routes: Box<[LoopRouteId]> },
    ProjectionMissing,
    ProjectionOwnerMismatch,
    ProjectionIdentityMismatch,
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
        })
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
}
