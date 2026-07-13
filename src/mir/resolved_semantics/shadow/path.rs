//! Structural path construction for the borrowed canonical AST.

use crate::mir::resolved_semantics::source_site::{
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone)]
pub(super) struct ShadowSourcePathV0(Vec<SourcePathSegmentV1>);

impl ShadowSourcePathV0 {
    pub(super) fn function_body() -> Self {
        Self(vec![SourcePathSegmentV1::FunctionBody])
    }

    pub(super) fn root_body(index: usize) -> Self {
        Self(vec![SourcePathSegmentV1::Body(index as u32)])
    }

    pub(super) fn child(&self, segment: SourcePathSegmentV1) -> Self {
        let mut segments = self.0.clone();
        segments.push(segment);
        Self(segments)
    }

    pub(super) fn node(&self) -> SourceNodeSiteV1 {
        SourceNodeSiteV1::from_segments(self.0.clone())
    }

    pub(super) fn stmt(&self) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(self.node())
    }

    pub(super) fn expr(&self) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(self.node())
    }
}
