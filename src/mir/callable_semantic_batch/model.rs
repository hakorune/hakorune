use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, VerifiedResolvedFunctionV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::CanonicalLoweringErrorV1;
use crate::parser::{ParserCallableSyntaxLoanErrorV1, VerifiedFinalCallableProgramSourceV1};

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
    pub(super) source: VerifiedFinalCallableProgramSourceV1,
    pub(super) rows: Box<[VerifiedResolvedCallableSemanticRowV1]>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedResolvedCallableSemanticDeclarationRefV1<'batch> {
    row: &'batch VerifiedResolvedCallableSemanticRowV1,
}

#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableSemanticBatchRefV1<'batch> {
    rows: Box<[VerifiedResolvedCallableSemanticRowRefV1<'batch>]>,
}

#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableSemanticRowRefV1<'batch> {
    semantic: &'batch VerifiedResolvedCallableSemanticRowV1,
    function: &'batch VerifiedResolvedFunctionV1,
    parameters: Box<[VerifiedResolvedCallableParameterSourceRefV1<'batch>]>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedResolvedCallableParameterSourceRefV1<'batch> {
    ordinal: u32,
    name: &'batch str,
    ordinary: bool,
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
            .with_callable_parameter_syntax(|catalog, loan| {
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
            .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::ParameterSourceUnavailable)?
    }

    pub(crate) fn with_declaration_semantics<R>(
        &self,
        callback: impl for<'source> FnOnce(VerifiedResolvedCallableSemanticBatchRefV1<'source>) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        self.source
            .with_callable_parameter_syntax(|catalog, loan| {
                if catalog.declarations().len() != self.rows.len()
                    || loan.declarations().len() != self.rows.len()
                {
                    return Err(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage);
                }
                let mut rows = Vec::with_capacity(self.rows.len());
                for (index, ((source, syntax), semantic)) in catalog
                    .declarations()
                    .iter()
                    .zip(loan.declarations())
                    .zip(self.rows.iter())
                    .enumerate()
                {
                    let source_row_index = u32::try_from(index)
                        .map_err(|_| ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage)?;
                    if syntax.source_row_index() != source_row_index
                        || semantic.source_row_index != source_row_index
                        || source.parameters().len()
                            != usize::try_from(semantic.parameter_count).unwrap_or(usize::MAX)
                    {
                        return Err(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage);
                    }
                    let function = semantic
                        .forest
                        .owner(semantic.owner)
                        .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch)?;
                    if function.function_origin() != semantic.function_origin {
                        return Err(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch);
                    }
                    let parameters = source
                        .parameters()
                        .iter()
                        .map(|parameter| VerifiedResolvedCallableParameterSourceRefV1 {
                            ordinal: parameter.ordinal(),
                            name: parameter.name(),
                            ordinary: parameter.transfer().is_ordinary(),
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice();
                    rows.push(VerifiedResolvedCallableSemanticRowRefV1 {
                        semantic,
                        function,
                        parameters,
                    });
                }
                Ok(callback(VerifiedResolvedCallableSemanticBatchRefV1 {
                    rows: rows.into_boxed_slice(),
                }))
            })
            .map_err(ResolvedCallableSemanticBatchLoanErrorV1::ParserSyntax)?
            .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::ParameterSourceUnavailable)?
    }
}

impl VerifiedResolvedCallableSemanticBatchRefV1<'_> {
    pub(crate) fn declarations(&self) -> &[VerifiedResolvedCallableSemanticRowRefV1<'_>] {
        &self.rows
    }
}

impl VerifiedResolvedCallableSemanticRowRefV1<'_> {
    pub(crate) const fn source_row_index(&self) -> u32 {
        self.semantic.source_row_index
    }

    pub(crate) const fn mode(&self) -> ResolvedCallableDeclarationModeV1 {
        self.semantic.mode
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.semantic.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.semantic.function_origin
    }

    pub(crate) const fn function(&self) -> &VerifiedResolvedFunctionV1 {
        self.function
    }

    pub(crate) fn parameters(&self) -> &[VerifiedResolvedCallableParameterSourceRefV1<'_>] {
        &self.parameters
    }
}

impl VerifiedResolvedCallableParameterSourceRefV1<'_> {
    pub(crate) const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn is_ordinary(self) -> bool {
        self.ordinary
    }

    pub(crate) const fn name(&self) -> &str {
        self.name
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
    ParameterSourceUnavailable,
    ParserSyntax(ParserCallableSyntaxLoanErrorV1),
    MissingSourceRow,
    SourceCoverage,
    LoweringInput(CanonicalLoweringErrorV1),
    OwnerMismatch,
}
