//! Independent original source/loan to actual producer correspondence.
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1 as Node;
use crate::mir::resolved_semantics::{ResolvedLiteralSourceV1 as Literal, SourceExprSiteV1};

/// Borrow only indexed LoopHeader Phi writers reached through this condition's
/// exact scalar source reads. Rebind/initializer reads keep their own emitted
/// reference packets, so this walk follows source ownership instead of MIR
/// operands.
pub(in crate::mir) fn borrowed_source_loop_phi_relations_v2(
    frame: &crate::mir::resolved_semantics::home_new_prefix::CheckedBoolConditionFrameV1,
    nodes: &[BoolConditionNodeProjectionV1],
    context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
    expected_product: Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
) -> Result<Vec<Rc<LocalSsaPhysicalRelationV1>>, String> {
    use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
    use crate::mir::resolved_semantics::home_new_prefix::{
        CheckedExpressionEvaluationV1, CheckedExpressionReferenceV1 as Reference,
        CurrentScalarProducerV1 as Producer,
    };
    use BoolConditionValueProofV1 as Proof;

    fn collect<'a>(
        evaluation: &CheckedExpressionEvaluationV1,
        read_for: &(dyn Fn(usize) -> Option<&'a ExpressionReferenceReadV1> + 'a),
        context: CallValidationContextV1<'_>,
        expected_product: Option<&Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>>,
        relations: &mut Vec<Rc<LocalSsaPhysicalRelationV1>>,
    ) -> Result<(), String> {
        for (&node, reference) in evaluation.references() {
            let Reference::Scalar(scalar) = reference else { continue };
            let Some(source) = scalar.original_read() else { continue };
            match source.producer() {
                Producer::LoopHeader(_) => {
                    let product = context.source_loop_point_product_v2(source.producer())?;
                    if !product.owns_expression_v2(evaluation.source())
                        || expected_product.is_some_and(|expected| !Rc::ptr_eq(expected, &product)) {
                        return Err(freeze("bool-condition/loop-header-source-scope"));
                    }
                    let read = read_for(node).ok_or_else(|| freeze("bool-condition/loop-header-read-missing"))?;
                    let (site, binding, _) = read.exact_read_v1().source_value();
                    if site.owner() != evaluation.source().owner() || site.site() != source.read()
                        || binding != source.binding() {
                        return Err(freeze("bool-condition/loop-header-read-source"));
                    }
                    let point = read.exact_read_v1().loan_scalar_join_v1(source.producer())?;
                    point.relation().identity_phi_binding_v1()?;
                    if !relations.iter().any(|relation| Rc::ptr_eq(relation, point.relation())) {
                        relations.push(Rc::clone(point.relation()));
                    }
                }
                Producer::BindingRebind(rebind) => {
                    context.with_original_rebind_v1(rebind, |packet| {
                        collect(rebind.evaluation(), &|node| packet.reference_read_v1(node),
                            context, expected_product, relations)
                    })?;
                }
                Producer::Initializer(initializer) => {
                    context.with_original_initializer_v1(initializer, |packet| {
                        collect(initializer.evaluation(), &|node| packet.reference_read_v1(node),
                            context, expected_product, relations)
                    })?;
                }
                Producer::SignedEntry(_) | Producer::Integer { .. } | Producer::Call(_)
                | Producer::BranchNormal(_) | Producer::LoopAfter { .. } => {}
            }
        }
        Ok(())
    }

    fn insert_read<'a>(
        reads: &mut std::collections::BTreeMap<usize, &'a ExpressionReferenceReadV1>,
        node: usize,
        read: &'a ExpressionReferenceReadV1,
    ) -> Result<(), String> {
        if reads.insert(node, read).is_some() {
            return Err(freeze("bool-condition/duplicate-source-read"));
        }
        Ok(())
    }

    let mut reads = std::collections::BTreeMap::new();
    for node in nodes {
        match &node.proof {
            Proof::OriginalReference(read) => insert_read(&mut reads, node.node, read)?,
            Proof::ScalarInvocation { input_reads, .. } => {
                for (&index, read) in input_reads { insert_read(&mut reads, index, read)?; }
            }
            _ => {}
        }
    }
    let mut relations = Vec::new();
    collect(frame.evaluation(), &|node| reads.get(&node).copied(), context,
        expected_product, &mut relations)?;
    Ok(relations)
}

impl EmittedBoolConditionProjectionV1 {
    pub(super) fn validate_source(&self, context: CallValidationContextV1<'_>,
        function: &MirFunction, finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.materialization.validate_source(&self.frame, context)?;
        let ledger = context.ledger();
        let source = self.frame.source();
        let text_leaves = self.frame.text_comparison_input_nodes_v1();
        let expected: Vec<_> = self.frame.ordered_nodes().iter().copied().filter(|index|
            !text_leaves.contains(index) && !self.frame.transfers_node(*index) && self.frame.class(*index) != Some(
                crate::mir::resolved_value_profile::expression_projection::ExpressionLeafClassV1::Identity)).collect();
        if self.nodes.iter().map(|node| node.node).ne(expected)
            || self.node(0)?.value != self.materialization.physical().values().0 {
            return Err(freeze("bool-condition/source-node-coverage"));
        }
        let mut observed_calls = Vec::new();
        let mut observed_text = std::collections::BTreeSet::new();
        let mut observed_references = std::collections::BTreeSet::new();
        for node in &self.nodes {
            let original = &source.nodes()[node.node];
            match (original.source(), &node.proof) {
                (Node::Literal(literal), BoolConditionValueProofV1::Literal(binding)) => {
                    if !matches!((literal, &binding.1),
                        (Literal::Integer(expected), MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(actual) })
                            if *dst == node.value && expected == actual)
                        && !matches!((literal, &binding.1),
                        (Literal::Bool(expected), MirInstruction::Const { dst, value: crate::mir::ConstValue::Bool(actual) })
                            if *dst == node.value && expected == actual)
                        && !matches!((literal, &binding.1), (Literal::Null, MirInstruction::Const {
                            dst, value: crate::mir::ConstValue::Null }) if *dst == node.value)
                    { return Err(freeze("bool-condition/literal-source")); }
                    // The captured finisher alone may remove a redundant Const.
                    if mapped(binding, finishing)?.is_some() { require_binding(function, binding, finishing)?; }
                },
                (Node::Reference(_), BoolConditionValueProofV1::Reference { read, producer }) => {
                    if !observed_references.insert(node.node) { return Err(freeze("bool-condition/reference-duplicate")); }
                    let Some(crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1::Scalar(reference)) = self.frame.references().get(&node.node)
                        else { return Err(freeze("bool-condition/reference-source")); };
                    if reference.original_read().is_some() { return Err(freeze("bool-condition/reference-source")); }
                    let binding = reference.binding();
                    if read.value_for(source.owner(), original.site().node(), binding)
                        .map_err(|error| error.to_string())? != node.value {
                        return Err(freeze("bool-condition/reference-value"));
                    }
                    if let Some(producer) = producer {
                        if producer.1.dst_value() != Some(node.value) { return Err(freeze("bool-condition/reference-producer")); }
                        if mapped(producer, finishing)?.is_some() { require_binding(function, producer, finishing)?; }
                    } else if !function.params.contains(&node.value) {
                        return Err(freeze("bool-condition/reference-parameter"));
                    }
                },
                (Node::Reference(_), BoolConditionValueProofV1::OriginalReference(read)) => {
                    if !observed_references.insert(node.node) { return Err(freeze("bool-condition/reference-duplicate")); }
                    let reference = self.frame.references().get(&node.node)
                        .ok_or_else(|| freeze("bool-condition/reference-source"))?;
                    if read.validate_source_reference_v1(source, node.node, reference, context, function, finishing)? != node.value {
                        return Err(freeze("bool-condition/original-reference-value"));
                    }
                    read.validate_dependencies_v1(self.frame.evaluation(), node.node, context, function, finishing)?;
                },
                (Node::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { .. }),
                    BoolConditionValueProofV1::Field(binding)) => {
                    let site = OwnedExprSiteV1::new(source.owner(), original.site().clone());
                    let field_binding_matches = match ledger.checked_field_pair_v1(&site, context)? {
                        Some([_, normal]) => normal == *binding,
                        None => ledger.expression_field_read_emission_v1(&site, source, context)? == *binding,
                    };
                    if !field_binding_matches || binding.1.dst_value() != Some(node.value) {
                        return Err(freeze("bool-condition/field-source"));
                    }
                    require_binding(function, binding, finishing)?;
                },
                (Node::MethodCall, BoolConditionValueProofV1::Call { packet, bindings }) => {
                    let observation = self.frame.calls().iter().find(|call| call.site().site() == original.site())
                        .ok_or_else(|| freeze("bool-condition/call-source"))?;
                    if packet.call_site() != observation.site()
                        || ledger.lexical_i64_call_source(packet.call_site()) != Some(observation)
                        || packet.value_in_context_v1(source.owner(), observation.arguments(), context)? != node.value {
                        return Err(freeze("bool-condition/call-value"));
                    }
                    observed_calls.push(observation.site());
                    let projected = bindings.iter().map(|binding| mapped(binding, finishing))
                        .filter_map(|row| match row { Ok(Some(row)) => Some(Ok(row)), Ok(None) => None, Err(error) => Some(Err(error)) })
                        .collect::<Result<Vec<_>, String>>()?;
                    packet.validate_recorded_projected(&projected, finishing)?;
                },
                (Node::MethodCall, BoolConditionValueProofV1::ScheduledSourceLoopCall { packet, bindings }) => {
                    let observation = self.frame.calls().iter().find(|call| call.site().site() == original.site())
                        .ok_or_else(|| freeze("bool-condition/call-source"))?;
                    let effect = ledger.source_loop_bool_call_effect_v2(&self.frame, observation)?
                        .ok_or_else(|| freeze("bool-condition/source-loop-call-effect"))?;
                    let original_packet = effect.lexical_rc_v1()
                        .ok_or_else(|| freeze("bool-condition/source-loop-call-packet"))?;
                    if !observation.is_expression_value()
                        || packet.call_site() != observation.site()
                        || !Rc::ptr_eq(packet, &original_packet)
                        || effect.bindings() != bindings.as_ref()
                        || packet.value_in_context_v1(source.owner(), observation.arguments(), context)? != node.value
                        || ledger.lexical_i64_call_source(packet.call_site()) != Some(observation)
                        || node.entry != node.exit
                    {
                        return Err(freeze("bool-condition/source-loop-call-value"));
                    }
                    observed_calls.push(observation.site());
                },
                (Node::Binary(binary), BoolConditionValueProofV1::Binary { append, operands }) => {
                    let loan = self.frame.operations().iter().find(|loan| loan.node() == node.node)
                        .ok_or_else(|| freeze("bool-condition/operator-loan"))?;
                    if !Rc::ptr_eq(loan.source(), source) { return Err(freeze("bool-condition/operator-source")); }
                    if loan.nullable_null_v1().is_some() {
                        if !loan.matches_nullable_null_reference_v1(self.frame.references()) {
                            return Err(freeze("bool-condition/operator-source"));
                        }
                    } else if loan.single_envelope().is_none() { return Err(freeze("bool-condition/operator-loan")); }
                    let lhs = self.value_at(binary.lhs())?;
                    let rhs = self.value_at(binary.rhs())?;
                    let expected = match binary.operator() {
                        ResolvedBinaryOperatorV1::Equal => Some(crate::mir::CompareOp::Eq),
                        ResolvedBinaryOperatorV1::NotEqual => Some(crate::mir::CompareOp::Ne),
                        ResolvedBinaryOperatorV1::Less => Some(crate::mir::CompareOp::Lt),
                        ResolvedBinaryOperatorV1::LessEqual => Some(crate::mir::CompareOp::Le),
                        ResolvedBinaryOperatorV1::Greater => Some(crate::mir::CompareOp::Gt),
                        ResolvedBinaryOperatorV1::GreaterEqual => Some(crate::mir::CompareOp::Ge),
                        ResolvedBinaryOperatorV1::Add => None,
                        _ => return Err(freeze("bool-condition/operator-kind")),
                    };
                    let (a, b) = match &append.1 {
                        MirInstruction::Compare { dst, op, lhs, rhs } if *dst == node.value && Some(*op) == expected => (*lhs, *rhs),
                        MirInstruction::BinOp { dst, op: crate::mir::BinaryOp::Add, lhs, rhs }
                            if *dst == node.value && binary.operator() == ResolvedBinaryOperatorV1::Add => (*lhs, *rhs),
                        _ => return Err(freeze("bool-condition/operator-append")),
                    };
                    if operands[0].values() != (lhs, a) || operands[1].values() != (rhs, b) {
                        return Err(freeze("bool-condition/ordered-operands"));
                    }
                    for relation in operands {
                        relation.verify_projected(function, &mut |binding| mapped(binding, finishing))?;
                    }
                    require_binding(function, append, finishing)?;
                },
                (Node::Binary(binary), BoolConditionValueProofV1::Logical { completed, conditions }) => {
                    self.validate_logical(node, binary, completed, conditions, function, finishing)?;
                },
                (Node::Binary(_) | Node::MethodCall, BoolConditionValueProofV1::ScalarInvocation { bindings, input_values,
                    input_reads, entry, terminal, continuation, receiver }) => {
                    if let Some(core) = self.frame.evaluation().cores().get(&node.node) {
                        core.field_map().ok_or_else(|| freeze("expression-core/source-identity"))?
                            .require_receiver_v1(context, function, receiver.as_ref().ok_or_else(|| freeze("expression-core/source-identity"))?)?;
                    } else if receiver.is_some() { return Err(freeze("expression-core/source-identity")); }
                    self.text_comparison_producers_v1(node, input_values, receiver.as_ref(), bindings)?;
                    let inputs = self.frame.evaluation().scalar_invoke_input_order_v1(node.node)
                        .ok_or_else(|| freeze("bool-condition/text-comparison-source"))?;
                    for input in inputs {
                        if !observed_text.insert(input) { return Err(freeze("bool-condition/text-leaf-duplicate")); }
                    }
                    for &input in input_reads.keys() {
                        if !observed_references.insert(input) { return Err(freeze("bool-condition/reference-duplicate")); }
                    }
                    let projection = self.comparison_inputs_v1(node)?;
                    projection.validate_scalar_continuation_v1(context, function, finishing,
                        entry, terminal, continuation, None)?;
                    for binding in bindings.iter() {
                        if mapped(binding, finishing)?.is_none() && matches!(binding.1,
                            MirInstruction::Jump { .. } | MirInstruction::Const { .. } | MirInstruction::Copy { .. }) { continue; }
                        require_binding(function, binding, finishing)?;
                    }
                },
                _ => return Err(freeze("bool-condition/source-proof-kind")),
            }
        }
        if observed_text != text_leaves || observed_references.iter().copied().ne(self.frame.references().keys().copied()) {
            return Err(freeze("bool-condition/reference-leaf-coverage"));
        }
        if observed_calls.into_iter().ne(self.frame.calls().iter().map(|call| call.site())) {
            return Err(freeze("bool-condition/call-coverage"));
        }
        self.materialization.physical().relation().verify_projected(function, &mut |binding| mapped(binding, finishing))?;
        for binding in self.materialization.physical().bindings() {
            match mapped(binding, finishing)? {
                Some(_) => require_binding(function, binding, finishing)?,
                None if matches!(binding.1, MirInstruction::Const { .. } | MirInstruction::Copy { .. }) => {},
                None => return Err(freeze("bool-condition/materialization-producer-removed")),
            }
        }
        Ok(())
    }
    pub(super) fn value_at(&self, site: &SourceExprSiteV1) -> Result<ValueId, String> {
        let index = self.frame.source().nodes().iter().position(|node| node.site() == site)
            .ok_or_else(|| freeze("bool-condition/child-source"))?;
        Ok(self.node(index)?.value)
    }

    pub(super) fn text_comparison_producers_v1<'a>(&self, node: &BoolConditionNodeProjectionV1,
        values: &[Option<EvaluatedExpressionValueV1>], receiver: Option<&crate::mir::builder::ExactLexicalReadV1>,
        bindings: &'a [Binding],
    ) -> Result<(&'a Binding, &'a Binding), String> {
        let operation = super::checked_scalar_invoke_operation_v1(self.frame.evaluation(), node.node, values, receiver)?;
        let mut invokes = bindings.iter().filter(|(_, row)| matches!(row,
            MirInstruction::Invoke { operation: actual, .. } if actual == &operation));
        let invoke = invokes.next().ok_or_else(|| freeze("bool-condition/text-compare-invoke"))?;
        if invokes.next().is_some() { return Err(freeze("bool-condition/text-compare-duplicate")); }
        let MirInstruction::Invoke { normal_landing, .. } = invoke.1 else { unreachable!() };
        let mut results = bindings.iter().filter(|(id, row)| *id == normal_landing && matches!(row,
            MirInstruction::InvokeNormalResult { invoke_block, dst } if *invoke_block == invoke.0 && *dst == node.value));
        let result = results.next().ok_or_else(|| freeze("bool-condition/text-compare-projection"))?;
        if results.next().is_some() || invoke.0 == result.0 { return Err(freeze("bool-condition/text-compare-projection-duplicate")); }
        Ok((invoke, result))
    }
}

impl EmittedBoolConditionProjectionV1 {
    /// The exact source null comparison chooses the surviving successor.
    pub(super) fn validate_nonnull_consumer_v1(
        &self, function: &MirFunction, finishing: Option<&Finished>, consumer: &Binding,
        producer: Option<&Binding>,
    ) -> Result<(), String> {
        let outer = self.outer.borrow();
        let outer = outer.as_ref().ok_or_else(|| freeze("bool-condition/outer-branch-pending"))?;
        self.verify_outer(function, outer, finishing)?;
        require_binding(function, consumer, finishing)?;
        let branch = mapped(outer, finishing)?.ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
        let consumer = mapped(consumer, finishing)?.ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
        let MirInstruction::Branch { then_bb, else_bb, .. } = branch.1
            else { return Err(freeze("bool-condition/outer-branch-source")); };
        let nonnull = match self.frame.source().nodes().first().and_then(|node| node.binary()).map(|binary| binary.operator()) {
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Equal) => else_bb,
            Some(crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::NotEqual) => then_bb,
            _ => return Err(freeze("bool-condition/outer-branch-source")),
        };
        let dom = crate::mir::verification::utils::compute_dominators(function);
        if !dom.dominates(branch.0, consumer.0) || !dom.dominates(nonnull, consumer.0) {
            return Err(freeze("bool-condition/outer-branch-source"));
        }
        if let Some(producer) = producer {
            require_binding(function, producer, finishing)?;
            let producer = mapped(producer, finishing)?.ok_or_else(|| freeze("bool-condition/required-producer-removed"))?;
            if !dom.dominates(producer.0, branch.0) { return Err(freeze("bool-condition/outer-branch-source")); }
        }
        Ok(())
    }
}

impl CallValidationContextV1<'_> {
    /// Source SUM refinement must also pass the SAME finished Bool/Branch owner.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_finite_entry_nonnull_use_v1(
        self, read: &crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
        exclusion: &crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1,
        function: &MirFunction, finishing: &Finished, consumer: &Binding,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
        self.with_local_call_group_tree_v2(read.requirement().binding().owner(), |groups| {
            let mut matches = groups.iter().filter_map(|group| {
                let packet = group.bool_condition()?;
                let guard = exclusion.condition_read_v1(packet.frame.source())?;
                Some((group, packet, guard))
            });
            let (group, packet, guard) = matches.next().ok_or_else(|| freeze("bool-condition/reference-source"))?;
            if matches.next().is_some() || exclusion.root_binding() != read.requirement().formal().1
                || !matches!(packet.materialization, BoolConditionOuterV2::If(_)) {
                return Err(freeze("bool-condition/reference-source"));
            }
            let node = packet.frame.source().nodes().iter().position(|node| node.site() == guard)
                .ok_or_else(|| freeze("bool-condition/reference-source"))?;
            let Some(Reference::FiniteEntry { read: original, exclusion: None }) = packet.frame.references().get(&node)
                else { return Err(freeze("bool-condition/reference-source")); };
            if original.requirement().formal() != read.requirement().formal() || original.null_compare_v1().is_none()
                || !matches!(packet.node(node)?.proof, BoolConditionValueProofV1::OriginalReference(_)) {
                return Err(freeze("bool-condition/reference-source"));
            }
            group.validate_recorded_in_context(self, function, Some(finishing))?;
            packet.validate_nonnull_consumer_v1(function, Some(finishing), consumer, None)
        })
    }
}
