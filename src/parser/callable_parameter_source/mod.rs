//! Parser-owned callable parameter source syntax.
//!
//! This module owns the one-shot parameter-list syntax product and its
//! complete sibling source catalog. It issues `Ordinary` syntax only; `Take`,
//! Home demand, resolver binding, Recipe, and MIR remain closed.

mod canonical_script_source_admission;
mod catalog;
mod composite_source;
mod issuer;
mod model;
mod normal_root_execution;
mod normal_source_plan_consumer;
mod normal_source_plan_seed;
mod normal_source_plan_surface;
mod parse_product;
mod parser_invocation_witness;
mod product;
mod retained;
mod script_source_authority;
mod script_source_rows;
mod script_source_rows_model;
mod session;
pub(in crate::parser) mod static_box_source;
#[cfg(test)]
#[path = "static_box_source_tests.rs"]
mod static_box_source_tests;
mod syntax_loan;

pub(in crate::parser) use catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
pub(super) use issuer::project_neutral_parameter_syntax_v1;
pub(crate) use model::ResolverMethodParameterSyntaxV1;
pub(in crate::parser) use model::{
    ParserCallableDeclarationKindV1, ParserCallableParameterDeclarationSourceV1,
};
pub(crate) use normal_root_execution::{
    ParserNormalRootExecutionRoleV1, ParserNormalRootExecutionSourceDispositionV1,
    ParserNormalRootExecutionSourceV1, ParserNormalRootExecutionTerminalClassV1,
};
pub(crate) use normal_source_plan_consumer::{
    AdmittedSourcePlanBoundNormalCallableSourceV1, NormalSourcePlanBoxRefV1,
    NormalSourcePlanRootRefV1, NormalSourcePlanSurfaceLoanErrorV1, NormalSourcePlanSurfaceLoanV1,
    NormalSourcePlanTopLevelRowRefV1, NormalSourcePlanUnsupportedKindV1,
    ParserNormalRootSourcePlanConsumeErrorV1, ParserNormalRootSourcePlanConsumerV1,
    RejectedParserNormalRootSourcePlanConsumptionV1, SourcePlanBoundNormalCallableSourceV1,
};
pub(in crate::parser) use normal_source_plan_seed::{
    ParserNormalSourcePlanSeedDispositionV1, ParserNormalSourcePlanSeedIssueV1,
    ParserNormalSourcePlanSeedV1,
};
pub(in crate::parser) use normal_source_plan_surface::ParserNormalSourcePlanSurfaceV1;
pub(in crate::parser) use parse_product::ParsedCallableParameterListV1;
pub(crate) use parser_invocation_witness::ParserInvocationWitnessV1;
pub(crate) use script_source_rows::{
    CanonicalScriptSourceRowsDispositionV1, CanonicalScriptSourceRowsV1,
};
#[cfg(test)]
#[path = "script_source_rows_tests.rs"]
mod script_source_rows_tests;
pub(crate) use composite_source::ParserCompositeSourceLoanV1;
pub(crate) use composite_source::{
    ParserCompositeIncompleteV1, ParserCompositeIntegrityIssueV1, ParserCompositeOutsideReasonV1,
    ParserCompositeSourceLoanRejectV1, ParserCompositeSourceUnavailableV1,
    ParserCompositeTransformRejectV1,
};
pub(crate) use product::{
    ParsedProgramWithCallableParameterSourceV1, ParserCallableSourceRetentionErrorV1,
    ParserNormalRawVmSourceExtractionErrorV1, ParserNormalRawVmSourceKindV1,
    PreparedParserNormalCompatibilityRawVmV1, PreparedParserNormalRawVmSourceRouteV1,
    PreparedParserNormalSourceBackedRawVmV1, RejectedParserNormalRawVmSourceExtractionV1,
};
pub(crate) use script_source_authority::{
    validate_parser_normal_program_source_transform_v1,
    with_parser_composite_source_loan_from_normal_authority,
    with_parser_normal_program_source_loan, ParserNormalProgramBodySourceRowV1,
    ParserNormalProgramBodySyntaxKindV1, ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1,
    ParserNormalProgramSourceTransformRejectV1,
};
pub(in crate::parser) use session::ParserCallableParameterSourceSessionV1;
pub(in crate::parser) use syntax_loan::borrow_callable_declaration_syntax_v1;
pub(crate) use syntax_loan::{
    ParserCallableDeclarationSyntaxLoanV1, ParserCallableSyntaxLoanErrorV1,
};
#[cfg(test)]
#[path = "normal_source_plan_surface_tests.rs"]
mod normal_source_plan_surface_tests;
#[cfg(test)]
mod retained_tests;
#[cfg(test)]
mod tests;
