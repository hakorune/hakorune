//! Lexical BlockExpr traversal and non-local-exit boundary.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::source_site::{ResolvedExitSiteV1, SourcePathSegmentV1};

use super::path::ShadowSourcePathV0;
use super::product::{ShadowRegionKindV0, ShadowResolveErrorV0, ShadowScopeKindV0};
use super::resolver::ShadowResolverV0;

impl<'ast> ShadowResolverV0<'ast> {
    pub(super) fn resolve_block_expr(
        &mut self,
        prelude: &'ast [ASTNode],
        tail: &'ast ASTNode,
        path: &ShadowSourcePathV0,
    ) -> Result<(), ShadowResolveErrorV0> {
        for (index, statement) in prelude.iter().enumerate() {
            if statement.contains_non_local_exit_outside_loops() {
                return Err(ShadowResolveErrorV0::BlockExprNonLocalExit {
                    site: ResolvedExitSiteV1::Statement(
                        path.child(SourcePathSegmentV1::BlockExprPrelude(index as u32))
                            .stmt(),
                    ),
                });
            }
        }
        let tail_path = path.child(SourcePathSegmentV1::BlockExprTail);
        if tail.contains_non_local_exit_outside_loops() {
            return Err(ShadowResolveErrorV0::BlockExprNonLocalExit {
                site: ResolvedExitSiteV1::Expression(tail_path.expr()),
            });
        }

        let root = path.child(SourcePathSegmentV1::BlockExprPreludeRoot);
        let (region, _) = self.enter_region_scope(
            ShadowRegionKindV0::BlockExpr,
            ShadowScopeKindV0::BlockExpr,
            &root,
        );
        let result = self
            .resolve_body(prelude, |index| {
                path.child(SourcePathSegmentV1::BlockExprPrelude(index as u32))
            })
            .and_then(|()| self.resolve_expr(tail, &tail_path));
        self.leave_region_scope(region);
        result
    }
}
