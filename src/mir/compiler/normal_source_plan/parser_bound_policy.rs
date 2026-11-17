//! Pure source-plan policy over the one parser-issued SourceSurface loan.
//!
//! This module never reads the AST, reissues a parser relation, or decides
//! App versus ProgramRuntime. It consumes the already-issued root relation
//! and maps that closed observation to the existing `SealedNormal*` family.


use super::rejection::{
    NormalSourcePlanErrorV1,
};







/// Shared source-policy boundary for the exact App root relation.
///
/// Both canonical source-plan admission and the normal/default root consumer
/// call this helper before any Builder projection. It observes the parser-
/// issued fact only; it does not reopen syntax or issue a second policy.
pub(in crate::mir) fn require_static_main_v1(
    main_box_is_static: bool,
) -> Result<(), NormalSourcePlanErrorV1> {
    if main_box_is_static {
        Ok(())
    } else {
        Err(NormalSourcePlanErrorV1::MainMustBeStatic)
    }
}
