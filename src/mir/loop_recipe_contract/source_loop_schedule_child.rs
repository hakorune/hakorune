//! Verify one child-Loop boundary step against the SAME child product.
use super::*;

impl ScheduleCheck<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn child_loop_step(&mut self, block: &LoopRecipeBlockV2, position: &mut usize,
        available: &mut BTreeSet<LoopValueKeyV1>, site: &SourceStmtSiteV1,
        product: &Rc<super::super::source_loop_v2::VerifiedSourceLoopRecipeProductV2>,
        inputs: &[(BindingRefV1, Option<LoopItemKeyV1>, LoopValueKeyV1)],
        outputs: &[(BindingRefV1, LoopItemKeyV1, Option<LoopItemKeyV1>, LoopValueKeyV1)],
    ) -> Result<(), String> {
        let (expected_inputs, expected_outputs) = super::super::source_loop_v2::child_loop_boundary_v2(product)?;
        if super::super::source_loop_v2::source_child_loop_v2(self.source, site).is_none_or(|original| !Rc::ptr_eq(original, product))
            || product.source.source.owner() != self.input.owner()
            || inputs.len() != expected_inputs.len() || outputs.len() != expected_outputs.len() {
            return Err(reject("schedule-child-loop-source"));
        }
        let first = *position;
        for ((binding, item, result), (expected, class)) in inputs.iter().zip(&expected_inputs) {
            if binding != expected || self.recipe.values.get(result.raw() as usize)
                .is_none_or(|value| value.key != *result || value.class != *class) {
                return Err(reject("schedule-child-loop-input"));
            }
            match item {
                // A child carrier enters with the parent's current producer value.
                None => {
                    let Some(super::super::source_loop_v2::SourceProducer::Scalar(producer)) =
                        product.source.headers.get(binding).map(|basis| basis.entry_producer()) else {
                        return Err(reject("schedule-child-loop-input"));
                    };
                    if super::super::source_loop_v2::producer_value_v2(producer, self.source, self.recipe,
                        self.values, self.bindings, self.draft) != Some(*result) || !available.contains(result) {
                        return Err(reject("schedule-child-loop-input"));
                    }
                }
                // A read-only operand is read from this Loop's binding.
                Some(item) => {
                    let key = self.bindings.get(binding).copied();
                    if product.source.headers.contains_key(binding)
                        || block.items.get(*position) != Some(item) || !self.items.insert(*item)
                        || self.draft.operations.contains_key(item)
                        || key.is_none_or(|key| !matches!(&self.recipe.items[item.raw() as usize].item,
                            LoopRecipeItemV2::Operation { operation: LoopOperationV2::ReadBinding { binding: actual, result: actual_result } }
                                if *actual == key && actual_result == result)) {
                        return Err(reject("schedule-child-loop-input"));
                    }
                    available.insert(*result); *position += 1;
                }
            }
        }
        let input_values: Vec<_> = inputs.iter().map(|row| row.2).collect();
        for ((binding, after, write, result), (expected, class)) in outputs.iter().zip(&expected_outputs) {
            if binding != expected || block.items.get(*position) != Some(after) || !self.items.insert(*after)
                || self.draft.operations.contains_key(after)
                || !matches!(&self.recipe.items[after.raw() as usize].item,
                    LoopRecipeItemV2::Operation { operation: LoopOperationV2::ChildLoopAfter { inputs: actual, class: actual_class, result: actual_result } }
                        if *actual == input_values && actual_class == class && actual_result == result) {
                return Err(reject("schedule-child-loop-output"));
            }
            *position += 1;
            if let Some(write) = write {
                let key = self.bindings.get(binding).copied();
                if block.items.get(*position) != Some(write) || !self.items.insert(*write)
                    || self.draft.operations.contains_key(write)
                    || key.is_none_or(|key| !matches!(&self.recipe.items[write.raw() as usize].item,
                        LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding { binding: actual, value } }
                            if *actual == key && value == result)) {
                    return Err(reject("schedule-child-loop-output"));
                }
                *position += 1;
            }
            available.insert(*result);
        }
        let first_item = block.items.get(first).copied().ok_or_else(|| reject("schedule-child-loop-empty"))?;
        self.control.push((site.clone(), first_item));
        Ok(())
    }
}
