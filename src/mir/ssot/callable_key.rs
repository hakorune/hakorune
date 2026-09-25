//! SSOT projection: `Callee` / canonical global target -> cataloged
//! same-module callable key plus receiver-slot count.
//!
//! R6-S0 (boundary S0-A): one schema-side visitor owner for the
//! callee->callable-key projection that was duplicated across
//! `published_backend_view` and `verification/invoke`. This module
//! performs a pure read-side projection only — it never issues calls,
//! never resolves names, and never repairs operands.
//!
//! Fault tags are retained verbatim from the former
//! `published_backend_view` owner to keep the split behavior-neutral.

use hakorune_mir_defs::{
    CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
    CanonicalSameModuleGlobalTargetV1, SameModuleCallableNamespaceV1,
};

use crate::mir::{Callee, ValueId};

/// Physical arity of a cataloged callable key: instance methods and
/// birth constructors reserve one slot the receiver already fills.
pub(crate) fn expected_physical_arity(key: &CanonicalSameModuleCallableKeyV1) -> usize {
    match key.namespace() {
        SameModuleCallableNamespaceV1::FreeFunction => key.arity() as usize,
        SameModuleCallableNamespaceV1::StaticBoxMethod => key.arity() as usize,
        SameModuleCallableNamespaceV1::InstanceBoxMethod
        | SameModuleCallableNamespaceV1::BirthConstructor => key.arity() as usize + 1,
    }
}

/// Project a canonical global target to its static-box method key, if
/// the target is a same-module static box method.
pub(crate) fn static_method_key(
    target: &CanonicalGlobalTargetV1,
) -> Option<CanonicalSameModuleCallableKeyV1> {
    let CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
        owner,
        method,
        arity,
    }) = target
    else {
        return None;
    };
    Some(CanonicalSameModuleCallableKeyV1::static_box_method(
        owner, method, *arity,
    ))
}

/// Project a canonical global target to its free-function key, if the
/// target is a same-module free function.
pub(crate) fn free_function_key(
    target: &CanonicalGlobalTargetV1,
) -> Option<CanonicalSameModuleCallableKeyV1> {
    let CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction {
        name,
        arity,
    }) = target
    else {
        return None;
    };
    Some(CanonicalSameModuleCallableKeyV1::free_function(
        name, *arity,
    ))
}

/// Project a callee to its cataloged ordinary-callable key. Only
/// static-box methods, free functions, and same-module instance-box
/// methods are cataloged ordinary callables; every other callee kind
/// fails fast.
pub(crate) fn ordinary_callable_key(
    callee: &Callee,
) -> Result<CanonicalSameModuleCallableKeyV1, String> {
    match callee {
        Callee::Global(target) => static_method_key(target)
            .or_else(|| free_function_key(target))
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

/// Project a callee to its receiver slot, if it is a cataloged
/// same-module instance-box method.
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

fn fault(reason: &str) -> String {
    format!("[freeze:contract][published-lifecycle-program/{reason}]")
}
