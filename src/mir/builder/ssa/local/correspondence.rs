//! Function-owned observations from the sole LocalSSA materialization writer.
//! These records carry physical provenance only, never scalar/source authority.
use crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

#[path = "phi_definition.rs"]
mod phi_definition;
use phi_definition::{PhysicalPhiDefinitionV1, PhysicalPhiPatchV1};

pub(in crate::mir) type PhysicalBinding = (BasicBlockId, MirInstruction);

#[derive(Debug, Clone, PartialEq)]
enum Definition {
    Instruction(PhysicalBinding),
    Phi(Rc<PhysicalPhiDefinitionV1>),
    Parameter { value: ValueId, index: usize },
}

impl PartialEq for PhysicalPhiDefinitionV1 {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}

impl Definition {
    fn resolved(&self) -> Result<Self, String> {
        match self { Self::Phi(phi) => Ok(Self::Instruction(phi.current()?.clone())), _ => Ok(self.clone()) }
    }
    fn binding(&self) -> Option<&PhysicalBinding> {
        match self { Self::Instruction(row) => Some(row), Self::Phi(phi) => phi.current().ok(), _ => None }
    }
}

#[derive(Debug)]
enum Step {
    Identity,
    Append(Vec<Rc<LocalSsaPhysicalRelationV1>>),
}

#[derive(Debug)]
pub(in crate::mir) struct LocalSsaPhysicalRelationV1 {
    from: ValueId,
    to: ValueId,
    source: Definition,
    target: Option<PhysicalBinding>,
    step: Step,
}

impl LocalSsaPhysicalRelationV1 {
    /// Lend only a real indexed Phi definition, retaining its writer-owned
    /// pending/completed/rollback state instead of cloning a provisional row.
    pub(in crate::mir) fn identity_phi_binding_v1(&self) -> Result<&PhysicalBinding, String> {
        if self.from != self.to || self.target.is_some() || !matches!(self.step, Step::Identity) {
            return Err(reject("not-identity-phi"));
        }
        match &self.source {
            Definition::Phi(phi) => phi.current(),
            _ => Err(reject("not-indexed-phi")),
        }
    }
    pub(in crate::mir) fn require_completed_v1(&self) -> Result<(), String> {
        if let Definition::Phi(phi) = &self.source { phi.require_completed()?; }
        if let Step::Append(children) = &self.step {
            for child in children { child.require_completed_v1()?; }
        }
        Ok(())
    }

    /// Resolve only a binding retained from this same writer observation.
    /// Final MIR is never a source for replacement evidence.
    pub(in crate::mir) fn resolve_observed_binding_v1(&self, original: &PhysicalBinding, require_completed: bool)
        -> Result<Option<PhysicalBinding>, String>
    {
        if let Definition::Phi(phi) = &self.source {
            if phi.resolves(original) {
                if require_completed { phi.require_completed()?; }
                return Ok(Some(phi.current()?.clone()));
            }
        }
        if let Step::Append(children) = &self.step {
            for child in children {
                if let Some(row) = child.resolve_observed_binding_v1(original, require_completed)? { return Ok(Some(row)); }
            }
        }
        Ok(None)
    }

    pub(in crate::mir) fn values(&self) -> (ValueId, ValueId) {
        (self.from, self.to)
    }

    /// Exact captured writer output; callers still need the original source loan.
    pub(in crate::mir) fn target_binding(&self) -> Option<&PhysicalBinding> {
        self.target.as_ref()
    }

    /// Complete observations for an enclosing source-owned instruction census.
    /// This grants no operand class and changes no removable-binding policy.
    pub(in crate::mir) fn append_observed_bindings_v1(&self, bindings: &mut Vec<PhysicalBinding>) {
        if let Some(original) = self.source.binding() {
            if !bindings.contains(original) { bindings.push(original.clone()); }
        }
        if let Some(target) = &self.target {
            if !bindings.contains(target) { bindings.push(target.clone()); }
        }
        if let Step::Append(children) = &self.step {
            for child in children { child.append_observed_bindings_v1(bindings); }
        }
    }

    pub(in crate::mir) fn append_required_bindings(&self, bindings: &mut Vec<PhysicalBinding>) {
        let mut append = |binding: &PhysicalBinding| {
            if !matches!(
                binding.1,
                MirInstruction::Const { .. } | MirInstruction::Copy { .. }
            ) && !bindings.contains(binding)
            {
                bindings.push(binding.clone());
            }
        };
        if let Some(original) = self.source.binding() {
            append(original);
        }
        if let Some(target) = &self.target {
            append(target);
        }
        if let Step::Append(children) = &self.step {
            for child in children {
                child.append_required_bindings(bindings);
            }
        }
    }
    pub(in crate::mir) fn verify(&self, function: &MirFunction) -> Result<(), String> {
        if definition(function, self.from).as_ref() != Some(&self.source.resolved()?) {
            return Err(reject("original-producer"));
        }
        if let Some(target) = &self.target {
            if definition(function, self.to).as_ref()
                != Some(&Definition::Instruction(target.clone()))
            {
                return Err(reject("materialized-producer"));
            }
        }
        self.verify_shape()?;
        if let Step::Append(children) = &self.step {
            for child in children {
                child.verify(function)?;
            }
        }
        Ok(())
    }

    /// Borrow the same captured finishing projection; it never issues source evidence.
    pub(in crate::mir) fn verify_projected(
        &self,
        function: &MirFunction,
        project: &mut impl FnMut(&PhysicalBinding) -> Result<Option<PhysicalBinding>, String>,
    ) -> Result<(), String> {
        self.verify_shape()?;
        let mut verify_binding = |original: &PhysicalBinding| -> Result<(), String> {
            match project(original)? {
                Some(mapped) => {
                    let value = mapped
                        .1
                        .dst_value()
                        .ok_or_else(|| reject("projected-dst"))?;
                    if !matches!(definition(function, value), Some(Definition::Instruction(ref actual)) if actual == &mapped) {
                        return Err(format!("{} function={} original={original:?} mapped={mapped:?} actual={:?}",
                            reject("projected-producer"), function.signature.name, definition(function, value)));
                    }
                }
                None if matches!(
                    original.1,
                    MirInstruction::Const { .. } | MirInstruction::Copy { .. }
                ) => {}
                None => return Err(reject("mandatory-producer-removed")),
            }
            Ok(())
        };
        match &self.source.resolved()? {
            Definition::Instruction(original) => verify_binding(original)?,
            Definition::Phi(_) => return Err(reject("unresolved-phi")),
            Definition::Parameter { value, index } => {
                if function.params.get(*index) != Some(value)
                    || definition(function, *value).as_ref() != Some(&self.source)
                {
                    return Err(reject("projected-parameter"));
                }
            }
        }
        if let Some(target) = &self.target {
            verify_binding(target)?;
        }
        if let Step::Append(children) = &self.step {
            for child in children {
                child.verify_projected(function, project)?;
            }
        }
        Ok(())
    }

    fn verify_shape(&self) -> Result<(), String> {
        let source = self.source.resolved()?;
        match &self.step {
            Step::Identity if self.from == self.to && self.target.is_none() => Ok(()),
            Step::Identity => Err(reject("identity")),
            Step::Append(children) => {
                let Some((_, target)) = &self.target else {
                    return Err(reject("append"));
                };
                match (&source, target) {
                    (
                        Definition::Instruction((
                            _,
                            MirInstruction::Const {
                                value: original, ..
                            },
                        )),
                        MirInstruction::Const {
                            value: returned, ..
                        },
                    ) if original == returned && children.is_empty() => Ok(()),
                    (
                        Definition::Instruction((
                            _,
                            MirInstruction::BinOp {
                                op: original_op,
                                lhs: original_lhs,
                                rhs: original_rhs,
                                ..
                            },
                        )),
                        MirInstruction::BinOp { op, lhs, rhs, .. },
                    ) if original_op == op && children.len() == 2 => {
                        if children[0].values() != (*original_lhs, *lhs)
                            || children[1].values() != (*original_rhs, *rhs)
                        {
                            return Err(reject("ordered-operands"));
                        }
                        Ok(())
                    }
                    (
                        Definition::Instruction((_, MirInstruction::Compare {
                            op: original_op, lhs: original_lhs, rhs: original_rhs, ..
                        })),
                        MirInstruction::Compare { op, lhs, rhs, .. },
                    ) if original_op == op && children.len() == 2 => {
                        if children[0].values() != (*original_lhs, *lhs)
                            || children[1].values() != (*original_rhs, *rhs)
                        {
                            return Err(reject("ordered-compare-operands"));
                        }
                        Ok(())
                    }
                    (
                        Definition::Instruction((_, MirInstruction::Copy { src: original, .. })),
                        MirInstruction::Copy { src, .. },
                    ) if children.len() == 1 => {
                        if children[0].values() != (*original, *src) {
                            return Err(reject("copy-operand"));
                        }
                        Ok(())
                    }
                    (_, MirInstruction::Copy { src, .. })
                        if *src == self.from && children.is_empty() =>
                    {
                        Ok(())
                    }
                    _ => Err(reject("append-shape")),
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub(in crate::mir) struct LocalSsaPhysicalIndexV1 {
    rows: BTreeMap<(ValueId, ValueId), Rc<LocalSsaPhysicalRelationV1>>,
    scalar_loans: BTreeMap<ValueId, Rc<OriginalScalarAppendV1>>,
    scalar_appends: BTreeMap<ValueId, (Rc<OriginalScalarAppendV1>, Rc<LocalSsaPhysicalRelationV1>)>,
    phis: BTreeMap<ValueId, Rc<PhysicalPhiDefinitionV1>>,
}

impl LocalSsaPhysicalIndexV1 {
    pub(in crate::mir::builder) fn prepare_provisional_phi_v1(
        &self, function: &MirFunction, block: BasicBlockId, dst: ValueId,
    ) -> Result<Rc<PhysicalPhiDefinitionV1>, String> {
        if self.phis.contains_key(&dst) || function.params.contains(&dst)
            || function.blocks.values().any(|body| body.all_instructions().any(|row| row.dst_value() == Some(dst))) {
            return Err(reject("phi-definition-origin"));
        }
        Ok(Rc::new(PhysicalPhiDefinitionV1::new(block, dst)))
    }

    pub(in crate::mir::builder) fn commit_provisional_phi_v1(&mut self, phi: Rc<PhysicalPhiDefinitionV1>) {
        let dst = phi.current().expect("new Phi observation is live").1.dst_value().expect("Phi defines dst");
        assert!(self.phis.insert(dst, phi).is_none(), "exclusive Phi definition preflight changed");
    }

    pub(in crate::mir::builder) fn prepare_phi_patch_v1(
        &self, function: &MirFunction, binding: PhysicalBinding,
    ) -> Result<Option<PhysicalPhiPatchV1>, String> {
        let dst = binding.1.dst_value().ok_or_else(|| reject("phi-completion-dst"))?;
        // Existing final-Phi patch callers never issued a provisional loan.
        // Tracked provisional loans, by contrast, must complete exactly once.
        let Some(phi) = self.phis.get(&dst) else { return Ok(None); };
        let before = instruction(function, dst).ok_or_else(|| reject("phi-completion-missing"))?;
        phi.prepare(&before, binding).map(Some)
    }

    pub(in crate::mir::builder) fn rollback_phi_v1(&self, dst: ValueId) {
        if let Some(phi) = self.phis.get(&dst) { phi.rollback(); }
    }

    fn observed_definition(&self, function: &MirFunction, value: ValueId) -> Option<Definition> {
        let actual = definition(function, value)?;
        if let Some(phi) = self.phis.get(&value) {
            if actual.binding() != phi.current().ok() { return None; }
            Some(Definition::Phi(Rc::clone(phi)))
        } else { Some(actual) }
    }

    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.scalar_loans.is_empty() && self.scalar_appends.is_empty() && self.phis.is_empty()
    }
    pub(in crate::mir::builder) fn clear(&mut self) {
        self.rows.clear();
        self.scalar_loans.clear();
        self.scalar_appends.clear();
        self.phis.clear();
    }

    pub(in crate::mir::builder) fn loan(
        &mut self,
        function: &MirFunction,
        from: ValueId,
        to: ValueId,
    ) -> Option<Rc<LocalSsaPhysicalRelationV1>> {
        if from == to && !self.rows.contains_key(&(from, to)) {
            let row = Rc::new(LocalSsaPhysicalRelationV1 {
                from,
                to,
                source: self.observed_definition(function, from)?,
                target: None,
                step: Step::Identity,
            });
            self.rows.insert((from, to), row);
        }
        self.rows.get(&(from, to)).cloned()
    }

    pub(in crate::mir) fn require_member(
        &self,
        row: &Rc<LocalSsaPhysicalRelationV1>,
    ) -> Result<(), String> {
        if self
            .rows
            .get(&row.values())
            .is_some_and(|current| Rc::ptr_eq(current, row))
        {
            Ok(())
        } else {
            Err(reject("foreign-index"))
        }
    }

    pub(in crate::mir::builder) fn selects_scalar(&self, value: ValueId) -> bool {
        self.scalar_loans.contains_key(&value)
    }

    pub(in crate::mir::builder) fn install_scalar_original(
        &mut self,
        function: &MirFunction,
        original: Rc<OriginalScalarAppendV1>,
    ) -> Result<(), String> {
        let value = original
            .original()
            .1
            .dst_value()
            .ok_or_else(|| reject("scalar-original-dst"))?;
        if self.scalar_loans.contains_key(&value) {
            return Err(reject("duplicate-scalar-install"));
        }
        if instruction(function, value).as_ref() != Some(original.original()) {
            return Err(reject("scalar-original-producer"));
        }
        for relation in original.operand_relations() {
            self.require_member(relation)?;
            relation.verify(function)?;
        }
        self.scalar_loans.insert(value, original);
        Ok(())
    }

    pub(super) fn record_selected_append(
        &mut self,
        function: &MirFunction,
        from: ValueId,
        to: ValueId,
    ) -> Result<(), String> {
        let Some(original) = self.scalar_loans.get(&from).cloned() else {
            return Ok(());
        };
        let relation = self
            .loan(function, from, to)
            .ok_or_else(|| reject("scalar-append-missing"))?;
        self.require_member(&relation)?;
        relation.verify(function)?;
        if self.scalar_appends.contains_key(&to) || self.scalar_loans.contains_key(&to) {
            return Err(reject("duplicate-scalar-target"));
        }
        self.scalar_appends
            .insert(to, (Rc::clone(&original), relation));
        self.scalar_loans.insert(to, original);
        Ok(())
    }

    pub(in crate::mir) fn scalar_source_for(
        &self,
        value: ValueId,
    ) -> Option<&Rc<OriginalScalarAppendV1>> {
        self.scalar_loans.get(&value)
    }

    pub(in crate::mir) fn scalar_originals(
        &self,
    ) -> impl Iterator<Item = &Rc<OriginalScalarAppendV1>> {
        self.scalar_loans.iter().filter_map(|(value, row)| {
            (row.original().1.dst_value() == Some(*value)).then_some(row)
        })
    }

    pub(in crate::mir) fn scalar_observations(
        &self,
    ) -> impl Iterator<Item = &(Rc<OriginalScalarAppendV1>, Rc<LocalSsaPhysicalRelationV1>)> {
        self.scalar_appends.values()
    }

    /// Every non-original loan must have exactly one observed writer append.
    pub(in crate::mir) fn validate_scalar_consumer_coverage(
        &self,
        observed: &BTreeSet<ValueId>,
    ) -> Result<(), String> {
        let expected: BTreeSet<_> = self
            .scalar_loans
            .iter()
            .filter_map(|(value, row)| {
                (row.original().1.dst_value() != Some(*value)).then_some(*value)
            })
            .collect();
        let recorded: BTreeSet<_> = self.scalar_appends.keys().copied().collect();
        if expected != recorded || expected != *observed {
            return Err(reject("scalar-consumer-coverage"));
        }
        Ok(())
    }

    /// An absent/ambiguous producer never adds a legacy rejection or a loan.
    /// Observe the sole pin writer's actual Copy before slot redirection can
    /// reuse it. This is physical provenance, with no scalar/type authority.
    pub(in crate::mir::builder) fn record_pin_copy(
        &mut self, function: &MirFunction, from: ValueId, to: ValueId, block: BasicBlockId,
    ) -> Result<(), String> {
        if !matches!(instruction(function, to), Some((id, MirInstruction::Copy { dst, src }))
            if id == block && dst == to && src == from)
        {
            return Err(reject("pin-copy-writer"));
        }
        self.record_append(function, from, instruction(function, from), to, block, None);
        self.loan(function, from, to).ok_or_else(|| reject("pin-copy-observation"))?
            .verify(function)
    }

    pub(super) fn record_append(
        &mut self,
        function: &MirFunction,
        from: ValueId,
        original: Option<PhysicalBinding>,
        to: ValueId,
        block: BasicBlockId,
        ordered: Option<Vec<(ValueId, ValueId)>>,
    ) {
        let mut source = match original {
            Some(binding) if binding.1.dst_value() == Some(from) => {
                Definition::Instruction(binding)
            }
            Some(_) => return,
            None => {
                let Some(source) = definition(function, from) else {
                    return;
                };
                if !matches!(source, Definition::Parameter { .. }) {
                    return;
                }
                source
            }
        };
        if let Some(phi) = self.phis.get(&from) {
            if source.binding() != phi.current().ok() { return; }
            source = Definition::Phi(Rc::clone(phi));
        }
        let Some(row) = function.blocks.get(&block).and_then(|row| {
            row.all_instructions()
                .find(|row| row.dst_value() == Some(to))
        }) else {
            return;
        };
        let target = (block, row.clone());
        let children = match ordered {
            Some(pairs) => {
                let mut children = Vec::new();
                for (input, returned) in pairs {
                    let Some(child) = self.loan(function, input, returned) else {
                        return;
                    };
                    children.push(child);
                }
                children
            }
            None => Vec::new(),
        };
        let row = Rc::new(LocalSsaPhysicalRelationV1 {
            from,
            to,
            source,
            target: Some(target),
            step: Step::Append(children),
        });
        if row.verify_shape().is_ok() {
            self.rows.entry((from, to)).or_insert(row);
        }
    }
}

fn definition(function: &MirFunction, value: ValueId) -> Option<Definition> {
    let has_instruction = function.blocks.values().any(|block| {
        block
            .all_instructions()
            .any(|row| row.dst_value() == Some(value))
    });
    if has_instruction {
        if function.params.contains(&value) {
            return None;
        }
        return instruction(function, value).map(Definition::Instruction);
    }
    let mut rows = function
        .params
        .iter()
        .enumerate()
        .filter(|(_, parameter)| **parameter == value);
    let (index, _) = rows.next()?;
    if rows.next().is_some() {
        return None;
    }
    Some(Definition::Parameter { value, index })
}

fn instruction(function: &MirFunction, value: ValueId) -> Option<PhysicalBinding> {
    let mut rows = function
        .blocks
        .iter()
        .flat_map(|(block, row)| {
            row.all_instructions()
                .map(move |instruction| (*block, instruction))
        })
        .filter(|(_, row)| row.dst_value() == Some(value));
    let (block, row) = rows.next()?;
    if rows.next().is_some() {
        return None;
    }
    Some((block, row.clone()))
}

fn reject(reason: &str) -> String {
    format!("[freeze:contract][local-ssa-correspondence/{reason}]")
}

#[cfg(test)]
#[path = "correspondence_tests.rs"]
mod tests;
