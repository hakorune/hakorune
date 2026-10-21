//! Physical consumption of one prepared exact New claim, never a target issuer.
use crate::mir::instruction::InvokeCallResultKind;
#[path = "selected/arguments.rs"]
mod arguments;
#[path = "selected/invoke_operation.rs"]
mod invoke_operation;
#[path = "selected/owned_array.rs"]
pub(in crate::mir::builder) mod owned_array;
#[path = "selected/read_argument.rs"]
pub(in crate::mir::builder) mod read_argument;
#[path = "selected/map.rs"]
pub(in crate::mir::builder) mod map;
#[path = "selected/root_graph.rs"]
pub(in crate::mir::builder) mod root_graph;
#[path = "selected/bool_condition.rs"]
pub(in crate::mir::builder) mod bool_condition;
#[path = "selected/expression_value.rs"]
mod expression_value;
pub(in crate::mir::builder) use expression_value::emit_scalar_operation_to_landing_v1;
#[path = "selected/return_expression.rs"]
mod return_expression;
#[path = "selected/terminal_map_get.rs"]
pub(in crate::mir::builder) mod terminal_map_get;
pub(in crate::mir::builder) use return_expression::emit as emit_return_expression_v1;
#[path = "selected/binding_rebind_expression.rs"]
pub(in crate::mir::builder) mod binding_rebind_expression;
#[path = "selected/initializer_expression.rs"]
pub(in crate::mir::builder) mod initializer_expression;
#[path = "selected/print_expression.rs"]
pub(in crate::mir::builder) mod print_expression;
#[path = "selected/terminal_call.rs"]
pub(in crate::mir::builder) mod terminal_call;
#[path = "selected/source_loop_cleanup.rs"]
pub(in crate::mir::builder) mod source_loop_cleanup;
use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewTrivialArgumentV1, PreparedTerminalI64AddReturnV1, PreparedTerminalI64FieldReturnV1,
    PreparedTerminalMapGetReturnV1, RootHomeFaultSequencesV1, RootHomeReleaseOriginV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{
    BasicBlock, BasicBlockId, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};
use root_graph::{cleanup_step_with_writer, RawRootGraphWriterV1, RootGraphWriterPortV1};

pub(in crate::mir::builder) trait RootExitFramePortV1 {
    fn materialize(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String>;
    fn validate(&self, function: &MirFunction) -> Result<(), String>;
}

impl RootExitFramePortV1 for CallableSemanticLoweringState {
    fn materialize(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        self.borrow_fault_frame(builder)
    }

    fn validate(&self, function: &MirFunction) -> Result<(), String> {
        self.validate_fault_frame(function)
    }
}

impl RootExitFramePortV1 for FunctionFaultFrameV1 {
    fn materialize(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        FunctionFaultFrameV1::materialize(self, builder)
    }

    fn validate(&self, function: &MirFunction) -> Result<(), String> {
        FunctionFaultFrameV1::validate(self, function)
    }
}

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: OrdinaryNewAdmissionClaimV1,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() {
        return Err(freeze("owner-or-argument-count"));
    }
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let arity = claim.arity();
    let rows = claim
        .argument_rows()
        .map_err(|_| freeze("argument-source-unavailable"))?
        .to_vec();
    let constructor = claim.constructor();
    emit_selected_new(
        builder,
        state,
        ledger,
        site,
        object,
        class,
        arity,
        constructor,
        &rows, consume_receiver, static_source,
    )
}

/// Physical consumption of one return-position result claim. The emitted
/// object value is the `Return { value }` operand the claim stands for —
/// one emit shape, no local destination step.
pub(in crate::mir::builder) fn emit_result(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: crate::mir::normal_callable_semantic_package::OrdinaryNewResultClaimV1,
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() {
        return Err(freeze("owner-or-argument-count"));
    }
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let arity = claim.arity();
    let rows = claim
        .argument_rows()
        .map_err(|_| freeze("argument-source-unavailable"))?
        .to_vec();
    let constructor = claim.constructor();
    emit_selected_new(
        builder,
        state,
        ledger,
        site,
        object,
        class,
        arity,
        constructor,
        &rows, consume_receiver, static_source,
    )
}

#[allow(clippy::too_many_arguments)]
fn emit_selected_new(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: crate::mir::resolved_semantics::OwnedExprSiteV1,
    object: hakorune_mir_defs::CanonicalObjectIdV1,
    class: String,
    arity: usize,
    constructor: OrdinaryNewConstructorDispositionV1,
    argument_rows: &[OrdinaryNewTrivialArgumentV1],
    consume_receiver: &dyn Fn(&crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1) -> Result<(), String>,
    static_source: Option<&terminal_call::StaticExpressionCallIssuerV1<'_>>,
) -> Result<ValueId, String> {
    let arguments = arguments::materialize_arguments(builder, state, ledger, &site, arity,
        argument_rows, consume_receiver, static_source)?;
    let (mut prior, reclaim_origin) = ledger.begin_new_emission(&site)?;
    let argument_ends = arguments::temporary_ends_v1(&arguments)?;
    prior.splice(0..0, argument_ends);
    let frame = state.borrow_fault_frame(builder)?;
    let result = builder.next_value_id();
    let frame_binding = fault_frame_binding(builder, state, frame)?;
    let mut bindings = vec![frame_binding];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let allocation_fault = cleanup_chain(builder, frame, prior, outward, &mut bindings)?;
    let mut reclaim = None;
    let birth_fault = if matches!(constructor, OrdinaryNewConstructorDispositionV1::Birth(_)) {
        let origin = reclaim_origin.ok_or_else(|| freeze("reclaim-origin-missing"))?;
        // Birth discharged its source-sealed partial fields before Fault.
        // This caller reclaims only unpublished storage, then prior Homes.
        let tail = cleanup_step(
            builder,
            frame,
            InvokeOperation::ReclaimUnpublished {
                object: origin.object(),
                value: result,
            },
            allocation_fault,
            allocation_fault,
            &mut bindings,
        )?;
        let (block, instruction) = bindings
            .last()
            .cloned()
            .ok_or_else(|| freeze("reclaim-origin-binding-missing"))?;
        reclaim = Some((origin.clone(), block, instruction));
        tail
    } else {
        if reclaim_origin.is_some() {
            return Err(freeze("reclaim-origin-unexpected"));
        }
        allocation_fault
    };
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let allocation = MirInstruction::Invoke {
        operation: InvokeOperation::NewBox { object },
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: allocation_fault,
    };
    builder.emit_instruction(allocation.clone())?;
    let allocation_binding = (origin, allocation);
    bindings.push(allocation_binding.clone());
    builder.start_new_block(normal)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    bindings.push((normal, projection));
    // Existing type metadata is a one-way source projection, not object identity.
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Box(class.clone()));
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(result, class);
    let mut birth_binding = None;
    if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
        let effects = recipe.physical_effect_mask();
        let call = crate::mir::instruction::BirthInvokeV1 {
            key: recipe.target(), receiver: result, arguments: arguments.iter().map(|argument| argument.payload().clone()).collect(), effects,
        };
        let after_birth = builder.next_block_id();
        let birth = MirInstruction::Invoke {
            operation: InvokeOperation::Birth(call),
            fault_frame: frame,
            normal_landing: after_birth,
            fault_landing: birth_fault,
        };
        builder.emit_instruction(birth.clone())?;
        birth_binding = Some((normal, birth.clone()));
        bindings.push((normal, birth));
        builder.start_new_block(after_birth)?;
    }
    arguments::finish_new_arguments_v1(builder, ledger, &site, result, frame, &arguments, &mut bindings)?;
    let continuation = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
    let cleanup = if arguments.iter().any(|argument| argument.expression_v1().is_some()) {
        Some(crate::mir::normal_callable_semantic_package::NewArgumentCleanupV1::new(allocation_binding, birth_binding, continuation))
    } else { None };
    for argument in &arguments {
        if let Some(expression) = argument.expression_v1() {
            for binding in expression.bindings() {
                if bindings.contains(binding) {
                    if matches!(binding.1, MirInstruction::FaultFrameEnter { .. }) { continue; }
                    return Err(freeze("emission-binding-drift"));
                }
                bindings.push(binding.clone());
            }
        }
    }
    ledger.record_new_argument_emission_v1(&site, result, arguments, cleanup, reclaim, bindings)?;
    Ok(result)
}

pub(in crate::mir::builder) fn emit_root_home_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    value: ValueId,
) -> Result<ValueId, String> {
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        Some(value),
        value,
        None,
    )
}

pub(in crate::mir::builder) fn emit_root_home_unit_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
) -> Result<ValueId, String> {
    let statement_result = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        site,
        None,
        statement_result,
        None,
    )
}

/// One physically-emitting terminal ingress: the existing terminal Call or
/// the readable-Map checked get. Both share the same Normal/Fault cleanup
/// graph; the ledger entry kind keeps them apart.
pub(super) enum RootExitIngress {
    Call(terminal_call::Emission),
    MapGet { operation: InvokeOperation, stored: Option<Box<crate::mir::normal_callable_semantic_package::StoredMapGetReturnEmissionV1>> },
}

/// Exact physical graph evidence. The ledger wrapper retains its affine
/// source rows; the selected canonical caller can validate this receipt before
/// its post-collector commit.
pub(in crate::mir::builder) struct RootGraphEmissionV1 {
    pub origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
    pub bindings: Vec<(BasicBlockId, MirInstruction)>,
    pub ingress: Option<(
        (BasicBlockId, MirInstruction),
        (BasicBlockId, MirInstruction),
    )>,
    pub frame: Option<ValueId>,
    pub clean: BasicBlockId,
}

fn emit_root_home_exit_payload(
    builder: &mut MirBuilder,
    frame_port: &mut impl RootExitFramePortV1,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
    ingress: Option<RootExitIngress>,
) -> Result<ValueId, String> {
    let operations = ledger.begin_root_home_exit(owner, site)?;
    if let Some(RootExitIngress::Call(call)) = &ingress {
        ledger.bind_root_direct_result_v1(owner, site, statement_result, call.result)?;
    } else if ingress.is_none() && return_value.is_some() {
        ledger.bind_root_fresh_result_v1(owner, site, statement_result)?;
    }
    if ingress.is_none() {
        if let Some(value) = return_value {
            let block = builder.function_state.current_block.ok_or_else(|| freeze("no-block"))?;
            if let Some(hold) = ledger.bind_root_text_return_hold_v1(owner, site, block, value)? {
                builder.emit_instruction(hold)?;
            }
        }
    }
    let fault_sequences = ledger.root_home_fault_sequences(owner, site)?;
    let ingress_operation = match &ingress {
        Some(RootExitIngress::Call(call)) => Some(InvokeOperation::Call {
            call: call.call.clone(),
            result: call.result,
        }),
        Some(RootExitIngress::MapGet { operation, .. }) => Some(operation.clone()),
        None => None,
    };
    let emission = emit_root_graph_v1(
        builder,
        frame_port,
        &mut RawRootGraphWriterV1,
        operations,
        fault_sequences,
        return_value,
        statement_result,
        ingress_operation,
    )?;
    let RootGraphEmissionV1 {
        origins,
        bindings,
        ingress: physical_ingress,
        frame,
        clean: _,
    } = emission;
    match (ingress, physical_ingress) {
        (Some(RootExitIngress::Call(call)), Some((invoke, projection))) => {
            let frame = frame.ok_or_else(|| freeze("root-frame-missing"))?;
            let frame_binding = fault_frame_binding_with_port(builder, frame_port, frame)?;
            ledger.record_root_call_exit(
                owner,
                site,
                call.source.finish(invoke.clone(), projection.clone()),
                call.arguments,
                invoke,
                projection,
                frame_binding,
                origins,
                bindings,
            )?;
        }
        (Some(RootExitIngress::MapGet { stored, .. }), Some((invoke, projection))) => {
            let frame = frame.ok_or_else(|| freeze("root-frame-missing"))?;
            let frame_binding = fault_frame_binding_with_port(builder, frame_port, frame)?;
            ledger.record_root_map_get_exit(
                owner,
                site,
                invoke,
                projection,
                frame_binding,
                origins,
                bindings, stored,
                builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?,
            )?;
        }
        (None, None) => ledger.record_root_home_exit(owner, site, origins, bindings)?,
        _ => return Err(freeze("root-ingress-graph-mismatch")),
    }
    Ok(statement_result)
}

/// Implicit source end uses the same graph writer and the body's existing frame.
pub(in crate::mir::builder) fn emit_implicit_root_home_exit_v1(
    builder: &mut MirBuilder, state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1, owner: FunctionOwnerIdV1,
    completion: std::rc::Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    producer: (BasicBlockId, MirInstruction), value: ValueId,
) -> Result<(), String> {
    use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;
    ledger.prepare_implicit_root_home_exit_v1(owner, &completion)?;
    let exit = RootHomeExitKeyV1::ImplicitBodyEnd;
    let operations = ledger.begin_root_home_exit_for_key_v1(owner, &exit)?;
    let faults = ledger.root_home_fault_sequences_for_exit_v1(owner, &exit)?;
    let emission = emit_root_graph_v1(builder, state, &mut RawRootGraphWriterV1,
        operations, faults, Some(value), value, None)?;
    ledger.record_implicit_root_home_exit_v1(owner, completion, producer,
        emission.origins, emission.bindings)
}

/// Sole root Home graph algorithm for raw and canonical writer ports. Source
/// order and fault suffixes are supplied by the existing Home ledger owner.
pub(in crate::mir::builder) fn emit_root_graph_v1(
    builder: &mut MirBuilder,
    frame_port: &mut impl RootExitFramePortV1,
    writer: &mut impl RootGraphWriterPortV1,
    operations: Vec<RootHomeReleaseOriginV1>,
    fault_sequences: RootHomeFaultSequencesV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
    ingress_operation: Option<InvokeOperation>,
) -> Result<RootGraphEmissionV1, String> {
    let fault_operations: Vec<_> = fault_sequences
        .full
        .iter()
        .map(|origin| origin.operation().clone())
        .collect();
    let mut bindings = Vec::new();
    let mut clean = writer.reserve_block(builder)?;
    writer.append_terminated(
        builder,
        clean,
        MirInstruction::Return {
            value: return_value,
        },
        &mut bindings,
    )?;
    let mut origins = Vec::with_capacity(operations.len());
    let mut frame_value = None;
    let mut ingress = None;
    // Empty-Home Plain has no Fault edge and therefore no frame definition.
    if !operations.is_empty() || ingress_operation.is_some() {
        let frame = frame_port.materialize(builder)?;
        frame_value = Some(frame);
        let cleanup = root_graph::emit_cleanup_suffixes_with_writer(
            writer, builder, frame, clean,
            &operations.iter().map(|origin| origin.operation().clone()).collect::<Vec<_>>(),
            &fault_operations, &fault_sequences.after_normal,
            if ingress_operation.is_some() { &fault_sequences.acquisition } else { &[] },
            &mut bindings,
        )?;
        clean = cleanup.clean;
        origins.extend(operations.into_iter().zip(cleanup.normal_origins)
            .map(|(origin, (block, instruction))| (origin, block, instruction)));
        let fault = if ingress_operation.is_some() {
            cleanup.acquisition_fault
        } else {
            cleanup.fault_terminal
        };
        if let Some(operation) = ingress_operation {
            ingress = Some(terminal_call::emit_ingress(
                writer,
                builder,
                frame,
                statement_result,
                clean,
                fault,
                operation,
                &mut bindings,
            )?);
        }
    }
    if ingress.is_none() {
        bindings.push(writer.emit_current(
            builder,
            MirInstruction::Jump {
                target: clean,
                edge_args: None,
            },
        )?);
    }
    Ok(RootGraphEmissionV1 {
        origins,
        bindings,
        ingress,
        frame: frame_value,
        clean,
    })
}

pub(in crate::mir::builder) fn emit_terminal_i64_add_return(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64AddReturnV1,
) -> Result<ValueId, String> {
    let return_site = prepared.return_site.clone();
    let mut values = [ValueId(0); 2];
    for (index, (site, base, field)) in prepared.reads.into_iter().enumerate() {
        let block = builder
            .function_state
            .current_block
            .ok_or_else(|| freeze("no-block"))?;
        let dst = builder.next_value_id();
        builder.emit_instruction(MirInstruction::ObjectFieldGet {
            dst,
            base,
            field: field.clone(),
        })?;
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, MirType::Integer);
        ledger.record_terminal_field_read(&site, block, dst, base, field)?;
        values[index] = dst;
    }
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let result = builder.next_value_id();
    builder.emit_instruction(MirInstruction::BinOp {
        dst: result,
        op: crate::mir::BinaryOp::Add,
        lhs: values[0],
        rhs: values[1],
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    ledger.record_terminal_i64_add(prepared.owner, &return_site, block, result, values[0], values[1])?;
    ledger.complete_terminal_i64_add_return(prepared.owner, &return_site, result)?;
    Ok(result)
}

pub(in crate::mir::builder) fn emit_terminal_i64_field_return(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64FieldReturnV1,
) -> Result<ValueId, String> {
    let block = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let result = builder.next_value_id();
    builder.emit_instruction(MirInstruction::ObjectFieldGet {
        dst: result,
        base: prepared.base,
        field: prepared.field.clone(),
    })?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    ledger.record_terminal_field_read(
        &prepared.site,
        block,
        result,
        prepared.base,
        prepared.field,
    )?;
    ledger.record_terminal_i64_field_return(
        prepared.site.owner(),
        &prepared.return_site,
        result,
    )?;
    Ok(result)
}

/// Emit the bounded readable-Map terminal: one checked `get` read whose i64
/// projection becomes the returned value. The ingress shares the terminal
/// Call's Normal/Fault cleanup graph — an owned map local still owes its
/// End on both paths; a borrowed formal releases nothing here.
pub(in crate::mir::builder) fn emit_terminal_map_get_return(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    prepared: PreparedTerminalMapGetReturnV1,
) -> Result<ValueId, String> {
    let value = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Integer);
    emit_root_home_exit_payload(
        builder,
        state,
        ledger,
        owner,
        &prepared.return_site,
        Some(value),
        value,
        Some(RootExitIngress::MapGet {
            operation: InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::CheckedGetI64 {
                map: prepared.map, utf8: prepared.utf8,
            }), stored: None,
        }),
    )
}

pub(in crate::mir::builder) fn cleanup_chain(
    builder: &mut MirBuilder,
    frame: ValueId,
    operations: Vec<InvokeOperation>,
    tail: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    let mut next = tail;
    // Build links backwards; execution preserves the source-issued order.
    for operation in operations.into_iter().rev() {
        next = cleanup_step(builder, frame, operation, next, next, bindings)?;
    }
    Ok(next)
}

fn cleanup_step(
    builder: &mut MirBuilder,
    frame: ValueId,
    operation: InvokeOperation,
    normal_next: BasicBlockId,
    fault_next: BasicBlockId,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<BasicBlockId, String> {
    cleanup_step_with_writer(
        &mut RawRootGraphWriterV1,
        builder,
        frame,
        operation,
        normal_next,
        fault_next,
        bindings,
    )
}

pub(in crate::mir::builder) fn append_block(
    builder: &mut MirBuilder,
    id: BasicBlockId,
    terminator: MirInstruction,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    RawRootGraphWriterV1.append_terminated(builder, id, terminator, bindings)
}

pub(in crate::mir::builder) fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/emission/{reason}]")
}

pub(in crate::mir::builder) fn fault_frame_binding(
    builder: &MirBuilder,
    state: &CallableSemanticLoweringState,
    frame: ValueId,
) -> Result<(BasicBlockId, MirInstruction), String> {
    fault_frame_binding_with_port(builder, state, frame)
}

fn fault_frame_binding_with_port(
    builder: &MirBuilder,
    frame_port: &impl RootExitFramePortV1,
    frame: ValueId,
) -> Result<(BasicBlockId, MirInstruction), String> {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("no-function"))?;
    frame_port.validate(function)?;
    let entry = function
        .blocks
        .get(&function.entry_block)
        .ok_or_else(|| freeze("no-entry"))?;
    let mut definitions = entry.all_instructions().filter(|instruction|
        matches!(instruction, MirInstruction::FaultFrameEnter { dst, .. } if *dst == frame));
    let definition = definitions
        .next()
        .ok_or_else(|| freeze("frame-definition-missing"))?
        .clone();
    if definitions.next().is_some() {
        return Err(freeze("frame-definition-duplicate"));
    }
    Ok((function.entry_block, definition))
}

#[path = "selected/field_array_append.rs"]
pub(in crate::mir::builder) mod field_array_append;

#[path = "selected/borrowed_field_write.rs"]
pub(in crate::mir::builder) mod borrowed_field_write;

#[path = "selected/checked_field_write.rs"]
pub(in crate::mir::builder) mod checked_field_write;

#[path = "selected/argument_expression.rs"]
mod argument_expression;
pub(in crate::mir::builder) use argument_expression::emit as emit_argument_expression_v1;
