//! Request-local realization of source-issued construction stores.
//! This state issues no field identity and never owns a published layout.

use super::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    ConstructionEligibilityV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{HomeDemandV1, SourceNodeSiteV1};
use crate::mir::{BasicBlock, BasicBlockId, MirBuilder, MirFunction, MirInstruction, ValueId};
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
    fault_frame: super::fault::CallableFaultFrame,
}

pub(in crate::mir::builder) type RetainedConstructionDrafts = Vec<(
    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    RetainedConstructionValidation,
)>;

impl RetainedConstructionValidation {

    pub(in crate::mir::builder) fn validate_artifact_after_compiler_finishing(
        self, function: &MirFunction,
    ) -> Result<(), String> {
        match &self.construction {
            ConstructionState::Selected { completed: true, .. } => {}
            ConstructionState::RetainedUnavailable(reason) => return Err(format!(
                "{} reason={reason:?}", fault("artifact-source-unavailable"))),
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
        self.construction.validate_bindings(function).map_err(|error| {
            format!("{error} owner={:?}", self.owner)
        })
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
    },
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
                .any(|demand| *demand != HomeDemandV1::Trivial)
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
            ConstructionState::Selected { stores, completed, .. } => {
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
        let value = match taken.rhs {
            ConstructionStoreRhsV1::LiteralI64(value) => {
                crate::mir::builder::emission::constant::emit_integer(builder, value)?
            }
            ConstructionStoreRhsV1::Parameter { site, binding } => {
                let value = self
                    .value_for_exact_binding(self.owner, binding)
                    .map_err(|error| error.to_string())?;
                self.observe_variable_site(site.node(), binding, value)?;
                value
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
        if *completed || store.field != taken.field || !matches!(store.progress, StoreProgress::Taken) {
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
        let origin = builder
            .function_state
            .current_block
            .ok_or_else(|| fault("no-block"))?;
        let normal = builder.next_block_id();
        builder.emit_instruction(MirInstruction::Invoke {
            operation: InvokeOperation::FieldSet {
                field: store.field,
                base,
                value,
            },
            fault_frame,
            normal_landing: normal,
            fault_landing,
        })?;
        builder.start_new_block(normal)?;
        store.progress = StoreProgress::Emitted {
            block: origin,
            normal,
            base,
            value,
        };
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
        let fault_frame = self.fault_frame.take().ok_or_else(|| fault("frame-missing"))?;
        let construction = std::mem::replace(
            &mut self.construction,
            ConstructionState::Transferred,
        );
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
        let actual_count = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Invoke { .. }
                )
            })
            .count();
        if actual_count != stores.len() {
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
                    MirInstruction::InvokeNormalResult { .. } => {
                        return Err(fault("unexpected-normal-result"));
                    }
                    MirInstruction::Call(call) if matches!(call.callee,
                        crate::mir::Callee::BirthConstructor { .. }) => {
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
            } = progress
            else {
                return Err(fault("store-residual"));
            };
            if !matches!(function.blocks.get(block).and_then(|b| b.terminator.as_ref()),
                Some(MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field: actual, base: b, value: v }, fault_frame, fault_landing, normal_landing })
                if actual == &field && b == base && v == value && normal_landing == normal && Some((*fault_frame, *fault_landing)) == *frame)
            {
                return Err(fault("emission-drift"));
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

fn fault(reason: &str) -> String {
    format!("[freeze:contract][construction-store/{reason}]")
}

#[cfg(test)]
#[path = "normal_callable_construction_state_tests.rs"]
mod tests;
