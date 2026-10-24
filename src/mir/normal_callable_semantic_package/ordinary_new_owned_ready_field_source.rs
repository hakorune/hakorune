//! Exact source readiness shared by direct LOCAL expression and terminal reads.
//! Physical store/read Normal dominance remains a separate required consumer proof.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedFieldWriteFrameV1, LocalFieldReadRequestV1};
#[path = "ordinary_new_owned_ready_field_physical.rs"]
mod physical;

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct OwnedReadyFieldReadSourceV1 {
    frame: Rc<CheckedFieldWriteFrameV1>,
    request: LocalFieldReadRequestV1,
    field: CanonicalFieldRefV1,
}
impl PartialEq for OwnedReadyFieldReadSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.frame, &other.frame) && self.request.owned_ready.as_ref().zip(other.request.owned_ready.as_ref())
            .is_some_and(|(a, b)| a.same_source(b)) && self.request == other.request && self.field == other.field
    }
}
impl Eq for OwnedReadyFieldReadSourceV1 {}
impl OwnedReadyFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn issue(
        candidates: &[super::super::candidate::OrdinaryNewCandidate],
        request: &LocalFieldReadRequestV1, field: CanonicalFieldRefV1, declared: Option<&str>,
    ) -> Result<Self, String> {
        let fail = || fault("owned-ready/source");
        let plan = request.owned_ready.as_ref().ok_or_else(fail)?;
        let (ready_field, storage) = plan.owned_i64_field().ok_or_else(fail)?;
        let declared_matches = match storage {
            crate::mir::function::TypedObjectFieldStorage::I64 => declared == Some("i64"),
            crate::mir::function::TypedObjectFieldStorage::Dynamic => declared.is_none(),
            _ => false,
        };
        let frame = plan.frame();
        let local = frame.owned_local().ok_or_else(fail)?;
        let mut matches = candidates.iter().filter(|row| row.destination == request.home);
        let candidate = matches.next().ok_or_else(fail)?;
        if matches.next().is_some() || request.receiver != request.home || request.home != local.root()
            || request.site.owner() != local.acquisition().owner() || request.receiver_site !=
                crate::mir::resolved_semantics::SourcePathV1::from_node(request.site.site().node())
                    .child(SourcePathSegmentV1::Receiver).expr()
            || request.alias_class.is_some() || request.path.is_some() || request.alias_path.is_some()
            || request.nullable || request.formal || request.received.is_some()
            || !declared_matches || field != ready_field || field != local.field() || candidate.object != field.object()
            || &candidate.site != local.acquisition()
            || !candidate.construction_source.as_ref().is_ok_and(|source| Rc::ptr_eq(source, local.construction()))
            || local.construction().allocation_only_field(&request.field).map(|field| field.field()) != Some(field)
            || local.original().write().field() != request.field.as_ref()
        { return Err(fail()); }
        Ok(Self { frame: Rc::clone(frame), request: request.clone(), field })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn frame(&self) -> &Rc<CheckedFieldWriteFrameV1> { &self.frame }
    pub(in crate::mir::normal_callable_semantic_package) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(in crate::mir::normal_callable_semantic_package) fn requires_checked_read_v1(&self) -> bool {
        self.request.owned_ready.as_ref().and_then(|plan| plan.owned_i64_field())
            .is_some_and(|(_, storage)| storage == crate::mir::function::TypedObjectFieldStorage::Dynamic)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_request(&self, request: &LocalFieldReadRequestV1) -> bool {
        self.request == *request && request.owned_ready.as_ref().zip(self.request.owned_ready.as_ref()).is_some_and(|(plan, original)|
            Rc::ptr_eq(plan.frame(), &self.frame) && plan.same_source(original))
    }
    fn recheck(&self, ledger: &OrdinaryNewClaimLedgerV1, site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1, receiver: BindingRefV1, home: BindingRefV1, field: CanonicalFieldRefV1,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let fail = || fault("owned-ready/identity");
        let local = self.frame.owned_local().ok_or_else(fail)?;
        if self.request.site != *site || self.request.receiver_site != *receiver_site
            || self.request.receiver != receiver || self.request.home != home || self.field != field
            || home != local.root() || receiver != home { return Err(fail()); }
        let source = ledger.field_write_source_at_v1(site.owner(), self.frame.source().root().statement())?
            .ok_or_else(fail)?;
        if !Rc::ptr_eq(source.frame(), &self.frame) || !source.qualified_rhs().zip(
            self.request.owned_ready.as_ref().and_then(|plan| plan.rhs()))
                .is_some_and(|(a, b)| Rc::ptr_eq(a, b)) { return Err(fail()); }
        source.require_current_execution(context)?;
        context.require_ledger(ledger)?;
        context.with_original_field_write_v1(&self.frame, |packet| {
            if packet.source().qualified_rhs().is_none() { return Err(fail()); }
            packet.source().validate_source(ledger)
        })
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum TerminalFieldValueSourceV1 {
    Declared,
    Numeric(super::super::lexical_instance_call::IntegerFieldSourceLoanV1),
    OwnedLocal(OwnedReadyFieldReadSourceV1),
}
impl TerminalFieldValueSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn requires_checked_read_v1(&self) -> bool {
        matches!(self, Self::OwnedLocal(source) if source.requires_checked_read_v1())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_terminal_field_value_v1(&self, site: &OwnedExprSiteV1, row: &FieldRead,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let request = self.terminal_relation.values().find_map(|relation| match relation {
            TerminalRelationV1::I64Add(relation) => relation.field_requests().iter().find(|request| &request.site == site),
            TerminalRelationV1::I64Field(relation) if relation.field_read_site() == site => Some(relation.field_request()),
            _ => None,
        });
        match (&row.value_source, request.and_then(|request| request.owned_ready.as_ref())) {
            (TerminalFieldValueSourceV1::OwnedLocal(source), Some(frame)) => {
                if !Rc::ptr_eq(source.frame(), frame.frame()) || !source.matches_request(request.unwrap()) {
                    return Err(fault("owned-ready/terminal-source-drift"));
                }
                source.recheck(self, site, &row.receiver_site, row.receiver, row.home, row.field, context)
            }
            (TerminalFieldValueSourceV1::OwnedLocal(_), None) | (_, Some(_)) => Err(fault("owned-ready/terminal-source-missing")),
            (TerminalFieldValueSourceV1::Numeric(source), None) => self.recheck_integer_field_source_v1(source, site, row),
            (TerminalFieldValueSourceV1::Declared, None) => Ok(()),
        }
    }
    pub(super) fn recheck_owned_local_read_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead,
        context: crate::mir::normal_callable_semantic_package::CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        let source = row.source.owned_ready.as_ref().ok_or_else(|| fault("owned-ready/local-source-missing"))?;
        let required = self.source_field_requirement_v1(site)?
            .ok_or_else(|| fault("owned-ready/read-context-missing"))?;
        if !source.matches_request(required.request()) || !required.prior_homes().contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Stable(row.home))
            || row.numeric_source.is_some() { return Err(fault("owned-ready/read-context-drift")); }
        source.recheck(self, site, &row.receiver_site, row.receiver, row.home, row.source.field(), context)
    }
}
impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_owned_ready(
        original: &Rc<Self>, source: OwnedReadyFieldReadSourceV1,
    ) -> Result<Rc<Self>, String> {
        let declared_matches = source.request.owned_ready.as_ref().and_then(|plan| plan.owned_i64_field())
            .is_some_and(|(_, storage)| match storage {
                crate::mir::function::TypedObjectFieldStorage::I64 => original.declared_type_name() == Some("i64"),
                crate::mir::function::TypedObjectFieldStorage::Dynamic => original.declared_type_name().is_none(),
                _ => false,
            });
        if !declared_matches || original.owned_ready.is_some() || original.received.is_some() || original.returned.is_some()
            || original.expression_path.is_some() || original.field != source.field()
            || *original.result() != crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar {
            return Err(fault("owned-ready/declaration"));
        }
        let mut descriptor = original.as_ref().clone(); descriptor.owned_ready = Some(source); Ok(Rc::new(descriptor))
    }
}
