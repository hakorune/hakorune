//! Construction-store emission: literal/parameter values, provider `new`
//! chains and the checked field-store invoke landings.
//! Emits no field identity and owns no source predicate — it consumes the
//! selected plan rows the parent installed.

use super::fault_cleanup::{emit_discharge, jump_landing};
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
        // Pre-take each sealed qualified-static argument's publication
        // handoff before the construction-state borrow begins — the port
        // installed exactly one `Selected` row per argument site through
        // the sole consumption boundary. `self` must stay unborrowed here.
        let mut call_handoffs: std::collections::BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1,
        > = std::collections::BTreeMap::new();
        if let ConstructionStoreRhsV1::ProviderConstruction { arguments, .. } = &taken.rhs {
            for argument in arguments.iter() {
                if let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
                    target,
                    ..
                } = argument.kind()
                {
                    let handoff = self.take_provider_static_result_publication(
                        argument.site(),
                        target,
                    )?;
                    call_handoffs.insert(argument.site().clone(), handoff);
                }
            }
        }
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
                // Source-sealed per-operation discharge precedes this
                // shared Fault tail; the caller reclaims outer storage.
                block.set_terminator(MirInstruction::ReturnFault { fault_frame: id });
                function.add_block(block);
                *frame = Some((id, landing));
                (id, landing)
            }
        };
        let fault_landing = emit_discharge(
            builder,
            &store.fault_discharge,
            base,
            fault_frame,
            fault_landing,
        )?;
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
                    owned_fields,
                    ..
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
                        // An owned-array child owes each sealed `ArrayBox`
                        // residence release before its storage on every
                        // in-flight cleanup edge — the plan's declared
                        // inventory must agree exactly with the sealed
                        // children, in declaration order. The provider
                        // `new` mints no `local`-bound claim, so its row
                        // was sealed by `seal_provider_owned_children_v1`
                        // beside the claims.
                        let teardown: Box<[CanonicalFieldRefV1]> = if owned_fields.is_empty() {
                            Box::new([])
                        } else {
                            let children = ledger
                                .owned_field_children_for(child)
                                .ok_or_else(|| fault("provider-children-missing"))?
                                .ok_or_else(|| fault("provider-children-unproven"))?;
                            if children.len() != owned_fields.len()
                                || children.iter().zip(owned_fields.iter()).any(
                                    |(child_row, field)| {
                                        !matches!(
                                            child_row.kind,
                                            crate::mir::normal_callable_semantic_package::OwnedFieldChildKindV1::Array
                                        ) || child_row.field != *field
                                    },
                                )
                            {
                                return Err(fault("provider-children-drift"));
                            }
                            owned_fields.clone()
                        };
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
                        // Invoke landings must stay distinct and exclusive:
                        // both converge on the shared fault edge through
                        // dedicated jump blocks. The reclaim chain is built
                        // before argument evaluation so a faulted
                        // qualified-static argument call can land on the
                        // same lifecycle cleanup the birth call owns —
                        // the unpublished child is reclaimed exactly once
                        // on every fault edge.
                        let reclaim = {
                            // Child Birth owns its partial residences.
                            // This caller reclaims only unpublished storage.
                            let tail = builder.next_block_id();
                            let tail_normal = jump_landing(builder, fault_landing)?;
                            let tail_fault = jump_landing(builder, fault_landing)?;
                            let mut tail_block = BasicBlock::new(tail);
                            tail_block.set_terminator(MirInstruction::Invoke {
                                operation: InvokeOperation::ReclaimUnpublished {
                                    object: child,
                                    value: allocation,
                                },
                                fault_frame,
                                normal_landing: tail_normal,
                                fault_landing: tail_fault,
                            });
                            builder
                                .function_state
                                .current_function
                                .as_mut()
                                .ok_or_else(|| fault("no-function"))?
                                .add_block(tail_block);
                            tail
                        };
                        let mut arg_values = Vec::with_capacity(arguments.len());
                        let mut arg_pairs = Vec::with_capacity(arguments.len());
                        let mut call_arg_records = Vec::new();
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
                                crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
                                    target,
                                    arguments: inner,
                                } => {
                                    // One invoke per sealed row, in source
                                    // order: the claim proved a
                                    // `StaticBoxMethod` target with an
                                    // `ExactI64` result, so the physical
                                    // call is a global static target and
                                    // the normal projection is the i64
                                    // actual the provider birth consumes.
                                    // A fault lands on `reclaim` — the same
                                    // cleanup chain the birth call owns.
                                    let handoff = call_handoffs
                                        .remove(argument.site())
                                        .ok_or_else(|| {
                                            fault("provider-argument-handoff")
                                        })?;
                                    if handoff.target() != target
                                        || !matches!(
                                            handoff.representation(),
                                            crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1::ExactI64
                                        )
                                    {
                                        return Err(fault(
                                            "provider-argument-handoff-drift",
                                        ));
                                    }
                                    let mut call_args = Vec::with_capacity(inner.len());
                                    for inner_argument in inner.iter() {
                                        call_args.push(match inner_argument {
                                            crate::mir::normal_callable_semantic_package::QualifiedStaticCallArgumentKindV1::Integer(literal) => {
                                                crate::mir::builder::emission::constant::emit_integer(
                                                    builder, *literal,
                                                )?
                                            }
                                            crate::mir::normal_callable_semantic_package::QualifiedStaticCallArgumentKindV1::Bool(literal) => {
                                                crate::mir::builder::emission::constant::emit_bool(
                                                    builder, *literal,
                                                )?
                                            }
                                        });
                                    }
                                    let invoke_block = builder
                                        .function_state
                                        .current_block
                                        .ok_or_else(|| fault("no-block"))?;
                                    let normal = builder.next_block_id();
                                    let global = target
                                        .canonical_global_target_v1()
                                        .map_err(|_| fault("provider-argument-target"))?;
                                    let MirInstruction::Call(call) = MirInstruction::call(
                                        None,
                                        crate::mir::Callee::Global(global),
                                        call_args,
                                        crate::mir::canonical_direct_call::materialize_direct_call_effect_v1(
                                            crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1::ConservativeBarrier,
                                        ),
                                    ) else {
                                        unreachable!("canonical Call constructor")
                                    };
                                    builder.emit_instruction(MirInstruction::Invoke {
                                        operation: InvokeOperation::Call {
                                            call,
                                            result:
                                                crate::mir::instruction::InvokeCallResultKind::I64,
                                        },
                                        fault_frame,
                                        normal_landing: normal,
                                        fault_landing: reclaim,
                                    })?;
                                    builder.start_new_block(normal)?;
                                    let projected = builder.next_value_id();
                                    builder.emit_instruction(
                                        MirInstruction::InvokeNormalResult {
                                            dst: projected,
                                            invoke_block,
                                        },
                                    )?;
                                    builder
                                        .function_state
                                        .type_ctx
                                        .value_types
                                        .insert(projected, MirType::Integer);
                                    call_arg_records.push(ProviderCallArgEmission {
                                        invoke_block,
                                        landing: normal,
                                        value: projected,
                                    });
                                    projected
                                }
                                _ => return Err(fault("provider-argument-unsupported")),
                            };
                            arg_values.push(arg_value);
                            arg_pairs.push((argument.clone(), arg_value));
                        }
                        let store_entry = builder.next_block_id();
                        // The Birth invoke terminates whichever block the
                        // argument chain ended on — `provider_normal` with
                        // no call arguments, the last landing otherwise.
                        let birth_call_block = builder
                            .function_state
                            .current_block
                            .ok_or_else(|| fault("no-block"))?;
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
                        object_child = Some((child, teardown));
                        provider_birth =
                            Some((provider_normal, birth_call_block, call_arg_records, reclaim));
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
        let (operation, store_fault_landing) = match &object_child {
            Some((child, teardown)) => {
                // The checked object-field store consumes the child lease
                // on both edges: a faulted store releases each sealed
                // `ArrayBox` residence newest-first, then discharges the
                // in-flight child with its own release — the child was
                // never installed, so reclaiming it through the parent
                // slot would release the wrong object.
                let tail = builder.next_block_id();
                let tail_normal = jump_landing(builder, fault_landing)?;
                let tail_fault = jump_landing(builder, fault_landing)?;
                let function = builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(|| fault("no-function"))?;
                let mut tail_block = BasicBlock::new(tail);
                tail_block.set_terminator(MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease {
                        object: *child,
                        value,
                    },
                    fault_frame,
                    normal_landing: tail_normal,
                    fault_landing: tail_fault,
                });
                function.add_block(tail_block);
                let mut head = tail;
                for field in teardown.iter() {
                    let step_id = builder.next_block_id();
                    let normal_step = jump_landing(builder, head)?;
                    let fault_step = jump_landing(builder, head)?;
                    let function = builder
                        .function_state
                        .current_function
                        .as_mut()
                        .ok_or_else(|| fault("no-function"))?;
                    let mut step = BasicBlock::new(step_id);
                    step.set_terminator(MirInstruction::Invoke {
                        operation: InvokeOperation::OwnedFieldResidenceRelease {
                            field: *field,
                            base: value,
                        },
                        fault_frame,
                        normal_landing: normal_step,
                        fault_landing: fault_step,
                    });
                    function.add_block(step);
                    head = step_id;
                }
                (
                    InvokeOperation::ObjectFieldSet {
                        field: store.field,
                        base,
                        value,
                        child: *child,
                    },
                    head,
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
            discharge: fault_landing,
            provider_birth: provider_birth.map(|(entry, birth_call, call_args, reclaim)| {
                ProviderBirthEmission {
                    entry,
                    birth_call,
                    call_args: call_args.into_boxed_slice(),
                    reclaim,
                    store_discharge: store_fault_landing,
                    owned_fields: object_child
                        .as_ref()
                        .map(|(_, fields)| fields.iter().rev().copied().collect())
                        .unwrap_or_default(),
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
