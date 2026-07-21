//! HEADERPORT0-REENTRANT-TERM0-I0-ACCESS0-S0: disconnected access vocabulary.
//!
//! This module fixes the three capability surfaces before any production
//! borrow is introduced.  It intentionally contains no `MirBuilder`,
//! collector, TLS, or fallback reference.  ACCESS0-P0 will materialize short
//! loans from this contract; production capture/commit remains disconnected.

/// The only surfaces an invocation access port may expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ModuleLoweringAccessSurfaceV1 {
    Header,
    Shell,
    Terminal,
}

/// Read-only completed-header operations.  The backing owner is selected by
/// the later access-port implementation; this vocabulary carries no storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ModuleLoweringHeaderOperationV1 {
    Signature,
    ContainsSymbol,
    SymbolCount,
    VisitSymbols,
}

/// Explicit module-shell operations.  Metadata operations that currently
/// reach through `current_module` are named here so ACCESS0-P0 cannot silently
/// leave a hidden fallback for closure interning or static-data plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ModuleLoweringShellOperationV1 {
    ModuleName,
    GlobalsRead,
    MetadataRead,
    GlobalWrite,
    SourceFileWrite,
    OptimizationLevelWrite,
    InternClosureBody,
    StaticDataPlanLookup,
}

/// Terminal operations are commit-only.  Body lowering and header observation
/// are not terminal operations and must finish before these are borrowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ModuleLoweringTerminalOperationV1 {
    CapturePending,
    PrepareAdmission,
    SealPending,
    CollectPending,
    DrainInvocation,
}

/// Disconnected, non-Clone contract for one future invocation access port.
///
/// The contract is deliberately a specification product, not a live borrow.
/// A live implementation must be introduced only in ACCESS0-P0 and must keep
/// the three loans short-lived and explicitly threaded.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringAccessPortV1 {
    surfaces: Box<[ModuleLoweringAccessSurfaceV1]>,
    header_operations: Box<[ModuleLoweringHeaderOperationV1]>,
    shell_operations: Box<[ModuleLoweringShellOperationV1]>,
    terminal_operations: Box<[ModuleLoweringTerminalOperationV1]>,
    _seal: ModuleLoweringAccessPortSealV1,
}

#[derive(Debug)]
struct ModuleLoweringAccessPortSealV1;

impl ModuleLoweringAccessPortV1 {
    /// Create the sole disconnected vocabulary used by ACCESS0-S0 fixtures.
    pub(in crate::mir::builder) fn disconnected() -> Self {
        Self {
            surfaces: Box::new([
                ModuleLoweringAccessSurfaceV1::Header,
                ModuleLoweringAccessSurfaceV1::Shell,
                ModuleLoweringAccessSurfaceV1::Terminal,
            ]),
            header_operations: Box::new([
                ModuleLoweringHeaderOperationV1::Signature,
                ModuleLoweringHeaderOperationV1::ContainsSymbol,
                ModuleLoweringHeaderOperationV1::SymbolCount,
                ModuleLoweringHeaderOperationV1::VisitSymbols,
            ]),
            shell_operations: Box::new([
                ModuleLoweringShellOperationV1::ModuleName,
                ModuleLoweringShellOperationV1::GlobalsRead,
                ModuleLoweringShellOperationV1::MetadataRead,
                ModuleLoweringShellOperationV1::GlobalWrite,
                ModuleLoweringShellOperationV1::SourceFileWrite,
                ModuleLoweringShellOperationV1::OptimizationLevelWrite,
                ModuleLoweringShellOperationV1::InternClosureBody,
                ModuleLoweringShellOperationV1::StaticDataPlanLookup,
            ]),
            terminal_operations: Box::new([
                ModuleLoweringTerminalOperationV1::CapturePending,
                ModuleLoweringTerminalOperationV1::PrepareAdmission,
                ModuleLoweringTerminalOperationV1::SealPending,
                ModuleLoweringTerminalOperationV1::CollectPending,
                ModuleLoweringTerminalOperationV1::DrainInvocation,
            ]),
            _seal: ModuleLoweringAccessPortSealV1,
        }
    }

    pub(in crate::mir::builder) fn surfaces(&self) -> &[ModuleLoweringAccessSurfaceV1] {
        &self.surfaces
    }

    pub(in crate::mir::builder) fn header_operations(&self) -> &[ModuleLoweringHeaderOperationV1] {
        &self.header_operations
    }

    pub(in crate::mir::builder) fn shell_operations(&self) -> &[ModuleLoweringShellOperationV1] {
        &self.shell_operations
    }

    pub(in crate::mir::builder) fn terminal_operations(
        &self,
    ) -> &[ModuleLoweringTerminalOperationV1] {
        &self.terminal_operations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_port_contract_has_exact_three_surfaces() {
        let port = ModuleLoweringAccessPortV1::disconnected();
        assert_eq!(
            port.surfaces(),
            &[
                ModuleLoweringAccessSurfaceV1::Header,
                ModuleLoweringAccessSurfaceV1::Shell,
                ModuleLoweringAccessSurfaceV1::Terminal,
            ]
        );
    }

    #[test]
    fn shell_contract_names_current_metadata_holes() {
        let port = ModuleLoweringAccessPortV1::disconnected();
        assert!(port
            .shell_operations()
            .contains(&ModuleLoweringShellOperationV1::InternClosureBody));
        assert!(port
            .shell_operations()
            .contains(&ModuleLoweringShellOperationV1::StaticDataPlanLookup));
    }

    #[test]
    fn terminal_contract_is_commit_only() {
        let port = ModuleLoweringAccessPortV1::disconnected();
        assert_eq!(port.header_operations().len(), 4);
        assert_eq!(port.terminal_operations().len(), 5);
        assert!(port
            .terminal_operations()
            .contains(&ModuleLoweringTerminalOperationV1::CapturePending));
    }
}
