//! Independent affine String state; source facts only lend exact aliases,
//! carrier transfers and operations. Normal/Fault order remains checked here.
use super::source::{Origin, TextSourceImage};
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Residence {
    Home(ValueId),
    Carrier(ValueId),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carrier {
    Static,
    Owned(ValueId),
    Ended,
}
#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Live {
    temporary: Vec<ValueId>,
    homes: Vec<Residence>,
    carriers: BTreeMap<ValueId, Carrier>,
    replacement: Option<(ValueId, ValueId)>,
    hold: Option<ValueId>,
    call_pending: Option<ValueId>,
}
impl Live {
    fn has_home(&self, value: ValueId) -> bool {
        self.homes.contains(&Residence::Home(value))
    }
    fn contains(&self, value: &ValueId) -> bool {
        self.temporary.contains(value) || self.has_home(*value)
    }
    fn active(&self, home: Residence) -> bool {
        match home {
            Residence::Home(_) => true,
            Residence::Carrier(slot) => matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))),
        }
    }
    fn is_empty(&self) -> bool {
        self.temporary.is_empty()
            && !self.homes.iter().any(|home| self.active(*home))
            && self
                .carriers
                .values()
                .all(|state| !matches!(state, Carrier::Owned(_)))
            && self.replacement.is_none()
            && self.hold.is_none()
            && self.call_pending.is_none()
    }
    fn compatible_join(&self, other: &Self, source: &TextSourceImage) -> bool {
        let raw_homes = |homes: &[Residence]| {
            homes
                .iter()
                .filter_map(|home| match home {
                    Residence::Home(value) => Some(*value),
                    Residence::Carrier(_) => None,
                })
                .collect::<Vec<_>>()
        };
        self.temporary == other.temporary
            && raw_homes(&self.homes) == raw_homes(&other.homes)
            && self.replacement == other.replacement
            && self.hold == other.hold
            && self.call_pending == other.call_pending
            && self
                .carriers
                .keys()
                .chain(other.carriers.keys())
                .all(|slot| source.has_slot(*slot))
    }
    fn descriptor_live(&self, value: ValueId, source: &TextSourceImage) -> bool {
        match source.origin(value) {
            Some(Origin::Slot(slot)) => matches!(
                self.carriers.get(&slot),
                Some(Carrier::Static | Carrier::Owned(_))
            ),
            _ => false,
        }
    }
    fn end(
        &mut self,
        value: ValueId,
        faulted: bool,
        retained: &[ValueId],
        source: &TextSourceImage,
    ) -> bool {
        if let Some(top) = self.temporary.last() {
            if *top != value {
                return false;
            }
            self.temporary.pop();
            if self
                .call_pending
                .is_some_and(|held| self.temporary.last() == Some(&held))
            {
                self.call_pending = None;
            }
            return true;
        }
        if let Some((new, old)) = self.replacement {
            if !faulted {
                if value != old || self.homes.last() != Some(&Residence::Home(new)) {
                    return false;
                }
                let Some(index) = self
                    .homes
                    .iter()
                    .position(|home| *home == Residence::Home(old))
                else {
                    return false;
                };
                self.homes.remove(index);
                self.replacement = None;
                return true;
            }
            if value == new {
                self.replacement = None;
            }
        }
        let mut keep = Vec::new();
        if !faulted {
            for retained in retained {
                let home = match source.origin(*retained) {
                    Some(Origin::Raw(raw)) if self.has_home(raw) => Residence::Home(raw),
                    Some(Origin::Slot(slot))
                        if matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))) =>
                    {
                        Residence::Carrier(slot)
                    }
                    Some(Origin::Static) => continue,
                    Some(Origin::Slot(slot))
                        if self.carriers.get(&slot) == Some(&Carrier::Static) =>
                    {
                        continue
                    }
                    _ => return false,
                };
                if home == Residence::Home(value) {
                    return false;
                }
                keep.push(home);
            }
        }
        let index = self.homes.iter().rposition(|home| {
            self.active(*home)
                && !keep.contains(home)
                && (faulted || !matches!(home, Residence::Home(value) if Some(*value) == self.hold))
        });
        let Some(index) = index else {
            return false;
        };
        if self.homes[index] != Residence::Home(value) {
            return false;
        }
        self.homes.remove(index);
        if self.hold == Some(value) {
            self.hold = None;
        }
        true
    }
    fn end_carrier(&mut self, value: ValueId, faulted: bool, source: &TextSourceImage) -> bool {
        let Some(Origin::Slot(slot)) = source.origin(value) else {
            return false;
        };
        let Some(state @ (Carrier::Static | Carrier::Owned(_))) = self.carriers.get(&slot).copied()
        else {
            return false;
        };
        if !self.temporary.is_empty() {
            return false;
        }
        if let Some((new, old)) = self.replacement.filter(|_| !faulted) {
            if old != value || self.homes.last() != Some(&Residence::Home(new)) {
                return false;
            }
            self.replacement = None;
        } else if matches!(state, Carrier::Owned(_))
            && self.homes.iter().rev().find(|home| self.active(**home))
                != Some(&Residence::Carrier(slot))
        {
            return false;
        }
        if matches!(state, Carrier::Owned(_)) {
            self.carriers.insert(slot, Carrier::Ended);
        }
        true
    }
    fn edge(
        &mut self,
        predecessor: BasicBlockId,
        target: BasicBlockId,
        source: &TextSourceImage,
    ) -> bool {
        let Some(transfers) = source.edges.get(&(predecessor, target)) else {
            return true;
        };
        // Resolve every incoming against the SAME predecessor state before any
        // simultaneous Phi transfer can change another descriptor's meaning.
        let mut pending = Vec::new();
        for (slot, value) in transfers {
            let Some(origin) = source.origin(*value) else {
                return false;
            };
            let state = match origin {
                Origin::Static => Carrier::Static,
                Origin::Raw(value) if self.has_home(value) => Carrier::Owned(value),
                Origin::Slot(previous) if previous == *slot => match self.carriers.get(&previous) {
                    Some(state @ (Carrier::Static | Carrier::Owned(_))) => *state,
                    _ => return false,
                },
                // Distinct source carrier lineages need an explicit affine
                // forwarding proof; a matching payload never supplies it.
                _ => return false,
            };
            pending.push((*slot, origin, state));
        }
        for (slot, origin, state) in pending {
            if matches!(origin, Origin::Slot(previous) if previous == slot) {
                continue;
            }
            if matches!(self.carriers.get(&slot), Some(Carrier::Owned(_))) {
                return false;
            }
            self.homes.retain(|home| *home != Residence::Carrier(slot));
            match origin {
                Origin::Raw(value) => {
                    let Some(index) = self
                        .homes
                        .iter()
                        .position(|home| *home == Residence::Home(value))
                    else {
                        return false;
                    };
                    self.homes[index] = Residence::Carrier(slot);
                }
                Origin::Static => self.homes.push(Residence::Carrier(slot)),
                Origin::Slot(_) => return false,
            }
            self.carriers.insert(slot, state);
        }
        true
    }
}

pub(super) fn check(
    function: &MirFunction,
    source: Option<&TextSourceImage>,
) -> Result<(), (BasicBlockId, &'static str, Option<String>)> {
    let mut site = function.entry_block;
    let mut outstanding = None;
    let result = (|| {
        let empty = TextSourceImage::default();
        let source = source.unwrap_or(&empty);
        // Lifecycle dependencies are not actual arguments. Only an owned Text
        // Invoke can carry them, even when no other String operation is selected.
        for (block_id, row) in function
            .blocks
            .iter()
            .flat_map(|(id, block)| block.all_instructions().map(move |row| (*id, row)))
        {
            site = block_id;
            match row {
                MirInstruction::Invoke {
                    operation: InvokeOperation::String(Op::EndIfOwned { .. }),
                    ..
                } if !source.permits_operation(block_id, row) => return Err("string-owned-escape"),
                MirInstruction::Call(call) if !call.text_argument_cleanup.is_empty() => {
                    return Err("string-owned-escape")
                }
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result },
                    ..
                } if !call.text_argument_cleanup.is_empty()
                    && *result != crate::mir::instruction::InvokeCallResultKind::OwnedString =>
                {
                    return Err("string-owned-escape")
                }
                _ => (),
            }
        }
        let selected = |instruction: &MirInstruction| {
            matches!(instruction, MirInstruction::StringResidenceInstall { .. })
                || matches!(
            instruction, MirInstruction::Invoke { operation, .. }
                if matches!(operation, InvokeOperation::FieldSet { mode, .. } if mode.has_owned_text_candidate_v1())
                    || matches!(operation, InvokeOperation::String(_)
                    | InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldHas { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldInstallIndexed { .. }
                        | crate::mir::instruction::MapInvokeOperation::FieldDeleteIndexed { .. })
                    | InvokeOperation::ArrayElementWrite { value: crate::mir::instruction::ArrayElementWriteValueV1::Text(_), .. })
                    || operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString))
        };
        if source.slots.is_empty()
            && !function
                .blocks
                .values()
                .any(|block| block.all_instructions().any(selected))
        {
            return Ok(());
        }
        let mut results = BTreeMap::<ValueId, BasicBlockId>::new();
        let mut producers = BTreeMap::<BasicBlockId, ValueId>::new();
        for block in function.blocks.values() {
            site = block.id;
            for instruction in block.all_instructions() {
                if let MirInstruction::InvokeNormalResult { dst, invoke_block } = instruction {
                    if matches!(
                        function
                            .blocks
                            .get(invoke_block)
                            .and_then(|b| b.terminator.as_ref()),
                        Some(MirInstruction::Invoke {
                            operation,
                            ..
                        }) if operation.normal_result_kind() == Some(InvokeNormalResultKind::OwnedString)
                    ) {
                        if results.insert(*dst, *invoke_block).is_some()
                            || producers.insert(*invoke_block, *dst).is_some()
                        {
                            return Err("string-result-duplicate");
                        }
                    }
                }
            }
        }
        for block in function.blocks.values() {
            site = block.id;
            for instruction in block.all_instructions() {
                if let MirInstruction::Invoke { operation, .. } = instruction {
                    if let InvokeOperation::ArrayElementWrite {
                        kind,
                        producer,
                        index,
                        value,
                        ..
                    } = operation
                    {
                        if !value.valid_for(*kind, *producer, *index) {
                            return Err("string-text-operand-domain");
                        }
                    }
                    if matches!(operation, InvokeOperation::FieldSet { mode, value, .. }
                    if mode.has_owned_text_candidate_v1() && !results.contains_key(value))
                    {
                        return Err("string-owned-result-missing");
                    }
                    if let InvokeOperation::String(operation) = operation {
                        if !operation.valid_operand_domains() {
                            return Err(if matches!(operation, Op::Add { .. }) {
                                "string-add-left-domain"
                            } else {
                                "string-text-operand-domain"
                            });
                        }
                        if let Op::Substring { start, end, .. } = operation {
                            for operand in [start, end] {
                                let Some(value) = operand.value() else {
                                    return Err("string-integer-domain");
                                };
                                if results.contains_key(&value)
                                    || matches!(operand, Operand::NormalInteger(_)
                                if check_borrowed_operand(function, value).is_ok())
                                {
                                    return Err("string-integer-domain");
                                }
                                if matches!(operand, Operand::Dynamic(_))
                                    && !is_tagged_dynamic_formal(function, value)
                                {
                                    return Err("string-dynamic-source-domain");
                                }
                            }
                        }
                    }
                    for operand in text_operands(operation) {
                        match operand {
                            Operand::TextOperand(value)
                                if !source.permits_operation(block.id, instruction)
                                    || !matches!(source.origin(*value), Some(Origin::Slot(_))) =>
                            {
                                return Err("string-text-operand-domain");
                            }
                            Operand::OwnedString(value) if !results.contains_key(value) => {
                                return Err("string-owned-result-missing");
                            }
                            Operand::BorrowedText(value) => {
                                check_borrowed_operand(function, *value)?
                            }
                            Operand::NormalInteger(value)
                                if results.contains_key(value)
                                    || check_borrowed_operand(function, *value).is_ok() =>
                            {
                                return Err("string-integer-domain");
                            }
                            Operand::Dynamic(value)
                                if !is_tagged_dynamic_formal(function, *value) =>
                            {
                                return Err("string-dynamic-source-domain");
                            }
                            _ => {}
                        }
                    }
                    if matches!(operation, InvokeOperation::String(Op::End { value }) if !results.contains_key(value))
                    {
                        return Err("string-end-result-missing");
                    }
                }
                if let MirInstruction::StringResidenceInstall {
                    value,
                    replaces,
                    returning,
                } = instruction
                {
                    if (*returning && replaces.is_some())
                        || replaces.is_some_and(|previous| {
                            previous == *value
                                || (!results.contains_key(&previous)
                                    && !(source.permits_operation(block.id, instruction)
                                        && matches!(
                                            source.origin(previous),
                                            Some(Origin::Slot(_))
                                        )))
                        })
                    {
                        return Err("string-residence-replacement-result-missing");
                    }
                    if !results.contains_key(value) {
                        return Err("string-residence-install-result-missing");
                    }
                    if block
                        .terminator
                        .as_ref()
                        .is_some_and(|term| std::ptr::eq(term, instruction))
                    {
                        return Err("string-residence-install-terminator");
                    }
                }
                // A pair cannot be copied, boxed, PHI-merged, returned, stored,
                // passed to arbitrary calls or interpreted as one scalar word.
                let uses = instruction.used_values();
                for value in uses.iter().copied() {
                    if source.is_alias(value) {
                        let actual_uses = uses.iter().filter(|used| **used == value).count();
                        let permitted = if source.permits_alias(block.id, instruction)
                            || matches!(instruction, MirInstruction::KeepAlive { .. })
                        {
                            actual_uses
                        } else if source.permits_operation(block.id, instruction) {
                            match instruction {
                                MirInstruction::StringResidenceInstall {
                                    value: installed,
                                    replaces: Some(old),
                                    ..
                                } if *old == value
                                    && *installed != value
                                    && matches!(source.origin(value), Some(Origin::Slot(_))) =>
                                {
                                    1
                                }
                                MirInstruction::Invoke { operation, .. } => {
                                    let text = text_operands(operation).iter().filter(|operand|
                                        matches!(operand, Operand::TextOperand(used) if *used == value)
                                            && matches!(source.origin(value), Some(Origin::Slot(_)))).count();
                                    let end = usize::from(
                                        matches!(operation,
                                        InvokeOperation::String(Op::EndIfOwned { value: ended }) if *ended == value)
                                            && matches!(
                                                source.origin(value),
                                                Some(Origin::Slot(_))
                                            ),
                                    );
                                    text + end
                                }
                                _ => 0,
                            }
                        } else {
                            0
                        };
                        // used_values preserves occurrences. A Text borrow in
                        // one operand cannot authorize an integer/base/frame
                        // operand that happens to reuse the same ValueId.
                        if permitted != actual_uses {
                            return Err("string-owned-escape");
                        }
                    }
                    if !results.contains_key(&value) {
                        continue;
                    }
                    let allowed = matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::String(operation), .. }
                    if matches!(operation, Op::End { value: end } if *end == value)
                        || operation.operands().iter().any(|operand|
                            matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Invoke { operation, .. }
                    if text_operands(operation).iter().any(|operand| matches!(operand, Operand::OwnedString(owned) if *owned == value)));
                    // A typed physical Return may transfer only an installed Home;
                    // the path check below requires that exact live pair on Normal.
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Invoke {
                    operation: InvokeOperation::FieldSet { mode, value: stored, .. }, .. }
                    if mode.has_owned_text_candidate_v1() && *stored == value);
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::Return { value: Some(returned) }
                    if *returned == value && function.signature.return_type == crate::mir::MirType::String);
                    // KeepAlive is optimizer metadata, checked for SSA availability
                    // by the shared verifier; it performs no runtime pair access.
                    let allowed = allowed
                        || matches!(instruction, MirInstruction::KeepAlive { .. })
                        || matches!(instruction, MirInstruction::StringResidenceInstall { value: installed, replaces, .. } if *installed == value || *replaces == Some(value));
                    let allowed = allowed
                        || matches!(instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Call { call,
                        result: crate::mir::instruction::InvokeCallResultKind::OwnedString }, .. }
                        if call.text_argument_cleanup.contains(&value));
                    let allowed = allowed || source.permits_alias(block.id, instruction);
                    if !allowed && !is_call_argument(instruction, value) {
                        return Err("string-owned-escape");
                    }
                }
            }
            if block
                .return_env
                .as_ref()
                .is_some_and(|env| env.iter().any(|v| results.contains_key(v)))
            {
                return Err("string-owned-escape");
            }
        }
        let mut seen = BTreeMap::<(BasicBlockId, bool), Vec<Live>>::new();
        let mut pending = vec![(function.entry_block, Live::default(), false)];
        while let Some((id, mut live, faulted)) = pending.pop() {
            site = id;
            let previous = seen.entry((id, faulted)).or_default();
            if previous.contains(&live) {
                continue;
            }
            if previous
                .iter()
                .any(|old| !old.compatible_join(&live, source))
            {
                return Err("string-lifetime-join");
            }
            previous.push(live.clone());
            let block = function.blocks.get(&id).ok_or("string-block-missing")?;
            for row in &block.instructions {
                if let MirInstruction::StringResidenceInstall {
                    value,
                    replaces,
                    returning,
                } = row
                {
                    if *returning {
                        if faulted
                            || live.hold.is_some()
                            || live.replacement.is_some()
                            || replaces.is_some()
                            || !live.temporary.is_empty()
                            || !live.has_home(*value)
                        {
                            return Err("string-residence-install-not-newest-temporary");
                        }
                        live.hold = Some(*value);
                        continue;
                    }
                    if live.hold.is_some()
                        || live.call_pending.is_some()
                        || faulted
                        || live.replacement.is_some()
                        || !results.contains_key(value)
                        || live.has_home(*value)
                        || live.temporary.pop() != Some(*value)
                    {
                        return Err("string-residence-install-not-newest-temporary");
                    }
                    if let Some(previous) = replaces {
                        if *previous == *value
                            || !(live.has_home(*previous)
                                || (source.permits_operation(id, row)
                                    && live.descriptor_live(*previous, source)))
                        {
                            return Err("string-residence-replacement-not-live-home");
                        }
                        live.replacement = Some((*value, *previous));
                    }
                    live.homes.push(Residence::Home(*value));
                }
            }
            if matches!(
                block.terminator,
                Some(MirInstruction::StringResidenceInstall { .. })
            ) {
                return Err("string-residence-install-terminator");
            }
            let term = block
                .terminator
                .as_ref()
                .ok_or("string-terminator-missing")?;
            if live.replacement.is_some()
                && matches!(term, MirInstruction::Invoke { operation, .. }
            if !matches!(operation, InvokeOperation::String(Op::End { .. } | Op::EndIfOwned { .. })))
            {
                return Err("string-effect-before-replacement-end");
            }
            if let MirInstruction::Invoke { operation, .. } = term {
                if !text_operands(operation).is_empty()
                    && (faulted
                        || live.replacement.is_some()
                        || live.hold.is_some()
                        || live.call_pending.is_some())
                    && !matches!(
                        operation,
                        InvokeOperation::String(Op::End { .. } | Op::EndIfOwned { .. })
                    )
                {
                    return Err("string-effect-after-fault");
                }
                for operand in text_operands(operation) {
                    if matches!(operand, Operand::TextOperand(value) if !live.descriptor_live(*value, source))
                    {
                        return Err("string-borrow-not-live");
                    }
                    if matches!(operand, Operand::OwnedString(value) if !live.contains(value)) {
                        return Err("string-borrow-not-live");
                    }
                }
                if let InvokeOperation::String(Op::EndIfOwned { value }) = operation {
                    if !source.permits_operation(id, term)
                        || !live.end_carrier(*value, faulted, source)
                    {
                        return Err("string-end-order-or-not-live");
                    }
                }
                if let InvokeOperation::String(Op::End { value }) = operation {
                    if !live.end(*value, faulted, source.retained_for(id, term), source) {
                        return Err("string-end-order-or-not-live");
                    }
                }
            }
            if let MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, .. },
                ..
            } = term
            {
                if live.replacement.is_some() || live.hold.is_some() || live.call_pending.is_some()
                {
                    return Err("string-effect-before-replacement-end");
                }
                if call.text_argument_cleanup.len() > live.temporary.len()
                    || !call
                        .text_argument_cleanup
                        .iter()
                        .rev()
                        .eq(live.temporary[live
                            .temporary
                            .len()
                            .saturating_sub(call.text_argument_cleanup.len())..]
                            .iter())
                {
                    return Err("string-end-order-or-not-live");
                }
                for value in &call.args {
                    if results.contains_key(value) && (faulted || !live.contains(value)) {
                        return Err("string-call-borrow-not-live");
                    }
                }
            }
            if let MirInstruction::Return { value: Some(value) } = term {
                if results.contains_key(value) {
                    if function.signature.return_type != crate::mir::MirType::String
                        || faulted
                        || live.replacement.is_some()
                        || !live.temporary.is_empty()
                        || live
                            .homes
                            .iter()
                            .copied()
                            .filter(|home| live.active(*home))
                            .collect::<Vec<_>>()
                            != [Residence::Home(*value)]
                        || live.hold.is_some_and(|held| held != *value)
                    {
                        return Err("string-owned-escape");
                    }
                    live.homes.retain(|home| *home != Residence::Home(*value));
                    live.hold = None;
                }
            }
            let targets = block.successors_from_terminator();
            if targets.is_empty() && !live.is_empty() {
                outstanding = Some(format!("outstanding={live:?}"));
                return Err("string-missing-end");
            }
            for target in targets.into_iter().rev() {
                let mut next = live.clone();
                let mut next_faulted = faulted;
                if let MirInstruction::Invoke {
                    operation,
                    normal_landing,
                    fault_landing,
                    ..
                } = term
                {
                    next_faulted |= target == *fault_landing;
                    // Source corroboration remains mandatory in the module checker.
                    // Initialize transfers only on Normal. Text Replace takes
                    // the candidate at entry and disposes it internally on a
                    // precommit Fault; a committed slot retains it on old-End Fault.
                    if let InvokeOperation::FieldSet { mode, value, .. } = operation {
                        if mode.has_owned_text_candidate_v1() {
                            if faulted
                                || live.replacement.is_some()
                                || live.hold.is_some()
                                || live.call_pending.is_some()
                                || live.temporary.last() != Some(value)
                            {
                                return Err("string-borrow-not-live");
                            }
                            if target == *normal_landing || mode.consumes_text_candidate_on_fault_v1() {
                                next.temporary.pop();
                            }
                        }
                    }
                    if target == *normal_landing
                        && operation.normal_result_kind()
                            == Some(InvokeNormalResultKind::OwnedString)
                    {
                        let result = *producers.get(&id).ok_or("string-result-missing")?;
                        if next.contains(&result) {
                            return Err("string-result-duplicate");
                        }
                        let suffix = match operation {
                            InvokeOperation::Call { call, .. } => call.text_argument_cleanup.len(),
                            _ => 0,
                        };
                        next.temporary.insert(next.temporary.len() - suffix, result);
                        if suffix != 0 {
                            next.call_pending = Some(result);
                        }
                    }
                }
                if !next.edge(id, target, source) {
                    return Err("string-lifetime-join");
                }
                pending.push((target, next, next_faulted));
            }
        }
        if function.blocks.values().any(|block| {
            block
                .instructions
                .iter()
                .any(|row| matches!(row, MirInstruction::StringResidenceInstall { .. }))
                && !seen.keys().any(|(id, _)| *id == block.id)
        }) {
            return Err("string-residence-install-unreachable");
        }
        Ok(())
    })();
    result.map_err(|reason| (site, reason, outstanding))
}
