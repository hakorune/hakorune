//! Original formal Return provenance and its source recheck.
use super::*;

/// Original nullable-forward formal Return, retaining the complete result roots.
/// This lends source identity only, never an executable result or an owned Home.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedFormalReturnSourceLoanV1 {
    key: CanonicalSameModuleCallableKeyV1,
    value: OwnedExprSiteV1,
    formal: BindingRefV1,
    ordinal: u32,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl PartialEq for BorrowedFormalReturnSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.value == other.value
            && self.formal == other.formal
            && self.ordinal == other.ordinal
            && self.roots.len() == other.roots.len()
            && self
                .roots
                .iter()
                .zip(other.roots.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for BorrowedFormalReturnSourceLoanV1 {}
impl BorrowedFormalReturnSourceLoanV1 {
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }
    pub(crate) fn value(&self) -> &OwnedExprSiteV1 {
        &self.value
    }
    pub(crate) fn formal(&self) -> BindingRefV1 {
        self.formal
    }
    pub(crate) fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub(crate) fn roots(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.roots
    }
}

impl OrdinaryNewResultClassClaimsV1 {
    pub(crate) fn borrowed_formal_return_source_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        value: &OwnedExprSiteV1,
        formal: BindingRefV1,
        ordinal: u32,
    ) -> Result<Option<BorrowedFormalReturnSourceLoanV1>, &'static str> {
        if self.get(key) != Some(&OrdinaryNewResultClassV1::NullableForwarded { ordinal }) {
            return Ok(None);
        }
        if formal.owner() != value.owner() {
            return Err("borrowed-formal-return/owner");
        }
        let roots = self.checked_original_source_roots_v1(key, value.owner())?;
        let matching: Vec<_> = roots.iter().filter(|root| root.site() == value).collect();
        if matching.is_empty() {
            return Ok(None);
        }
        let origins: BTreeSet<_> = matching.iter().map(|root| root.origin().clone()).collect();
        if matching.len() != 2
            || origins
                != BTreeSet::from([
                    ResultValueOriginV1::Null,
                    ResultValueOriginV1::ForwardFormal { ordinal },
                ])
            || matching.iter().any(|root| {
                !matches!(root.step(),
                ResultWitnessStepV1::Formal { binding, ordinal: original }
                    if *binding == formal && *original == ordinal)
            })
        {
            return Err("borrowed-formal-return/witness");
        }
        Ok(Some(BorrowedFormalReturnSourceLoanV1 {
            key: key.clone(),
            value: value.clone(),
            formal,
            ordinal,
            roots: roots.into_iter().cloned().collect(),
        }))
    }

    pub(crate) fn recheck_borrowed_formal_return_source_v1(
        &self,
        loan: &BorrowedFormalReturnSourceLoanV1,
    ) -> Result<(), &'static str> {
        if self
            .borrowed_formal_return_source_v1(&loan.key, &loan.value, loan.formal, loan.ordinal)?
            .as_ref()
            != Some(loan)
        {
            return Err("borrowed-formal-return/source-drift");
        }
        Ok(())
    }
}
