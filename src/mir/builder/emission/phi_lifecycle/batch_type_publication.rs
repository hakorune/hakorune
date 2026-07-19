use std::collections::BTreeSet;

use super::PhiBatchItem;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlockId, ValueId};

pub(super) fn define_phi_batch_prepend(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    items: Vec<PhiBatchItem>,
    tag: &str,
) -> Result<(), String> {
    preflight(builder, block, &items, tag)?;

    let mut type_rows = items
        .iter()
        .map(|item| {
            crate::mir::builder::phi_type_publication::prepare_for_builder(
                builder,
                item.dst,
                &item.inputs,
                item.type_hint.as_ref(),
            )
            .map(|prepared| (item.dst, prepared))
        })
        .collect::<Result<Vec<_>, _>>()?;
    type_rows.sort_by_key(|(dst, _)| *dst);

    let mut candidate = builder
        .function_state
        .current_function
        .as_ref()
        .expect("batch preflight sealed current function")
        .clone();
    let mut physical_rows = Vec::with_capacity(items.len());
    for mut item in items {
        for (pred, incoming) in &mut item.inputs {
            *incoming = crate::mir::builder::ssa::phi_input_materializer::for_pred(
                &mut candidate,
                *pred,
                *incoming,
                &item.item_tag,
                "phi_batch",
            )?;
        }
        item.inputs.sort_by_key(|(bb, _)| bb.0);
        physical_rows.push((item.dst, item.inputs, item.type_hint, item.span));
    }

    crate::mir::ssot::cf_common::insert_phi_batch_prepend_spanned_with_type_hint(
        &mut candidate,
        block,
        physical_rows,
    )
    .map_err(|error| format!("{error} op=define_phi_batch_prepend tag={tag}"))?;

    if crate::config::env::joinir_dev::debug_enabled() {
        let caller = std::panic::Location::caller();
        for (dst, _) in &type_rows {
            builder.record_value_origin_caller(*dst, caller);
            if let Some(location) = builder.value_origin_caller(*dst) {
                candidate
                    .metadata
                    .value_origin_callers
                    .insert(*dst, location.to_string());
            }
        }
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&format!(
            "[phi_lifecycle/batch_prepend] fn={} bb={:?} count={} tag={}",
            candidate.signature.name,
            block,
            type_rows.len(),
            tag
        ));
    }

    *builder
        .function_state
        .current_function
        .as_mut()
        .expect("batch preflight sealed current function") = candidate;
    for (dst, prepared) in type_rows {
        crate::mir::builder::phi_type_publication::commit_for_builder(builder, dst, prepared);
    }
    Ok(())
}

fn preflight(
    builder: &MirBuilder,
    block: BasicBlockId,
    items: &[PhiBatchItem],
    tag: &str,
) -> Result<(), String> {
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| {
        format!(
            "[freeze:contract][phi_lifecycle/batch_prepend_no_function] tag={tag} No current function"
        )
    })?;
    if function.get_block(block).is_none() {
        return Err(format!(
            "[freeze:contract][phi_lifecycle/batch_prepend_missing_block] bb={block} tag={tag}"
        ));
    }

    let mut destinations = BTreeSet::<ValueId>::new();
    for item in items {
        if !destinations.insert(item.dst) {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/batch_duplicate_dst] dst=%{} tag={tag}",
                item.dst.0
            ));
        }
        if function.blocks.values().any(|candidate_block| {
            candidate_block
                .instructions
                .iter()
                .any(|instruction| instruction.dst_value() == Some(item.dst))
                || candidate_block
                    .terminator
                    .as_ref()
                    .is_some_and(|instruction| instruction.dst_value() == Some(item.dst))
        }) {
            return Err(format!(
                "[freeze:contract][phi_lifecycle/batch_dst_already_defined] dst=%{} tag={tag}",
                item.dst.0
            ));
        }
    }
    Ok(())
}
