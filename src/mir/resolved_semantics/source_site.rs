//! Structural source provenance for one canonical function AST.

use super::ids::FunctionOwnerIdV1;

/// Function provenance within one compilation input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionOriginV1 {
    compilation_unit_ordinal: u32,
    function_ordinal: u32,
}

impl FunctionOriginV1 {
    pub(crate) const fn new(compilation_unit_ordinal: u32, function_ordinal: u32) -> Self {
        Self {
            compilation_unit_ordinal,
            function_ordinal,
        }
    }

    pub const fn compilation_unit_ordinal(self) -> u32 {
        self.compilation_unit_ordinal
    }

    pub const fn function_ordinal(self) -> u32 {
        self.function_ordinal
    }
}

/// A typed step in a path relative to a function AST root.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourcePathSegmentV1 {
    FunctionBody,
    Body(u32),
    ScopeBodyRoot,
    ScopeBody(u32),
    TaskScopeBodyRoot,
    TaskScopeBody(u32),
    FastMemBodyRoot,
    FastMemBody(u32),
    IfCondition,
    IfThenBody,
    IfThen(u32),
    IfElseBody,
    IfElse(u32),
    LoopCondition,
    LoopBodyRoot,
    LoopBody(u32),
    Receiver,
    Callee,
    Argument(u32),
    Element(u32),
    EntryValue(u32),
    FieldValue(u32),
    UpdateValue(u32),
    Base,
    CheckItem(u32),
    Target,
    Value,
    Lhs,
    Rhs,
    Operand,
    Initializer(u32),
    Binding(u32),
    LambdaBodyRoot,
    LambdaBody(u32),
    QMarkOperand,
    MatchScrutinee,
    MatchArm(u32),
    MatchElse,
    EnumMatchScrutinee,
    EnumMatchArm(u32),
    EnumMatchElse,
    BlockExprPreludeRoot,
    BlockExprPrelude(u32),
    BlockExprTail,
    TryBodyRoot,
    TryBody(u32),
    CatchClause(u32),
    CatchBodyRoot,
    CatchBody(u32),
    CleanupBodyRoot,
    CleanupBody(u32),
}

/// Structural node provenance relative to one function root.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceNodeSiteV1(Box<[SourcePathSegmentV1]>);

impl SourceNodeSiteV1 {
    pub(crate) fn from_segments(segments: Vec<SourcePathSegmentV1>) -> Self {
        Self(segments.into_boxed_slice())
    }

    pub fn segments(&self) -> &[SourcePathSegmentV1] {
        &self.0
    }
}

/// Immutable path builder shared by resolver production and source projection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourcePathV1(Vec<SourcePathSegmentV1>);

impl SourcePathV1 {
    pub(crate) fn function_body() -> Self {
        Self(vec![SourcePathSegmentV1::FunctionBody])
    }

    pub(crate) fn lambda_body() -> Self {
        Self(vec![SourcePathSegmentV1::LambdaBodyRoot])
    }

    pub(crate) fn root_body(index: usize) -> Self {
        Self(vec![SourcePathSegmentV1::Body(index as u32)])
    }

    pub(crate) fn lambda_body_item(index: usize) -> Self {
        Self(vec![SourcePathSegmentV1::LambdaBody(index as u32)])
    }

    pub(crate) fn from_node(site: &SourceNodeSiteV1) -> Self {
        Self(site.segments().to_vec())
    }

    pub(crate) fn child(&self, segment: SourcePathSegmentV1) -> Self {
        let mut segments = self.0.clone();
        segments.push(segment);
        Self(segments)
    }

    pub(crate) fn node(&self) -> SourceNodeSiteV1 {
        SourceNodeSiteV1::from_segments(self.0.clone())
    }

    pub(crate) fn stmt(&self) -> SourceStmtSiteV1 {
        SourceStmtSiteV1::from_node(self.node())
    }

    pub(crate) fn expr(&self) -> SourceExprSiteV1 {
        SourceExprSiteV1::from_node(self.node())
    }
}

/// Structural statement provenance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceStmtSiteV1(SourceNodeSiteV1);

impl SourceStmtSiteV1 {
    pub(crate) fn from_node(site: SourceNodeSiteV1) -> Self {
        Self(site)
    }

    pub fn node(&self) -> &SourceNodeSiteV1 {
        &self.0
    }
}

/// Structural expression provenance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceExprSiteV1(SourceNodeSiteV1);

impl SourceExprSiteV1 {
    pub(crate) fn from_node(site: SourceNodeSiteV1) -> Self {
        Self(site)
    }

    pub fn node(&self) -> &SourceNodeSiteV1 {
        &self.0
    }
}

/// Exact source origin for a control transfer.
///
/// Statement and expression exits share one index without fabricating one
/// source family as the other. The first resolver slice publishes statement
/// exits only; expression provenance remains passive until its language row is
/// accepted independently.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ResolvedExitSiteV1 {
    Statement(SourceStmtSiteV1),
    Expression(SourceExprSiteV1),
}

impl ResolvedExitSiteV1 {
    pub fn node(&self) -> &SourceNodeSiteV1 {
        match self {
            Self::Statement(site) => site.node(),
            Self::Expression(site) => site.node(),
        }
    }
}

impl From<SourceStmtSiteV1> for ResolvedExitSiteV1 {
    fn from(site: SourceStmtSiteV1) -> Self {
        Self::Statement(site)
    }
}

impl From<SourceExprSiteV1> for ResolvedExitSiteV1 {
    fn from(site: SourceExprSiteV1) -> Self {
        Self::Expression(site)
    }
}

/// Expression provenance branded by the semantic owner whose syntax contains it.
///
/// A bare `SourceExprSiteV1` is relative to one owner root. Cross-owner maps
/// must use this type so identical relative paths in sibling owners cannot
/// alias.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedExprSiteV1 {
    owner: FunctionOwnerIdV1,
    site: SourceExprSiteV1,
}

impl OwnedExprSiteV1 {
    pub(crate) const fn new(owner: FunctionOwnerIdV1, site: SourceExprSiteV1) -> Self {
        Self { owner, site }
    }

    pub const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }
}

/// Declaration provenance. It is a checked index key, not binding identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SourceBindingSiteV1 {
    Receiver,
    Parameter {
        index: u32,
    },
    Local {
        statement: SourceStmtSiteV1,
        ordinal: u32,
    },
    Outbox {
        statement: SourceStmtSiteV1,
        ordinal: u32,
    },
    Nowait {
        statement: SourceStmtSiteV1,
    },
    LoopBinder {
        loop_site: SourceStmtSiteV1,
    },
    CatchBinder {
        node: SourceNodeSiteV1,
        ordinal: u32,
    },
    PatternBinder {
        node: SourceNodeSiteV1,
        ordinal: u32,
    },
}
