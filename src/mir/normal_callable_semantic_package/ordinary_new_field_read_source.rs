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
    /// One source-qualified checked operation, shared by emission and correspondence.
    pub(in crate::mir) fn checked_operation_v1(
        &self, base: ValueId,
    ) -> Option<(crate::mir::instruction::InvokeOperation, crate::mir::MirType)> {
        use crate::mir::instruction::{InvokeOperation, StringInvokeOperation};
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        use crate::mir::MirType;
        match self.result() {
            LocalFieldReadResultV1::BorrowedObject(child) => Some((
                InvokeOperation::ObjectFieldBorrow { field: self.field, base,
                    child: child.child(), nullable: self.checked_object_nullable_v1() },
                MirType::Box(child.child_class().into()),
            )),
            LocalFieldReadResultV1::Text(text) if text.field() == self.field => Some((
                InvokeOperation::String(StringInvokeOperation::FieldMaterialize { field: self.field, base }),
                MirType::String,
            )),
            LocalFieldReadResultV1::Scalar => Some((
                InvokeOperation::FieldGet { field: self.field, base }, MirType::Integer,
            )),
            _ => None,
        }
    }
    pub(in crate::mir) fn matches_checked_operation_v1(&self, operation: &crate::mir::instruction::InvokeOperation, base: ValueId) -> bool {
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        if matches!(self.result(), LocalFieldReadResultV1::BorrowedObject(child) if child.field() != self.field) {
            return false;
        }
        self.checked_operation_v1(base).is_some_and(|(expected, _)| expected == *operation)
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
    /// Raw representation and its original compatibility-field obligation, if any.
    /// The optional source site is checked by the writer before that projection.
    pub(in crate::mir) fn raw_projection_v1(&self) -> Option<(
        crate::mir::MirType, Option<(FunctionOwnerIdV1, &SourceExprSiteV1, &str)>,
    )> {
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1;
        use crate::mir::MirType;
        if self.requires_checked_field_read_v1() { return None; }
        let (ty, array) = match self.result() {
            LocalFieldReadResultV1::Scalar => (MirType::Integer, None),
            LocalFieldReadResultV1::Alias(class) => (MirType::Box(class.to_string()), None),
            LocalFieldReadResultV1::BorrowedArray(array) => {
                if array.integer_requirement_v1().is_some_and(|source| source.field() != self.field) {
                    return None;
                }
                if array.integer_requirement_v1().is_none() && self.array_reads.is_empty() {
                    return None;
                }
                let origin = self.array_reads.is_empty().then(|| array.integer_requirement_v1())
                    .flatten().map(|source| (source.owner(), source.field_site(), source.field_name()));
                (MirType::Box("ArrayBox".into()), origin)
            }
            LocalFieldReadResultV1::ArrayField(array) => {
                if array.field() != self.field { return None; }
                let origin = (array.text_append_origin_v1().is_none() && self.array_reads.is_empty())
                    .then(|| (array.site().owner(), array.site().site(), array.name()));
                (MirType::Box("ArrayBox".into()), origin)
            }
            _ => return None,
        };
        Some((ty, array))
    }
    pub(super) fn corroborates_instruction(&self, instruction: &MirInstruction) -> bool {
        let Some((ty, array)) = self.raw_projection_v1() else { return false; };
        match (array, instruction) {
            (Some((_, _, name)), MirInstruction::FieldGet { field, declared_type, .. }) =>
                field == name && declared_type.as_ref() == Some(&ty),
            (None, MirInstruction::ObjectFieldGet { field, .. }) => *field == self.field,
            _ => false,
        }
    }
}
