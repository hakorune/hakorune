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
    Declared(crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1),
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
    pub(crate) fn from_declared_v1(read: crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1) -> Self {
        Self { kind: ReadSourceKindV1::Declared(read) }
    }
    pub(crate) fn declared_read_v1(&self) -> Option<&crate::mir::resolved_semantics::home_new_prefix::BorrowedResultReadV1> {
        match &self.kind { ReadSourceKindV1::Declared(read) => Some(read), _ => None }
    }
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
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) | ReadSourceKindV1::Declared(_) => None,
        }
    }
    pub(in crate::mir) fn field_parts_v1(&self) -> Option<(
        &Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        BindingRefV1, &crate::mir::resolved_semantics::SourceExprSiteV1,
    )> {
        match &self.kind {
            ReadSourceKindV1::Array(_) | ReadSourceKindV1::MapField(_) | ReadSourceKindV1::Declared(_) => None,
            ReadSourceKindV1::Field(source) => {
                let loan = source.returned_argument_v1().expect("checked Field source").2;
                Some((source, loan.parent(), loan.receiver_site()))
            }
        }
    }
    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        match &self.kind {
            ReadSourceKindV1::Declared(read) => read.read(),
            ReadSourceKindV1::Array(source) => source.site(),
            ReadSourceKindV1::Field(source) => source.returned_argument_v1().expect("checked Field source").2.read(),
            ReadSourceKindV1::MapField(source) => &source.read,
        }
    }
    pub(crate) fn class(&self) -> &str {
        match &self.kind {
            ReadSourceKindV1::Declared(read) => read.source().declared_read_result_v1().expect("declared read source").declared_type(),
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
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) | ReadSourceKindV1::Declared(_) => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_array_input_v1(
        &self, original: &ArrayElementSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Array(source) => source.site() == original.read()
                && source.recipe.original_read_v1(source.site()) == Some(original),
            ReadSourceKindV1::Field(_) | ReadSourceKindV1::MapField(_) | ReadSourceKindV1::Declared(_) => false,
        }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_field_input_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32,
        original: &super::super::result_class_claim::ReturnedFieldSourceLoanV1,
    ) -> bool {
        match &self.kind {
            ReadSourceKindV1::Field(source) => source.returned_argument_v1()
                .is_some_and(|(actual_call, actual_ordinal, loan)| actual_call == call && actual_ordinal == ordinal && loan == original),
            ReadSourceKindV1::Array(_) | ReadSourceKindV1::MapField(_) | ReadSourceKindV1::Declared(_) => false,
        }
    }
}

#[derive(Debug)]
enum EmittedReadKindV1 {
    Declared { source: ReadActualSourceLoanV1, call: OwnedExprSiteV1, read: Rc<crate::mir::builder::ExactLexicalReadV1>, pair: [Binding; 2] },
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
    pub(in crate::mir) fn from_declared_v1(source: ReadActualSourceLoanV1, call: OwnedExprSiteV1,
        read: Rc<crate::mir::builder::ExactLexicalReadV1>, context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<Self, String> {
        let view = source.declared_read_v1().ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
        if call.owner() != view.read().owner() { return Err(freeze("read-argument/emitted-source-kind-drift")); }
        let (_, pair) = context.declared_result_read_v1(view, &read)?;
        for binding in pair.iter().chain(read.original_copy_dependencies_v1()?) {
            if !super::physical_boundary::check_binding(function, None, binding.0, &binding.1)? {
                return Err(freeze("read-argument/emitted-source-kind-drift"));
            }
        }
        Ok(Self { kind: EmittedReadKindV1::Declared { source, call, read, pair } })
    }
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
        match &self.kind { EmittedReadKindV1::Array(read) => Some(read), EmittedReadKindV1::Field(_) | EmittedReadKindV1::MapField { .. } | EmittedReadKindV1::Declared { .. } => None }
    }
    pub(crate) fn producer_bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Declared { .. } => Box::new(std::iter::empty()),
            EmittedReadKindV1::Array(read) => Box::new(read.producer_bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())),
            EmittedReadKindV1::MapField { pair, .. } => Box::new(pair.iter()) }
    }
    pub(crate) fn bindings(&self) -> Box<dyn Iterator<Item = &Binding> + '_> {
        match &self.kind { EmittedReadKindV1::Declared { .. } => Box::new(std::iter::empty()),
            EmittedReadKindV1::Array(read) => Box::new(read.bindings()),
            EmittedReadKindV1::Field(read) => Box::new(std::iter::once(read.binding())),
            EmittedReadKindV1::MapField { bindings, .. } => Box::new(bindings.iter()) }
    }
    pub(crate) fn value_with_source_v1(
        &self, call: &OwnedExprSiteV1, source: &ReadActualSourceLoanV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        match (&self.kind, &source.kind) {
            (EmittedReadKindV1::Declared { source: retained, call: original, read, pair }, ReadSourceKindV1::Declared(view))
                if retained == source && original == call => {
                let (value, current) = context.declared_result_read_v1(view, read)?;
                if &current != pair { return Err(freeze("read-argument/emitted-source-kind-drift")); }
                Ok(value)
            }
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
            EmittedReadKindV1::Declared { read, .. } => read.original_copy_dependencies_v1(),
        }
    }
    pub(super) fn validate_finished_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction, projection: &super::physical_boundary::FinishedBindings,
        context: CallValidationContextV1<'_>, call: &OwnedExprSiteV1, outer: &Binding,
        coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        match &self.kind {
            EmittedReadKindV1::Declared { source, call: original, read, pair } => {
                if original != call { return Err(freeze("read-argument/emitted-source-kind-drift")); }
                self.validate_declared_finished_v1(source, read, pair, module, function, projection, context, call, outer, coordinate, ordinal)
            }
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

impl EmittedReadArgumentV1 {
    fn validate_declared_finished_v1(&self, source: &ReadActualSourceLoanV1, read: &crate::mir::builder::ExactLexicalReadV1,
        pair: &[Binding; 2], module: &crate::mir::MirModule, function: &MirFunction,
        projection: &super::physical_boundary::FinishedBindings, context: CallValidationContextV1<'_>,
        call: &OwnedExprSiteV1, outer: &Binding, coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        let payload = self.value_with_source_v1(call, source, context)?;
        let view = source.declared_read_v1().ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
        projection.required_effect_binding(function, &pair[0])?;
        projection.relation_binding(function, &pair[1])?.ok_or_else(|| freeze("field-argument/finished-read-removed"))?;
        for copy in read.original_copy_dependencies_v1()? { projection.relation_binding(function, copy)?; }
        let outer = projection.required_effect_binding(function, outer)?;
        if field::interval::locate(function, &outer)? != coordinate
            || !matches!(&outer.1, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }
                if call.args.get(ordinal as usize) == Some(&payload)) { return Err(freeze("field-argument/finished-argument-slot")); }
        // The earliest retained Call supplies the lifetime anchor. Its actual
        // is never substituted for the payload delivered to the new callee.
        let mut base = view.source();
        while let Some(parent) = base.reborrowed_read_v1() { base = parent.source(); }
        let (anchor, start) = context.with_taken_packet_v1(base.call(), |packet| {
            let original = context.ledger().borrowed_nullable_call_source_v1(base.call())
                .ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
            if original.borrowed_nullable() != Some(base) { return Err(freeze("read-argument/emitted-source-kind-drift")); }
            let actual = packet.call_in_context(base.call().owner(), original.arguments(), context)?;
            let value = match base.result_support_v1() {
                crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal) => actual.args.get(ordinal as usize).copied(),
                crate::mir::instruction::BorrowedResultSupportV1::Receiver => actual.callee.receiver(),
                _ => None,
            }.ok_or_else(|| freeze("read-argument/emitted-source-kind-drift"))?;
            Ok((value, packet.outer_bindings()?.1.clone()))
        })?;
        let start = projection.relation_binding(function, &start)?.ok_or_else(|| freeze("field-argument/finished-parent-removed"))?;
        let intervals = IntervalViewV1::new(function);
        intervals.require_before(&start, &outer)?;
        let CallValidationContextV1::Finalized(handoff) = context else { return Err(freeze("field-argument/source-missing")); };
        let mut edges = Vec::new();
        handoff.visit_finalized_call_structure_v1(module, |_, _, packet, _, caller, at, _| {
            let original = packet.original_source();
            edges.push((caller.signature.name.clone(), at, original.target().clone(), original.callee_owner()));
            Ok(())
        })?;
        let mut checked = std::collections::BTreeSet::new();
        validate_readonly_edge_v1(handoff, module, function, coordinate, &edges, &mut checked)?;
        intervals.borrowed_result_v1(anchor, payload)?.check_with_effects(&start, &outer, |at, row| {
            if matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. }) {
                validate_readonly_edge_v1(handoff, module, function, at, &edges, &mut checked)?;
                Ok(false)
            } else { Ok(!readonly_instruction_v1(row)) }
        })
    }
}

fn readonly_instruction_v1(row: &MirInstruction) -> bool {
    matches!(row,
        MirInstruction::Const { .. } | MirInstruction::Copy { .. } | MirInstruction::Phi { .. }
        | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. } | MirInstruction::Branch { .. }
        | MirInstruction::Jump { .. } | MirInstruction::Return { .. } | MirInstruction::ReturnFault { .. }
        | MirInstruction::FaultFrameEnter { .. } | MirInstruction::InvokeNormalResult { .. } | MirInstruction::TypeOp { .. }
        | MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. }
        | MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. } | InvokeOperation::ObjectFieldBorrow { .. }, .. })
}
type ReadCallEdgeV1 = (String, (BasicBlockId, usize), hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, FunctionOwnerIdV1);
fn validate_readonly_edge_v1(handoff: &super::super::FinalizedRootSourceHandoffV1,
    module: &crate::mir::MirModule, caller: &MirFunction, at: (BasicBlockId, usize), edges: &[ReadCallEdgeV1],
    checked: &mut std::collections::BTreeSet<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
) -> Result<(), String> {
    let mut pending = vec![(caller, at)];
    while let Some((caller, at)) = pending.pop() {
        let mut original = edges.iter().filter(|(name, position, _, _)| name == &caller.signature.name && *position == at);
        let (_, _, target, owner) = original.next().ok_or_else(|| freeze("field-argument/source-missing"))?;
        if original.next().is_some() { return Err(freeze("field-argument/source-missing")); }
        let row = caller.blocks.get(&at.0).and_then(|block| block.all_instructions().nth(at.1)).ok_or_else(|| freeze("field-argument/source-missing"))?;
        if !matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } if match &call.callee {
            crate::mir::Callee::SameModuleInstance { key, .. } => key == target,
            crate::mir::Callee::Global(global) => target.canonical_global_target_v1().as_ref() == Ok(global), _ => false,
        }) { return Err(freeze("field-argument/finished-parent-link")); }
        if !checked.insert(target.clone()) { continue; }
        let callee = handoff.ledger.with_finished_projection(*owner, |symbol, _| {
            if module.canonical_callable_definition_symbol(target) != Some(symbol) { return Err(freeze("field-argument/source-missing")); }
            module.functions.get(symbol).ok_or_else(|| freeze("field-argument/source-missing"))
        })?;
        for (block, body) in &callee.blocks {
            for (index, row) in body.all_instructions().enumerate() {
                if matches!(row, MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. }) { pending.push((callee, (*block, index))); }
                else if !readonly_instruction_v1(row) { return Err(freeze("field-argument/parent-or-child-invalidated")); }
            }
        }
    }
    Ok(())
}
