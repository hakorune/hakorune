//! TX0-only handoff from completed Main/helper semantics to draft preparation.
//!
//! The retained source authority owns the original Program/catalog and Main
//! source identity. Helper lowering plans borrow it only inside one consuming
//! callback, so no product stores an owner beside a surviving borrowed plan.

use std::collections::BTreeMap;

use crate::mir::compiler::acyclic_callable_graph::VerifiedAcyclicCallableGraphV1;
use crate::mir::compiler::callable_graph_inventory::{
    VerifiedCallableGraphInventoryV1, VerifiedCallableGraphSiteV1,
};
use crate::mir::compiler::callable_scc_partition::VerifiedCallableSccPartitionV1;
use crate::mir::compiler::capability::{
    bind_sealed_normal_main_parts_v1, CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1,
    CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, FunctionOwnerIdV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::resolved_value_profile::product::VerifiedTrivialCanonicalOwnerV1;

use super::main_resolved_source::VerifiedNormalMainRoleV1;
use super::normal_acyclic_module_plan::{
    CompletedNormalMainHelperResolutionV1, NormalAcyclicCallableModuleErrorV1,
};
use super::product::{NormalMainMethodSiteV1, NormalSourceIdentityV1, NormalTopLevelSiteV1};

/// Exact Main facts that remain meaningful after a lowering plan is consumed.
#[derive(Debug)]
pub(crate) struct RetainedNormalMainSourceAuthorityV1 {
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    owner: FunctionOwnerIdV1,
}

/// Durable TX0 semantic authority. It intentionally has no AST extraction API.
#[derive(Debug)]
pub(crate) struct RetainedNormalCallableSourceAuthorityV1 {
    helpers: VerifiedResolvedCallableModuleV1,
    main: RetainedNormalMainSourceAuthorityV1,
}

/// One catalog-derived helper ABI expectation. This is source evidence only;
/// draft correspondence remains a Builder-side responsibility.
#[derive(Debug)]
pub(crate) struct NormalHelperDraftAbiExpectationV1 {
    symbol: Box<str>,
    arity: usize,
}

impl NormalHelperDraftAbiExpectationV1 {
    pub(crate) const fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Debug)]
pub(crate) enum NormalHelperDraftAbiExpectationErrorV1 {
    MissingHeader(CanonicalCallableKeyV1),
}

impl RetainedNormalCallableSourceAuthorityV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.main.identity.display_name()
    }

    pub(crate) const fn main_owner(&self) -> FunctionOwnerIdV1 {
        self.main.owner
    }

    pub(crate) fn helper_count(&self) -> usize {
        self.helpers.functions_by_key().len()
    }

    pub(crate) fn helper_draft_abi(
        &self,
        key: &CanonicalCallableKeyV1,
    ) -> Result<NormalHelperDraftAbiExpectationV1, NormalHelperDraftAbiExpectationErrorV1> {
        let header = self
            .helpers
            .source()
            .catalog()
            .index()
            .lookup(key)
            .ok_or_else(|| NormalHelperDraftAbiExpectationErrorV1::MissingHeader(key.clone()))?;
        Ok(NormalHelperDraftAbiExpectationV1 {
            symbol: header.symbol().as_mir_name().into(),
            arity: header.signature().arity(),
        })
    }

    pub(crate) fn borrow_main_input(
        &self,
    ) -> Result<ResolvedFunctionLoweringInputV1<'_>, CanonicalLoweringErrorV1> {
        let function = self
            .helpers
            .source()
            .embedded_function(
                self.main.main_box.statement_index(),
                self.main.main_method.method_key(),
            )
            .ok_or_else(|| CanonicalLoweringErrorV1::SourceNavigation {
                detail: "normal_callable_handoff_main_syntax_missing".to_owned(),
            })?;
        ResolvedFunctionLoweringInputV1::from_exact_parts_with_callable_index(
            function.function_ast(),
            &self.main.forest,
            &self.main.projection,
            self.helpers.source().catalog().index(),
        )
    }
}

/// The one-shot Main proof. DRAFTS0 will bind it to a fresh borrowed input.
#[derive(Debug)]
pub(crate) struct ConsumableNormalMainLoweringProofV1 {
    if_control: VerifiedResolvedFunctionIfControlV1,
    completion: VerifiedFunctionCompletionV1,
    profile: VerifiedTrivialCanonicalOwnerV1,
    block_expr_count: usize,
    role: VerifiedNormalMainRoleV1,
}

impl ConsumableNormalMainLoweringProofV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.completion.owner()
    }

    pub(crate) const fn role(&self) -> VerifiedNormalMainRoleV1 {
        self.role
    }

    pub(crate) fn direct_call_count(&self) -> usize {
        self.profile.direct_calls().len()
    }

    fn validates_input(&self, input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
        let owner = input.owner();
        self.if_control.owner() == owner
            && self.completion.owner() == owner
            && self.profile.owner() == owner
            && input.function().owner() == owner
            && input.source().owner() == owner
    }
}

/// The sole TX0 source owner before any Builder effect.
#[derive(Debug)]
pub(crate) struct OpenNormalCallableModuleTransactionV1 {
    source: RetainedNormalCallableSourceAuthorityV1,
    main_lowering: Option<ConsumableNormalMainLoweringProofV1>,
}

/// A topology witness owning the existing graph or SCC partition.
#[derive(Debug)]
pub(crate) enum PreparedNormalHelperTopologyReceiptV1 {
    Acyclic(VerifiedAcyclicCallableGraphV1),
    Recursive(VerifiedCallableSccPartitionV1),
}

impl PreparedNormalHelperTopologyReceiptV1 {
    pub(crate) fn helper_count(&self) -> usize {
        match self {
            Self::Acyclic(graph) => graph.nodes().len(),
            Self::Recursive(partition) => partition.inventory().nodes().len(),
        }
    }

    pub(crate) fn recursive_component_count(&self) -> usize {
        match self {
            Self::Acyclic(_) => 0,
            Self::Recursive(partition) => partition.recursive_component_count(),
        }
    }
}

/// A callback-scoped schedule. `BTreeMap` is the sole helper order authority.
#[derive(Debug)]
pub(crate) struct OwnedNormalHelperLoweringScheduleV1<'source> {
    topology: PreparedNormalHelperTopologyReceiptV1,
    plans: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'source>>,
}

impl<'source> OwnedNormalHelperLoweringScheduleV1<'source> {
    pub(crate) const fn topology(&self) -> &PreparedNormalHelperTopologyReceiptV1 {
        &self.topology
    }

    pub(crate) fn helper_keys(&self) -> impl Iterator<Item = &CanonicalCallableKeyV1> {
        self.plans.keys()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        PreparedNormalHelperTopologyReceiptV1,
        BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'source>>,
    ) {
        (self.topology, self.plans)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableHandoffStageV1 {
    TopologyReceipt,
    HelperSchedule,
}

#[derive(Debug)]
pub(crate) struct RejectedNormalCallableHandoffV1 {
    source: RetainedNormalCallableSourceAuthorityV1,
    main_lowering: ConsumableNormalMainLoweringProofV1,
    stage: NormalCallableHandoffStageV1,
    error: NormalAcyclicCallableModuleErrorV1,
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainProofBindingV1 {
    owner: OpenNormalCallableModuleTransactionV1,
    proof: ConsumableNormalMainLoweringProofV1,
    error: CanonicalLoweringErrorV1,
}

impl RejectedNormalMainProofBindingV1 {
    pub(crate) fn error(&self) -> &CanonicalLoweringErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl RejectedNormalCallableHandoffV1 {
    pub(crate) const fn stage(&self) -> NormalCallableHandoffStageV1 {
        self.stage
    }

    pub(crate) const fn error(&self) -> &NormalAcyclicCallableModuleErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl CompletedNormalMainHelperResolutionV1 {
    /// Consuming TX0 split. No source, catalog, graph, or resolver is rebuilt.
    pub(crate) fn into_tx0_handoff(self) -> OpenNormalCallableModuleTransactionV1 {
        let (
            helpers,
            identity,
            main_box,
            main_method,
            forest,
            projection,
            role,
            if_control,
            completion,
            profile,
            block_expr_count,
        ) = self.into_tx0_parts();
        let owner = completion.owner();
        debug_assert_eq!(if_control.owner(), owner);
        OpenNormalCallableModuleTransactionV1 {
            source: RetainedNormalCallableSourceAuthorityV1 {
                helpers,
                main: RetainedNormalMainSourceAuthorityV1 {
                    identity,
                    main_box,
                    main_method,
                    forest,
                    projection,
                    role,
                    owner,
                },
            },
            main_lowering: Some(ConsumableNormalMainLoweringProofV1 {
                if_control,
                completion,
                profile,
                block_expr_count,
                role,
            }),
        }
    }
}

impl OpenNormalCallableModuleTransactionV1 {
    pub(crate) const fn source(&self) -> &RetainedNormalCallableSourceAuthorityV1 {
        &self.source
    }

    pub(crate) const fn has_main_lowering_proof(&self) -> bool {
        self.main_lowering.is_some()
    }

    pub(crate) fn take_main_lowering_proof(&mut self) -> ConsumableNormalMainLoweringProofV1 {
        self.main_lowering
            .take()
            .expect("TX0 handoff seals exactly one Main lowering proof")
    }

    /// Binds the already-sealed Main proof to one exact borrowed input. The
    /// callback must consume the plan; it cannot escape beside this owner.
    pub(crate) fn with_main_lowering_plan<R>(
        mut self,
        consume: impl for<'source> FnOnce(
            &'source RetainedNormalCallableSourceAuthorityV1,
            CanonicalTrivialBindingSsaPlanV1<'source>,
        ) -> R,
    ) -> Result<(Self, R), RejectedNormalMainProofBindingV1> {
        let proof = self.take_main_lowering_proof();
        let input = match self.source.borrow_main_input() {
            Ok(input) => input,
            Err(error) => {
                return Err(RejectedNormalMainProofBindingV1 {
                    owner: self,
                    proof,
                    error,
                })
            }
        };
        if !proof.validates_input(input) {
            return Err(RejectedNormalMainProofBindingV1 {
                owner: self,
                proof,
                error: CanonicalLoweringErrorV1::SourceUnitResolution {
                    detail: "normal_main_sealed_fact_owner_mismatch".to_owned(),
                },
            });
        }
        let plan = bind_sealed_normal_main_parts_v1(
            input,
            proof.if_control,
            proof.completion,
            proof.profile,
            proof.block_expr_count,
        )
        .expect("validated Main proof binds without reclassification");
        let result = consume(&self.source, plan);
        Ok((self, result))
    }

    /// Consumes every helper plan inside `consume`, then returns this open
    /// owner after the plans' source borrow has ended.
    pub(crate) fn with_helper_plans<R>(
        self,
        consume: impl for<'source> FnOnce(
            &'source RetainedNormalCallableSourceAuthorityV1,
            OwnedNormalHelperLoweringScheduleV1<'source>,
        ) -> R,
    ) -> Result<(Self, R), RejectedNormalCallableHandoffV1> {
        let schedule = match self.prepare_helper_schedule() {
            Ok(schedule) => schedule,
            Err(error) => return Err(self.reject_schedule(error)),
        };
        let result = consume(&self.source, schedule);
        Ok((self, result))
    }

    fn prepare_helper_schedule(
        &self,
    ) -> Result<OwnedNormalHelperLoweringScheduleV1<'_>, NormalAcyclicCallableModuleErrorV1> {
        let inventory = VerifiedCallableGraphInventoryV1::verify(&self.source.helpers)
            .map_err(NormalAcyclicCallableModuleErrorV1::Inventory)?;
        let partition = VerifiedCallableSccPartitionV1::verify(inventory)
            .map_err(NormalAcyclicCallableModuleErrorV1::Partition)?;
        let plans = self.prepare_helper_plans(
            partition.inventory().nodes(),
            partition.inventory().call_sites(),
        )?;
        self.verify_main_correspondence()?;
        let topology = if partition.recursive_component_count() == 0 {
            PreparedNormalHelperTopologyReceiptV1::Acyclic(
                VerifiedAcyclicCallableGraphV1::from_nonrecursive_partition(partition)
                    .map_err(NormalAcyclicCallableModuleErrorV1::Graph)?,
            )
        } else {
            PreparedNormalHelperTopologyReceiptV1::Recursive(partition)
        };
        Ok(OwnedNormalHelperLoweringScheduleV1 { topology, plans })
    }

    fn prepare_helper_plans<'source>(
        &'source self,
        nodes: &[CanonicalCallableKeyV1],
        call_sites: &[VerifiedCallableGraphSiteV1],
    ) -> Result<
        BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'source>>,
        NormalAcyclicCallableModuleErrorV1,
    > {
        if nodes.iter().any(|key| key.name() == "main") {
            return Err(NormalAcyclicCallableModuleErrorV1::MainCatalogMembership);
        }
        let mut plans = BTreeMap::new();
        for key in nodes {
            let input =
                self.source.helpers.function_input(key).map_err(|_| {
                    NormalAcyclicCallableModuleErrorV1::MissingFunction(key.clone())
                })?;
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
            plans.insert(key.clone(), plan);
        }
        let functions = self.source.helpers.functions_by_key().len();
        if nodes.len() != functions || plans.len() != functions {
            return Err(
                NormalAcyclicCallableModuleErrorV1::HelperCardinalityMismatch {
                    graph: nodes.len(),
                    functions,
                    plans: plans.len(),
                },
            );
        }
        Ok(plans)
    }

    fn verify_main_correspondence(&self) -> Result<(), NormalAcyclicCallableModuleErrorV1> {
        let main = self
            .main_lowering
            .as_ref()
            .expect("TX0 helper schedule precedes Main proof consumption");
        let main_owner = main.owner();
        for &site in self.source.helpers.source().declaration_sites() {
            let declaration = self
                .source
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
        for call in main.profile.direct_calls() {
            let target = call.target().callable();
            if self
                .source
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

    fn reject_schedule(
        self,
        error: NormalAcyclicCallableModuleErrorV1,
    ) -> RejectedNormalCallableHandoffV1 {
        let stage = match error {
            NormalAcyclicCallableModuleErrorV1::Inventory(_)
            | NormalAcyclicCallableModuleErrorV1::Partition(_)
            | NormalAcyclicCallableModuleErrorV1::Graph(_) => {
                NormalCallableHandoffStageV1::TopologyReceipt
            }
            _ => NormalCallableHandoffStageV1::HelperSchedule,
        };
        RejectedNormalCallableHandoffV1 {
            source: self.source,
            main_lowering: self
                .main_lowering
                .expect("TX0 rejection precedes Main proof consumption"),
            stage,
            error,
        }
    }
}

#[cfg(test)]
#[path = "normal_callable_transaction_handoff_tests.rs"]
mod tests;
