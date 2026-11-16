//! Physical origin of an existing exact i64 read, after its CFG has closed.
//! Source identity and consumer correspondence stay with their existing owners.
use super::*;

impl ExpressionReferenceReadV1 {
    pub(in crate::mir) fn requires_i64_origin_v1(source: &CurrentScalarReadV1) -> bool {
        source.kind() == crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer
            && matches!(
                source.producer(),
                Producer::Call(_) | Producer::Integer { .. }
            )
    }
    pub(in crate::mir) fn source_requires_i64_origin_v1(source: &Reference) -> bool {
        matches!(source, Reference::Scalar(scalar) if scalar.original_read().is_some_and(Self::requires_i64_origin_v1))
    }
    /// Source census, not a new grant: every eligible original read must carry
    /// its existing obligation; a missing flag cannot turn the gate into a no-op.
    pub(in crate::mir) fn visit_i64_origin_v1(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        node: usize,
        source: &Reference,
        visit: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        if !source.matches_node(plan, node)
            || self.deferred_i64 != Self::source_requires_i64_origin_v1(source)
        {
            return Err(freeze("expression/reference-read-census"));
        }
        self.read
            .value_for(
                plan.owner(),
                plan.nodes()[node].site().node(),
                source.binding(),
            )
            .map_err(|error| error.to_string())?;
        if self.deferred_i64 {
            visit(self)?;
        }
        Ok(())
    }
    pub(in crate::mir) fn visit_scalar_i64_origin_v1(
        &self,
        plan: &std::rc::Rc<
            crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
        >,
        source: &CurrentScalarReadV1,
        visit: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.deferred_i64 != Self::requires_i64_origin_v1(source)
            || !plan.nodes().iter().any(|node| node.site() == source.read())
        {
            return Err(freeze("expression/reference-read-census"));
        }
        self.read
            .value_for(plan.owner(), source.read().node(), source.binding())
            .map_err(|error| error.to_string())?;
        if self.deferred_i64 {
            visit(self)?;
        }
        Ok(())
    }
    pub(in crate::mir) fn validate_finished_i64_origin_v1(
        &self,
        analysis: &crate::mir::verification::scalar_origin::FinishedI64OriginsV1<'_, '_>,
        projection: &Finished,
    ) -> Result<(), String> {
        if !self.deferred_i64 {
            return Ok(());
        }
        let geometry = analysis.geometry();
        let expected = self
            .definition()
            .ok_or_else(|| freeze("expression/scalar-producer-kind"))?;
        let value = self.read.source_value().2;
        let original = projection.captured_definition(value)?;
        let Some(current) = projection.relation_binding_with_geometry(geometry, original)? else {
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
        if Self::requires_i64_origin_v1(source) {
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
