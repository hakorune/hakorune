//! Structural source provenance for one canonical function AST.

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
