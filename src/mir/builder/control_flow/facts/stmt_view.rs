//! Analysis-only statement views for Facts (no AST rewrite).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::policies::cond_prelude_vocab::classify_cond_prelude_stmt;
use crate::mir::builder::control_flow::plan::recipe_tree::build_stmt_only_block;
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBlock, RecipeBodies};
use crate::mir::builder::control_flow::recipes::RecipeBody;

/// Opaque original-body coordinate for one flattened analysis statement.
///
/// It deliberately has no AST resolver. Route Facts may retain a coordinate
/// only when their own extractor observed the corresponding analysis item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopSourceBodySiteV1 {
    raw_body_index: u32,
    scope_box_children: Box<[u32]>,
}

impl LoopSourceBodySiteV1 {
    pub(in crate::mir::builder) fn raw_body_index(&self) -> u32 {
        self.raw_body_index
    }

    pub(in crate::mir::builder) fn scope_box_children(&self) -> &[u32] {
        &self.scope_box_children
    }
}

/// Alignment from flattened analysis-body index to original ScopeBox lineage.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(in crate::mir::builder) struct LoopSourceProjectionV1 {
    flattened_body_sites: Option<Box<[LoopSourceBodySiteV1]>>,
}

impl LoopSourceProjectionV1 {
    fn available(sites: Vec<LoopSourceBodySiteV1>) -> Self {
        Self {
            flattened_body_sites: Some(sites.into()),
        }
    }

    pub(in crate::mir::builder) fn site_for_flattened_index(
        &self,
        index: usize,
    ) -> Option<&LoopSourceBodySiteV1> {
        self.flattened_body_sites.as_deref()?.get(index)
    }

    pub(in crate::mir::builder) fn flattened_body_len(&self) -> Option<usize> {
        self.flattened_body_sites.as_ref().map(|sites| sites.len())
    }
}

/// Existing cloned analysis statements coupled with their opaque source sites.
pub(in crate::mir::builder) struct FlattenedScopeBoxBodyV1 {
    statements: Vec<ASTNode>,
    projection: LoopSourceProjectionV1,
}

impl FlattenedScopeBoxBodyV1 {
    pub(in crate::mir::builder) fn into_parts(self) -> (Vec<ASTNode>, LoopSourceProjectionV1) {
        (self.statements, self.projection)
    }
}

/// Flatten ScopeBox nodes into a single sequential statement list.
///
/// This is an analysis-only view: it clones existing AST nodes but does not
/// rewrite or mutate the original AST.
pub(in crate::mir::builder) fn flatten_scope_boxes(body: &[ASTNode]) -> Vec<ASTNode> {
    flatten_scope_boxes_with_projection(body).into_parts().0
}

/// Builds the existing ScopeBox-flattened analysis view and aligned provenance.
pub(in crate::mir::builder) fn flatten_scope_boxes_with_projection(
    body: &[ASTNode],
) -> FlattenedScopeBoxBodyV1 {
    let mut out = Vec::new();
    let mut sites = Vec::new();

    fn strip_stmt(node: &ASTNode) -> ASTNode {
        match node {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => ASTNode::If {
                condition: condition.clone(),
                then_body: flatten_scope_boxes(then_body),
                else_body: else_body.as_ref().map(|body| flatten_scope_boxes(body)),
                span: span.clone(),
            },
            ASTNode::Loop {
                condition,
                body,
                span,
            } => ASTNode::Loop {
                condition: condition.clone(),
                body: flatten_scope_boxes(body),
                span: span.clone(),
            },
            ASTNode::LoopRange {
                var_name,
                start,
                end,
                body,
                span,
            } => ASTNode::LoopRange {
                var_name: var_name.clone(),
                start: start.clone(),
                end: end.clone(),
                body: flatten_scope_boxes(body),
                span: span.clone(),
            },
            ASTNode::Lambda { params, body, span } => ASTNode::Lambda {
                params: params.clone(),
                body: flatten_scope_boxes(body),
                span: span.clone(),
            },
            ASTNode::TryCatch {
                try_body,
                catch_clauses,
                finally_body,
                span,
            } => ASTNode::TryCatch {
                try_body: flatten_scope_boxes(try_body),
                catch_clauses: catch_clauses
                    .iter()
                    .map(|clause| crate::ast::CatchClause {
                        exception_type: clause.exception_type.clone(),
                        variable_name: clause.variable_name.clone(),
                        body: flatten_scope_boxes(&clause.body),
                        span: clause.span,
                    })
                    .collect(),
                finally_body: finally_body.as_ref().map(|body| flatten_scope_boxes(body)),
                span: span.clone(),
            },
            _ => node.clone(),
        }
    }

    fn push_node(
        node: &ASTNode,
        raw_body_index: u32,
        scope_box_children: &mut Vec<u32>,
        out: &mut Vec<ASTNode>,
        sites: &mut Vec<LoopSourceBodySiteV1>,
    ) {
        match node {
            ASTNode::ScopeBox { body, .. } => {
                for (child_index, inner) in body.iter().enumerate() {
                    scope_box_children.push(child_index as u32);
                    push_node(inner, raw_body_index, scope_box_children, out, sites);
                    scope_box_children.pop();
                }
            }
            _ => {
                out.push(strip_stmt(node));
                sites.push(LoopSourceBodySiteV1 {
                    raw_body_index,
                    scope_box_children: scope_box_children.clone().into(),
                });
            }
        }
    }

    for (raw_body_index, stmt) in body.iter().enumerate() {
        push_node(
            stmt,
            raw_body_index as u32,
            &mut Vec::new(),
            &mut out,
            &mut sites,
        );
    }

    FlattenedScopeBoxBodyV1 {
        statements: out,
        projection: LoopSourceProjectionV1::available(sites),
    }
}

// Return-prelude container recipe SSOT is in `facts::return_prelude`.

/// Facts SSOT: Try to build a `RecipeBlock` for a "stmt-only effects" block.
///
/// Contract (v1):
/// - Every statement must be in `cond_prelude_vocab`.
/// - No statement may contain non-local exits (`ASTNode::contains_non_local_exit()`).
/// - Empty blocks are rejected (returns `None`).
///
/// Notes:
/// - This does not rewrite the AST. It registers cloned statements into a fresh arena.
/// - Callers should pass in the exact statement list they want to represent; this function
///   intentionally does not flatten `ScopeBox` wrappers to avoid accidental acceptance drift.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct StmtOnlyBlockRecipe {
    pub arena: RecipeBodies,
    pub block: RecipeBlock,
}

pub(in crate::mir::builder) fn try_build_stmt_only_block_recipe(
    stmts: &[ASTNode],
) -> Option<StmtOnlyBlockRecipe> {
    if stmts.is_empty() {
        return None;
    }

    for stmt in stmts {
        if stmt.contains_non_local_exit() {
            return None;
        }
        if classify_cond_prelude_stmt(stmt).is_none() {
            return None;
        }
    }

    let stmt_count = stmts.len();
    let mut arena = RecipeBodies::new();
    let body_id = arena.register(RecipeBody::new(stmts.to_vec()));
    let block = build_stmt_only_block(body_id, stmt_count);
    Some(StmtOnlyBlockRecipe { arena, block })
}

#[cfg(test)]
mod tests {
    use super::try_build_stmt_only_block_recipe;
    use super::{flatten_scope_boxes, flatten_scope_boxes_with_projection};
    use crate::ast::{ASTNode, Span};

    #[test]
    fn flatten_scope_boxes_expands_nested_scopes() {
        let span = Span::unknown();
        let body = vec![
            ASTNode::ScopeBox {
                body: vec![ASTNode::Break { span }, ASTNode::Continue { span }],
                span,
            },
            ASTNode::Return { value: None, span },
        ];

        let flat = flatten_scope_boxes(&body);
        assert_eq!(flat.len(), 3);
        assert!(matches!(flat[0], ASTNode::Break { .. }));
        assert!(matches!(flat[1], ASTNode::Continue { .. }));
        assert!(matches!(flat[2], ASTNode::Return { .. }));
    }

    #[test]
    fn flatten_projection_retains_nested_scope_box_lineage() {
        let span = Span::unknown();
        let body = vec![
            ASTNode::ScopeBox {
                body: vec![
                    ASTNode::ScopeBox {
                        body: vec![ASTNode::Break { span }],
                        span,
                    },
                    ASTNode::Continue { span },
                ],
                span,
            },
            ASTNode::Return { value: None, span },
        ];

        let (_, projection) = flatten_scope_boxes_with_projection(&body).into_parts();

        assert_eq!(projection.flattened_body_len(), Some(3));
        assert_eq!(
            projection
                .site_for_flattened_index(0)
                .unwrap()
                .raw_body_index(),
            0
        );
        assert_eq!(
            projection
                .site_for_flattened_index(0)
                .unwrap()
                .scope_box_children(),
            &[0, 0]
        );
        assert_eq!(
            projection
                .site_for_flattened_index(1)
                .unwrap()
                .scope_box_children(),
            &[1]
        );
        assert_eq!(
            projection
                .site_for_flattened_index(2)
                .unwrap()
                .raw_body_index(),
            1
        );
        assert!(projection
            .site_for_flattened_index(2)
            .unwrap()
            .scope_box_children()
            .is_empty());
    }

    #[test]
    fn flatten_scope_boxes_strips_if_branch_wrappers() {
        let span = Span::unknown();
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Literal {
                value: crate::ast::LiteralValue::Bool(true),
                span,
            }),
            then_body: vec![ASTNode::ScopeBox {
                body: vec![ASTNode::Break { span }],
                span,
            }],
            else_body: None,
            span,
        }];

        let flat = flatten_scope_boxes(&body);
        match &flat[0] {
            ASTNode::If { then_body, .. } => {
                assert!(matches!(then_body[0], ASTNode::Break { .. }));
            }
            _ => panic!("expected If after flatten"),
        }
    }

    #[test]
    fn try_build_stmt_only_block_recipe_accepts_local_print() {
        let span = Span::unknown();
        let stmts = vec![
            ASTNode::Local {
                variables: vec!["x".to_string()],
                initial_values: vec![None],
                declared_type_names: Vec::new(),
                span,
            },
            ASTNode::Print {
                expression: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span,
                }),
                span,
            },
        ];

        assert!(try_build_stmt_only_block_recipe(&stmts).is_some());
    }

    #[test]
    fn try_build_stmt_only_block_recipe_rejects_exit() {
        let span = Span::unknown();
        let stmts = vec![ASTNode::Break { span }];
        assert!(try_build_stmt_only_block_recipe(&stmts).is_none());
    }
}
