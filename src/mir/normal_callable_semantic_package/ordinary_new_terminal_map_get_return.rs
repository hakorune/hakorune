//! Source-backed Map.get acquisition and terminal consumption.
//! Shared payload facts qualify local gets independently of Return producers.
//! Borrowed Return results still require their own complete source witnesses.
//! Physical consumers resolve already-checked operands; no source rewrite,
//! Recipe key, owning Home or fallback is issued here.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1, SourceStmtSiteV1};
use crate::mir::ValueId;

/// One resolved terminal map-get: the call site is retained source
/// evidence, `map` is the receiver binding's existing physical value, and
/// `utf8` is the sealed literal key carried inline like a prepared key.
pub(crate) struct PreparedTerminalMapGetReturnV1 {
    pub(crate) return_site: SourceStmtSiteV1,
    pub(crate) site: OwnedExprSiteV1,
    pub(crate) map: ValueId,
    pub(crate) utf8: String,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_map_get_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &SourceNodeSiteV1,
        mut resolve_binding: impl FnMut(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<Option<PreparedTerminalMapGetReturnV1>, String> {
        let stmt_site = SourceStmtSiteV1::from_node(return_site.clone());
        let Some(relation) = self.terminal_map_get_return_for_owner_at(owner, &stmt_site) else {
            return Ok(None);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(fault("completion-missing"));
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site)
        {
            return Err(fault("source-drift"));
        }
        let map = resolve_binding(relation.receiver(), relation.receiver_site().node())?;
        Ok(Some(PreparedTerminalMapGetReturnV1 {
            return_site: stmt_site,
            site: relation.call_site().clone(),
            map,
            utf8: relation.key().into(),
        }))
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-map-get/{reason}]")
}

/// One get from the shared whole-field source, with THIS function's entry
/// receiver. The producer's key ordinal and receiver are never substituted.
#[derive(Debug, Clone)]
pub(crate) struct StoredMapGetSourceLoanV1 {
    payload: std::rc::Rc<super::container_field_uses::MapFieldPayloadSourceV1>,
    result: Option<std::rc::Rc<super::MapStoredResultSourceV1>>,
    get: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    receiver: std::rc::Rc<super::super::core_method_source::FieldMapReceiverSourceV1>,
}
impl PartialEq for StoredMapGetSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.payload.same_source_v1(&other.payload)
            && match (&self.result, &other.result) {
                (Some(a), Some(b)) => std::rc::Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
            && std::sync::Arc::ptr_eq(&self.get, &other.get)
            && self.receiver.matches_source(&other.receiver)
    }
}
impl Eq for StoredMapGetSourceLoanV1 {}
impl StoredMapGetSourceLoanV1 {
    pub(crate) fn payload_source_v1(&self) -> &std::rc::Rc<super::container_field_uses::MapFieldPayloadSourceV1> { &self.payload }
    pub(crate) fn result_source_v1(&self) -> Option<&std::rc::Rc<super::MapStoredResultSourceV1>> { self.result.as_ref() }
    pub(crate) fn get(&self) -> &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1> { &self.get }
    pub(crate) fn receiver(&self) -> &super::super::core_method_source::FieldMapReceiverSourceV1 { &self.receiver }
    pub(in crate::mir::normal_callable_semantic_package) fn issue_for_site_v1(
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        at: &OwnedExprSiteV1,
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        selected: &super::super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        constructors: &super::super::VerifiedInstanceConstructorSemanticBatchV1,
        receiver: Option<(crate::mir::resolved_semantics::BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
        entry: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
        classes: &super::result_class_claim::OrdinaryNewResultClassClaimsV1,
        fields: &super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    ) -> Result<Option<Self>, String> {
        if at.owner() != input.owner() { return Err(fault("source-drift")); }
        let Some(original) = input.function().method_call_source_v1(at.site()) else { return Ok(None) };
        if original.selector() != "get" || original.arity() != 1 { return Ok(None); }
        let result = classes.map_stored_get_source_v1(at).map_err(fault)?;
        let (Some(receiver), Some(entry)) = (receiver, entry) else {
            return if result.is_some() { Err(fault("source-drift")) } else { Ok(None) };
        };
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) = selected.key_for_batch_slot(entry.batch_slot())
            else { return Err(fault("source-drift")) };
        let payload = if let Some(result) = &result {
            std::rc::Rc::clone(result.payload_source_v1())
        } else {
            use crate::mir::resolved_semantics::BodyExpressionShapeV1 as Shape;
            let Some(Shape::FieldAccess { object, field, .. }) = input.body_shape()
                .and_then(|shape| shape.expression_shape(original.receiver_site())) else { return Ok(None); };
            if !matches!(input.function().variable_ref(object),
                Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if binding == receiver.0)
                || !matches!(input.body_shape().and_then(|shape| shape.expression_shape(object)),
                    Some(Shape::Me { receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), .. }) if *binding == receiver.0)
                || key.namespace() != hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod { return Ok(None); }
            let Some(payload) = super::container_field_uses::MapFieldPayloadSourceV1::issue(
                batch, selected, constructors, fields, key.owner(), field) else { return Ok(None) };
            std::rc::Rc::new(payload)
        };
        let get = payload.original_get_at_v1(at).ok_or_else(|| fault("source-drift"))?;
        if entry.owner() != input.owner() || !std::sync::Arc::ptr_eq(original, get) { return Err(fault("source-drift")); }
        let receiver = super::super::core_method_source::FieldMapReceiverSourceV1::issue(
            input, key, get, constructors, fields, receiver, entry)?;
        if receiver.field() != payload.source.field() || !payload.source.same_inventory(fields)
            || result.as_ref().is_some_and(|source| !receiver.same_field_source_v1(source.receiver_v1())) {
            return Err(fault("source-drift"));
        }
        Ok(Some(Self { get: std::sync::Arc::clone(get), payload, result, receiver: std::rc::Rc::new(receiver) }))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_v1(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let at = OwnedExprSiteV1::new(self.get.owner(), self.get.site().clone());
        if !self.payload.source.same_inventory(&ledger.field_write_claims)
            || !self.payload.original_get_at_v1(&at).is_some_and(|get| std::sync::Arc::ptr_eq(get, &self.get))
            || self.receiver.field() != self.payload.source.field()
            || !self.receiver.same_source(&ledger.field_write_claims) { return Err(fault("source-drift")); }
        if let Some(result) = &self.result {
            let retained = ledger.callable_result_classes.map_stored_get_source_v1(&at).map_err(fault)?
                .ok_or_else(|| fault("source-drift"))?;
            if !std::rc::Rc::ptr_eq(&retained, result)
                || !std::rc::Rc::ptr_eq(retained.payload_source_v1(), &self.payload)
                || !self.receiver.same_field_source_v1(retained.receiver_v1()) { return Err(fault("source-drift")); }
        }
        self.receiver.require_entry_source_v1(ledger)
    }
}

/// The original stored-object Return, with its independent get receiver use.
/// This source loan owns neither a payload Home nor a physical result ABI.
#[derive(Debug, Clone)]
pub(crate) struct StoredMapGetReturnSourceLoanV1 {
    terminal: crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
}
impl StoredMapGetReturnSourceLoanV1 {
    pub(crate) fn terminal(&self) -> &crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1 { &self.terminal }
    pub(crate) fn parts(&self) -> (&std::rc::Rc<super::MapStoredResultSourceV1>,
        &std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
        &crate::mir::resolved_semantics::SourceExprSiteV1) {
        let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read } = self.terminal.returned()
            else { unreachable!("source-issued stored get loan") };
        (source.result_source_v1().expect("verified stored Return source"), key, receiver_read)
    }
    pub(crate) fn get_source(&self) -> &StoredMapGetSourceLoanV1 {
        let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::StoredMapGet { source, .. } = self.terminal.returned()
            else { unreachable!("source-issued stored get loan") };
        source
    }

}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn stored_map_get_return_source_v1(&self, owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<StoredMapGetReturnSourceLoanV1>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalReturnedSourceV1};
        let Some(TerminalRelationV1::Value(terminal)) = self.terminal_relation_for_owner_at(owner, exit)
            else { return Ok(None) };
        let TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read } = terminal.returned()
            else { return Ok(None) };
        let site = OwnedExprSiteV1::new(owner, terminal.value_site().clone());
        let retained = self.callable_result_classes.map_stored_source_v1(&site).map_err(fault)?
            .ok_or_else(|| fault("source-drift"))?;
        source.recheck_v1(self)?;
        let result = source.result_source_v1().ok_or_else(|| fault("source-drift"))?;
        let get = result.present_get_v1(&site).ok_or_else(|| fault("source-drift"))?;
        let completion = self.completion_source_for_owner_v1(owner).ok_or_else(|| fault("completion-missing"))?;
        if terminal.owner() != owner || terminal.return_site() != exit || !completion.explicit_sites().contains(exit)
            || !std::rc::Rc::ptr_eq(&retained, result)
            || !std::sync::Arc::ptr_eq(source.get(), get)
            || receiver_read != source.receiver().receiver_site()
            || key.source().owner() != owner || key.source().nodes().len() != 1
            || get.arguments().len() != 1 || key.source().nodes()[0].site() != get.arguments()[0].site()
            || !matches!(key.source().root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument { call, ordinal: 0, .. }
                if std::sync::Arc::ptr_eq(call, get))
            || !key.reference_coverage_is_exact() || !key.calls().is_empty() || !key.cores().is_empty()
            || !key.operations().is_empty() || !key.owned_result_nodes().is_empty()
        { return Err(fault("source-drift")); }
        Ok(Some(StoredMapGetReturnSourceLoanV1 { terminal: terminal.clone() }))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn stored_map_get_local_source_v1(&self, owner: FunctionOwnerIdV1,
        statement: &SourceNodeSiteV1,
    ) -> Result<Option<std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation;
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None) };
        let mut rows = flow.maps().iter().filter_map(|row| match row {
            MapHomeObservation::FieldGet(frame) if frame.statement().node() == statement => Some(frame), _ => None,
        });
        let Some(frame) = rows.next() else { return Ok(None) };
        if rows.next().is_some() || frame.site().owner() != owner { return Err(fault("source-drift")); }
        frame.source().recheck_v1(self)?;
        Ok(Some(std::rc::Rc::clone(frame)))
    }
}
