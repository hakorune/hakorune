//! Exact formal-parameter adoption for the trivial Binding SSA route.
//!
//! Parameter ValueIds are reserved by `MirFunction::new`. This module only
//! connects sealed source identity and ABI rows to those reserved values; it
//! never allocates or publishes through the legacy name maps.

use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::BindingKindV1;
use crate::mir::resolved_value_profile::{
    product::TrivialRepresentationV1, TrivialProfileConsumptionV1,
};
use crate::mir::ValueId;

use super::identity::ResolvedSsaIdentityStateV2;

pub(super) fn publish_parameter_entries_v1(
    builder: &mut MirBuilder,
    identity: &mut ResolvedSsaIdentityStateV2<'_>,
    profile: &mut TrivialProfileConsumptionV1,
) -> Result<(), String> {
    let expected_count = profile.parameter_entry_count();
    let (entry_block, function_params, signature_params) = {
        let function = builder.scope_ctx.current_function.as_ref().ok_or_else(|| {
            "[freeze:contract][canonical_binding_ssa/parameter_function_missing]".to_string()
        })?;
        (
            function.entry_block,
            function.params.clone(),
            function.signature.params.clone(),
        )
    };
    if function_params.len() != expected_count || signature_params.len() != expected_count {
        return Err(format!(
            "[freeze:contract][canonical_binding_ssa/parameter_cardinality_drift] profile={} values={} signature={}",
            expected_count,
            function_params.len(),
            signature_params.len(),
        ));
    }
    if builder.current_block != Some(entry_block) {
        return Err(format!(
            "[freeze:contract][canonical_binding_ssa/parameter_entry_block_drift] expected={entry_block:?} actual={:?}",
            builder.current_block,
        ));
    }

    for index in 0..expected_count {
        let formal_index = u32::try_from(index).map_err(|_| {
            "[freeze:contract][canonical_binding_ssa/parameter_index_overflow]".to_string()
        })?;
        let row = profile.claim_parameter_entry(formal_index)?;
        if row.representation() != TrivialRepresentationV1::InlineI64 {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/parameter_representation_drift] index={formal_index} actual={:?}",
                row.representation(),
            ));
        }
        let value = function_params[index];
        let expected_value = ValueId::new(formal_index);
        if value != expected_value {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/parameter_value_drift] index={formal_index} expected={expected_value:?} actual={value:?}"
            ));
        }
        let expected_type = row.abi().mir_type();
        if signature_params[index] != expected_type {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/parameter_type_drift] index={formal_index} expected={expected_type:?} actual={:?}",
                signature_params[index],
            ));
        }
        let binding = identity.publish_declaration(
            row.site(),
            BindingKindV1::Parameter {
                index: formal_index,
            },
            row.source_name(),
            entry_block,
            value,
        )?;
        if binding != row.binding() {
            return Err(format!(
                "[freeze:contract][canonical_binding_ssa/parameter_binding_drift] index={formal_index} expected={:?} actual={binding:?}",
                row.binding(),
            ));
        }
        builder.register_value_kind(
            value,
            hakorune_mir_core::MirValueKind::Parameter(formal_index),
        );
        builder
            .type_ctx
            .value_types
            .insert(value, expected_type.clone());
        if let Some(registry) = builder.comp_ctx.current_slot_registry.as_mut() {
            registry.ensure_slot(row.source_name(), Some(expected_type));
        }
    }
    Ok(())
}
