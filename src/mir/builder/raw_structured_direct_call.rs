//! Forward the existing direct-call capability through structured child scopes.
use super::*;

impl<Port> DirectCallDispositionPortV1 for RawStructuredChildScopePortV1<'_, Port>
where
    Port: DirectCallDispositionPortV1,
{
    fn take_direct_call_disposition_v1(
        &mut self,
    ) -> Result<crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1, String>
    {
        self.child.take_direct_call_disposition_v1()
    }

    fn has_static_execution_for_owner_v1(&self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1) -> bool {
        self.child.has_static_execution_for_owner_v1(owner)
    }
    fn take_optional_static_execution_row_v1(&mut self) -> Result<Option<crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1>, String> {
        self.child.take_optional_static_execution_row_v1()
    }

    fn emit_static_argument_expression_v1(&mut self, builder: &mut MirBuilder,
        call: &crate::mir::resolved_semantics::OwnedExprSiteV1, ordinal: u32, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<(ValueId, crate::mir::normal_callable_semantic_package::LexicalCallArgumentProjectionV1), String> {
        self.child.emit_static_argument_expression_v1(builder, call, ordinal, site)
    }
    fn static_execution_argument_source_v1(&self, row: &crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1)
        -> Result<crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1, String> {
        self.child.static_execution_argument_source_v1(row)
    }
    fn take_exact_static_argument_read_v1(&mut self,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<crate::mir::builder::ExactLexicalReadV1, String> {
        self.child.take_exact_static_argument_read_v1(site, binding)
    }

    fn prepare_source_required_static_scalar_call_v1(&self) -> Result<Option<std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>>, String> {
        self.child.prepare_source_required_static_scalar_call_v1()
    }
    fn record_source_required_static_scalar_call_v1(&mut self, builder: &mut crate::mir::MirBuilder,
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>, observations: Vec<(u32, crate::mir::resolved_semantics::SourceExprSiteV1, crate::mir::ValueId)>,
        binding: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    ) -> Result<(), String> {
        self.child.record_source_required_static_scalar_call_v1(builder, source, observations, binding)
    }

    fn record_scalar_direct_call_v1(&mut self, builder: &mut crate::mir::MirBuilder,
        row: crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1,
        arguments: Vec<crate::mir::ValueId>, binding: (crate::mir::BasicBlockId, crate::mir::MirInstruction),
    ) -> Result<(), String> {
        self.child.record_scalar_direct_call_v1(builder, row, arguments, binding)
    }

    fn emit_local_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        row: crate::mir::normal_callable_semantic_package::DirectCallDispositionRowV1,
        arguments: Vec<ValueId>,
    ) -> Result<Option<ValueId>, String> {
        self.child
            .emit_local_lifecycle_call_v1(builder, row, arguments)
    }

    fn emit_local_static_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        arity: usize,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_local_static_lifecycle_call_v1(builder, owner, method, arity)
    }

    fn emit_local_lexical_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        method: &str,
    ) -> Result<Option<ValueId>, String> {
        self.child.emit_local_lexical_lifecycle_call_v1(builder, method)
    }

    fn emit_receiver_nullable_lifecycle_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
        read: Option<crate::mir::builder::ExactLexicalReadV1>,
    ) -> Result<Option<ValueId>, String> {
        self.child
            .emit_receiver_nullable_lifecycle_call_v1(builder, key, receiver, read)
    }

    fn validate_current_call_argument_site_v1(
        &self,
        expected: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<(), String> {
        self.child.validate_current_call_argument_site_v1(expected)
    }
}
