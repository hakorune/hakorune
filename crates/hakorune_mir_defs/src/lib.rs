pub mod call_unified;

pub mod callable_key;

pub mod global_target;
pub use call_unified::{CallFlags, Callee, CalleeBoxKind, MirCall, TypeCertainty};
pub use callable_key::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
pub use global_target::{
    CanonicalBuiltinGlobalV1, CanonicalGlobalTargetComponentV1,
    CanonicalGlobalTargetConstructionErrorV1, CanonicalGlobalTargetV1,
    CanonicalSameModuleGlobalTargetV1,
};
