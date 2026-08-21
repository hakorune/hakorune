//! Parser-owned composite source preservation for the first Script cohort.
//!
//! This module owns only source presence and exact transform preservation. It
//! does not resolve the receiver, select a target, or issue A/C/Recipe/MIR
//! meaning.

mod issuer;
mod model;
mod transform_guard;

pub(crate) use issuer::issue_parser_composite_source_v1;
pub(crate) use model::{
    ParserCompositeSourceDispositionV1, ParserCompositeSourceUnavailableV1,
    ParserCompositeTransformRejectV1,
};
pub(crate) use transform_guard::validate_parser_composite_transform_v1;
