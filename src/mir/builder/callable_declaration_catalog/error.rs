use super::CanonicalSameModuleCallableKeyV1;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SameModuleCallableDeclarationCatalogErrorV1 {
    ProgramRequired,
    DuplicateBoxOwner {
        owner: String,
    },
    MethodMustBeFunction {
        owner: String,
        method: String,
    },
    MethodNameMismatch {
        owner: String,
        map_name: String,
        declaration_name: String,
    },
    ParameterDeclarationCardinality {
        key: CanonicalSameModuleCallableKeyV1,
        params: usize,
        declarations: usize,
    },
    ParameterNameMismatch {
        key: CanonicalSameModuleCallableKeyV1,
        index: usize,
    },
    ArityOverflow {
        owner: String,
        method: String,
    },
    DuplicateCanonicalKey(CanonicalSameModuleCallableKeyV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SameModuleCallableDeclarationCatalogSessionErrorV1 {
    QueryBeforeInstall,
    DuplicateInstall,
}

impl fmt::Display for SameModuleCallableDeclarationCatalogSessionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueryBeforeInstall => write!(
                formatter,
                "[mir/callable-catalog/session/query-before-install]"
            ),
            Self::DuplicateInstall => write!(
                formatter,
                "[mir/callable-catalog/session/duplicate-install]"
            ),
        }
    }
}

impl std::error::Error for SameModuleCallableDeclarationCatalogSessionErrorV1 {}
