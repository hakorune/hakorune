use crate::ast::ASTNode;

use super::super::callable_parameter_source::{
    validate_parser_normal_program_source_transform_v1, ParserCallableParameterSourceDispositionV1,
    ParserCompositeTransformRejectV1, ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceTransformRejectV1, ParserNormalRootExecutionSourceDispositionV1,
};
use super::super::callable_source_anchor::PreparedCallableSourceV1;
use super::super::constructor_source_catalog::ParserConstructorSourceCatalogV1;
use super::super::initial_callable_program_source::{
    declaration_at, expected_callable_slots, InitialCallableFinalSlotV1,
};
use super::normal_root_execution_preservation::{
    ParserNormalRootExecutionPreservationIssuerV1, ParserNormalRootExecutionPreservationRejectV1,
};
use super::{PreparedNormalCallableProgramSourceV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FinalCallableProgramSourceRejectV1 {
    CallableCoverage,
    CallableDeclarationChanged { row: usize },
    ConstructorSourceChanged,
    Composite(ParserCompositeTransformRejectV1),
    ProgramSource(ParserNormalProgramSourceTransformRejectV1),
    RootPreservation(ParserNormalRootExecutionPreservationRejectV1),
}

/// Opaque, one-shot input for exact final-transform validation.
///
/// Only the prepared parser product can construct this aggregate. Keeping its
/// fields private prevents callers from splitting and re-pairing source
/// authority, callable rows, constructor coverage, and root execution.
#[derive(Debug)]
pub(super) struct PreparedNormalCallableTransformInputV1 {
    initial_ast: ASTNode,
    sources: Box<[PreparedCallableSourceV1]>,
    slots: Box<[InitialCallableFinalSlotV1]>,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    constructor_source: ParserConstructorSourceCatalogV1,
    normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
}

impl PreparedNormalCallableTransformInputV1 {
    pub(super) fn issue(
        initial_ast: ASTNode,
        sources: Box<[PreparedCallableSourceV1]>,
        slots: Box<[InitialCallableFinalSlotV1]>,
        parameter_source: ParserCallableParameterSourceDispositionV1,
        source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
        constructor_source: ParserConstructorSourceCatalogV1,
        normal_root_execution: ParserNormalRootExecutionSourceDispositionV1,
    ) -> Self {
        Self {
            initial_ast,
            sources,
            slots,
            parameter_source,
            source_authority,
            constructor_source,
            normal_root_execution,
        }
    }

    fn discard_at_named_transform_reject_terminal(self, transformed: Option<ASTNode>) {
        let Self {
            initial_ast,
            sources,
            slots,
            parameter_source,
            source_authority,
            constructor_source,
            normal_root_execution,
        } = self;
        ParserNormalRootExecutionPreservationIssuerV1::discard_at_named_transform_reject_terminal(
            normal_root_execution,
        );
        discard_transform_remainder_at_named_terminal(
            initial_ast,
            transformed,
            sources,
            slots,
            parameter_source,
            source_authority,
            constructor_source,
        );
    }
}

pub(super) fn issue_exact_callable_program_source_v1(
    initial: PreparedNormalCallableProgramSourceV1,
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    issue_callable_program_source_v1(initial, None)
}

#[cfg(test)]
pub(super) fn issue_test_callable_program_source_v1(
    initial: PreparedNormalCallableProgramSourceV1,
    transformed: ASTNode,
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    issue_callable_program_source_v1(initial, Some(transformed))
}

fn issue_callable_program_source_v1(
    initial: PreparedNormalCallableProgramSourceV1,
    transformed: Option<ASTNode>,
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    let input = initial.into_transform_input();
    let transformed_ast = transformed.as_ref().unwrap_or(&input.initial_ast);
    if let Err(error) = validate_parser_normal_program_source_transform_v1(
        &input.source_authority,
        &input.initial_ast,
        transformed_ast,
    ) {
        let error = match error {
            ParserNormalProgramSourceTransformRejectV1::Composite(error) => {
                FinalCallableProgramSourceRejectV1::Composite(error)
            }
            other => FinalCallableProgramSourceRejectV1::ProgramSource(other),
        };
        input.discard_at_named_transform_reject_terminal(transformed);
        return Err(error);
    }
    let transformed_slots = match expected_callable_slots(transformed_ast) {
        Ok(slots) => slots,
        Err(_) => {
            input.discard_at_named_transform_reject_terminal(transformed);
            return Err(FinalCallableProgramSourceRejectV1::CallableCoverage);
        }
    };
    if transformed_slots.as_slice() != input.slots.as_ref()
        || input.sources.len() != input.slots.len()
    {
        drop(transformed_slots);
        input.discard_at_named_transform_reject_terminal(transformed);
        return Err(FinalCallableProgramSourceRejectV1::CallableCoverage);
    }
    for (row, slot) in input.slots.iter().copied().enumerate() {
        if declaration_at(&input.initial_ast, slot) != declaration_at(transformed_ast, slot) {
            drop(transformed_slots);
            input.discard_at_named_transform_reject_terminal(transformed);
            return Err(FinalCallableProgramSourceRejectV1::CallableDeclarationChanged { row });
        }
    }
    if input
        .constructor_source
        .validate_transform(&input.initial_ast, transformed_ast)
        .is_err()
    {
        drop(transformed_slots);
        input.discard_at_named_transform_reject_terminal(transformed);
        return Err(FinalCallableProgramSourceRejectV1::ConstructorSourceChanged);
    }
    drop(transformed_slots);
    let PreparedNormalCallableTransformInputV1 {
        initial_ast,
        sources,
        slots,
        parameter_source,
        source_authority,
        constructor_source,
        normal_root_execution,
    } = input;
    let transformed_ast = transformed.as_ref().unwrap_or(&initial_ast);
    let normal_root_execution =
        match ParserNormalRootExecutionPreservationIssuerV1::seal_after_transform(
            normal_root_execution,
            &source_authority,
            &initial_ast,
            transformed_ast,
        ) {
            Ok(root) => root,
            Err(error) => {
                discard_transform_remainder_at_named_terminal(
                    initial_ast,
                    transformed,
                    sources,
                    slots,
                    parameter_source,
                    source_authority,
                    constructor_source,
                );
                return Err(FinalCallableProgramSourceRejectV1::RootPreservation(error));
            }
        };
    let final_ast = transformed.unwrap_or(initial_ast);
    Ok(VerifiedFinalCallableProgramSourceV1::issue(
        final_ast,
        sources,
        slots,
        parameter_source,
        source_authority,
        constructor_source,
        normal_root_execution,
    ))
}

fn discard_transform_remainder_at_named_terminal(
    initial_ast: ASTNode,
    transformed: Option<ASTNode>,
    sources: Box<[PreparedCallableSourceV1]>,
    slots: Box<[InitialCallableFinalSlotV1]>,
    parameter_source: ParserCallableParameterSourceDispositionV1,
    source_authority: ParserNormalProgramSourceAuthorityDispositionV1,
    constructor_source: ParserConstructorSourceCatalogV1,
) {
    drop((
        initial_ast,
        transformed,
        sources,
        slots,
        parameter_source,
        source_authority,
        constructor_source,
    ));
}

#[cfg(test)]
pub(super) fn reject_foreign_root_authority_for_test(
    source: PreparedNormalCallableProgramSourceV1,
    foreign: PreparedNormalCallableProgramSourceV1,
) -> ParserNormalRootExecutionPreservationRejectV1 {
    let PreparedNormalCallableTransformInputV1 {
        initial_ast,
        sources,
        slots,
        parameter_source,
        source_authority,
        constructor_source,
        normal_root_execution,
    } = source.into_transform_input();
    let PreparedNormalCallableTransformInputV1 {
        initial_ast: foreign_initial_ast,
        sources: foreign_sources,
        slots: foreign_slots,
        parameter_source: foreign_parameter_source,
        source_authority: foreign_authority,
        constructor_source: foreign_constructor_source,
        normal_root_execution: foreign_root_execution,
    } = foreign.into_transform_input();
    ParserNormalRootExecutionPreservationIssuerV1::discard_at_named_transform_reject_terminal(
        foreign_root_execution,
    );
    drop((
        sources,
        slots,
        parameter_source,
        source_authority,
        constructor_source,
        foreign_initial_ast,
        foreign_sources,
        foreign_slots,
        foreign_parameter_source,
        foreign_constructor_source,
    ));
    let rejected = ParserNormalRootExecutionPreservationIssuerV1::seal_after_transform(
        normal_root_execution,
        &foreign_authority,
        &initial_ast,
        &initial_ast,
    )
    .expect_err("foreign invocation must reject before source pairing");
    drop((foreign_authority, initial_ast));
    rejected
}
