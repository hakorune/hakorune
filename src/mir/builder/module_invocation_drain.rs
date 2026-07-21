//! HEADERPORT0 I0-SHELL-I0-S0: disconnected invocation drain vocabulary.
//!
//! This owner is intentionally not wired into a production root yet.  It
//! proves that shell/collector inventory, main/condition policy, and the
//! final function batch are checked before either owner is consumed.

use super::module_draft_collector::{CompletedDraftSignatureViewV1, ModuleDraftCollectorV1};
use super::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
};
use crate::mir::MirModule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionFnPolicyV1 {
    Required,
    Optional,
    Forbidden,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationDrainPreflightErrorV1 {
    ShellAlreadyPublished {
        count: usize,
    },
    InventoryMismatch {
        expected: Box<[String]>,
        actual: Box<[String]>,
    },
    MissingMain,
    MissingConditionFn,
    UnexpectedConditionFn,
}

impl std::fmt::Display for InvocationDrainPreflightErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][invocation_drain] {self:?}")
    }
}

impl std::error::Error for InvocationDrainPreflightErrorV1 {}

/// Expected complete function inventory for one invocation drain.
#[derive(Debug)]
pub(in crate::mir::builder) struct InvocationDrainExpectationV1 {
    inventory: ModuleLoweringShellDrainInventoryV1,
    require_main: bool,
    condition_fn: ConditionFnPolicyV1,
}

impl InvocationDrainExpectationV1 {
    pub(in crate::mir::builder) fn new(
        symbols: impl IntoIterator<Item = String>,
        require_main: bool,
        condition_fn: ConditionFnPolicyV1,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        Ok(Self {
            inventory: ModuleLoweringShellDrainInventoryV1::from_symbols(symbols)?,
            require_main,
            condition_fn,
        })
    }
}

/// One invocation-owned shell and collector.  This is the only S0 object
/// allowed to turn a preflight result into a single-use drain product.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringInvocationDrainOwnerV1 {
    shell: ModuleLoweringShellV1,
    collector: ModuleDraftCollectorV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedInvocationDrainV1 {
    shell: ModuleLoweringShellV1,
    collector: ModuleDraftCollectorV1,
    expectation: InvocationDrainExpectationV1,
    _seal: PreparedInvocationDrainSealV1,
}

#[derive(Debug)]
struct PreparedInvocationDrainSealV1;

impl ModuleLoweringInvocationDrainOwnerV1 {
    pub(in crate::mir::builder) fn new(
        shell: ModuleLoweringShellV1,
        collector: ModuleDraftCollectorV1,
    ) -> Self {
        Self { shell, collector }
    }

    /// All checks run while both owners are still borrowed by this object.
    pub(in crate::mir::builder) fn prepare(
        self,
        expectation: InvocationDrainExpectationV1,
    ) -> Result<PreparedInvocationDrainV1, InvocationDrainPreflightErrorV1> {
        if self.shell.has_published_functions() {
            return Err(InvocationDrainPreflightErrorV1::ShellAlreadyPublished {
                count: self.shell.published_function_count(),
            });
        }

        let mut actual = Vec::new();
        self.collector
            .visit_symbols(&mut |symbol| actual.push(symbol.to_owned()));
        let actual = actual.into_boxed_slice();
        if actual.as_ref() != expectation.inventory.symbols() {
            return Err(InvocationDrainPreflightErrorV1::InventoryMismatch {
                expected: expectation.inventory.symbols().to_vec().into_boxed_slice(),
                actual,
            });
        }

        if expectation.require_main && !self.collector.contains_symbol("main") {
            return Err(InvocationDrainPreflightErrorV1::MissingMain);
        }
        match expectation.condition_fn {
            ConditionFnPolicyV1::Required if !self.collector.contains_symbol("condition_fn") => {
                return Err(InvocationDrainPreflightErrorV1::MissingConditionFn)
            }
            ConditionFnPolicyV1::Forbidden if self.collector.contains_symbol("condition_fn") => {
                return Err(InvocationDrainPreflightErrorV1::UnexpectedConditionFn)
            }
            _ => {}
        }

        Ok(PreparedInvocationDrainV1 {
            shell: self.shell,
            collector: self.collector,
            expectation,
            _seal: PreparedInvocationDrainSealV1,
        })
    }
}

impl PreparedInvocationDrainV1 {
    /// The only S0 terminal.  Preflight has closed every fallible check, so
    /// this consumes both owners and returns the assembled module directly.
    pub(in crate::mir::builder) fn drain(self) -> MirModule {
        let functions = self.collector.into_draft_functions();
        self.shell
            .prepare_drain(self.expectation.inventory)
            .commit_preflighted(functions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1,
    };
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn draft(symbol: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn collector(symbols: &[&str]) -> ModuleDraftCollectorV1 {
        let mut collector = ModuleDraftCollectorV1::default();
        for symbol in symbols {
            let prepared = collector
                .prepare_admission(
                    FunctionDraftKeyV1::LegacySymbol((*symbol).to_owned()),
                    (*symbol).to_owned(),
                    0,
                    DraftPublicationPolicyV1::LegacyReplaceWholePair,
                )
                .unwrap();
            prepared.seal(draft(symbol)).unwrap().collect();
        }
        collector
    }

    #[test]
    fn drain_preflights_complete_inventory_and_required_roots() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".into())).unwrap();
        let owner =
            ModuleLoweringInvocationDrainOwnerV1::new(shell, collector(&["condition_fn", "main"]));
        let expectation = InvocationDrainExpectationV1::new(
            vec!["main".into(), "condition_fn".into()],
            true,
            ConditionFnPolicyV1::Required,
        )
        .unwrap();
        let module = owner.prepare(expectation).unwrap().drain();
        assert_eq!(module.functions.len(), 2);
        assert!(module.functions.contains_key("main"));
        assert!(module.functions.contains_key("condition_fn"));
    }

    #[test]
    fn drain_rejects_missing_main_before_consuming_the_candidate() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".into())).unwrap();
        let owner = ModuleLoweringInvocationDrainOwnerV1::new(shell, collector(&["child/0"]));
        let expectation = InvocationDrainExpectationV1::new(
            vec!["child/0".into()],
            true,
            ConditionFnPolicyV1::Optional,
        )
        .unwrap();
        assert_eq!(
            owner.prepare(expectation).unwrap_err(),
            InvocationDrainPreflightErrorV1::MissingMain
        );
    }

    #[test]
    fn drain_rejects_inventory_mismatch_before_any_shell_mutation() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".into())).unwrap();
        let owner = ModuleLoweringInvocationDrainOwnerV1::new(shell, collector(&["main"]));
        let expectation = InvocationDrainExpectationV1::new(
            vec!["main".into(), "extra".into()],
            true,
            ConditionFnPolicyV1::Forbidden,
        )
        .unwrap();
        assert!(matches!(
            owner.prepare(expectation),
            Err(InvocationDrainPreflightErrorV1::InventoryMismatch { .. })
        ));
    }
}
