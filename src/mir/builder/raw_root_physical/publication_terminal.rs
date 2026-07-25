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

    /// Execute the already-published Raw Main target without exposing the
    /// module.  This is the only VM-reference execution terminal for the Raw
    /// publication owner; entry discovery remains disconnected.
    #[cfg(feature = "vm-reference")]
    pub(in crate::mir) fn execute_exact_vm_entry(
        &self,
        symbol: &str,
    ) -> Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError> {
        let mut interpreter = crate::backend::mir_interpreter::MirInterpreter::new();
        interpreter.execute_function_with_args(&self.module, symbol, &[])
    }
}
