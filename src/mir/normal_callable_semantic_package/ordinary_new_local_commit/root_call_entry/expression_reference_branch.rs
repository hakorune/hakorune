//! Branch value observations over the existing source and physical join owners.
use super::*;

/// The original IfForm observation and its sole indexed Phi. This belongs to
/// the same value purpose; it is not another CFG writer or source registry.
#[derive(Debug)]
pub(in crate::mir) struct BranchScalarPhiObservationV1 {
    source:
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    producers: [ReferenceProducerV1; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}

/// Exact physical observation for the ordered Normal inputs of a source Text
/// branch join. This is separate from Loop Header/After Text point purpose.
#[derive(Debug)]
pub(in crate::mir) struct BranchTextPhiObservationV1 {
    source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>,
    pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
    branch: Binding,
    exits: [BasicBlockId; 2],
    jumps: [Binding; 2],
    inputs: [BindingValueObservationV2; 2],
    relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
}
impl BranchTextPhiObservationV1 {
    pub(in crate::mir) fn pending(&self) -> &std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1> { &self.pending }
    pub(in crate::mir) fn source(&self) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1> { &self.source }
    pub(in crate::mir) fn relation(&self) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> { &self.relation }
    pub(super) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        context.ledger().require_branch_text_source_v1(&self.source)?;
        let map_block = |block| finishing.and_then(|projection| projection.destination(block)).unwrap_or(block);
        let actual = |row: &Binding| -> Result<Option<Binding>, String> {
            match finishing {
                Some(projection) if row.1.dst_value().is_some() => projection.relation_binding(function, row),
                Some(projection) => projection.captured_effect_binding(function, row).map(Some),
                None => {
                    if function.blocks.get(&row.0).is_none_or(|body|
                        body.all_instructions().filter(|old| **old == row.1).count() != 1) {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                    Ok(Some(row.clone()))
                }
            }
        };
        self.relation.require_completed_v1()?;
        if finishing.is_some() {
            self.relation.verify_projected(function, &mut |row| actual(row))?;
            self.pending.relation().verify_projected(function, &mut |row| actual(row))?;
        } else {
            self.relation.verify(function)?;
            self.pending.relation().verify(function)?;
        }
        let branch = actual(&self.branch)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let merge = map_block(self.pending.merge());
        let arms = [map_block(self.pending.arms().0), map_block(self.pending.arms().1)];
        let exits = self.exits.map(map_block);
        if branch.0 != map_block(self.pending.block())
            || !matches!(branch.1, MirInstruction::Branch { condition, then_bb, else_bb, .. }
                if condition == self.pending.values().1 && [then_bb, else_bb] == arms) {
            return Err(freeze("expression/text-producer-kind"));
        }
        for (index, jump) in self.jumps.iter().enumerate() {
            let jump = actual(jump)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
            if jump.0 != exits[index]
                || !matches!(jump.1, MirInstruction::Jump { target, .. } if target == merge)
                || !branch_reaches_v1(function, arms[index], exits[index], merge)
                || branch_reaches_v1(function, arms[index], exits[1 - index], merge) {
                return Err(freeze("expression/text-producer-kind"));
            }
        }
        let expected: std::collections::BTreeSet<_> = exits.into_iter().collect();
        let actual_preds: std::collections::BTreeSet<_> =
            crate::mir::verification::utils::compute_predecessors(function)
                .remove(&merge).unwrap_or_default().into_iter().collect();
        if expected.len() != 2 || actual_preds != expected
            || function.blocks.get(&merge).is_none_or(|body| body.predecessors != expected) {
            return Err(freeze("expression/text-producer-kind"));
        }
        let phi = actual(self.relation.identity_phi_binding_v1()?)?
            .ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let values = [self.inputs[0].binding_value().1, self.inputs[1].binding_value().1];
        let mut expected_inputs = [(exits[0], values[0]), (exits[1], values[1])];
        expected_inputs.sort_by_key(|(block, _)| block.0);
        if phi.0 != merge || !matches!(phi.1, MirInstruction::Phi { dst, ref inputs, .. }
            if dst == self.relation.values().0 && inputs.as_slice() == expected_inputs) {
            return Err(freeze("expression/text-producer-kind"));
        }
        for (index, (_, _, source)) in self.source.incoming().iter().enumerate() {
            ExpressionReferenceReadV1::validate_text_binding_with_finishing_v2(
                self.source.binding(), source, &self.inputs[index], context, function, finishing,
            )?;
        }
        Ok(())
    }
}

impl BranchScalarPhiObservationV1 {
    pub(in crate::mir) fn pending(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1> {
        &self.pending
    }
    pub(in crate::mir) fn source(
        &self,
    ) -> &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>
    {
        &self.source
    }
    pub(in crate::mir) fn relation(
        &self,
    ) -> &std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1> {
        &self.relation
    }
    pub(in crate::mir) fn validate(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        context
            .ledger()
            .require_branch_scalar_source_v1(&self.source)?;
        let map_block = |block| {
            finishing
                .and_then(|projection| projection.destination(block))
                .unwrap_or(block)
        };
        let actual = |row: &Binding| -> Result<Option<Binding>, String> {
            match finishing {
                Some(projection) if row.1.dst_value().is_some() => {
                    projection.relation_binding(function, row)
                }
                Some(projection) => projection.captured_effect_binding(function, row).map(Some),
                None => {
                    if function.blocks.get(&row.0).is_none_or(|body| {
                        body.all_instructions().filter(|old| **old == row.1).count() != 1
                    }) {
                        return Err(freeze("expression/reference-dependency-missing"));
                    }
                    Ok(Some(row.clone()))
                }
            }
        };
        self.relation.require_completed_v1()?;
        if finishing.is_some() {
            self.relation
                .verify_projected(function, &mut |row| actual(row))?;
            self.pending
                .relation()
                .verify_projected(function, &mut |row| actual(row))?;
        } else {
            self.relation.verify(function)?;
            self.pending.relation().verify(function)?;
        }
        let branch = actual(&self.branch)?
            .ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let merge = map_block(self.pending.merge());
        let arms = [
            map_block(self.pending.arms().0),
            map_block(self.pending.arms().1),
        ];
        let exits = self.exits.map(map_block);
        if branch.0 != map_block(self.pending.block())
            || !matches!(branch.1,
            MirInstruction::Branch { condition, then_bb, else_bb, .. }
                if condition == self.pending.values().1 && [then_bb, else_bb] == arms)
        {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        for (index, jump) in self.jumps.iter().enumerate() {
            let jump =
                actual(jump)?.ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
            if jump.0 != exits[index]
                || !matches!(jump.1, MirInstruction::Jump { target, .. } if target == merge)
                || !branch_reaches_v1(function, arms[index], exits[index], merge)
                || branch_reaches_v1(function, arms[index], exits[1 - index], merge)
            {
                return Err(freeze("expression/scalar-producer-kind"));
            }
        }
        let expected: std::collections::BTreeSet<_> = exits.into_iter().collect();
        let actual_preds: std::collections::BTreeSet<_> =
            crate::mir::verification::utils::compute_predecessors(function)
                .remove(&merge)
                .unwrap_or_default()
                .into_iter()
                .collect();
        if expected.len() != 2
            || actual_preds != expected
            || function
                .blocks
                .get(&merge)
                .is_none_or(|body| body.predecessors != expected)
        {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        let phi = actual(self.relation.identity_phi_binding_v1()?)?
            .ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
        let values = [
            self.inputs[0].binding_value().1,
            self.inputs[1].binding_value().1,
        ];
        let mut expected_inputs = [(exits[0], values[0]), (exits[1], values[1])];
        expected_inputs.sort_by_key(|(block, _)| block.0);
        if phi.0 != merge
            || !matches!(phi.1, MirInstruction::Phi { dst, ref inputs, .. }
            if dst == self.relation.values().0 && inputs.as_slice() == expected_inputs)
        {
            return Err(freeze("expression/scalar-producer-kind"));
        }
        for (index, (_, _, source)) in self.source.incoming().iter().enumerate() {
            validate_scalar_binding_observation_v1(
                self.source.binding(),
                source,
                &self.inputs[index],
                &self.producers[index],
                context,
                function,
                finishing,
            )?;
        }
        Ok(())
    }
}
fn branch_reaches_v1(
    function: &MirFunction,
    entry: BasicBlockId,
    exit: BasicBlockId,
    merge: BasicBlockId,
) -> bool {
    let mut pending = vec![entry];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(block) = pending.pop() {
        if block == merge || !seen.insert(block) {
            continue;
        }
        if block == exit {
            return true;
        }
        if let Some(body) = function.blocks.get(&block) {
            pending.extend(body.out_edges().into_iter().map(|edge| edge.target));
        }
    }
    false
}
pub(super) fn validate_scalar_binding_observation_v1(
    binding: crate::mir::resolved_semantics::BindingRefV1,
    source: &Producer,
    read: &BindingValueObservationV2,
    producer: &ReferenceProducerV1,
    context: CallValidationContextV1<'_>,
    function: &MirFunction,
    finishing: Option<&Finished>,
) -> Result<(), String> {
    validate_entry_layout(binding, source, producer, context, function)?;
    validate_scalar_origin(binding, source, read, producer, context)?;
    let finite_initializer_entry = matches!((source, producer),
        (Producer::Initializer(frame), ReferenceProducerV1::Entry(_))
            if matches!(frame.evaluation().references().get(&0), Some(Reference::FiniteEntry { .. })));
    if let (Producer::Initializer(frame), ReferenceProducerV1::Entry(entry)) = (source, producer) {
        if finite_initializer_entry {
            validate_initializer_finite_entry_v1(binding, frame, read, *entry, context, function)?;
        }
    }
    if matches!(source, Producer::BranchNormal(_)) {
        read.loan_scalar_join_v1(source)?
            .branch()
            .ok_or_else(|| freeze("expression/scalar-producer-kind"))?
            .validate(context, function, finishing)?;
    }
    if let Some(projection) = finishing {
        if let ReferenceProducerV1::Phi(point) = producer {
            point.require_completed_v1()?;
            point.verify_projected(function, &mut |row| {
                projection.relation_binding(function, row)
            })?;
        }
        if let ReferenceProducerV1::Definition(row) = producer {
            projection.relation_binding(function, row)?;
        }
        for row in read.original_copy_dependencies_v1()? {
            projection.relation_binding(function, row)?;
        }
    } else if !finite_initializer_entry {
        validate_dependencies(read, producer, function)?;
    }
    Ok(())
}
impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn prepare_branch_text_phi_v1(
        source: std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<BranchTextPhiObservationV1>, String> {
        context.ledger().require_branch_text_source_v1(&source)?;
        let [Some(then_exit), Some(else_exit)] = exits else {
            return Err(freeze("expression/text-producer-kind"));
        };
        let get_jump = |block| function.blocks.get(&block).and_then(|body| body.terminator.clone())
            .map(|row| (block, row)).ok_or_else(|| freeze("expression/reference-dependency-missing"));
        let observed = std::rc::Rc::new(BranchTextPhiObservationV1 {
            source, pending, branch, exits: [then_exit, else_exit],
            jumps: [get_jump(then_exit)?, get_jump(else_exit)?], inputs, relation,
        });
        observed.validate(context, function, None)?;
        Ok(observed)
    }

    pub(in crate::mir) fn prepare_branch_scalar_phi_v1(
        source: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1,
        >,
        pending: std::rc::Rc<crate::mir::builder::IfConditionMaterializationV1>,
        branch: Binding,
        exits: [Option<BasicBlockId>; 2],
        inputs: [BindingValueObservationV2; 2],
        relation: std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<std::rc::Rc<BranchScalarPhiObservationV1>, String> {
        context.ledger().require_branch_scalar_source_v1(&source)?;
        let [Some(then_exit), Some(else_exit)] = exits else {
            return Err(freeze("expression/scalar-producer-kind"));
        };
        let get_jump = |block| {
            function
                .blocks
                .get(&block)
                .and_then(|body| body.terminator.clone())
                .map(|row| (block, row))
                .ok_or_else(|| freeze("expression/reference-dependency-missing"))
        };
        let producers = [
            prepare_scalar_binding_producer(
                source.binding(),
                &source.incoming()[0].2,
                &inputs[0],
                context,
                function,
            )?,
            prepare_scalar_binding_producer(
                source.binding(),
                &source.incoming()[1].2,
                &inputs[1],
                context,
                function,
            )?,
        ];
        let observed = std::rc::Rc::new(BranchScalarPhiObservationV1 {
            source,
            pending,
            branch,
            exits: [then_exit, else_exit],
            jumps: [get_jump(then_exit)?, get_jump(else_exit)?],
            inputs,
            producers,
            relation,
        });
        observed.validate(context, function, None)?;
        Ok(observed)
    }
}
