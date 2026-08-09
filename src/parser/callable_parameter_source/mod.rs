//! Parser-owned callable parameter source syntax.
//!
//! This module is the physical owner selected before the transfer-source I0.
//! It currently preserves the neutral parameter name/type projection only;
//! transfer syntax, parser provenance, exact declaration identity, and the
//! complete source catalog remain closed until their issuing row lands.

mod issuer;
mod model;

pub(super) use issuer::project_neutral_parameter_syntax_v1;
pub(crate) use model::ResolverMethodParameterSyntaxV1;

#[cfg(test)]
mod tests;
