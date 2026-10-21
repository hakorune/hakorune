//! Request-local realization of source-issued construction stores.
//! This state issues no field identity and never owns a published layout.
//!
//! Responsibility children: `emission` lowers each taken store into the
//! physical draft (provider `new` chains included); `validation` compares
//! recorded progress rows with the emitted function. The parent keeps the
//! source-plan installation, the take/complete lifecycle and the shared
//! fault tag.

use super::CallableSemanticLoweringState;
use crate::mir::normal_callable_semantic_package::{
    ConstructionEligibilityV1, ConstructionPlanV1, ConstructionSourceStepV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{HomeDemandV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::BTreeMap;

#[path = "normal_callable_construction_state/emission.rs"]
mod emission;
#[path = "normal_callable_construction_state/provider_argument.rs"]
mod provider_argument;
#[path = "normal_callable_construction_state/fault_cleanup.rs"]
mod fault_cleanup;
#[path = "normal_callable_construction_state/child_call.rs"]
mod child_call;
#[path = "normal_callable_construction_state/validation.rs"]
mod validation;
#[path = "normal_callable_construction_state/text_materialization.rs"]
mod text_materialization;

#[derive(Debug)]
pub(super) enum ConstructionState {
    NotConstruction,
    /// Owned state moved with the exact draft after draft validation.
    Transferred,
    RetainedUnavailable(ConstructionUnavailableV1),
    Selected {
        /// SAME source/execution projection retained through artifact replay.
        plan: ConstructionPlanV1,
        /// Original constructor parameters from the same resolved source owner.
        parameters: Box<[crate::mir::resolved_semantics::BindingRefV1]>,
        /// Original semantic order; neither field ordinal nor map iteration order.
        steps: Box<[ConstructionSourceStepV1]>,
        next_step: usize,
        stores: BTreeMap<SourceNodeSiteV1, SelectedConstructionStore>,
        /// Actual affine Call packets, in the same source-step sequence.
        calls: Vec<child_call::ConstructionCallEmissionV1>,
        frame: Option<(ValueId, BasicBlockId)>,
        completed: bool,
    },
}

/// The existing request-local state moved with its draft, not a new source
/// receipt. Only the selected constructor capture can take this payload.
#[derive(Debug)]
pub(in crate::mir) struct RetainedConstructionValidation {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    construction: ConstructionState,
    fault_frame: crate::mir::builder::function_fault_frame::FunctionFaultFrameV1,
}

pub(in crate::mir) type RetainedConstructionDrafts = Vec<(
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    RetainedConstructionValidation,
)>;

#[derive(Debug)]
pub(super) enum StoreProgress {
    Pending,
    Taken,
    Emitted {
        block: BasicBlockId,
        normal: BasicBlockId,
        base: ValueId,
        value: ValueId,
        /// Invoke origin of the proven provider `new` — present only for
        /// `ProviderConstruction` stores.
        provider: Option<BasicBlockId>,
        /// Independent Text acquisition from this original formal, not a provider New.
        text: Option<text_materialization::TextStoreAcquisitionV1>,
        /// A user-class provider child's emitted coordinates — `Birth`
        /// carries the `birth_call`/`reclaim_unpublished`/`home_release`
        /// chain; `NoBirthZero` carries only the store-discharge head —
        /// no call was emitted. `None` for builtin providers.
        provider_child: Option<ProviderChildEmissionV1>,
        /// Head of this store's source-sealed prior-field Fault discharge.
        discharge: BasicBlockId,
    },
}

/// Emitted coordinates of one provider `new` argument's qualified static
/// call — one invoke per sealed `QualifiedStaticCall` row, in source
/// order.
#[derive(Debug)]
pub(in crate::mir) struct ProviderCallArgEmission {
    source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ProviderStaticArgumentSourceV1>,
    packet: Option<crate::mir::normal_callable_semantic_package::PreparedProviderStaticArgumentV1>,
    publication: crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
    ordinal: u32,
    arguments: Box<[ValueId]>,
    /// Block whose terminator is `Invoke{Call{Global, I64}}`.
    invoke_block: BasicBlockId,
    /// Normal landing hosting the `InvokeNormalResult` projection.
    landing: BasicBlockId,
    /// Projected i64 value — the provider Birth actual at this ordinal.
    value: ValueId,
}

/// Emitted provider-child coordinates for one user-class provider
/// store — the binding validator checks each arm's block terminators
/// exactly.
#[derive(Debug)]
pub(super) enum ProviderChildEmissionV1 {
    Birth(ProviderBirthEmission),
    /// `NoBirthZero` zero-init child: the only post-allocation physical
    /// evidence is the store-fault `HomeRelease` chain head — the plan
    /// sealed arity 0 + fieldless, so no `birth_call`/`reclaim` exists.
    NoBirthZero { store_discharge: BasicBlockId },

}

/// Emitted provider-Birth chain coordinates for one user-class provider
/// store — the binding validator checks each block's terminator exactly.
#[derive(Debug)]
pub(super) struct ProviderBirthEmission {
    /// Chain head: the `new_box` normal landing. Each sealed
    /// qualified-static argument call extends the chain from here;
    /// with no call arguments the head is `birth_call` itself.
    entry: BasicBlockId,
    /// Block whose terminator is `Invoke{Birth}`.
    birth_call: BasicBlockId,
    /// Qualified-static argument invokes in emitted (source) order:
    /// `entry`'s terminator is the first, each `landing` carries the
    /// projection and the next invoke, and the last `landing`
    /// terminates in `birth_call`.
    call_args: Box<[ProviderCallArgEmission]>,
    /// Storage-only reclaim after argument or child-Birth Fault. Child
    /// Birth owns its partial fields; both outcomes lead to this parent's
    /// source-sealed prior-field discharge.
    reclaim: BasicBlockId,
    /// Head of the store-fault discharge chain under the same layout,
    /// ending in the `Invoke{HomeRelease}` tail.
    store_discharge: BasicBlockId,
    /// The child's sealed typed owned residences in emitted teardown
    /// order (reverse declaration order). Empty for a plain child.
    owned_fields: Box<[crate::mir::normal_callable_semantic_package::OwnedFieldChildV1]>,
}

#[derive(Debug)]
pub(super) struct SelectedConstructionStore {
    field: CanonicalFieldRefV1,
    receiver_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    receiver_binding: crate::mir::resolved_semantics::BindingRefV1,
    rhs: ConstructionStoreRhsV1,
    mode: crate::mir::instruction::FieldSetModeV1,
    fault_discharge: Box<[crate::mir::normal_callable_semantic_package::OwnedFieldChildV1]>,
    /// `me` receiver sites the sealed RHS reads through — observed
    /// against the receiver object at take time, never a field read.
    me_reads: Box<[crate::mir::resolved_semantics::SourceExprSiteV1]>,
    progress: StoreProgress,
}

/// Physical loan result, not a second semantic receipt or reusable plan.
#[derive(Debug)]
pub(in crate::mir::builder) struct TakenConstructionStore {
    site: SourceNodeSiteV1,
    field: CanonicalFieldRefV1,
    receiver: ValueId,
    rhs: ConstructionStoreRhsV1,
}

impl TakenConstructionStore {
    pub(in crate::mir::builder) const fn rhs(&self) -> &ConstructionStoreRhsV1 {
        &self.rhs
    }
}

impl ConstructionState {
    fn require_next_store(&self, site: &SourceNodeSiteV1) -> Result<(), String> {
        let Self::Selected { steps, next_step, stores, completed, .. } = self else {
            return Err(fault("step-unselected"));
        };
        if *completed { return Err(fault("take-after-completion")); }
        if !matches!(steps.get(*next_step), Some(ConstructionSourceStepV1::Store(expected)) if expected.node() == site) {
            return Err(fault("step-order-or-kind"));
        }
        if !stores.get(site).is_some_and(|store| matches!(store.progress, StoreProgress::Pending | StoreProgress::Taken)) {
            return Err(fault("step-store-residual"));
        }
        Ok(())
    }

    fn validate_step_coverage(&self) -> Result<(), String> {
        let Self::Selected { steps, stores, .. } = self else { return Ok(()); };
        let mut seen = std::collections::BTreeSet::new();
        for step in steps.iter() {
            if let ConstructionSourceStepV1::Store(site) = step {
                if !seen.insert(site.node()) || !stores.contains_key(site.node()) {
                    return Err(fault("step-store-coverage"));
                }
            }
        }
        if seen.len() != stores.len() { return Err(fault("step-store-coverage")); }
        Ok(())
    }

    pub(super) fn finish(&self) -> Result<(), String> {
        match self {
            Self::Selected {
                completed: false, ..
            } => Err(fault("completion-missing")),
            Self::Selected { steps, next_step, stores, calls, .. } => {
                self.validate_step_coverage()?;
                if calls.len() != steps.iter().filter(|step| matches!(step, ConstructionSourceStepV1::StoredChildCall(_))).count() {
                    return Err(fault("step-call-unconsumed"));
                }
                if *next_step != steps.len() { return Err(fault("step-residual")); }
                if stores.values().any(|store| !matches!(store.progress, StoreProgress::Emitted { .. })) {
                    return Err(fault("store-residual"));
                }
                Ok(())
            }
            Self::RetainedUnavailable(reason) => {
                let _ = reason;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl CallableSemanticLoweringState {
    pub(in crate::mir::builder) fn install_construction(
        &mut self,
        source: &crate::parser::ConstructorSourceIdV1,
        kind: crate::parser::ConstructorSourceKindV1,
        eligibility: &ConstructionEligibilityV1,
    ) -> Result<(), String> {
        if !matches!(self.construction, ConstructionState::NotConstruction) {
            return Err(fault("duplicate-installation"));
        }
        if kind != crate::parser::ConstructorSourceKindV1::Birth {
            return Ok(());
        }
        let plan = match eligibility {
            Ok(plan) => plan,
            Err(reason) => {
                self.construction = ConstructionState::RetainedUnavailable(*reason);
                return Ok(());
            }
        };
        let Some((expected, owner)) = plan.constructor() else {
            return Err(fault("source-missing"));
        };
        if !expected.same_as(source)
            || *owner != self.owner
            || plan
                .field_demands()
                .iter()
                .enumerate()
                .any(|(ordinal, demand)| {
                    let provider = plan.stores().iter().any(|store| {
                        store.field().declaration_ordinal() == ordinal as u32
                            && matches!(
                                store.rhs(),
                                ConstructionStoreRhsV1::ProviderConstruction { .. }
                            )
                    });
                    if provider {
                        return *demand != HomeDemandV1::Handle;
                    }
                    // A caller-provided object store (`me.<field> =
                    // <Parameter>`) carries `Handle` demand like a provider
                    // — the field type is the semantic layer's class
                    // authority, so `Handle` is accepted only on ordinals
                    // carrying a `Parameter` store.
                    let provided = plan.stores().iter().any(|store| {
                        store.field().declaration_ordinal() == ordinal as u32
                            && matches!(
                                store.rhs(),
                                ConstructionStoreRhsV1::Parameter { .. }
                            )
                    });
                    !provided && *demand != HomeDemandV1::Trivial
                })
        {
            return Err(fault("source-or-cleanup-contract"));
        }
        let original_stores: Vec<_> = plan.steps().iter().filter_map(|step| match step {
            ConstructionSourceStepV1::Store(site) => Some(site),
            ConstructionSourceStepV1::StoredChildCall(_) => None,
        }).collect();
        if original_stores.len() != plan.stores().len()
            || original_stores.iter().zip(plan.stores()).any(|(site, store)|
                *site != store.assignment().statement_site()) {
            return Err(fault("source-store-order-drift"));
        }
        let mut stores = BTreeMap::new();
        for (ordinal, store) in plan.stores().iter().enumerate() {
            let execution = plan.store_execution_v1(ordinal).ok_or_else(|| fault("source-or-cleanup-contract"))?;
            if !match (execution.mode(), execution.text_input_v1()) {
                (crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, Some(sum)) => sum.text_only(),
                (crate::mir::instruction::FieldSetModeV1::DynamicTextInitialize, None) => false,
                (_, loan) => loan.is_none(),
            } {
                return Err(fault("source-or-cleanup-contract"));
            }
            if stores
                .insert(
                    store.assignment().statement_site().node().clone(),
                    SelectedConstructionStore {
                        field: store.field(),
                        receiver_site: store.receiver_site().clone(),
                        receiver_binding: store.receiver_binding(),
                        rhs: store.rhs().clone(),
                        mode: execution.mode(),
                        fault_discharge: execution.fault_discharge().into(),
                        me_reads: store.me_reads().into(),
                        progress: StoreProgress::Pending,
                    },
                )
                .is_some()
            {
                return Err(fault("duplicate-source-store"));
            }
        }
        self.construction = ConstructionState::Selected {
            plan: plan.clone(), parameters: self.parameters.clone(),
            steps: plan.steps().into(), next_step: 0,
            stores, calls: Vec::new(),
            frame: None,
            completed: false,
        };
        Ok(())
    }

    pub(in crate::mir::builder) fn take_construction_store(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<TakenConstructionStore>, String> {
        if matches!(self.construction, ConstructionState::Transferred) {
            return Err(fault("state-transferred"));
        }
        if !matches!(self.construction, ConstructionState::Selected { .. }) {
            return Ok(None);
        }
        self.construction.require_next_store(site)?;
        let binding = self.receiver.ok_or_else(|| fault("receiver-missing"))?;
        let (field, receiver_site, receiver_binding, rhs, me_reads) =
            match &self.construction {
                ConstructionState::Selected {
                    stores, completed, ..
                } => {
                    if *completed {
                        return Err(fault("take-after-completion"));
                    }
                    let store = stores
                        .get(site)
                        .ok_or_else(|| fault("foreign-or-missing-store"))?;
                    if !matches!(store.progress, StoreProgress::Pending) {
                        return Err(fault("duplicate-store-take"));
                    }
                    if store.receiver_binding != binding {
                        return Err(fault("receiver-binding-drift"));
                    }
                    (
                        store.field,
                        store.receiver_site.clone(),
                        store.receiver_binding,
                        store.rhs.clone(),
                        store.me_reads.clone(),
                    )
                }
                _ => unreachable!(),
            };
        let receiver = self
            .value_for_exact_binding(self.owner, binding)
            .map_err(|error| error.to_string())?;
        self.observe_variable_site(receiver_site.node(), receiver_binding, receiver)?;
        for read_site in me_reads.iter() {
            self.observe_variable_site(read_site.node(), receiver_binding, receiver)?;
        }
        let ConstructionState::Selected {
            stores, completed, ..
        } = &mut self.construction
        else {
            unreachable!()
        };
        let store = stores
            .get_mut(site)
            .ok_or_else(|| fault("foreign-or-missing-store"))?;
        if *completed || !matches!(store.progress, StoreProgress::Pending) {
            return Err(fault("duplicate-store-take"));
        }
        store.progress = StoreProgress::Taken;
        Ok(Some(TakenConstructionStore {
            site: site.clone(),
            field,
            receiver,
            rhs,
        }))
    }

    pub(in crate::mir::builder) fn complete_construction_stores(
        &mut self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_fault_frame(function)?;
        self.construction.validate_bindings(function)?;
        if let ConstructionState::Selected { completed, .. } = &mut self.construction {
            if *completed {
                return Err(fault("duplicate-completion"));
            }
            *completed = true;
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn validate_finalized_construction_stores(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_fault_frame(function)?;
        self.construction.finish()?;
        self.construction.validate_bindings(function)
    }

    pub(in crate::mir::builder) fn take_finalized_construction_validation(
        &mut self,
        function: &MirFunction,
    ) -> Result<Option<RetainedConstructionValidation>, String> {
        if matches!(self.construction, ConstructionState::Transferred) {
            return Err(fault("duplicate-state-transfer"));
        }
        self.validate_finalized_construction_stores(function)?;
        if matches!(self.construction, ConstructionState::NotConstruction) {
            return Ok(None);
        }
        let fault_frame = self
            .fault_frame
            .take()
            .ok_or_else(|| fault("frame-missing"))?;
        let construction =
            std::mem::replace(&mut self.construction, ConstructionState::Transferred);
        Ok(Some(RetainedConstructionValidation {
            owner: self.owner,
            construction,
            fault_frame,
        }))
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][construction-store/{reason}]")
}

#[cfg(test)]
#[path = "normal_callable_construction_state_tests.rs"]
mod tests;
