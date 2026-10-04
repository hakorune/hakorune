//! Construction-store emission: literal/parameter values, provider `new`
//! chains and the checked field-store invoke landings.
//! Emits no field identity and owns no source predicate — it consumes the
//! selected plan rows the parent installed.

use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::{BasicBlock, BasicBlockId, MirBuilder, MirInstruction, MirType};

impl CallableSemanticLoweringState {
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
