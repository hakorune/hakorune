//! One source consult for field reads and original field-write plans.
//! Borrowing a plan grants no FieldSet, Home or scalar operation permission.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
use std::rc::Rc;

pub(crate) trait HomeFieldSourceConsultV1<E> {
    /// Exact selected New/destination source; retention grants no field value.
    fn construction_source(
        &mut self,
        _site: &OwnedExprSiteV1,
        _destination: BindingRefV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>>,
        E,
    > {
        Ok(None)
    }
    /// Exact destination-less Shared payload construction; no local New is invented.
    fn shared_payload_construction_source(
        &mut self, _site: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>>, E> {
        Ok(None)
    }
    fn scalar_field(
        &mut self,
        _site: &OwnedExprSiteV1,
        _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1,
        _home: BindingRefV1,
        _field: &str,
    ) -> Result<bool, E> { Ok(false) }
    fn argument_i64_field(&mut self, _site: &OwnedExprSiteV1, _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1, _home: BindingRefV1, _field: &str,
    ) -> Result<bool, E> { Ok(false) }
    fn container_field(&mut self, _site: &OwnedExprSiteV1, _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1, _home: BindingRefV1, _field: &str,
    ) -> Result<bool, E> { Ok(false) }
    fn array_i64_field(&mut self, _site: &OwnedExprSiteV1, _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1, _home: BindingRefV1, _field: &str,
    ) -> Result<bool, E> { Ok(false) }
    fn formal_i64_field(&mut self, _site: &OwnedExprSiteV1, _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1, _home: BindingRefV1, _field: &str,
    ) -> Result<bool, E> { Ok(false) }
    fn local_field_read(&mut self, _requests: &[LocalFieldReadRequestV1], _demand: LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E> { Ok(None) }
    fn owned_local_write(
        &mut self,
        _input: ResolvedFunctionLoweringInputV1<'_>,
        _acquisition: &OwnedExprSiteV1,
        _root: BindingRefV1,
        _construction: &Rc<
            crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1,
        >,
        _target: &SourceExprSiteV1,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1>, E>
    {
        Ok(None)
    }
    fn entry_write(
        &mut self,
        _input: ResolvedFunctionLoweringInputV1<'_>,
        _target: &SourceExprSiteV1,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1>, E>
    {
        Ok(None)
    }
    fn array_text_append_source(
        &mut self,
        _input: ResolvedFunctionLoweringInputV1<'_>,
        _site: &SourceExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
        E,
    > {
        Ok(None)
    }
    /// Full-use input families retain scalar alias reads in the shared frame.
    /// This selects responsibility, never executable completion or scalar type.
    fn requires_initializer_use_frame(&mut self, _site: &OwnedExprSiteV1) -> Result<bool, E> {
        Ok(false)
    }
    fn scalar_read_sum(
        &mut self,
        _read: &CurrentScalarReadV1,
    ) -> Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>> {
        None
    }
    /// Borrow the same original entry read for an Assignment RHS. No view or
    /// executable carrier is issued by this source-only consultation.
    fn finite_entry_read(
        &mut self,
        _input: ResolvedFunctionLoweringInputV1<'_>,
        _plan: &Rc<ExpressionSourcePlanV1>,
        _read: &SourceExprSiteV1,
        _binding: BindingRefV1,
        _root: BindingRefV1,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>, E> {
        Ok(None)
    }
    /// Preserve the exact original writer inventory for owned LOCAL stores.
    fn original_writer(
        &mut self,
        _owner: FunctionOwnerIdV1,
        _target: &SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OriginalFieldWriteSourceLoanV1>,
        E,
    > {
        Ok(None)
    }
    fn assignment_source(
        &mut self,
        _owner: FunctionOwnerIdV1,
        _target: &SourceExprSiteV1,
    ) -> Result<Option<Rc<ExpressionSourcePlanV1>>, E> {
        Ok(None)
    }
}

// Existing bounded test/source batches retain only their local-field query;
// construction, scalar, argument and writer proofs remain unavailable.
impl<E, F> HomeFieldSourceConsultV1<E> for F
where
    F: FnMut(&[LocalFieldReadRequestV1], LocalFieldReadDemandV1)
        -> Result<Option<Vec<LocalFieldReadResultV1>>, E>,
{
    fn local_field_read(&mut self, requests: &[LocalFieldReadRequestV1], demand: LocalFieldReadDemandV1,
    ) -> Result<Option<Vec<LocalFieldReadResultV1>>, E> { self(requests, demand) }
}

/// Source-only sibling with no declared scalar-field proof or writer loan.
pub(crate) struct NoScalarFieldSourceV1;
impl<E> HomeFieldSourceConsultV1<E> for NoScalarFieldSourceV1 {
    fn scalar_field(
        &mut self,
        _: &OwnedExprSiteV1,
        _: &SourceExprSiteV1,
        _: BindingRefV1,
        _: BindingRefV1,
        _: &str,
    ) -> Result<bool, E> {
        Ok(false)
    }
}
