//! Finished non-null guard and guarded operand correspondence.
use super::*;

impl EmittedBoolConditionProjectionV1 {
    /// The exact source null comparison chooses the surviving successor.
    pub(in super::super) fn validate_nonnull_consumer_v1(
        &self,
        function: &MirFunction,
        finishing: Option<&Finished>,
        consumer: &Binding,
        producer: Option<&Binding>,
    ) -> Result<(), String> {
        let outer = self.outer.borrow();
        let outer = outer
            .as_ref()
            .ok_or_else(|| freeze("bool-condition/outer-branch-pending"))?;
        self.verify_outer(function, outer, finishing)?;
        require_binding(function, consumer, finishing)?;
        let branch = mapped(outer, finishing)?
            .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
        let consumer = mapped(consumer, finishing)?
            .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
        let MirInstruction::Branch {
            then_bb, else_bb, ..
        } = branch.1
        else {
            return Err(freeze("bool-condition/outer-branch-source"));
        };
        let nonnull = match self
            .frame
            .source()
            .nodes()
            .first()
            .and_then(|node| node.binary())
            .map(|binary| binary.operator())
        {
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Equal) => else_bb,
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::NotEqual) => then_bb,
            _ => return Err(freeze("bool-condition/outer-branch-source")),
        };
        let (producer, execution) = if let Some(producer) = producer {
            require_binding(function, producer, finishing)?;
            let producer = mapped(producer, finishing)?
                .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
            let execution = match &producer.1 {
                MirInstruction::InvokeNormalResult { invoke_block, .. } => *invoke_block,
                _ => producer.0,
            };
            (Some(producer.0), Some(execution))
        } else {
            (None, None)
        };
        // During nested If emission the enclosing edge has not been written.
        // Keep the existing local checks here; committed groups repeat this
        // check with FinishedBindings before lifecycle publication.
        let dom = crate::mir::verification::utils::compute_dominators(function);
        if !dom.dominates(branch.0, consumer.0)
            || !dom.dominates(nonnull, consumer.0)
            || producer.is_some_and(|producer| !dom.dominates(producer, branch.0))
            || (finishing.is_some()
                && !nonnull_geometry_v1(function, branch.0, nonnull, consumer.0, execution))
        {
            return Err(freeze("bool-condition/outer-branch-source"));
        }
        Ok(())
    }
}

/// A guard belongs to one reaching value: no other edge can enter its
/// nonnull arm, and a new producer cannot execute before the guarded use.
fn nonnull_geometry_v1(
    function: &MirFunction,
    branch: BasicBlockId,
    nonnull: BasicBlockId,
    consumer: BasicBlockId,
    producer: Option<BasicBlockId>,
) -> bool {
    if !matches!(function.blocks.get(&branch).and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::Branch { then_bb, else_bb, .. })
            if then_bb != else_bb && (*then_bb == nonnull || *else_bb == nonnull))
        || function.blocks.values().any(|block| {
            block.successors != block.successors_from_terminator()
                || block
                    .successors
                    .iter()
                    .any(|target| !function.blocks.contains_key(target))
        })
    {
        return false;
    }
    let dom = crate::mir::verification::utils::compute_dominators(function);
    if !dom.is_reachable(branch)
        || !dom.is_reachable(nonnull)
        || !dom.is_reachable(consumer)
        || !dom.dominates(branch, consumer)
        || !dom.dominates(nonnull, consumer)
    {
        return false;
    }
    let predecessors: Vec<_> = function
        .blocks
        .iter()
        .flat_map(|(id, block)| {
            block
                .successors_from_terminator()
                .into_iter()
                .filter_map(move |next| (next == nonnull).then_some(*id))
        })
        .collect();
    if predecessors.as_slice() != [branch] {
        return false;
    }
    let Some(producer) = producer else {
        return true;
    };
    if !dom.is_reachable(producer) || !dom.dominates(producer, branch) {
        return false;
    }
    let mut pending = vec![nonnull];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(block) = pending.pop() {
        if block == producer {
            return false;
        }
        if block == branch || block == consumer || !seen.insert(block) {
            continue;
        }
        let Some(body) = function.blocks.get(&block) else {
            return false;
        };
        pending.extend(body.successors_from_terminator());
    }
    true
}

#[cfg(test)]
mod nonnull_geometry_tests {
    use super::*;
    use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

    #[test]
    fn guard_rejects_foreign_entry_unreachable_use_and_reexecuted_producer() {
        let b = BasicBlockId::new;
        let mut f = MirFunction::new(
            FunctionSignature {
                name: "nonnull-geometry".into(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::CONTROL,
            },
            b(0),
        );
        for id in 0..=5 {
            f.add_block(BasicBlock::new(b(id)));
        }
        let jump = |target| MirInstruction::Jump {
            target,
            edge_args: None,
        };
        let branch = |then_bb, else_bb| MirInstruction::Branch {
            condition: ValueId::new(0),
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        };
        f.blocks.get_mut(&b(0)).unwrap().set_terminator(jump(b(1)));
        f.blocks
            .get_mut(&b(1))
            .unwrap()
            .set_terminator(branch(b(2), b(3)));
        f.blocks.get_mut(&b(2)).unwrap().set_terminator(jump(b(4)));
        for id in [3, 4, 5] {
            f.blocks
                .get_mut(&b(id))
                .unwrap()
                .set_terminator(MirInstruction::Return { value: None });
        }
        assert!(nonnull_geometry_v1(&f, b(1), b(2), b(4), Some(b(0))));
        assert!(!nonnull_geometry_v1(&f, b(1), b(2), b(3), Some(b(0))));
        assert!(!nonnull_geometry_v1(&f, b(1), b(2), b(5), Some(b(0))));
        let mut same = f.clone();
        same.blocks
            .get_mut(&b(1))
            .unwrap()
            .set_terminator(branch(b(2), b(2)));
        assert!(!nonnull_geometry_v1(&same, b(1), b(2), b(4), Some(b(0))));
        let mut stale = f.clone();
        stale.blocks.get_mut(&b(3)).unwrap().successors.insert(b(2));
        assert!(!nonnull_geometry_v1(&stale, b(1), b(2), b(4), Some(b(0))));
        let mut extra = f.clone();
        extra
            .blocks
            .get_mut(&b(3))
            .unwrap()
            .set_terminator(jump(b(2)));
        assert!(!nonnull_geometry_v1(&extra, b(1), b(2), b(4), Some(b(0))));
        let mut reentry = f;
        reentry
            .blocks
            .get_mut(&b(2))
            .unwrap()
            .set_terminator(branch(b(0), b(4)));
        assert!(!nonnull_geometry_v1(&reentry, b(1), b(2), b(4), Some(b(0))));
    }
}

impl CallValidationContextV1<'_> {
    /// Source SUM refinement must also pass the SAME finished Bool/Branch owner.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_finite_entry_nonnull_use_v1(
        self,
        read: &crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
        exclusion: &crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1,
        function: &MirFunction,
        finishing: &Finished,
        consumer: &Binding,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
        self.with_local_call_group_tree_v2(read.requirement().binding().owner(), |groups| {
            let mut matches = groups.iter().filter_map(|group| {
                let packet = group.bool_condition()?;
                let guard = exclusion.condition_read_v1(packet.frame.source())?;
                Some((group, packet, guard))
            });
            let (group, packet, guard) = matches
                .next()
                .ok_or_else(|| freeze("bool-condition/reference-source"))?;
            if matches.next().is_some()
                || exclusion.root_binding() != read.requirement().formal().1
                || !matches!(packet.materialization, BoolConditionOuterV2::If(_))
            {
                return Err(freeze("bool-condition/reference-source"));
            }
            let node = packet
                .frame
                .source()
                .nodes()
                .iter()
                .position(|node| node.site() == guard)
                .ok_or_else(|| freeze("bool-condition/reference-source"))?;
            let Some(Reference::FiniteEntry {
                read: original,
                exclusion: None,
            }) = packet.frame.references().get(&node)
            else {
                return Err(freeze("bool-condition/reference-source"));
            };
            if original.requirement().formal() != read.requirement().formal()
                || original.null_compare_v1().is_none()
                || !matches!(
                    packet.node(node)?.proof,
                    BoolConditionValueProofV1::OriginalReference(_)
                )
            {
                return Err(freeze("bool-condition/reference-source"));
            }
            group.validate_recorded_in_context(self, function, Some(finishing))?;
            packet.validate_nonnull_consumer_v1(function, Some(finishing), consumer, None)
        })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Join one original Loop operand to its sealed Bool packet. This lends an
    /// occurrence and its exact Copy ancestry, never a legacy Greater guard.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn visit_finished_loop_compare_operand_v1(
        &self,
        owner: FunctionOwnerIdV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
        loop_site: &crate::mir::resolved_semantics::SourceStmtSiteV1,
        binary: &OwnedExprSiteV1,
        original_read: &OwnedExprSiteV1,
        binding: BindingRefV1,
        expected: ValueId,
        mut visit: impl FnMut(Binding, usize, Vec<Binding>) -> Result<(), String>,
    ) -> Result<(), String> {
        let mut found = false;
        self.visit_verified_condition_groups_v1(owner, context, function, projection, |packet| {
            use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
            if !matches!(packet.materialization, BoolConditionOuterV2::SourceLoop { .. }) { return Ok(()); }
            let source = packet.frame.source();
            let ExpressionPlanRootV1::LoopCondition { statement, condition, .. } = source.root()
                else { return Ok(()); };
            if statement != loop_site { return Ok(()); }
            if found || source.owner() != owner || binary.owner() != owner || original_read.owner() != owner
                || condition != binary.site() {
                return Err(freeze("static-input/loop-condition-use-source"));
            }
            let (binary_index, operation) = source.nodes().iter().enumerate()
                .find(|(_, node)| node.site() == binary.site())
                .ok_or_else(|| freeze("bool-condition/operator-source"))?;
            let Node::Binary(operation) = operation.source() else { return Err(freeze("bool-condition/operator-source")); };
            let side = match (operation.lhs() == original_read.site(), operation.rhs() == original_read.site()) {
                (true, false) => 0, (false, true) => 1,
                _ => return Err(freeze("bool-condition/ordered-operands")),
            };
            let read_index = source.nodes().iter().position(|node| node.site() == original_read.site())
                .ok_or_else(|| freeze("bool-condition/reference-source"))?;
            let original = &source.nodes()[read_index];
            if !matches!(original.source(), Node::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(actual)) if *actual == binding) {
                return Err(freeze("bool-condition/reference-source"));
            }
            let reference = packet.node(read_index)?;
            let read = match &reference.proof {
                BoolConditionValueProofV1::Reference { read, .. } => read,
                BoolConditionValueProofV1::OriginalReference(read) => read.exact_read_v1(),
                _ => return Err(freeze("bool-condition/reference-source")),
            };
            if read.value_for(owner, original_read.site().node(), binding).map_err(|issue| issue.to_string())? != reference.value {
                return Err(freeze("bool-condition/reference-value"));
            }
            let mut copies = read.loan_forwarded_copies(binding, expected)?.to_vec();
            let node = packet.node(binary_index)?;
            let BoolConditionValueProofV1::Binary { append, operands } = &node.proof
                else { return Err(freeze("bool-condition/operator-append")); };
            let MirInstruction::Compare { lhs, rhs, .. } = &append.1
                else { return Err(freeze("bool-condition/operator-append")); };
            if operands[side].values() != (reference.value, if side == 0 { *lhs } else { *rhs }) {
                return Err(freeze("bool-condition/ordered-operands"));
            }
            operands[side].require_completed_v1()?;
            operands[side].verify_projected(function, &mut |row| projection.relation_binding(function, row))?;
            operands[side].append_observed_bindings_v1(&mut copies);
            let mut finished_copies = Vec::new();
            for original in copies {
                if !matches!(original.1, MirInstruction::Copy { .. }) { continue; }
                if let Some(actual) = projection.relation_binding(function, &original)? {
                    if !finished_copies.contains(&actual) { finished_copies.push(actual); }
                }
            }
            let compare = mapped(append, Some(projection))?
                .ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
            visit(compare, side, finished_copies)?;
            found = true;
            Ok(())
        })?;
        if !found {
            return Err(freeze("static-input/loop-condition-use-source"));
        }
        Ok(())
    }
}

impl CallValidationContextV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn declared_read_branch_points_v1(
        self,
        owner: FunctionOwnerIdV1,
    ) -> Result<
        Vec<Rc<crate::mir::normal_callable_semantic_package::BranchReadPhiObservationV1>>,
        String,
    > {
        let mut points = self.ledger().field_read_branch_points_v1(owner)?;
        self.with_local_call_group_tree_v2(owner, |groups| {
            for group in groups {
                if let Some(packet) = group.bool_condition() {
                    for point in packet.read_branch_points_v1()? {
                        if !points.iter().any(|old| Rc::ptr_eq(old, &point)) {
                            points.push(point);
                        }
                    }
                }
            }
            Ok(())
        })?;
        Ok(points)
    }
}
