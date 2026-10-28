//! Original argument use on the existing local-field descriptor and affine map.
use super::*;
use super::super::result_class_claim::ReturnedFieldSourceLoanV1;
use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReturnedFieldArgumentSourceV1 {
    call: OwnedExprSiteV1,
    ordinal: u32,
    source: ReturnedFieldSourceLoanV1,
}
impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn returned_argument_v1(
        &self,
    ) -> Option<(&OwnedExprSiteV1, u32, &ReturnedFieldSourceLoanV1)> {
        self.returned.as_ref().map(|row| (&row.call, row.ordinal, &row.source))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    fn recheck_returned_argument_v1(&self, row: &ReturnedFieldArgumentSourceV1) -> Result<(), String> {
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("returned/ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let original = ingress.original_returned_field_actual_source_v1(&row.call, row.ordinal)?
            .ok_or_else(|| fault("returned/original-argument-missing"))?;
        if original != &row.source { return Err(fault("returned/original-argument-drift")); }
        self.callable_result_classes.recheck_returned_field_graph_v1(original, &self.field_write_claims,
            &mut |site| ingress.object_source_target_at_v1(site).map_err(|_| "returned-field/current-target-identity"))
            .map_err(|issue| fault(issue))
    }

    /// Stage only the source descriptor. Existing progress and Rc survive every
    /// reconstruction; a Pending row lends no Home, parent value or field value.
    pub(in crate::mir::normal_callable_semantic_package) fn retained_returned_field_read_source_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32, parent: BindingRefV1, acquisition: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<LocalFieldReadSourceV1>>, String> {
        let ingress = self.borrowed_formal_source.as_ref().ok_or_else(|| fault("returned/ingress-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let original = ingress.original_returned_field_actual_source_v1(call, ordinal)?
            .ok_or_else(|| fault("returned/original-argument-missing"))?;
        if original.parent() != parent || original.acquisition() != acquisition
            || original.read().owner() != call.owner() {
            return Err(fault("returned/observed-parent-drift"));
        }
        let retained = ReturnedFieldArgumentSourceV1 { call: call.clone(), ordinal, source: original.clone() };
        self.recheck_returned_argument_v1(&retained)?;
        let Some(class) = original.class() else { return Ok(None); };
        {
            let rows = self.local_field_reads.borrow();
            if let Some(row) = rows.get(original.read()) {
                if row.source.returned.as_ref() != Some(&retained) {
                    return Err(fault("returned/staged-source-drift"));
                }
                self.recheck_returned_field_read_v1(original.read(), row)?;
                return Ok(Some(Rc::clone(&row.source)));
            }
        }
        let source = Rc::new(LocalFieldReadSourceV1 { field: original.field(),
            result: LocalFieldReadResultV1::Alias(class.into()), declared_type_name: None,
            expression_path: None, received: None, returned: Some(retained), owned_ready: None, array_reads: Vec::new() });
        let row = LocalFieldRead { local: None, receiver_site: original.receiver_site().clone(),
            receiver: parent, home: parent, source, numeric_source: None, progress: LocalReadProgress::Pending };
        super::super::coseal_issue::source_claims::stage_local_field_read_batch(
            &mut self.local_field_reads.borrow_mut(), vec![(original.read().clone(), row)])
            .map_err(|issue| format!("{}: {issue:?}", fault("returned/staging")))?;
        let rows = self.local_field_reads.borrow();
        let row = rows.get(original.read()).ok_or_else(|| fault("returned/staged-read-missing"))?;
        self.recheck_returned_field_read_v1(original.read(), row)?;
        Ok(Some(Rc::clone(&row.source)))
    }

    /// Revalidate current membership without staging or changing affine progress.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_returned_field_argument_source_v1(
        &self, source: &Rc<LocalFieldReadSourceV1>,
    ) -> Result<(), String> {
        let retained = source.returned.as_ref().ok_or_else(|| fault("returned/source-missing"))?;
        let rows = self.local_field_reads.borrow();
        let row = rows.get(retained.source.read()).ok_or_else(|| fault("returned/read-missing"))?;
        if !Rc::ptr_eq(&row.source, source) { return Err(fault("returned/source-identity")); }
        self.recheck_returned_field_read_v1(retained.source.read(), row)
    }

    /// Exact current map membership is required even after source metadata loss.
    pub(in crate::mir) fn returned_field_argument_parent_v1(
        &self, source: &Rc<LocalFieldReadSourceV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(ValueId, (BasicBlockId, MirInstruction), (BasicBlockId, MirInstruction)), String> {
        context.require_ledger(self)?;
        let retained = source.returned.as_ref().ok_or_else(|| fault("returned/source-missing"))?;
        let loan = &retained.source;
        self.recheck_returned_field_argument_source_v1(source)?;
        let outer = self.local_call_for_owner(retained.call.owner(), retained.call.site())
            .ok_or_else(|| fault("returned/outer-call-missing"))?;
        if outer.prior_homes().iter().filter(|home| home.binding() == loan.parent()).count() != 1
            || !matches!(outer.arguments().get(retained.ordinal as usize),
                Some(crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal, site })
                    if *ordinal == retained.ordinal && site == loan.read().site())
        { return Err(fault("returned/outer-home-or-argument")); }
        let parent = self.local_call_for_owner(loan.acquisition().owner(), loan.acquisition().site())
            .ok_or_else(|| fault("returned/parent-call-missing"))?;
        if parent.local_binding().map(|(_, binding)| binding) != Some(loan.parent()) {
            return Err(fault("returned/parent-destination"));
        }
        context.with_taken_packet_v1(loan.acquisition(), |packet| {
            let row = packet.original_row()?;
            let original = self.borrowed_object_call_packet_v1(row)?;
            if row.call_site() != loan.acquisition()
                || original.result() != crate::mir::instruction::InvokeCallResultKind::Handle
            { return Err(fault("returned/parent-handle-packet")); }
            let value = packet.value_in_context_v1(loan.read().owner(), original.arguments(), context)?;
            let (invoke, normal) = packet.outer_bindings()?;
            Ok((value, invoke.clone(), normal.clone()))
        })
    }

    pub(in crate::mir) fn returned_field_argument_binding_v1(
        &self, source: &Rc<LocalFieldReadSourceV1>,
    ) -> Result<(BasicBlockId, MirInstruction), String> {
        let retained = source.returned.as_ref().ok_or_else(|| fault("returned/source-missing"))?;
        let rows = self.local_field_reads.borrow();
        let row = rows.get(retained.source.read()).ok_or_else(|| fault("returned/read-missing"))?;
        self.recheck_returned_field_read_v1(retained.source.read(), row)?;
        if !Rc::ptr_eq(source, &row.source) { return Err(fault("returned/source-identity")); }
        match &row.progress {
            LocalReadProgress::Emitted { block, instruction, source: emitted } if Rc::ptr_eq(emitted, source) =>
                Ok((*block, instruction.clone())),
            _ => Err(fault("returned/read-not-emitted")),
        }
    }

    pub(super) fn recheck_returned_field_read_v1(
        &self, site: &OwnedExprSiteV1, row: &LocalFieldRead,
    ) -> Result<(), String> {
        let retained = row.source.returned.as_ref().ok_or_else(|| fault("returned/source-missing"))?;
        let original = &retained.source;
        if site != original.read() || row.receiver != original.parent() || row.home != original.parent()
            || &row.receiver_site != original.receiver_site() || row.source.field() != original.field()
            || !matches!(row.source.result(), LocalFieldReadResultV1::Alias(class) if Some(class.as_ref()) == original.class())
            || row.source.expression_path.is_some() || row.source.received.is_some() || row.numeric_source.is_some()
            || row.local.is_some() || row.source.declared_type_name.is_some()
        { return Err(fault("returned/descriptor-drift")); }
        self.recheck_returned_argument_v1(retained)
    }
}
