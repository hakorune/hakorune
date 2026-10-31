//! The existing field-read progress map carries original nested Return paths.
//! Immediate receivers are actual predecessor-node outputs, never fake bindings.
use super::*;
use crate::mir::normal_callable_semantic_package::CallableScalarResultSourceLoanV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn stage_result_field_paths_v1(&self, result: &CallableScalarResultSourceLoanV1) -> Result<(), String> {
        let canonical = self.callable_scalar_result_source_v1(result.owner())?
            .ok_or_else(|| fault("result-path/result-missing"))?;
        if !canonical.same_facts(result) { return Err(fault("result-path/result-drift")); }
        let mut staged = BTreeMap::new();
        for loan in result.nested_result_field_sources_v1()? {
            loan.recheck(&canonical)?;
            let site = loan.site();
            let (field, kind, declared) = loan.descriptor()?;
            let source = Rc::new(LocalFieldReadSourceV1 { field, result: kind,
                declared_type_name: declared.map(Into::into), expression_path: Some(FieldPathSourceV1::Return(loan.clone())), received: None, owned_ready: None, returned: None, shared_support: None, array_reads: Vec::new() });
            if staged.insert(site, LocalFieldRead { local: None, receiver_site: loan.receiver_site().clone(),
                receiver: loan.root(), home: loan.root(), source, numeric_source: None,
                progress: LocalReadProgress::Pending }).is_some() { return Err(fault("result-path/duplicate")); }
        }
        let mut rows = self.local_field_reads.borrow_mut();
        for (site, row) in &staged {
            if let Some(previous) = rows.get(site) {
                if previous.source != row.source || !matches!(previous.progress, LocalReadProgress::Pending) {
                    return Err(fault("result-path/stage-conflict"));
                }
            }
        }
        for (site, row) in staged { rows.entry(site).or_insert(row); }
        Ok(())
    }
    pub(super) fn validate_result_path_fields_present_v1(&self, owner: FunctionOwnerIdV1,
        rows: &BTreeMap<OwnedExprSiteV1, LocalFieldRead>,
    ) -> Result<Option<CallableScalarResultSourceLoanV1>, String> {
        for (_, row) in rows.iter().filter(|(site, _)| site.owner() == owner) {
            if let Some(path) = &row.source.expression_path {
                let parent = path.parent().and_then(|site| rows.get(&site))
                    .and_then(|row| row.source.expression_path.as_ref());
                if !path.matches_parent(parent) { return Err(fault("expression-path/parent-source-drift")); }
            }
        }
        let Some(result) = self.callable_scalar_result_source_v1(owner)? else { return Ok(None); };
        for loan in result.nested_result_field_sources_v1()? {
            let row = rows.get(&loan.site()).ok_or_else(|| fault("result-path/required-read-missing"))?;
            if row.source.expression_path.as_ref() != Some(&FieldPathSourceV1::Return(loan)) { return Err(fault("result-path/required-source-drift")); }
        }
        Ok(Some(result))
    }
    pub(super) fn recheck_result_path_field_v1(&self, site: &OwnedExprSiteV1, row: &LocalFieldRead, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let loan = row.source.expression_path.as_ref().ok_or_else(|| fault("result-path/source-missing"))?;
        context.require_ledger(self)?;
        let result = loan.result_source_for_recheck_v1(self)?;
        self.recheck_result_path_field_using_result_v1(site, row, context, result.as_ref())
    }
    pub(super) fn recheck_result_path_field_using_result_v1(&self, site: &OwnedExprSiteV1,
        row: &LocalFieldRead, context: CallValidationContextV1<'_>, result: Option<&CallableScalarResultSourceLoanV1>,
    ) -> Result<(), String> {
        let loan = row.source.expression_path.as_ref().ok_or_else(|| fault("result-path/source-missing"))?;
        loan.recheck_using_result_v1(self, context, result)?;
        let (field, kind, declared) = loan.descriptor()?;
        if loan.site() != *site || row.receiver_site != *loan.receiver_site()
            || row.receiver != loan.root() || row.home != loan.home() || row.numeric_source.is_some()
            || row.source.field != field || row.source.result != kind
            || row.source.declared_type_name.as_deref() != declared {
            return Err(fault("result-path/descriptor-drift"));
        }
        Ok(())
    }
    pub(crate) fn reserve_result_path_field_v1(&self, site: &OwnedExprSiteV1,
        resolve_root: impl FnOnce(BindingRefV1) -> Result<ValueId, String>,
    ) -> Result<bool, String> {
        let mut rows = self.local_field_reads.borrow_mut();
        let Some(row) = rows.get_mut(site).filter(|row| row.source.expression_path.is_some()) else {
            if let Some(result) = self.callable_scalar_result_source_v1(site.owner())? {
                if result.nested_result_field_sources_v1()?.iter().any(|loan| loan.site() == *site) {
                    return Err(fault("result-path/required-read-missing"));
                }
            }
            if self.source_field_requirement_v1(site)?.is_some_and(|required| required.request().path.is_some()) {
                return Err(fault("expression-path/required-read-missing"));
            }
            return Ok(false);
        };
        self.recheck_result_path_field_v1(site, row, CallValidationContextV1::Lowering(self))?;
        if !matches!(row.progress, LocalReadProgress::Pending) { return Err(fault("already-taken")); }
        row.progress = LocalReadProgress::SourceReceiverPending { root: resolve_root(row.receiver)? };
        Ok(true)
    }
    pub(crate) fn bind_result_path_receiver_v1(&self, site: &OwnedExprSiteV1, actual: ValueId,
    ) -> Result<Rc<LocalFieldReadSourceV1>, String> {
        let mut rows = self.local_field_reads.borrow_mut();
        let row = rows.get(site).ok_or_else(|| fault("missing-source-site"))?;
        let loan = row.source.expression_path.as_ref().ok_or_else(|| fault("result-path/source-missing"))?;
        let result = loan.result_source_for_recheck_v1(self)?;
        self.recheck_result_path_field_using_result_v1(site, row, CallValidationContextV1::Lowering(self), result.as_ref())?;
        let LocalReadProgress::SourceReceiverPending { root } = row.progress else { return Err(fault("result-path/not-reserved")); };
        let loan = row.source.expression_path.as_ref().expect("checked path source");
        let expected = if let Some(parent) = loan.parent() {
            let previous = rows.get(&parent).ok_or_else(|| fault("result-path/ancestor-missing"))?;
            if !loan.matches_parent(previous.source.expression_path.as_ref()) { return Err(fault("expression-path/parent-source-drift")); }
            self.recheck_result_path_field_using_result_v1(&parent, previous,
                CallValidationContextV1::Lowering(self), result.as_ref())?;
            match &previous.progress {
                LocalReadProgress::Emitted { instruction: MirInstruction::ObjectFieldGet { dst, .. }, source, .. }
                    if !source.requires_checked_field_read_v1() && Rc::ptr_eq(source, &previous.source) => *dst,
                LocalReadProgress::CheckedEmitted { normal: (_, MirInstruction::InvokeNormalResult { dst, .. }), source, .. }
                    if source.requires_checked_field_read_v1() && Rc::ptr_eq(source, &previous.source) => *dst,
                _ => return Err(fault("result-path/ancestor-unconsumed")),
            }
        } else { root };
        if actual != expected { return Err(fault("result-path/receiver-drift")); }
        let row = rows.get_mut(site).expect("checked source row");
        let source = Rc::clone(&row.source);
        row.progress = LocalReadProgress::Taken { base: actual, source: Rc::clone(&source) };
        Ok(source)
    }
}
