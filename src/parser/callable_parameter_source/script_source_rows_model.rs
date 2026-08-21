//! AST-free payload types for the canonical pure-Script parser rows.

use super::parser_invocation_witness::ParserInvocationWitnessV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScriptBodySyntaxKindV1 {
    ExecutableItem,
    FunctionDeclaration,
    BrandDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    GlobalVar,
    StaticConstTable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptBodyRowV1 {
    ordinal: u32,
    kind: ScriptBodySyntaxKindV1,
}

impl ScriptBodyRowV1 {
    pub(super) const fn new(ordinal: u32, kind: ScriptBodySyntaxKindV1) -> Self {
        Self { ordinal, kind }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn kind(&self) -> ScriptBodySyntaxKindV1 {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptParameterSyntaxRowV1 {
    ordinal: u32,
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
}

impl ScriptParameterSyntaxRowV1 {
    pub(super) fn new(ordinal: u32, name: Box<str>, declared_type_name: Option<Box<str>>) -> Self {
        Self {
            ordinal,
            name,
            declared_type_name,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptDeclarationSyntaxSnapshotV1 {
    ordinal: u32,
    kind: ScriptBodySyntaxKindV1,
    name: Box<str>,
    parameters: Box<[ScriptParameterSyntaxRowV1]>,
}

impl ScriptDeclarationSyntaxSnapshotV1 {
    pub(super) fn new(
        ordinal: u32,
        kind: ScriptBodySyntaxKindV1,
        name: Box<str>,
        parameters: Box<[ScriptParameterSyntaxRowV1]>,
    ) -> Self {
        Self {
            ordinal,
            kind,
            name,
            parameters,
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn kind(&self) -> ScriptBodySyntaxKindV1 {
        self.kind
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn parameters(&self) -> &[ScriptParameterSyntaxRowV1] {
        &self.parameters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrandSyntaxSnapshotV1 {
    ordinal: u32,
    name: Box<str>,
    underlying_type_name: Box<str>,
}

impl BrandSyntaxSnapshotV1 {
    pub(super) fn new(ordinal: u32, name: Box<str>, underlying_type_name: Box<str>) -> Self {
        Self {
            ordinal,
            name,
            underlying_type_name,
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn underlying_type_name(&self) -> &str {
        &self.underlying_type_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScriptImportConfigSnapshotV1 {
    explicit: bool,
    complete: bool,
}

impl ScriptImportConfigSnapshotV1 {
    pub(super) const fn no_imports() -> Self {
        Self {
            explicit: true,
            complete: true,
        }
    }

    pub(crate) const fn is_explicit(&self) -> bool {
        self.explicit
    }

    pub(crate) const fn is_complete(&self) -> bool {
        self.complete
    }
}

#[derive(Debug)]
pub(crate) struct CanonicalScriptSourceRowsV1 {
    pub(super) parser_brand: ParserInvocationWitnessV1,
    pub(super) statement_count: u32,
    pub(super) body_rows: Box<[ScriptBodyRowV1]>,
    pub(super) declarations: Box<[ScriptDeclarationSyntaxSnapshotV1]>,
    pub(super) brands: Box<[BrandSyntaxSnapshotV1]>,
    pub(super) import_config: ScriptImportConfigSnapshotV1,
    pub(super) seal: CanonicalScriptSourceRowsSealV1,
}

#[derive(Debug)]
pub(super) struct CanonicalScriptSourceRowsSealV1;

impl CanonicalScriptSourceRowsV1 {
    pub(crate) fn parser_invocation_witness(&self) -> &ParserInvocationWitnessV1 {
        &self.parser_brand
    }

    pub(crate) const fn statement_count(&self) -> u32 {
        self.statement_count
    }

    pub(crate) fn body_rows(&self) -> &[ScriptBodyRowV1] {
        &self.body_rows
    }

    pub(crate) fn declarations(&self) -> &[ScriptDeclarationSyntaxSnapshotV1] {
        &self.declarations
    }

    pub(crate) fn brands(&self) -> &[BrandSyntaxSnapshotV1] {
        &self.brands
    }

    pub(crate) fn import_config(&self) -> &ScriptImportConfigSnapshotV1 {
        &self.import_config
    }
}

#[derive(Debug)]
pub(crate) enum CanonicalScriptSourceRowsDispositionV1 {
    NotApplicable,
    CompatibilitySource,
    Deferred,
    AdmissionMissing,
    SourceAuthorityUnavailable,
    CohortUnresolved,
    ObservationIncomplete,
    IntegrityInvalid,
    NonCandidate,
    HandoffReady(CanonicalScriptSourceRowsV1),
    MovedToParallelHandoff,
    DispositionTransported,
}

impl CanonicalScriptSourceRowsDispositionV1 {
    pub(crate) fn parser_invocation_witness(&self) -> Option<ParserInvocationWitnessV1> {
        match self {
            Self::HandoffReady(rows) => Some(rows.parser_brand.clone()),
            Self::NotApplicable
            | Self::CompatibilitySource
            | Self::Deferred
            | Self::AdmissionMissing
            | Self::SourceAuthorityUnavailable
            | Self::CohortUnresolved
            | Self::ObservationIncomplete
            | Self::IntegrityInvalid
            | Self::NonCandidate
            | Self::MovedToParallelHandoff
            | Self::DispositionTransported => None,
        }
    }
}
