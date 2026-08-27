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

    fn current_source_context_v1(&self) -> Option<RawInvocationSourceContextV1> {
        self.active_source.clone()
    }
}
