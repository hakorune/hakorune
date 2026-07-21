//! HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0: live shell/header bundle.
//!
//! This capability is short-lived and never stores a Builder.  Shell metadata
//! access and collector-owned header access remain separate loans; callers
//! cannot obtain a module function map or an ambient-module fallback.

use super::function_signature_lookup::FunctionSignatureLookupV1;
use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_lowering_invocation::LoweringHeaderPortV1;
use super::module_lowering_shell::ModuleLoweringShellV1;

pub(in crate::mir::builder) struct ModuleLoweringInvocationAccessPortV1<'shell, 'collector> {
    shell: &'shell mut ModuleLoweringShellV1,
    collector: &'collector mut ModuleDraftCollectorV1,
    _seal: ModuleLoweringInvocationAccessPortSealV1,
}

struct ModuleLoweringInvocationAccessPortSealV1;

impl<'shell, 'collector> ModuleLoweringInvocationAccessPortV1<'shell, 'collector> {
    pub(in crate::mir::builder) fn new(
        shell: &'shell mut ModuleLoweringShellV1,
        collector: &'collector mut ModuleDraftCollectorV1,
    ) -> Self {
        Self {
            shell,
            collector,
            _seal: ModuleLoweringInvocationAccessPortSealV1,
        }
    }

    pub(in crate::mir::builder) fn with_shell<R>(
        &mut self,
        use_shell: impl FnOnce(&mut super::module_lowering_shell::ModuleLoweringShellPortV1<'_>) -> R,
    ) -> R {
        self.shell.with_port(use_shell)
    }

    pub(in crate::mir::builder) fn with_headers<R>(
        &self,
        observe: impl for<'header> FnOnce(&'header LoweringHeaderPortV1<'header>) -> R,
    ) -> R {
        let headers = LoweringHeaderPortV1::from_view(&*self.collector);
        observe(&headers)
    }

    pub(in crate::mir::builder) fn with_finalizer_headers<R>(
        &self,
        finalize: impl for<'header> FnOnce(&'header dyn FunctionSignatureLookupV1) -> R,
    ) -> R {
        self.with_headers(|headers| finalize(headers))
    }
}
