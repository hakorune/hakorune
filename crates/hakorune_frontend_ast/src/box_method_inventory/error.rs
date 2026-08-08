use std::fmt;

use crate::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum BoxMethodInventoryErrorV1 {
    DuplicateMethod {
        name: Box<str>,
        first_span: Span,
        duplicate_span: Span,
    },
    NotFunctionDeclaration,
    DeclarationNameMismatch {
        inventory_name: Box<str>,
        declaration_name: Box<str>,
    },
    InvalidSelectedGateProvenance,
    BranchMemberOrdinalCountMismatch {
        methods: usize,
        ordinals: usize,
    },
    NonContiguousSelectedMethodOrdinal {
        expected: u32,
        found: u32,
    },
    EmptySelectedBuildGatePath,
    OrdinalOverflow,
}

impl fmt::Display for BoxMethodInventoryErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateMethod {
                name,
                first_span,
                duplicate_span,
            } => {
                write!(
                    formatter,
                    "duplicate Box method declaration `{name}` at {duplicate_span}; first declared at {first_span}"
                )
            }
            Self::NotFunctionDeclaration => {
                formatter.write_str("Box method entry is not a FunctionDeclaration")
            }
            Self::DeclarationNameMismatch {
                inventory_name,
                declaration_name,
            } => write!(
                formatter,
                "Box method inventory name `{inventory_name}` does not match declaration `{declaration_name}`"
            ),
            Self::InvalidSelectedGateProvenance => formatter.write_str(
                "selected Box member gate contains a method provenance that cannot be source-selected",
            ),
            Self::BranchMemberOrdinalCountMismatch { methods, ordinals } => write!(
                formatter,
                "selected Box member gate has {methods} methods but {ordinals} source-member ordinals",
            ),
            Self::NonContiguousSelectedMethodOrdinal { expected, found } => write!(
                formatter,
                "Box method roundtrip row has selected ordinal {found}; expected contiguous ordinal {expected}",
            ),
            Self::EmptySelectedBuildGatePath => formatter.write_str(
                "selected Box build-gate provenance must contain at least one gate selection",
            ),
            Self::OrdinalOverflow => {
                formatter.write_str("Box method declaration ordinal exceeds u32")
            }
        }
    }
}

impl std::error::Error for BoxMethodInventoryErrorV1 {}
