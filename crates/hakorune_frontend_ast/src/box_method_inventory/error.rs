use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoxMethodInventoryErrorV1 {
    DuplicateMethod {
        name: Box<str>,
    },
    NotFunctionDeclaration,
    DeclarationNameMismatch {
        inventory_name: Box<str>,
        declaration_name: Box<str>,
    },
    InvalidSelectedGateProvenance,
    OrdinalOverflow,
}

impl fmt::Display for BoxMethodInventoryErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateMethod { name } => {
                write!(formatter, "duplicate Box method declaration `{name}`")
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
            Self::OrdinalOverflow => {
                formatter.write_str("Box method declaration ordinal exceeds u32")
            }
        }
    }
}

impl std::error::Error for BoxMethodInventoryErrorV1 {}
