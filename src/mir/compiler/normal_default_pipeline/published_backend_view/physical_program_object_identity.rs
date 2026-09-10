use hakorune_mir_defs::{CanonicalObjectIdV1, SameModuleCallableNamespaceV1};

use crate::mir::MirModule;

use super::fault;

pub(super) fn ordinary_receiver_object(
    module: &MirModule,
    key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
) -> Result<Option<CanonicalObjectIdV1>, String> {
    if key.namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod {
        return Ok(None);
    }
    let object = module
        .metadata
        .canonical_object_membership
        .as_ref()
        .and_then(|membership| membership.get(key.owner()).copied())
        .ok_or_else(|| fault("ordinary-callee-object-missing"))?;
    Ok(Some(object))
}
