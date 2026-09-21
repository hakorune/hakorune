//! Caller-zero source projection for the bounded direct LoopBreak shape.
//!
//! This observer keeps only resolver-issued source sites and the paired exit
//! record.  It does not issue a Recipe, select a route, or touch Builder state.

use std::collections::BTreeSet;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1, VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::{LocatedBodyV1, LocatedStmtV1};
use super::loop_cond_break_continue_projection::{
    issue_loop_cond_break_continue_source_forest_projection_v1,
    LoopCondBreakContinueForestProjectionRejectV1,
};
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopBreakSourceProjectionRejectV1 {
    ForeignOwner,
    Forest(LoopCondBreakContinueForestProjectionRejectV1),
    DuplicateSite,
    OutOfRoot,
    SourceLookup,
    SourceNavigation,
    ScopeBox,
    BodyInventory(LoopBreakSourceBodyInventoryRejectV1),
    BodyArity,
    BreakIfShape,
    BreakIfBodyArity,
    BreakIfElseBody,
    CarrierUpdateShape,
    StepShape,
    ExitResolution,
    ExitTargetMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopBreakSourceBodyInventoryRejectV1 {
    ForeignOwner,
    SourceNavigation,
    UnsupportedBodyChild,
    DuplicateSite,
}

/// Ordered source statement roles owned by one LoopBreak root.
///
/// This is deliberately a source-only product. It records exact sites and
/// nested-loop membership for the next Recipe slice, without minting any
/// Recipe or physical identifiers.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBreakSourceBodyInventoryV1 {
    owner: FunctionOwnerIdV1,
    root: SourceStmtSiteV1,
    statements: Box<[SourceStmtSiteV1]>,
    nested_loops: Box<[SourceStmtSiteV1]>,
}

impl VerifiedLoopBreakSourceBodyInventoryV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn root(&self) -> &SourceStmtSiteV1 {
        &self.root
    }

    pub(crate) fn statements(&self) -> &[SourceStmtSiteV1] {
        &self.statements
    }

    pub(crate) fn nested_loops(&self) -> &[SourceStmtSiteV1] {
        &self.nested_loops
    }
}

/// Source-only proof for the direct three-statement LoopBreak profile.
///
/// The three body sites are retained separately so a later Facts owner can
/// co-seal them with its existing `LoopBreakSourceTopologyV1` instead of
/// rebuilding source identity from flattened AST coordinates.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBreakSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_condition_site: SourceExprSiteV1,
    break_if_site: SourceStmtSiteV1,
    break_condition_site: SourceExprSiteV1,
    break_exit_site: SourceStmtSiteV1,
    break_exit: ResolvedExitRecordV1,
    forest: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    body_inventory: VerifiedLoopBreakSourceBodyInventoryV1,
    carrier_update_site: SourceStmtSiteV1,
    step_site: SourceStmtSiteV1,
    root_frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
}

impl VerifiedLoopBreakSourceProjectionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn loop_condition_site(&self) -> &SourceExprSiteV1 {
        &self.loop_condition_site
    }

    pub(crate) fn break_if_site(&self) -> &SourceStmtSiteV1 {
        &self.break_if_site
    }

    pub(crate) fn break_condition_site(&self) -> &SourceExprSiteV1 {
        &self.break_condition_site
    }

    pub(crate) fn break_exit_site(&self) -> &SourceStmtSiteV1 {
        &self.break_exit_site
    }

    pub(crate) const fn break_exit(&self) -> ResolvedExitRecordV1 {
        self.break_exit
    }

    pub(crate) fn forest(&self) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.forest
    }

    pub(crate) fn body_inventory(&self) -> &VerifiedLoopBreakSourceBodyInventoryV1 {
        &self.body_inventory
    }

    pub(crate) fn carrier_update_site(&self) -> &SourceStmtSiteV1 {
        &self.carrier_update_site
    }

    pub(crate) fn step_site(&self) -> &SourceStmtSiteV1 {
        &self.step_site
    }

    pub(crate) const fn root_frame_key(
        &self,
    ) -> &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.function_origin == function_origin
            && self.source_kind == source_kind
            && &self.loop_site == site
    }
}

pub(crate) fn issue_loop_break_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedLoopBreakSourceProjectionV1, LoopBreakSourceProjectionRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(LoopBreakSourceProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    verify_source_identity(function, loop_stmt, &resolved_source)?;
    let forest = issue_loop_cond_break_continue_source_forest_projection_v1(input, loop_stmt)
        .map_err(LoopBreakSourceProjectionRejectV1::Forest)?;
    let source = input.source();
    let loop_condition = source
        .child_expr_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    let loop_body = source
        .child_body_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if loop_body
        .statements()
        .iter()
        .any(|stmt| matches!(stmt, ASTNode::ScopeBox { .. }))
    {
        return Err(LoopBreakSourceProjectionRejectV1::ScopeBox);
    }
    let body_inventory = issue_loop_break_source_body_inventory_v1(input, loop_stmt, &loop_body)
        .map_err(LoopBreakSourceProjectionRejectV1::BodyInventory)?;
    if loop_body.statements().len() != 3 {
        return Err(LoopBreakSourceProjectionRejectV1::BodyArity);
    }

    let break_if = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    let ASTNode::If { else_body, .. } = break_if.node() else {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfShape);
    };
    if else_body.is_some() {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfElseBody);
    }
    let then_body = source
        .child_body_from_stmt(
            &break_if,
            crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if then_body.statements().len() != 1 {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfBodyArity);
    }
    let break_exit = source
        .body_stmt(&then_body, 0)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(break_exit.node(), ASTNode::Break { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfShape);
    }
    function
        .if_region_bundle(break_if.site())
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceLookup)?;
    let break_condition = source
        .child_expr_from_stmt(
            &break_if,
            crate::mir::resolved_semantics::ExprChildRoleV1::IfCondition,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;

    let carrier_update = source
        .body_stmt(&loop_body, 1)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(carrier_update.node(), ASTNode::Assignment { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::CarrierUpdateShape);
    }
    let step = source
        .body_stmt(&loop_body, 2)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(step.node(), ASTNode::Assignment { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::StepShape);
    }
    let statement_sites = [
        loop_stmt.site(),
        break_if.site(),
        break_exit.site(),
        carrier_update.site(),
        step.site(),
    ];
    validate_direct_statement_sites(loop_stmt.site(), &statement_sites)?;

    let loop_region = function
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceLookup)?
        .loop_pair()
        .region();
    let exit_record = function
        .resolved_exit(&ResolvedExitSiteV1::Statement(break_exit.site().clone()))
        .ok_or(LoopBreakSourceProjectionRejectV1::ExitResolution)?;
    if exit_record.origin() != ResolvedExitOriginV1::ExplicitBreak
        || exit_record.transfer()
            != (ResolvedControlTransferV1::Break {
                target_loop: loop_region,
            })
    {
        return Err(LoopBreakSourceProjectionRejectV1::ExitTargetMismatch);
    }
    if !forest
        .exits()
        .iter()
        .any(|exit| exit.site() == break_exit.site())
    {
        return Err(LoopBreakSourceProjectionRejectV1::ExitResolution);
    }

    Ok(VerifiedLoopBreakSourceProjectionV1 {
        owner: input.owner(),
        function_origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_condition_site: loop_condition.site().clone(),
        break_if_site: break_if.site().clone(),
        break_condition_site: break_condition.site().clone(),
        break_exit_site: break_exit.site().clone(),
        break_exit: *exit_record,
        forest,
        body_inventory,
        carrier_update_site: carrier_update.site().clone(),
        step_site: step.site().clone(),
        root_frame_key: resolved_source.frame_key(),
    })
}

pub(crate) fn issue_loop_break_source_body_inventory_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &LocatedStmtV1<'_>,
    body: &LocatedBodyV1<'_>,
) -> Result<VerifiedLoopBreakSourceBodyInventoryV1, LoopBreakSourceBodyInventoryRejectV1> {
    if input.owner() != root.owner() || input.owner() != body.site().owner() {
        return Err(LoopBreakSourceBodyInventoryRejectV1::ForeignOwner);
    }
    let mut statements = Vec::new();
    let mut nested_loops = Vec::new();
    let mut seen = BTreeSet::new();
    collect_loop_break_body_roles(
        input,
        root.site(),
        body,
        &mut statements,
        &mut nested_loops,
        &mut seen,
    )?;
    Ok(VerifiedLoopBreakSourceBodyInventoryV1 {
        owner: input.owner(),
        root: root.site().clone(),
        statements: statements.into_boxed_slice(),
        nested_loops: nested_loops.into_boxed_slice(),
    })
}

fn collect_loop_break_body_roles(
    input: ResolvedFunctionLoweringInputV1<'_>,
    root: &SourceStmtSiteV1,
    body: &LocatedBodyV1<'_>,
    statements: &mut Vec<SourceStmtSiteV1>,
    nested_loops: &mut Vec<SourceStmtSiteV1>,
    seen: &mut BTreeSet<SourceStmtSiteV1>,
) -> Result<(), LoopBreakSourceBodyInventoryRejectV1> {
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(body, index)
            .map_err(|_| LoopBreakSourceBodyInventoryRejectV1::SourceNavigation)?;
        if statement.owner() != input.owner()
            || !statement
                .site()
                .node()
                .segments()
                .starts_with(root.node().segments())
        {
            return Err(LoopBreakSourceBodyInventoryRejectV1::ForeignOwner);
        }
        if !seen.insert(statement.site().clone()) {
            return Err(LoopBreakSourceBodyInventoryRejectV1::DuplicateSite);
        }
        statements.push(statement.site().clone());
        match statement.node() {
            ASTNode::If { else_body, .. } => {
                let then_body = input
                    .source()
                    .child_body_from_stmt(
                        &statement,
                        crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
                    )
                    .map_err(|_| LoopBreakSourceBodyInventoryRejectV1::SourceNavigation)?;
                collect_loop_break_body_roles(
                    input,
                    root,
                    &then_body,
                    statements,
                    nested_loops,
                    seen,
                )?;
                if else_body.is_some() {
                    let else_body = input
                        .source()
                        .child_body_from_stmt(
                            &statement,
                            crate::mir::resolved_semantics::BodyChildRoleV1::IfElse,
                        )
                        .map_err(|_| LoopBreakSourceBodyInventoryRejectV1::SourceNavigation)?;
                    collect_loop_break_body_roles(
                        input,
                        root,
                        &else_body,
                        statements,
                        nested_loops,
                        seen,
                    )?;
                }
            }
            ASTNode::Loop { .. } => {
                nested_loops.push(statement.site().clone());
                let nested_body = input
                    .source()
                    .child_body_from_stmt(
                        &statement,
                        crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
                    )
                    .map_err(|_| LoopBreakSourceBodyInventoryRejectV1::SourceNavigation)?;
                collect_loop_break_body_roles(
                    input,
                    root,
                    &nested_body,
                    statements,
                    nested_loops,
                    seen,
                )?;
            }
            ASTNode::LoopRange { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::TryCatch { .. } => {
                return Err(LoopBreakSourceBodyInventoryRejectV1::UnsupportedBodyChild);
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_source_identity(
    function: &VerifiedResolvedFunctionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    source: &VerifiedResolvedLoopSourceV1,
) -> Result<(), LoopBreakSourceProjectionRejectV1> {
    if source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        Ok(())
    } else {
        Err(LoopBreakSourceProjectionRejectV1::SourceLookup)
    }
}

fn validate_direct_statement_sites(
    root: &SourceStmtSiteV1,
    sites: &[&SourceStmtSiteV1],
) -> Result<(), LoopBreakSourceProjectionRejectV1> {
    for (index, site) in sites.iter().enumerate() {
        if !site.node().segments().starts_with(root.node().segments()) {
            return Err(LoopBreakSourceProjectionRejectV1::OutOfRoot);
        }
        if sites[..index].iter().any(|previous| previous == site) {
            return Err(LoopBreakSourceProjectionRejectV1::DuplicateSite);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{issue_loop_break_source_projection_v1, LoopBreakSourceProjectionRejectV1};
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_forest_projection_v1;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1};

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn assignment(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn direct_loop(scope_boxed: bool, explicit_else: bool) -> ASTNode {
        let statements = vec![
            ASTNode::If {
                condition: Box::new(variable("stop")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: explicit_else.then_some(Vec::new()),
                span: Span::unknown(),
            },
            assignment(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("sum")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            ),
            assignment(
                "i",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            ),
        ];
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: statements,
                span: Span::unknown(),
            }]
        } else {
            statements
        };
        ASTNode::FunctionDeclaration {
            name: "loop_break_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into(), "sum".into(), "stop".into()],
                    initial_values: vec![
                        Some(Box::new(integer(0))),
                        Some(Box::new(integer(0))),
                        Some(Box::new(ASTNode::Literal {
                            value: LiteralValue::Bool(true),
                            span: Span::unknown(),
                        })),
                    ],
                    declared_type_names: vec![None, None, None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(3)),
                        span: Span::unknown(),
                    }),
                    body,
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
    fn direct_three_statement_loop_break_projection_is_source_bound() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, false))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        let projection = issue_loop_break_source_projection_v1(input, &loop_stmt, resolved)
            .expect("direct LoopBreak projection");
        assert_eq!(projection.owner(), input.owner());
        assert_eq!(projection.forest().member_sites().len(), 1);
        assert_eq!(projection.forest().exits().len(), 1);
        assert_eq!(projection.body_inventory().owner(), input.owner());
        assert_eq!(projection.body_inventory().root(), loop_stmt.site());
        assert_eq!(projection.body_inventory().statements().len(), 4);
        assert!(projection.body_inventory().nested_loops().is_empty());
        assert_eq!(projection.break_if_site().node().segments().len(), 2);
        assert_eq!(projection.carrier_update_site().node().segments().len(), 2);
        assert_eq!(projection.step_site().node().segments().len(), 2);
        assert_eq!(
            projection.break_exit().origin(),
            crate::mir::resolved_semantics::ResolvedExitOriginV1::ExplicitBreak
        );
    }

    #[test]
    fn scope_box_loop_break_is_rejected_before_projection() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(true, false))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        assert_eq!(
            issue_loop_break_source_projection_v1(input, &loop_stmt, resolved),
            Err(LoopBreakSourceProjectionRejectV1::ScopeBox)
        );
    }

    #[test]
    fn explicit_else_is_rejected_for_direct_break_if() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, true))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        assert_eq!(
            issue_loop_break_source_projection_v1(input, &loop_stmt, resolved),
            Err(LoopBreakSourceProjectionRejectV1::BreakIfElseBody)
        );
    }

    #[test]
    fn loop_break_projection_rejects_foreign_owner_before_forest_issue() {
        let first = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, false))
            .expect("first resolved fixture");
        let second = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, false))
            .expect("second resolved fixture");
        let input = first.root_function_input().expect("first input");
        let foreign_input = second.root_function_input().expect("second input");
        let body = foreign_input.source().root_body().expect("foreign body");
        let foreign_loop = foreign_input
            .source()
            .body_stmt(&body, 1)
            .expect("foreign loop");
        let foreign_source = foreign_input
            .function()
            .resolved_loop_source(foreign_loop.site())
            .expect("foreign source");
        assert_eq!(
            issue_loop_break_source_projection_v1(input, &foreign_loop, foreign_source),
            Err(LoopBreakSourceProjectionRejectV1::ForeignOwner)
        );
    }

    #[test]
    fn loop_break_forest_guard_rejects_missing_root_before_shape_reads() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, false))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let local = input.source().body_stmt(&body, 0).expect("local statement");
        assert_eq!(
            issue_loop_cond_break_continue_source_forest_projection_v1(input, &local),
            Err(super::super::loop_cond_break_continue_projection::
                LoopCondBreakContinueForestProjectionRejectV1::ForestLookup)
        );
    }

    fn statement_site(segments: Vec<SourcePathSegmentV1>) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
    }

    #[test]
    fn direct_statement_relation_rejects_duplicate_and_out_of_root_sites() {
        let root = statement_site(vec![SourcePathSegmentV1::Body(0)]);
        let child = statement_site(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::LoopBody(0),
        ]);
        let outside = statement_site(vec![SourcePathSegmentV1::Body(1)]);
        assert_eq!(
            super::validate_direct_statement_sites(&root, &[&root, &child, &child]),
            Err(LoopBreakSourceProjectionRejectV1::DuplicateSite)
        );
        assert_eq!(
            super::validate_direct_statement_sites(&root, &[&root, &outside]),
            Err(LoopBreakSourceProjectionRejectV1::OutOfRoot)
        );
    }
}
