//! Carrier merge helpers for loop(cond) normalization.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::normalizer::loop_body_lowering;
use crate::mir::builder::control_flow::plan::CoreEffectPlan;
use crate::mir::builder::MirBuilder;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_assignment_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    carrier_phis: &BTreeMap<String, crate::mir::ValueId>,
    carrier_updates: &mut BTreeMap<String, crate::mir::ValueId>,
    target: &ASTNode,
    value: &ASTNode,
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let (binding, effects) = loop_body_lowering::lower_assignment_stmt(
        builder,
        current_bindings,
        target,
        value,
        error_prefix,
    )?;
    let Some((name, value_id)) = binding else {
        return Ok(effects);
    };
    if carrier_phis.contains_key(&name) {
        carrier_updates.insert(name.clone(), value_id);
    }
    if carrier_phis.contains_key(&name) || current_bindings.contains_key(&name) {
        current_bindings.insert(name.clone(), value_id);
    }
    builder.variable_ctx.variable_map.insert(name, value_id);
    Ok(effects)
}

pub(in crate::mir::builder) fn lower_local_init_stmt(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, crate::mir::ValueId>,
    variables: &[String],
    initial_values: &[Option<Box<ASTNode>>],
    error_prefix: &str,
) -> Result<Vec<CoreEffectPlan>, String> {
    let (inits, effects) = loop_body_lowering::lower_local_init_values(
        builder,
        current_bindings,
        variables,
        initial_values,
        error_prefix,
    )?;
    for (name, value_id) in inits {
        current_bindings.insert(name.clone(), value_id);
        builder.variable_ctx.variable_map.insert(name, value_id);
    }
    Ok(effects)
}
