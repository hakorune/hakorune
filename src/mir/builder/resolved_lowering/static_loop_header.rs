//! Unpublished header of the selected source-bound Static I64 Loop.
//! The verified V2 Recipe supplies operation order; canonical SSA/CFG own MIR.

use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::MirBuilder;
use crate::mir::definitions::MirCall;
use crate::mir::loop_recipe_contract::{
    LoopCompareI64OpV2, LoopConditionV2, LoopOperationV2, LoopRecipeItemV2,
};
use crate::mir::normal_callable_semantic_package::PreparedSelectedStaticLoopCallProjectionV1;
use crate::mir::resolved_semantics::ResolvedMethodCallReceiverSourceV1;
use crate::mir::{CompareOp, MirInstruction, MirType};

pub(super) fn emit_unpublished_header_v1(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    entry_packet: &PreparedSelectedStaticLoopCallProjectionV1,
    frame_owner: &mut FunctionFaultFrameV1,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-header-source-drift]".to_owned();
    let roles = semantic.roles();
    let recipe = semantic.recipe().as_recipe();
    let root = recipe
        .loops
        .iter()
        .find(|row| row.key == recipe.root_loop)
        .ok_or_else(reject)?;
    let LoopConditionV2::Predicate {
        block: condition_block,
        value: condition_value,
    } = root.condition
    else {
        return Err(reject());
    };
    let block = recipe
        .blocks
        .iter()
        .find(|row| row.key == condition_block)
        .ok_or_else(reject)?;
    if block.items
        != [
            roles.header_bin_read,
            roles.header_call,
            roles.header_compare,
        ]
        || recipe.carriers.len() != 1
        || recipe.carriers[0].owner_loop != recipe.root_loop
        || entry_packet.source_site() != semantic.source_calls().0.call_site()
    {
        return Err(reject());
    }
    let operation = |key| {
        recipe
            .items
            .iter()
            .find(|row| row.key == key)
            .and_then(|row| match &row.item {
                LoopRecipeItemV2::Operation { operation } => Some(operation),
                _ => None,
            })
            .ok_or_else(reject)
    };
    match (
        operation(roles.header_bin_read)?,
        operation(roles.header_call)?,
        operation(roles.header_compare)?,
    ) {
        (
            LoopOperationV2::ReadBinding {
                binding,
                result: read,
            },
            LoopOperationV2::CallSlot {
                receiver: None,
                args,
                result: Some(call),
            },
            LoopOperationV2::CompareI64 {
                op: LoopCompareI64OpV2::LessEqual,
                left,
                right,
                result,
            },
        ) if *binding == recipe.carriers[0].binding
            && args.is_empty()
            && *left == *read
            && *right == *call
            && *result == condition_value => {}
        _ => return Err(reject()),
    }
    let header_source = semantic.source_calls().1;
    let original = header_source.original();
    if header_source.placement()
        != &crate::mir::resolved_semantics::ResolvedLoopPlacementV1::Condition
        || !header_source.claim().is_current_owner_zeroarg()
        || !header_source.claim().corroborates_source(
            header_source.call_site(),
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            0,
        )
        || !header_source.argument_sites().is_empty()
        || original.target().arity() != 0
        || header_source.call_site().owner() != canonical.owner()
        || roles.header_call_site != *header_source.call_site().site()
    {
        return Err(reject());
    }
    let preheader = draft.function_state.current_block.ok_or_else(|| {
        "[freeze:contract][callable-loop/static-header-preheader-missing]".to_owned()
    })?;
    let header = canonical.create_unpublished_block(draft)?;
    let body = canonical.create_unpublished_block(draft)?;
    let after = canonical.create_unpublished_block(draft)?;
    let normal = canonical.create_unpublished_block(draft)?;
    let fault = canonical.create_unpublished_block(draft)?;
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-header-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .emit_jump(function, preheader, header)
            .map_err(|error| error.to_string())?;
    }
    canonical
        .cfg
        .select_block(draft, header)
        .map_err(|error| error.to_string())?;
    canonical
        .identity
        .claim_variable_use_binding(&roles.header_bin_read_site, roles.bin_binding)?;
    let read = canonical.identity.read_entry_receipt_with_deferred_entry(
        draft,
        &mut canonical.phis,
        header,
        roles.bin_binding,
        preheader,
    )?;
    if read.owner() != canonical.owner()
        || read.binding() != roles.bin_binding
        || read.physical_block() != header
    {
        return Err("[freeze:contract][callable-loop/static-header-read-drift]".into());
    }
    match draft
        .function_state
        .type_ctx
        .get_type(read.physical_value())
    {
        None | Some(MirType::Unknown) => {
            canonical.publish_physical_value_type(
                draft,
                read.physical_value(),
                MirType::Integer,
            )?;
        }
        Some(MirType::Integer) => {}
        _ => return Err("[freeze:contract][callable-loop/static-header-read-type]".into()),
    }
    let target = original
        .target()
        .canonical_global_target_v1()
        .map_err(|_| "[freeze:contract][callable-loop/static-header-target-missing]".to_owned())?;
    let call = MirCall::global(None, target, vec![]);
    let frame = frame_owner.materialize(draft)?;
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-header-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .emit_i64_invoke(function, header, call, frame, normal, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_invoke_fault(function, fault, frame)
            .map_err(|error| error.to_string())?;
    }
    canonical
        .cfg
        .select_block(draft, normal)
        .map_err(|error| error.to_string())?;
    let call_result = canonical.issue_physical_value_id(draft)?;
    draft.emit_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: header,
        dst: call_result,
    })?;
    canonical.publish_physical_value_type(draft, call_result, MirType::Integer)?;
    let condition = canonical.issue_physical_value_id(draft)?;
    loop_operation::emit_compare_i64_at_with_dst(
        draft,
        normal,
        condition,
        CompareOp::Le,
        read.physical_value(),
        call_result,
    )?;
    canonical.publish_physical_value_type(draft, condition, MirType::Bool)?;
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-header-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .prepare_branch(function, normal, condition, body, after)
            .map_err(|error| error.to_string())?
            .commit(function);
        frame_owner.validate(function)?;
    }
    Ok(())
}
