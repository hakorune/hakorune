//! Final parser source product for the bounded R6-S3 slice.
//!
//! This module owns the post-prune/post-delegate boundary. The ordinary
//! parser transaction issues only a prepared payload; the postpass product is
//! the only owner that can compare that payload with the final AST inventory
//! and issue the non-Clone source seal.

mod finalize;
mod gate_projection;
mod model;

pub(super) use finalize::map_error;
pub(super) use model::{
    OpenParserPostpassProductV1, ParsedProgramWithSourceV1, ParserBoxSourceSealV1,
    PreparedBoxSourceSealV1,
};

#[cfg(test)]
use crate::ast::{ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodProvenanceV1};
#[cfg(test)]
use crate::parser::NyashParser;
#[cfg(test)]
use model::SourceSealFinalizationErrorV1;

#[cfg(test)]
#[path = "../source_seal_delegate_tests.rs"]
mod source_seal_delegate_tests;

#[cfg(test)]
#[path = "../source_seal_misc_tests.rs"]
mod source_seal_misc_tests;

#[cfg(test)]
#[path = "../source_seal_finalizer_tests.rs"]
mod source_seal_finalizer_tests;
