//! HEADERPORT0-I0-DRAIN0-S0: route-owned drained module candidate.
//!
//! This product is the boundary after an unpublished invocation drain and
//! before the post-drain finalizer.  It owns one candidate module and one
//! completion inventory; it has no Builder, collector, retry, or fallback.

use super::module_invocation_drain::ConditionFnPolicyV1;
use super::root_body_completion::CompletedRootBodyV1;
use crate::mir::MirModule;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum DrainedModuleCandidateErrorV1 {
    DuplicateInventorySymbol {
        symbol: String,
    },
    InventoryMismatch {
        expected: Box<[String]>,
        actual: Box<[String]>,
    },
    MissingMain,
    MissingConditionFn,
    UnexpectedConditionFn,
}

impl std::fmt::Display for DrainedModuleCandidateErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][drained_candidate] {self:?}")
    }
}

impl std::error::Error for DrainedModuleCandidateErrorV1 {}

/// A route-owned, deterministic symbol inventory.  It is not supplied again
/// by a finalizer or reconstructed from the candidate module.
#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedInvocationInventoryV1 {
    symbols: Box<[String]>,
    root_body: CompletedRootBodyV1,
    condition_fn: ConditionFnPolicyV1,
    _seal: CompletedInvocationInventorySealV1,
}

#[derive(Debug)]
struct CompletedInvocationInventorySealV1;

/// The only candidate accepted by the future post-drain finalizer.
#[derive(Debug)]
pub(in crate::mir::builder) struct DrainedModuleCandidateV1 {
    module: MirModule,
    inventory: CompletedInvocationInventoryV1,
    _seal: DrainedModuleCandidateSealV1,
}

#[derive(Debug)]
struct DrainedModuleCandidateSealV1;

impl CompletedInvocationInventoryV1 {
    pub(in crate::mir::builder) fn new(
        symbols: impl IntoIterator<Item = String>,
        root_body: CompletedRootBodyV1,
        condition_fn: ConditionFnPolicyV1,
    ) -> Result<Self, DrainedModuleCandidateErrorV1> {
        let mut symbols = symbols.into_iter().collect::<Vec<_>>();
        symbols.sort();
        for pair in symbols.windows(2) {
            if pair[0] == pair[1] {
                return Err(DrainedModuleCandidateErrorV1::DuplicateInventorySymbol {
                    symbol: pair[0].clone(),
                });
            }
        }
        Ok(Self {
            symbols: symbols.into_boxed_slice(),
            root_body,
            condition_fn,
            _seal: CompletedInvocationInventorySealV1,
        })
    }

    pub(in crate::mir::builder) fn symbols(&self) -> &[String] {
        &self.symbols
    }

    pub(in crate::mir::builder) fn root_body(&self) -> &CompletedRootBodyV1 {
        &self.root_body
    }

    pub(in crate::mir::builder) fn condition_fn(&self) -> ConditionFnPolicyV1 {
        self.condition_fn
    }
}

impl DrainedModuleCandidateV1 {
    pub(in crate::mir::builder) fn from_drained_module(
        module: MirModule,
        inventory: CompletedInvocationInventoryV1,
    ) -> Result<Self, DrainedModuleCandidateErrorV1> {
        let actual = module.functions.keys().cloned().collect::<Vec<_>>();
        let actual = actual.into_boxed_slice();
        if actual.as_ref() != inventory.symbols() {
            return Err(DrainedModuleCandidateErrorV1::InventoryMismatch {
                expected: inventory.symbols().to_vec().into_boxed_slice(),
                actual,
            });
        }
        if !module.functions.contains_key("main") {
            return Err(DrainedModuleCandidateErrorV1::MissingMain);
        }
        match inventory.condition_fn() {
            ConditionFnPolicyV1::Required if !module.functions.contains_key("condition_fn") => {
                return Err(DrainedModuleCandidateErrorV1::MissingConditionFn)
            }
            ConditionFnPolicyV1::Forbidden if module.functions.contains_key("condition_fn") => {
                return Err(DrainedModuleCandidateErrorV1::UnexpectedConditionFn)
            }
            _ => {}
        }
        Ok(Self {
            module,
            inventory,
            _seal: DrainedModuleCandidateSealV1,
        })
    }

    pub(in crate::mir::builder) fn module(&self) -> &MirModule {
        &self.module
    }

    pub(in crate::mir::builder) fn inventory(&self) -> &CompletedInvocationInventoryV1 {
        &self.inventory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn function(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn root_body() -> CompletedRootBodyV1 {
        RootBodyCompletionTrackerV1::new()
            .complete(RootBodyResultV1::NoValue)
            .unwrap()
    }

    fn module(symbols: &[&str]) -> MirModule {
        let mut module = MirModule::new("drained".into());
        for symbol in symbols {
            module.add_function(function(symbol, 0));
        }
        module
    }

    #[test]
    fn exact_inventory_and_root_policy_co_seal_candidate() {
        let inventory = CompletedInvocationInventoryV1::new(
            vec!["condition_fn".into(), "main".into()],
            root_body(),
            ConditionFnPolicyV1::Required,
        )
        .unwrap();
        let candidate = DrainedModuleCandidateV1::from_drained_module(
            module(&["main", "condition_fn"]),
            inventory,
        )
        .unwrap();
        assert_eq!(candidate.module().functions.len(), 2);
        assert_eq!(candidate.inventory().symbols(), ["condition_fn", "main"]);
    }

    #[test]
    fn inventory_and_condition_failures_happen_before_candidate_issue() {
        assert_eq!(
            CompletedInvocationInventoryV1::new(
                vec!["main".into(), "main".into()],
                root_body(),
                ConditionFnPolicyV1::Optional,
            )
            .unwrap_err(),
            DrainedModuleCandidateErrorV1::DuplicateInventorySymbol {
                symbol: "main".into(),
            }
        );

        let inventory = CompletedInvocationInventoryV1::new(
            vec!["main".into()],
            root_body(),
            ConditionFnPolicyV1::Required,
        )
        .unwrap();
        assert_eq!(
            DrainedModuleCandidateV1::from_drained_module(module(&["main"]), inventory)
                .unwrap_err(),
            DrainedModuleCandidateErrorV1::MissingConditionFn
        );
    }

    #[test]
    fn candidate_does_not_expose_a_bare_module_consumer() {
        let inventory = CompletedInvocationInventoryV1::new(
            vec!["main".into()],
            root_body(),
            ConditionFnPolicyV1::Forbidden,
        )
        .unwrap();
        let candidate =
            DrainedModuleCandidateV1::from_drained_module(module(&["main"]), inventory).unwrap();
        assert!(candidate.module().functions.contains_key("main"));
    }
}
