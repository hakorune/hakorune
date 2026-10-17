//! One source consult for declared scalar reads and original field-write plans.
//! Borrowing a plan grants no FieldSet, Home or scalar operation permission.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
use std::rc::Rc;

pub(crate) trait HomeFieldSourceConsultV1<E> {
    fn scalar_field(&mut self, site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1, home: BindingRefV1, field: &str) -> Result<bool, E>;
    fn assignment_source(&mut self, _owner: FunctionOwnerIdV1, _target: &SourceExprSiteV1)
        -> Result<Option<Rc<ExpressionSourcePlanV1>>, E> { Ok(None) }
}

// Existing read-only probes keep their source-only contract. Only the original
// package inventory supplies assignment plans in the production implementation.
impl<E, F> HomeFieldSourceConsultV1<E> for F
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str) -> Result<bool, E> {
    fn scalar_field(&mut self, site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1, home: BindingRefV1, field: &str) -> Result<bool, E> {
        self(site, receiver_site, receiver, home, field)
    }
}

/// Source-only sibling with no declared scalar-field proof or writer loan.
pub(crate) struct NoScalarFieldSourceV1;
impl<E> HomeFieldSourceConsultV1<E> for NoScalarFieldSourceV1 {
    fn scalar_field(&mut self, _: &OwnedExprSiteV1, _: &SourceExprSiteV1,
        _: BindingRefV1, _: BindingRefV1, _: &str) -> Result<bool, E> { Ok(false) }
}
