use hakorune_mir_defs::SameModuleCallableNamespaceV1;

use crate::mir::{Callee, ValueId};

use super::fault;

pub(crate) fn ordinary_callable_key(
    callee: &Callee,
) -> Result<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, String> {
    match callee {
        Callee::Global(target) => super::super::static_method_key(target)
            .or_else(|| super::super::free_function_key(target))
            .ok_or_else(|| fault("ordinary-call-target")),
        Callee::SameModuleInstance { key, .. }
            if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod =>
        {
            Ok(key.clone())
        }
        Callee::SameModuleInstance { .. } => Err(fault("ordinary-call-instance-namespace")),
        _ => Err(fault("ordinary-call-callee")),
    }
}

pub(crate) fn ordinary_call_receiver(callee: &Callee) -> Result<Option<ValueId>, String> {
    match callee {
        Callee::Global(_) => Ok(None),
        Callee::SameModuleInstance { key, receiver }
            if key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod =>
        {
            Ok(Some(*receiver))
        }
        Callee::SameModuleInstance { .. } => Err(fault("ordinary-call-instance-namespace")),
        _ => Err(fault("ordinary-call-callee")),
    }
}
