mod array_write;
mod basic;
mod calls;
mod control_flow;
mod fastmem;
mod fields;
mod phi;
mod sum;
mod weak;

use crate::mir::function::ExactNumericRuntimeCheckContractKind;
use crate::mir::MirInstruction as I;

pub(crate) fn emit_phi_instructions(
    func: &crate::mir::MirFunction,
    block: &crate::mir::BasicBlock,
) -> Vec<serde_json::Value> {
    let mut insts = Vec::new();
    for inst in &block.instructions {
        if let I::Phi { .. } = inst {
            if let Some(value) = phi::emit_phi(inst, &func.metadata.value_types) {
                insts.push(value);
            }
        }
    }
    insts
}

pub(crate) fn emit_non_phi_instructions(
    func: &crate::mir::MirFunction,
    block: &crate::mir::BasicBlock,
    boxed_sum_abi_plans: &[crate::mir::boxed_sum_abi_plan::BoxedSumAbiPlanV1],
    boxed_sum_site_plans: &std::collections::BTreeMap<
        (crate::mir::BasicBlockId, usize),
        crate::mir::boxed_sum_abi_plan::BoxedSumSitePlan,
    >,
    insts: &mut Vec<serde_json::Value>,
) -> Result<(), String> {
    for (instruction_index, inst) in block.instructions.iter().enumerate() {
        if let I::Phi { .. } = inst {
            continue;
        }
        let value = emit_instruction(
            func,
            block.id,
            instruction_index,
            inst,
            boxed_sum_abi_plans,
            boxed_sum_site_plans,
        )?;
        insts.push(value);
    }
    Ok(())
}

pub(crate) fn emit_terminator(
    terminator: &Option<crate::mir::MirInstruction>,
) -> Result<Option<serde_json::Value>, String> {
    match terminator.as_ref() {
        Some(term) => control_flow::emit_terminator(term).map(Some),
        None => Ok(None),
    }
}

fn emit_instruction(
    func: &crate::mir::MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    inst: &crate::mir::MirInstruction,
    boxed_sum_abi_plans: &[crate::mir::boxed_sum_abi_plan::BoxedSumAbiPlanV1],
    boxed_sum_site_plans: &std::collections::BTreeMap<
        (crate::mir::BasicBlockId, usize),
        crate::mir::boxed_sum_abi_plan::BoxedSumSitePlan,
    >,
) -> Result<serde_json::Value, String> {
    if let Some(code) = crate::mir::contracts::backend_core_ops::legacy_callsite_reject_code(inst) {
        return Err(format!(
            "[freeze:contract][mir-json/callsite:{}] inst={}",
            code,
            crate::mir::contracts::backend_core_ops::instruction_tag(inst)
        ));
    }

    if !crate::mir::contracts::backend_core_ops::is_supported_mir_json_instruction(inst) {
        return Err(format!(
            "MIR JSON emit contract violation: unsupported instruction {}",
            crate::mir::contracts::backend_core_ops::instruction_tag(inst)
        ));
    }

    match inst {
        I::ArrayStateContractClaim { contract_id, array } => Ok(serde_json::json!({
            "op": "array_state_contract_claim",
            "contract_id": contract_id,
            "array": array.as_u32(),
        })),
        I::Copy { dst, src } => Ok(basic::emit_copy(dst, src)),
        I::LocalContractWrite {
            dst,
            src,
            local_slot_id,
            write_kind,
        } => Ok(serde_json::json!({
            "op": "local_contract_write",
            "dst": dst.as_u32(),
            "src": src.as_u32(),
            "local_slot_id": local_slot_id.binding_id().raw(),
            "write_kind": match write_kind {
                crate::mir::function::LocalContractWriteKind::Init => "init",
                crate::mir::function::LocalContractWriteKind::Reassign => "reassign",
            },
        })),
        I::RecordFieldContractCheck {
            contract_id,
            schema_fingerprint,
            field_index,
            value,
        } => Ok(serde_json::json!({
            "op": "record_field_contract_check",
            "contract_id": contract_id,
            "schema_fingerprint": schema_fingerprint,
            "field_index": field_index,
            "value": value.as_u32(),
        })),
        I::RecordValuePublish {
            dst,
            contract_id,
            boundary,
            diagnostic_record_name,
            schema_fingerprint,
            base,
            fields,
        } => Ok(serde_json::json!({
            "op": "record_value_publish",
            "dst": dst.as_u32(),
            "contract_id": contract_id,
            "boundary": match boundary {
                crate::mir::function::RecordValueBoundaryKind::Construct => "construct",
                crate::mir::function::RecordValueBoundaryKind::WithUpdate => "with_update",
            },
            "diagnostic_record_name": diagnostic_record_name,
            "schema_fingerprint": schema_fingerprint,
            "base": base.map(|value| value.as_u32()),
            "fields": fields.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
        })),
        I::UnaryOp { dst, op, operand } => Ok(basic::emit_unary_op(dst, op, operand)),
        I::Const { dst, value } => Ok(basic::emit_const(dst, value)),
        I::StaticDataLoad {
            dst,
            source_name,
            symbol,
            element,
            len,
            align,
            index,
        } => Ok(basic::emit_static_data_load(
            dst,
            source_name,
            symbol,
            element,
            *len,
            *align,
            index,
        )),
        I::TypeOp { dst, op, value, ty } => Ok(basic::emit_type_op(dst, op, value, ty)),
        I::BinOp { dst, op, lhs, rhs } => Ok(basic::emit_bin_op(
            dst,
            op,
            lhs,
            rhs,
            &func.metadata.value_types,
        )),
        I::Compare { dst, op, lhs, rhs } => Ok(basic::emit_compare(
            dst,
            op,
            lhs,
            rhs,
            &func.metadata.value_types,
        )),
        I::Debug { value, message } => Ok(basic::emit_debug(value, message)),
        I::Safepoint => Ok(basic::emit_safepoint()),
        I::FutureNew { dst, value } => Ok(basic::emit_future_new(dst, value)),
        I::FutureSet { future, value } => Ok(basic::emit_future_set(future, value)),
        I::Await { dst, future } => Ok(basic::emit_await(dst, future)),
        I::FieldGet {
            dst,
            base,
            field,
            declared_type,
        } => Ok(fields::emit_field_get(dst, base, field, declared_type)),
        I::FieldSet {
            base,
            field,
            value,
            declared_type,
        } => Ok(fields::emit_field_set(
            base,
            field,
            value,
            declared_type,
            exact_numeric_runtime_check_for_field_set(func, block, instruction_index, field, value),
        )),
        I::WeakFieldWrite {
            site_id,
            contract_id,
            base,
            field_index,
            value,
        } => Ok(fields::emit_weak_field_write(
            site_id,
            contract_id,
            base,
            *field_index,
            value,
        )),
        I::VariantMake {
            dst,
            enum_name,
            variant,
            tag,
            payload,
            payload_type,
        } => Ok(sum::emit_variant_make(
            dst,
            enum_name,
            variant,
            *tag,
            payload.as_ref(),
            payload_type.as_ref(),
            boxed_sum_site_plans.get(&(block, instruction_index)),
        )),
        I::VariantTag {
            dst,
            value,
            enum_name,
        } => Ok(sum::emit_variant_tag(
            dst,
            value,
            enum_name,
            boxed_sum_site_plans.get(&(block, instruction_index)),
        )),
        I::VariantProject {
            dst,
            value,
            enum_name,
            variant,
            tag,
            payload_type,
        } => Ok(sum::emit_variant_project(
            boxed_sum_abi_plans,
            dst,
            value,
            enum_name,
            variant,
            *tag,
            payload_type.as_ref(),
        )),
        I::Select {
            dst,
            cond,
            then_val,
            else_val,
            ..
        } => Ok(basic::emit_select(dst, cond, then_val, else_val)),
        I::MemOp {
            region,
            kind,
            dst,
            operands,
            access,
            effects,
        } => Ok(fastmem::emit_memop(
            region, kind, dst, operands, access, effects,
        )),
        I::PinnedTextOp { dst, plan, kind } => Ok(basic::emit_pinned_text_op(dst, plan, *kind)),
        I::PinnedTextResidenceEnter {
            plan,
            normal_landing,
            trap_landing,
        } => Ok(basic::emit_pinned_text_residence_enter(
            plan,
            normal_landing,
            trap_landing,
        )),
        I::PinnedTextResidenceFinish { residence } => {
            Ok(basic::emit_pinned_text_residence_finish(residence))
        }
        I::ArrayElementWrite {
            site_id,
            dst,
            kind,
            producer,
            receiver,
            index,
            value,
        } => Ok(array_write::emit(
            *site_id, *dst, *kind, *producer, *receiver, *index, *value,
        )),
        I::Call {
            dst,
            func,
            callee,
            args,
            effects,
            ..
        } => calls::emit_call(dst, func, callee.as_ref(), args, effects)
            .ok_or_else(|| "MIR JSON emit contract violation: failed to emit Call".to_string()),
        I::CheckedCallOutNormalResult { site_id, dst } => Ok(
            control_flow::emit_checked_callout_normal_result(site_id, dst),
        ),
        I::CheckedCallOutEnd {
            site_id,
            lease_slot,
        } => Ok(control_flow::emit_checked_callout_end(site_id, lease_slot)),
        I::NewBox {
            dst,
            box_type,
            args,
        } => Ok(calls::emit_new_box(dst, box_type, args)),
        I::NewClosure {
            dst,
            params,
            captures,
            me,
            ..
        } => Ok(calls::emit_new_closure(dst, params, captures, me)),
        I::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        } => Ok(control_flow::emit_branch(condition, then_bb, else_bb)),
        I::Jump { target, .. } => Ok(control_flow::emit_jump(target)),
        I::Return { value } => Ok(control_flow::emit_return(value.as_ref())),
        I::WeakRef { dst, op, value } => Ok(weak::emit_weak_ref(dst, op, value)),
        I::KeepAlive { values } => Ok(weak::emit_keep_alive(values)),
        I::CopyOwned { dst, src } => Ok(weak::emit_copy_owned(*dst, *src)),
        I::DestroyOwned { value } => Ok(weak::emit_destroy_owned(*value)),
        I::ReleaseStrong { values } => Ok(weak::emit_release_strong(values)),
        _ => unreachable!("pre-checked by backend_core_ops allowlist"),
    }
}

fn exact_numeric_runtime_check_for_field_set<'a>(
    func: &'a crate::mir::MirFunction,
    block: crate::mir::BasicBlockId,
    instruction_index: usize,
    field: &str,
    value: &crate::mir::ValueId,
) -> Option<&'a str> {
    func.metadata
        .exact_numeric_runtime_check_contracts
        .iter()
        .find(|contract| {
            contract.kind == ExactNumericRuntimeCheckContractKind::DynamicIntegerRange
                && contract.block == block
                && contract.instruction_index == instruction_index
                && contract.field == field
                && contract.value == *value
        })
        .map(|contract| contract.declared_type_name.as_str())
}
