use crate::ast::{BoxMethodInventoryOrdinalV1, ParamDecl};

use super::super::source_authority::SourceBoxMethodSiteV1;

/// AST-free neutral parameter syntax carried by the parser handoff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverMethodParameterSyntaxV1 {
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
}

/// Closed source vocabulary. `Take` is reserved but has no parser issuer in
/// this implementation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserParameterTransferKindV1 {
    Ordinary,
    #[allow(dead_code)]
    Take,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParserParameterTransferSyntaxV1 {
    kind: ParserParameterTransferKindV1,
}

impl ParserParameterTransferSyntaxV1 {
    pub(super) const fn ordinary() -> Self {
        Self {
            kind: ParserParameterTransferKindV1::Ordinary,
        }
    }

    pub(crate) const fn is_ordinary(self) -> bool {
        matches!(self.kind, ParserParameterTransferKindV1::Ordinary)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserParameterDeclaredTypeSyntaxV1 {
    Absent,
    Explicit(Box<str>),
}

impl ParserParameterDeclaredTypeSyntaxV1 {
    pub(crate) fn as_deref(&self) -> Option<&str> {
        match self {
            Self::Absent => None,
            Self::Explicit(name) => Some(name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserCallableParameterSourceRowV1 {
    ordinal: u32,
    name: Box<str>,
    declared_type: ParserParameterDeclaredTypeSyntaxV1,
    transfer: ParserParameterTransferSyntaxV1,
}

impl ParserCallableParameterSourceRowV1 {
    pub(super) fn ordinary(ordinal: u32, declaration: &ParamDecl) -> Self {
        let declared_type = match declaration.declared_type_name.as_deref() {
            Some(name) => ParserParameterDeclaredTypeSyntaxV1::Explicit(name.into()),
            None => ParserParameterDeclaredTypeSyntaxV1::Absent,
        };
        Self {
            ordinal,
            name: declaration.name.clone().into_boxed_str(),
            declared_type,
            transfer: ParserParameterTransferSyntaxV1::ordinary(),
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn declared_type(&self) -> &ParserParameterDeclaredTypeSyntaxV1 {
        &self.declared_type
    }

    pub(crate) const fn transfer(&self) -> ParserParameterTransferSyntaxV1 {
        self.transfer
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserCallableDeclarationKindV1 {
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug)]
pub(crate) struct ParserCallableParameterDeclarationSourceV1 {
    source_site: SourceBoxMethodSiteV1,
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    kind: ParserCallableDeclarationKindV1,
    diagnostic_name: Box<str>,
    parameters: Box<[ParserCallableParameterSourceRowV1]>,
}

impl ParserCallableParameterDeclarationSourceV1 {
    pub(super) fn new(
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: String,
        parameters: Box<[ParserCallableParameterSourceRowV1]>,
    ) -> Self {
        Self {
            source_site,
            inventory_ordinal,
            kind,
            diagnostic_name: diagnostic_name.into_boxed_str(),
            parameters,
        }
    }

    pub(in crate::parser) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    /// Descriptive placement inside the selected method inventory.
    ///
    /// Source identity remains `source_site`; this receipt exists only so a
    /// later parser-owned loan can locate the already-committed declaration
    /// without a name lookup.
    pub(crate) const fn inventory_ordinal(&self) -> BoxMethodInventoryOrdinalV1 {
        self.inventory_ordinal
    }

    pub(crate) const fn kind(&self) -> ParserCallableDeclarationKindV1 {
        self.kind
    }

    pub(crate) const fn is_static(&self) -> bool {
        matches!(self.kind, ParserCallableDeclarationKindV1::StaticBoxMethod)
    }

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(crate) fn parameters(&self) -> &[ParserCallableParameterSourceRowV1] {
        &self.parameters
    }

    pub(crate) fn box_statement_ordinal(&self) -> u32 {
        self.source_site.box_site().statement_ordinal()
    }

    pub(crate) fn source_member_ordinal(&self) -> u32 {
        self.source_site.source_member_ordinal()
    }
}

impl ResolverMethodParameterSyntaxV1 {
    pub(super) fn from_neutral_syntax(name: String, declared_type_name: Option<String>) -> Self {
        Self {
            name: name.into_boxed_str(),
            declared_type_name: declared_type_name.map(String::into_boxed_str),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}
