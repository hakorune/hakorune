//! Lexical BlockExpr traversal and non-local-exit boundary.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, ResolvedExitSiteV1};

use super::path::ShadowSourcePathV0;
use super::product::{ShadowRegionKindV0, ShadowResolveErrorV0, ShadowScopeKindV0};
use super::resolver::ShadowResolverV0;

impl<'ast> ShadowResolverV0<'ast> {
    pub(super) fn resolve_block_expr(
        &mut self,
        expression: &'ast ASTNode,
        prelude: &'ast [ASTNode],
        tail: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        for (index, statement) in prelude.iter().enumerate() {
            if statement.contains_non_local_exit_outside_loops() {
                return Err(ShadowResolveErrorV0::BlockExprNonLocalExit {
                    site: ResolvedExitSiteV1::Statement(
                        path.child(
                            BodyChildRoleV1::BlockExprPrelude
                                .kind_for(expression)
                                .expect("[freeze:contract][source_path/block_expr_body]")
                                .item_segment(index as u32),
                        )
                        .stmt(),
                    ),
                });
            }
        }
        let tail_path = path.child(
            ExprChildRoleV1::BlockExprTail
                .segment_for(expression)
                .expect("[freeze:contract][source_path/block_expr_tail]"),
        );
        if tail.contains_non_local_exit_outside_loops() {
            return Err(ShadowResolveErrorV0::BlockExprNonLocalExit {
                site: ResolvedExitSiteV1::Expression(tail_path.expr()),
            });
        }

        let body_kind = BodyChildRoleV1::BlockExprPrelude
            .kind_for(expression)
            .expect("[freeze:contract][source_path/block_expr_body]");
        let root = path.child(
            body_kind
                .root_segment()
                .expect("[freeze:contract][source_path/block_expr_root]"),
        );
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::BlockExpr,
            ShadowScopeKindV0::BlockExpr,
            &root,
        );
        let result = self
            .resolve_body(prelude, |index| {
                path.child(body_kind.item_segment(index as u32))
            })
            .and_then(|()| self.resolve_expr(tail, &tail_path));
        self.leave_region_scope(region);
        result
    }
}
