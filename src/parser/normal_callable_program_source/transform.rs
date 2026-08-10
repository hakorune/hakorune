use crate::ast::ASTNode;

use super::super::initial_callable_program_source::{declaration_at, expected_callable_slots};
use super::{PreparedNormalCallableProgramSourceV1, VerifiedFinalCallableProgramSourceV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FinalCallableProgramSourceRejectV1 {
    CallableCoverage,
    CallableDeclarationChanged { row: usize },
}

pub(crate) fn issue_final_callable_program_source_v1(
    initial: PreparedNormalCallableProgramSourceV1,
    transformed: ASTNode,
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    let (initial_ast, sources, slots, parameter_source) = initial.into_transform_parts();
    let transformed_slots = expected_callable_slots(&transformed)
        .map_err(|_| FinalCallableProgramSourceRejectV1::CallableCoverage)?;
    if transformed_slots.as_slice() != slots.as_ref() || sources.len() != slots.len() {
        return Err(FinalCallableProgramSourceRejectV1::CallableCoverage);
    }
    for (row, slot) in slots.iter().copied().enumerate() {
        if declaration_at(&initial_ast, slot) != declaration_at(&transformed, slot) {
            return Err(FinalCallableProgramSourceRejectV1::CallableDeclarationChanged { row });
        }
    }
    Ok(VerifiedFinalCallableProgramSourceV1::issue(
        transformed,
        sources,
        slots,
        parameter_source,
    ))
}
