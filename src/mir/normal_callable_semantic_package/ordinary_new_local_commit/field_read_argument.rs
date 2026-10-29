//! Retain the sole ObjectFieldGet and its original parent packet through finish.
use super::*;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::normal_callable_semantic_package::{CallValidationContextV1, LocalFieldReadSourceV1};
#[path = "field_read_argument_interval.rs"]
pub(super) mod interval;

#[derive(Debug)]
pub(super) struct EmittedFieldReadArgumentV1 {
    source: Rc<LocalFieldReadSourceV1>,
    call: OwnedExprSiteV1,
    receiver: ExactLexicalReadV1,
    read: Binding,
    parent_invoke: Binding,
    parent_normal: Binding,
}
impl EmittedFieldReadArgumentV1 {
    pub(super) fn new(
        source: Rc<LocalFieldReadSourceV1>, call: OwnedExprSiteV1,
        receiver: ExactLexicalReadV1, read: Binding,
        context: CallValidationContextV1<'_>, function: &MirFunction,
    ) -> Result<Self, String> {
        let (_, parent_invoke, parent_normal) = context.ledger().returned_field_argument_parent_v1(&source, context)?;
        let result = Self { source, call, receiver, read, parent_invoke, parent_normal };
        let child = result.value(&result.call, &result.source, context)?;
        let parent = result.copies(context).and_then(|_| {
            context.ledger().returned_field_argument_parent_v1(&result.source, context).map(|row| row.0)
        })?;
        let intervals = interval::IntervalViewV1::new(function);
        intervals.require_before(&result.parent_invoke, &result.parent_normal)?;
        intervals.require_before(&result.parent_normal, &result.read)?;
        intervals.read(parent, child, result.source.field())?.check(&result.parent_normal, &result.read)?;
        Ok(result)
    }
    pub(super) fn binding(&self) -> &Binding { &self.read }
    pub(super) fn value(
        &self, call: &OwnedExprSiteV1, source: &Rc<LocalFieldReadSourceV1>,
        context: CallValidationContextV1<'_>,
    ) -> Result<ValueId, String> {
        if &self.call != call || !Rc::ptr_eq(source, &self.source) {
            return Err(freeze("field-argument/original-source-identity"));
        }
        let (original_call, _, loan) = source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        if original_call != call { return Err(freeze("field-argument/call-identity")); }
        let (parent, invoke, normal) = context.ledger().returned_field_argument_parent_v1(source, context)?;
        if invoke != self.parent_invoke || normal != self.parent_normal
            || context.ledger().returned_field_argument_binding_v1(source)? != self.read {
            return Err(freeze("field-argument/original-producer-drift"));
        }
        let base = self.receiver.value_for(call.owner(), loan.receiver_site().node(), loan.parent())
            .map_err(|issue| issue.to_string())?;
        self.receiver.loan_forwarded_copies(loan.parent(), parent)?;
        match self.read.1 {
            MirInstruction::ObjectFieldGet { dst, base: actual, field }
                if actual == base && field == source.field() && dst != base => Ok(dst),
            _ => Err(freeze("field-argument/read-operands")),
        }
    }
    pub(super) fn copies(&self, context: CallValidationContextV1<'_>) -> Result<&[Binding], String> {
        let (_, _, loan) = self.source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        self.receiver.loan_forwarded_copies(loan.parent(), parent)
    }
    pub(super) fn validate_anchor_ends_v1(
        &self, context: CallValidationContextV1<'_>, intervals: &interval::IntervalViewV1<'_>,
        projection: Option<&super::super::physical_boundary::FinishedBindings>, ends: &[Binding],
    ) -> Result<(), String> {
        if ends.is_empty() { return Ok(()); }
        let function = intervals.function();
        let child = self.value(&self.call, &self.source, context)?;
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        let normal = match projection {
            Some(projection) => projection.relation_binding(function, &self.parent_normal)?
                .ok_or_else(|| freeze("field-end/finished-parent-missing"))?,
            None => self.parent_normal.clone(),
        };
        let read = intervals.read(parent, child, self.source.field())?;
        for end in ends {
            intervals.require_before(&normal, end)?;
            read.check(&normal, end)?;
        }
        Ok(())
    }
    pub(super) fn validate_finished(
        &self, function: &MirFunction, projection: &super::super::physical_boundary::FinishedBindings,
        context: CallValidationContextV1<'_>, call: &OwnedExprSiteV1, outer: &Binding,
        coordinate: (BasicBlockId, usize), ordinal: u32,
    ) -> Result<(), String> {
        let child = self.value(call, &self.source, context)?;
        let (_, expected_ordinal, _) = self.source.returned_argument_v1()
            .ok_or_else(|| freeze("field-argument/source-missing"))?;
        if expected_ordinal != ordinal { return Err(freeze("field-argument/ordinal-drift")); }
        let map = |original: &Binding| -> Result<Binding, String> {
            let mapped = projection.binding(original.0, &original.1)?
                .ok_or_else(|| freeze("field-argument/finished-producer-removed"))?;
            if !projection.recorded().contains(&mapped) {
                return Err(freeze("field-argument/finished-unrecorded"));
            }
            Ok(mapped)
        };
        let parent_invoke = map(&self.parent_invoke)?;
        let parent_normal = projection.relation_binding(function, &self.parent_normal)?
            .ok_or_else(|| freeze("field-argument/finished-parent-removed"))?;
        let read = projection.relation_binding(function, &self.read)?
            .ok_or_else(|| freeze("field-argument/finished-read-removed"))?;
        let outer = map(outer)?;
        if interval::locate(function, &outer)? != coordinate {
            return Err(freeze("field-argument/finished-call-coordinate"));
        }
        let MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } = &outer.1 else {
            return Err(freeze("field-argument/finished-call-missing"));
        };
        if call.args.get(ordinal as usize) != Some(&child) {
            return Err(freeze("field-argument/finished-argument-slot"));
        }
        let parent = context.ledger().returned_field_argument_parent_v1(&self.source, context)?.0;
        if !matches!((&parent_invoke.1, &parent_normal.1),
            (MirInstruction::Invoke { normal_landing, operation: InvokeOperation::Call { result: crate::mir::instruction::InvokeCallResultKind::Handle, .. }, .. },
             MirInstruction::InvokeNormalResult { invoke_block, dst })
                if *invoke_block == parent_invoke.0 && *normal_landing == parent_normal.0 && *dst == parent)
        { return Err(freeze("field-argument/finished-parent-link")); }
        let intervals = interval::IntervalViewV1::new(function);
        intervals.require_before(&parent_invoke, &parent_normal)?;
        intervals.require_before(&parent_normal, &read)?;
        intervals.require_before(&read, &outer)?;
        // Copy omission is legal only under the captured boundary's permission.
        for copy in self.copies(context)? { projection.relation_binding(function, copy)?; }
        intervals.read(parent, child, self.source.field())?.check(&parent_normal, &outer)
    }
}

/// A Map payload keeps its slot support; it is never a returned owned parent.
pub(super) fn validate_map_field_v1(
    emitted: &EmittedReadArgumentV1, source: &ReadActualSourceLoanV1, receiver: &ExactLexicalReadV1,
    pair: &[Binding; 2], module: &crate::mir::MirModule, function: &MirFunction,
    projection: &super::super::physical_boundary::FinishedBindings, context: CallValidationContextV1<'_>,
    call: &OwnedExprSiteV1, outer: &Binding, coordinate: (BasicBlockId, usize), ordinal: u32,
) -> Result<(), String> {
    let ReadSourceKindV1::MapField(original) = &source.kind else { return Err(freeze("field-argument/source-missing")); };
    let child = emitted.value_with_source_v1(call, source, context)?;
    let (frame, exclusion) = context.ledger().stored_get_field_root_source_v1(source.site(), receiver, context)?
        .ok_or_else(|| freeze("field-argument/source-missing"))?;
    if frame.source() != &original.get { return Err(freeze("field-argument/original-source-identity")); }
    context.validate_stored_get_nonnull_read_v1(&frame, &exclusion, receiver, function, Some(projection), &pair[0])?;
    let (_, get) = context.stored_map_get_read_v1(&frame, receiver, function, Some(projection))?;
    let get_invoke = projection.required_effect_binding(function, &get[0])?;
    let get_normal = projection.relation_binding(function, &get[1])?
        .ok_or_else(|| freeze("field-argument/finished-parent-removed"))?;
    let read = projection.required_effect_binding(function, &pair[0])?;
    let normal = projection.relation_binding(function, &pair[1])?
        .ok_or_else(|| freeze("field-argument/finished-read-removed"))?;
    let outer = projection.required_effect_binding(function, outer)?;
    if interval::locate(function, &outer)? != coordinate { return Err(freeze("field-argument/finished-call-coordinate")); }
    let MirInstruction::Invoke { operation: InvokeOperation::Call { call: actual, .. }, .. } = &outer.1 else {
        return Err(freeze("field-argument/finished-call-missing"));
    };
    if actual.args.get(ordinal as usize) != Some(&child) { return Err(freeze("field-argument/finished-argument-slot")); }
    let MirInstruction::InvokeNormalResult { dst: parent, .. } = get_normal.1 else { return Err(freeze("field-argument/finished-parent-link")); };
    if !matches!((&read.1, &normal.1), (MirInstruction::Invoke { operation: InvokeOperation::ObjectFieldBorrow { field, .. }, normal_landing, .. },
        MirInstruction::InvokeNormalResult { invoke_block, dst })
        if *field == original.field.field() && *invoke_block == read.0 && *normal_landing == normal.0 && *dst == child) {
        return Err(freeze("field-argument/read-operands"));
    }
    for copy in emitted.copy_dependencies_v1(context)? { projection.relation_binding(function, copy)?; }
    let intervals = interval::IntervalViewV1::new(function);
    for (before, after) in [(&get_invoke, &get_normal), (&get_normal, &read), (&read, &normal), (&normal, &outer)] {
        intervals.require_before(before, after)?;
    }
    let CallValidationContextV1::Finalized(view) = context else { return Err(freeze("field-argument/source-missing")); };
    validate_map_completion_effects_v1(view, module, function, coordinate,
        original.get.receiver().field(), original.field.field())?;
    let mut calls = std::collections::BTreeSet::new();
    context.with_local_call_group_tree_v2(call.owner(), |groups| {
        for group in groups {
            for packet in group.lexical_packets() {
                let Ok(row) = packet.original_row() else { continue; };
                let Some(loan) = row.source_target().direct_map_receiver_source_v1() else { continue; };
                if loan.source() != frame.source() || loan.binding() != frame.binding() { continue; }
                context.ledger().check_direct_map_numeric_effects_v1(row.source_target())?;
                let expected = packet.call_in_context(call.owner(), &[], context)?;
                let invoke = projection.required_effect_binding(function, packet.invoke_binding())?;
                if !matches!(&invoke.1, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } if call == &expected) {
                    return Err(freeze("field-argument/finished-parent-link"));
                }
                let CallValidationContextV1::Finalized(view) = context else { return Err(freeze("field-argument/source-missing")); };
                view.ledger.with_finished_projection(row.callee_owner(), |symbol, _| {
                    if symbol != row.target().mir_symbol_projection() { return Err(freeze("field-argument/source-missing")); }
                    let callee = module.functions.get(symbol).ok_or_else(|| freeze("field-argument/source-missing"))?;
                    validate_map_scalar_callee_v1(module, callee, original.field.field())
                })?;
                calls.insert(interval::locate(function, &invoke)?);
            }
        }
        Ok(())
    })?;
    intervals.read(parent, child, original.field.field())?.check_with_effects(&get_normal, &outer, |at, instruction| {
        use crate::mir::instruction::MapInvokeOperation as Map;
        Ok(match instruction {
            MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. } => !calls.contains(&at),
            MirInstruction::Invoke { operation: InvokeOperation::Map(map), .. } => !matches!(map,
                Map::FieldGetIndexed { .. } | Map::FieldHas { .. } | Map::FieldKeys { .. }
                    | Map::CheckedGetI64 { .. } | Map::ArrayLength { .. } | Map::ArrayIndexMap { .. }
                    | Map::MapGetText { .. } | Map::New | Map::PrepareKey { .. }),
            MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }
                | InvokeOperation::ObjectFieldBorrow { .. } | InvokeOperation::String(_), .. }
            | MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. } | MirInstruction::Const { .. } | MirInstruction::Copy { .. }
            | MirInstruction::Phi { .. } | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. }
            | MirInstruction::Branch { .. } | MirInstruction::Jump { .. } | MirInstruction::Return { .. }
            | MirInstruction::InvokeNormalResult { .. } | MirInstruction::FaultFrameEnter { .. }
            | MirInstruction::ReturnFault { .. } | MirInstruction::TypeOp { .. } => false,
            _ => true,
        })
    })
}

/// Recheck the source-approved numeric receiver effects on the actual callee.
/// Neither a method name nor an advertised effect mask authorizes this body.
fn validate_map_scalar_callee_v1(module: &crate::mir::MirModule, function: &MirFunction, protected: hakorune_mir_defs::CanonicalFieldRefV1) -> Result<(), String> {
    // A named field access is resolved only through the declaration of the
    // protected field's own object, so a bare name never selects another Box.
    let named_ordinal = |name: &str| -> Option<usize> {
        let definition = module.canonical_object_definitions()?.get(protected.object().declaration_index() as usize)?;
        let mut rows = definition.fields().iter().enumerate().filter(|(_, field)| field.name == name && !field.is_weak);
        let (ordinal, _) = rows.next()?;
        rows.next().is_none().then_some(ordinal)
    };
    let receiver = *function.params.first().ok_or_else(|| freeze("field-argument/source-missing"))?;
    let origins = crate::mir::value_origin::AcyclicUnanimousValueOrigins::new(function);
    for instruction in function.blocks.values().flat_map(|block| block.all_instructions()) {
        let allowed = match instruction {
            MirInstruction::Invoke { operation: InvokeOperation::FieldGet { base, field }, .. }
            | MirInstruction::ObjectFieldGet { base, field, .. } => origins.origin(*base) == Some(receiver) && field.object() == protected.object(),
            MirInstruction::Invoke { operation: InvokeOperation::FieldSet { base, field, mode, .. }, .. } =>
                origins.origin(*base) == Some(receiver) && field.object() == protected.object() && *field != protected
                    && matches!(mode, crate::mir::instruction::FieldSetModeV1::ExactSlot | crate::mir::instruction::FieldSetModeV1::DynamicI64Replace),
            MirInstruction::FieldGet { base, field, .. } =>
                origins.origin(*base) == Some(receiver) && named_ordinal(field).is_some(),
            MirInstruction::FieldSet { base, field, declared_type: Some(crate::mir::MirType::Integer), .. } =>
                origins.origin(*base) == Some(receiver)
                    && named_ordinal(field).is_some_and(|ordinal| ordinal as u32 != protected.declaration_ordinal()),
            MirInstruction::Const { .. } | MirInstruction::Copy { .. } | MirInstruction::Phi { .. }
            | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. } | MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. } | MirInstruction::Return { .. } | MirInstruction::InvokeNormalResult { .. }
            | MirInstruction::FaultFrameEnter { .. } | MirInstruction::ReturnFault { .. } | MirInstruction::TypeOp { .. } => true,
            _ => false,
        };
        if !allowed { return Err(format!("{} callee={} instruction={instruction:?}", freeze("field-argument/parent-or-child-invalidated"), function.signature.name)); }
    }
    Ok(())
}

/// Close every source-backed callee edge before granting the outer borrow.
/// The worklist accepts cycles only after every reachable body has been checked.
fn validate_map_completion_effects_v1(
    view: &super::super::FinalizedRootSourceHandoffV1, module: &crate::mir::MirModule,
    caller: &MirFunction, at: (BasicBlockId, usize),
    map_field: hakorune_mir_defs::CanonicalFieldRefV1, child_field: hakorune_mir_defs::CanonicalFieldRefV1,
) -> Result<(), String> {
    let mut edges = Vec::new();
    view.visit_finalized_call_structure_v1(module, |_, _, packet, _, function, at, _| {
        let source = packet.original_source();
        edges.push((function.signature.name.clone(), at, source.target().clone(), source.callee_owner()));
        Ok(())
    })?;
    let mut root = edges.iter().filter(|(name, coordinate, _, _)| name == &caller.signature.name && *coordinate == at);
    let (_, _, target, owner) = root.next().ok_or_else(|| freeze("field-argument/source-missing"))?;
    if root.next().is_some() { return Err(freeze("field-argument/source-missing")); }
    let mut pending = vec![(target.clone(), *owner)];
    let mut checked = std::collections::BTreeSet::new();
    while let Some((target, owner)) = pending.pop() {
        if checked.contains(&target) { continue; }
        view.ledger.with_finished_projection(owner, |symbol, _| {
            if symbol != target.mir_symbol_projection() { return Err(freeze("field-argument/source-missing")); }
            let function = module.functions.get(symbol).ok_or_else(|| freeze("field-argument/source-missing"))?;
            let origins = crate::mir::value_origin::AcyclicUnanimousValueOrigins::new(function);
            let me_value = function.params.first().copied();
            let def_map = crate::mir::value_origin::build_value_def_map(function);
            let box_name = symbol.split('.').next().unwrap_or("");
            // The array a write targets is either the field read itself or the
            // Normal result of that read; both name the same declared field.
            let array_owner_target = |value: crate::mir::ValueId| -> Option<ArrayFieldTargetV1> {
                let (block, index) = *def_map.get(&value)?;
                let instruction = function.blocks.get(&block)?.instructions.get(index)?;
                match instruction {
                    MirInstruction::ObjectFieldGet { base, field, .. }
                    | MirInstruction::Invoke { operation: InvokeOperation::FieldGet { base, field }, .. } =>
                        Some(ArrayFieldTargetV1::Canonical { base: *base, field: *field }),
                    MirInstruction::FieldGet { base, field, .. } =>
                        Some(ArrayFieldTargetV1::Named { base: *base, name: field.clone().into() }),
                    MirInstruction::InvokeNormalResult { invoke_block, .. } => {
                        let block = function.blocks.get(invoke_block)?;
                        let mut rows = block.instructions.iter().filter_map(|row| match row {
                            MirInstruction::Invoke { operation: InvokeOperation::FieldGet { base, field }, .. } =>
                                Some(ArrayFieldTargetV1::Canonical { base: *base, field: *field }),
                            MirInstruction::FieldGet { base, field, .. } =>
                                Some(ArrayFieldTargetV1::Named { base: *base, name: field.clone().into() }),
                            _ => None,
                        });
                        let first = rows.next()?;
                        rows.next().is_none().then_some(first)
                    }
                    _ => None,
                }
            };
            for (block, body) in &function.blocks {
                for (index, instruction) in body.all_instructions().enumerate() {
                    let allowed = match instruction {
                        MirInstruction::Invoke { operation: InvokeOperation::Call { .. }, .. } => {
                            let mut matching = edges.iter().filter(|(name, at, _, _)| name == symbol && *at == (*block, index));
                            let (_, _, callee, owner) = matching.next().ok_or_else(|| freeze("field-argument/source-missing"))?;
                            if matching.next().is_some() { return Err(freeze("field-argument/source-missing")); }
                            pending.push((callee.clone(), *owner));
                            true
                        }
                        MirInstruction::Invoke { operation: InvokeOperation::FieldSet { field, mode, .. }, .. } =>
                            *field != map_field && *field != child_field
                                && matches!(mode, crate::mir::instruction::FieldSetModeV1::ExactSlot
                                    | crate::mir::instruction::FieldSetModeV1::DynamicI64Replace)
                                && module.canonical_field_definition(*field).and_then(|field| field.declared_type_name.as_deref())
                                    .is_some_and(crate::mir::numeric_substrate::is_numeric_integer_type_name),
                        // The named form of the same integer slot write: the field is
                        // resolved through the declaration of the callee's own Box and
                        // must be a declared-integer slot of `me`, never the protected ones.
                        MirInstruction::FieldSet { base, field, declared_type: Some(crate::mir::MirType::Integer), .. } =>
                            me_value.is_some_and(|me| {
                                ArrayFieldTargetV1::Named { base: *base, name: field.clone().into() }
                                    .resolve(module, box_name)
                                    .is_some_and(|(base, field)| origins.origin(base) == Some(me)
                                        && field != map_field && field != child_field
                                        && module.canonical_field_definition(field)
                                            .and_then(|definition| definition.declared_type_name.as_deref())
                                            .is_some_and(crate::mir::numeric_substrate::is_numeric_integer_type_name))
                            }),
                        // A Set on an integer-array field of this same `me`: the
                        // field census owns the all-writer integer proof, the origin
                        // owns the receiver, and the numeric facts own index/value.
                        MirInstruction::ArrayElementWrite {
                            kind: crate::mir::ArrayElementWriteKind::Set,
                            producer: crate::mir::ArrayWriteProducerKind::MethodCall,
                            receiver,
                            index: Some(index),
                            value,
                            ..
                        } => {
                            let numeric = |value: crate::mir::ValueId| {
                                function.metadata.exact_numeric_value_facts.contains_key(&value)
                                    || function.metadata.value_types.get(&value)
                                        == Some(&crate::mir::MirType::Integer)
                            };
                            let allowed = me_value.is_some_and(|me| {
                                array_owner_target(*receiver)
                                    .and_then(|row| {
                                        let (base, field) = row.resolve(module, box_name)?;
                                        (origins.origin(base) == Some(me)).then_some(field)
                                    })
                                    .is_some_and(|field| {
                                        let census = view.ledger
                                            .array_i64_fields
                                            .get(box_name)
                                            .is_some_and(|fields| fields.contains(&field));
                                        // Otherwise the same all-writer solve must close the
                                        // element class to signed integers (formals included).
                                        let solved = || view.ledger.borrowed_formal_source.as_ref()
                                            .and_then(|source| source.as_ref().ok())
                                            .and_then(|source| source.source_array_element_sum_v1(&field))
                                            .is_some_and(|sum| sum.signed_only());
                                        (census || solved())
                                            && numeric(*index)
                                            && numeric(*value)
                                    })
                            });
                            if !allowed && crate::config::env::builder_debug_enabled() {
                                let raw_def = def_map.get(receiver).and_then(|(block, index)|
                                    function.blocks.get(block).and_then(|body| body.instructions.get(*index)))
                                    .map(|instruction| {
                                        let text = format!("{instruction:?}");
                                        text.chars().take(120).collect::<String>()
                                    });
                                eprintln!(
                                    "[field-argument/array-set] callee={symbol} box={box_name} receiver={receiver:?} receiver_def={:?} raw={raw_def:?} numeric_index={} numeric_value={}",
                                    {
                                        let row = array_owner_target(*receiver);
                                        let resolved = row.map(|row| row.resolve(module, box_name));
                                        format!("{resolved:?} census={:?}", view.ledger.array_i64_fields.get(box_name).map(|fields| fields.iter().map(|field| field.declaration_ordinal()).collect::<Vec<_>>()))
                                    },
                                    numeric(*index),
                                    numeric(*value)
                                );
                            }
                            allowed
                        }
                        MirInstruction::Invoke { operation: InvokeOperation::FieldGet { .. }
                            | InvokeOperation::ObjectFieldBorrow { .. }, .. }
                        | MirInstruction::ObjectFieldGet { .. } | MirInstruction::FieldGet { .. } | MirInstruction::Const { .. } | MirInstruction::Copy { .. }
                        | MirInstruction::Phi { .. } | MirInstruction::BinOp { .. } | MirInstruction::Compare { .. }
                        | MirInstruction::Branch { .. } | MirInstruction::Jump { .. } | MirInstruction::Return { .. }
                        | MirInstruction::InvokeNormalResult { .. } | MirInstruction::FaultFrameEnter { .. }
                        | MirInstruction::ReturnFault { .. } | MirInstruction::TypeOp { .. } | MirInstruction::ArrayRead { .. } => true,
                        _ => false,
                    };
                    if !allowed {
                        return Err(format!("{} callee={symbol} instruction={instruction:?}",
                            freeze("field-argument/parent-or-child-invalidated")));
                    }
                }
            }
            Ok(())
        })?;
        checked.insert(target);
    }
    Ok(())
}

/// The array a write targets: the canonical field read itself, or the
/// compatibility read, which names a declaration and is resolved through that
/// declaration (never through a runtime type or a bare name guess).
#[derive(Debug)]
enum ArrayFieldTargetV1 {
    Canonical { base: crate::mir::ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1 },
    Named { base: crate::mir::ValueId, name: Box<str> },
}
impl ArrayFieldTargetV1 {
    fn resolve(
        self,
        module: &crate::mir::MirModule,
        box_name: &str,
    ) -> Option<(crate::mir::ValueId, hakorune_mir_defs::CanonicalFieldRefV1)> {
        match self {
            Self::Canonical { base, field } => Some((base, field)),
            Self::Named { base, name } => {
                let definitions = module.canonical_object_definitions()?;
                let mut objects = definitions
                    .iter()
                    .enumerate()
                    .filter(|(_, definition)| definition.diagnostic_name() == box_name);
                let (object_index, definition) = objects.next()?;
                if objects.next().is_some() {
                    return None;
                }
                let mut fields = definition
                    .fields()
                    .iter()
                    .enumerate()
                    .filter(|(_, field)| field.name == name.as_ref());
                let (ordinal, field) = fields.next()?;
                if fields.next().is_some() || field.is_weak {
                    return None;
                }
                let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(
                    object_index,
                )?;
                Some((
                    base,
                    hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(
                        object, ordinal,
                    )?,
                ))
            }
        }
    }
}
