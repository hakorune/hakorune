use crate::ast::{BoxMethodInventoryOrdinalV1, ParamDecl};
use crate::parser::callable_source_anchor::CallableDeclarationIdentityV1;

use super::super::source_authority::SourceBoxMethodSiteV1;

/// AST-free neutral parameter syntax carried by the parser handoff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverMethodParameterSyntaxV1 {
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParserParameterTransferSyntaxV1 {
    kind: ParserTransferKindV1,
}

impl ParserParameterTransferSyntaxV1 {
    pub(in crate::parser) const fn ordinary() -> Self {
        Self::from_take(false)
    }

    pub(in crate::parser) const fn from_take(take: bool) -> Self {
        Self { kind: if take { ParserTransferKindV1::Take } else { ParserTransferKindV1::Ordinary } }
    }

    pub(in crate::parser) const fn edit() -> Self {
        Self { kind: ParserTransferKindV1::Edit }
    }

    pub(crate) const fn is_take(self) -> bool {
        matches!(self.kind, ParserTransferKindV1::Take)
    }

    pub(crate) const fn is_ordinary(self) -> bool {
        matches!(self.kind, ParserTransferKindV1::Ordinary)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserParameterDeclaredTypeSyntaxV1 {
    Absent,
    Explicit { spelling: Box<str>, reference: crate::ast::TypeReferenceV1 },
}

impl ParserParameterDeclaredTypeSyntaxV1 {
    pub(crate) fn reference(&self) -> Option<&crate::ast::TypeReferenceV1> {
        match self { Self::Absent => None, Self::Explicit { reference, .. } => Some(reference) }
    }

    pub(crate) fn as_deref(&self) -> Option<&str> {
        match self {
            Self::Absent => None,
            Self::Explicit { spelling, .. } => Some(spelling),
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
    #[cfg(test)]
    pub(super) fn ordinary(ordinal: u32, declaration: &ParamDecl) -> Self {
        Self::parsed(ordinal, declaration, ParserParameterTransferSyntaxV1::ordinary()).expect("valid neutral parameter fixture")
    }

    pub(super) fn parsed(ordinal: u32, declaration: &ParamDecl, transfer: ParserParameterTransferSyntaxV1) -> Result<Self, String> {
        let declared_type = match declaration.declared_type_name.as_deref() {
            Some(name) => ParserParameterDeclaredTypeSyntaxV1::Explicit { spelling: name.into(), reference: crate::ast::TypeReferenceV1::parse(name)? },
            None => ParserParameterDeclaredTypeSyntaxV1::Absent,
        };
        Ok(Self {
            ordinal,
            name: declaration.name.clone().into_boxed_str(),
            declared_type,
            transfer,
        })
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

/// Closed declaration-side transfer vocabulary shared by method-head receivers
/// and parameters; the parser issues it from the original source marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ParserTransferKindV1 {
    #[default]
    Ordinary,
    Edit,
    Take,
}

/// Passive declared read-result support, retained with the original header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParserBorrowedResultSupportSyntaxV1 {
    Receiver,
    Parameter { ordinal: u32, name: Box<str> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParserBorrowedResultSyntaxV1 {
    declared_type: Box<str>,
    support: ParserBorrowedResultSupportSyntaxV1,
}

impl ParserBorrowedResultSyntaxV1 {
    pub(in crate::parser) fn parsed(declared_type: String, support: ParserBorrowedResultSupportSyntaxV1) -> Self {
        Self { declared_type: declared_type.into_boxed_str(), support }
    }
    pub(crate) fn declared_type(&self) -> &str { &self.declared_type }
    pub(crate) fn support(&self) -> &ParserBorrowedResultSupportSyntaxV1 { &self.support }
}

#[derive(Debug)]
pub(crate) struct ParserCallableParameterDeclarationSourceV1 {
    source_site: SourceBoxMethodSiteV1,
    inventory_ordinal: BoxMethodInventoryOrdinalV1,
    callable_identity: CallableDeclarationIdentityV1,
    kind: ParserCallableDeclarationKindV1,
    diagnostic_name: Box<str>,
    parameters: Box<[ParserCallableParameterSourceRowV1]>,
    borrowed_result: Option<ParserBorrowedResultSyntaxV1>,
    receiver_transfer: ParserTransferKindV1,
    declared_result_type: Option<crate::ast::TypeReferenceV1>,
}

impl ParserCallableParameterDeclarationSourceV1 {
    pub(super) fn new(
        source_site: SourceBoxMethodSiteV1,
        inventory_ordinal: BoxMethodInventoryOrdinalV1,
        callable_identity: CallableDeclarationIdentityV1,
        kind: ParserCallableDeclarationKindV1,
        diagnostic_name: String,
        parameters: Box<[ParserCallableParameterSourceRowV1]>,
        borrowed_result: Option<ParserBorrowedResultSyntaxV1>,
        receiver_transfer: ParserTransferKindV1,
        declared_result_type: Option<crate::ast::TypeReferenceV1>,
    ) -> Self {
        Self {
            source_site,
            inventory_ordinal,
            callable_identity,
            kind,
            diagnostic_name: diagnostic_name.into_boxed_str(),
            parameters,
            borrowed_result,
            receiver_transfer,
            declared_result_type,
        }
    }

    pub(in crate::parser) fn source_site(&self) -> &SourceBoxMethodSiteV1 {
        &self.source_site
    }

    /// Comparison-only identity from the sole direct callable anchor issuer.
    pub(in crate::parser) fn callable_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.callable_identity
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

    pub(crate) fn diagnostic_name(&self) -> &str {
        &self.diagnostic_name
    }

    pub(crate) fn parameters(&self) -> &[ParserCallableParameterSourceRowV1] {
        &self.parameters
    }

    pub(crate) fn borrowed_result(&self) -> Option<&ParserBorrowedResultSyntaxV1> {
        self.borrowed_result.as_ref()
    }
    pub(crate) fn declared_result_type(&self) -> Option<&crate::ast::TypeReferenceV1> { self.declared_result_type.as_ref() }

    pub(crate) fn receiver_transfer(&self) -> ParserTransferKindV1 { self.receiver_transfer }

    pub(in crate::parser) fn requires_retained_source(&self) -> bool {
        self.receiver_transfer != ParserTransferKindV1::Ordinary || self.borrowed_result.is_some() || self.parameters.iter().any(|row| !row.transfer().is_ordinary())
    }

    pub(crate) fn box_statement_ordinal(&self) -> u32 {
        self.source_site.box_site().statement_ordinal()
    }

    pub(crate) fn source_member_ordinal(&self) -> u32 {
        self.source_site.source_member_ordinal()
    }
}

impl ResolverMethodParameterSyntaxV1 {
    #[cfg(test)]
    pub(super) fn from_neutral_syntax(name: String, declared_type_name: Option<String>) -> Self {
        Self {
            name: name.into_boxed_str(),
            declared_type_name: declared_type_name.map(String::into_boxed_str),
        }
    }

    #[cfg(test)]
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}
