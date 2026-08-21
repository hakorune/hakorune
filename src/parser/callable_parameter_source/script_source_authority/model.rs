use super::super::composite_source::ParserCompositeSourceDispositionV1;
use super::super::parser_invocation_witness::ParserInvocationWitnessV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramBodySyntaxKindV1 {
    BoxDeclaration,
    BuildGate,
    FunctionDeclaration,
    BrandDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    GlobalVar,
    StaticConstTable,
    UsingStatement,
    ImportStatement,
    ExecutableItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParserNormalProgramBodySourceRowV1 {
    position: u32,
    kind: ParserNormalProgramBodySyntaxKindV1,
}

impl ParserNormalProgramBodySourceRowV1 {
    pub(super) const fn new(
        position: u32,
        kind: ParserNormalProgramBodySyntaxKindV1,
    ) -> Self {
        Self { position, kind }
    }

    pub(crate) const fn position(self) -> u32 {
        self.position
    }

    pub(crate) const fn kind(self) -> ParserNormalProgramBodySyntaxKindV1 {
        self.kind
    }
}

#[derive(Debug)]
pub(crate) struct ParserNormalProgramSourceAuthorityV1 {
    invocation: ParserInvocationWitnessV1,
    body_rows: Box<[ParserNormalProgramBodySourceRowV1]>,
    composite: ParserCompositeSourceDispositionV1,
    _seal: ParserNormalProgramSourceAuthoritySealV1,
}

#[derive(Debug)]
pub(super) struct ParserNormalProgramSourceAuthoritySealV1;

#[derive(Debug)]
pub(crate) enum ParserNormalProgramSourceAuthorityDispositionV1 {
    Ready(ParserNormalProgramSourceAuthorityV1),
    SourceAuthorityUnavailable(ParserNormalProgramSourceAuthorityUnavailableV1),
    Incomplete(ParserNormalProgramSourceAuthorityIncompleteV1),
    IntegrityInvalid(ParserNormalProgramSourceAuthorityIntegrityIssueV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramSourceAuthorityUnavailableV1 {
    PostpassNotSourceBacked,
    ParameterSourceUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramSourceAuthorityIncompleteV1 {
    ProgramBodyMissing,
    StatementPositionOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalProgramSourceAuthorityIntegrityIssueV1 {
    CompositeReadyWithoutProgramBody,
    BodyCoverageMismatch,
    BodyKindMismatch,
}

impl ParserNormalProgramSourceAuthorityDispositionV1 {
    pub(crate) fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }

    pub(crate) fn composite_source_is_ready(&self) -> bool {
        matches!(
            self,
            Self::Ready(authority) if authority.composite.is_ready()
        )
    }

    pub(crate) fn invocation_witness(&self) -> Option<&ParserInvocationWitnessV1> {
        match self {
            Self::Ready(authority) => Some(&authority.invocation),
            Self::SourceAuthorityUnavailable(_)
            | Self::Incomplete(_)
            | Self::IntegrityInvalid(_) => None,
        }
    }
}

impl ParserNormalProgramSourceAuthorityV1 {
    pub(super) fn new(
        invocation: ParserInvocationWitnessV1,
        body_rows: Box<[ParserNormalProgramBodySourceRowV1]>,
        composite: ParserCompositeSourceDispositionV1,
    ) -> Self {
        Self {
            invocation,
            body_rows,
            composite,
            _seal: ParserNormalProgramSourceAuthoritySealV1,
        }
    }

    pub(crate) fn invocation_witness(&self) -> &ParserInvocationWitnessV1 {
        &self.invocation
    }

    pub(crate) fn body_rows(&self) -> &[ParserNormalProgramBodySourceRowV1] {
        &self.body_rows
    }

    pub(crate) fn composite_source(&self) -> &ParserCompositeSourceDispositionV1 {
        &self.composite
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        ParserInvocationWitnessV1,
        Box<[ParserNormalProgramBodySourceRowV1]>,
        ParserCompositeSourceDispositionV1,
    ) {
        (self.invocation, self.body_rows, self.composite)
    }
}
