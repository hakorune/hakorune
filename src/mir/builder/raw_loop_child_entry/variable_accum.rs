use std::cell::RefCell;
use std::rc::Rc;

use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::resolved_lowering::loop_recipe_physicalizer::lower_loop_node_physical_admission_with_callable_entry_values_v1;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::loop_node_physical_admission::issue_variable_accum_recurrence_physical_admission_v1;
use crate::mir::loop_recipe_contract::{
    LoopBindingEffectAnchorV1, LoopOperationV1, LoopRecipeItemV1,
    VerifiedVariableAccumRecurrenceRecipeProductV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, SourceNodeSiteV1, SourceStmtSiteV1};
use crate::mir::ValueId;

pub(super) fn lower(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    site: SourceStmtSiteV1,
    product: VerifiedVariableAccumRecurrenceRecipeProductV1,
    callable_ledger: &Rc<RefCell<CallableSemanticLoweringState>>,
) -> Result<ValueId, String> {
    let (read_sites, write_sites, result_binding) =
        source_consumption(input.owner(), &site, &product)?;
    let entries = {
        let state = callable_ledger.borrow();
        state.validate_variable_accum_source_sites(&read_sites, &write_sites)?;
        product
            .inputs()
            .rows()
            .iter()
            .map(|row| {
                let binding = row.source_binding();
                let value = state
                    .value_for_exact_binding(input.owner(), binding)
                    .map_err(|error| {
                        format!(
                            "[freeze:contract][callable-loop/variable-accum/entry-value] {error}"
                        )
                    })?;
                Ok((binding, value))
            })
            .collect::<Result<Vec<_>, String>>()?
    };
    if product.inputs().owner() != input.owner() {
        return Err("[freeze:contract][callable-loop/variable-accum/input-owner]".to_owned());
    }

    let admission = issue_variable_accum_recurrence_physical_admission_v1(input, site, product)
        .map_err(|error| {
            format!("[freeze:contract][callable-loop/variable-accum/admission] {error:?}")
        })?;
    let continuation = lower_loop_node_physical_admission_with_callable_entry_values_v1(
        builder, input, admission, &entries,
    )?;
    let after = continuation.root_after();
    let writebacks = continuation.writebacks();
    let result = writebacks
        .iter()
        .find_map(|(binding, value)| (*binding == result_binding).then_some(*value))
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/variable-accum/result-writeback-missing]".to_owned()
        })?;
    callable_ledger
        .borrow_mut()
        .consume_variable_accum_source_sites(&read_sites, &write_sites, writebacks)?;
    builder.function_state.current_block = Some(after);
    #[cfg(test)]
    if super::test_route_observation::take_failure_after_variable_accum_consume() {
        return Err(
            "[test-only][callable-loop/variable-accum/failure-after-source-consume]".to_owned(),
        );
    }
    Ok(result)
}

fn source_consumption(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: &SourceStmtSiteV1,
    product: &VerifiedVariableAccumRecurrenceRecipeProductV1,
) -> Result<
    (
        Box<[(SourceNodeSiteV1, BindingRefV1)]>,
        Box<[(SourceNodeSiteV1, BindingRefV1)]>,
        BindingRefV1,
    ),
    String,
> {
    let operations = product.operations();
    let core = operations.core();
    let recipe = core.recipe().as_recipe();
    if core.owner() != owner {
        return Err("[freeze:contract][callable-loop/variable-accum/recipe-owner]".to_owned());
    }
    let mut reads = Vec::new();
    let mut writes = Vec::new();
    for row in &recipe.items {
        let LoopRecipeItemV1::Operation { operation } = &row.item else {
            continue;
        };
        let expected_binding = match operation {
            LoopOperationV1::ReadBinding { binding, .. }
            | LoopOperationV1::WriteBinding { binding, .. } => Some(*binding),
            _ => None,
        };
        let Some(expected_binding) = expected_binding else {
            continue;
        };
        let evidence = operations
            .evidence()
            .iter()
            .find(|evidence| evidence.item() == row.key)
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/variable-accum/operation-evidence-missing]"
                    .to_owned()
            })?;
        if evidence.source_loop() != site {
            return Err(
                "[freeze:contract][callable-loop/variable-accum/operation-site-mismatch]"
                    .to_owned(),
            );
        }
        let source_binding = evidence.source_binding().ok_or_else(|| {
            "[freeze:contract][callable-loop/variable-accum/operation-binding-missing]".to_owned()
        })?;
        let binding_relation = core
            .binding_relations()
            .iter()
            .find(|relation| relation.recipe_binding() == expected_binding)
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/variable-accum/binding-relation-missing]"
                    .to_owned()
            })?;
        if source_binding != binding_relation.source_binding() || source_binding.owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/variable-accum/source-binding-mismatch]"
                    .to_owned(),
            );
        }
        let LoopBindingEffectAnchorV1::Expr(expr) = evidence.anchor() else {
            return Err(
                "[freeze:contract][callable-loop/variable-accum/operation-anchor-shape]".to_owned(),
            );
        };
        if expr.owner() != owner {
            return Err(
                "[freeze:contract][callable-loop/variable-accum/operation-anchor-owner]".to_owned(),
            );
        }
        let row = (expr.site().node().clone(), source_binding);
        match operation {
            LoopOperationV1::ReadBinding { .. } => reads.push(row),
            LoopOperationV1::WriteBinding { .. } => writes.push(row),
            _ => unreachable!("binding-bearing operation matched above"),
        }
    }

    let root_carrier = recipe
        .carriers
        .iter()
        .find(|carrier| carrier.owner_loop == recipe.root_loop)
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/variable-accum/root-carrier-missing]".to_owned()
        })?;
    let result_binding = core
        .binding_relations()
        .iter()
        .find(|relation| relation.recipe_binding() == root_carrier.binding)
        .map(|relation| relation.source_binding())
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/variable-accum/result-binding-missing]".to_owned()
        })?;
    Ok((
        reads.into_boxed_slice(),
        writes.into_boxed_slice(),
        result_binding,
    ))
}
