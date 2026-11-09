//! Original scalar and Text binding provenance, shared by source consumers.
use super::*;

impl ExpressionReferenceReadV1 {
    /// The SAME scalar producer/provenance check for a loop's original Entry.
    /// No expression frame or read site is synthesized or consumed.
    pub(in crate::mir) fn validate_scalar_binding_v2(
        binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &Producer,
        read: &BindingValueObservationV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        let producer = prepare_scalar_binding_producer(binding, source, read, context, function)?;
        let mut dependencies = Vec::new();
        match producer {
            ReferenceProducerV1::Definition(row) => dependencies.push(row),
            ReferenceProducerV1::Phi(point) => point.append_observed_bindings_v1(&mut dependencies),
            ReferenceProducerV1::Entry(_) => {}
        }
        dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        Ok(dependencies)
    }

    /// The writer-owned Phi relation behind a scalar edge producer, if any.
    /// Later validation resolves provisional Phi dependencies through THIS
    /// relation, never by searching the final MIR for a matching Phi.
    pub(in crate::mir) fn scalar_binding_join_v2(
        binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &Producer,
        read: &BindingValueObservationV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Option<std::rc::Rc<crate::mir::builder::LocalSsaPhysicalRelationV1>>, String> {
        Ok(
            match prepare_scalar_binding_producer(binding, source, read, context, function)? {
                ReferenceProducerV1::Phi(point) => Some(point),
                _ => None,
            },
        )
    }

    /// Validate the existing Text producer at a source Loop edge. Static and
    /// Owned keep their original physical producer; Header/After require the
    /// exact prior typed descriptor PHI. No source class is inferred from MIR.
    pub(in crate::mir) fn validate_text_binding_v2(
        binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1,
        read: &BindingValueObservationV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        Self::validate_text_binding_with_finishing_v2(
            binding, source, read, context, function, None,
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_text_binding_with_finishing_v2(
        binding: crate::mir::resolved_semantics::BindingRefV1,
        source: &crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1,
        read: &BindingValueObservationV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<Vec<Binding>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1 as Source;
        if read.binding_value().0 != binding {
            return Err(freeze("source-loop/text-binding"));
        }
        let mut dependencies = Vec::new();
        match source {
            Source::Static {
                binding: source_binding,
                utf8,
                ..
            } => {
                if *source_binding != binding {
                    return Err(freeze("source-loop/text-static-binding"));
                }
                let value = read.loan_local_initializer_v1(binding)?.0;
                let mut rows = function
                    .blocks
                    .iter()
                    .flat_map(|(block, body)| body.all_instructions().map(move |row| (*block, row)))
                    .filter(|(_, row)| row.dst_value() == Some(value));
                let (block, row) = rows
                    .next()
                    .ok_or_else(|| freeze("source-loop/text-static-definition"))?;
                if rows.next().is_some()
                    || function.params.contains(&value)
                    || !matches!(row, MirInstruction::Const { dst, value: ConstValue::String(text) }
                        if *dst == value && text == utf8.as_ref())
                {
                    return Err(freeze("source-loop/text-static-value"));
                }
                dependencies.push((block, row.clone()));
            }
            Source::Residence(acquisition) => {
                if acquisition.binding() != binding || !acquisition.is_text_residence() {
                    return Err(freeze("source-loop/text-residence-binding"));
                }
                context.with_text_residence_binding_v1(acquisition, read, |value, original| {
                    if original.1.dst_value() != Some(value) {
                        return Err(freeze("source-loop/text-residence-producer"));
                    }
                    dependencies.push(original.clone());
                    Ok(())
                })?;
            }
            Source::Header(_) | Source::After { .. } => {
                let product = context.source_loop_text_point_product_v2(source)?;
                if product.source.headers.get(&binding).is_none_or(|basis|
                    !matches!(basis.entry_producer(), crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Text(_))) {
                    return Err(freeze("source-loop/text-point-binding"));
                }
                let point = read.loan_text_join_v1(source)?;
                if let Some(finishing) = finishing {
                    point.relation().require_completed_v1()?;
                    point.relation().verify_projected(function, &mut |row| {
                        finishing.relation_binding(function, row)
                    })?;
                } else {
                    point.relation().verify(function)?;
                }
                dependencies.push(point.relation().identity_phi_binding_v1()?.clone());
            }
            Source::BranchNormal(source) => {
                context.ledger().require_branch_text_source_v1(source)?;
                let point =
                    read.loan_text_branch_v1(&Source::BranchNormal(std::rc::Rc::clone(source)))?;
                point.validate(context, function, finishing)?;
                dependencies.push(point.relation().identity_phi_binding_v1()?.clone());
            }
        }
        dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        for row in &dependencies {
            match finishing {
                Some(finishing) => {
                    finishing.relation_binding(function, row)?;
                }
                None if function.blocks.get(&row.0).is_none_or(|body| {
                    body.all_instructions().filter(|old| **old == row.1).count() != 1
                }) =>
                {
                    return Err(freeze("expression/reference-dependency-missing"));
                }
                None => {}
            }
        }
        Ok(dependencies)
    }
}

#[derive(Debug)]
pub(in crate::mir) struct ScalarBindingReadProofV1 {
    source: CurrentScalarReadV1,
    producer: ReferenceProducerV1,
}
impl ScalarBindingReadProofV1 {
    pub(in crate::mir) fn prepare(
        source: CurrentScalarReadV1,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Self, String> {
        read.value_for(
            source.binding().owner(),
            source.read().node(),
            source.binding(),
        )
        .map_err(|error| error.to_string())?;
        let producer = prepare_scalar_binding_producer(
            source.binding(),
            source.producer(),
            read.binding_observation_v2(),
            context,
            function,
        )?;
        Ok(Self { source, producer })
    }
    pub(in crate::mir) fn source(&self) -> &CurrentScalarReadV1 {
        &self.source
    }
    pub(in crate::mir) fn value(&self, read: &ExactLexicalReadV1) -> Result<ValueId, String> {
        read.value_for(
            self.source.binding().owner(),
            self.source.read().node(),
            self.source.binding(),
        )
        .map_err(|error| error.to_string())
    }
    pub(in crate::mir) fn validate(
        &self,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<(), String> {
        self.value(read)?;
        validate_scalar_binding_observation_v1(
            self.source.binding(),
            self.source.producer(),
            read.binding_observation_v2(),
            &self.producer,
            context,
            function,
            finishing,
        )
    }
    pub(in crate::mir) fn dependencies(
        &self,
        read: &ExactLexicalReadV1,
    ) -> Result<Vec<Binding>, String> {
        self.value(read)?;
        let mut dependencies = Vec::new();
        match &self.producer {
            ReferenceProducerV1::Definition(row) => dependencies.push(row.clone()),
            ReferenceProducerV1::Phi(point) => point.append_observed_bindings_v1(&mut dependencies),
            ReferenceProducerV1::Entry(_) => {}
        }
        dependencies.extend_from_slice(read.original_copy_dependencies_v1()?);
        Ok(dependencies)
    }
}
