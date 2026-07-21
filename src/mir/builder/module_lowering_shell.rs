//! I0-SHELL-S0: function-empty module shell and one-shot drain vocabulary.
//!
//! The shell is intentionally disconnected from production lowering. It owns
//! module-level name/globals/metadata state while the invocation collector
//! owns every function draft and completed-function header.

use std::collections::BTreeSet;

use crate::mir::{MirFunction, MirModule};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleLoweringShellErrorV1 {
    FunctionMapNotEmpty { count: usize },
    DuplicateFunction { symbol: String },
}

impl std::fmt::Display for ModuleLoweringShellErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][module_shell] {self:?}")
    }
}

impl std::error::Error for ModuleLoweringShellErrorV1 {}

/// A module shell whose function map is empty until the one final drain.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringShellV1 {
    module: MirModule,
    _seal: ModuleLoweringShellSealV1,
}

#[derive(Debug)]
struct ModuleLoweringShellSealV1;

/// Non-Clone, single-use drain capability for one shell.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedModuleLoweringShellDrainV1 {
    shell: ModuleLoweringShellV1,
    _seal: ModuleLoweringShellDrainSealV1,
}

#[derive(Debug)]
struct ModuleLoweringShellDrainSealV1;

impl ModuleLoweringShellV1 {
    pub(in crate::mir::builder) fn from_empty_module(
        module: MirModule,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let count = module.functions.len();
        if count != 0 {
            return Err(ModuleLoweringShellErrorV1::FunctionMapNotEmpty { count });
        }
        Ok(Self {
            module,
            _seal: ModuleLoweringShellSealV1,
        })
    }

    pub(in crate::mir::builder) fn name(&self) -> &str {
        &self.module.name
    }

    pub(in crate::mir::builder) fn has_published_functions(&self) -> bool {
        !self.module.functions.is_empty()
    }

    pub(in crate::mir::builder) fn prepare_drain(self) -> PreparedModuleLoweringShellDrainV1 {
        PreparedModuleLoweringShellDrainV1 {
            shell: self,
            _seal: ModuleLoweringShellDrainSealV1,
        }
    }
}

impl PreparedModuleLoweringShellDrainV1 {
    /// Preflight every function symbol before mutating the shell.
    pub(in crate::mir::builder) fn commit(
        mut self,
        functions: Vec<MirFunction>,
    ) -> Result<MirModule, ModuleLoweringShellErrorV1> {
        let mut symbols = BTreeSet::new();
        for function in &functions {
            let symbol = function.signature.name.clone();
            if !symbols.insert(symbol.clone()) {
                return Err(ModuleLoweringShellErrorV1::DuplicateFunction { symbol });
            }
        }

        for function in functions {
            self.shell.module.add_function(function);
        }
        Ok(self.shell.module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn function(symbol: &str) -> MirFunction {
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

    #[test]
    fn shell_rejects_an_already_published_function_map() {
        let mut module = MirModule::new("main".to_owned());
        module.add_function(function("already/0"));
        assert_eq!(
            ModuleLoweringShellV1::from_empty_module(module).unwrap_err(),
            ModuleLoweringShellErrorV1::FunctionMapNotEmpty { count: 1 }
        );
    }

    #[test]
    fn shell_drain_commits_one_function_batch() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap();
        assert_eq!(shell.name(), "main");
        assert!(!shell.has_published_functions());
        let module = shell
            .prepare_drain()
            .commit(vec![function("child/0")])
            .unwrap();
        assert_eq!(module.functions.len(), 1);
        assert!(module.functions.contains_key("child/0"));
    }

    #[test]
    fn shell_drain_rejects_duplicate_symbols_before_commit() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap();
        let error = shell
            .prepare_drain()
            .commit(vec![function("child/0"), function("child/0")])
            .unwrap_err();
        assert_eq!(
            error,
            ModuleLoweringShellErrorV1::DuplicateFunction {
                symbol: "child/0".to_owned()
            }
        );
    }
}
