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

    /// Stable allocation identity that survives moving the catalog value.
    ///
    /// The catalog itself is moved into the compilation context during
    /// installation, so its struct address is not a valid session identity.
    /// The private `Arc` allocation is the identity that all cloned views
    /// intentionally share.
    pub(in crate::mir) fn identity(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
    }

    pub(in crate::mir) fn is_same(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
