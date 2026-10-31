//! Cleanup provenance on the original checked read; enclosing expression
//! graphs keep their own acquisitions, source reads keep their current Homes.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::resolved_semantics::home_new_prefix::ExpressionFieldRequirementV1;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) enum CheckedFieldReadCleanupV1 {
    EnclosingExpression,
    SourceHomes {
        requirement: ExpressionFieldRequirementV1,
        frame: Binding,
        normal_prefix: Vec<Binding>,
        fault: Vec<Binding>,
    },
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn record_enclosed_checked_local_field_read_v1(
        &self, site: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        read: Option<Rc<ExactLexicalReadV1>>, invoke: Binding, normal: Binding,
    ) -> Result<(), String> {
        self.record_checked_local_field_read_v1(site, source, read, invoke, normal,
            CheckedFieldReadCleanupV1::EnclosingExpression)
    }
    pub(in crate::mir) fn record_source_checked_local_field_read_v1(
        &self, site: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        read: Rc<ExactLexicalReadV1>, invoke: Binding, normal: Binding,
        frame: Binding, normal_prefix: Vec<Binding>, fault: Vec<Binding>,
    ) -> Result<(), String> {
        let requirement = self.source_field_requirement_v1(site)?
            .ok_or_else(|| fault_error("source-cleanup/requirement"))?.clone();
        self.record_checked_local_field_read_v1(site, source, Some(read), invoke, normal,
            CheckedFieldReadCleanupV1::SourceHomes { requirement, frame, normal_prefix, fault })
    }
    pub(in crate::mir) fn checked_field_normal_prefix_v1(&self, site: &OwnedExprSiteV1) -> Result<Vec<Binding>, String> {
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault_error("source-cleanup/read"))?;
        match &row.progress {
            LocalReadProgress::CheckedEmitted { cleanup: CheckedFieldReadCleanupV1::SourceHomes { normal_prefix, .. }, .. } => Ok(normal_prefix.clone()),
            LocalReadProgress::CheckedEmitted { .. } => Ok(Vec::new()),
            _ => Err(fault_error("source-cleanup/unconsumed")),
        }
    }
    pub(in crate::mir) fn checked_field_cleanup_bindings_v1(&self, site: &OwnedExprSiteV1) -> Result<Vec<Binding>, String> {
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault_error("source-cleanup/read"))?;
        match &row.progress {
            LocalReadProgress::CheckedEmitted { cleanup: CheckedFieldReadCleanupV1::SourceHomes { frame, fault, .. }, .. } =>
                Ok(std::iter::once(frame.clone()).chain(fault.iter().cloned()).collect()),
            LocalReadProgress::CheckedEmitted { .. } => Ok(Vec::new()),
            _ => Err(fault_error("source-cleanup/unconsumed")),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn validate_checked_field_cleanup_v1(&self, site: &OwnedExprSiteV1,
        function: &MirFunction, context: CallValidationContextV1<'_>,
        project: &mut impl FnMut(&Binding) -> Result<Option<Binding>, String>,
    ) -> Result<(), String> {
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault_error("source-cleanup/read"))?;
        let LocalReadProgress::CheckedEmitted { cleanup, invoke, .. } = &row.progress
            else { return Err(fault_error("source-cleanup/unconsumed")); };
        let CheckedFieldReadCleanupV1::SourceHomes { requirement, frame, normal_prefix, fault } = cleanup else { return Ok(()); };
        context.require_ledger(self)?;
        let original = self.source_field_requirement_v1(site)?.ok_or_else(|| fault_error("source-cleanup/requirement"))?;
        if !Rc::ptr_eq(original.plan(), requirement.plan()) || original != requirement {
            return Err(fault_error("source-cleanup/requirement-drift"));
        }
        let mut map = |binding: &Binding| -> Result<Binding, String> {
            let mapped = project(binding)?.ok_or_else(|| fault_error("source-cleanup/removed"))?;
            if function.blocks.get(&mapped.0).into_iter().flat_map(|body| body.all_instructions())
                .filter(|row| **row == mapped.1).count() != 1 {
                return Err(fault_error("source-cleanup/membership"));
            }
            Ok(mapped)
        };
        let invoke = map(invoke)?;
        let frame = map(frame)?;
        if !matches!((&frame.1, &invoke.1),
            (MirInstruction::FaultFrameEnter { dst, .. }, MirInstruction::Invoke { fault_frame, .. }) if dst == fault_frame) {
            return Err(fault_error("source-cleanup/frame"));
        }
        for row in normal_prefix { map(row)?; }
        let fault = fault.iter().map(&mut map).collect::<Result<Vec<_>, _>>()?;
        self.validate_source_field_fault_v1(
            site.owner(), function, &invoke, &fault, requirement.prior_homes(),
        )
    }
    pub(in crate::mir) fn original_checked_field_bindings_v1(&self, owner: FunctionOwnerIdV1) -> Result<Vec<Binding>, String> {
        let sites: Vec<_> = self.local_field_reads.borrow().iter()
            .filter(|(site, row)| site.owner() == owner && row.source.requires_checked_field_read_v1())
            .map(|(site, _)| site.clone()).collect();
        let mut result = Vec::new();
        for site in sites {
            result.extend(self.checked_field_normal_prefix_v1(&site)?);
            if let Some(pair) = self.checked_field_pair_v1(&site, CallValidationContextV1::Lowering(self))? { result.extend(pair); }
            result.extend(self.checked_field_cleanup_bindings_v1(&site)?);
        }
        let mut unique = Vec::new();
        for row in result { if !unique.contains(&row) { unique.push(row); } }
        Ok(unique)
    }
}
fn fault_error(reason: &str) -> String { fault(reason) }
