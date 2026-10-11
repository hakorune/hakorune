//! Function-owned observations from the sole LocalSSA materialization writer.
//! These records carry physical provenance only, never scalar/source authority.
use crate::mir::normal_callable_semantic_package::OriginalScalarAppendV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

pub(in crate::mir) type PhysicalBinding = (BasicBlockId, MirInstruction);

#[derive(Debug, Clone, PartialEq)]
enum Definition {
    Instruction(PhysicalBinding),
    Parameter { value: ValueId, index: usize },
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
    pub(in crate::mir) fn values(&self) -> (ValueId, ValueId) {
        (self.from, self.to)
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
        if let Definition::Instruction(original) = &self.source {
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
        if definition(function, self.from).as_ref() != Some(&self.source) {
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
                    if definition(function, value).as_ref()
                        != Some(&Definition::Instruction(mapped))
                    {
                        return Err(reject("projected-producer"));
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
        match &self.source {
            Definition::Instruction(original) => verify_binding(original)?,
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
        match &self.step {
            Step::Identity if self.from == self.to && self.target.is_none() => Ok(()),
            Step::Identity => Err(reject("identity")),
            Step::Append(children) => {
                let Some((_, target)) = &self.target else {
                    return Err(reject("append"));
                };
                match (&self.source, target) {
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
}

impl LocalSsaPhysicalIndexV1 {
    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.rows.is_empty() && self.scalar_loans.is_empty() && self.scalar_appends.is_empty()
    }
    pub(in crate::mir::builder) fn clear(&mut self) {
        self.rows.clear();
        self.scalar_loans.clear();
        self.scalar_appends.clear();
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
                source: definition(function, from)?,
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
    pub(super) fn record_append(
        &mut self,
        function: &MirFunction,
        from: ValueId,
        original: Option<PhysicalBinding>,
        to: ValueId,
        block: BasicBlockId,
        ordered: Option<Vec<(ValueId, ValueId)>>,
    ) {
        let source = match original {
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
