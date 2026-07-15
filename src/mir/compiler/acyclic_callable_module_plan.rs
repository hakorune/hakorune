//! P0c-F-V0 disconnected typed activation witness.
//!
//! This box combines the S0 topology proof with one finite trivial plan per
//! canonical key before Builder effects. It introduces no new semantic
//! authority and performs no MIR drafting or publication.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::acyclic_callable_graph::{AcyclicCallableGraphErrorV1, VerifiedAcyclicCallableGraphV1};
use super::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1, CanonicalTrivialBindingSsaPlanV1,
};
use super::lowering_input::CanonicalLoweringErrorV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug)]
pub(crate) enum AcyclicCallableModulePlanErrorV1 {
    FunctionCardinality {
        actual: usize,
    },
    Graph(AcyclicCallableGraphErrorV1),
    DirectCallCardinality {
        actual: usize,
    },
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
        graph: usize,
        profile: usize,
    },
    DuplicateCanonicalKey(CanonicalCallableKeyV1),
    CardinalityMismatch {
        graph: usize,
        functions: usize,
        plans: usize,
    },
}

#[derive(Debug)]
pub(crate) struct VerifiedAcyclicCallableModulePlanV1<'a> {
    module: &'a VerifiedResolvedCallableModuleV1,
    graph: VerifiedAcyclicCallableGraphV1,
    plans_by_key: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
}

impl<'a> VerifiedAcyclicCallableModulePlanV1<'a> {
    pub(crate) fn verify(
        module: &'a VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, AcyclicCallableModulePlanErrorV1> {
        if module.functions_by_key().len() < 2 {
            return Err(AcyclicCallableModulePlanErrorV1::FunctionCardinality {
                actual: module.functions_by_key().len(),
            });
        }
        let graph = VerifiedAcyclicCallableGraphV1::verify(module)
            .map_err(AcyclicCallableModulePlanErrorV1::Graph)?;
        if graph.call_sites().is_empty() {
            return Err(AcyclicCallableModulePlanErrorV1::DirectCallCardinality { actual: 0 });
        }

        let mut plans_by_key = BTreeMap::new();
        for key in graph.nodes() {
            let input = module.function_input(key).map_err(|_| {
                AcyclicCallableModulePlanErrorV1::MissingFunction { key: key.clone() }
            })?;
            let plan =
                CanonicalLoweringPreflightV1::verify_function_with_finite_direct_calls_v1(input)
                    .map_err(
                        |source| AcyclicCallableModulePlanErrorV1::FunctionPreflight {
                            key: key.clone(),
                            source,
                        },
                    )?;
            let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
                return Err(AcyclicCallableModulePlanErrorV1::UnsupportedPlanFamily {
                    key: key.clone(),
                });
            };
            let graph_calls = graph
                .call_sites()
                .iter()
                .filter(|row| row.caller() == key)
                .count();
            let profile_calls = plan.direct_call_count();
            if graph_calls != profile_calls {
                return Err(
                    AcyclicCallableModulePlanErrorV1::FunctionCallSiteCountMismatch {
                        key: key.clone(),
                        graph: graph_calls,
                        profile: profile_calls,
                    },
                );
            }
            if plans_by_key.insert(key.clone(), plan).is_some() {
                return Err(AcyclicCallableModulePlanErrorV1::DuplicateCanonicalKey(
                    key.clone(),
                ));
            }
        }

        if graph.nodes().len() != module.functions_by_key().len()
            || plans_by_key.len() != module.functions_by_key().len()
        {
            return Err(AcyclicCallableModulePlanErrorV1::CardinalityMismatch {
                graph: graph.nodes().len(),
                functions: module.functions_by_key().len(),
                plans: plans_by_key.len(),
            });
        }
        Ok(Self {
            module,
            graph,
            plans_by_key,
        })
    }

    pub(crate) const fn module(&self) -> &'a VerifiedResolvedCallableModuleV1 {
        self.module
    }

    pub(crate) const fn graph(&self) -> &VerifiedAcyclicCallableGraphV1 {
        &self.graph
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
        VerifiedAcyclicCallableGraphV1,
        BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
    ) {
        (self.module, self.graph, self.plans_by_key)
    }
}

#[cfg(test)]
mod tests;
