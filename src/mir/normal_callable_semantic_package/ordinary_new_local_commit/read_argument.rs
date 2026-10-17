//! Read-argument dispatch; each read issuer keeps its source and execution law.
use super::*;
use crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::ArrayElementSourceLoanV1;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
#[path = "field_read_argument.rs"]
mod field;
pub(super) use field::interval::IntervalViewV1;
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReadSourceKindV1 {
    Array(OwnedArrayReadSourceLoanV1),
    Field(Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReadActualSourceLoanV1 {
    kind: ReadSourceKindV1,
}
impl ReadActualSourceLoanV1 {
    pub(crate) fn from_array_v1(source: OwnedArrayReadSourceLoanV1) -> Self {
        Self { kind: ReadSourceKindV1::Array(source) }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn from_field_v1(
        source: Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
    ) -> Result<Self, String> {
        if !matches!((source.returned_argument_v1(), source.result()),
            (Some((_, _, loan)), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias(class))
                if loan.class() == Some(class.as_ref())) {
            return Err(freeze("read-argument/field-source-kind"));
        }
        Ok(Self { kind: ReadSourceKindV1::Field(source) })
    }
    /// Only the read-argument emitter borrows this specialized issuer input.
    pub(crate) fn array_source_v1(&self) -> Option<&OwnedArrayReadSourceLoanV1> {
        match &self.kind { ReadSourceKindV1::Array(source) => Some(source), ReadSourceKindV1::Field(_) => None }
    }
    pub(in crate::mir) fn field_parts_v1(&self) -> Option<(
        &Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        BindingRefV1, &crate::mir::resolved_semantics::SourceExprSiteV1,
    )> {
        match &self.kind {
            ReadSourceKindV1::Array(_) => None,
            ReadSourceKindV1::Field(source) => {
                let loan = source.returned_argument_v1().expect("checked Field source").2;
                Some((source, loan.parent(), loan.receiver_site()))
            }
        }
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.site(),
            ReadSourceKindV1::Field(source) => source.returned_argument_v1().expect("checked Field source").2.read(),
        }
    }
    pub(crate) fn class(&self) -> &str {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.class(),
            ReadSourceKindV1::Field(source) => source.returned_argument_v1().expect("checked Field source").2.class()
                .expect("checked Field class"),
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_array_input_v1(
        &self, original: &ArrayElementSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.site() == original.read()
                && source.recipe.original_read_v1(source.site()) == Some(original),
            ReadSourceKindV1::Field(_) => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_field_input_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32,
        original: &super::super::result_class_claim::ReturnedFieldSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Field(source) => source.returned_argument_v1()
                .is_some_and(|(actual_call, actual_ordinal, loan)| actual_call == call && actual_ordinal == ordinal && loan == original),
            ReadSourceKindV1::Array(_) => false,
        }
    }
}

#[derive(Debug)]
enum EmittedReadKindV1 {
    Array(Rc<EmittedOwnedArrayReadV1>),
    Field(field::EmittedFieldReadArgumentV1),
}
/// Retain the original emitted allocation; never reissue an equivalent Get.
#[derive(Debug)]
pub(crate) struct EmittedReadArgumentV1 {
    kind: EmittedReadKindV1,
}
impl EmittedReadArgumentV1 {
    pub(crate) fn from_array_v1(read: Rc<EmittedOwnedArrayReadV1>) -> Self {
        Self { kind: EmittedReadKindV1::Array(read) }
    }
    pub(in crate::mir) fn from_field_v1(
        source: Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        call: OwnedExprSiteV1, receiver: crate::mir::builder::ExactLexicalReadV1,
        read: Binding, context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<Self, String> {
        Ok(Self { kind: EmittedReadKindV1::Field(field::EmittedFieldReadArgumentV1::new(
            source, call, receiver, read, context, function)?) })
    }
    /// The Get holder census still owns the exact original Get Rc identity.
    pub(super) fn array_projection_v1(&self) -> Option<&Rc<EmittedOwnedArrayReadV1>> {
        match &self.kind { EmittedReadKindV1::Array(read) => Some(read), EmittedReadKindV1::Field(_) => None }
    }
    pub(crate) fn producer_bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Array(read) => Box::new(read.producer_bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())) }
    }
    pub(crate) fn bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Array(read) => Box::new(read.bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())) }
    }
    pub(crate) fn value_with_source_v1(
        &self, call: &OwnedExprSiteV1, source: &ReadActualSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        match (&self.kind, &source.kind) {
            (EmittedReadKindV1::Array(read), ReadSourceKindV1::Array(source)) =>
                read.value_with_source_v1(call, source, context.ledger()),
            (EmittedReadKindV1::Field(read), ReadSourceKindV1::Field(source)) => read.value(call, source, context),
            (EmittedReadKindV1::Array(_), ReadSourceKindV1::Field(_))
            | (EmittedReadKindV1::Field(_), ReadSourceKindV1::Array(_)) => Err(freeze("read-argument/emitted-source-kind-drift")),
        }
    }
    pub(super) fn validate_field_anchor_ends_v1(
        &self, call: &OwnedExprSiteV1, source: &ReadActualSourceLoanV1,
        context: CallValidationContextV1<'_>, intervals: &IntervalViewV1<'_>,
        projection: Option<&super::physical_boundary::FinishedBindings>, ends: &[Binding],
    ) -> Result<(), String> {
        if ends.is_empty() { return Ok(()); }
        self.value_with_source_v1(call, source, context)?;
        match (&self.kind, &source.kind) {
            (EmittedReadKindV1::Field(read), ReadSourceKindV1::Field(_)) => read.validate_anchor_ends_v1(context, intervals, projection, ends),
            _ => Err(freeze("field-end/physical-read-kind")),
        }
    }
    /// Called after the enclosing packet's original argument identity check.
    pub(super) fn copy_dependencies_v1(&self, context: CallValidationContextV1<'_>) -> Result<&[Binding], String> {
        match &self.kind {
            EmittedReadKindV1::Array(read) => read.receiver().loan_forwarded_copies(
                read.source().holder(), context.ledger().live_owned_array_read_value_v1(read.source())?),
            EmittedReadKindV1::Field(read) => read.copies(context),
        }
    }
    pub(super) fn validate_finished_v1(
        &self, function: &MirFunction, projection: &super::physical_boundary::FinishedBindings,
        context: CallValidationContextV1<'_>, call: &OwnedExprSiteV1, outer: &Binding,
        coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        match &self.kind {
            EmittedReadKindV1::Array(read) => read.validate_finished_v1(
                function, projection, context.ledger(), call, outer, coordinate, ordinal),
            EmittedReadKindV1::Field(read) => read.validate_finished(
                function, projection, context, call, outer, coordinate, ordinal),
        }
    }
}
