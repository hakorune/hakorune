//! Whole-normal-source acyclic Main/helper semantic plan.

use std::collections::BTreeMap;

use crate::mir::compiler::acyclic_callable_graph::{
    AcyclicCallableGraphErrorV1, VerifiedAcyclicCallableGraphV1,
};
use crate::mir::compiler::callable_graph_inventory::{
    CallableGraphInventoryErrorV1, VerifiedCallableGraphInventoryV1, VerifiedCallableGraphSiteV1,
};
use crate::mir::compiler::callable_scc_partition::{
    CallableSccPartitionErrorV1, VerifiedCallableSccPartitionV1,
};
use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::resolved_callable_module::{
    ResolveCallableModuleErrorV1, VerifiedResolvedCallableModuleV1,
};
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CanonicalCallableKeyV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

use super::main_direct_call_plan::VerifiedNormalMainDirectCallPlanV1;
use super::main_resolved_source::VerifiedNormalMainRoleV1;
use super::product::{NormalMainMethodSiteV1, NormalSourceIdentityV1, NormalTopLevelSiteV1};

#[derive(Debug)]
struct NormalMainSemanticEvidenceV1 {
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
    role: VerifiedNormalMainRoleV1,
}

#[derive(Debug)]
pub(crate) struct PreparedNormalMainHelperResolutionV1 {
    catalog: CallableCatalogSealOutcomeV1,
    main: NormalMainSemanticEvidenceV1,
}

#[derive(Debug)]
pub(crate) struct CompletedNormalMainHelperResolutionV1 {
    helpers: VerifiedResolvedCallableModuleV1,
    main: NormalMainSemanticEvidenceV1,
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainHelperResolutionV1 {
    catalog: CallableCatalogSealOutcomeV1,
    main: NormalMainSemanticEvidenceV1,
    error: ResolveCallableModuleErrorV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalMainHelperResolutionStageV1 {
    HelperSemanticResolution,
}

impl RejectedNormalMainHelperResolutionV1 {
    pub(crate) const fn stage(&self) -> NormalMainHelperResolutionStageV1 {
        NormalMainHelperResolutionStageV1::HelperSemanticResolution
    }

    pub(crate) const fn error(&self) -> &ResolveCallableModuleErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalMainDirectCallPlanV1 {
    pub(crate) fn prepare_helper_resolution(self) -> PreparedNormalMainHelperResolutionV1 {
        let (source, if_control, completion, profile, block_expr_count) = self.into_parts();
        let (catalog, identity, main_box, main_method, forest, projection, role) =
            source.into_parts();
        PreparedNormalMainHelperResolutionV1 {
            catalog,
            main: NormalMainSemanticEvidenceV1 {
                identity,
                main_box,
                main_method,
                forest,
                projection,
                if_control,
                completion,
                profile,
                block_expr_count,
                role,
            },
        }
    }
}

impl PreparedNormalMainHelperResolutionV1 {
    pub(crate) fn resolve(
        self,
    ) -> Result<CompletedNormalMainHelperResolutionV1, RejectedNormalMainHelperResolutionV1> {
        match VerifiedResolvedCallableModuleV1::resolve_retaining(self.catalog) {
            Ok(helpers) => Ok(CompletedNormalMainHelperResolutionV1 {
                helpers,
                main: self.main,
            }),
            Err(rejected) => {
                let (catalog, error) = rejected.into_normal_composition_parts();
                Err(RejectedNormalMainHelperResolutionV1 {
                    catalog,
                    main: self.main,
                    error,
                })
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum NormalAcyclicCallableModuleErrorV1 {
    Inventory(CallableGraphInventoryErrorV1),
    Partition(CallableSccPartitionErrorV1),
    Graph(AcyclicCallableGraphErrorV1),
    MissingFunction(CanonicalCallableKeyV1),
    FunctionPreflight {
        key: CanonicalCallableKeyV1,
        error: CanonicalLoweringErrorV1,
    },
    UnsupportedPlanFamily(CanonicalCallableKeyV1),
    CallCountMismatch {
        key: CanonicalCallableKeyV1,
        graph: usize,
        profile: usize,
    },
    HelperCardinalityMismatch {
        graph: usize,
        functions: usize,
        plans: usize,
    },
    MissingHelperDeclaration,
    MainCatalogMembership,
    MainTargetMissing,
    CompilationBrandMismatch,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalAcyclicCallableModulePlanV1<'a> {
    owner: &'a CompletedNormalMainHelperResolutionV1,
    graph: VerifiedAcyclicCallableGraphV1,
    helper_plans: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalRecursiveCallableModulePlanV1<'a> {
    owner: &'a CompletedNormalMainHelperResolutionV1,
    partition: VerifiedCallableSccPartitionV1,
    helper_plans: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
}

#[derive(Debug)]
pub(crate) enum VerifiedNormalHelperTopologyPlanV1<'a> {
    Acyclic(VerifiedNormalAcyclicCallableModulePlanV1<'a>),
    Recursive(VerifiedNormalRecursiveCallableModulePlanV1<'a>),
}

impl CompletedNormalMainHelperResolutionV1 {
    pub(crate) fn prepare_topology_plan(
        &self,
    ) -> Result<VerifiedNormalHelperTopologyPlanV1<'_>, NormalAcyclicCallableModuleErrorV1> {
        let inventory = VerifiedCallableGraphInventoryV1::verify(&self.helpers)
            .map_err(NormalAcyclicCallableModuleErrorV1::Inventory)?;
        let partition = VerifiedCallableSccPartitionV1::verify(inventory)
            .map_err(NormalAcyclicCallableModuleErrorV1::Partition)?;
        let helper_plans = self.prepare_helper_plans(
            partition.inventory().nodes(),
            partition.inventory().call_sites(),
        )?;
        self.verify_main_correspondence()?;
        if partition.recursive_component_count() == 0 {
            let graph = VerifiedAcyclicCallableGraphV1::from_nonrecursive_partition(partition)
                .map_err(NormalAcyclicCallableModuleErrorV1::Graph)?;
            Ok(VerifiedNormalHelperTopologyPlanV1::Acyclic(
                VerifiedNormalAcyclicCallableModulePlanV1 {
                    owner: self,
                    graph,
                    helper_plans,
                },
            ))
        } else {
            Ok(VerifiedNormalHelperTopologyPlanV1::Recursive(
                VerifiedNormalRecursiveCallableModulePlanV1 {
                    owner: self,
                    partition,
                    helper_plans,
                },
            ))
        }
    }

    pub(crate) fn prepare_acyclic_plan(
        &self,
    ) -> Result<VerifiedNormalAcyclicCallableModulePlanV1<'_>, NormalAcyclicCallableModuleErrorV1>
    {
        let graph = VerifiedAcyclicCallableGraphV1::verify(&self.helpers)
            .map_err(NormalAcyclicCallableModuleErrorV1::Graph)?;
        let helper_plans = self.prepare_helper_plans(graph.nodes(), graph.call_sites())?;
        self.verify_main_correspondence()?;
        Ok(VerifiedNormalAcyclicCallableModulePlanV1 {
            owner: self,
            graph,
            helper_plans,
        })
    }

    fn prepare_helper_plans<'a>(
        &'a self,
        nodes: &[CanonicalCallableKeyV1],
        call_sites: &[VerifiedCallableGraphSiteV1],
    ) -> Result<
        BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
        NormalAcyclicCallableModuleErrorV1,
    > {
        if nodes.iter().any(|key| key.name() == "main") {
            return Err(NormalAcyclicCallableModuleErrorV1::MainCatalogMembership);
        }
        let mut helper_plans = BTreeMap::new();
        for key in nodes {
            let input = self
                .helpers
                .function_input(key)
                .map_err(|_| NormalAcyclicCallableModuleErrorV1::MissingFunction(key.clone()))?;
            let plan = if input.function().direct_call_targets().next().is_none() {
                CanonicalLoweringPreflightV1::verify_function(input)
            } else {
                CanonicalLoweringPreflightV1::verify_function_with_finite_direct_calls_v1(input)
            }
            .map_err(|error| {
                NormalAcyclicCallableModuleErrorV1::FunctionPreflight {
                    key: key.clone(),
                    error,
                }
            })?;
            let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
                return Err(NormalAcyclicCallableModuleErrorV1::UnsupportedPlanFamily(
                    key.clone(),
                ));
            };
            let graph_calls = call_sites.iter().filter(|row| row.caller() == key).count();
            if graph_calls != plan.direct_call_count() {
                return Err(NormalAcyclicCallableModuleErrorV1::CallCountMismatch {
                    key: key.clone(),
                    graph: graph_calls,
                    profile: plan.direct_call_count(),
                });
            }
            helper_plans.insert(key.clone(), plan);
        }
        if nodes.len() != self.helpers.functions_by_key().len()
            || helper_plans.len() != self.helpers.functions_by_key().len()
        {
            return Err(
                NormalAcyclicCallableModuleErrorV1::HelperCardinalityMismatch {
                    graph: nodes.len(),
                    functions: self.helpers.functions_by_key().len(),
                    plans: helper_plans.len(),
                },
            );
        }
        Ok(helper_plans)
    }

    fn verify_main_correspondence(&self) -> Result<(), NormalAcyclicCallableModuleErrorV1> {
        let main_owner = self.main.completion.owner();
        for &site in self.helpers.source().declaration_sites() {
            let declaration = self
                .helpers
                .source()
                .catalog()
                .declaration(site)
                .ok_or(NormalAcyclicCallableModuleErrorV1::MissingHelperDeclaration)?;
            if main_owner.compilation_brand() != declaration.callable().owner().compilation_brand()
            {
                return Err(NormalAcyclicCallableModuleErrorV1::CompilationBrandMismatch);
            }
        }
        for call in self.main.profile.direct_calls() {
            let target = call.target().callable();
            if self
                .helpers
                .source()
                .catalog()
                .index()
                .header_for_callable(target)
                .is_err()
            {
                return Err(NormalAcyclicCallableModuleErrorV1::MainTargetMissing);
            }
            if main_owner.compilation_brand() != target.owner().compilation_brand() {
                return Err(NormalAcyclicCallableModuleErrorV1::CompilationBrandMismatch);
            }
        }
        Ok(())
    }
}

impl VerifiedNormalAcyclicCallableModulePlanV1<'_> {
    pub(crate) fn helper_count(&self) -> usize {
        self.helper_plans.len()
    }

    pub(crate) fn main_direct_call_count(&self) -> usize {
        self.owner.main.profile.direct_calls().len()
    }

    pub(crate) fn helper_edge_count(&self) -> usize {
        self.graph.unique_edges().len()
    }
}

impl VerifiedNormalRecursiveCallableModulePlanV1<'_> {
    pub(crate) const fn partition(&self) -> &VerifiedCallableSccPartitionV1 {
        &self.partition
    }

    pub(crate) fn helper_count(&self) -> usize {
        self.helper_plans.len()
    }

    pub(crate) fn recursive_component_count(&self) -> usize {
        self.partition.recursive_component_count()
    }

    pub(crate) fn main_direct_call_count(&self) -> usize {
        self.owner.main.profile.direct_calls().len()
    }
}
