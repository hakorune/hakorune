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
    pub(crate) fn source_signature_v1(&self) -> &DeclaredCallReadSourceV1 { &self.source }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.source.binding() }
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

impl BorrowedSupportReadV1 {
    fn same_original_source_v1(&self, other: &Self) -> bool {
        self.source == other.source && self.anchor == other.anchor
            && self.loans.len() == other.loans.len()
            && self.loans().all(|loan| other.loans().filter(|original|
                loan.same_original_source_v1(original)).count() == 1)
    }
}
impl BorrowedCallSupportV1 {
    pub(super) fn same_original_source_v1(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Reborrow { callee: a, actual: x }, Self::Reborrow { callee: b, actual: y }) =>
                a == b && x.same_original_source_v1(y),
            _ => self == other,
        }
    }
}
impl BorrowedNullableLocalSourceLoanV1 {
    fn same_original_source_v1(&self, other: &Self) -> bool {
        self.declaration == other.declaration && self.destination == other.destination
            && self.common.same_original_source_v1(&other.common)
    }
    /// Bind every nested support to this preparation's existing producer map.
    /// The outer producer retains its original witnesses; no second issuer is
    /// created and Prefix still requires exact canonical-loan equality.
    pub(in crate::mir::normal_callable_semantic_package) fn canonical_supports_v1(
        producers: &BTreeMap<OwnedExprSiteV1, Box<Self>>,
    ) -> Result<BTreeMap<OwnedExprSiteV1, Box<Self>>, (OwnedExprSiteV1, &'static str)> {
        fn visit(call: &OwnedExprSiteV1, producers: &BTreeMap<OwnedExprSiteV1, Box<BorrowedNullableLocalSourceLoanV1>>,
            active: &mut BTreeSet<OwnedExprSiteV1>, memo: &mut BTreeMap<OwnedExprSiteV1, Box<BorrowedNullableLocalSourceLoanV1>>,
        ) -> Result<Box<BorrowedNullableLocalSourceLoanV1>, &'static str> {
            if let Some(loan) = memo.get(call) { return Ok(loan.clone()); }
            if !active.insert(call.clone()) { return Err("borrowed-nullable/source-identity"); }
            let original = producers.get(call).ok_or("borrowed-nullable/source-missing")?;
            let mut loan = original.clone();
            if let BorrowedCallSupportV1::Reborrow { callee, actual } = &original.support {
                let mut leaves = Vec::new();
                for proposed in actual.loans() {
                    let canonical = visit(proposed.call(), producers, active, memo)?;
                    if !proposed.same_original_source_v1(&canonical) { return Err("borrowed-nullable/source-identity"); }
                    leaves.push(*canonical);
                }
                loan.common.support = BorrowedCallSupportV1::Reborrow { callee: *callee,
                    actual: BorrowedSupportReadV1::issue(actual.source.clone(), leaves)? };
            }
            active.remove(call);
            memo.insert(call.clone(), loan.clone());
            Ok(loan)
        }
        let mut memo = BTreeMap::new();
        let mut active = BTreeSet::new();
        for call in producers.keys() {
            visit(call, producers, &mut active, &mut memo).map_err(|issue| (call.clone(), issue))?;
        }
        Ok(memo)
    }
}
