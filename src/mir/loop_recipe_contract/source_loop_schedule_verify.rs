//! Validation of the source-owned schedule and its value operations.
use super::*;

impl SourceLoopScheduleDraftV2 {
    pub(super) fn verify(&self, input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
        verified: &VerifiedLoopRecipeV2, execution: &[SourceLoopEvaluationLifecycleV2],
        values: &[SourceLoopValueV2], control: &[SourceLoopItemV2], bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    ) -> Result<(), String> {
        let recipe = verified.as_recipe();
        let demanded = super::super::source_loop_v2::demanded_branch_joins_v2(source)?;
        if demanded.len() != self.joins_v2().count() || demanded.iter().any(|source|
            self.joins_v2().filter(|join| &join.source == source).count() != 1) {
            return Err(reject("schedule-operation-source"));
        }
        let demanded_text = super::super::source_loop_v2::demanded_text_branch_joins_v2(source)?;
        if demanded_text.len() != self.text_joins_v2().count() || demanded_text.iter().any(|source|
            self.text_joins_v2().filter(|join| &join.source == source).count() != 1) {
            return Err(reject("schedule-text-operation-source"));
        }
        if source.source.owner() != input.owner() || self.frames.len() != execution.len()
            || self.blocks.len() != recipe.blocks.len() || recipe.loops.len() != 1
            || self.operations.len() + self.blocks.values().flatten()
                .filter(|step| matches!(step, SourceLoopScheduledStepV2::OwnedArrayAppend { .. } | SourceLoopScheduledStepV2::Call { .. })).count()
                + self.joins_v2().count()
                + self.joins_v2().filter(|join| join.write_item.is_some()).count()
                + self.text_joins_v2().count()
                + self.text_joins_v2().filter(|join| join.write_item.is_some()).count()
                + self.blocks.values().flatten().map(|step| match step {
                    SourceLoopScheduledStepV2::ChildLoop { inputs, outputs, .. } =>
                        inputs.iter().filter(|row| row.1.is_some()).count()
                            + outputs.iter().map(|row| 1 + usize::from(row.2.is_some())).sum::<usize>(),
                    _ => 0,
                }).sum::<usize>()
                != recipe.items.iter().filter(|row| matches!(row.item, LoopRecipeItemV2::Operation { .. })).count() {
            return Err(reject("schedule-census"));
        }
        for (frame, execution) in self.frames.iter().zip(execution) {
            let original = match frame.prefix() {
                Prefix::BoolCondition { frame, .. } => Rc::ptr_eq(frame, &source.condition)
                    || source.body_expressions.iter().any(|plan| plan.condition_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                Prefix::InitializerExpression { frame, .. } => source.body_expressions.iter()
                    .any(|plan| plan.initializer_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                Prefix::BindingRebind { frame, .. } => source.body_expressions.iter()
                    .any(|plan| plan.rebind_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                _ => false,
            };
            if !original || !Rc::ptr_eq(frame.source(), &execution.source)
                || frame.prefix.site() != &OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[0].site().clone())
                || values.iter().filter(|row| row.node == 0 && Rc::ptr_eq(&row.source, frame.source())
                    && row.value == frame.result).count() != 1 {
                return Err(reject("schedule-original-frame"));
            }
            let text_transition = match (frame.prefix(), execution.text_carrier_transition.as_ref()) {
                (Prefix::BindingRebind { frame: rebind, .. }, Some(transition)) => {
                    let same_basis = rebind.text_loop_carrier_v1()
                        .is_some_and(|basis| Rc::ptr_eq(basis, &transition.basis));
                    let old_end_matches = match (&transition.before, transition.old_residence) {
                        (super::super::source_loop_v2::TextProducer::Header(basis),
                            super::super::source_loop_execution::SourceLoopTextOldResidenceV2::ConditionalHeader) =>
                            Rc::ptr_eq(basis, &transition.basis),
                        (super::super::source_loop_v2::TextProducer::Residence(_),
                            super::super::source_loop_execution::SourceLoopTextOldResidenceV2::OwnedResidence) => true,
                        (super::super::source_loop_v2::TextProducer::BranchNormal(source),
                            super::super::source_loop_execution::SourceLoopTextOldResidenceV2::ConditionalHeader) =>
                            source.is_loop_header_or_owned_residence_tree_v1(&transition.basis),
                        _ => false,
                    };
                    same_basis && old_end_matches && rebind.target() == transition.basis.binding()
                        && Rc::ptr_eq(rebind.source(), &transition.evaluation)
                        && Rc::ptr_eq(&transition.evaluation, &execution.source)
                        && transition.after.binding() == rebind.target()
                        && Rc::ptr_eq(transition.after.source(), rebind.source())
                        && !execution.normal_temporary_ends.contains(&0)
                        && execution.faults.iter().all(|fault| fault.text_carriers.iter().any(|state|
                            Rc::ptr_eq(&state.basis, &transition.basis) && state.producer == transition.before))
                }
                (Prefix::BindingRebind { frame: rebind, .. }, None) => rebind.text_loop_carrier_v1().is_none(),
                (_, None) => true,
                (_, Some(_)) => false,
            };
            if !text_transition { return Err(reject("schedule-text-carrier-transition")); }
        }
        let root = &recipe.loops[0];
        let LoopConditionV2::Predicate { block, value } = root.condition else { return Err(reject("schedule-root-condition")); };
        if !matches!(self.blocks.get(&block).map(Vec::as_slice), Some([SourceLoopScheduledStepV2::Frame(0)]))
            || self.frames.first().is_none_or(|frame| !Rc::ptr_eq(frame.source(), source.condition.source()) || frame.result != value) {
            return Err(reject("schedule-root-condition"));
        }
        let mut check = ScheduleCheck { draft: self, recipe, input, source, bindings, values,
            frames: Vec::new(), items: BTreeSet::new(), blocks: BTreeSet::new(), control: Vec::new() };
        let available = recipe.inputs.iter().copied().collect();
        let available = check.block(block, available)?.ok_or_else(|| reject("schedule-condition-exit"))?;
        check.block(root.body, available)?;
        if check.blocks.len() != recipe.blocks.len() || check.items.len() != recipe.items.len()
            || check.frames != (0..execution.len()).collect::<Vec<_>>()
            || check.control.len() != control.len()
            || check.control.iter().zip(control).any(|((site, item), old)| site != &old.site || item != &old.item) {
            return Err(reject("schedule-ordered-coverage"));
        }
        Ok(())
    }
}

impl ScheduleCheck<'_, '_> {
    pub(super) fn array_dynamic_core_slot_matches(
        &self,
        frame: &SourceLoopScheduledFrameV2,
        args: &[LoopValueKeyV1],
        result: Option<LoopValueKeyV1>,
    ) -> bool {
        let Some(result) = result else { return false; };
        let mut matches = frame.evaluation().cores().iter().filter(|(&node, core)| {
            if !core.is_array_dynamic_get_v1()
                || !core.matches_plan(frame.source(), node)
                || self.values.iter().filter(|value| {
                    Rc::ptr_eq(&value.source, frame.source())
                        && value.node == node
                        && value.value == result
                }).count() != 1
                || self.recipe.values.get(result.raw() as usize).is_none_or(|value| {
                    value.key != result || value.class != LoopValueClassV2::Dynamic
                })
            {
                return false;
            }
            let expected: Option<Vec<_>> = core.source().source().arguments().iter().map(|argument| {
                let mut rows = self.values.iter().filter(|value| {
                    Rc::ptr_eq(&value.source, frame.source())
                        && frame.source().nodes().get(value.node)
                            .is_some_and(|node| node.site() == argument.site())
                });
                let value = rows.next()?.value;
                rows.next().is_none().then_some(value)
            }).collect();
            expected.as_deref() == Some(args)
        });
        matches.next().is_some() && matches.next().is_none()
    }

    pub(super) fn integer_operation(&self, frame: &SourceLoopScheduledFrameV2, op: LoopBinaryI64OpV2,
        left: LoopValueKeyV1, right: LoopValueKeyV1, result: LoopValueKeyV1) -> Result<(), String> {
        let mut results = self.values.iter().filter(|value| value.value == result
            && Rc::ptr_eq(&value.source, frame.source()));
        let value = results.next().ok_or_else(|| reject("schedule-operation-source"))?;
        let binary = frame.source().nodes().get(value.node).and_then(|node| node.binary())
            .ok_or_else(|| reject("schedule-operation-source"))?;
        let operand_matches = |site, key| self.values.iter().filter(|value|
            Rc::ptr_eq(&value.source, frame.source()) && value.value == key
                && frame.source().nodes().get(value.node).is_some_and(|node| node.site() == site)).count() == 1;
        if results.next().is_some()
            || super::super::source_loop_v2::integer_operation_v2(frame.evaluation(), value.node) != Some(op)
            || !operand_matches(binary.lhs(), left) || !operand_matches(binary.rhs(), right) {
            return Err(reject("schedule-operation-source"));
        }
        Ok(())
    }
    pub(super) fn text_add_operation(&self, frame: &SourceLoopScheduledFrameV2,
        left: LoopValueKeyV1, right: LoopValueKeyV1, result: LoopValueKeyV1) -> Result<(), String> {
        let mut results = self.values.iter().filter(|value| value.value == result
            && Rc::ptr_eq(&value.source, frame.source()));
        let value = results.next().ok_or_else(|| reject("schedule-operation-source"))?;
        let binary = frame.source().nodes().get(value.node).and_then(|node| node.binary())
            .ok_or_else(|| reject("schedule-operation-source"))?;
        let operand_matches = |site, key| self.values.iter().filter(|value|
            Rc::ptr_eq(&value.source, frame.source()) && value.value == key
                && frame.source().nodes().get(value.node).is_some_and(|node| node.site() == site)).count() == 1;
        if results.next().is_some() || !operand_matches(binary.lhs(), left)
            || !operand_matches(binary.rhs(), right) {
            return Err(reject("schedule-operation-source"));
        }
        super::super::source_loop_v2::verify_text_add_source_v2(frame.evaluation(), value.node, binary)
    }
    pub(super) fn text_read(&self, frame: &SourceLoopScheduledFrameV2, binding: LoopBindingKeyV1,
        result: LoopValueKeyV1) -> Result<(), String> {
        let mut references = frame.references().iter().filter(|(node, _)| self.values.iter().any(|value|
            value.node == **node && value.value == result && Rc::ptr_eq(&value.source, frame.source())));
        let (_, reference) = references.next().ok_or_else(|| reject("schedule-operation-source"))?;
        // Static and entry reads keep their existing source owners. Only the
        // newly imported outer residence requires this acquisition contract.
        if !matches!(reference, super::super::source_loop_v2::Reference::Residence(_)) { return Ok(()); }
        if references.next().is_some()
            || self.bindings.get(&reference.binding()) != Some(&binding)
            || !super::super::source_loop_v2::is_outer_text_reference_v2(self.source, reference) {
            return Err(reject("schedule-operation-source"));
        }
        let mut inputs = self.recipe.read_only_inputs.iter().filter(|input| input.binding == binding);
        let input = inputs.next().ok_or_else(|| reject("schedule-operation-source"))?;
        if inputs.next().is_some() || input.owner_loop != self.recipe.root_loop
            || input.class != LoopValueClassV2::Text || !self.recipe.inputs.contains(&input.entry_value)
            || self.recipe.values.get(input.entry_value.raw() as usize)
                .is_none_or(|value| value.key != input.entry_value || value.class != LoopValueClassV2::Text) {
            return Err(reject("schedule-operation-source"));
        }
        Ok(())
    }
}
