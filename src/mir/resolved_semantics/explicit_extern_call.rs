//! Function-local source identity for canonical explicit `externcall`.
//!
//! The symbol is decoded source text. Runtime ABI admission and return-type
//! selection remain downstream responsibilities.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedExplicitExternCallV1 {
    symbol: Box<str>,
}

impl ResolvedExplicitExternCallV1 {
    pub(super) fn from_source(symbol: impl Into<Box<str>>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }
}
