//! Match every new occurrence to an existing child or a source control writer.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::physical_boundary::{self, FinishedBindings};
use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
use crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1 as Producer;
use crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1 as LoopProducer;
use crate::mir::resolved_semantics::home_new_prefix::SourceLoopDynamicProducerV1 as DynamicProducer;

pub(super) fn validate_owned_v2(
    graph: &EmittedSourceLoopGraphV2,
    owned: &[Binding],
    children: &[RootLifecyclePrefixBindingGroupV1],
    phis: &BTreeMap<BasicBlockId, Box<[MirInstruction]>>,
    function: &MirFunction,
) -> Result<(), String> {
    let mut expected: Vec<Binding> = Vec::new();
    let mut claim = |row: &Binding, distinct_occurrence: bool| -> Result<(), String> {
        let count = owned.iter().filter(|actual| *actual == row).count();
        if count == 0 {
            if !graph.is_prior_binding_v2(row) {
                return Err(freeze("source-loop/foreign-dependency"));
            }
        } else if distinct_occurrence || !expected.contains(row) {
            expected.push(row.clone());
        }
        Ok(())
    };
    for child in children {
        for row in child.bindings() {
            claim(row, matches!(row.1, MirInstruction::KeepAlive { .. }))?;
        }
    }
    for (block, rows) in phis {
        for row in rows.iter() {
            claim(&(*block, row.clone()), false)?;
        }
    }
    claim(graph.result.original(), false)?;
    for frame in &graph.frames {
        let mut dependencies = Vec::new();
        frame
            .canonical
            .append_observed_bindings_v1(&mut dependencies);
        for row in &dependencies {
            let current = frame
                .canonical
                .resolve_observed_binding_v1(row, true)?
                .unwrap_or_else(|| row.clone());
            claim(&current, false)?;
        }
        if let (Some(condition), Some((yes, no))) = (&frame.condition, frame.targets) {
            for row in condition.bindings() {
                claim(row, false)?;
            }
            claim(
                &(
                    condition.block(),
                    MirInstruction::Branch {
                        condition: condition.values().1,
                        then_bb: yes,
                        else_bb: no,
                        then_edge_args: Some(crate::mir::EdgeArgs {
                            layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                            values: vec![],
                        }),
                        else_edge_args: Some(crate::mir::EdgeArgs {
                            layout: crate::mir::edge_args::JumpArgsLayout::CarriersOnly,
                            values: vec![],
                        }),
                    },
                ),
                false,
            )?;
        }
    }
    for effect in &graph.effects {
        if let Some((_, relation)) = &effect.result {
            let mut dependencies = Vec::new();
            relation.append_observed_bindings_v1(&mut dependencies);
            for row in &dependencies {
                let current = relation
                    .resolve_observed_binding_v1(row, true)?
                    .unwrap_or_else(|| row.clone());
                claim(&current, false)?;
            }
        }
    }
    for returned in &graph.returns {
        for row in &returned.bindings {
            claim(row, false)?;
        }
        if let Some(read) = &returned.read {
            let dependencies = ExpressionReferenceReadV1::without_borrowed_phis_v2(
                read.dependencies()?.collect(),
                std::iter::once(read),
            )?;
            for row in &dependencies {
                claim(row, false)?;
            }
        }
    }
    for edge in &graph.edges {
        if let Some(cleanup) = &edge.cleanup {
            for row in &cleanup.bindings {
                claim(row, false)?;
            }
        }
        for value in &edge.values {
            for relation in std::iter::once(&value.localized).chain(value.canonical.iter()) {
                let mut dependencies = Vec::new();
                relation.append_observed_bindings_v1(&mut dependencies);
                for row in &dependencies {
                    let current = relation
                        .resolve_observed_binding_v1(row, true)?
                        .unwrap_or_else(|| row.clone());
                    claim(&current, false)?;
                }
            }
        }
        claim(
            &(
                edge.predecessor,
                MirInstruction::Jump {
                    target: edge.target,
                    edge_args: None,
                },
            ),
            false,
        )?;
    }
    for fork in &graph.forks {
        for point in &fork.joins {
            claim(point.relation().identity_phi_binding_v1()?, false)?;
        }
        for point in &fork.text_joins {
            claim(point.relation().identity_phi_binding_v1()?, false)?;
        }
        if let Some(join) = fork.join {
            for (end, normal) in [(fork.ends.0, fork.normal.0), (fork.ends.1, fork.normal.1)] {
                if normal {
                    claim(
                        &(
                            end,
                            MirInstruction::Jump {
                                target: join,
                                edge_args: None,
                            },
                        ),
                        false,
                    )?;
                }
            }
        }
    }
    for block in &graph.blocks {
        if let Some((index, row)) = block.scope_close.occurrence() {
            if function
                .blocks
                .get(&row.0)
                .and_then(|block| block.instructions.get(index))
                != Some(&row.1)
            {
                return Err(freeze("source-loop/scope-occurrence"));
            }
            claim(row, true)?;
        }
    }
    if expected.len() != owned.len()
        || expected.iter().any(|row| {
            expected.iter().filter(|actual| *actual == row).count()
                != owned.iter().filter(|actual| *actual == row).count()
        })
    {
        return Err(freeze("source-loop/unexplained-physical-write"));
    }
    Ok(())
}

impl SourceLoopPhysicalProofV2 {
    /// Lend only producers already required by this same completed proof.
    /// DCE liveness is not a second source/ownership decision.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn source_dependency_bindings_v2(
        &self,
    ) -> Result<Vec<Binding>, String> {
        let mut output = Vec::new();
        for returned in &self.graph.returns {
            if let Some(read) = &returned.read {
                output.extend(read.dependencies()?);
            }
        }
        // These exact join producers are consumed again by validate_v2 below,
        // including an After value with no later source read. Lend the same
        // requirements to finishing liveness instead of allowing DCE to remove
        // a producer while the independent proof still requires it.
        for (block, phis) in self.points.phis() {
            output.extend(phis.iter().map(|phi| (*block, phi.clone())));
        }
        for point in self.graph.forks.iter().flat_map(|fork| &fork.joins) {
            point.relation().append_observed_bindings_v1(&mut output);
        }
        for edge in &self.graph.edges {
            for value in &edge.values {
                let relations: Vec<_> = std::iter::once(&value.localized)
                    .chain(value.canonical.iter())
                    .collect();
                let mut dependencies = value.dependencies.clone();
                for relation in &relations {
                    relation.append_observed_bindings_v1(&mut dependencies);
                }
                for dependency in dependencies {
                    let mut current = dependency;
                    for relation in &relations {
                        if let Some(resolved) =
                            relation.resolve_observed_binding_v1(&current, true)?
                        {
                            current = resolved;
                        }
                    }
                    output.push(current);
                }
            }
        }
        for relation in self
            .graph
            .frames
            .iter()
            .map(|frame| &frame.canonical)
            .chain(
                self.graph
                    .effects
                    .iter()
                    .filter_map(|effect| effect.result.as_ref().map(|(_, relation)| relation)),
            )
            .chain(
                self.graph
                    .edges
                    .iter()
                    .filter_map(|edge| edge.cleanup.as_ref())
                    .flat_map(|cleanup| cleanup.operations.iter())
                    .flat_map(|operation| operation.operands.iter()),
            )
        {
            let mut dependencies = Vec::new();
            relation.append_observed_bindings_v1(&mut dependencies);
            for dependency in dependencies {
                output.push(
                    relation
                        .resolve_observed_binding_v1(&dependency, true)?
                        .unwrap_or(dependency),
                );
            }
        }
        Ok(output)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_v2(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        observed: &[Binding],
        coordinates: Option<&FinishedBindings>,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        context.require_source_loop_parent_v2(&self.graph.product, true)?;
        let expected = match coordinates {
            Some(projection) => projection.bindings(&self.owned)?,
            None => self.owned.clone(),
        };
        if observed != expected {
            return Err(freeze("source-loop/aggregate-correspondence"));
        }
        if finishing.is_none() {
            self.graph.verify_control_v2(function)?;
            for row in &self.owned {
                if !physical_boundary::check_binding(function, None, row.0, &row.1)? {
                    return Err(freeze("source-loop/original-write"));
                }
            }
        }
        if let (CallValidationContextV1::Finalized(_), Some(projection)) = (context, finishing) {
            for row in &self.owned {
                match &row.1 {
                    MirInstruction::Jump {
                        edge_args: None, ..
                    } => {
                        projection.structural_jump_binding(function, row, true)?;
                    }
                    MirInstruction::KeepAlive { .. } => {
                        projection.validate_metadata_correspondence_v2(function, row, true)?;
                    }
                    _ if row.1.dst_value().is_some() => {
                        projection.relation_binding(function, row)?;
                    }
                    _ => {
                        projection.required_effect_binding(function, row)?;
                    }
                }
            }
        }
        for relation in self
            .graph
            .effects
            .iter()
            .filter_map(|effect| effect.result.as_ref().map(|(_, relation)| relation))
        {
            relation.require_completed_v1()?;
            match finishing {
                Some(projection) => relation.verify_projected(function, &mut |row| {
                    projection.relation_binding(function, row)
                })?,
                None => relation.verify(function)?,
            }
        }
        let mut phi_rows = Vec::new();
        for (id, phis) in self.points.phis() {
            for phi in phis.iter() {
                let row = match finishing {
                    Some(projection) => projection
                        .relation_binding(function, &(*id, phi.clone()))?
                        .ok_or_else(|| freeze("source-loop/phi-removed"))?,
                    None => (*id, phi.clone()),
                };
                let MirInstruction::Phi { dst, inputs, .. } = row.1 else {
                    return Err(freeze("source-loop/phi-kind"));
                };
                phi_rows.push((row.0, dst, inputs));
            }
        }
        crate::mir::builder::capture_exact_loop_phis_v2(function, &phi_rows)?;
        for returned in &self.graph.returns {
            let returned_value_matches = match (&returned.source.read, &returned.read) {
                (Some(source_read), Some(read)) => read.validate_scalar_read_v2(
                    &returned.source.source,
                    source_read,
                    context,
                    function,
                    finishing,
                )? == returned.value,
                (None, None) => {
                    let literal = returned.source.integer_literal_value_v1()
                        .ok_or_else(|| freeze("source-loop/frame-source"))?;
                    let dominators = crate::mir::verification::utils::compute_dominators(function);
                    let mut constants = function.blocks.iter().flat_map(|(block_id, block)| {
                        block.instructions.iter().filter_map(move |instruction| match instruction {
                            MirInstruction::Const { dst, value: crate::mir::ConstValue::Integer(value) }
                                if *dst == returned.value && *value == literal => Some(*block_id),
                            _ => None,
                        })
                    });
                    constants.next().is_some_and(|block| {
                        constants.next().is_none()
                            && dominators.dominates(block, returned.terminal.0)
                    })
                }
                _ => false,
            };
            if context
                .ledger()
                .source_loop_return_bindings_v2(&self.graph.product, &returned.source)?
                != returned.bindings
                || !returned_value_matches
            {
                return Err(freeze("source-loop/frame-source"));
            }
            let terminal = &returned.terminal;
            if terminal.1
                != (MirInstruction::Return {
                    value: Some(returned.value),
                })
                || returned
                    .bindings
                    .iter()
                    .filter(|row| *row == terminal)
                    .count()
                    != 1
            {
                return Err(freeze("source-loop/original-write"));
            }
            for binding in &returned.bindings {
                match finishing {
                    Some(projection) => match &binding.1 {
                        MirInstruction::Jump {
                            edge_args: None, ..
                        } => {
                            projection.structural_jump_binding(
                                function,
                                binding,
                                matches!(context, CallValidationContextV1::Finalized(_)),
                            )?;
                        }
                        row if row.dst_value().is_some() => {
                            projection.relation_binding(function, binding)?;
                        }
                        _ => {
                            projection.required_effect_binding(function, binding)?;
                        }
                    },
                    None if !physical_boundary::check_binding(
                        function, None, binding.0, &binding.1,
                    )? =>
                    {
                        return Err(freeze("source-loop/original-write"));
                    }
                    None => {}
                }
            }
        }
        for edge in &self.graph.edges {
            if let Some(cleanup) = &edge.cleanup {
                cleanup
                    .plan
                    .validate_emitted_v2(cleanup, context, function, finishing)?;
            }
            let jump = (
                edge.predecessor,
                MirInstruction::Jump {
                    target: edge.target,
                    edge_args: None,
                },
            );
            if let Some(projection) = finishing {
                projection.structural_jump_binding(
                    function,
                    &jump,
                    matches!(context, CallValidationContextV1::Finalized(_)),
                )?;
            }
            let original = self
                .graph
                .product
                .source
                .incoming
                .as_ref()
                .map_err(|_| freeze("source-loop/source-incoming"))?;
            for value in &edge.values {
                self.graph.product.validate_incoming_value_v2(
                    &edge.source,
                    value.binding,
                    value.logical,
                )?;
                let source = original
                    .iter()
                    .find(|row| row.basis.binding() == value.binding)
                    .and_then(|row| {
                        row.header_incoming
                            .iter()
                            .chain(row.after_incoming.iter())
                            .find(|row| row.edge == edge.source)
                    })
                    .ok_or_else(|| freeze("source-loop/source-carrier"))?;
                if value.localized.values().0 != value.observation.binding_value().1 {
                    return Err(freeze("source-loop/incoming-observation"));
                }
                for relation in std::iter::once(&value.localized).chain(value.canonical.iter()) {
                    relation.require_completed_v1()?;
                    match finishing {
                        Some(projection) => relation.verify_projected(function, &mut |row| {
                            projection.relation_binding(function, row)
                        })?,
                        None => relation.verify(function)?,
                    }
                }
                for dependency in &value.dependencies {
                    let resolved = match &source.producer {
                        LoopProducer::Scalar(producer @ (Producer::LoopHeader(_) | Producer::LoopAfter { .. })) =>
                            value.observation.loan_scalar_join_v1(producer)?.relation()
                                .resolve_observed_binding_v1(dependency, true)?.unwrap_or_else(|| dependency.clone()),
                        LoopProducer::Text(producer @ (crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::Header(_)
                            | crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1::After { .. })) =>
                            value.observation.loan_text_join_v1(producer)?.relation()
                                .resolve_observed_binding_v1(dependency, true)?.unwrap_or_else(|| dependency.clone()),
                        LoopProducer::Dynamic(producer @ (DynamicProducer::Header(_)
                            | DynamicProducer::After { .. })) =>
                            value.observation.loan_dynamic_join_v1(producer)?.relation()
                                .resolve_observed_binding_v1(dependency, true)?.unwrap_or_else(|| dependency.clone()),
                        _ => dependency.clone(),
                    };
                    match finishing {
                        Some(projection) => {
                            projection.relation_binding(function, &resolved)?;
                        }
                        None if !physical_boundary::check_binding(
                            function,
                            None,
                            resolved.0,
                            &resolved.1,
                        )? =>
                        {
                            return Err(freeze("source-loop/source-dependency"));
                        }
                        None => {}
                    }
                }
            }
        }
        for fork in &self.graph.forks {
            for point in &fork.joins {
                if !Rc::ptr_eq(point.pending(), &fork.condition_materialization)
                    || self
                        .graph
                        .product
                        .schedule()
                        .frames()
                        .get(fork.condition)
                        .is_none_or(|frame| frame.prefix().statement() != point.source().site())
                {
                    return Err(freeze("source-loop/frame-source"));
                }
                point.validate(context, function, finishing)?;
            }
            if let (Some(join), Some(projection)) = (fork.join, finishing) {
                for (end, normal) in [(fork.ends.0, fork.normal.0), (fork.ends.1, fork.normal.1)] {
                    if normal {
                        projection.structural_jump_binding(
                            function,
                            &(
                                end,
                                MirInstruction::Jump {
                                    target: join,
                                    edge_args: None,
                                },
                            ),
                            matches!(context, CallValidationContextV1::Finalized(_)),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}
