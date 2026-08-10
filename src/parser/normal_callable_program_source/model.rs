use crate::ast::ASTNode;

use super::super::callable_parameter_source::{
    borrow_callable_declaration_syntax_v1, ParserCallableDeclarationSyntaxLoanV1,
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
    ParserCallableSyntaxLoanErrorV1,
};
use super::super::callable_source_anchor::{
    DirectCallableDeclarationKindV1, PreparedCallableSourceV1,
};
use super::super::initial_callable_program_source::{
    InitialCallableFinalSlotV1, VerifiedInitialCallableProgramSourceV1,
};
use super::super::source_path::SourceProgramCallablePathV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableParserCompatibilityV1 {
    InterfaceBox,
    RecordBox,
    MixedProgram,
    TopLevelBuildGate,
    NoBoxDeclarations,
    NonProgram,
    UnsupportedCallableSource,
}

#[derive(Debug)]
pub(crate) enum ParsedNormalCallableProgramV1 {
    SourceBacked(PreparedNormalCallableProgramSourceV1),
    Compatibility {
        ast: ASTNode,
        cohort: NormalCallableParserCompatibilityV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum NormalCallableParameterSourceRejectV1 {
    ForeignParser,
    MissingDirectMethod,
    DuplicateDirectMethod,
    UnexpectedDirectMethod,
    SyntaxMismatch,
}

#[derive(Debug)]
pub(crate) struct PreparedNormalCallableProgramSourceV1 {
    initial: VerifiedInitialCallableProgramSourceV1,
    parameter_source: ParserCallableParameterSourceDispositionV1,
}

impl PreparedNormalCallableProgramSourceV1 {
    pub(in crate::parser) fn issue(
        initial: VerifiedInitialCallableProgramSourceV1,
        parameter_source: ParserCallableParameterSourceDispositionV1,
    ) -> Result<Self, NormalCallableParameterSourceRejectV1> {
        if let ParserCallableParameterSourceDispositionV1::Complete(catalog) = &parameter_source {
            validate_direct_parameter_coverage(initial.callable_rows(), catalog)?;
            borrow_callable_declaration_syntax_v1(initial.ast(), catalog)
                .map_err(|_| NormalCallableParameterSourceRejectV1::SyntaxMismatch)?;
        }
        Ok(Self {
            initial,
            parameter_source,
        })
    }

    pub(crate) fn ast(&self) -> &ASTNode {
        self.initial.ast()
    }

    pub(crate) fn into_ast(self) -> ASTNode {
        self.initial.into_ast()
    }

    pub(in crate::parser) fn into_transform_parts(
        self,
    ) -> (
        ASTNode,
        Box<[PreparedCallableSourceV1]>,
        Box<[InitialCallableFinalSlotV1]>,
        ParserCallableParameterSourceDispositionV1,
    ) {
        let (ast, sources, slots) = self.initial.into_transform_parts();
        (ast, sources, slots, self.parameter_source)
    }
}

impl ParsedNormalCallableProgramV1 {
    pub(crate) fn ast(&self) -> &ASTNode {
        match self {
            Self::SourceBacked(source) => source.ast(),
            Self::Compatibility { ast, .. } => ast,
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedFinalCallableProgramSourceV1 {
    ast: ASTNode,
    sources: Box<[PreparedCallableSourceV1]>,
    slots: Box<[InitialCallableFinalSlotV1]>,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    _lineage: ExactCallablePreservingTransformReceiptV1,
}

#[derive(Debug)]
struct ExactCallablePreservingTransformReceiptV1;

impl VerifiedFinalCallableProgramSourceV1 {
    pub(super) fn issue(
        ast: ASTNode,
        sources: Box<[PreparedCallableSourceV1]>,
        slots: Box<[InitialCallableFinalSlotV1]>,
        parameter_source: ParserCallableParameterSourceDispositionV1,
    ) -> Self {
        Self {
            ast,
            sources,
            slots,
            parameter_source,
            _lineage: ExactCallablePreservingTransformReceiptV1,
        }
    }

    pub(crate) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(in crate::parser) fn callable_count(&self) -> usize {
        debug_assert_eq!(self.sources.len(), self.slots.len());
        self.sources.len()
    }

    /// Lend exact direct-method parameter syntax without allowing AST-backed
    /// declaration references to escape. Selected member-gate programs retain
    /// their callable anchors but deliberately return `Ok(None)` until their
    /// own exact parameter-source issuer exists.
    pub(crate) fn with_callable_parameter_syntax<R>(
        &self,
        callback: impl for<'source> FnOnce(
            &'source ParserCallableParameterSourceCatalogV1,
            ParserCallableDeclarationSyntaxLoanV1<'source>,
        ) -> R,
    ) -> Result<Option<R>, ParserCallableSyntaxLoanErrorV1> {
        let ParserCallableParameterSourceDispositionV1::Complete(catalog) = &self.parameter_source
        else {
            return Ok(None);
        };
        let loan = borrow_callable_declaration_syntax_v1(&self.ast, catalog)?;
        Ok(Some(callback(catalog, loan)))
    }
}

fn validate_direct_parameter_coverage(
    sources: &[PreparedCallableSourceV1],
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> Result<(), NormalCallableParameterSourceRejectV1> {
    if sources
        .iter()
        .any(|source| !catalog.same_parser_brand(source.parser_brand()))
    {
        return Err(NormalCallableParameterSourceRejectV1::ForeignParser);
    }

    let expected = sources
        .iter()
        .filter_map(PreparedCallableSourceV1::direct)
        .filter(|source| {
            matches!(
                source.path(),
                SourceProgramCallablePathV1::BoxMethod { gate_path, .. }
                    if gate_path.is_empty()
            )
        })
        .collect::<Vec<_>>();
    if expected.len() != catalog.declarations().len() {
        return Err(if expected.len() > catalog.declarations().len() {
            NormalCallableParameterSourceRejectV1::MissingDirectMethod
        } else {
            NormalCallableParameterSourceRejectV1::UnexpectedDirectMethod
        });
    }

    for declaration in catalog.declarations() {
        let mut matches = expected
            .iter()
            .copied()
            .filter(|source| direct_source_matches_parameter_declaration(source, declaration));
        if matches.next().is_none() {
            return Err(NormalCallableParameterSourceRejectV1::UnexpectedDirectMethod);
        }
        if matches.next().is_some() {
            return Err(NormalCallableParameterSourceRejectV1::DuplicateDirectMethod);
        }
    }
    Ok(())
}

fn direct_source_matches_parameter_declaration(
    source: &super::super::callable_source_anchor::PreparedDirectCallableSourceV1,
    declaration: &super::super::callable_parameter_source::ParserCallableParameterDeclarationSourceV1,
) -> bool {
    let SourceProgramCallablePathV1::BoxMethod {
        declaration: source_box,
        gate_path,
        member_ordinal,
    } = source.path()
    else {
        return false;
    };
    if !gate_path.is_empty()
        || source_box.compatibility_box_path() != declaration.source_site().box_site().path()
        || *member_ordinal != declaration.source_member_ordinal()
    {
        return false;
    }
    matches!(
        (source.kind(), declaration.kind()),
        (
            DirectCallableDeclarationKindV1::StaticBoxMethod,
            super::super::callable_parameter_source::ParserCallableDeclarationKindV1::StaticBoxMethod
        ) | (
            DirectCallableDeclarationKindV1::InstanceBoxMethod,
            super::super::callable_parameter_source::ParserCallableDeclarationKindV1::InstanceBoxMethod
        )
    )
}
