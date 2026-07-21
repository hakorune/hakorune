//! I0-SHELL-S0: function-empty module shell and one-shot drain vocabulary.
//!
//! The shell is intentionally disconnected from production lowering. It owns
//! module-level name/globals/metadata state while the invocation collector
//! owns every function draft and completed-function header.

use std::collections::BTreeSet;

use crate::mir::function::ModuleMetadata;
use crate::mir::{ConstValue, MirFunction, MirModule};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum ModuleLoweringShellErrorV1 {
    FunctionMapNotEmpty {
        count: usize,
    },
    DuplicateFunction {
        symbol: String,
    },
    DuplicateInventorySymbol {
        symbol: String,
    },
    InventoryMismatch {
        expected: Box<[String]>,
        actual: Box<[String]>,
    },
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

/// Narrow metadata/global capability; it never exposes the backing module or
/// its function map.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringShellPortV1<'shell> {
    shell: &'shell mut ModuleLoweringShellV1,
    _seal: ModuleLoweringShellPortSealV1,
}

#[derive(Debug)]
struct ModuleLoweringShellPortSealV1;

/// Deterministic symbol inventory prepared before a collector-to-shell drain.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringShellDrainInventoryV1 {
    symbols: Box<[String]>,
    _seal: ModuleLoweringShellDrainInventorySealV1,
}

#[derive(Debug)]
struct ModuleLoweringShellDrainInventorySealV1;

/// Non-Clone, single-use drain capability for one shell.
#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedModuleLoweringShellDrainV1 {
    shell: ModuleLoweringShellV1,
    inventory: ModuleLoweringShellDrainInventoryV1,
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

    pub(in crate::mir::builder) fn with_port<R>(
        &mut self,
        use_port: impl FnOnce(&mut ModuleLoweringShellPortV1<'_>) -> R,
    ) -> R {
        let mut port = ModuleLoweringShellPortV1 {
            shell: self,
            _seal: ModuleLoweringShellPortSealV1,
        };
        use_port(&mut port)
    }

    pub(in crate::mir::builder) fn prepare_drain(
        self,
        inventory: ModuleLoweringShellDrainInventoryV1,
    ) -> PreparedModuleLoweringShellDrainV1 {
        PreparedModuleLoweringShellDrainV1 {
            shell: self,
            inventory,
            _seal: ModuleLoweringShellDrainSealV1,
        }
    }
}

impl ModuleLoweringShellDrainInventoryV1 {
    pub(in crate::mir::builder) fn from_symbols(
        symbols: impl IntoIterator<Item = String>,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let mut symbols = symbols.into_iter().collect::<Vec<_>>();
        symbols.sort();
        for pair in symbols.windows(2) {
            if pair[0] == pair[1] {
                return Err(ModuleLoweringShellErrorV1::DuplicateInventorySymbol {
                    symbol: pair[0].clone(),
                });
            }
        }
        Ok(Self {
            symbols: symbols.into_boxed_slice(),
            _seal: ModuleLoweringShellDrainInventorySealV1,
        })
    }
}

impl ModuleLoweringShellPortV1<'_> {
    pub(in crate::mir::builder) fn module_name(&self) -> &str {
        &self.shell.module.name
    }

    pub(in crate::mir::builder) fn globals(
        &self,
    ) -> &std::collections::HashMap<String, ConstValue> {
        &self.shell.module.globals
    }

    pub(in crate::mir::builder) fn metadata(&self) -> &ModuleMetadata {
        &self.shell.module.metadata
    }

    pub(in crate::mir::builder) fn set_global(&mut self, name: String, value: ConstValue) {
        self.shell.module.globals.insert(name, value);
    }

    pub(in crate::mir::builder) fn set_source_file(&mut self, source_file: Option<String>) {
        self.shell.module.metadata.source_file = source_file;
    }

    pub(in crate::mir::builder) fn set_optimization_level(&mut self, level: u32) {
        self.shell.module.metadata.optimization_level = level;
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
        let actual = symbols.into_iter().collect::<Vec<_>>().into_boxed_slice();
        if actual != self.inventory.symbols {
            return Err(ModuleLoweringShellErrorV1::InventoryMismatch {
                expected: self.inventory.symbols,
                actual,
            });
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
        let inventory =
            ModuleLoweringShellDrainInventoryV1::from_symbols(vec!["child/0".to_owned()]).unwrap();
        let module = shell
            .prepare_drain(inventory)
            .commit(vec![function("child/0")])
            .unwrap();
        assert_eq!(module.functions.len(), 1);
        assert!(module.functions.contains_key("child/0"));
    }

    #[test]
    fn shell_drain_rejects_duplicate_symbols_before_commit() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap();
        let inventory =
            ModuleLoweringShellDrainInventoryV1::from_symbols(vec!["child/0".to_owned()]).unwrap();
        let error = shell
            .prepare_drain(inventory)
            .commit(vec![function("child/0"), function("child/0")])
            .unwrap_err();
        assert_eq!(
            error,
            ModuleLoweringShellErrorV1::DuplicateFunction {
                symbol: "child/0".to_owned()
            }
        );
    }

    #[test]
    fn shell_drain_rejects_inventory_function_mismatch_before_commit() {
        let shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap();
        let inventory =
            ModuleLoweringShellDrainInventoryV1::from_symbols(vec!["expected/0".to_owned()])
                .unwrap();
        let error = shell
            .prepare_drain(inventory)
            .commit(vec![function("actual/0")])
            .unwrap_err();
        assert_eq!(
            error,
            ModuleLoweringShellErrorV1::InventoryMismatch {
                expected: vec!["expected/0".to_owned()].into_boxed_slice(),
                actual: vec!["actual/0".to_owned()].into_boxed_slice(),
            }
        );
    }

    #[test]
    fn shell_drain_inventory_rejects_duplicate_symbols_before_commit() {
        let error = ModuleLoweringShellDrainInventoryV1::from_symbols(vec![
            "child/0".to_owned(),
            "child/0".to_owned(),
        ])
        .unwrap_err();
        assert_eq!(
            error,
            ModuleLoweringShellErrorV1::DuplicateInventorySymbol {
                symbol: "child/0".to_owned()
            }
        );
    }

    #[test]
    fn shell_metadata_port_is_the_only_narrow_metadata_write_surface() {
        let mut shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("main".to_owned())).unwrap();
        shell.with_port(|port| {
            assert_eq!(port.module_name(), "main");
            assert!(port.globals().is_empty());
            port.set_source_file(Some("source.hako".to_owned()));
            port.set_optimization_level(3);
        });
        assert_eq!(
            shell.module.metadata.source_file.as_deref(),
            Some("source.hako")
        );
        assert_eq!(shell.module.metadata.optimization_level, 3);
        assert!(!shell.has_published_functions());
    }
}
