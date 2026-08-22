//! Parser-owned whole-Program source authority for the selected normal lane.
//!
//! The authority is AST-free and non-`Clone`. It co-seals generic ProgramBody
//! coverage with the already-issued composite source disposition under one
//! parser invocation witness. Later stages may borrow one paired AST/source
//! cursor, but they cannot issue or reconstruct this authority.

mod issuer;
mod loan;
mod model;
mod transform_guard;

pub(super) use issuer::issue_parser_normal_program_source_authority_v1;
pub(crate) use loan::{
    with_parser_composite_source_loan_from_normal_authority,
    with_parser_normal_program_source_loan, ParserNormalProgramSourceLoanRejectV1,
    ParserNormalProgramSourceLoanV1,
};
pub(crate) use model::{
    ParserNormalProgramBodySourceRowV1, ParserNormalProgramBodySyntaxKindV1,
    ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceAuthorityUnavailableV1,
};
pub(crate) use transform_guard::{
    validate_parser_normal_program_source_transform_v1,
    ParserNormalProgramSourceTransformRejectV1,
};
