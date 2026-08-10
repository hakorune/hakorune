use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::CanonicalLoweringErrorV1;
use crate::parser::{ParserCallableSyntaxLoanErrorV1, RetainedParserCallableSemanticSourceV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedCallableDeclarationModeV1 {
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug)]
pub(super) struct VerifiedResolvedCallableSemanticRowV1 {
    pub(super) source_row_index: u32,
    pub(super) mode: ResolvedCallableDeclarationModeV1,
    pub(super) parameter_count: u32,
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) function_origin: FunctionOriginV1,
    pub(super) forest: VerifiedSemanticOwnerForestV1,
    pub(super) projection: VerifiedSourceProjectionV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableSemanticBatchV1 {
    pub(super) source: RetainedParserCallableSemanticSourceV1,
    pub(super) rows: Box<[VerifiedResolvedCallableSemanticRowV1]>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedResolvedCallableSemanticDeclarationRefV1<'batch> {
    row: &'batch VerifiedResolvedCallableSemanticRowV1,
}

impl VerifiedResolvedCallableSemanticBatchV1 {
    pub(crate) fn declarations(
        &self,
    ) -> impl ExactSizeIterator<Item = VerifiedResolvedCallableSemanticDeclarationRefV1<'_>> {
        self.rows
            .iter()
            .map(|row| VerifiedResolvedCallableSemanticDeclarationRefV1 { row })
    }

    pub(crate) fn with_lowering_input<R>(
        &self,
        source_row_index: u32,
        callback: impl for<'source> FnOnce(ResolvedFunctionLoweringInputV1<'source>) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        let index = usize::try_from(source_row_index)
            .map_err(|_| ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;
        let semantic = self
            .rows
            .get(index)
            .filter(|row| row.source_row_index == source_row_index)
            .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;

        self.source
            .with_callable_declaration_syntax(|catalog, loan| {
                let source = catalog
                    .declarations()
                    .get(index)
                    .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage)?;
                let syntax = loan
                    .declarations()
                    .get(index)
                    .filter(|row| row.source_row_index() == source_row_index)
                    .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage)?;
                if source.parameters().len()
                    != usize::try_from(semantic.parameter_count).unwrap_or(usize::MAX)
                {
                    return Err(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage);
                }
                let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
                    syntax.declaration(),
                    &semantic.forest,
                    &semantic.projection,
                )
                .map_err(ResolvedCallableSemanticBatchLoanErrorV1::LoweringInput)?;
                if input.owner() != semantic.owner {
                    return Err(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch);
                }
                Ok(callback(input))
            })
            .map_err(ResolvedCallableSemanticBatchLoanErrorV1::ParserSyntax)?
    }
}

impl VerifiedResolvedCallableSemanticDeclarationRefV1<'_> {
    pub(crate) const fn source_row_index(self) -> u32 {
        self.row.source_row_index
    }

    pub(crate) const fn mode(self) -> ResolvedCallableDeclarationModeV1 {
        self.row.mode
    }

    pub(crate) const fn parameter_count(self) -> u32 {
        self.row.parameter_count
    }

    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.row.owner
    }

    pub(crate) const fn function_origin(self) -> FunctionOriginV1 {
        self.row.function_origin
    }
}

#[derive(Debug)]
pub(crate) enum ResolvedCallableSemanticBatchLoanErrorV1 {
    ParserSyntax(ParserCallableSyntaxLoanErrorV1),
    MissingSourceRow,
    SourceCoverage,
    LoweringInput(CanonicalLoweringErrorV1),
    OwnerMismatch,
}
