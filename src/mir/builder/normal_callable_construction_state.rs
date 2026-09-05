//! Request-local realization of source-issued construction stores.
//! This state issues no field identity and never owns a published layout.

use super::CallableSemanticLoweringState;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation};
use crate::mir::normal_callable_semantic_package::{
    ConstructionEligibilityV1, ConstructionUnavailableV1,
};
use crate::mir::resolved_semantics::{HomeDemandV1, SourceNodeSiteV1};
use crate::mir::{BasicBlock, BasicBlockId, MirBuilder, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(super) enum ConstructionState {
    NotConstruction,
    RetainedUnavailable(ConstructionUnavailableV1),
    Selected {
        stores: BTreeMap<SourceNodeSiteV1, (CanonicalFieldRefV1, StoreProgress)>,
        frame: Option<(ValueId, BasicBlockId)>,
        completed: bool,
    },
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

/// Physical loan result, not a second semantic receipt or reusable plan.
#[derive(Debug)]
pub(in crate::mir::builder) struct TakenConstructionStore {
    site: SourceNodeSiteV1,
    field: CanonicalFieldRefV1,
    receiver: ValueId,
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
        for (assignment, field) in plan.stores() {
            if stores
                .insert(
                    assignment.statement_site().node().clone(),
                    (*field, StoreProgress::Pending),
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
        if !matches!(self.construction, ConstructionState::Selected { .. }) {
            return Ok(None);
        }
        let binding = self.receiver.ok_or_else(|| fault("receiver-missing"))?;
        let receiver = self
            .value_for_exact_binding(self.owner, binding)
            .map_err(|error| error.to_string())?;
        let ConstructionState::Selected {
            stores, completed, ..
        } = &mut self.construction
        else {
            unreachable!()
        };
        if *completed {
            return Err(fault("take-after-completion"));
        }
        let (field, progress) = stores
            .get_mut(site)
            .ok_or_else(|| fault("foreign-or-missing-store"))?;
        if !matches!(progress, StoreProgress::Pending) {
            return Err(fault("duplicate-store-take"));
        }
        *progress = StoreProgress::Taken;
        Ok(Some(TakenConstructionStore {
            site: site.clone(),
            field: *field,
            receiver,
        }))
    }

    pub(in crate::mir::builder) fn emit_construction_store(
        &mut self,
        builder: &mut MirBuilder,
        taken: TakenConstructionStore,
        base: ValueId,
        value: ValueId,
    ) -> Result<(), String> {
        if base != taken.receiver {
            return Err(fault("receiver-drift"));
        }
        let ConstructionState::Selected {
            stores,
            frame,
            completed,
        } = &mut self.construction
        else {
            return Err(fault("emission-unselected"));
        };
        let (field, progress) = stores
            .get_mut(&taken.site)
            .ok_or_else(|| fault("emission-site"))?;
        if *completed || *field != taken.field || !matches!(progress, StoreProgress::Taken) {
            return Err(fault("emission-state"));
        }
        let (fault_frame, fault_landing) = match *frame {
            Some(frame) => frame,
            None => {
                let id = builder.next_value_id();
                let landing = builder.next_block_id();
                let function = builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("no-function"))?;
                function
                    .blocks
                    .get_mut(&function.entry_block)
                    .ok_or_else(|| fault("no-entry"))?
                    .insert_instruction_after_phis(MirInstruction::FaultFrameEnter {
                        dst: id,
                        mode: FaultFrameMode::Borrowed,
                    });
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
                field: *field,
                base,
                value,
            },
            fault_frame,
            normal_landing: normal,
            fault_landing,
        })?;
        builder.start_new_block(normal)?;
        *progress = StoreProgress::Emitted {
            block: origin,
            normal,
            base,
            value,
        };
        Ok(())
    }

    pub(in crate::mir::builder) fn complete_construction_stores(
        &mut self,
        function: &MirFunction,
    ) -> Result<(), String> {
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
        self.construction.finish()?;
        self.construction.validate_bindings(function)
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
            .filter(|block| {
                matches!(
                    block.terminator,
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::FieldSet { .. },
                        ..
                    })
                )
            })
            .count();
        if actual_count != stores.len() {
            return Err(fault("emission-count"));
        }
        for (field, progress) in stores.values() {
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
                if actual == field && b == base && v == value && normal_landing == normal && Some((*fault_frame, *fault_landing)) == *frame)
            {
                return Err(fault("emission-drift"));
            }
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][construction-store/{reason}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::SourcePathV1;
    use crate::mir::{EffectMask, FunctionSignature, MirType};

    #[test]
    fn completed_store_bindings_reject_finalizer_drift_and_residuals() {
        // Physical validator unit test only. Source issuance is exercised by
        // the fixed Pair production publication test, not by this local row.
        let field = CanonicalFieldRefV1::from_declaration_ordinal(
            hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
            0,
        )
        .unwrap();
        let origin = BasicBlockId::new(0);
        let normal = BasicBlockId::new(1);
        let landing = BasicBlockId::new(2);
        let base = ValueId::new(0);
        let value = ValueId::new(1);
        let frame = ValueId::new(2);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "physical_store_binding".into(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::WRITE,
            },
            origin,
        );
        let mut block = BasicBlock::new(origin);
        block.set_terminator(MirInstruction::Invoke {
            operation: InvokeOperation::FieldSet { field, base, value },
            fault_frame: frame,
            normal_landing: normal,
            fault_landing: landing,
        });
        function.add_block(block);
        let site = SourcePathV1::function_body().node();
        let mut state = ConstructionState::Selected {
            stores: BTreeMap::from([(
                site.clone(),
                (
                    field,
                    StoreProgress::Emitted {
                        block: origin,
                        normal,
                        base,
                        value,
                    },
                ),
            )]),
            frame: Some((frame, landing)),
            completed: true,
        };
        state.validate_bindings(&function).unwrap();
        for case in 0..7 {
            let mut changed = function.clone();
            let terminator = &mut changed.blocks.get_mut(&origin).unwrap().terminator;
            if case == 6 {
                *terminator = Some(MirInstruction::Return { value: None });
            } else {
                let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::FieldSet { field, base, value },
                    fault_frame,
                    normal_landing,
                    fault_landing,
                }) = terminator
                else {
                    unreachable!()
                };
                match case {
                    0 => {
                        *field = CanonicalFieldRefV1::from_declaration_ordinal(field.object(), 1)
                            .unwrap()
                    }
                    1 => *base = ValueId::new(9),
                    2 => *value = ValueId::new(9),
                    3 => *fault_frame = ValueId::new(9),
                    4 => *normal_landing = BasicBlockId::new(9),
                    5 => *fault_landing = BasicBlockId::new(9),
                    _ => unreachable!(),
                }
            }
            assert!(state.validate_bindings(&changed).is_err(), "drift {case}");
        }
        let ConstructionState::Selected {
            stores, completed, ..
        } = &mut state
        else {
            unreachable!()
        };
        *completed = false;
        stores.get_mut(&site).unwrap().1 = StoreProgress::Taken;
        assert!(state.finish().unwrap_err().contains("completion-missing"));
        assert!(state
            .validate_bindings(&function)
            .unwrap_err()
            .contains("store-residual"));
    }
}
