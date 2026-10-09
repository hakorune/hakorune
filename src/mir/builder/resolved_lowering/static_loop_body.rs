//! Unpublished predicate for the selected source-bound Static I64 Loop body.
//! The source/Recipe select one call; canonical SSA/CFG own its physical draft.

use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::static_loop_header::StaticLoopHeaderContinuationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::definitions::MirCall;
use crate::mir::loop_recipe_contract::{
    LoopCompareI64OpV2, LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2,
    LoopJoinEdgeRoleV1, LoopOperationV2, LoopRecipeItemV2,
};
use crate::mir::resolved_semantics::{
    ResolvedLoopPlacementV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use crate::mir::{BasicBlockId, CompareOp, MirInstruction, MirType, ValueId};

pub(super) fn emit_unpublished_body_predicate_v1(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    header: &StaticLoopHeaderContinuationV1,
    frame_owner: &mut FunctionFaultFrameV1,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-body-source-drift]".to_owned();
    let recipe = semantic.recipe().as_recipe();
    let roles = semantic.roles();
    let root = recipe
        .loops
        .iter()
        .find(|row| row.key == recipe.root_loop)
        .ok_or_else(reject)?;
    let body = recipe
        .blocks
        .iter()
        .find(|row| row.key == root.body)
        .ok_or_else(reject)?;
    if body.items.get(..4)
        != Some(&[
            roles.body_actual_read,
            roles.body_call,
            roles.body_compare,
            roles.body_if,
        ])
        || recipe.carriers.len() != 1
        || body.owner_loop != recipe.root_loop
    {
        return Err(reject());
    }
    let item = |key| {
        recipe
            .items
            .iter()
            .find(|row| row.key == key)
            .map(|row| &row.item)
            .ok_or_else(reject)
    };
    let (read_value, call_value, then_key, condition_key) = match (
        item(roles.body_actual_read)?,
        item(roles.body_call)?,
        item(roles.body_compare)?,
        item(roles.body_if)?,
    ) {
        (
            LoopRecipeItemV2::Operation {
                operation:
                    LoopOperationV2::ReadBinding {
                        binding,
                        result: read,
                    },
            },
            LoopRecipeItemV2::Operation {
                operation:
                    LoopOperationV2::CallSlot {
                        receiver: None,
                        args,
                        result: Some(call),
                    },
            },
            LoopRecipeItemV2::Operation {
                operation:
                    LoopOperationV2::CompareI64 {
                        op: LoopCompareI64OpV2::LessEqual,
                        left,
                        right,
                        result,
                    },
            },
            LoopRecipeItemV2::If {
                condition,
                then_block,
                else_block: None,
            },
        ) if *binding == recipe.carriers[0].binding
            && args.as_slice() == [*read]
            && *left == roles.n_input
            && *right == *call
            && *result == *condition
            && recipe.blocks.iter().any(|row| {
                row.key == *then_block && row.items == [roles.return_bin_read, roles.return_exit]
            }) =>
        {
            (*read, *call, *then_block, *condition)
        }
        _ => return Err(reject()),
    };
    let transfer = semantic
        .join()
        .logical_transfer_view()
        .map_err(|_| reject())?;
    let [branch] = transfer.branches() else {
        return Err(reject());
    };
    match (branch.then_arm, branch.else_arm) {
        (
            LoopJoinBranchArmTransferRefV2::Exit(exit),
            LoopJoinBranchArmTransferRefV2::Fallthrough { continuation, .. },
        ) if branch.owner_loop == recipe.root_loop
            && branch.if_item == roles.body_if
            && branch.condition == condition_key
            && exit.exit_item == roles.return_exit
            && exit.role == LoopJoinEdgeRoleV1::Return
            && exit.target == LoopJoinBranchExitTargetV2::FunctionExit
            && continuation.block == body.key
            && continuation.item == roles.backedge_bin_read
            && then_key != body.key => {}
        _ => return Err(reject()),
    }
    if read_value == call_value || header.body == header.header || header.body == header.after {
        return Err(reject());
    }
    let source = semantic.source_calls().2;
    let original = source.original();
    if source.placement() != &ResolvedLoopPlacementV1::Body
        || !source.claim().corroborates_source(
            source.call_site(),
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            1,
        )
        || source.claim().current_owner_source_required_i64_arguments() != Some(&[0][..])
        || source.argument_sites() != [roles.body_actual_site.clone()]
        || source.call_site().owner() != canonical.owner()
        || *source.call_site().site() != roles.body_call_site
        || original.target().arity() != 1
    {
        return Err(reject());
    }
    canonical
        .cfg
        .select_block(draft, header.body)
        .map_err(|error| error.to_string())?;
    let n = read_i64(
        draft,
        canonical,
        header.body,
        header.preheader,
        &roles.body_n_read_site,
        roles.n_binding,
    )?;
    let actual = read_i64(
        draft,
        canonical,
        header.body,
        header.preheader,
        &roles.body_actual_site,
        roles.bin_binding,
    )?;
    let normal = canonical.create_unpublished_block(draft)?;
    let fault = canonical.create_unpublished_block(draft)?;
    let then_block = canonical.create_unpublished_block(draft)?;
    let step_block = canonical.create_unpublished_block(draft)?;
    let target = original
        .target()
        .canonical_global_target_v1()
        .map_err(|_| "[freeze:contract][callable-loop/static-body-target-missing]".to_owned())?;
    let frame = frame_owner.materialize(draft)?;
    let call = MirCall::global(None, target, vec![actual]);
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-body-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .emit_i64_invoke(function, header.body, call, frame, normal, fault)
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
    let result = canonical.issue_physical_value_id(draft)?;
    draft.emit_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: header.body,
        dst: result,
    })?;
    canonical.publish_physical_value_type(draft, result, MirType::Integer)?;
    let predicate = canonical.issue_physical_value_id(draft)?;
    loop_operation::emit_compare_i64_at_with_dst(
        draft,
        normal,
        predicate,
        CompareOp::Le,
        n,
        result,
    )?;
    canonical.publish_physical_value_type(draft, predicate, MirType::Bool)?;
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-body-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .prepare_branch(function, normal, predicate, then_block, step_block)
            .map_err(|error| error.to_string())?
            .commit(function);
        frame_owner.validate(function)?;
        if !matches!(
            function.blocks.get(&normal).and_then(|block| block.terminator.as_ref()),
            Some(MirInstruction::Branch { then_bb, else_bb, .. }) if *then_bb == then_block && *else_bb == step_block
        ) {
            return Err("[freeze:contract][callable-loop/static-body-branch-drift]".into());
        }
    }
    Ok(())
}

fn read_i64(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    block: BasicBlockId,
    deferred_entry: BasicBlockId,
    site: &SourceExprSiteV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
) -> Result<ValueId, String> {
    canonical
        .identity
        .claim_variable_use_binding(site, binding)?;
    let read = canonical.identity.read_entry_receipt_with_deferred_entry(
        draft,
        &mut canonical.phis,
        block,
        binding,
        deferred_entry,
    )?;
    if read.owner() != canonical.owner()
        || read.binding() != binding
        || read.physical_block() != block
    {
        return Err("[freeze:contract][callable-loop/static-body-read-drift]".into());
    }
    canonical.publish_physical_value_type(draft, read.physical_value(), MirType::Integer)?;
    Ok(read.physical_value())
}
