//! P0c-MR-V0 disconnected recursive exact-i64 module plan.
//!
//! This product combines one deterministic SCC partition with one finite
//! trivial Binding-SSA plan per canonical callable key before Builder effects.
//! It owns admission only: no target, ABI, effect, MIR, publication, backend,
//! or runtime authority is introduced here.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::callable_graph_inventory::{
    CallableGraphInventoryErrorV1, VerifiedCallableGraphInventoryV1,
};
use super::callable_scc_partition::{CallableSccPartitionErrorV1, VerifiedCallableSccPartitionV1};
use super::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1, CanonicalTrivialBindingSsaPlanV1,
};
use super::lowering_input::CanonicalLoweringErrorV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug)]
pub(crate) enum RecursiveCallableModulePlanErrorV1 {
    FunctionCardinality {
        actual: usize,
    },
    Inventory(CallableGraphInventoryErrorV1),
    Partition(CallableSccPartitionErrorV1),
    DirectCallCardinality {
        actual: usize,
    },
    NoRecursiveComponent,
    MissingFunction {
        key: CanonicalCallableKeyV1,
    },
    FunctionPreflight {
        key: CanonicalCallableKeyV1,
        source: CanonicalLoweringErrorV1,
    },
    UnsupportedPlanFamily {
        key: CanonicalCallableKeyV1,
    },
    FunctionCallSiteCountMismatch {
        key: CanonicalCallableKeyV1,
        inventory: usize,
        profile: usize,
    },
    DuplicateCanonicalKey(CanonicalCallableKeyV1),
    CardinalityMismatch {
        inventory: usize,
        functions: usize,
        components: usize,
        plans: usize,
    },
}

#[derive(Debug)]
pub(crate) struct VerifiedRecursiveCallableModulePlanV1<'a> {
    module: &'a VerifiedResolvedCallableModuleV1,
    partition: VerifiedCallableSccPartitionV1,
    plans_by_key: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecursiveFunctionCardinalityV1 {
    ExistingTwoOrMore,
    OneOrMoreForR0,
}

impl RecursiveFunctionCardinalityV1 {
    const fn minimum(self) -> usize {
        match self {
            Self::ExistingTwoOrMore => 2,
            Self::OneOrMoreForR0 => 1,
        }
    }
}

impl<'a> VerifiedRecursiveCallableModulePlanV1<'a> {
    pub(crate) fn verify(
        module: &'a VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, RecursiveCallableModulePlanErrorV1> {
        Self::verify_with_admission(module, RecursiveFunctionCardinalityV1::ExistingTwoOrMore)
    }

    /// Disconnected R0-S0 admission. Production remains on `verify()` until
    /// the later atomic CUT0 authority replacement.
    #[cfg(test)]
    pub(crate) fn verify_one_or_more_for_r0(
        module: &'a VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, RecursiveCallableModulePlanErrorV1> {
        Self::verify_with_admission(module, RecursiveFunctionCardinalityV1::OneOrMoreForR0)
    }

    fn verify_with_admission(
        module: &'a VerifiedResolvedCallableModuleV1,
        admission: RecursiveFunctionCardinalityV1,
    ) -> Result<Self, RecursiveCallableModulePlanErrorV1> {
        if module.functions_by_key().len() < admission.minimum() {
            return Err(RecursiveCallableModulePlanErrorV1::FunctionCardinality {
                actual: module.functions_by_key().len(),
            });
        }
        let inventory = VerifiedCallableGraphInventoryV1::verify(module)
            .map_err(RecursiveCallableModulePlanErrorV1::Inventory)?;
        if inventory.call_sites().is_empty() {
            return Err(RecursiveCallableModulePlanErrorV1::DirectCallCardinality { actual: 0 });
        }
        let partition = VerifiedCallableSccPartitionV1::verify(inventory)
            .map_err(RecursiveCallableModulePlanErrorV1::Partition)?;
        if partition.recursive_component_count() == 0 {
            return Err(RecursiveCallableModulePlanErrorV1::NoRecursiveComponent);
        }

        let mut plans_by_key = BTreeMap::new();
        for key in partition.inventory().nodes() {
            let input = module.function_input(key).map_err(|_| {
                RecursiveCallableModulePlanErrorV1::MissingFunction { key: key.clone() }
            })?;
            let plan =
                CanonicalLoweringPreflightV1::verify_function_with_finite_direct_calls_v1(input)
                    .map_err(
                        |source| RecursiveCallableModulePlanErrorV1::FunctionPreflight {
                            key: key.clone(),
                            source,
                        },
                    )?;
            let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
                return Err(RecursiveCallableModulePlanErrorV1::UnsupportedPlanFamily {
                    key: key.clone(),
                });
            };
            let inventory_calls = partition
                .inventory()
                .call_sites()
                .iter()
                .filter(|row| row.caller() == key)
                .count();
            let profile_calls = plan.direct_call_count();
            if inventory_calls != profile_calls {
                return Err(
                    RecursiveCallableModulePlanErrorV1::FunctionCallSiteCountMismatch {
                        key: key.clone(),
                        inventory: inventory_calls,
                        profile: profile_calls,
                    },
                );
            }
            if plans_by_key.insert(key.clone(), plan).is_some() {
                return Err(RecursiveCallableModulePlanErrorV1::DuplicateCanonicalKey(
                    key.clone(),
                ));
            }
        }

        let inventory_count = partition.inventory().nodes().len();
        let function_count = module.functions_by_key().len();
        let component_members = partition
            .components()
            .iter()
            .map(|component| component.members().len())
            .sum();
        if inventory_count != function_count
            || component_members != function_count
            || plans_by_key.len() != function_count
        {
            return Err(RecursiveCallableModulePlanErrorV1::CardinalityMismatch {
                inventory: inventory_count,
                functions: function_count,
                components: component_members,
                plans: plans_by_key.len(),
            });
        }

        Ok(Self {
            module,
            partition,
            plans_by_key,
        })
    }

    pub(crate) const fn module(&self) -> &'a VerifiedResolvedCallableModuleV1 {
        self.module
    }

    pub(crate) const fn partition(&self) -> &VerifiedCallableSccPartitionV1 {
        &self.partition
    }

    pub(crate) fn plans_by_key(
        &self,
    ) -> &BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>> {
        &self.plans_by_key
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        &'a VerifiedResolvedCallableModuleV1,
        VerifiedCallableSccPartitionV1,
        BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
    ) {
        (self.module, self.partition, self.plans_by_key)
    }
}

#[cfg(test)]
mod tests;
