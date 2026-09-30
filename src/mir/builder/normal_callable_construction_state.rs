//! Request-local realization of source-issued construction stores.
//! This state issues no field identity and never owns a published layout.

use super::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    ConstructionEligibilityV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{HomeDemandV1, SourceNodeSiteV1};
use crate::mir::{
    BasicBlock, BasicBlockId, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::BTreeMap;

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

impl RetainedConstructionValidation {
    pub(in crate::mir::builder) fn validate_artifact_after_compiler_finishing(
        self,
        function: &MirFunction,
    ) -> Result<(), String> {
        match &self.construction {
            ConstructionState::Selected {
                completed: true, ..
            } => {}
            ConstructionState::RetainedUnavailable(reason) => {
                return Err(format!(
                    "{} reason={reason:?}",
                    fault("artifact-source-unavailable")
                ))
            }
            _ => return Err(fault("artifact-construction-not-complete")),
        }
        self.validate_after_compiler_finishing(function)
    }

    pub(in crate::mir::builder) fn validate_after_compiler_finishing(
        self,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.fault_frame.validate(function)?;
        self.construction.finish()?;
        self.construction
            .validate_bindings(function)
            .map_err(|error| format!("{error} owner={:?}", self.owner))
    }
}

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
    /// Block whose terminator is `Invoke{ReclaimUnpublished}` on the child.
    reclaim: BasicBlockId,
    /// Block whose terminator is `Invoke{HomeRelease}` on the child,
    /// reached when the checked object-field store faults.
    store_discharge: BasicBlockId,
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
                        *demand != HomeDemandV1::Handle
                    } else {
                        *demand != HomeDemandV1::Trivial
                    }
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

    pub(in crate::mir::builder) fn emit_construction_store(
        &mut self,
        builder: &mut MirBuilder,
        taken: TakenConstructionStore,
        provider_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        let source_ready = matches!(
            &self.construction,
            ConstructionState::Selected { stores, completed: false, .. }
                if matches!(stores.get(&taken.site), Some(store)
                    if store.field == taken.field && matches!(store.progress, StoreProgress::Taken))
        );
        if !source_ready {
            return Err(fault("emission-state"));
        }
        if provider_value.is_some() {
            return Err(fault("provider-value-foreign"));
        }
        let mut provider_site = None;
        let value = match &taken.rhs {
            ConstructionStoreRhsV1::LiteralI64(value) => Some(
                crate::mir::builder::emission::constant::emit_integer(builder, *value)?,
            ),
            ConstructionStoreRhsV1::Parameter { site, binding } => {
                let value = self
                    .value_for_exact_binding(self.owner, *binding)
                    .map_err(|error| error.to_string())?;
                self.observe_variable_site(site.node(), *binding, value)?;
                Some(value)
            }
            ConstructionStoreRhsV1::ProviderConstruction { site, .. } => {
                provider_site = Some(site.clone());
                None
            }
        };
        let base = taken.receiver;
        let shared_frame = self.borrow_fault_frame(builder)?;
        let ConstructionState::Selected {
            stores,
            frame,
            completed,
        } = &mut self.construction
        else {
            return Err(fault("emission-unselected"));
        };
        let store = stores
            .get_mut(&taken.site)
            .ok_or_else(|| fault("emission-site"))?;
        if *completed
            || store.field != taken.field
            || !matches!(store.progress, StoreProgress::Taken)
        {
            return Err(fault("emission-state"));
        }
        let (fault_frame, fault_landing) = match *frame {
            Some(frame) => frame,
            None => {
                let id = shared_frame;
                let landing = builder.next_block_id();
                let function = builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("no-function"))?;
                let mut block = BasicBlock::new(landing);
                // The selected source plan proves every initialized field Trivial.
                // No parent fini or field release is owed inside this Birth.
                block.set_terminator(MirInstruction::ReturnFault { fault_frame: id });
                function.add_block(block);
                *frame = Some((id, landing));
                (id, landing)
            }
        };
        let mut provider_origin = None;
        let mut provider_birth = None;
        let mut object_child = None;
        let value = match value {
            Some(value) => value,
            None => {
                // The proven provider `new` emits its checked allocation on
                // the shared fault frame; the normal result in its own
                // landing is the value the field store installs.
                let ConstructionStoreRhsV1::ProviderConstruction {
                    class,
                    site: rhs_site,
                    object,
                    arguments,
                } = &taken.rhs
                else {
                    unreachable!("provider value implies provider rhs")
                };
                let allocation = builder.next_value_id();
                let origin = builder
                    .function_state
                    .current_block
                    .ok_or_else(|| fault("no-block"))?;
                let provider_normal = builder.next_block_id();
                match object {
                    None => {
                        // The plan only admits a bare `new` of a builtin
                        // class; this physical consumer owns `ArrayBox` and
                        // emits the checked intrinsic itself.
                        if class.as_ref() != "ArrayBox" {
                            return Err(fault("provider-class-unsupported"));
                        }
                        builder.emit_instruction(MirInstruction::Invoke {
                            operation: InvokeOperation::IntrinsicArrayNew,
                            fault_frame,
                            normal_landing: provider_normal,
                            fault_landing,
                        })?;
                        builder.start_new_block(provider_normal)?;
                        builder.emit_instruction(MirInstruction::InvokeNormalResult {
                            dst: allocation,
                            invoke_block: origin,
                        })?;
                    }
                    Some(child) => {
                        // A user-class provider runs the canonical Birth
                        // path on the shared fault frame: `new_box`, then
                        // `birth_call` whose fault edge reclaims the
                        // unpublished child; the store below moves the
                        // proven handle into the parent slot.
                        let child = *child;
                        let ledger = self
                            .ordinary_new_claim_ledger
                            .as_ref()
                            .ok_or_else(|| fault("provider-ledger-missing"))?;
                        let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                            self.owner,
                            rhs_site.clone(),
                        );
                        let (recipe, handoff) = ledger
                            .take_birth_site_recipe(
                                &owned_site,
                                class,
                                arguments.len(),
                            )
                            .map_err(|_| fault("provider-birth-recipe-drift"))?
                            .ok_or_else(|| fault("provider-birth-recipe-missing"))?;
                        builder.emit_instruction(MirInstruction::Invoke {
                            operation: InvokeOperation::NewBox { object: child },
                            fault_frame,
                            normal_landing: provider_normal,
                            fault_landing,
                        })?;
                        builder.start_new_block(provider_normal)?;
                        builder.emit_instruction(MirInstruction::InvokeNormalResult {
                            dst: allocation,
                            invoke_block: origin,
                        })?;
                        let mut arg_values = Vec::with_capacity(arguments.len());
                        let mut arg_pairs = Vec::with_capacity(arguments.len());
                        for argument in arguments.iter() {
                            let arg_value = match argument.kind() {
                                crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Integer(
                                    literal,
                                ) => crate::mir::builder::emission::constant::emit_integer(
                                    builder, *literal,
                                )?,
                                crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Bool(
                                    literal,
                                ) => crate::mir::builder::emission::constant::emit_bool(
                                    builder, *literal,
                                )?,
                                _ => return Err(fault("provider-argument-unsupported")),
                            };
                            arg_values.push(arg_value);
                            arg_pairs.push((argument.clone(), arg_value));
                        }
                        let reclaim = builder.next_block_id();
                        // Invoke landings must stay distinct and exclusive:
                        // both converge on the shared fault edge through
                        // dedicated jump blocks.
                        let reclaim_normal = jump_landing(builder, fault_landing)?;
                        let reclaim_fault = jump_landing(builder, fault_landing)?;
                        let mut reclaim_block = BasicBlock::new(reclaim);
                        reclaim_block.set_terminator(MirInstruction::Invoke {
                            operation: InvokeOperation::ReclaimUnpublished {
                                object: child,
                                value: allocation,
                            },
                            fault_frame,
                            normal_landing: reclaim_normal,
                            fault_landing: reclaim_fault,
                        });
                        builder
                            .function_state
                            .current_function
                            .as_mut()
                            .ok_or_else(|| fault("no-function"))?
                            .add_block(reclaim_block);
                        let store_entry = builder.next_block_id();
                        let effects = recipe.physical_effect_mask();
                        let MirInstruction::Call(call) = MirInstruction::call(
                            None,
                            crate::mir::Callee::BirthConstructor {
                                key: recipe.target(),
                                receiver: allocation,
                            },
                            arg_values,
                            effects,
                        ) else {
                            unreachable!("canonical Call constructor")
                        };
                        builder.emit_instruction(MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call,
                                result: crate::mir::instruction::InvokeCallResultKind::Unit,
                            },
                            fault_frame,
                            normal_landing: store_entry,
                            fault_landing: reclaim,
                        })?;
                        builder.start_new_block(store_entry)?;
                        ledger.record_provider_birth(
                            owned_site,
                            child,
                            handoff,
                            allocation,
                            arg_pairs,
                        )?;
                        object_child = Some(child);
                        provider_birth = Some((provider_normal, reclaim));
                    }
                }
                builder
                    .function_state
                    .type_ctx
                    .value_origin_newbox
                    .insert(allocation, class.as_ref().into());
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(allocation, MirType::Box(class.as_ref().into()));
                builder
                    .comp_ctx
                    .type_registry
                    .record_newbox(allocation, class.as_ref().to_string());
                builder
                    .comp_ctx
                    .type_registry
                    .record_type(allocation, MirType::Box(class.as_ref().into()));
                provider_origin = Some(origin);
                allocation
            }
        };
        let origin = builder
            .function_state
            .current_block
            .ok_or_else(|| fault("no-block"))?;
        let normal = builder.next_block_id();
        let (operation, store_fault_landing) = match object_child {
            Some(child) => {
                // The checked object-field store consumes the child lease
                // on both edges: a faulted store discharges the in-flight
                // child with its own plain-object release before landing
                // on the shared fault edge.
                let discharge = builder.next_block_id();
                let discharge_normal = jump_landing(builder, fault_landing)?;
                let discharge_fault = jump_landing(builder, fault_landing)?;
                let function = builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("no-function"))?;
                let mut block = BasicBlock::new(discharge);
                block.set_terminator(MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease {
                        object: child,
                        value,
                    },
                    fault_frame,
                    normal_landing: discharge_normal,
                    fault_landing: discharge_fault,
                });
                function.add_block(block);
                (
                    InvokeOperation::ObjectFieldSet {
                        field: store.field,
                        base,
                        value,
                        child,
                    },
                    discharge,
                )
            }
            None => (
                InvokeOperation::FieldSet {
                    field: store.field,
                    base,
                    value,
                },
                fault_landing,
            ),
        };
        builder.emit_instruction(MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing: normal,
            fault_landing: store_fault_landing,
        })?;
        builder.start_new_block(normal)?;
        store.progress = StoreProgress::Emitted {
            block: origin,
            normal,
            base,
            value,
            provider: provider_origin,
            provider_birth: provider_birth.map(|(birth_call, reclaim)| {
                ProviderBirthEmission {
                    birth_call,
                    reclaim,
                    store_discharge: store_fault_landing,
                }
            }),
        };
        if let Some(site) = provider_site {
            if object_child.is_some() {
                return Ok(value);
            }
            // The generic new lane records a claimed provider allocation on
            // both ledgers; this consumer keeps the same accounting exact.
            self.record_named_array_allocation(&site, value)?;
            if self.named_array_field_provider(&site).is_some() {
                let function = builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("provider-function-missing"))?;
                if function
                    .metadata
                    .named_array_field_allocations
                    .insert(site, value)
                    .is_some()
                {
                    return Err(fault("provider-allocation-duplicate"));
                }
            }
        }
        Ok(value)
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

impl ConstructionState {
    fn validate_bindings(&self, function: &MirFunction) -> Result<(), String> {
        let ConstructionState::Selected { stores, frame, .. } = self else {
            return Ok(());
        };
        let provider_count = stores
            .values()
            .filter(|store| {
                matches!(
                    store.rhs,
                    ConstructionStoreRhsV1::ProviderConstruction { .. }
                )
            })
            .count();
        let birth_providers = stores
            .values()
            .filter(|store| {
                matches!(
                    &store.rhs,
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(_),
                        ..
                    }
                )
            })
            .count();
        let actual_count = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| matches!(instruction, MirInstruction::Invoke { .. }))
            .count();
        // A user-class provider adds three more invokes on the frame:
        // `birth_call`, the `reclaim_unpublished` on its fault edge, and the
        // `home_release` discharge on the object-field store's fault edge.
        if actual_count != stores.len() + provider_count + 3 * birth_providers {
            return Err(fault("emission-count"));
        }
        let mut fault_returns = 0;
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                match instruction {
                    MirInstruction::ReturnFault { fault_frame } => {
                        if *frame != Some((*fault_frame, block.id)) {
                            return Err(fault("fault-return-drift"));
                        }
                        fault_returns += 1;
                    }
                    MirInstruction::InvokeNormalResult { invoke_block, dst } => {
                        // Only a proven provider `new` may land a normal
                        // result — its store records the exact pair, and the
                        // projection sits in the allocation's normal
                        // landing (the `birth_call` block for a user-class
                        // provider, the store block otherwise).
                        let proven = stores.values().any(|store| {
                            matches!(
                                &store.progress,
                                StoreProgress::Emitted {
                                    block: home,
                                    value,
                                    provider: Some(origin),
                                    provider_birth,
                                    ..
                                } if provider_birth
                                    .as_ref()
                                    .map(|birth| birth.birth_call)
                                    .unwrap_or(*home)
                                    == block.id
                                    && *origin == *invoke_block
                                    && *value == *dst
                            )
                        });
                        if !proven {
                            return Err(fault("unexpected-normal-result"));
                        }
                    }
                    MirInstruction::Call(call)
                        if matches!(call.callee, crate::mir::Callee::BirthConstructor { .. }) =>
                    {
                        return Err(fault("unowned-birth-call"));
                    }
                    _ => {}
                }
            }
        }
        if fault_returns != usize::from(frame.is_some()) {
            return Err(fault("fault-return-count"));
        }
        for store in stores.values() {
            let field = store.field;
            let progress = &store.progress;
            let StoreProgress::Emitted {
                block,
                normal,
                base,
                value,
                provider,
                provider_birth,
            } = progress
            else {
                return Err(fault("store-residual"));
            };
            let store_ok = match (&store.rhs, provider_birth) {
                (
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    },
                    Some(birth),
                ) => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldSet { field: actual, base: b, value: v, child: stored }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && stored == child
                        && normal_landing == normal
                        && *fault_landing == birth.store_discharge
                        && frame.is_some_and(|(id, _)| *fault_frame == id)),
                _ => matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                    Some(MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field: actual, base: b, value: v }, fault_frame, fault_landing, normal_landing })
                    if actual == &field && b == base && v == value && normal_landing == normal && Some((*fault_frame, *fault_landing)) == *frame),
            };
            if !store_ok {
                return Err(fault("emission-drift"));
            }
            match (provider, &store.rhs, provider_birth) {
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: None, ..
                    },
                    None,
                ) => {
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::IntrinsicArrayNew,
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *normal_landing == *block
                            && Some((*fault_frame, *fault_landing)) == *frame)
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                }
                (
                    Some(provider_origin),
                    ConstructionStoreRhsV1::ProviderConstruction {
                        object: Some(child),
                        ..
                    },
                    Some(birth),
                ) => {
                    let Some((_, frame_landing)) = *frame else {
                        return Err(fault("emission-state"));
                    };
                    if !matches!(function.blocks.get(provider_origin).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::NewBox { object },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child
                            && *normal_landing == birth.birth_call
                            && *fault_landing == frame_landing
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-emission-drift"));
                    }
                    if !matches!(function.blocks.get(&birth.birth_call).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call,
                                result: crate::mir::instruction::InvokeCallResultKind::Unit,
                            },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if matches!(call.callee, crate::mir::Callee::BirthConstructor { ref receiver, .. } if *receiver == *value)
                            && *normal_landing == *block
                            && *fault_landing == birth.reclaim
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-birth-drift"));
                    }
                    if !matches!(function.blocks.get(&birth.reclaim).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::ReclaimUnpublished { object, value: reclaimed },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *reclaimed == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, frame_landing)
                            && lands_on(function, *fault_landing, frame_landing)
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-reclaim-drift"));
                    }
                    if !matches!(function.blocks.get(&birth.store_discharge).and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation: InvokeOperation::HomeRelease { object, value: released },
                            fault_frame,
                            fault_landing,
                            normal_landing,
                        }) if *object == *child && *released == *value
                            && normal_landing != fault_landing
                            && lands_on(function, *normal_landing, frame_landing)
                            && lands_on(function, *fault_landing, frame_landing)
                            && frame.is_some_and(|(id, _)| *fault_frame == id))
                    {
                        return Err(fault("provider-discharge-drift"));
                    }
                }
                (None, ConstructionStoreRhsV1::ProviderConstruction { .. }, _) => {
                    return Err(fault("provider-emission-missing"));
                }
                (
                    Some(_),
                    ConstructionStoreRhsV1::ProviderConstruction { .. },
                    _,
                ) => return Err(fault("provider-birth-missing")),
                (Some(_), _, _) => return Err(fault("provider-emission-foreign")),
                (None, _, Some(_)) => return Err(fault("provider-birth-foreign")),
                (None, _, None) => {}
            }
            if let ConstructionStoreRhsV1::LiteralI64(expected) = &store.rhs {
                let literal_matches = function
                    .blocks
                    .get(block)
                    .into_iter()
                    .flat_map(|block| block.all_instructions())
                    .any(|instruction| {
                        matches!(instruction,
                            MirInstruction::Const {
                                dst,
                                value: crate::mir::ConstValue::Integer(actual),
                            } if *dst == *value && *actual == *expected)
                    });
                if !literal_matches {
                    return Err(fault("literal-value-drift"));
                }
            }
        }
        Ok(())
    }
}

/// Dedicated single-predecessor landing that forwards to `target`: invoke
/// normal/fault edges may converge on the same continuation only through
/// distinct jump blocks.
fn jump_landing(
    builder: &mut MirBuilder,
    target: BasicBlockId,
) -> Result<BasicBlockId, String> {
    let id = builder.next_block_id();
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| fault("no-function"))?;
    if function.blocks.contains_key(&id) {
        return Err(fault("duplicate-block"));
    }
    let mut block = BasicBlock::new(id);
    block.set_terminator(MirInstruction::Jump {
        target,
        edge_args: None,
    });
    function.add_block(block);
    Ok(id)
}

/// True when `landing` is a dedicated jump block forwarding to `target`.
fn lands_on(
    function: &crate::mir::MirFunction,
    landing: BasicBlockId,
    target: BasicBlockId,
) -> bool {
    matches!(
        function.blocks.get(&landing).and_then(|b| b.terminator.as_ref()),
        Some(MirInstruction::Jump { target: actual, edge_args: None }) if *actual == target
    )
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][construction-store/{reason}]")
}

#[cfg(test)]
#[path = "normal_callable_construction_state_tests.rs"]
mod tests;
