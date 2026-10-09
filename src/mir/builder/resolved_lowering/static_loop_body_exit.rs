//! Selected Static I64 Loop's Return and carrier backedge in one unpublished draft.

use crate::mir::builder::emission::loop_operation;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::static_loop_body::{
    read_i64, StaticLoopBodyContinuationV1,
};
use crate::mir::builder::resolved_lowering::static_loop_header::StaticLoopHeaderContinuationV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV2, LoopExitKindV2, LoopOperationV2, LoopRecipeItemV2,
};
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::MirType;

pub(super) fn emit_unpublished_body_exit_v1(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    header: &StaticLoopHeaderContinuationV1,
    body_blocks: &StaticLoopBodyContinuationV1,
) -> Result<(), String> {
    let reject = || "[freeze:contract][callable-loop/static-body-exit-source-drift]".to_owned();
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
    if body.items.get(4..)
        != Some(&[
            roles.backedge_bin_read,
            roles.backedge_const,
            roles.backedge_add,
            roles.backedge_write,
        ])
        || recipe.carriers.len() != 1
        || body_blocks.then_block == body_blocks.step_block
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
    let return_value = match (item(roles.return_bin_read)?, item(roles.return_exit)?) {
        (
            LoopRecipeItemV2::Operation { operation: LoopOperationV2::ReadBinding { binding, result } },
            LoopRecipeItemV2::Exit { exit },
        ) if *binding == recipe.carriers[0].binding
            && recipe.exits.iter().any(|row| row.key == *exit && row.owner_loop == recipe.root_loop
                && matches!(row.kind, LoopExitKindV2::Return { value: Some(value) } if value == *result)) => *result,
        _ => return Err(reject()),
    };
    let then_block = recipe
        .blocks
        .iter()
        .find(|row| row.items == [roles.return_bin_read, roles.return_exit])
        .ok_or_else(reject)?;
    if then_block.owner_loop != recipe.root_loop {
        return Err(reject());
    }
    let (step_value, constant_value) = match (
        item(roles.backedge_bin_read)?,
        item(roles.backedge_const)?,
        item(roles.backedge_add)?,
        item(roles.backedge_write)?,
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
                    LoopOperationV2::ConstI64 {
                        result: one,
                        value: 1,
                    },
            },
            LoopRecipeItemV2::Operation {
                operation:
                    LoopOperationV2::BinaryI64 {
                        op: LoopBinaryI64OpV2::Add,
                        left,
                        right,
                        result: sum,
                    },
            },
            LoopRecipeItemV2::Operation {
                operation:
                    LoopOperationV2::WriteBinding {
                        binding: written,
                        value,
                    },
            },
        ) if *binding == recipe.carriers[0].binding
            && *written == *binding
            && *left == *read
            && *right == *one
            && *value == *sum =>
        {
            (*read, *one)
        }
        _ => return Err(reject()),
    };
    if return_value == step_value || step_value == constant_value {
        return Err(reject());
    }
    canonical
        .cfg
        .select_block(draft, body_blocks.then_block)
        .map_err(|error| error.to_string())?;
    let returned = read_i64(
        draft,
        canonical,
        body_blocks.then_block,
        header.preheader,
        &roles.return_value_site,
        roles.bin_binding,
    )?;
    canonical.completion.claim_explicit_return(
        &roles.return_site,
        canonical.target_function(),
        body_blocks.then_block,
        returned,
    )?;
    canonical
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(roles.return_site.clone()))?;

    canonical
        .cfg
        .select_block(draft, body_blocks.step_block)
        .map_err(|error| error.to_string())?;
    let old_bin = read_i64(
        draft,
        canonical,
        body_blocks.step_block,
        header.preheader,
        &roles.step_left_site,
        roles.bin_binding,
    )?;
    let one = loop_operation::emit_const_i64(draft, 1)?;
    let next = canonical.issue_physical_value_id(draft)?;
    loop_operation::emit_add_i64_at_with_dst(draft, body_blocks.step_block, next, old_bin, one)?;
    canonical.publish_physical_value_type(draft, next, MirType::Integer)?;
    canonical.identity.define_assignment_exact(
        &roles.assignment_target,
        roles.bin_binding,
        body_blocks.step_block,
        next,
    )?;
    let witness = {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-body-exit-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .emit_jump(function, body_blocks.step_block, header.header)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .seal_block(function, header.header)
            .map_err(|error| error.to_string())?
    };
    canonical
        .identity
        .seal_block(draft, &mut canonical.phis, header.header, &witness)?;
    Ok(())
}
