//! Test-only bridge witness for one canonical Loop source window.
//!
//! This seam is intentionally owned by `mir`, below the compiler products and
//! resolver products it borrows. It proves only that one canonical source unit
//! can lend paired raw/resolved views through one non-Clone receipt. It does
//! not classify a family or publish a Builder/MIR artifact.

use crate::ast::ASTNode;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SharedLoopSourceWindowRejectV1 {
    ForeignOwner,
    NotLoop,
    SourceNavigation,
    SourceLookup,
    SourceForest,
    ForestEmpty,
    ForestRootMismatch,
    FrameMismatch,
    UnsupportedSourceKind(SemanticOwnerSourceKindV1),
}

/// The sole receipt for one resolver-owned source window. It is deliberately
/// non-`Clone` and non-`Copy`; `with_views` is the only paired-view exit.
#[derive(Debug)]
pub(crate) struct VerifiedSharedLoopSourceWindowV1<'a> {
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    condition: &'a ASTNode,
    body: &'a [ASTNode],
}

/// Raw view: the exact source AST borrowed from the canonical unit. No
/// flattening, reparse, AST rewrite, or route-local identity is allowed here.
#[derive(Debug, Clone)]
pub(crate) struct SharedRawLoopViewV1<'a> {
    owner: FunctionOwnerIdV1,
    loop_site: SourceStmtSiteV1,
    condition: &'a ASTNode,
    body: &'a [ASTNode],
}

impl<'a> SharedRawLoopViewV1<'a> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) const fn condition(&self) -> &'a ASTNode {
        self.condition
    }

    pub(crate) const fn body(&self) -> &'a [ASTNode] {
        self.body
    }
}

/// Resolver view: the same owner/site/frame plus the consumed source forest.
#[derive(Debug)]
pub(crate) struct SharedResolvedLoopViewV1<'a> {
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    forest: VerifiedResolvedLoopSourceForestV1,
}

impl<'a> SharedResolvedLoopViewV1<'a> {
    pub(crate) const fn source_unit(&self) -> &'a VerifiedResolvedSourceUnitV1 {
        self.source_unit
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) fn forest(&self) -> &VerifiedResolvedLoopSourceForestV1 {
        &self.forest
    }
}

impl<'a> VerifiedSharedLoopSourceWindowV1<'a> {
    /// Consume the only receipt and lend both views from the same source unit.
    pub(crate) fn with_views<R>(
        self,
        f: impl FnOnce(SharedRawLoopViewV1<'a>, SharedResolvedLoopViewV1<'a>) -> R,
    ) -> R {
        let Self {
            source_unit,
            owner,
            function_origin,
            source_kind,
            loop_site,
            frame,
            forest,
            condition,
            body,
        } = self;
        f(
            SharedRawLoopViewV1 {
                owner,
                loop_site: loop_site.clone(),
                condition,
                body,
            },
            SharedResolvedLoopViewV1 {
                source_unit,
                owner,
                function_origin,
                source_kind,
                loop_site,
                frame,
                forest,
            },
        )
    }
}

/// Issue one receipt only after exact owner, Loop syntax, forest, and frame
/// validation. The input statement is borrowed from the same canonical source
/// lifetime, but the owner check still rejects a foreign located statement.
pub(crate) fn issue_shared_loop_source_window_v1<'a>(
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    loop_stmt: &LocatedStmtV1<'a>,
) -> Result<VerifiedSharedLoopSourceWindowV1<'a>, SharedLoopSourceWindowRejectV1> {
    let input = source_unit
        .root_function_input()
        .map_err(|_| SharedLoopSourceWindowRejectV1::SourceNavigation)?;
    if loop_stmt.owner() != input.owner() {
        return Err(SharedLoopSourceWindowRejectV1::ForeignOwner);
    }
    let (condition, body) = match loop_stmt.node() {
        ASTNode::Loop {
            condition, body, ..
        } => (condition.as_ref(), body.as_slice()),
        _ => return Err(SharedLoopSourceWindowRejectV1::NotLoop),
    };

    let function = input.function();
    let source_kind = function.source_kind();
    if source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(SharedLoopSourceWindowRejectV1::UnsupportedSourceKind(
            source_kind,
        ));
    }
    let loop_site = loop_stmt.site().clone();
    let loop_source = function
        .resolved_loop_source(&loop_site)
        .map_err(|_| SharedLoopSourceWindowRejectV1::SourceLookup)?;
    let frame = loop_source.frame_key();
    let function_origin = function.function_origin();
    let forest = function
        .resolved_loop_source_forest(&loop_site)
        .map_err(map_forest_reject)?;
    let Some(root) = forest.members().first() else {
        return Err(SharedLoopSourceWindowRejectV1::ForestEmpty);
    };
    if root.parent_index().is_some()
        || !root
            .source()
            .matches_identity(function_origin, source_kind, &loop_site)
    {
        return Err(SharedLoopSourceWindowRejectV1::ForestRootMismatch);
    }
    if !root.source().frame_key().matches(&frame) {
        return Err(SharedLoopSourceWindowRejectV1::FrameMismatch);
    }
    Ok(VerifiedSharedLoopSourceWindowV1 {
        source_unit,
        owner: input.owner(),
        function_origin,
        source_kind,
        loop_site,
        frame,
        forest,
        condition,
        body,
    })
}

fn map_forest_reject(
    reject: crate::mir::resolved_semantics::ResolvedLoopSourceForestRejectV1,
) -> SharedLoopSourceWindowRejectV1 {
    use crate::mir::resolved_semantics::ResolvedLoopSourceForestRejectV1;

    match reject {
        ResolvedLoopSourceForestRejectV1::UnsupportedOwnerRoot(kind) => {
            SharedLoopSourceWindowRejectV1::UnsupportedSourceKind(kind)
        }
        ResolvedLoopSourceForestRejectV1::MissingRoot(_) => {
            SharedLoopSourceWindowRejectV1::ForestRootMismatch
        }
        _ => SharedLoopSourceWindowRejectV1::SourceForest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::nested_function_for_p3_test;

    fn unit() -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(nested_function_for_p3_test())
            .expect("nested source unit resolves")
    }

    fn body_stmt<'a>(
        source_unit: &'a VerifiedResolvedSourceUnitV1,
        index: usize,
    ) -> LocatedStmtV1<'a> {
        let input = source_unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        input
            .source()
            .body_stmt(&body, index)
            .expect("body statement")
    }

    #[test]
    fn d4_witness_lends_one_canonical_nested_loop_pair() {
        let source_unit = unit();
        let loop_stmt = body_stmt(&source_unit, 1);
        let receipt = issue_shared_loop_source_window_v1(&source_unit, &loop_stmt)
            .expect("canonical nested loop window");
        receipt.with_views(|raw, resolved| {
            assert_eq!(raw.owner(), resolved.owner());
            assert_eq!(raw.site(), resolved.site());
            assert_eq!(
                resolved.source_kind(),
                SemanticOwnerSourceKindV1::DeclaredFunction
            );
            assert_eq!(resolved.forest().members().len(), 2);
            assert!(resolved.forest().members()[0]
                .source()
                .frame_key()
                .matches(resolved.frame()));
            assert!(matches!(raw.condition(), ASTNode::BinaryOp { .. }));
            assert_eq!(raw.body().len(), 4);
            assert!(matches!(raw.body()[0], ASTNode::Local { .. }));
        });
    }

    #[test]
    fn d4_witness_rejects_foreign_located_statement() {
        let foreign_unit = unit();
        let source_unit = unit();
        let foreign_loop = body_stmt(&foreign_unit, 1);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &foreign_loop),
            Err(SharedLoopSourceWindowRejectV1::ForeignOwner)
        ));
    }

    #[test]
    fn d4_witness_rejects_non_loop_statement() {
        let source_unit = unit();
        let local = body_stmt(&source_unit, 0);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &local),
            Err(SharedLoopSourceWindowRejectV1::NotLoop)
        ));
    }

    #[test]
    fn d4_witness_keeps_equal_shape_sessions_distinct() {
        let left_unit = unit();
        let right_unit = unit();
        let left_stmt = body_stmt(&left_unit, 1);
        let right_stmt = body_stmt(&right_unit, 1);
        let left_owner = left_unit.root_function_input().expect("left input").owner();
        let right_owner = right_unit
            .root_function_input()
            .expect("right input")
            .owner();
        assert_ne!(left_owner, right_owner);
        let left = issue_shared_loop_source_window_v1(&left_unit, &left_stmt).expect("left window");
        let right =
            issue_shared_loop_source_window_v1(&right_unit, &right_stmt).expect("right window");
        left.with_views(|left_raw, _| {
            right.with_views(|right_raw, _| {
                assert_ne!(left_raw.owner(), right_raw.owner());
                assert_eq!(left_raw.site(), right_raw.site());
            });
        });
    }
}
