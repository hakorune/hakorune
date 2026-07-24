//! PUBLICATION0 opaque Raw module transition.

use crate::mir::MirModule;

#[derive(Debug)]
pub(in crate::mir) struct RawPublishedModuleV1 {
    module: MirModule,
    _seal: RawPublishedModuleSealV1,
}

#[derive(Debug)]
struct RawPublishedModuleSealV1;

impl RawPublishedModuleV1 {
    pub(in crate::mir::builder) fn from_module(module: MirModule) -> Self {
        Self {
            module,
            _seal: RawPublishedModuleSealV1,
        }
    }

    /// The single compiler-side authority-erasure handoff.
    pub(in crate::mir) fn into_compatibility_module(self) -> MirModule {
        self.module
    }
}
