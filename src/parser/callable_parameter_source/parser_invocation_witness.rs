//! Opaque parser-invocation identity used by source handoff boundaries.
//!
//! The underlying parser brand stays private to the parser authority owner.
//! Consumers may compare two witnesses, but cannot issue or reconstruct the
//! brand from an AST, path, digest, or Builder state.

use crate::parser::source_authority::ParserInvocationBrandV1;

#[derive(Debug, Clone)]
pub(crate) struct ParserInvocationWitnessV1(ParserInvocationBrandV1);

impl ParserInvocationWitnessV1 {
    pub(super) fn from_brand(brand: &ParserInvocationBrandV1) -> Self {
        Self(brand.clone())
    }

    pub(crate) fn same_as(&self, other: &Self) -> bool {
        self.0.same_as(&other.0)
    }
}
