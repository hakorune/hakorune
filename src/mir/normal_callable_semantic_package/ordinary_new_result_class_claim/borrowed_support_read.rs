//! Passive result-support reads retain every original loan, never Prefix permission.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{DeclaredCallReadSourceV1, DeclaredReadValueV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedSupportReadV1 {
    source: DeclaredCallReadSourceV1,
    loans: Rc<[BorrowedNullableLocalSourceLoanV1]>,
    anchor: BorrowedLifetimeAnchorV1,
}
impl BorrowedSupportReadV1 {
    pub(super) fn issue(source: DeclaredCallReadSourceV1, loans: Vec<BorrowedNullableLocalSourceLoanV1>) -> Result<Self, &'static str> {
        let first = loans.first().ok_or("borrowed-nullable/source-missing")?;
        let anchor = first.lifetime_anchor_v1().ok_or("borrowed-nullable/actual-root")?;
        if source.original_calls_v1().count() != loans.len() || loans.iter().any(|loan|
            !loan.is_declared_read_v1() || loan.lifetime_anchor_v1().as_ref() != Some(&anchor)
                || source.original_calls_v1().filter(|(call, binding, declaration)| *call == loan.call() && *binding == loan.destination()
                    && loan.declared_read_result_v1().is_some_and(|actual| std::sync::Arc::ptr_eq(actual, declaration))).count() != 1)
            || loans.iter().map(|loan| loan.call()).collect::<BTreeSet<_>>().len() != loans.len() {
            return Err("borrowed-nullable/source-identity");
        }
        Ok(Self { source, loans: loans.into(), anchor })
    }
    pub(crate) fn read(&self) -> &OwnedExprSiteV1 { self.source.read() }
    pub(crate) fn declared_type(&self) -> &str { self.source.declared_type() }
    pub(crate) fn loans(&self) -> impl Iterator<Item = &BorrowedNullableLocalSourceLoanV1> { self.loans.iter() }
    pub(crate) fn anchor(&self) -> &BorrowedLifetimeAnchorV1 { &self.anchor }
    pub(crate) fn single_loan(&self) -> Option<&BorrowedNullableLocalSourceLoanV1> {
        self.source.source_read().and_then(|_| match self.loans.as_ref() { [loan] => Some(loan), _ => None })
    }
    pub(crate) fn payload_nonnull(&self) -> bool {
        !self.source.has_literal_null_v1() && self.loans().all(|loan| loan.payload_nonnull_v1())
    }
    pub(crate) fn matches_value(&self, current: &DeclaredReadValueV1) -> bool {
        self.source.matches_value_v1(current) && self.loans.len() == current.loans().count()
            && self.loans().all(|original| current.loans().filter(|loan| *loan == original).count() == 1)
    }
}
