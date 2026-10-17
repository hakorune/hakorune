//! Exact source readiness shared by direct LOCAL expression and terminal reads.
//! Physical store/read Normal dominance remains a separate required consumer proof.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{CheckedFieldWriteFrameV1, LocalFieldReadRequestV1};

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct OwnedReadyFieldReadSourceV1 {
    frame: Rc<CheckedFieldWriteFrameV1>,
    request: LocalFieldReadRequestV1,
    field: CanonicalFieldRefV1,
}
impl PartialEq for OwnedReadyFieldReadSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.frame, &other.frame) && self.request == other.request && self.field == other.field
    }
}
impl Eq for OwnedReadyFieldReadSourceV1 {}
impl OwnedReadyFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn issue(
        candidates: &[super::super::candidate::OrdinaryNewCandidate],
        request: &LocalFieldReadRequestV1, field: CanonicalFieldRefV1, declared: Option<&str>,
    ) -> Result<Self, String> {
        let fail = || fault("owned-ready/source");
        let frame = request.owned_ready.as_ref().ok_or_else(fail)?;
        let local = frame.owned_local().ok_or_else(fail)?;
        let mut matches = candidates.iter().filter(|row| row.destination == request.home);
        let candidate = matches.next().ok_or_else(fail)?;
        if matches.next().is_some() || request.receiver != request.home || request.home != local.root()
            || request.site.owner() != local.acquisition().owner() || request.receiver_site !=
                crate::mir::resolved_semantics::SourcePathV1::from_node(request.site.site().node())
                    .child(SourcePathSegmentV1::Receiver).expr()
            || request.alias_class.is_some() || request.path.is_some() || request.alias_path.is_some()
            || request.nullable || request.formal || request.received.is_some()
            || declared != Some("i64") || field != local.field() || candidate.object != field.object()
            || &candidate.site != local.acquisition()
            || !candidate.construction_source.as_ref().is_ok_and(|source| Rc::ptr_eq(source, local.construction()))
            || local.construction().allocation_only_i64_field(&request.field) != Some(field)
            || local.original().write().field() != request.field.as_ref()
        { return Err(fail()); }
        Ok(Self { frame: Rc::clone(frame), request: request.clone(), field })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn frame(&self) -> &Rc<CheckedFieldWriteFrameV1> { &self.frame }
    pub(in crate::mir::normal_callable_semantic_package) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_request(&self, request: &LocalFieldReadRequestV1) -> bool {
        self.request == *request && request.owned_ready.as_ref().is_some_and(|frame| Rc::ptr_eq(frame, &self.frame))
    }
    fn recheck(&self, ledger: &OrdinaryNewClaimLedgerV1, site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1, receiver: BindingRefV1, home: BindingRefV1, field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let fail = || fault("owned-ready/identity");
        let local = self.frame.owned_local().ok_or_else(fail)?;
        if self.request.site != *site || self.request.receiver_site != *receiver_site
            || self.request.receiver != receiver || self.request.home != home || self.field != field
            || home != local.root() || receiver != home { return Err(fail()); }
        ledger.field_write_claims.recheck_original_write_source_v1(local.original()).map_err(|_| fail())?;
        ledger.with_local_new_source_v1(local.acquisition(), home, |_, object, _, _| {
            if object != field.object() { return Err(fail()); } Ok(())
        })?.ok_or_else(fail)?;
        let completion = ledger.completion_for_owner(site.owner()).ok_or_else(fail)?;
        let flow = completion.cleanup().root_flow().ok_or_else(fail)?;
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.field_write_frame())
            .filter(|frame| Rc::ptr_eq(frame, &self.frame));
        if frames.next().is_none() || frames.next().is_some() { return Err(fail()); }
        Ok(())
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum TerminalFieldValueSourceV1 {
    Declared,
    Numeric(super::super::lexical_instance_call::IntegerFieldSourceLoanV1),
    OwnedLocal(OwnedReadyFieldReadSourceV1),
}
impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn recheck_terminal_field_value_v1(&self, site: &OwnedExprSiteV1, row: &FieldRead) -> Result<(), String> {
        let request = self.terminal_relation.values().find_map(|relation| match relation {
            TerminalRelationV1::I64Add(relation) => relation.field_requests().iter().find(|request| &request.site == site),
            TerminalRelationV1::I64Field(relation) if relation.field_read_site() == site => Some(relation.field_request()),
            _ => None,
        });
        match (&row.value_source, request.and_then(|request| request.owned_ready.as_ref())) {
            (TerminalFieldValueSourceV1::OwnedLocal(source), Some(frame)) => {
                if !Rc::ptr_eq(source.frame(), frame) || !source.matches_request(request.unwrap()) {
                    return Err(fault("owned-ready/terminal-source-drift"));
                }
                source.recheck(self, site, &row.receiver_site, row.receiver, row.home, row.field)?;
                Err(fault("owned-ready/physical-consumer-pending"))
            }
            (TerminalFieldValueSourceV1::OwnedLocal(_), None) | (_, Some(_)) => Err(fault("owned-ready/terminal-source-missing")),
            (TerminalFieldValueSourceV1::Numeric(source), None) => self.recheck_integer_field_source_v1(source, site, row),
            (TerminalFieldValueSourceV1::Declared, None) => Ok(()),
        }
    }
    pub(super) fn recheck_owned_local_read_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead) -> Result<(), String> {
        let source = row.source.owned_ready.as_ref().ok_or_else(|| fault("owned-ready/local-source-missing"))?;
        source.recheck(self, site, &row.receiver_site, row.receiver, row.home, row.source.field())?;
        Err(fault("owned-ready/physical-consumer-pending"))
    }
}
impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_owned_ready(
        original: &Rc<Self>, source: OwnedReadyFieldReadSourceV1,
    ) -> Result<Rc<Self>, String> {
        if original.owned_ready.is_some() || original.received.is_some() || original.returned.is_some()
            || original.expression_path.is_some() || original.field != source.field()
            || original.declared_type_name() != Some("i64")
            || *original.result() != crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar {
            return Err(fault("owned-ready/declaration"));
        }
        let mut descriptor = original.as_ref().clone(); descriptor.owned_ready = Some(source); Ok(Rc::new(descriptor))
    }
}
