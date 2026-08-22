//! Parser-owned Box declaration syntax captured before postpass transforms.
//!
//! This is source syntax, not Box identity. The enclosing source seal owns
//! the parser brand and declaration site; this payload carries only the
//! declaration spelling needed by a later source cohort issuer.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum ParserBoxDeclarationKindV1 {
    /// The only declaration kind currently admitted by `ParserBoxSourceSealV1`.
    Ordinary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::parser) struct ParserBoxDeclarationSyntaxV1 {
    name: Box<str>,
    kind: ParserBoxDeclarationKindV1,
    is_sync: bool,
}

impl ParserBoxDeclarationSyntaxV1 {
    pub(in crate::parser) fn ordinary(name: String, is_sync: bool) -> Self {
        Self {
            name: name.into_boxed_str(),
            kind: ParserBoxDeclarationKindV1::Ordinary,
            is_sync,
        }
    }

    pub(in crate::parser) fn name(&self) -> &str {
        &self.name
    }

    pub(in crate::parser) fn kind(&self) -> ParserBoxDeclarationKindV1 {
        self.kind
    }

    pub(in crate::parser) fn is_sync(&self) -> bool {
        self.is_sync
    }
}
