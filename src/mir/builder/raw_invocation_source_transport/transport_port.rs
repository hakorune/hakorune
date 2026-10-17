use super::*;

/// Temporal source scope used only by the selected invocation port.
///
/// The callback executes exactly once. Restoring the parent after it returns
/// is structural recursion bookkeeping, not a retry or route reselection.
pub(in crate::mir::builder) trait RawSourceTransportPortV1 {
    fn with_source_transport_v1<T, R>(
        &mut self,
        transport: RawInvocationSourceTransportV1<T>,
        execute: impl FnOnce(&mut Self, T) -> R,
    ) -> R;

    fn current_source_context_v1(&self) -> Option<RawInvocationSourceContextV1>;

    fn finish_source_body_v1(&mut self, _builder: &mut MirBuilder,
        _source: &RawInvocationSourceContextV1, _end: usize) -> Result<(), String> { Ok(()) }
}

impl RawSourceTransportPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn with_source_transport_v1<T, R>(
        &mut self,
        transport: RawInvocationSourceTransportV1<T>,
        execute: impl FnOnce(&mut Self, T) -> R,
    ) -> R {
        let (node, source) = RawInvocationSourceContextV1::from_transport(transport);
        let parent = self.active_source.replace(source);
        let result = execute(self, node);
        self.active_source = parent;
        result
    }

    fn finish_source_body_v1(&mut self, builder: &mut MirBuilder,
        source: &RawInvocationSourceContextV1, end: usize) -> Result<(), String> {
        if !matches!(source, RawInvocationSourceContextV1::Located {
            site, body_kind: Some(SourceBodyKindV1::Function), ..
        } if site == &crate::mir::resolved_semantics::SourcePathV1::function_body().node()) {
            return Ok(());
        }
        let Some(completion) = crate::mir::builder::normal_callable_binding_materialization_port::CallableBindingMaterializationPortV1::prepare_implicit_callable_end_v1(self)?
            else { return Ok(()); };
        completion.emit_source_body_end_v1(builder, source, end)
    }

    fn current_source_context_v1(&self) -> Option<RawInvocationSourceContextV1> {
        self.active_source.clone()
    }
}
