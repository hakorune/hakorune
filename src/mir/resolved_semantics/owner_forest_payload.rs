//! Shared forest payload boundary.
//!
//! The current producer still seals Function/Lambda owners only. Keeping the
//! payload enum here lets the forest accept a future Script wrapper without
//! creating a second forest authority or changing the existing Function API.

use super::product::{VerifiedResolvedFunctionV1, VerifiedResolvedScriptV1};

#[derive(Debug)]
pub(crate) enum VerifiedSemanticOwnerProductV1 {
    Function(VerifiedResolvedFunctionV1),
    Script(VerifiedResolvedScriptV1),
}

impl VerifiedSemanticOwnerProductV1 {
    pub(crate) fn into_function(self) -> Option<VerifiedResolvedFunctionV1> {
        match self {
            Self::Function(product) => Some(product),
            Self::Script(_) => None,
        }
    }

    pub(crate) fn as_function(&self) -> Option<&VerifiedResolvedFunctionV1> {
        match self {
            Self::Function(product) => Some(product),
            Self::Script(_) => None,
        }
    }
}
