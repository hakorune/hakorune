//! Original post-loop `return me.huge_bin()` in the selected unpublished draft.

use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::static_loop_header::StaticLoopHeaderContinuationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::definitions::MirCall;
use crate::mir::loop_recipe_contract::LoopValueClassV2;
use crate::mir::resolved_semantics::{ResolvedExitSiteV1, ResolvedMethodCallReceiverSourceV1};
use crate::mir::{MirInstruction, MirType};

pub(super) fn emit_unpublished_tail_v1(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    header: &StaticLoopHeaderContinuationV1,
    frame_owner: &mut FunctionFaultFrameV1,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-tail-source-drift]".to_owned();
    let recipe = semantic.recipe().as_recipe();
    let transfer = semantic
        .join()
        .logical_transfer_view()
        .map_err(|_| reject())?;
    let after = transfer.after();
    let [carrier] = recipe.carriers.as_slice() else {
        return Err(reject());
    };
    if after.loop_key() != recipe.root_loop
        || after.binding() != carrier.binding
        || after.class() != LoopValueClassV2::I64
        || header.after == header.body
    {
        return Err(reject());
    }
    let tail = semantic.tail_call();
    let original = tail.original();
    if tail.loop_site() != &semantic.roles().loop_site
        || tail.call_site().owner() != canonical.owner()
        || !tail.claim().is_current_owner_zeroarg()
        || !tail.claim().corroborates_source(
            tail.call_site(),
            ResolvedMethodCallReceiverSourceV1::CurrentOwner,
            0,
        )
        || original.target().arity() != 0
    {
        return Err(reject());
    }
    canonical
        .cfg
        .select_block(draft, header.after)
        .map_err(|error| error.to_string())?;
    let normal = canonical.create_unpublished_block(draft)?;
    let fault = canonical.create_unpublished_block(draft)?;
    let target = original
        .target()
        .canonical_global_target_v1()
        .map_err(|_| "[freeze:contract][callable-loop/static-tail-target-missing]".to_owned())?;
    let frame = frame_owner.materialize(draft)?;
    let call = MirCall::global(None, target, vec![]);
    let after_witness = {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-tail-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .emit_i64_invoke(function, header.after, call, frame, normal, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_invoke_fault(function, fault, frame)
            .map_err(|error| error.to_string())?;
        let witness = canonical
            .cfg
            .seal_block(function, header.after)
            .map_err(|error| error.to_string())?;
        if witness.predecessors() != [header.normal]
            || !matches!(
                function.blocks.get(&header.normal).and_then(|block| block.terminator.as_ref()),
                Some(MirInstruction::Branch { then_bb, else_bb, .. })
                    if *then_bb == header.body && *else_bb == header.after
            )
        {
            return Err("[freeze:contract][callable-loop/static-tail-after-edge-drift]".into());
        }
        witness
    };
    canonical
        .identity
        .seal_block(draft, &mut canonical.phis, header.after, &after_witness)?;
    canonical
        .cfg
        .select_block(draft, normal)
        .map_err(|error| error.to_string())?;
    let value = canonical.issue_physical_value_id(draft)?;
    draft.emit_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: header.after,
        dst: value,
    })?;
    canonical.publish_physical_value_type(draft, value, MirType::Integer)?;
    canonical.completion.claim_explicit_return(
        tail.return_site(),
        canonical.target_function(),
        normal,
        value,
    )?;
    canonical
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(tail.return_site().clone()))?;
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-tail-function-missing]".to_owned()
        })?;
    frame_owner.validate(function)?;
    Ok(())
}
