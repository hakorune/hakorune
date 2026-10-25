//! Final Map-slot Return provenance lent by the original checked get packet.
//! No formal substitution, owning result or discovery from emitted MIR.
use super::*;
use super::super::root_home::RootHomeCleanupOrderV1;
use crate::mir::instruction::{BorrowedResultSupportV1, MapInvokeOperation};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct FinalizedMapSlotReturnV1 {
    source: Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1>,
    producer: Binding,
    invoke: Binding,
    installed: Option<InstalledMapSlotReturnV1>,
    present: Option<Vec<Binding>>,
}
#[derive(Debug, Clone, PartialEq)]
struct InstalledMapSlotReturnV1 {
    normal: Binding,
    end: Binding,
}
impl FinalizedMapSlotReturnV1 {
    pub(super) fn issue(
        source: &Rc<crate::mir::normal_callable_semantic_package::MapStoredResultSourceV1>,
        terminal: &crate::mir::resolved_semantics::home_new_prefix::TerminalValueReturnV1,
        entry: &RootHomeExitEntry,
        order: &RootHomeCleanupOrderV1,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: &super::super::super::physical_boundary::FinishedBindings,
    ) -> Result<Self, String> {
        if let TerminalReturnedSourceV1::StoredMapInstalled(frame) = terminal.returned() {
            let retained = order
                .installed_map_return()
                .ok_or_else(|| freeze("borrowed-return/source-kind"))?;
            if !Rc::ptr_eq(source, frame.source()) || !Rc::ptr_eq(retained.install().frame(), frame)
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
            retained.validate(context, function, Some(finishing), entry)?;
            let install = retained.install();
            let row = Self {
                source: Rc::clone(source),
                producer: finishing
                    .relation_binding(function, retained.alias())?
                    .ok_or_else(|| freeze("borrowed-return/return-missing"))?,
                invoke: checked_projection(function, finishing, install.invoke())?,
                installed: Some(InstalledMapSlotReturnV1 {
                    normal: checked_projection(function, finishing, install.normal())?,
                    end: checked_projection(function, finishing, install.end())?,
                }),
                present: None,
            };
            row.validate(function, row.support())?;
            return Ok(row);
        }
        let TerminalReturnedSourceV1::StoredMapGet { source: get, .. } = terminal.returned() else {
            return Err(freeze("borrowed-return/source-kind"));
        };
        if get.result_source_v1().is_none_or(|result| !Rc::ptr_eq(source, result))
            || !source
                .present_get_v1(&OwnedExprSiteV1::new(
                    terminal.owner(),
                    terminal.value_site().clone(),
                ))
                .is_some_and(|original| std::sync::Arc::ptr_eq(original, get.get()))
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        let RootHomeExitEntry::MapGet {
            stored: Some(stored),
            invoke,
            projection,
            ..
        } = entry
        else {
            return Err(freeze("borrowed-return/original-exit"));
        };
        if stored.validate(context, function, Some(finishing), invoke, projection)?
            != (InvokeCallResultKind::BorrowedNullableHandle {
                support: source.result_support_v1(),
            })
        {
            return Err(freeze("borrowed-return/original-contract"));
        }
        let row = Self {
            source: Rc::clone(source),
            producer: checked_projection(function, finishing, projection)?,
            invoke: checked_projection(function, finishing, invoke)?,
            installed: None,
            present: Some(context.finished_map_presence_v1(
                source.present_has_v1(&OwnedExprSiteV1::new(
                    terminal.owner(), terminal.value_site().clone(),
                )).ok_or_else(|| freeze("borrowed-return/original-contract"))?,
                function, finishing,
            )?),
        };
        row.validate(function, row.source.result_support_v1())?;
        Ok(row)
    }
    pub(in crate::mir) fn producer(&self) -> &Binding {
        &self.producer
    }
    pub(in crate::mir) fn invoke(&self) -> &Binding {
        &self.invoke
    }
    pub(in crate::mir) fn bindings(&self) -> impl Iterator<Item = &Binding> {
        [&self.producer, &self.invoke].into_iter().chain(
            self.installed
                .iter()
                .flat_map(|row| [&row.normal, &row.end]),
        ).chain(self.present.iter().flat_map(|rows| rows.iter()))
    }
    pub(in crate::mir) fn installed_bindings(&self) -> Option<(&Binding, &Binding)> {
        self.installed.as_ref().map(|row| (&row.normal, &row.end))
    }
    pub(in crate::mir) fn present_bindings(&self) -> Option<&[Binding]> {
        self.present.as_deref()
    }
    pub(in crate::mir) fn support(&self) -> BorrowedResultSupportV1 {
        self.source.result_support_v1()
    }
    pub(super) fn value(&self) -> Result<ValueId, String> {
        self.producer
            .1
            .dst_value()
            .ok_or_else(|| freeze("borrowed-return/return-missing"))
    }
    pub(super) fn validate(
        &self,
        function: &MirFunction,
        support: BorrowedResultSupportV1,
    ) -> Result<(), String> {
        if support != self.support() {
            return Err(freeze("borrowed-return/original-contract"));
        }
        let BorrowedResultSupportV1::ReceiverFieldSlot { field, object, .. } = support else {
            return Err(freeze("borrowed-return/original-contract"));
        };
        if let Some(installed) = &self.installed {
            if self.present.is_some() {
                return Err(freeze("borrowed-return/original-contract"));
            }
            let MirInstruction::Copy {
                src: candidate,
                dst,
            } = self.producer.1
            else {
                return Err(freeze("borrowed-return/original-contract"));
            };
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Map(MapInvokeOperation::FieldInstallIndexed {
                        field: actual,
                        object: payload,
                        value,
                        ..
                    }),
                normal_landing,
                fault_frame,
                ..
            } = self.invoke.1
            else {
                return Err(freeze("borrowed-return/original-contract"));
            };
            if actual != field
                || payload != object
                || value != candidate
                || dst == candidate
                || installed.normal.0 != normal_landing
                || !matches!(installed.normal.1, MirInstruction::InvokeNormalResult { invoke_block, .. } if invoke_block == self.invoke.0)
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
            let outcome = installed
                .normal
                .1
                .dst_value()
                .ok_or_else(|| freeze("map-get-shape"))?;
            let MirInstruction::Invoke {
                operation: InvokeOperation::Map(MapInvokeOperation::EndOutcome { outcome: ended }),
                normal_landing,
                fault_frame: end_frame,
                ..
            } = installed.end.1
            else {
                return Err(freeze("borrowed-return/original-contract"));
            };
            if ended != outcome
                || end_frame != fault_frame
                || !crate::mir::verification::utils::compute_dominators(function)
                    .dominates(normal_landing, self.producer.0)
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
        } else {
            let Some(rows) = self.present.as_deref().filter(|rows| rows.len() >= 3) else {
                return Err(freeze("borrowed-return/original-contract"));
            };
            let (has, normal, branch) = (&rows[0], &rows[1], &rows[rows.len() - 1]);
            let mut condition_value = normal.1.dst_value()
                .ok_or_else(|| freeze("borrowed-return/original-contract"))?;
            for binding in &rows[2..rows.len() - 1] {
                let MirInstruction::Copy { src, dst } = binding.1 else {
                    return Err(freeze("borrowed-return/original-contract"));
                };
                if src != condition_value { return Err(freeze("borrowed-return/original-contract")); }
                condition_value = dst;
            }
            let MirInstruction::Invoke {
                operation: InvokeOperation::Map(MapInvokeOperation::FieldGetIndexed { field: get_field, base: get_base, key: get_key, .. }), ..
            } = &self.invoke.1 else { return Err(freeze("borrowed-return/original-contract")) };
            let MirInstruction::Invoke {
                operation: InvokeOperation::Map(MapInvokeOperation::FieldHas { field: has_field, base: has_base, key: has_key }), normal_landing, ..
            } = &has.1 else { return Err(freeze("borrowed-return/original-contract")) };
            let MirInstruction::Branch { condition, then_bb, .. } = &branch.1 else {
                return Err(freeze("borrowed-return/original-contract"));
            };
            let dominators = crate::mir::verification::utils::compute_dominators(function);
            if (get_field, get_base, get_key) != (has_field, has_base, has_key)
                || *normal_landing != normal.0
                || !matches!(&normal.1, MirInstruction::InvokeNormalResult { invoke_block, dst, .. }
                    if *invoke_block == has.0 && Some(*dst) == normal.1.dst_value())
                || *condition != condition_value
                || !dominators.dominates(normal.0, branch.0)
                || !dominators.dominates(branch.0, self.invoke.0)
                || !dominators.dominates(*then_bb, self.invoke.0)
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
            if !matches!(&self.invoke.1, MirInstruction::Invoke {
            operation: InvokeOperation::Map(MapInvokeOperation::FieldGetIndexed {
                field: actual, object: payload, .. }), normal_landing, ..
            } if *actual == field && *payload == object && *normal_landing == self.producer.0)
                || !matches!(self.producer.1, MirInstruction::InvokeNormalResult { invoke_block, .. }
                if invoke_block == self.invoke.0)
            {
                return Err(freeze("borrowed-return/original-contract"));
            }
        }
        for binding in self.bindings() {
            if !super::super::super::physical_boundary::check_binding(
                function, None, binding.0, &binding.1,
            )? {
                return Err(freeze("object-return/physical-binding"));
            }
        }
        Ok(())
    }
    /// The get result may flow only through its original Return transport.
    /// A Copy of a borrowed payload creates no formal or independent owner.
    pub(super) fn validate_uses(
        &self,
        function: &MirFunction,
        returned: &Binding,
        copies: &[Binding],
    ) -> Result<(), String> {
        let mut values = BTreeSet::from([self.value()?]);
        for (_, instruction) in copies {
            let MirInstruction::Copy { dst, src } = instruction else {
                return Err(freeze("object-return/physical-binding"));
            };
            if !values.contains(src) || !values.insert(*dst) {
                return Err(freeze("object-return/physical-binding"));
            }
        }
        if !matches!(returned.1, MirInstruction::Return { value: Some(value) } if values.contains(&value))
        {
            return Err(freeze("borrowed-return/return-missing"));
        }
        for (id, block) in &function.blocks {
            for instruction in block.all_instructions() {
                if instruction
                    .used_values()
                    .iter()
                    .any(|value| values.contains(value))
                    && (*id, instruction.clone()) != *returned
                    && !copies.contains(&(*id, instruction.clone()))
                    && !matches!(instruction, MirInstruction::KeepAlive { .. })
                {
                    return Err(freeze("object-return/physical-binding"));
                }
            }
        }
        Ok(())
    }
}
