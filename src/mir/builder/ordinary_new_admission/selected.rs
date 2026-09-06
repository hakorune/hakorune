//! Physical consumption of one prepared exact New claim, never a target issuer.
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewAdmissionClaimV1, OrdinaryNewClaimLedgerV1, OrdinaryNewConstructorDispositionV1,
    OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1, PreparedTerminalI64AddReturnV1,
};
use crate::mir::{BasicBlock, BasicBlockId, Callee, MirBuilder, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) fn emit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    claim: OrdinaryNewAdmissionClaimV1,
) -> Result<ValueId, String> {
    if state.owner() != claim.site().owner() {
        return Err(freeze("owner-or-argument-count"));
    }
    let arguments = materialize_arguments(builder, state, &claim)?;
    let (prior, reclaim_origin) = ledger.begin_new_emission(claim.site())?;
    let site = claim.site().clone();
    let object = claim.object();
    let class = claim.class().to_owned();
    let frame = state.borrow_fault_frame(builder)?;
    let result = builder.next_value_id();
    let frame_binding = {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("no-function"))?;
        state.validate_fault_frame(function)?;
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
        (function.entry_block, definition)
    };
    let mut bindings = vec![frame_binding];
    let outward = builder.next_block_id();
    append_block(
        builder,
        outward,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let prior_operations = prior
        .into_iter()
        .map(|(object, value)| InvokeOperation::HomeRelease { object, value })
        .collect::<Vec<_>>();
    let allocation_fault = cleanup_chain(builder, frame, prior_operations, outward, &mut bindings)?;
    let constructor = claim.constructor();
    let mut reclaim = None;
    let birth_fault = if matches!(constructor, OrdinaryNewConstructorDispositionV1::Birth(_)) {
        let origin = reclaim_origin.ok_or_else(|| freeze("reclaim-origin-missing"))?;
        let fault = cleanup_chain(
            builder,
            frame,
            vec![InvokeOperation::ReclaimUnpublished {
                object: origin.object(),
                value: result,
            }],
            allocation_fault,
            &mut bindings,
        )?;
        let (block, instruction) = bindings
            .last()
            .cloned()
            .ok_or_else(|| freeze("reclaim-origin-binding-missing"))?;
        reclaim = Some((origin, block, instruction));
        fault
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
    bindings.push((origin, allocation));
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
    if let OrdinaryNewConstructorDispositionV1::Birth(recipe) = constructor {
        let effects = recipe.physical_effect_mask();
        let MirInstruction::Call(call) = MirInstruction::call(
            None,
            Callee::BirthConstructor {
                key: recipe.target(),
                receiver: result,
            },
            arguments.clone(),
            effects,
        ) else {
            unreachable!("canonical Call constructor")
        };
        let after_birth = builder.next_block_id();
        let birth = MirInstruction::Invoke {
            operation: InvokeOperation::Call(call),
            fault_frame: frame,
            normal_landing: after_birth,
            fault_landing: birth_fault,
        };
        builder.emit_instruction(birth.clone())?;
        bindings.push((normal, birth));
        builder.start_new_block(after_birth)?;
    }
    ledger.record_new_emission(&site, result, arguments, reclaim, bindings)?;
    Ok(result)
}

fn materialize_arguments(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    claim: &OrdinaryNewAdmissionClaimV1,
) -> Result<Vec<ValueId>, String> {
    let rows = claim
        .argument_rows()
        .map_err(|_| freeze("argument-source-unavailable"))?;
    validate_argument_rows_v1(claim.site().owner(), claim.site(), claim.arity(), rows)?;
    rows.iter()
        .map(|row| match row.kind() {
            OrdinaryNewTrivialArgumentKindV1::Integer(value) => {
                crate::mir::builder::emission::constant::emit_integer(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Bool(value) => {
                crate::mir::builder::emission::constant::emit_bool(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Local { binding } => {
                let value = state
                    .value_for_exact_binding(claim.site().owner(), *binding)
                    .map_err(|_| freeze("argument-binding-unavailable"))?;
                state
                    .observe_variable_site(row.site().node(), *binding, value)
                    .map_err(|_| freeze("argument-local-observation"))?;
                Ok(value)
            }
        })
        .collect()
}

fn validate_argument_rows_v1(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    arity: usize,
    rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<(), String> {
    if rows.len() != arity {
        return Err(freeze("argument-row-count"));
    }
    let mut sites = std::collections::BTreeSet::new();
    for (index, row) in rows.iter().enumerate() {
        let ordinal = u32::try_from(index).map_err(|_| freeze("argument-ordinal-overflow"))?;
        if row.owner() != owner
            || row.new_site() != new_site
            || row.ordinal() != ordinal
            || !sites.insert(row.site().clone())
        {
            return Err(freeze("argument-row-drift"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };

    fn site(segments: Vec<SourcePathSegmentV1>) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            issuer.issue().expect("owner"),
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments)),
        )
    }

    fn integer_row(
        new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        ordinal: u32,
        argument: SourceExprSiteV1,
    ) -> OrdinaryNewTrivialArgumentV1 {
        OrdinaryNewTrivialArgumentV1::new(
            owner,
            new_site.clone(),
            ordinal,
            argument,
            OrdinaryNewTrivialArgumentKindV1::Integer(1),
        )
    }

    #[test]
    fn selected_new_argument_rows_reject_malformed_source_relations() {
        let new_site = site(vec![SourcePathSegmentV1::Body(0)]);
        let argument = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Argument(0),
        ]));
        let valid = integer_row(&new_site, new_site.owner(), 0, argument.clone());
        assert!(
            validate_argument_rows_v1(new_site.owner(), &new_site, 1, &[])
                .unwrap_err()
                .contains("argument-row-count")
        );
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            2,
            &[
                valid.clone(),
                integer_row(&new_site, new_site.owner(), 1, argument)
            ],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        let foreign_site = site(vec![SourcePathSegmentV1::Body(1)]);
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                foreign_site.owner(),
                0,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                new_site.owner(),
                1,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
    }
}

pub(in crate::mir::builder) fn emit_root_home_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    value: ValueId,
) -> Result<ValueId, String> {
    emit_root_home_exit_payload(builder, state, ledger, Some(value), value)
}

pub(in crate::mir::builder) fn emit_root_home_unit_exit(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
) -> Result<ValueId, String> {
    let statement_result = crate::mir::builder::emission::constant::emit_void(builder)?;
    emit_root_home_exit_payload(builder, state, ledger, None, statement_result)
}

fn emit_root_home_exit_payload(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    return_value: Option<ValueId>,
    statement_result: ValueId,
) -> Result<ValueId, String> {
    let operations = ledger.begin_root_home_exit()?;
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings = Vec::new();
    let mut clean = builder.next_block_id();
    let mut fault = builder.next_block_id();
    append_block(
        builder,
        clean,
        MirInstruction::Return { value: return_value },
        &mut bindings,
    )?;
    append_block(
        builder,
        fault,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let count = operations.len();
    let mut origins = Vec::with_capacity(count);
    for (index, origin) in operations.into_iter().rev().enumerate() {
        let operation = InvokeOperation::HomeRelease {
            object: origin.object(),
            value: origin.value(),
        };
        // A clean call's Fault skips its own retry and joins the remaining
        // fault-pending suffix. Later Normal outcomes cannot clear that Fault.
        let next_clean = cleanup_step(
            builder,
            frame,
            operation.clone(),
            clean,
            fault,
            &mut bindings,
        )?;
        let (block, instruction) = bindings
            .last()
            .cloned()
            .ok_or_else(|| freeze("root-home-release-binding-missing"))?;
        origins.push((origin, block, instruction));
        if index + 1 < count {
            fault = cleanup_step(builder, frame, operation, fault, fault, &mut bindings)?;
        }
        clean = next_clean;
    }
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let jump = MirInstruction::Jump {
        target: clean,
        edge_args: None,
    };
    builder.emit_instruction(jump.clone())?;
    bindings.push((origin, jump));
    origins.reverse();
    ledger.record_root_home_exit(origins, bindings)?;
    Ok(statement_result)
}

pub(in crate::mir::builder) fn emit_terminal_i64_add_return(
    builder: &mut MirBuilder,
    ledger: &OrdinaryNewClaimLedgerV1,
    prepared: PreparedTerminalI64AddReturnV1,
) -> Result<ValueId, String> {
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
    ledger.record_terminal_i64_add(block, result, values[0], values[1])?;
    ledger.complete_terminal_i64_add_return(result)?;
    Ok(result)
}

fn cleanup_chain(
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
    let origin = builder.next_block_id();
    let normal = builder.next_block_id();
    let fault = builder.next_block_id();
    for (landing, target) in [(normal, normal_next), (fault, fault_next)] {
        append_block(
            builder,
            landing,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            bindings,
        )?;
    }
    append_block(
        builder,
        origin,
        MirInstruction::Invoke {
            operation,
            fault_frame: frame,
            normal_landing: normal,
            fault_landing: fault,
        },
        bindings,
    )?;
    Ok(origin)
}

fn append_block(
    builder: &mut MirBuilder,
    id: BasicBlockId,
    terminator: MirInstruction,
    bindings: &mut Vec<(BasicBlockId, MirInstruction)>,
) -> Result<(), String> {
    let function = builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| freeze("no-function"))?;
    if function.blocks.contains_key(&id) {
        return Err(freeze("duplicate-block"));
    }
    let mut block = BasicBlock::new(id);
    block.set_terminator(terminator.clone());
    function.add_block(block);
    bindings.push((id, terminator));
    Ok(())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/emission/{reason}]")
}
