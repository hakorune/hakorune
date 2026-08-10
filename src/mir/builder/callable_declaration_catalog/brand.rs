//! Opaque allocation identity shared by one declaration catalog and its views.

use std::sync::Arc;

/// Non-semantic identity for one exact declaration-catalog allocation.
///
/// Equal callable keys from separately sealed catalogs are deliberately not
/// interchangeable.  The token is cloneable only so verified views can retain
/// the same allocation identity; callers cannot construct a fresh token.
#[derive(Clone, Debug)]
pub(crate) struct SameModuleCallableCatalogBrandV1(Arc<()>);

impl SameModuleCallableCatalogBrandV1 {
    pub(super) fn fresh() -> Self {
        Self(Arc::new(()))
    }

    pub(in crate::mir) fn is_same(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
