//! Total parser-owned normal-root execution relation.
//!
//! `SourceSurface` remains the sole observed-facts owner. This module consumes
//! that surface once and co-seals only the total App/ProgramRuntime relation.
//! It issues no compiler source-plan policy, Builder state, Recipe, or MIR.

mod compatibility;
mod issuer;
mod model;
#[cfg(test)]
mod test_terminal;

pub(crate) use compatibility::{
    ParserNormalRootExecutionCompatibilityClosureV1, ParserNormalRootExecutionCompatibilityRejectV1,
};
pub(in crate::parser) use issuer::ParserNormalRootExecutionIssuerV1;
pub(crate) use model::{
    ParserNormalRootExecutionRoleV1, ParserNormalRootExecutionSourceDispositionV1,
    ParserNormalRootExecutionSourceV1, ParserNormalRootExecutionTerminalClassV1,
};
#[cfg(test)]
pub(in crate::parser) use test_terminal::{
    ParserNormalRootExecutionTestLoanV1, ParserNormalRootExecutionTestTerminalV1,
    ParserRetainedCallableSemanticSourceTestLoanV1,
};

#[cfg(test)]
mod tests;
