//! Existing local/nested lexical I64 emission from sealed source rows.
use super::*;
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
type Binding = (crate::mir::BasicBlockId, MirInstruction);

/// Physical observations of the original ordered argument tree. These rows
/// cannot classify a source argument or choose a call target.
#[derive(Debug)]
enum ArgumentProjection {
    Integer(Binding),
    Scalar(ExactLexicalReadV1),
    CallResult(Box<EmittedCall>),
}

#[derive(Debug)]
struct PreparedCall {
    receiver: ExactLexicalReadV1,
    arguments: Vec<ArgumentProjection>,
}

#[derive(Debug)]
struct EmittedCall {
    row: LexicalInstanceCallDispositionRowV1,
    prepared: PreparedCall,
    invoke: Binding,
    projection: Binding,
}

impl PreparedCall {
    fn materialize(
        &self,
        owner: FunctionOwnerIdV1,
        row: &LexicalInstanceCallDispositionRowV1,
        source: &[LocalCallArgumentV1],
    ) -> Result<MirCall, String> {
        if row.call_site().owner() != owner
            || self.arguments.len() != source.len()
            || source.len() != row.argument_sites().len()
            || source.len() != row.target().arity() as usize
        {
            return Err(freeze("lexical-i64/prepared-arity-or-owner"));
        }
        let receiver = self
            .receiver
            .value_for(owner, row.receiver_site().node(), row.receiver_binding())
            .map_err(|error| format!("[freeze:contract][lexical-i64/receiver/{error:?}]"))?;
        let mut values = Vec::with_capacity(source.len());
        for ((projection, source), site) in
            self.arguments.iter().zip(source).zip(row.argument_sites())
        {
            let value = match (projection, source) {
                (
                    ArgumentProjection::Integer((
                        _,
                        MirInstruction::Const {
                            dst,
                            value: crate::mir::ConstValue::Integer(actual),
                        },
                    )),
                    LocalCallArgumentV1::Integer(expected),
                ) if actual == expected => *dst,
                (ArgumentProjection::Scalar(read), LocalCallArgumentV1::Scalar(binding)) => read
                    .value_for(owner, site.node(), *binding)
                    .map_err(|error| {
                        format!("[freeze:contract][lexical-i64/argument/{error:?}]")
                    })?,
                (
                    ArgumentProjection::CallResult(emitted),
                    LocalCallArgumentV1::CallResult(inner),
                ) if inner.site().owner() == owner
                    && inner.site().site() == site
                    && emitted.row.call_site() == inner.site() =>
                {
                    emitted.value_for_source(owner, inner.arguments())?
                }
                _ => return Err(freeze("lexical-i64/argument-projection-drift")),
            };
            values.push(value);
        }
        Ok(MirCall::new(
            None,
            crate::mir::definitions::Callee::SameModuleInstance {
                key: row.target().clone(),
                receiver,
            },
            values,
        ))
    }
}

impl EmittedCall {
    fn value_for_source(
        &self,
        owner: FunctionOwnerIdV1,
        source: &[LocalCallArgumentV1],
    ) -> Result<ValueId, String> {
        let expected = self.prepared.materialize(owner, &self.row, source)?;
        let MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
            normal_landing,
            ..
        } = &self.invoke.1
        else {
            return Err(freeze("lexical-i64/nested-invoke-shape"));
        };
        let MirInstruction::InvokeNormalResult { invoke_block, dst } = &self.projection.1 else {
            return Err(freeze("lexical-i64/nested-projection-shape"));
        };
        if self.row.result() != Some(InvokeCallResultKind::I64)
            || *call != expected
            || *invoke_block != self.invoke.0
            || *normal_landing != self.projection.0
        {
            return Err(freeze("lexical-i64/nested-producer-drift"));
        }
        Ok(*dst)
    }
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
    if row.call_site() != &owned_site {
        return Err(freeze("lexical-i64/call-site-drift"));
    }
    let relation = ledger
        .lexical_i64_call_source(&owned_site)
        .ok_or_else(|| freeze("lexical-i64-source-missing"))?;
    let mut bindings = Vec::new();
    let emitted = emit_lexical_i64_call(
        builder,
        state,
        ledger,
        owner,
        relation.prior_homes(),
        relation.arguments(),
        row,
        &mut bindings,
    )?;
    // The disposition issuer already routed this site onto the lifecycle
    // lane; the exit entries claim the recorded group in source order —
    // nested argument-position calls fold their emitted instructions
    // into this one group (they have no destination binding of their
    // own).
    let result = emitted.value_for_source(owner, relation.arguments())?;
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
    row: LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<(crate::mir::BasicBlockId, MirInstruction)>,
) -> Result<EmittedCall, String> {
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
        .take_exact_lexical_read(owner, row.receiver_site().node(), row.receiver_binding())
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
    let prepared = prepare_arguments(
        builder,
        state,
        ledger,
        owner,
        receiver,
        sealed_arguments,
        &row,
        bindings,
    )?;
    let call = prepared.materialize(owner, &row, sealed_arguments)?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
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
    let invoke = (origin, invoke);
    let projection = (normal_landing, projection);
    bindings.push(invoke.clone());
    bindings.push(projection.clone());
    Ok(EmittedCall {
        row,
        prepared,
        invoke,
        projection,
    })
}

/// Materialize only the source-issued receiver/ordered arguments. The caller
/// owns its outer Invoke; nested calls retain their original affine rows here.
fn prepare_arguments(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    receiver: ExactLexicalReadV1,
    source: &[LocalCallArgumentV1],
    row: &LexicalInstanceCallDispositionRowV1,
    bindings: &mut Vec<Binding>,
) -> Result<PreparedCall, String> {
    let mut arguments = Vec::with_capacity(source.len());
    for (argument, site) in source.iter().zip(row.argument_sites()) {
        let projection = match argument {
            LocalCallArgumentV1::Integer(literal) => {
                let block = builder
                    .function_state
                    .current_block
                    .ok_or_else(|| freeze("no-block"))?;
                let dst = crate::mir::builder::emission::constant::emit_integer(builder, *literal)?;
                let producer = (
                    block,
                    MirInstruction::Const {
                        dst,
                        value: crate::mir::ConstValue::Integer(*literal),
                    },
                );
                bindings.push(producer.clone());
                ArgumentProjection::Integer(producer)
            }
            LocalCallArgumentV1::Scalar(binding) => ArgumentProjection::Scalar(
                state
                    .take_exact_lexical_read(owner, site.node(), *binding)
                    .map_err(|error| {
                        format!("[freeze:contract][lexical-i64/argument/{error:?}]")
                    })?,
            ),
            LocalCallArgumentV1::CallResult(inner) => {
                if inner.site().owner() != owner || inner.site().site() != site {
                    return Err(freeze("lexical-i64/argument-site-mismatch"));
                }
                let inner_row = ledger
                    .take_lexical_instance_call(owner, site)?
                    .ok_or_else(|| freeze("lexical-i64/inner-disposition-missing"))?;
                ArgumentProjection::CallResult(Box::new(emit_lexical_i64_call(
                    builder,
                    state,
                    ledger,
                    owner,
                    inner.prior_homes(),
                    inner.arguments(),
                    inner_row,
                    bindings,
                )?))
            }
            other => {
                return Err(format!(
                    "[freeze:contract][lexical-i64/argument-class]{other:?}"
                ))
            }
        };
        arguments.push(projection);
    }
    Ok(PreparedCall {
        receiver,
        arguments,
    })
}

#[cfg(test)]
#[path = "lexical_i64_tests.rs"]
mod tests;
