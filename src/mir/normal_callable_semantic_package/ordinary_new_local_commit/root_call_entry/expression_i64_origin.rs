//! Physical origin of an existing exact i64 read, after its CFG has closed.
//! Source identity and consumer correspondence stay with their existing owners.
use super::*;

impl ExpressionReferenceReadV1 {
    pub(super) fn validate_i64_origin_in_module_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        projection: &Finished,
    ) -> Result<(), String> {
        if !self.deferred_i64 {
            return Ok(());
        }
        let module = context
            .module()
            .ok_or_else(|| freeze("local-read/initializer-missing"))?;
        let validate =
            |analysis: &crate::mir::verification::scalar_origin::FinishedI64OriginsV1<'_, '_>| {
                let geometry = analysis.geometry();
                if !std::ptr::eq(geometry.as_function(), function)
                    || analysis
                        .module()
                        .is_none_or(|actual| !std::ptr::eq(actual, module))
                {
                    return Err(freeze("local-read/initializer-missing"));
                }
                let expected = self
                    .definition()
                    .ok_or_else(|| freeze("expression/scalar-producer-kind"))?;
                // Original membership and the actual consumer still belong to the
                // existing projection. The shared index is only a lookup mechanism.
                let value = self.read.source_value().2;
                let original = projection.captured_definition(value)?;
                let Some(current) =
                    projection.relation_binding_with_geometry(geometry, original)?
                else {
                    return Ok(()); // existing checked no-definition/no-use omission
                };
                let current = current
                    .1
                    .dst_value()
                    .ok_or_else(|| freeze("expression/reference-dependency-dst"))?;
                let expected = projection
                    .relation_binding_with_geometry(geometry, expected)?
                    .and_then(|row| row.1.dst_value())
                    .ok_or_else(|| freeze("expression/reference-dependency-missing"))?;
                use crate::mir::verification::scalar_origin::SameI64Definition;
                match analysis.same_definition(current, expected) {
                    Ok(SameI64Definition::Same) => Ok(()),
                    _ => Err("[freeze:contract][local-read/initializer-missing]".into()),
                }
            };
        match context.i64_origins() {
            Some(analysis) => validate(analysis),
            None => context
                .ledger()
                .with_function_geometry_v1(function, |geometry| {
                    validate(
                        &crate::mir::verification::scalar_origin::FinishedI64OriginsV1::new(
                            geometry,
                            Some(module),
                        ),
                    )
                }),
        }
    }
}

impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn prepare_scalar_read_v2(
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        read: &ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<PreparedExpressionReferenceReadV1, String> {
        if source.kind()
            == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer
            && matches!(
                source.producer(),
                Producer::Call(_) | Producer::Integer { .. }
            )
        {
            let expected =
                original_i64_read_value(source.producer(), read.binding_observation_v2(), context)?;
            let producer = definition_for_initializer(expected, function, context)?;
            validate_i64_read_source(plan, source, read, &producer, context)?;
            return Ok(PreparedExpressionReferenceReadV1 {
                producer,
                deferred_i64: true,
            });
        }
        let producer = prepare_scalar_binding_producer(
            source.binding(),
            source.producer(),
            read.binding_observation_v2(),
            context,
            function,
        )?;
        validate_source(plan, source, read, &producer, context)?;
        Ok(PreparedExpressionReferenceReadV1 {
            producer,
            deferred_i64: false,
        })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_scalar_read_v2(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&Finished>,
    ) -> Result<ValueId, String> {
        if self.deferred_i64 {
            let value =
                validate_i64_read_source(plan, source, &self.read, &self.producer, context)?;
            if let (Some(_), Some(projection)) = (context.module(), finishing) {
                self.validate_i64_origin_in_module_v1(context, function, projection)?;
            }
            return Ok(value);
        }
        let value = validate_source(plan, source, &self.read, &self.producer, context)?;
        validate_scalar_binding_observation_v1(
            source.binding(),
            source.producer(),
            self.read.binding_observation_v2(),
            &self.producer,
            context,
            function,
            finishing,
        )?;
        Ok(value)
    }
}
