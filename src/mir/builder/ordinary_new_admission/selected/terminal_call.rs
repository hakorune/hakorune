//! Physical terminal Call lowering uses the ledger's original literal relation.
//! The affine row stays owned by root exit progress after emission.
use super::*;
use crate::mir::definitions::MirCall;
use crate::mir::normal_callable_semantic_package::{
    DirectCallDispositionRowV1, LexicalInstanceCallDispositionRowV1, RootCallDispositionV1,
    RootInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

pub(in crate::mir::builder::ordinary_new_admission) struct Emission {
    pub(super) row: RootCallDispositionV1,
    pub(super) arguments: Vec<(BasicBlockId, MirInstruction)>,
    pub(super) call: MirCall,
    pub(super) result: InvokeCallResultKind,
}

fn result_type(result: InvokeCallResultKind, class: Option<&str>) -> Result<MirType, String> {
    match result {
        InvokeCallResultKind::Map => Ok(MirType::Box("MapBox".to_string())),
        InvokeCallResultKind::Handle => class
            .map(|class| MirType::Box(class.to_string()))
            .ok_or_else(|| freeze("handle-result-class-missing")),
        // A nullable result is a box handle on the live arm and the `Void`
        // sentinel otherwise — the sealed claim (not the runtime value)
        // authorizes the `Box` classification of the live arm.
        InvokeCallResultKind::NullableHandle => class
            .map(|class| MirType::Box(class.to_string()))
            .ok_or_else(|| freeze("nullable-result-class-missing")),
        _ => Ok(MirType::Integer),
    }
}

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    row: DirectCallDispositionRowV1,
) -> Result<ValueId, String> {
    let emission = row.physical_emission();
    let result = row.result();
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let mut arguments = Vec::new();
    let mut values = Vec::new();
    for argument in ledger
        .terminal_call_arguments_for_owner_at(owner, site)
        .ok_or_else(|| freeze("call-source-missing"))?
    {
        use crate::mir::resolved_semantics::home_new_prefix::TerminalCallArgumentV1;
        match argument {
            TerminalCallArgumentV1::I64(literal) => {
                let value =
                    crate::mir::builder::emission::constant::emit_integer(builder, *literal)?;
                arguments.push((
                    block,
                    MirInstruction::Const {
                        dst: value,
                        value: crate::mir::ConstValue::Integer(*literal),
                    },
                ));
                values.push((
                    value,
                    crate::mir::resolved_semantics::ExactCallableParamAbiV1::I64,
                ));
            }
            TerminalCallArgumentV1::Map(site) => {
                // The sealed CallArgument flow row drives construction;
                // the caller keeps the lease — the callee borrows the
                // storage pointer and the caller's exit chain Ends it.
                let (value, projection) = super::map::emit_argument(builder, state, ledger, site)?;
                arguments.push(projection);
                values.push((
                    value,
                    crate::mir::resolved_semantics::ExactCallableParamAbiV1::Map,
                ));
            }
            TerminalCallArgumentV1::Lexical(_) => {
                return Err(freeze("direct-call-lexical-argument"))
            }
        }
    }
    let call = emission
        .materialize_call_typed(None, values)
        .map_err(|_| freeze("call-projection-failed"))?;
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, result_type(result, None)?);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        Some(value),
        value,
        Some(RootExitIngress::Call(Emission {
            row: RootCallDispositionV1::Direct(row),
            arguments,
            call,
            result,
        })),
    )
}

pub(in crate::mir::builder) fn emit_instance(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    row: RootInstanceCallDispositionRowV1,
    receiver: ValueId,
) -> Result<ValueId, String> {
    let call = MirCall::new(
        None,
        crate::mir::definitions::Callee::SameModuleInstance {
            key: row.target().clone(),
            receiver,
        },
        Vec::new(),
    );
    let result = row.result();
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, result_type(result, None)?);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        Some(value),
        value,
        Some(RootExitIngress::Call(Emission {
            row: RootCallDispositionV1::Instance(row),
            arguments: Vec::new(),
            call,
            result,
        })),
    )
}

pub(super) fn emit_ingress(
    builder: &mut MirBuilder,
    frame: ValueId,
    value: ValueId,
    clean: BasicBlockId,
    fault: BasicBlockId,
    operation: InvokeOperation,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<
    (
        (BasicBlockId, MirInstruction),
        (BasicBlockId, MirInstruction),
    ),
    String,
> {
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let fault_landing = builder.next_block_id();
    for (id, target) in [(normal_landing, clean), (fault_landing, fault)] {
        append_block(
            builder,
            id,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            bindings,
        )?;
    }
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: value,
    };
    builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| freeze("no-function"))?
        .blocks
        .get_mut(&normal_landing)
        .ok_or_else(|| freeze("no-normal-landing"))?
        .add_instruction(projection.clone());
    let invoke = MirInstruction::Invoke {
        operation,
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    Ok(((origin, invoke), (normal_landing, projection)))
}

/// Emit one source-issued local Call result and leave its normal continuation
/// open for the enclosing local statement. This bounded slice accepts only a
/// local relation with no prior Homes; terminal cleanup remains root-owned.
pub(in crate::mir::builder) fn emit_local(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: DirectCallDispositionRowV1,
    arguments: Vec<ValueId>,
) -> Result<ValueId, String> {
    let relation = ledger
        .local_call_for_owner(owner, site)
        .ok_or_else(|| freeze("local-call-source-missing"))?;
    if !relation.prior_homes().is_empty() {
        return Err(freeze("local-call-prior-homes-unsupported"));
    }
    let result_kind = row.result();
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    let lifecycle = row
        .lifecycle_emission()
        .map_err(|_| freeze("local-call-source-mismatch"))?;
    match result_kind {
        InvokeCallResultKind::Map => ledger.begin_map_call_emission(&owned_site)?,
        // The callee's canonical object moves to this owner at the Return
        // edge; begin records it so terminal cleanup owes one HomeRelease.
        InvokeCallResultKind::Handle => {
            ledger.begin_handle_call_emission(&owned_site, lifecycle.target().callable().owner())?
        }
        _ => {}
    }
    let call = lifecycle
        .materialize_call(None, arguments)
        .map_err(|_| freeze("local-call-projection-failed"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let fault_landing = builder.next_block_id();
    let result = builder.next_value_id();
    // Every lifecycle-emitting local call records the shared frame
    // definition in its own group: under a Plain terminal exit there is no
    // Call entry `frame` field to carry it, and under a Call exit the entry
    // records the same pair again — coverage deduplicates on (block,
    // instruction), so the group stays authoritative in both shapes.
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    append_block(
        builder,
        fault_landing,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: result_kind,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    let result_class = match result_kind {
        InvokeCallResultKind::Handle => Some(
            lifecycle
                .target()
                .published_key()
                .and_then(|key| ledger.callable_result_class(key))
                .ok_or_else(|| freeze("handle-result-class-missing"))?,
        ),
        _ => None,
    };
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, result_type(result_kind, result_class)?);
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    match result_kind {
        InvokeCallResultKind::Map => ledger.record_map_emission(&owned_site, result, bindings)?,
        InvokeCallResultKind::Handle => {
            ledger.record_handle_call_emission(&owned_site, result, bindings)?
        }
        _ => ledger.record_root_local_call_bindings(owner, owned_site, bindings)?,
    }
    Ok(result)
}

/// Emit one source-issued lexical instance-call local result
/// (`local h = recv.m(...)`) whose co-sealed result contract is `Handle`.
/// The sealed local-call relation is sole membership, the disposition row
/// names the unique selected callee, the callee's retained
/// `Value(Construction)` terminal proves the transfer, and its minted
/// object is the identity the received handle keeps. The receiver value is
/// the ledger-installed binding value, never a re-lowered expression;
/// arguments come from the sealed literal relation, not a re-walk of the
/// AST. Prior homes (the receiver among them) stay live across the call.
pub(in crate::mir::builder) fn emit_local_lexical(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    if row.result() != Some(InvokeCallResultKind::Handle) {
        return Err(freeze("lexical-handle-result-mismatch"));
    }
    let relation = ledger
        .handle_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-handle-source-missing"))?;
    if relation.prior_homes().is_empty()
        || !relation.prior_homes().contains(&row.receiver_binding())
    {
        return Err(freeze("lexical-handle-receiver-home-missing"));
    }
    let unwind = ledger.handle_call_prior_home_unwind(&owned_site)?;
    if relation.arguments().len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-handle-arity-mismatch"));
    }
    ledger.begin_handle_call_emission(&owned_site, row.callee_owner())?;
    let receiver = state
        .take_exact_lexical_value(owner, row.receiver_site().node(), row.receiver_binding())
        .map_err(|error| format!("[freeze:contract][lexical-handle/receiver/{error:?}]"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = builder.next_value_id();
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    // The call's fault path unwinds the prior Homes exactly like a `new`
    // emission does — the receiver Home can never leak past a fault.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, &mut bindings)?;
    let arguments = relation
        .arguments()
        .iter()
        .map(|argument| -> Result<ValueId, String> {
            // The lexical-handle lane seals integer literals only — a scalar
            // or bool row here means a claim boundary the emitter never
            // admits, so fail closed instead of guessing a value.
            match argument {
                crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(
                    literal,
                ) => crate::mir::builder::emission::constant::emit_integer(builder, *literal),
                other => Err(format!(
                    "[freeze:contract][lexical-handle/argument-class]{other:?}"
                )),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call: MirCall::new(
                None,
                crate::mir::definitions::Callee::SameModuleInstance {
                    key: row.target().clone(),
                    receiver,
                },
                arguments,
            ),
            result: InvokeCallResultKind::Handle,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    let result_class = ledger
        .callable_result_class(row.target())
        .ok_or_else(|| freeze("handle-result-class-missing"))?;
    builder.function_state.type_ctx.value_types.insert(
        result,
        result_type(InvokeCallResultKind::Handle, Some(result_class))?,
    );
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    ledger.record_handle_call_emission(&owned_site, result, bindings)?;
    Ok(result)
}

/// Emit one source-issued lexical instance-call local result
/// (`local x = recv.m(...)`) whose co-sealed result contract is `I64`.
/// The sealed local-call relation is sole membership, the disposition row
/// names the unique selected callee, and the callee's literal-only exits
/// keep the exact-i64 contract — this lane never transfers a Home and
/// never consumes a handle argument. The receiver value is the
/// ledger-installed binding value; arguments come from the sealed
/// relation — integer literals and sealed integer-scalar bindings only,
/// zipped against the disposition's exact argument sites. Prior homes
/// (the receiver among them) stay live across the call and unwind
/// newest-first on the fault edge.
pub(in crate::mir::builder) fn emit_local_lexical_i64(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    row: LexicalInstanceCallDispositionRowV1,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    let relation = ledger
        .lexical_i64_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-i64-source-missing"))?;
    let mut bindings = Vec::new();
    let result = emit_lexical_i64_call(
        builder,
        state,
        ledger,
        owner,
        relation.prior_homes(),
        relation.arguments(),
        &row,
        &mut bindings,
    )?;
    // The disposition issuer already routed this site onto the lifecycle
    // lane; the exit entries claim the recorded group in source order —
    // nested argument-position calls fold their emitted instructions
    // into this one group (they have no destination binding of their
    // own).
    ledger.record_root_local_call_bindings(owner, owned_site, bindings)?;
    Ok(result)
}

/// Emit one proven-i64 lexical call — a `local x = recv.m(..)`
/// continuation or a `CallResult` argument nested inside one — from its
/// sealed evidence: the prior-Home list (receiver included), the sealed
/// argument rows, and the exact disposition. Every emitted instruction
/// lands on `bindings` in physical order so the enclosing site records
/// one coherent group.
fn emit_lexical_i64_call(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    prior_homes: &[crate::mir::resolved_semantics::BindingRefV1],
    sealed_arguments: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
    row: &LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<ValueId, String> {
    if row.result() != Some(InvokeCallResultKind::I64) {
        return Err(freeze("lexical-i64-result-mismatch"));
    }
    if prior_homes.is_empty() || !prior_homes.contains(&row.receiver_binding()) {
        return Err(freeze("lexical-i64-receiver-home-missing"));
    }
    let unwind = ledger.prior_home_unwind_for(prior_homes)?;
    if sealed_arguments.len() != row.argument_sites().len()
        || row.argument_sites().len() != row.target().arity() as usize
    {
        return Err(freeze("lexical-i64-arity-mismatch"));
    }
    let receiver = state
        .take_exact_lexical_value(owner, row.receiver_site().node(), row.receiver_binding())
        .map_err(|error| format!("[freeze:contract][lexical-i64/receiver/{error:?}]"))?;
    let frame = state.borrow_fault_frame(builder)?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = builder.next_value_id();
    bindings.push(fault_frame_binding(builder, state, frame)?);
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        bindings,
    )?;
    // The call's fault path unwinds the prior Homes exactly like the
    // Handle lane does — the receiver Home can never leak past a fault.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, bindings)?;
    let mut arguments = Vec::with_capacity(sealed_arguments.len());
    for (argument, site) in sealed_arguments.iter().zip(row.argument_sites().iter()) {
        // The lexical-i64 lane seals integer literals, integer-scalar
        // bindings, and proven-i64 nested calls only — any other row here
        // means a claim boundary the emitter never admits, so fail
        // closed instead of guessing a value.
        let value = match argument {
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Integer(
                literal,
            ) => crate::mir::builder::emission::constant::emit_integer(builder, *literal)?,
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::Scalar(
                binding,
            ) => state
                .take_exact_lexical_value(owner, site.node(), *binding)
                .map_err(|error| format!("[freeze:contract][lexical-i64/argument/{error:?}]"))?,
            crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::CallResult(
                inner,
            ) => {
                if inner.site().site() != site {
                    return Err(freeze("lexical-i64/argument-site-mismatch"));
                }
                let inner_row = ledger
                    .take_lexical_instance_call(owner, site)?
                    .ok_or_else(|| freeze("lexical-i64/inner-disposition-missing"))?;
                // The inner Invoke runs before this call and feeds the
                // argument slot; its instructions join this call's
                // recorded binding group in physical order.
                emit_lexical_i64_call(
                    builder,
                    state,
                    ledger,
                    owner,
                    inner.prior_homes(),
                    inner.arguments(),
                    &inner_row,
                    bindings,
                )?
            }
            other => {
                return Err(format!(
                    "[freeze:contract][lexical-i64/argument-class]{other:?}"
                ))
            }
        };
        arguments.push(value);
    }
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call: MirCall::new(
                None,
                crate::mir::definitions::Callee::SameModuleInstance {
                    key: row.target().clone(),
                    receiver,
                },
                arguments,
            ),
            result: InvokeCallResultKind::I64,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, result_type(InvokeCallResultKind::I64, None)?);
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    Ok(result)
}

/// Emit one source-issued `local x = me.m(..)` receiver call whose sealed
/// package observation carries the callee's `NullableObject` claim. The
/// flow row is sole membership, the observation row is the sole typed
/// argument/callee authority, and the `me` receiver arrives already
/// materialized from the declared-instance ingress — this lane never
/// re-lowers an expression or falls back to the generic call. The result
/// binds a nullable handle: the caller's exit owes `HomeReleaseIfLive`,
/// never an unconditional release.
pub(in crate::mir::builder) fn emit_receiver_nullable(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    receiver: ValueId,
) -> Result<ValueId, String> {
    let owned_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
    let relation = ledger
        .nullable_call_source(&owned_site)
        .ok_or_else(|| freeze("nullable-call-source-missing"))?;
    // The flow row proves membership; the observation row is the sealed
    // evidence that minted it. One without the other is a half-sealed
    // edge — freeze, never infer.
    let observation = ledger
        .receiver_call_observation(&owned_site)
        .ok_or_else(|| freeze("nullable-observation-missing"))?;
    let class = match observation.class() {
        crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(
            class,
        ) => class.clone(),
        _ => return Err(freeze("nullable-class-drift")),
    };
    let (_, destination) = relation
        .local_binding()
        .ok_or_else(|| freeze("nullable-call-destination-drift"))?;
    if observation.callee() != key || observation.destination() != destination {
        return Err(freeze("nullable-target-drift"));
    }
    if observation.arguments().len() != key.arity() as usize {
        return Err(freeze("nullable-arity-mismatch"));
    }
    ledger.begin_nullable_call_emission(&owned_site, key)?;
    let unwind = ledger.nullable_call_prior_home_unwind(&owned_site)?;
    let frame = state.borrow_fault_frame(builder)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal_landing = builder.next_block_id();
    let outward = builder.next_block_id();
    let result = builder.next_value_id();
    let mut bindings = vec![fault_frame_binding(builder, state, frame)?];
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    // The call's fault path unwinds the prior Homes exactly like every
    // other lifecycle call — the received nullable does not exist yet.
    let fault_landing = cleanup_chain(builder, frame, unwind, outward, &mut bindings)?;
    let mut arguments = Vec::with_capacity(observation.arguments().len());
    for argument in observation.arguments() {
        use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1;
        let value = match argument.kind() {
            SelectedNewArgumentKindV1::Integer(literal) => {
                crate::mir::builder::emission::constant::emit_integer(builder, *literal)?
            }
            SelectedNewArgumentKindV1::Local { binding } => {
                let value = state
                    .take_exact_lexical_value(owner, argument.site().node(), *binding)
                    .map_err(|error| format!("[freeze:contract][nullable-argument/{error:?}]"))?;
                // The sealed scalar call-edge rule (`check_call_edge`)
                // admits a recorded `Integer`/`Unknown` carrier — or an
                // unrecorded slot — and rejects every recorded concrete
                // non-i64 carrier; the recorded type is corroboration,
                // never a re-classification of the binding.
                match builder.function_state.type_ctx.value_types.get(&value) {
                    None | Some(MirType::Integer) | Some(MirType::Unknown) => {}
                    Some(_) => return Err(freeze("nullable-argument-carrier")),
                }
                value
            }
            // Bool/Null literal arguments stay unadmitted — their const
            // carriers record concrete non-i64 types the sealed scalar
            // edge corroborates as drift, never a guessed lane.
            _ => return Err(freeze("nullable-argument-carrier")),
        };
        arguments.push(value);
    }
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call: MirCall::new(
                None,
                crate::mir::definitions::Callee::SameModuleInstance {
                    key: key.clone(),
                    receiver,
                },
                arguments,
            ),
            result: InvokeCallResultKind::NullableHandle,
        },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    builder.function_state.type_ctx.value_types.insert(
        result,
        result_type(InvokeCallResultKind::NullableHandle, Some(class.as_ref()))?,
    );
    bindings.push((origin, invoke));
    bindings.push((normal_landing, projection));
    ledger.record_handle_call_emission(&owned_site, result, bindings)?;
    Ok(result)
}
