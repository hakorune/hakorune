//! Original call actuals joined to the caller's sealed entry, not metadata.
use super::*;

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract) fn local_field_writes_entry(&mut self, source: &FinalizedRootSourceHandoffV1,
        owner: FunctionOwnerIdV1, function: &MirFunction,
    ) -> Result<(), String> {
        let state = self.functions.entry(function.signature.name.clone()).or_default();
        field_write::local_entry(state, source, owner, function)
    }
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract) fn entry(
        &mut self,
        source: &FinalizedRootSourceHandoffV1,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(function.signature.name.clone())
            .or_default();
        let entry = source.borrowed_ordinary_entry_source_for_function_v1(owner, function)?;
        state.finite_input = entry.uses_finite_input_v1();
        for (ordinal, formal, value) in source.borrowed_ordinary_entry_values_v1(owner)?.iter() {
            if let Some(domain) = entry.formal_domain_v1(*ordinal, *formal)? {
                if state.domains.insert(*value, domain.clone()).is_some_and(|previous| previous != *domain) {
                    return Err(fault("borrowed-use/conflicting-domain"));
                }
            }
            if function
                .params
                .iter()
                .filter(|parameter| *parameter == value)
                .count()
                != 1
            {
                return Err(fault("borrowed-use/entry-parameter"));
            }
            if let Some(previous) = state.roots.insert(*value, *formal) {
                if previous != *formal {
                    return Err(fault("borrowed-use/conflicting-root"));
                }
            }
        }
        source.with_borrowed_ordinary_alias_copies_v1(owner, function, |_, _, _, _, _, copies| {
            for original in copies {
                if let Some((finished, coordinate)) =
                    source.borrowed_ordinary_alias_finished_copy_v1(owner, function, original)?
                {
                    state.copy(&finished, Some(coordinate))?;
                }
            }
            Ok(())
        })?;
        for (_, formal, _) in source
            .borrowed_ordinary_compare_uses_v1(owner, function)?
            .iter()
        {
            *state.compare_admissions.entry(*formal).or_default() += 1;
        }
        let mut expected = BTreeMap::<_, BTreeSet<_>>::new();
        let mut compare_sites = BTreeMap::new();
        source.with_borrowed_ordinary_compares_v1(owner, function, |loan, _, binding| {
            let block = function.blocks.get(&binding.0)
                .ok_or_else(|| fault("borrowed-use/compare-block"))?;
            let mut rows = block.all_instructions().enumerate()
                .filter(|(_, row)| **row == binding.1);
            let (index, _) = rows.next().ok_or_else(|| fault("borrowed-use/compare-missing"))?;
            if rows.next().is_some() { return Err(fault("borrowed-use/compare-duplicate")); }
            if compare_sites.insert(loan.site().clone(), (binding.0, index)).is_some() {
                return Err(fault("borrowed-use/compare-source-duplicate"));
            }
            for (formal, side) in loan.operand_formals() {
                if !expected.entry(formal).or_default().insert(((binding.0, index), side)) {
                    return Err(fault("borrowed-use/compare-source-duplicate"));
                }
            }
            Ok(())
        })?;
        source.with_finished_loop_compare_uses_v1(owner, function, |formal, coordinate, compare, side, copies| {
            if !expected.entry(formal).or_default().insert((coordinate, side)) {
                return Err(fault("borrowed-use/compare-source-duplicate"));
            }
            if state.loop_compares.insert(coordinate, compare.clone()).is_some_and(|old| old != compare) {
                return Err(fault("borrowed-use/compare-source-duplicate"));
            }
            *state.compare_admissions.entry(formal).or_default() += 1;
            let mut rooted: Vec<_> = state.copies.values().filter_map(|(row, at)|
                at.map(|at| (row.clone(), at))).collect();
            rooted.extend(copies);
            state.retain_rooted_copies_v1(state.roots.keys().copied().collect(), &rooted)?;
            Ok(())
        })?;
        state.compare_coordinates = Some(expected);
        source.with_borrowed_ordinary_compare_carrier_copies_v1(owner, function, |copy, coordinate| {
            state.copy(copy, Some(coordinate))
        })?;
        guarded_arguments::entry(state, source, owner, function, &compare_sites)?;
        field_write::entry(state, source, owner, function, &compare_sites)?;
        array_set::entry(state, source, owner, function, &compare_sites)?;
        mul::entry(state, source, owner, function, &compare_sites)?;
        integer_return::entry(state, source, owner, function)?;
        for (_, formal, _) in source
            .borrowed_ordinary_add_uses_v1(owner, function)?
            .iter()
        {
            *state.add_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_array_element_uses_v1(owner, function)?
            .iter()
        {
            *state.set_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _, _) in source
            .borrowed_ordinary_new_argument_uses_v1(owner, function)?
            .iter()
        {
            *state.ctor_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_null_compare_uses_v1(owner, function)?
            .iter()
        {
            *state.null_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_field_read_uses_v1(owner, function)?
            .iter()
        {
            *state.field_admissions.entry(*formal).or_default() += 1;
        }
        if state.finite_input {
            source.with_finished_bool_bindings_v1(owner, function, |coordinate, instruction| {
                if state.finite_scalar_bindings.insert(coordinate, instruction.clone())
                    .is_some_and(|previous| previous != instruction) {
                    return Err(fault("borrowed-use/coordinate-duplicate"));
                }
                Ok(())
            })?;
            let mut scalar_copies: Vec<_> = state.copies.values().filter_map(|(binding, coordinate)|
                coordinate.map(|coordinate| (binding.clone(), coordinate))).collect();
            source.with_finished_scalar_bindings_v1(owner, function, |original, coordinate| {
                if state.finite_scalar_bindings.insert(coordinate, original.1.clone())
                    .is_some_and(|previous| previous != original.1) {
                    return Err(fault("borrowed-use/coordinate-duplicate"));
                }
                if matches!(original.1, MirInstruction::Copy { .. }) {
                    scalar_copies.push((original.clone(), coordinate));
                }
                Ok(())
            })?;
            state.retain_rooted_copies_v1(state.roots.keys().copied().collect(), &scalar_copies)?;
        }
        let tracked = state.tracked()?;
        source.with_finished_tagged_text_operands_v1(owner, function, |formal, value, side, coordinate| {
            let expected = crate::mir::instruction::StringInvokeOperand::Dynamic(value);
            let key = (coordinate.0, coordinate.1, side);
            if tracked.get(&value) != Some(&formal) {
                return Err(format!("{} function={} phase=source-image coordinate={coordinate:?} side={side} value={value:?} formal={formal:?} tracked={:?}",
                    fault("borrowed-text/operand-correspondence"), function.signature.name, tracked.get(&value)));
            }
            if let Some(previous) = state.text_operands.insert(key, expected.clone()) {
                return Err(format!("{} function={} phase=source-image coordinate={coordinate:?} side={side} previous={previous:?} expected={expected:?}",
                    fault("borrowed-text/operand-correspondence"), function.signature.name));
            }
            Ok(())
        })?;
        state.object_views =
            source.borrowed_ordinary_formal_object_views_v1(owner, function)?;
        Ok(())
    }


}

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn call(
        &mut self,
        caller: &MirFunction,
        coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1],
        copies: &[(Binding, Coordinate)],
        caller_entry: &[(u32, BindingRefV1, ValueId)],
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(caller.signature.name.clone())
            .or_default();
        // The packet has independently validated every copy, including
        // ordinary scalar/receiver dependencies. This closure tracks only
        // descendants of the SAME source-issued borrowed entry roots.
        state.retain_rooted_copies_v1(caller_entry.iter().map(|(_, _, value)| *value).collect(), copies)?;
        for actual in actuals {
            // A closed scalar seed that names the caller's own opaque formal is
            // still that formal's value on the wire: the same entry slot, the
            // same handle argument. Its integer class stays with the source
            // domain, so only the entry root can make this a forward row.
            let origin = match &actual.source {
                BorrowedFormalActualSourceV1::Scalar { binding, .. } => Some((*binding, *binding)),
                _ => actual.source.caller_formal_origin_v1(),
            };
            if let Some((_, formal)) = origin {
                // DeclaredObject also exists outside the selected borrowed
                // transport. Only the original finished caller entry can
                // connect it to this operand closure; visitation order cannot.
                if matches!(
                    actual.source,
                    BorrowedFormalActualSourceV1::DeclaredFormal { .. }
                        | BorrowedFormalActualSourceV1::Scalar { .. }
                ) && !caller_entry.iter().any(|(_, root, _)| *root == formal)
                {
                    continue;
                }
                let key = (coordinate.0, coordinate.1, actual.ordinal as usize);
                if let Some(previous) = state.arguments.insert(key, formal) {
                    if previous != formal {
                        return Err(fault("borrowed-use/conflicting-argument"));
                    }
                }
            }
        }
        Ok(())
    }
}

impl BorrowedCallUses {
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view::compiled_entry_contract::borrowed_call_incoming) fn computed_call(
        &mut self, caller: &MirFunction, coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1], operands: &BTreeMap<u32, crate::mir::instruction::StringInvokeOperand>,
    ) -> Result<(), String> {
        let required: BTreeSet<_> = actuals.iter().filter(|actual|
            matches!(actual.source, BorrowedFormalActualSourceV1::Expression(_))).map(|actual| actual.ordinal).collect();
        if required != operands.keys().copied().collect() { return Err(fault("borrowed-text/source-coverage")); }
        let state = self.functions.entry(caller.signature.name.clone()).or_default();
        for (ordinal, operand) in operands {
            let actual = actuals.iter().find(|actual| actual.ordinal == *ordinal)
                .ok_or_else(|| fault("borrowed-text/source-coverage"))?;
            match (&actual.source, operand) {
                (BorrowedFormalActualSourceV1::Expression(source), crate::mir::instruction::StringInvokeOperand::NormalInteger(_))
                    if source.frame().is_i64() => continue,
                (BorrowedFormalActualSourceV1::Expression(source), crate::mir::instruction::StringInvokeOperand::OwnedString(_))
                    if !source.frame().is_i64() => {},
                _ => return Err(fault("borrowed-text/operand-correspondence")),
            }
            if state.text_operands.insert((coordinate.0, coordinate.1, *ordinal as usize), operand.clone()).is_some() {
                return Err(fault("borrowed-text/duplicate-occurrence"));
            }
        }
        Ok(())
    }
}
impl FunctionUses {
    /// Both source consumers select physical Text occurrences independently;
    /// only the SAME packet image can authorize any selected occurrence.
    pub(super) fn verify_text_occurrences_v1(&self, indexed: &[(Coordinate, &MirInstruction)]) -> Result<(), String> {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        let owned: BTreeSet<_> = indexed.iter().filter_map(|(_, instruction)| match instruction {
            MirInstruction::InvokeNormalResult { dst, invoke_block }
                if indexed.iter().any(|(coordinate, instruction)| coordinate.0 == *invoke_block
                    && matches!(instruction, MirInstruction::Invoke { operation, .. }
                        if operation.normal_result_kind() == Some(crate::mir::instruction::InvokeNormalResultKind::OwnedString))) => Some(*dst),
            _ => None,
        }).collect();
        let mut seen = BTreeSet::new();
        for (coordinate, instruction) in indexed {
            if let MirInstruction::Invoke { operation: InvokeOperation::String(
                crate::mir::instruction::StringInvokeOperation::Add { left, right }), .. } = instruction {
                for (side, operand) in [left, right].into_iter().enumerate() {
                    let key = (coordinate.0, coordinate.1, side);
                    if let Some(expected) = self.text_operands.get(&key) {
                        if expected != operand || !matches!(expected, Operand::Dynamic(_)) || !seen.insert(key) {
                            return Err(format!("{} phase=string-add coordinate={coordinate:?} side={side} expected={expected:?} actual={operand:?}",
                                fault("borrowed-text/operand-correspondence")));
                        }
                    }
                }
                continue;
            }
            let call = match instruction {
                MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. } => call,
                MirInstruction::Call(call) if call.args.iter().any(|value| owned.contains(value)) => return Err(fault("borrowed-text/plain-call")),
                _ => continue,
            };
            for (ordinal, value) in call.args.iter().enumerate() {
                let key = (coordinate.0, coordinate.1, ordinal);
                if !owned.contains(value) && !self.text_operands.contains_key(&key) { continue; }
                if !matches!(self.text_operands.get(&key), Some(Operand::OwnedString(original)) if original == value && owned.contains(value))
                    || !seen.insert(key) {
                    return Err(format!("{} phase=call coordinate={coordinate:?} ordinal={ordinal} expected={:?} actual={value:?}",
                        fault("borrowed-text/operand-correspondence"), self.text_operands.get(&key)));
                }
            }
        }
        if seen != self.text_operands.keys().copied().collect() { return Err(fault("borrowed-text/occurrence-coverage")); }
        Ok(())
    }
}

impl FunctionUses {
    /// Both final scans corroborate readonly String operands against the same
    /// sealed Text entry domains; ordinary numeric/object uses stay refused.
    pub(super) fn verify_text_operation_v1(&self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        coordinate: Coordinate, operation: &crate::mir::instruction::StringInvokeOperation, fault_frame: ValueId,
    ) -> Result<(), String> {
        let scalar_values = match operation {
            crate::mir::instruction::StringInvokeOperation::Substring { start, end, .. } => {
                let Some(start) = start.value() else { return Err(fault("string-index-operand")); };
                let Some(end) = end.value() else { return Err(fault("string-index-operand")); };
                vec![start, end]
            }
            crate::mir::instruction::StringInvokeOperation::End { value }
                | crate::mir::instruction::StringInvokeOperation::EndIfOwned { value } => vec![*value],
            _ => Vec::new(),
        };
        let operands = operation.operands();
        let mut ordinary = Vec::new();
        for (side, operand) in operands.iter().copied().enumerate() {
            let key = (coordinate.0, coordinate.1, side);
            if matches!(operation, crate::mir::instruction::StringInvokeOperation::Add { .. })
                && matches!(operand, crate::mir::instruction::StringInvokeOperand::Dynamic(_))
                && self.text_operands.get(&key) == Some(operand) {
                continue;
            }
            ordinary.push(operand);
        }
        self.verify_text_operands_v1(tracked, views, &ordinary, &scalar_values, fault_frame)
    }

    /// One typed operand check for String and synchronous Map field reads.
    /// Scalar receiver/frame operands cannot smuggle a borrowed tagged value.
    pub(super) fn verify_text_operands_v1(&self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        operands: &[&crate::mir::instruction::StringInvokeOperand], scalar_values: &[ValueId], fault_frame: ValueId,
    ) -> Result<(), String> {
        use crate::mir::instruction::StringInvokeOperand as Operand;
        if tracked.contains_key(&fault_frame) || views.views.contains_key(&fault_frame) {
            return Err(fault("borrowed-use/string-fault-frame"));
        }
        for operand in operands.iter().copied() {
            if let Operand::BorrowedText(value) = operand {
                let formal = tracked.get(value).ok_or_else(|| fault("borrowed-use/string-formal"))?;
                let mut roots = self.roots.iter().filter(|(_, original)| *original == formal);
                let (root, _) = roots.next().ok_or_else(|| fault("borrowed-use/string-formal"))?;
                if roots.next().is_some() || !self.domains.get(root).is_some_and(|domain| domain.text_only()) {
                    return Err(fault("borrowed-use/string-domain"));
                }
            }
        }
        for operand in operands.iter().copied() {
            if operand.value().is_some_and(|value| tracked.contains_key(&value) || views.views.contains_key(&value))
                && !matches!(operand, Operand::BorrowedText(_)) {
                return Err(fault("borrowed-use/string-operand"));
            }
        }
        if scalar_values.iter().any(|value| tracked.contains_key(value) || views.views.contains_key(value)) {
            return Err(fault("borrowed-use/string-operand"));
        }
        Ok(())
    }
}

impl FunctionUses {
    fn retain_rooted_copies_v1(&mut self, mut borrowed_values: BTreeSet<ValueId>, copies: &[(Binding, Coordinate)])
        -> Result<(), String> {
        let mut admitted = BTreeSet::new();
        loop {
            let mut progressed = false;
            for (index, (original, finished)) in copies.iter().enumerate() {
                let MirInstruction::Copy { src, dst } = &original.1 else {
                    return Err(fault("borrowed-use/copy-instruction"));
                };
                if admitted.contains(&index) || !borrowed_values.contains(src) { continue; }
                self.copy(original, Some(*finished))?;
                borrowed_values.insert(*dst);
                admitted.insert(index);
                progressed = true;
            }
            if !progressed { break; }
        }
        Ok(())
    }

    /// The same Copy closure keeps source-owned scalar aliases tracked, so a
    /// finished prefix cannot lend them to an unproved later operand or Return.
    pub(super) fn prepare_finite_source_copies_v1(&mut self) -> Result<(), String> {
        if !self.finite_input { return Ok(()); }
        let mut copies: Vec<_> = self.copies.values().filter_map(|(binding, coordinate)|
            coordinate.map(|coordinate| (binding.clone(), coordinate))).collect();
        copies.extend(self.finite_scalar_bindings.iter().filter_map(|(coordinate, instruction)|
            matches!(instruction, MirInstruction::Copy { .. }).then_some(((coordinate.0, instruction.clone()), *coordinate))));
        copies.extend(self.owned_field_writes.values().flat_map(|write| write.bindings.iter())
            .filter(|(binding, _)| matches!(binding.1, MirInstruction::Copy { .. })).cloned());
        self.retain_rooted_copies_v1(self.roots.keys().copied().collect(), &copies)
    }
}

/// Exact source-issued scalar argument, not a count or a generic constructor rule.
pub(super) struct FiniteBirthArgumentUse {
    instruction: MirInstruction,
    value: ValueId,
    domain: BorrowedFormalDomainV1,
}
impl FunctionUses {
    pub(super) fn finite_birth_arguments_v1(
        &mut self, source: &FinalizedRootSourceHandoffV1,
        actuals: &[crate::mir::normal_callable_semantic_package::FinalizedBirthActualsV1],
        function: &MirFunction,
    ) -> Result<(), String> {
        if !self.finite_input { return Ok(()); }
        let owners: BTreeSet<_> = self.roots.values().map(|formal| formal.owner()).collect();
        let owners: Vec<_> = owners.into_iter().collect();
        let [owner] = owners.as_slice() else {
            return Err(fault("borrowed-use/entry-parameter"));
        };
        for actual in actuals.iter().filter(|actual| actual.owner() == *owner) {
            for (ordinal, argument) in actual.arguments().iter().enumerate() {
                let Some((domain, binding, coordinate)) = source.finite_entry_birth_argument_v1(actual, ordinal as u32, function)?
                    else { continue; };
                if self.finite_birth_arguments.insert((coordinate, ordinal), FiniteBirthArgumentUse {
                    instruction: binding.1, value: argument.value_only_v1()?, domain,
                }).is_some() { return Err(fault("borrowed-use/ctor-coverage")); }
            }
        }
        Ok(())
    }
    pub(super) fn observe_finite_birth_argument_v1(
        &self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        coordinate: Coordinate, ordinal: usize, instruction: &MirInstruction, value: ValueId,
        scan: &mut Scan,
    ) -> Result<bool, String> {
        let Some(expected) = self.finite_birth_arguments.get(&(coordinate, ordinal)) else { return Ok(false); };
        let formal = tracked.get(&value).or(views.views.get(&value));
        let roots: Vec<_> = self.roots.iter().filter(|(_, root)| **root == expected.domain.binding()).collect();
        if !self.finite_input || instruction != &expected.instruction || value != expected.value
            || formal != Some(&expected.domain.binding()) || !expected.domain.signed_only()
            || roots.len() != 1 || self.domains.get(roots[0].0) != Some(&expected.domain)
            || !scan.finite_birth_arguments.insert((coordinate, ordinal)) {
            return Err(fault("borrowed-use/undominated-view"));
        }
        Ok(true)
    }
}

pub(super) struct CompletedReceiverUse {
    domain: BorrowedFormalDomainV1,
    value: ValueId,
    instruction: MirInstruction,
}
impl BorrowedCallUses {
    pub(in super::super) fn receiver(
        &mut self, source: &FinalizedRootSourceHandoffV1,
        packet: crate::mir::normal_callable_semantic_package::FinalizedBorrowedCallPacketV1<'_>,
        arguments: &[crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1],
        function: &MirFunction, coordinate: Coordinate, copies: &[(Binding, Coordinate)],
    ) -> Result<(), String> {
        let Some((domain, value, instruction, read_copies)) = source.completed_call_receiver_v1(packet, arguments, function, coordinate)?
            else { return Ok(()); };
        let state = self.functions.entry(function.signature.name.clone()).or_default();
        let entry = source.borrowed_ordinary_entry_values_v1(packet.call_site().owner())?;
        state.retain_rooted_copies_v1(entry.iter().map(|(_, _, value)| *value).collect(), copies)?;
        for (binding, coordinate) in read_copies { state.copy(&binding, Some(coordinate))?; }
        if state.receivers.insert(coordinate, CompletedReceiverUse { domain, value, instruction }).is_some() {
            return Err(fault("borrowed-use/conflicting-argument"));
        }
        Ok(())
    }
}
impl FunctionUses {
    pub(super) fn observe_receiver_v1(
        &self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        coordinate: Coordinate, instruction: &MirInstruction, scan: &mut Scan,
    ) -> Result<Option<ValueId>, String> {
        let Some(expected) = self.receivers.get(&coordinate) else { return Ok(None); };
        let roots: Vec<_> = self.roots.iter().filter(|(_, formal)| **formal == expected.domain.binding()).collect();
        if roots.len() != 1 || self.domains.get(roots[0].0) != Some(&expected.domain)
            || tracked.get(&expected.value).or(views.views.get(&expected.value)) != Some(&expected.domain.binding())
            || instruction != &expected.instruction
            || !matches!(instruction, MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }
                if matches!(&call.callee, Callee::SameModuleInstance { key, receiver }
                    if *receiver == expected.value && expected.domain.single_object_class_v1() == Some(key.owner())))
            || !scan.receivers.insert(coordinate) {
            return Err(fault("borrowed-use/callee-or-fault-frame"));
        }
        Ok(Some(expected.value))
    }
    pub(super) fn completed_object_views_v1(&mut self, module: &crate::mir::MirModule) -> Result<(), String> {
        for (value, domain) in &self.domains {
            let Some(class) = domain.single_object_class_v1() else { continue; };
            let object = module.metadata.canonical_object_membership.as_ref()
                .and_then(|membership| membership.get(class)).copied()
                .ok_or_else(|| fault("borrowed-use/entry-parameter"))?;
            if module.canonical_object_definition(object).is_none() { return Err(fault("borrowed-use/entry-parameter")); }
            if self.roots.get(value) != Some(&domain.binding())
                || self.object_views.insert(domain.binding(), object).is_some_and(|previous| previous != object) {
                return Err(fault("borrowed-use/entry-parameter"));
            }
        }
        Ok(())
    }
}
