//! Canonical whole-function close for the selected unpublished Static Loop.
//! A successful Ready DraftSeal is discarded until ABI and packet proofs exist.

use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    finish_profile_close, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::draft_seal::ReadyFunctionDraftSealV1;
use crate::mir::builder::resolved_lowering::static_loop_body::StaticLoopBodyContinuationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, MirInstruction};

pub(super) fn finish_unpublished_static_loop_v1(
    draft: &mut MirBuilder,
    mut canonical: CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    terminal: BasicBlockId,
    body: &StaticLoopBodyContinuationV1,
) -> Result<ReadyFunctionDraftSealV1, String> {
    let reject = || "[freeze:contract][callable-loop/static-finish-shape-drift]".to_owned();
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(reject)?;
    body.corroborate_unpublished_function(function)?;
    let ids = function.block_ids();
    let tail = semantic.tail_call();
    let landing = function.blocks.get(&terminal).ok_or_else(reject)?;
    if ids.len() != 14
        || draft.function_state.current_block != Some(terminal)
        || tail.loop_site() != &semantic.roles().loop_site
        || !matches!(
            landing.instructions.first(),
            Some(MirInstruction::InvokeNormalResult { .. })
        )
        || landing.predecessors.len() != 1
    {
        return Err(reject());
    }
    for block in ids {
        if draft
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.blocks.get(&block))
            .ok_or_else(reject)?
            .is_sealed()
        {
            continue;
        }
        let witness = {
            let function = draft
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(reject)?;
            canonical
                .cfg
                .seal_block(function, block)
                .map_err(|error| error.to_string())?
        };
        canonical
            .identity
            .seal_block(draft, &mut canonical.phis, block, &witness)?;
    }
    let close = finish_profile_close(canonical.owner(), terminal, || {
        if semantic.recipe().as_recipe().carriers.len() != 1
            || semantic.source_calls().2.argument_sites()
                != [semantic.roles().body_actual_site.clone()]
        {
            return Err(reject());
        }
        Ok(())
    })?;
    canonical
        .finish_for_draft_seal(draft, close)
        .map_err(|error| error.to_string())
}
