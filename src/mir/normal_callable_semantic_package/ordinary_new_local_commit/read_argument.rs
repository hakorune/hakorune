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
    /// Original Map-held payload field read, without any live-borrow authority.
    MapField(MapFieldSourceV1),
}
#[derive(Debug, Clone)]
struct MapFieldSourceV1 {
    read: OwnedExprSiteV1,
    class: Box<str>,
    get: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
    field: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1>,
}
impl PartialEq for MapFieldSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read && self.class == other.class && self.get == other.get
            && Rc::ptr_eq(&self.field, &other.field)
    }
}
impl Eq for MapFieldSourceV1 {}
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
    pub(in crate::mir::normal_callable_semantic_package) fn from_map_field_v1(
        read: OwnedExprSiteV1, class: Box<str>,
        get: crate::mir::normal_callable_semantic_package::StoredMapGetSourceLoanV1,
        field: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1>,
    ) -> Self {
        Self { kind: ReadSourceKindV1::MapField(MapFieldSourceV1 { read, class, get, field }) }
    }
    pub(crate) fn is_map_field_v1(&self) -> bool {
        matches!(self.kind, ReadSourceKindV1::MapField(_))
    }
    /// Only the read-argument emitter borrows this specialized issuer input.
    pub(crate) fn array_source_v1(&self) -> Option<&OwnedArrayReadSourceLoanV1> {
        match &self.kind {
            ReadSourceKindV1::Array(source) => Some(source),
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) => None,
        }
    }
    pub(in crate::mir) fn field_parts_v1(&self) -> Option<(
        &Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        BindingRefV1, &crate::mir::resolved_semantics::SourceExprSiteV1,
    )> {
        match &self.kind {
            ReadSourceKindV1::Array(_) | ReadSourceKindV1::MapField(_) => None,
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
            ReadSourceKindV1::MapField(source) => &source.read,
        }
    }
    pub(crate) fn class(&self) -> &str {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.class(),
            ReadSourceKindV1::Field(source) => source.returned_argument_v1().expect("checked Field source").2.class()
                .expect("checked Field class"),
            ReadSourceKindV1::MapField(source) => &source.class,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_method_source_v1(
        &self, method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.recipe.original_read_v1(source.site())
                .is_some_and(|original| original.corroborates_method_source_v1(method)),
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_array_input_v1(
        &self, original: &ArrayElementSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.site() == original.read()
                && source.recipe.original_read_v1(source.site()) == Some(original),
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_field_input_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32,
        original: &super::super::result_class_claim::ReturnedFieldSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Field(source) => source.returned_argument_v1()
                .is_some_and(|(actual_call, actual_ordinal, loan)| actual_call == call && actual_ordinal == ordinal && loan == original),
            ReadSourceKindV1::Array(_) | ReadSourceKindV1::MapField(_) => false,
        }
    }
}

#[derive(Debug)]
enum EmittedReadKindV1 {
    Array(Rc<EmittedOwnedArrayReadV1>),
    Field(field::EmittedFieldReadArgumentV1),
    MapField { source: ReadActualSourceLoanV1, call: OwnedExprSiteV1, receiver: Rc<crate::mir::builder::ExactLexicalReadV1>, pair: [Binding; 2], bindings: Vec<Binding> },
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
    pub(in crate::mir) fn from_map_field_v1(source: ReadActualSourceLoanV1, call: OwnedExprSiteV1,
        receiver: Rc<crate::mir::builder::ExactLexicalReadV1>, context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<Self, String> {
        let ReadSourceKindV1::MapField(original) = &source.kind else { return Err(freeze("read-argument/emitted-source-kind-drift")); };
        let ledger = context.ledger();
        let pair = ledger.checked_field_pair_v1(source.site(), context)?.ok_or_else(|| freeze("field-argument/source-missing"))?;
        let (frame, exclusion) = ledger.stored_get_field_root_source_v1(source.site(), &receiver, context)?
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        if frame.source() != &original.get { return Err(freeze("field-argument/original-source-identity")); }
        context.validate_stored_get_nonnull_read_v1(&frame, &exclusion, &receiver, function, None, &pair[0])?;
        let mut bindings = ledger.checked_field_normal_prefix_v1(source.site())?;
        bindings.extend(pair.clone());
        bindings.extend(ledger.checked_field_cleanup_bindings_v1(source.site())?);
        let mut unique = Vec::new();
        for row in bindings { if !unique.contains(&row) { unique.push(row); } }
        Ok(Self { kind: EmittedReadKindV1::MapField { source, call, receiver, pair, bindings: unique } })
    }
    /// The Get holder census still owns the exact original Get Rc identity.
    pub(super) fn array_projection_v1(&self) -> Option<&Rc<EmittedOwnedArrayReadV1>> {
        match &self.kind { EmittedReadKindV1::Array(read) => Some(read), EmittedReadKindV1::Field(_) | EmittedReadKindV1::MapField { .. } => None }
    }
    pub(crate) fn producer_bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Array(read) => Box::new(read.producer_bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())),
            EmittedReadKindV1::MapField { pair, .. } => Box::new(pair.iter()) }
    }
    pub(crate) fn bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Array(read) => Box::new(read.bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())),
            EmittedReadKindV1::MapField { bindings, .. } => Box::new(bindings.iter()) }
    }
    pub(crate) fn value_with_source_v1(
        &self, call: &OwnedExprSiteV1, source: &ReadActualSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        match (&self.kind, &source.kind) {
            (EmittedReadKindV1::Array(read), ReadSourceKindV1::Array(source)) =>
                read.value_with_source_v1(call, source, context.ledger()),
            (EmittedReadKindV1::Field(read), ReadSourceKindV1::Field(source)) => read.value(call, source, context),
            (EmittedReadKindV1::MapField { source: retained, call: original_call, pair, .. }, ReadSourceKindV1::MapField(_))
                if retained == source && original_call == call => {
                if context.ledger().checked_field_pair_v1(source.site(), context)?.as_ref() != Some(pair) {
                    return Err(freeze("field-argument/original-producer-drift"));
                }
                match pair[1].1 { MirInstruction::InvokeNormalResult { dst, .. } => Ok(dst),
                    _ => Err(freeze("field-argument/read-operands")) }
            }
            _ => Err(freeze("read-argument/emitted-source-kind-drift")),
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
            EmittedReadKindV1::MapField { receiver, .. } => receiver.original_copy_dependencies_v1(),
        }
    }
    pub(super) fn validate_finished_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction, projection: &super::physical_boundary::FinishedBindings,
        context: CallValidationContextV1<'_>, call: &OwnedExprSiteV1, outer: &Binding,
        coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        match &self.kind {
            EmittedReadKindV1::Array(read) => read.validate_finished_v1(
                function, projection, context.ledger(), call, outer, coordinate, ordinal),
            EmittedReadKindV1::Field(read) => read.validate_finished(
                function, projection, context, call, outer, coordinate, ordinal),
            EmittedReadKindV1::MapField { source, call: original_call, receiver, pair, .. } => {
                if original_call != call { return Err(freeze("field-argument/call-identity")); }
                field::validate_map_field_v1(self, source, receiver, pair, module, function,
                    projection, context, call, outer, coordinate, ordinal)
            },
        }
    }
}
