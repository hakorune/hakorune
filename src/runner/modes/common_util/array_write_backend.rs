//! Physical ArrayElementWrite backend boundary.
//!
//! The canonical MIR owner remains `crate::mir::array_element_write`.  This
//! small runner seam is intentionally behavior-neutral: it is the only place
//! where compatibility projection may be called while the typed selected-C
//! consumer is being installed.

use crate::mir::MirModule;

pub(crate) fn project_for_legacy_backend(module: &MirModule) -> Result<MirModule, String> {
    crate::mir::array_element_write::project_module_to_legacy_calls(module)
}
