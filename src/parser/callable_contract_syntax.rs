//! Parser-only normalization for the first callable-contract rune.
//!
//! This module owns syntax spelling, not callable semantics. The enclosing
//! explicit method relation supplies the exact method source site; this row
//! supplies the declaration-local rune coordinate.

use crate::ast::{ASTNode, DeclarationAttrs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CallableContractRuneSiteV1 {
    rune_ordinal: u32,
}

impl CallableContractRuneSiteV1 {
    pub(super) fn rune_ordinal(&self) -> u32 {
        self.rune_ordinal
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableContractSyntaxV1 {
    Query {
        source_site: CallableContractRuneSiteV1,
    },
}

impl CallableContractSyntaxV1 {
    pub(super) fn from_instance_method(declaration: &ASTNode) -> Option<Self> {
        let attrs = match declaration {
            ASTNode::FunctionDeclaration { attrs, .. } => attrs,
            _ => return None,
        };
        Self::from_attrs(attrs)
    }

    fn from_attrs(attrs: &DeclarationAttrs) -> Option<Self> {
        attrs
            .runes
            .iter()
            .enumerate()
            .find_map(|(rune_ordinal, rune)| {
                (rune.name == "CallableContract"
                    && rune.args.first().map(String::as_str) == Some("query"))
                .then_some(Self::Query {
                    source_site: CallableContractRuneSiteV1 {
                        rune_ordinal: rune_ordinal as u32,
                    },
                })
            })
    }

    pub(super) fn source_site(&self) -> &CallableContractRuneSiteV1 {
        match self {
            Self::Query { source_site } => source_site,
        }
    }
}

#[cfg(test)]
#[path = "callable_contract_syntax_tests.rs"]
mod tests;
