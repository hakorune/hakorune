//! Source-only projection for a composite LoopBreak root.
//!
//! This is the same-owner extension selected by the I1 design audit. It
//! retains the resolver-owned loop forest, exit ledger, and ordered body
//! inventory without issuing a Recipe or touching a physical lowering route.
//!
//! Admission requires explicit exit evidence under the root: a root-targeted
//! `Break` or an `ExplicitReturn`/`Return{target_function}` record
//! (`RootExitMissing` otherwise). Return-in-body `loop(cond){…return…}`
//! shares this owner — the composite recipe already emits `ExitKind::Return`
//! for the `ExitAllowed` body contract.

use std::collections::BTreeSet;

use crate::mir::compiler::located::{LocatedStmtV1, SourceBodySiteV1};
use crate::mir::compiler::loop_break_composite_body_role::{
    issue_loop_break_composite_body_role_map_v1, LoopBreakCompositeBodyRoleRejectV1,
    VerifiedLoopBreakCompositeBodyRoleMapV1,
};
use crate::mir::compiler::loop_break_source_projection::{
    issue_loop_break_source_body_inventory_v1, LoopBreakSourceBodyInventoryRejectV1,
    VerifiedLoopBreakSourceBodyInventoryV1,
};
use crate::mir::compiler::loop_cond_break_continue_projection::{
    issue_loop_cond_break_continue_source_forest_projection_v1,
    LoopCondBreakContinueForestProjectionRejectV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, ResolvedControlTransferV1, ResolvedExitOriginV1,
    SemanticOwnerSourceKindV1, SourceExprSiteV1, SourceStmtSiteV1, VerifiedResolvedLoopSourceV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopBreakCompositeSourceProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    Forest(LoopCondBreakContinueForestProjectionRejectV1),
    BodyInventory(LoopBreakSourceBodyInventoryRejectV1),
    BodyRole(LoopBreakCompositeBodyRoleRejectV1),
    DuplicateExit,
    ExitOutsideRoot,
    RootExitMissing,
    RootBreakTargetMismatch,
}

/// One resolver-branded source product for a composite LoopBreak root.
///
/// The body inventory and forest are co-sealed from the same
/// `ResolvedFunctionLoweringInputV1`; consumers must not pair either product
/// with a different root or reconstruct the relation from syntax coordinates.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBreakCompositeSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_condition_site: SourceExprSiteV1,
    loop_body_site: SourceBodySiteV1,
    body_inventory: VerifiedLoopBreakSourceBodyInventoryV1,
    body_roles: VerifiedLoopBreakCompositeBodyRoleMapV1,
    forest: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    root_frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
}

impl VerifiedLoopBreakCompositeSourceProjectionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
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

    pub(crate) fn loop_body_site(&self) -> &SourceBodySiteV1 {
        &self.loop_body_site
    }

    pub(crate) fn body_inventory(&self) -> &VerifiedLoopBreakSourceBodyInventoryV1 {
        &self.body_inventory
    }

    pub(crate) fn body_roles(&self) -> &VerifiedLoopBreakCompositeBodyRoleMapV1 {
        &self.body_roles
    }

    pub(crate) fn forest(&self) -> &VerifiedLoopCondBreakContinueSourceForestProjectionV1 {
        &self.forest
    }
}

pub(crate) fn issue_loop_break_composite_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<
    VerifiedLoopBreakCompositeSourceProjectionV1,
    LoopBreakCompositeSourceProjectionRejectV1,
> {
    if input.owner() != loop_stmt.owner() {
        return Err(LoopBreakCompositeSourceProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    if !resolved_source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        return Err(LoopBreakCompositeSourceProjectionRejectV1::SourceLookup);
    }

    let forest = issue_loop_cond_break_continue_source_forest_projection_v1(input, loop_stmt)
        .map_err(LoopBreakCompositeSourceProjectionRejectV1::Forest)?;
    if forest.owner() != input.owner()
        || forest.members().first().map(|member| member.source().site()) != Some(loop_stmt.site())
    {
        return Err(LoopBreakCompositeSourceProjectionRejectV1::SourceLookup);
    }
    validate_exit_ledger(&forest, loop_stmt, function)?;

    let source = input.source();
    let loop_condition = source
        .child_expr_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
        )
        .map_err(|_| LoopBreakCompositeSourceProjectionRejectV1::SourceNavigation)?;
    let loop_body = source
        .child_body_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .map_err(|_| LoopBreakCompositeSourceProjectionRejectV1::SourceNavigation)?;
    let body_inventory = issue_loop_break_source_body_inventory_v1(input, loop_stmt, &loop_body)
        .map_err(LoopBreakCompositeSourceProjectionRejectV1::BodyInventory)?;
    let body_roles = issue_loop_break_composite_body_role_map_v1(
        input,
        loop_stmt,
        &loop_body,
        &forest,
    )
    .map_err(LoopBreakCompositeSourceProjectionRejectV1::BodyRole)?;

    Ok(VerifiedLoopBreakCompositeSourceProjectionV1 {
        owner: input.owner(),
        function_origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_condition_site: loop_condition.site().clone(),
        loop_body_site: loop_body.site().clone(),
        body_inventory,
        body_roles,
        forest,
        root_frame_key: resolved_source.frame_key(),
    })
}

fn validate_exit_ledger(
    forest: &VerifiedLoopCondBreakContinueSourceForestProjectionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
) -> Result<(), LoopBreakCompositeSourceProjectionRejectV1> {
    let root_segments = loop_stmt.site().node().segments();
    let mut seen = BTreeSet::new();
    let mut root_exit_count = 0usize;
    let root_region = function
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| LoopBreakCompositeSourceProjectionRejectV1::RootBreakTargetMismatch)?
        .loop_pair()
        .region();
    for exit in forest.exits() {
        if !seen.insert(exit.site().clone()) {
            return Err(LoopBreakCompositeSourceProjectionRejectV1::DuplicateExit);
        }
        if !exit.site().node().segments().starts_with(root_segments) {
            return Err(LoopBreakCompositeSourceProjectionRejectV1::ExitOutsideRoot);
        }
        let root_exit = match exit.record().transfer() {
            ResolvedControlTransferV1::Break { target_loop } => {
                exit.record().origin() == ResolvedExitOriginV1::ExplicitBreak
                    && target_loop == root_region
            }
            ResolvedControlTransferV1::Return { .. } => {
                exit.record().origin() == ResolvedExitOriginV1::ExplicitReturn
            }
            ResolvedControlTransferV1::Continue { .. } => false,
        };
        if root_exit {
            root_exit_count += 1;
        }
    }
    if root_exit_count == 0 {
        return Err(LoopBreakCompositeSourceProjectionRejectV1::RootExitMissing);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        issue_loop_break_composite_source_projection_v1,
        LoopBreakCompositeSourceProjectionRejectV1,
    };
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        }
    }

    fn composite_loop() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "composite_loop_break_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into(), "stop".into()],
                    initial_values: vec![
                        Some(Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(0),
                            span: Span::unknown(),
                        })),
                        Some(Box::new(ASTNode::Literal {
                            value: LiteralValue::Bool(true),
                            span: Span::unknown(),
                        })),
                    ],
                    declared_type_names: vec![None, None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    body: vec![
                        ASTNode::If {
                            condition: Box::new(variable("stop")),
                            then_body: vec![ASTNode::Break {
                                span: Span::unknown(),
                            }],
                            else_body: None,
                            span: Span::unknown(),
                        },
                        ASTNode::Loop {
                            condition: Box::new(variable("stop")),
                            body: vec![ASTNode::Continue {
                                span: Span::unknown(),
                            }],
                            span: Span::unknown(),
                        },
                        ASTNode::Assignment {
                            target: Box::new(variable("i")),
                            value: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(variable("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
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

    fn single_structured_loop() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "single_structured_loop_break_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    body: vec![
                        ASTNode::If {
                            condition: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Equal,
                                left: Box::new(variable("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            }),
                            then_body: vec![ASTNode::Break {
                                span: Span::unknown(),
                            }],
                            else_body: None,
                            span: Span::unknown(),
                        },
                        ASTNode::Assignment {
                            target: Box::new(variable("i")),
                            value: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(variable("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
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
    fn composite_projection_retains_nested_loop_and_root_exit() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(composite_loop())
            .expect("resolved composite fixture");
        let input = unit.root_function_input().expect("root input");
        let root_body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&root_body, 1).expect("root loop");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("root loop source");
        let projection = issue_loop_break_composite_source_projection_v1(
            input,
            &loop_stmt,
            resolved,
        )
        .expect("composite source projection");

        assert_eq!(projection.owner(), input.owner());
        assert_eq!(projection.loop_site(), loop_stmt.site());
        assert_eq!(projection.body_inventory().root(), loop_stmt.site());
        assert_eq!(projection.body_inventory().statements().len(), 5);
        assert_eq!(projection.body_inventory().nested_loops().len(), 1);
        assert_eq!(projection.forest().members().len(), 2);
        assert_eq!(projection.forest().exits().len(), 2);
        assert_eq!(projection.body_roles().root().count_loops(), 2);
        assert_eq!(projection.body_roles().root().count_exits(), 2);
        assert_eq!(projection.loop_body_site().owner(), input.owner());
    }

    #[test]
    fn single_structured_projection_is_admitted_by_composite_owner() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(single_structured_loop())
            .expect("resolved single structured fixture");
        let input = unit.root_function_input().expect("root input");
        let root_body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&root_body, 1).expect("root loop");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("root loop source");
        let projection = issue_loop_break_composite_source_projection_v1(
            input,
            &loop_stmt,
            resolved,
        )
        .expect("single structured projection");
        assert_eq!(projection.forest().members().len(), 1);
        assert!(projection.body_roles().root().contains_control());
        crate::mir::builder::issue_composite_source_candidate_v1(
            input,
            &loop_stmt,
            input
                .function()
                .resolved_loop_source(loop_stmt.site())
                .expect("root loop source for candidate"),
        )
        .expect("one-member structured candidate");
    }

    fn return_in_body_loop() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "return_in_body_loop_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    body: vec![
                        ASTNode::If {
                            condition: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Equal,
                                left: Box::new(variable("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            }),
                            then_body: vec![ASTNode::Return {
                                value: Some(Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(0),
                                    span: Span::unknown(),
                                })),
                                span: Span::unknown(),
                            }],
                            else_body: None,
                            span: Span::unknown(),
                        },
                        ASTNode::Assignment {
                            target: Box::new(variable("i")),
                            value: Box::new(ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(variable("i")),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        },
                    ],
                    span: Span::unknown(),
                },
                ASTNode::Return {
                    value: Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    })),
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

    fn exit_free_loop() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "exit_free_loop_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into()],
                    initial_values: vec![Some(Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }))],
                    declared_type_names: vec![None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(3),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    body: vec![ASTNode::Assignment {
                        target: Box::new(variable("i")),
                        value: Box::new(ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left: Box::new(variable("i")),
                            right: Box::new(ASTNode::Literal {
                                value: LiteralValue::Integer(1),
                                span: Span::unknown(),
                            }),
                            span: Span::unknown(),
                        }),
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
    fn return_in_body_projection_is_admitted_by_composite_owner() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(return_in_body_loop())
            .expect("resolved return-in-body fixture");
        let input = unit.root_function_input().expect("root input");
        let root_body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&root_body, 1).expect("root loop");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("root loop source");
        let projection =
            issue_loop_break_composite_source_projection_v1(input, &loop_stmt, resolved)
                .expect("return-in-body composite projection");
        assert_eq!(projection.forest().members().len(), 1);
        assert_eq!(projection.forest().exits().len(), 1);
        assert!(projection.body_roles().root().contains_control());
        crate::mir::builder::issue_composite_source_candidate_v1(
            input,
            &loop_stmt,
            input
                .function()
                .resolved_loop_source(loop_stmt.site())
                .expect("root loop source for candidate"),
        )
        .expect("return-in-body composite candidate");
    }

    #[test]
    fn exit_free_loop_declines_with_root_exit_missing() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(exit_free_loop())
            .expect("resolved exit-free fixture");
        let input = unit.root_function_input().expect("root input");
        let root_body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&root_body, 1).expect("root loop");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("root loop source");
        assert_eq!(
            issue_loop_break_composite_source_projection_v1(input, &loop_stmt, resolved),
            Err(LoopBreakCompositeSourceProjectionRejectV1::RootExitMissing)
        );
    }

    #[test]
    fn composite_projection_rejects_foreign_root_before_forest_issue() {
        let first = VerifiedResolvedSourceUnitV1::resolve_function(composite_loop())
            .expect("first resolved composite fixture");
        let second = VerifiedResolvedSourceUnitV1::resolve_function(composite_loop())
            .expect("second resolved composite fixture");
        let input = first.root_function_input().expect("first root input");
        let foreign_input = second.root_function_input().expect("second root input");
        let root_body = foreign_input.source().root_body().expect("foreign root body");
        let foreign_loop = foreign_input
            .source()
            .body_stmt(&root_body, 1)
            .expect("foreign root loop");
        let foreign_source = foreign_input
            .function()
            .resolved_loop_source(foreign_loop.site())
            .expect("foreign loop source");

        assert_eq!(
            issue_loop_break_composite_source_projection_v1(input, &foreign_loop, foreign_source),
            Err(LoopBreakCompositeSourceProjectionRejectV1::ForeignOwner)
        );
    }

}
