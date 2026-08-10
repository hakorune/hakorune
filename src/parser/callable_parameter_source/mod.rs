//! Parser-owned callable parameter source syntax.
//!
//! This module owns the one-shot parameter-list syntax product and its
//! complete sibling source catalog. It issues `Ordinary` syntax only; `Take`,
//! Home demand, resolver binding, Recipe, and MIR remain closed.

mod catalog;
mod issuer;
mod model;
mod parse_product;
mod product;
mod session;

pub(super) use issuer::project_neutral_parameter_syntax_v1;
pub(in crate::parser) use model::ParserCallableDeclarationKindV1;
pub(crate) use model::ResolverMethodParameterSyntaxV1;
pub(in crate::parser) use parse_product::ParsedCallableParameterListV1;
pub(crate) use product::ParsedProgramWithCallableParameterSourceV1;
pub(in crate::parser) use session::ParserCallableParameterSourceSessionV1;

#[cfg(test)]
mod tests;
