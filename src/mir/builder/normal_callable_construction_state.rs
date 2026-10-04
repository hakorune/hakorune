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
    ConstructionEligibilityV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{HomeDemandV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::BTreeMap;

#[path = "normal_callable_construction_state/emission.rs"]
mod emission;
#[path = "normal_callable_construction_state/validation.rs"]
mod validation;

#[derive(Debug)]
pub(super) enum ConstructionState {
    NotConstruction,
    /// Owned state moved with the exact draft after draft validation.
    Transferred,
    RetainedUnavailable(ConstructionUnavailableV1),
    Selected {
        stores: BTreeMap<SourceNodeSiteV1, SelectedConstructionStore>,
        frame: Option<(ValueId, BasicBlockId)>,
        completed: bool,
    },
}

/// The existing request-local state moved with its draft, not a new source
/// receipt. Only the selected constructor capture can take this payload.
#[derive(Debug)]
pub(in crate::mir::builder) struct RetainedConstructionValidation {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    construction: ConstructionState,
    fault_frame: crate::mir::builder::function_fault_frame::FunctionFaultFrameV1,
}

pub(in crate::mir::builder) type RetainedConstructionDrafts = Vec<(
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
        /// A user-class provider emits `birth_call` on the allocation's
        /// normal landing, with `reclaim_unpublished`/`home_release`
        /// discharge blocks on the two fault edges.
        provider_birth: Option<ProviderBirthEmission>,
    },
}

/// Emitted provider-Birth chain coordinates for one user-class provider
/// store — the binding validator checks each block's terminator exactly.
#[derive(Debug)]
pub(super) struct ProviderBirthEmission {
    /// Block whose terminator is `Invoke{Call{BirthConstructor}}`.
    birth_call: BasicBlockId,
    /// Head of the birth-fault cleanup chain: one
    /// `Invoke{OwnedFieldResidenceRelease}` block per `owned_fields`
    /// entry (emitted order), then the `Invoke{ReclaimUnpublished}` tail
    /// — with no owned fields the head is the reclaim tail itself.
    reclaim: BasicBlockId,
    /// Head of the store-fault discharge chain under the same layout,
    /// ending in the `Invoke{HomeRelease}` tail.
    store_discharge: BasicBlockId,
    /// The child's sealed `ArrayBox` residences in emitted teardown
    /// order (reverse declaration order). Empty for a plain child.
    owned_fields: Box<[CanonicalFieldRefV1]>,
}

#[derive(Debug)]
pub(super) struct SelectedConstructionStore {
    field: CanonicalFieldRefV1,
    receiver_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    receiver_binding: crate::mir::resolved_semantics::BindingRefV1,
    rhs: ConstructionStoreRhsV1,
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
    pub(super) fn finish(&self) -> Result<(), String> {
        match self {
            Self::Selected {
                completed: false, ..
            } => Err(fault("completion-missing")),
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
        let mut stores = BTreeMap::new();
        for store in plan.stores() {
            if stores
                .insert(
                    store.assignment().statement_site().node().clone(),
                    SelectedConstructionStore {
                        field: store.field(),
                        receiver_site: store.receiver_site().clone(),
                        receiver_binding: store.receiver_binding(),
                        rhs: store.rhs().clone(),
                        progress: StoreProgress::Pending,
                    },
                )
                .is_some()
            {
                return Err(fault("duplicate-source-store"));
            }
        }
        self.construction = ConstructionState::Selected {
            stores,
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
        let binding = self.receiver.ok_or_else(|| fault("receiver-missing"))?;
        let (field, receiver_site, receiver_binding, rhs) = match &self.construction {
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
                )
            }
            _ => unreachable!(),
        };
        let receiver = self
            .value_for_exact_binding(self.owner, binding)
            .map_err(|error| error.to_string())?;
        self.observe_variable_site(receiver_site.node(), receiver_binding, receiver)?;
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
