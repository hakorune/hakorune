//! Exact field declaration and source-qualified read operation contract.
use super::*;

/// Immutable declaration descriptor, shared by the original source row and
/// its exact take/emission. Metadata is not a numeric producer/value proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalFieldReadSourceV1 {
    pub(super) field: CanonicalFieldRefV1,
    pub(super) result: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
    pub(super) declared_type_name: Option<Box<str>>,
    pub(super) expression_path: Option<FieldPathSourceV1>,
    pub(super) received: Option<ReceivedFieldReadSourceV1>,
    pub(super) returned: Option<returned::ReturnedFieldArgumentSourceV1>,
    pub(super) owned_ready: Option<OwnedReadyFieldReadSourceV1>,
    pub(super) shared_support: Option<(OwnedExprSiteV1, OwnedExprSiteV1)>,
    pub(super) receiver_return: Option<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ReceiverFieldResultSourceV1>,
    pub(super) array_reads: Vec<Rc<CheckedFieldArrayAccessSourceV1>>,
}
impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_declaration(
        field: CanonicalFieldRefV1,
        result: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1,
        declared_type_name: Option<Box<str>>,
    ) -> Rc<Self> {
        Rc::new(Self { field, result, declared_type_name, expression_path: None, received: None, returned: None, owned_ready: None, shared_support: None, receiver_return: None, array_reads: Vec::new() })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn with_receiver_return_v1(
        original: &Rc<Self>, returned: crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::ReceiverFieldResultSourceV1,
    ) -> Result<Rc<Self>, String> {
        if !matches!(original.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedObject(source)
            if Rc::ptr_eq(source, returned.source())) || original.receiver_return.is_some() {
            return Err(fault("received/field-declaration"));
        }
        let mut source = original.as_ref().clone();
        source.receiver_return = Some(returned);
        Ok(Rc::new(source))
    }
    pub(in crate::mir) fn checked_object_nullable_v1(&self) -> bool {
        self.receiver_return.as_ref().map(|returned| returned.source().nullable()).unwrap_or(true)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn with_received(
        original: &Rc<Self>, received: ReceivedFieldReadSourceV1,
    ) -> Result<Rc<Self>, String> {
        if original.received.is_some() || original.expression_path.is_some() || original.returned.is_some() || original.owned_ready.is_some()
            || original.result != crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar
            || received.mixed().is_some_and(|loan| loan.object() != original.field.object()) {
            return Err(fault("received/field-declaration"));
        }
        let mut source = original.as_ref().clone();
        source.received = Some(received);
        Ok(Rc::new(source))
    }
    pub(in crate::mir) fn requires_checked_field_read_v1(&self) -> bool {
        matches!(self.result, crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedObject(_)
            | crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Text(_))
            || (self.declared_type_name.is_none()
            && self.result == crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar)
            || self.owned_ready.as_ref().is_some_and(|source| source.requires_checked_read_v1()) || self.shared_support.is_some() || self.received.is_some() || self.statement_path().is_some_and(|path| path.requires_checked_scalar_v1())
    }
    pub(in crate::mir) fn requires_exact_receiver_v1(&self) -> bool {
        self.requires_checked_field_read_v1() || self.owned_ready.is_some()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn received_root_source_v1(&self) -> Option<&ReceivedFieldReadSourceV1> {
        self.received.as_ref().or_else(|| self.statement_path().and_then(|path| path.received_root_v1()))
    }
    pub(in crate::mir) fn matches_checked_operation_v1(&self, operation: &crate::mir::instruction::InvokeOperation, base: ValueId) -> bool {
        use crate::mir::instruction::InvokeOperation;
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        match (self.result(), operation) {
            (LocalFieldReadResultV1::BorrowedObject(source), InvokeOperation::ObjectFieldBorrow { field, base: actual, child, nullable }) =>
                *field == self.field && source.field() == self.field && *actual == base && *child == source.child()
                    && *nullable == self.checked_object_nullable_v1(),
            (LocalFieldReadResultV1::Text(source), InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::FieldMaterialize { field, base: actual })) =>
                *field == self.field && source.field() == self.field && *actual == base,
            (LocalFieldReadResultV1::Scalar, InvokeOperation::FieldGet { field, base: actual }) => *field == self.field && *actual == base,
            _ => false,
        }
    }
    pub(crate) fn field(&self) -> CanonicalFieldRefV1 { self.field }
    pub(in crate::mir) fn shared_support_v1(&self) -> Option<&(OwnedExprSiteV1, OwnedExprSiteV1)> {
        self.shared_support.as_ref()
    }
    pub(crate) fn result(&self) -> &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1 {
        &self.result
    }
    pub(crate) fn array_operation_sources_v1(&self) -> &[Rc<CheckedFieldArrayAccessSourceV1>] {
        &self.array_reads
    }
    pub(crate) fn declared_type_name(&self) -> Option<&str> { self.declared_type_name.as_deref() }
    pub(super) fn corroborates_instruction(&self, instruction: &MirInstruction) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        if self.requires_checked_field_read_v1() { return false; }
        match (self.result(), instruction) {
            (LocalFieldReadResultV1::BorrowedArray(source), MirInstruction::FieldGet { field, declared_type: Some(crate::mir::MirType::Box(class)), .. }) =>
                source.integer_requirement_v1().is_some_and(|source| source.field() == self.field() && source.field_name() == field && class == "ArrayBox"),
            (LocalFieldReadResultV1::BorrowedArray(_), MirInstruction::ObjectFieldGet { field, .. }) =>
                !self.array_reads.is_empty() && *field == self.field(),
            (LocalFieldReadResultV1::BorrowedArray(_), _) => false,
            (LocalFieldReadResultV1::ArrayField(source), MirInstruction::ObjectFieldGet { field, .. })
                if source.text_append_origin_v1().is_some() || !self.array_reads.is_empty() => source.field() == self.field() && *field == self.field(),
            (LocalFieldReadResultV1::ArrayField(source), MirInstruction::FieldGet { field, declared_type: Some(crate::mir::MirType::Box(class)), .. }) =>
                source.text_append_origin_v1().is_none() && source.field() == self.field() && source.name() == field && class == "ArrayBox",
            (LocalFieldReadResultV1::ArrayField(_), _) => false,
            (_, MirInstruction::ObjectFieldGet { field, .. }) => *field == self.field(),
            _ => false,
        }
    }
}
