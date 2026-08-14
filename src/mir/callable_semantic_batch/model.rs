use std::sync::Arc;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedFunctionV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::CanonicalLoweringErrorV1;
use crate::parser::{CallableDeclarationIdentityV1, CallableMethodSourceObservationV1};
use crate::parser::{FinalCallableSemanticSyntaxLoanErrorV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedCallableDeclarationModeV1 {
    TopLevel,
    StaticBoxMethod,
    InstanceBoxMethod,
}

#[derive(Debug)]
pub(super) struct VerifiedResolvedCallableSemanticRowV1 {
    pub(super) batch_slot: u32,
    pub(super) identity: CallableDeclarationIdentityV1,
    pub(super) mode: ResolvedCallableDeclarationModeV1,
    pub(super) parameter_count: u32,
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) function_origin: FunctionOriginV1,
    pub(super) forest: VerifiedSemanticOwnerForestV1,
    pub(super) body_shape: Arc<VerifiedResolvedBodyShapeInventoryV1>,
    pub(super) projection: VerifiedSourceProjectionV1,
    pub(super) method_source_observation: Option<CallableMethodSourceObservationV1>,
}

#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableSemanticBatchV1 {
    pub(super) source: VerifiedFinalCallableProgramSourceV1,
    pub(super) rows: Box<[VerifiedResolvedCallableSemanticRowV1]>,
}

/// Selected-callable identity transport for downstream scoped loans.
///
/// This is a copied opaque identity view, not a lookup key or semantic
/// authority.  It carries no AST, batch slot, or resolver allocation handle;
/// the batch row remains the sole issuer.
#[derive(Debug, Clone)]
pub(crate) struct VerifiedResolvedCallableSourceIdentityV1 {
    identity: CallableDeclarationIdentityV1,
    mode: ResolvedCallableDeclarationModeV1,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    method_source_observation: Option<CallableMethodSourceObservationV1>,
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
    body_shape: &'batch Arc<VerifiedResolvedBodyShapeInventoryV1>,
    parameters: Option<Box<[VerifiedResolvedCallableParameterSourceRefV1<'batch>]>>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedResolvedCallableParameterSourceRefV1<'batch> {
    ordinal: u32,
    name: &'batch str,
    declared_type_name: Option<&'batch str>,
    ordinary: bool,
}

impl VerifiedResolvedCallableSemanticBatchV1 {
    pub(crate) fn source_ast(&self) -> &crate::ast::ASTNode {
        self.source.ast()
    }

    pub(crate) fn declarations(
        &self,
    ) -> impl ExactSizeIterator<Item = VerifiedResolvedCallableSemanticDeclarationRefV1<'_>> {
        self.rows
            .iter()
            .map(|row| VerifiedResolvedCallableSemanticDeclarationRefV1 { row })
    }

    pub(crate) fn with_lowering_input<R>(
        &self,
        batch_slot: u32,
        callback: impl for<'source> FnOnce(ResolvedFunctionLoweringInputV1<'source>) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        self.with_lowering_input_and_method_source(batch_slot, |input, _| callback(input))
    }

    pub(crate) fn with_lowering_input_and_source_identity<R>(
        &self,
        batch_slot: u32,
        callback: impl for<'source> FnOnce(
            ResolvedFunctionLoweringInputV1<'source>,
            VerifiedResolvedCallableSourceIdentityV1,
        ) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        let index = usize::try_from(batch_slot)
            .map_err(|_| ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;
        let semantic = self
            .rows
            .get(index)
            .filter(|row| row.batch_slot == batch_slot)
            .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;
        let identity = VerifiedResolvedCallableSourceIdentityV1 {
            identity: semantic.identity.clone(),
            mode: semantic.mode,
            owner: semantic.owner,
            function_origin: semantic.function_origin,
            method_source_observation: semantic.method_source_observation.clone(),
        };
        self.with_lowering_input(batch_slot, |input| callback(input, identity))
    }

    pub(crate) fn with_lowering_input_and_method_source<R>(
        &self,
        batch_slot: u32,
        callback: impl for<'source> FnOnce(
            ResolvedFunctionLoweringInputV1<'source>,
            Option<CallableMethodSourceObservationV1>,
        ) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        let index = usize::try_from(batch_slot)
            .map_err(|_| ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;
        let semantic = self
            .rows
            .get(index)
            .filter(|row| row.batch_slot == batch_slot)
            .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)?;

        self.source
            .with_callable_semantic_syntax(|loan| {
                let syntax = loan
                    .rows()
                    .get(index)
                    .filter(|row| row.batch_slot() == batch_slot)
                    .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage)?;
                let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
                    syntax.declaration(),
                    &semantic.forest,
                    &semantic.projection,
                )
                .map_err(ResolvedCallableSemanticBatchLoanErrorV1::LoweringInput)?;
                if input.owner() != semantic.owner {
                    return Err(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch);
                }
                Ok(callback(input, semantic.method_source_observation.clone()))
            })
            .map_err(ResolvedCallableSemanticBatchLoanErrorV1::ParserSyntax)?
    }

    pub(crate) fn with_declaration_semantics<R>(
        &self,
        callback: impl for<'source> FnOnce(VerifiedResolvedCallableSemanticBatchRefV1<'source>) -> R,
    ) -> Result<R, ResolvedCallableSemanticBatchLoanErrorV1> {
        self.source
            .with_callable_semantic_syntax(|loan| {
                if loan.rows().len() != self.rows.len() {
                    return Err(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage);
                }
                let mut rows = Vec::with_capacity(self.rows.len());
                for (index, (syntax, semantic)) in
                    loan.rows().iter().zip(self.rows.iter()).enumerate()
                {
                    let batch_slot = u32::try_from(index)
                        .map_err(|_| ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage)?;
                    if syntax.batch_slot() != batch_slot
                        || semantic.batch_slot != batch_slot
                        || !syntax.identity().same_as(&semantic.identity)
                        || syntax.parameters().is_some_and(|parameters| {
                            parameters.len()
                                != usize::try_from(semantic.parameter_count).unwrap_or(usize::MAX)
                        })
                    {
                        return Err(ResolvedCallableSemanticBatchLoanErrorV1::SourceCoverage);
                    }
                    let function = semantic
                        .forest
                        .owner(semantic.owner)
                        .ok_or(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch)?;
                    if semantic.body_shape.owner() != semantic.owner {
                        return Err(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch);
                    }
                    if function.function_origin() != semantic.function_origin {
                        return Err(ResolvedCallableSemanticBatchLoanErrorV1::OwnerMismatch);
                    }
                    let parameters = syntax.parameters().map(|parameters| {
                        parameters
                            .iter()
                            .map(|parameter| VerifiedResolvedCallableParameterSourceRefV1 {
                                ordinal: parameter.ordinal(),
                                name: parameter.name(),
                                declared_type_name: parameter.declared_type_name(),
                                ordinary: parameter.is_ordinary(),
                            })
                            .collect::<Vec<_>>()
                            .into_boxed_slice()
                    });
                    rows.push(VerifiedResolvedCallableSemanticRowRefV1 {
                        semantic,
                        function,
                        body_shape: &semantic.body_shape,
                        parameters,
                    });
                }
                Ok(callback(VerifiedResolvedCallableSemanticBatchRefV1 {
                    rows: rows.into_boxed_slice(),
                }))
            })
            .map_err(ResolvedCallableSemanticBatchLoanErrorV1::ParserSyntax)?
    }
}

impl VerifiedResolvedCallableSourceIdentityV1 {
    pub(crate) fn identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> ResolvedCallableDeclarationModeV1 {
        self.mode
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) fn method_source_observation(&self) -> Option<&CallableMethodSourceObservationV1> {
        self.method_source_observation.as_ref()
    }
}

impl VerifiedResolvedCallableSemanticBatchRefV1<'_> {
    pub(crate) fn declarations(&self) -> &[VerifiedResolvedCallableSemanticRowRefV1<'_>] {
        &self.rows
    }
}

impl VerifiedResolvedCallableSemanticRowRefV1<'_> {
    pub(crate) const fn batch_slot(&self) -> u32 {
        self.semantic.batch_slot
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

    pub(crate) fn body_shape(&self) -> &VerifiedResolvedBodyShapeInventoryV1 {
        self.body_shape.as_ref()
    }

    pub(crate) fn body_shape_arc(&self) -> Arc<VerifiedResolvedBodyShapeInventoryV1> {
        Arc::clone(self.body_shape)
    }

    pub(crate) fn parameters(&self) -> Option<&[VerifiedResolvedCallableParameterSourceRefV1<'_>]> {
        self.parameters.as_deref()
    }

    pub(super) fn source_ledger(
        &self,
    ) -> Result<
        crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
        crate::mir::resolved_semantics::CallableSourceLedgerRejectV1,
    > {
        self.semantic.forest.callable_source_ledger(self.owner())
    }

    // The source-bound S6C producer is intentionally caller-zero until its
    // Facts/Recipe row lands; keep the scoped transport warning-free.
    #[allow(dead_code)]
    pub(crate) fn with_source_ledger<R>(
        &self,
        callback: impl for<'source> FnOnce(
            crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'source>,
        ) -> R,
    ) -> Result<R, crate::mir::resolved_semantics::CallableSourceLedgerRejectV1> {
        self.semantic
            .forest
            .callable_source_ledger(self.owner())
            .map(callback)
    }
}

impl VerifiedResolvedCallableParameterSourceRefV1<'_> {
    pub(crate) const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn is_ordinary(self) -> bool {
        self.ordinary
    }

    pub(crate) const fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name
    }

    pub(crate) const fn name(&self) -> &str {
        self.name
    }
}

impl VerifiedResolvedCallableSemanticDeclarationRefV1<'_> {
    pub(crate) fn same_declaration_identity(
        self,
        identity: &CallableDeclarationIdentityV1,
    ) -> bool {
        self.row.identity.same_as(identity)
    }

    pub(crate) const fn batch_slot(self) -> u32 {
        self.row.batch_slot
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
    ParserSyntax(FinalCallableSemanticSyntaxLoanErrorV1),
    MissingSourceRow,
    SourceCoverage,
    LoweringInput(CanonicalLoweringErrorV1),
    OwnerMismatch,
}
