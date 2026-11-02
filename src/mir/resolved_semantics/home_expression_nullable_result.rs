//! Nullable identity keeps its genuine source; it grants no payload view/Home.
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};
use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, MapFieldGetLocalV1};
use crate::mir::normal_callable_semantic_package::BorrowedNullableLocalSourceLoanV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) enum NullableResultReferenceV1 {
    StoredMapGet(Rc<MapFieldGetLocalV1>),
    DeclaredFrom(Box<BorrowedNullableLocalSourceLoanV1>),
}
impl PartialEq for NullableResultReferenceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StoredMapGet(a), Self::StoredMapGet(b)) => Rc::ptr_eq(a, b),
            (Self::DeclaredFrom(a), Self::DeclaredFrom(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for NullableResultReferenceV1 {}
impl NullableResultReferenceV1 {
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match self { Self::StoredMapGet(frame) => frame.binding(), Self::DeclaredFrom(loan) => loan.destination() }
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match self { Self::StoredMapGet(frame) => frame.site(), Self::DeclaredFrom(loan) => loan.call() }
    }
    pub(crate) fn map_frame(&self) -> Option<&Rc<MapFieldGetLocalV1>> {
        match self { Self::StoredMapGet(frame) => Some(frame), _ => None }
    }
    pub(crate) fn matches_source(&self) -> bool {
        match self {
            Self::StoredMapGet(frame) => frame.source().get().owner() == frame.site().owner()
                && frame.source().get().site() == frame.site().site(),
            Self::DeclaredFrom(loan) => loan.is_declared_read_v1() && loan.destination().owner() == loan.call().owner(),
        }
    }
    pub(crate) fn accepts_prior_homes(&self, homes: &[LiveHomeV1]) -> bool {
        match self {
            Self::StoredMapGet(frame) => homes == frame.prior_homes(),
            Self::DeclaredFrom(loan) => !homes.iter().any(|home| home.binding() == loan.destination())
                && match loan.owned_local_support_v1() {
                    Some((root, _, _)) => homes.iter().any(|home| home.binding() == root),
                    None => loan.root().is_some(),
                },
        }
    }
}
