//! Sealed whole-owner trivial representation product.

use crate::mir::compiler::located::SourceBodySiteV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TrivialRepresentationV1 {
    InlineI64,
    InlineBool,
    InlineF64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLocatedTrivialValueV1 {
    site: SourceExprSiteV1,
    representation: TrivialRepresentationV1,
}

impl VerifiedLocatedTrivialValueV1 {
    pub(super) const fn new(
        site: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
    ) -> Self {
        Self {
            site,
            representation,
        }
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TrivialBindingDefinitionOriginV1 {
    Declaration(SourceBindingSiteV1),
    Assignment(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialBindingDefinitionV1 {
    binding: BindingRefV1,
    origin: TrivialBindingDefinitionOriginV1,
    representation: TrivialRepresentationV1,
}

impl VerifiedTrivialBindingDefinitionV1 {
    pub(super) const fn new(
        binding: BindingRefV1,
        origin: TrivialBindingDefinitionOriginV1,
        representation: TrivialRepresentationV1,
    ) -> Self {
        Self {
            binding,
            origin,
            representation,
        }
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn origin(&self) -> &TrivialBindingDefinitionOriginV1 {
        &self.origin
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialIfMergeProfileV1 {
    statement: SourceStmtSiteV1,
    binding: BindingRefV1,
    representation: TrivialRepresentationV1,
}

impl VerifiedTrivialIfMergeProfileV1 {
    pub(super) const fn new(
        statement: SourceStmtSiteV1,
        binding: BindingRefV1,
        representation: TrivialRepresentationV1,
    ) -> Self {
        Self {
            statement,
            binding,
            representation,
        }
    }

    pub(crate) const fn statement(&self) -> &SourceStmtSiteV1 {
        &self.statement
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn representation(&self) -> TrivialRepresentationV1 {
        self.representation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrivialTerminalProfileV1 {
    ExplicitValue {
        statement: SourceStmtSiteV1,
        value: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
    },
    ExplicitNoValue {
        statement: SourceStmtSiteV1,
    },
    ImplicitNoValue {
        body: SourceBodySiteV1,
        body_end: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TrivialProfileCoverageSubjectV1 {
    Value(SourceExprSiteV1),
    Definition {
        binding: BindingRefV1,
        origin: TrivialBindingDefinitionOriginV1,
    },
    IfMergeProfile {
        statement: SourceStmtSiteV1,
        binding: BindingRefV1,
    },
    ExplicitValueTerminal(SourceStmtSiteV1),
    ExplicitNoValueTerminal(SourceStmtSiteV1),
    ImplicitNoValueTerminal {
        body: SourceBodySiteV1,
        body_end: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialProfileCoverageV1 {
    ordered_subjects: Box<[TrivialProfileCoverageSubjectV1]>,
}

impl VerifiedTrivialProfileCoverageV1 {
    pub(super) fn from_verified_order(
        ordered_subjects: Vec<TrivialProfileCoverageSubjectV1>,
    ) -> Self {
        Self {
            ordered_subjects: ordered_subjects.into_boxed_slice(),
        }
    }

    pub(crate) fn ordered_subjects(&self) -> &[TrivialProfileCoverageSubjectV1] {
        &self.ordered_subjects
    }
}

#[derive(Debug)]
struct TrivialCanonicalOwnerSealV1;

#[derive(Debug)]
pub(crate) struct VerifiedTrivialCanonicalOwnerV1 {
    owner: FunctionOwnerIdV1,
    values: Box<[VerifiedLocatedTrivialValueV1]>,
    definitions: Box<[VerifiedTrivialBindingDefinitionV1]>,
    merge_profiles: Box<[VerifiedTrivialIfMergeProfileV1]>,
    terminal: TrivialTerminalProfileV1,
    coverage: VerifiedTrivialProfileCoverageV1,
    _seal: TrivialCanonicalOwnerSealV1,
}

impl VerifiedTrivialCanonicalOwnerV1 {
    pub(super) fn from_verified_parts(
        owner: FunctionOwnerIdV1,
        values: Vec<VerifiedLocatedTrivialValueV1>,
        definitions: Vec<VerifiedTrivialBindingDefinitionV1>,
        merge_profiles: Vec<VerifiedTrivialIfMergeProfileV1>,
        terminal: TrivialTerminalProfileV1,
        coverage: VerifiedTrivialProfileCoverageV1,
    ) -> Self {
        Self {
            owner,
            values: values.into_boxed_slice(),
            definitions: definitions.into_boxed_slice(),
            merge_profiles: merge_profiles.into_boxed_slice(),
            terminal,
            coverage,
            _seal: TrivialCanonicalOwnerSealV1,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn values(&self) -> &[VerifiedLocatedTrivialValueV1] {
        &self.values
    }

    pub(crate) fn definitions(&self) -> &[VerifiedTrivialBindingDefinitionV1] {
        &self.definitions
    }

    /// Homogeneous representation witnesses for exact visible bindings.
    ///
    /// These rows never decide whether a PHI exists or where it is placed;
    /// function-owned Binding SSA remains the sole PHI authority.
    pub(crate) fn merge_profiles(&self) -> &[VerifiedTrivialIfMergeProfileV1] {
        &self.merge_profiles
    }

    pub(crate) const fn terminal(&self) -> &TrivialTerminalProfileV1 {
        &self.terminal
    }

    pub(crate) const fn coverage(&self) -> &VerifiedTrivialProfileCoverageV1 {
        &self.coverage
    }

    pub(crate) fn representation_at(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<TrivialRepresentationV1> {
        self.values
            .iter()
            .find(|row| row.site() == site)
            .map(VerifiedLocatedTrivialValueV1::representation)
    }
}
