//! Physical terminal Call lowering uses the ledger's original literal relation.
//! The affine row stays owned by root exit progress after emission.
use super::*;
#[path = "terminal_call/lexical_i64.rs"]
mod lexical_i64;
#[path = "terminal_call/lexical_receiver.rs"]
mod lexical_receiver;
#[path = "terminal_call/lexical_object_expression.rs"]
mod lexical_object_expression;
pub(in crate::mir::builder) use lexical_object_expression::emit as emit_object_expression;
use crate::mir::definitions::MirCall;
use crate::mir::normal_callable_semantic_package::{
    DirectCallDispositionRowV1, LexicalInstanceCallDispositionRowV1, RootCallDispositionV1,
    RootInstanceCallDispositionRowV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
pub(in crate::mir::builder) use lexical_i64::prepare_arguments as prepare_lexical_arguments;
pub(in crate::mir::builder) use lexical_i64::{emit_constructor_unit, emit_unit_discard, emit_expression_call, emit_expression_call_with_prefix, emit_expression_call_with_dynamic_and_argument_frames, emit_bool_condition_call, emit_local_lexical_i64, emit_local_lexical_source, StaticExpressionCallIssuerV1};
#[path = "terminal_call/static_return.rs"]
mod static_return;
pub(in crate::mir::builder) use static_return::emit as emit_static_return;

pub(in crate::mir::builder::ordinary_new_admission) struct Emission {
    pub(super) source: Source,
    pub(super) arguments: Vec<(BasicBlockId, MirInstruction)>,
    pub(super) call: MirCall,
    pub(super) result: InvokeCallResultKind,
}

pub(in crate::mir::builder::ordinary_new_admission) enum Source {
    Existing(RootCallDispositionV1),
    Packet {
        row: crate::mir::normal_callable_semantic_package::CallPacketSourceV1,
        prepared: crate::mir::normal_callable_semantic_package::PreparedLexicalCallProjectionV1,
    },
}

impl Source {
    pub(super) fn deferred_argument_ends_v1(&self, ledger: &OrdinaryNewClaimLedgerV1)
        -> Result<Vec<InvokeOperation>, String> {
        match self {
            Self::Packet { prepared, .. } => prepared.deferred_argument_ends_v1(
                crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(ledger)),
            Self::Existing(RootCallDispositionV1::Lexical(_)) => Err(freeze("call-source-missing")),
            Self::Existing(_) => Ok(Vec::new()),
        }
    }
    pub(super) fn finish(
        self,
        invoke: (BasicBlockId, MirInstruction),
        projection: (BasicBlockId, MirInstruction),
        cleanup: Option<root_graph::RootArgumentCleanupV1>,
    ) -> Result<RootCallDispositionV1, String> {
        match self {
            Self::Existing(row) if cleanup.is_none() => Ok(row),
            Self::Existing(_) => Err(freeze("call-source-missing")),
            Self::Packet { row, prepared } => {
                let mut packet = crate::mir::normal_callable_semantic_package::EmittedLexicalCallProjectionV1::from_source(
                    row, prepared, invoke, projection);
                if let Some((ingress, continuation, normal, fault_suffix)) = cleanup {
                    packet = packet.with_root_argument_cleanup_v1(ingress, continuation, normal, fault_suffix)?;
                }
                Ok(RootCallDispositionV1::Lexical(std::rc::Rc::new(packet)))
            }
        }
    }
}

fn result_type(result: InvokeCallResultKind, class: Option<&str>) -> Result<MirType, String> {
    match result {
        InvokeCallResultKind::Unit => Err(freeze("unit-call-has-no-value")),
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
        InvokeCallResultKind::BorrowedNullableHandle { .. } => Ok(MirType::Unknown),
        InvokeCallResultKind::MixedHandle { .. } => Ok(MirType::Unknown),
        InvokeCallResultKind::I64 => Ok(MirType::Integer),
        InvokeCallResultKind::OwnedString => Ok(MirType::String),
        InvokeCallResultKind::DynamicCarrier => Ok(MirType::Unknown),
        InvokeCallResultKind::Bool => Ok(MirType::Bool),
    }
}

/// Sole physical Call/Normal pair for source-qualified lexical results.
/// Callers retain source/argument authority, reserve IDs in their original
/// order, and own the Fault cleanup and final inventory. This helper neither
/// invents a local destination nor transfers a Home.
fn emit_call_normal(
    builder: &mut MirBuilder,
    call: MirCall,
    result_kind: InvokeCallResultKind,
    frame: ValueId,
    normal_landing: BasicBlockId,
    fault_landing: BasicBlockId,
    result: ValueId,
    carrier: MirType,
) -> Result<
    ((BasicBlockId, MirInstruction), (BasicBlockId, MirInstruction)),
    String,
> {
    if result_kind == InvokeCallResultKind::Unit { return Err(freeze("unit-call-has-no-value")); }
    let invoke = emit_call_normal_start(builder, call, result_kind, frame, normal_landing, fault_landing)?;
    let origin = invoke.0;
    let projection = MirInstruction::InvokeNormalResult { invoke_block: origin, dst: result };
    builder.emit_instruction(projection.clone())?;
    builder.function_state.type_ctx.value_types.insert(result, carrier);
    Ok((invoke, (normal_landing, projection)))
}

/// The one actual Call and Normal edge, shared by value and Unit continuations.
/// Unit callers stop here: no result ID, carrier or projection is fabricated.
fn emit_call_normal_start(
    builder: &mut MirBuilder, call: MirCall, result_kind: InvokeCallResultKind,
    frame: ValueId, normal_landing: BasicBlockId, fault_landing: BasicBlockId,
) -> Result<(BasicBlockId, MirInstruction), String> {
    let origin = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call { call, result: result_kind },
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    builder.emit_instruction(invoke.clone())?;
    builder.start_new_block(normal_landing)?;
    Ok((origin, invoke))
}

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    row: DirectCallDispositionRowV1,
) -> Result<ValueId, String> {
    let emission = row.physical_emission().map_err(|_| freeze("direct-source-required"))?;
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
            source: Source::Existing(RootCallDispositionV1::Direct(row)),
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
            source: Source::Existing(RootCallDispositionV1::Instance(row)),
            arguments: Vec::new(),
            call,
            result,
        })),
    )
}

pub(super) fn emit_ingress(
    writer: &mut impl RootGraphWriterPortV1,
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
    let normal_landing = writer.reserve_block(builder)?;
    let fault_landing = writer.reserve_block(builder)?;
    for (id, target) in [(normal_landing, clean), (fault_landing, fault)] {
        writer.append_terminated(
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
    writer.insert_projection(builder, normal_landing, projection.clone())?;
    let invoke = MirInstruction::Invoke {
        operation,
        fault_frame: frame,
        normal_landing,
        fault_landing,
    };
    let emitted = writer.emit_current(builder, invoke)?;
    Ok((emitted, (normal_landing, projection)))
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
            ledger.begin_handle_call_emission(&owned_site, row.callee_owner_v1())?
        }
        _ => {}
    }
    let call = lifecycle
        .materialize_call(None, arguments.clone())
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
            row.published_key_v1()
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
    let invoke = (origin, invoke);
    let projection = (normal_landing, projection);
    bindings.push(invoke.clone());
    bindings.push(projection.clone());
    match result_kind {
        InvokeCallResultKind::Map => ledger.record_map_emission(&owned_site, result, bindings)?,
        InvokeCallResultKind::Handle => {
            ledger.record_handle_call_emission(&owned_site, result, bindings)?
        }
        InvokeCallResultKind::I64 => ledger.record_root_direct_scalar_call_bindings_v1(
            row, relation.clone(), arguments, invoke, projection, bindings,
            builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?)?,
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
        || !relation.prior_homes().contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(row.receiver_binding()?))
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
        .take_exact_lexical_value(owner, row.receiver_site().node(), row.receiver_binding()?)
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

#[path = "terminal_call/lexical_return.rs"]
mod lexical_return;
pub(in crate::mir::builder) use lexical_return::emit as emit_borrowed_return;

#[path = "terminal_call/lexical_nullable.rs"]
mod lexical_nullable;
pub(in crate::mir::builder) use lexical_nullable::emit_local_lexical_object;
pub(in crate::mir::builder) use lexical_nullable::emit_receiver_object;
