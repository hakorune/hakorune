//! Historical import and test paths; the normal compiler owns the sole view.

#[cfg(test)]
pub(crate) use crate::mir::compiler::published_backend_view::PublishedStaticMethodCFrameV1;
pub(crate) use crate::mir::compiler::published_backend_view::{
    PublishedMirBackendView, PublishedStaticMethodRouteV1,
};

#[cfg(test)]
use crate::mir::compiler::published_backend_view::{
    PublishedCallKindV1, PublishedMirBackendViewErrorV1,
};
#[cfg(test)]
use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, ValueId};
#[cfg(test)]
use hakorune_mir_defs::{CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1};

#[cfg(test)]
#[path = "published_backend_view_tests.rs"]
mod tests;
