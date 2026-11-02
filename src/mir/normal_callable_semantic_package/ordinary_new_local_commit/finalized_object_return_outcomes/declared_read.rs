//! Source-issued readonly value transport retained across final consumers.
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct FinalizedDeclaredReadTransportV1 {
    source: crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1,
    read: crate::mir::builder::BindingValueObservationV2,
    value: ValueId,
    bindings: Vec<Binding>,
}
impl FinalizedDeclaredReadTransportV1 {
    pub(super) fn issue(context: CallValidationContextV1<'_>,
        source: &crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1,
        read: &crate::mir::builder::ExactLexicalReadV1,
        function: &MirFunction, finishing: &super::super::super::physical_boundary::FinishedBindings,
    ) -> Result<Self, String> {
        context.validate_declared_read_transport_v1(source.source(), read.binding_observation_v2(), function, Some(finishing))?;
        let (value, _) = context.declared_result_read_v1(source, read)?;
        let mut original = Vec::new();
        context.append_declared_read_dependencies_v1(source.source(), read.binding_observation_v2(), &mut original)?;
        let mut bindings = Vec::new();
        for row in original {
            let projected = if row.1.dst_value().is_some() { finishing.relation_binding(function, &row)? }
                else { Some(finishing.captured_effect_binding(function, &row)?) };
            if let Some(row) = projected { if !bindings.contains(&row) { bindings.push(row); } }
        }
        Ok(Self { source: source.clone(), read: read.binding_observation_v2().clone(), value, bindings })
    }
    pub(in crate::mir) fn value(&self) -> ValueId { self.value }
    pub(in crate::mir) fn bindings(&self) -> &[Binding] { &self.bindings }
}
