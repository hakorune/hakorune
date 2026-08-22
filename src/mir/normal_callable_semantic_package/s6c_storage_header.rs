//! S6C-specific storage-facing header projection.
//!
//! The catalog declaration remains the semantic/source authority. This
//! package-owned projection is only the scoped physical header sibling lent
//! with the installed S6C cohort; it is distinct from the Dynamic projection.

use crate::ast::{DeclarationAttrs, ParamDecl};
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, VerifiedSameModuleCallableDeclarationV1,
};

#[derive(Debug)]
pub(crate) struct VerifiedS6CStorageHeaderProjectionV1 {
    key: CanonicalSameModuleCallableKeyV1,
    params: Box<[String]>,
    param_decls: Box<[ParamDecl]>,
    return_type_name: Option<Box<str>>,
    uses: Box<[String]>,
    attrs: DeclarationAttrs,
}

impl VerifiedS6CStorageHeaderProjectionV1 {
    pub(super) fn from_catalog_declaration(
        declaration: &VerifiedSameModuleCallableDeclarationV1,
    ) -> Self {
        Self {
            key: declaration.key().clone(),
            params: declaration.params().to_vec().into_boxed_slice(),
            param_decls: declaration.param_decls().to_vec().into_boxed_slice(),
            return_type_name: declaration
                .return_type_name()
                .map(str::to_owned)
                .map(Into::into),
            uses: declaration.uses().to_vec().into_boxed_slice(),
            attrs: declaration.attrs().clone(),
        }
    }

    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }

    pub(crate) fn params(&self) -> &[String] {
        &self.params
    }

    pub(crate) fn param_decls(&self) -> &[ParamDecl] {
        &self.param_decls
    }

    pub(crate) fn return_type_name(&self) -> Option<&str> {
        self.return_type_name.as_deref()
    }

    pub(crate) fn uses(&self) -> &[String] {
        &self.uses
    }

    pub(crate) fn attrs(&self) -> &DeclarationAttrs {
        &self.attrs
    }
}
