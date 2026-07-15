//! Co-seal the source return ABI with the existing terminal and coverage.

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::error::{
    stop, AnalysisResultV1, TrivialProfileContractErrorV1, TrivialProfileStopReasonV1,
    TrivialProfileStopSiteV1,
};
use super::product::{
    TrivialProfileCoverageSubjectV1, TrivialRepresentationV1, TrivialTerminalProfileV1,
    VerifiedTrivialFunctionReturnV1, VerifiedTrivialProfileCoverageV1,
};

pub(super) fn seal_function_return_v1(
    owner: FunctionOwnerIdV1,
    requested: Option<ExactTrivialReturnAbiV1>,
    terminal: &TrivialTerminalProfileV1,
    coverage: &VerifiedTrivialProfileCoverageV1,
) -> AnalysisResultV1<Option<VerifiedTrivialFunctionReturnV1>> {
    let Some(abi) = requested else {
        return Ok(None);
    };
    let TrivialTerminalProfileV1::ExplicitValue {
        statement,
        representation: TrivialRepresentationV1::InlineI64,
        ..
    } = terminal
    else {
        return stop(
            TrivialProfileStopSiteV1::Owner(owner),
            TrivialProfileStopReasonV1::TypedSignatureOutsideProfile,
        );
    };
    let matching_terminal_rows = coverage
        .ordered_subjects()
        .iter()
        .filter(|subject| {
            matches!(
                subject,
                TrivialProfileCoverageSubjectV1::ExplicitValueTerminal(site)
                    if site == statement
            )
        })
        .count();
    if matching_terminal_rows != 1 {
        return Err(TrivialProfileContractErrorV1::FunctionReturnCoverageMismatch.into());
    }
    Ok(Some(VerifiedTrivialFunctionReturnV1::new(abi)))
}
