//! Immutable syntax carriers paired with exact owner-branded source sites.

use std::num::NonZeroU32;

use crate::ast::ASTNode;
pub(crate) use crate::mir::resolved_semantics::SourceBodyKindV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathV1, SourceStmtSiteV1,
};

use super::source_view::SourceViewSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceBodySiteV1 {
    owner: FunctionOwnerIdV1,
    parent: Option<SourceNodeSiteV1>,
    kind: SourceBodyKindV1,
}

impl SourceBodySiteV1 {
    pub(super) const fn new_root(
        owner: FunctionOwnerIdV1,
        kind: SourceBodyKindV1,
        _seal: SourceViewSealV1,
    ) -> Self {
        Self {
            owner,
            parent: None,
            kind,
        }
    }

    pub(super) fn new_child(
        owner: FunctionOwnerIdV1,
        parent: SourceNodeSiteV1,
        kind: SourceBodyKindV1,
        _seal: SourceViewSealV1,
    ) -> Self {
        Self {
            owner,
            parent: Some(parent),
            kind,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn kind(&self) -> SourceBodyKindV1 {
        self.kind
    }

    pub(crate) fn root(&self) -> SourceNodeSiteV1 {
        match (self.kind, self.parent.as_ref()) {
            (SourceBodyKindV1::Function, None) => SourcePathV1::function_body().node(),
            (SourceBodyKindV1::Lambda, None) => SourcePathV1::lambda_body().node(),
            (kind, Some(parent)) => SourcePathV1::from_node(parent)
                .child(kind.root_segment().expect("child body has root segment"))
                .node(),
            _ => unreachable!("[freeze:contract][canonical_source/body_site_shape]"),
        }
    }

    pub(super) fn statement(&self, index: u32, _seal: SourceViewSealV1) -> SourceStmtSiteV1 {
        match self.parent.as_ref() {
            Some(parent) => SourcePathV1::from_node(parent)
                .child(self.kind.item_segment(index))
                .stmt(),
            None => match self.kind {
                SourceBodyKindV1::Function => SourcePathV1::root_body(index as usize).stmt(),
                SourceBodyKindV1::Lambda => SourcePathV1::lambda_body_item(index as usize).stmt(),
                _ => unreachable!("[freeze:contract][canonical_source/root_body_kind]"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocatedBodyV1<'a> {
    site: SourceBodySiteV1,
    statements: &'a [ASTNode],
    _seal: SourceViewSealV1,
}

impl<'a> LocatedBodyV1<'a> {
    pub(super) const fn new(
        site: SourceBodySiteV1,
        statements: &'a [ASTNode],
        seal: SourceViewSealV1,
    ) -> Self {
        Self {
            site,
            statements,
            _seal: seal,
        }
    }

    pub(crate) const fn site(&self) -> &SourceBodySiteV1 {
        &self.site
    }

    pub(crate) const fn statements(&self) -> &'a [ASTNode] {
        self.statements
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocatedStmtV1<'a> {
    owner: FunctionOwnerIdV1,
    site: SourceStmtSiteV1,
    node: &'a ASTNode,
    _seal: SourceViewSealV1,
}

impl<'a> LocatedStmtV1<'a> {
    pub(super) const fn new(
        owner: FunctionOwnerIdV1,
        site: SourceStmtSiteV1,
        node: &'a ASTNode,
        seal: SourceViewSealV1,
    ) -> Self {
        Self {
            owner,
            site,
            node,
            _seal: seal,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) const fn node(&self) -> &'a ASTNode {
        self.node
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocatedExprV1<'a> {
    owner: FunctionOwnerIdV1,
    site: SourceExprSiteV1,
    node: &'a ASTNode,
    _seal: SourceViewSealV1,
}

impl<'a> LocatedExprV1<'a> {
    pub(super) const fn new(
        owner: FunctionOwnerIdV1,
        site: SourceExprSiteV1,
        node: &'a ASTNode,
        seal: SourceViewSealV1,
    ) -> Self {
        Self {
            owner,
            site,
            node,
            _seal: seal,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn node(&self) -> &'a ASTNode {
        self.node
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LocatedBodySuffixV1<'a> {
    body: LocatedBodyV1<'a>,
    start_index: u32,
    _seal: SourceViewSealV1,
}

impl<'a> LocatedBodySuffixV1<'a> {
    pub(super) const fn new(
        body: LocatedBodyV1<'a>,
        start_index: u32,
        seal: SourceViewSealV1,
    ) -> Self {
        Self {
            body,
            start_index,
            _seal: seal,
        }
    }

    pub(crate) const fn body(&self) -> &LocatedBodyV1<'a> {
        &self.body
    }

    pub(crate) const fn start_index(&self) -> u32 {
        self.start_index
    }

    #[cfg(test)]
    pub(super) const fn new_for_test(body: LocatedBodyV1<'a>, start_index: u32) -> Self {
        Self {
            body,
            start_index,
            _seal: SourceViewSealV1::for_test(),
        }
    }
}

/// One nonempty, exact, contiguous prefix consumed from a located body suffix.
///
/// Only `FunctionSourceViewV1` can construct this in production safe code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConsumedSourceRangeV1 {
    body: SourceBodySiteV1,
    start: u32,
    count: NonZeroU32,
    _seal: SourceViewSealV1,
}

impl ConsumedSourceRangeV1 {
    pub(super) const fn new(
        body: SourceBodySiteV1,
        start: u32,
        count: NonZeroU32,
        seal: SourceViewSealV1,
    ) -> Self {
        Self {
            body,
            start,
            count,
            _seal: seal,
        }
    }

    pub(crate) const fn body(&self) -> &SourceBodySiteV1 {
        &self.body
    }

    pub(crate) const fn start(&self) -> u32 {
        self.start
    }

    pub(crate) const fn count(&self) -> NonZeroU32 {
        self.count
    }

    #[cfg(test)]
    pub(super) const fn new_for_test(
        body: SourceBodySiteV1,
        start: u32,
        count: NonZeroU32,
    ) -> Self {
        Self {
            body,
            start,
            count,
            _seal: SourceViewSealV1::for_test(),
        }
    }
}
