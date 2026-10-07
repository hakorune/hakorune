//! Selected checked Compare consumers reuse the same completed Bool, never replay.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

use super::copy_type::PreparedLocalSsaPhysicalCopyTypeV1;
use super::error::LocalSsaMaterializationErrorV1 as Error;
use super::post_success::{
    LocalSsaMaterializationKindV1, LocalSsaPhysicalCopyReasonV1, LocalSsaSourceTypeEntryV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::normal_callable_semantic_package::BorrowedCompareMaterializationV1 as Record;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};

use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState as Entry;

#[path = "checked_compare_literal.rs"]
mod literal;
pub(in crate::mir::builder) use literal::install_literal;

#[path = "checked_compare_carrier.rs"]
mod carrier;
pub(in crate::mir::builder) use carrier::install_carriers;

type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
struct BoolCopy {
    source: Rc<Record>,
    original: Binding,
}

/// One function-owned consumer of the SAME immutable ledger records. Copy
/// observations are local materialization history, not a second source issuer.
#[derive(Debug, Default)]
pub(in crate::mir::builder) struct CheckedCompareReuseV1 {
    entry: Option<Weak<RefCell<Entry>>>,
    literals: literal::LiteralReuse,
    carriers: carrier::CarrierReuse,
    records: BTreeMap<ValueId, Rc<Record>>,
    copies: BTreeMap<ValueId, BoolCopy>,
    branches: Vec<(Rc<Record>, Binding)>,
}
impl CheckedCompareReuseV1 {
    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.carriers.is_empty()
            && self.literals.is_empty()
            && self.entry.is_none()
            && self.records.is_empty()
            && self.copies.is_empty()
            && self.branches.is_empty()
    }
    pub(in crate::mir::builder) fn clear(&mut self) {
        self.carriers = carrier::CarrierReuse::default();
        self.literals = literal::LiteralReuse::default();
        self.entry = None;
        self.records.clear();
        self.copies.clear();
        self.branches.clear();
    }
    pub(in crate::mir::builder) fn adopt_entry(
        &mut self,
        entry: Rc<RefCell<Entry>>,
    ) -> Result<(), String> {
        entry.borrow().checked_compare_entry_owner_v1()?;
        if !self.is_empty() {
            return Err(fault("entry-nonempty"));
        }
        self.entry = Some(Rc::downgrade(&entry));
        Ok(())
    }
    pub(in crate::mir::builder) fn require_same_entry(
        &self,
        entry: &Rc<RefCell<Entry>>,
    ) -> Result<(), String> {
        if self
            .entry
            .as_ref()
            .is_some_and(|installed| !Weak::ptr_eq(installed, &Rc::downgrade(entry)))
        {
            return Err(fault("entry-drift"));
        }
        if !self.records.is_empty() && self.entry.is_none() {
            return Err(fault("entry-missing"));
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn contains(&self, value: ValueId) -> bool {
        self.literals.contains(value)
            || self.records.contains_key(&value)
            || self.copies.contains_key(&value)
    }
    pub(in crate::mir::builder) fn contains_operand(&self, value: ValueId) -> bool {
        self.contains(value) || self.carriers.contains(value)
    }
    pub(in crate::mir::builder) fn carrier_observations(
        &self,
    ) -> impl Iterator<
        Item = (
            &Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareCarrierOperandLoanV1>,
            Vec<Binding>,
        ),
    > {
        self.carriers.observations()
    }
    pub(in crate::mir::builder) fn records(&self) -> impl Iterator<Item = &Rc<Record>> {
        self.records.values()
    }
    pub(in crate::mir::builder) fn literal_observations(&self) -> impl Iterator<Item = (&Rc<crate::mir::normal_callable_semantic_package::BorrowedCompareIntegerLiteralMaterializationV1>, Vec<Binding>)>{
        self.literals.observations()
    }
    pub(in crate::mir::builder) fn observations(
        &self,
    ) -> impl Iterator<Item = (&Rc<Record>, Vec<Binding>, Vec<Binding>)> {
        self.records.values().map(|record| {
            (
                record,
                self.copies
                    .values()
                    .filter(|copy| Rc::ptr_eq(&copy.source, record))
                    .map(|copy| copy.original.clone())
                    .collect(),
                self.branches
                    .iter()
                    .filter(|(source, _)| Rc::ptr_eq(source, record))
                    .map(|(_, binding)| binding.clone())
                    .collect(),
            )
        })
    }
    fn source(&self, value: ValueId) -> Option<(Rc<Record>, Binding)> {
        if let Some(record) = self.records.get(&value) {
            return Some((Rc::clone(record), record.original().clone()));
        }
        self.copies
            .get(&value)
            .map(|copy| (Rc::clone(&copy.source), copy.original.clone()))
    }
    pub(in crate::mir::builder) fn verify(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| fault("function-missing"))?;
        self.carriers.verify(builder)?;
        self.literals.verify(builder)?;
        for record in self.records.values() {
            check_owner(builder, record)?;
            exact_definition(function, record.value(), record.original())?;
        }
        for (value, copy) in &self.copies {
            check_owner(builder, &copy.source)?;
            exact_definition(function, *value, &copy.original)?;
            if !matches!(copy.original.1, MirInstruction::Copy { dst, src }
                if dst == *value && src == copy.source.value())
            {
                return Err(fault("copy-identity"));
            }
            require_definition_order(function, copy.source.original(), &copy.original)?;
        }
        for (record, branch) in &self.branches {
            check_owner(builder, record)?;
            let MirInstruction::Branch { condition, .. } = &branch.1 else {
                return Err(fault("branch-kind"));
            };
            let (source, definition) = self
                .source(*condition)
                .ok_or_else(|| fault("branch-condition"))?;
            if !Rc::ptr_eq(&source, record)
                || function
                    .blocks
                    .get(&branch.0)
                    .and_then(|block| block.terminator.as_ref())
                    != Some(&branch.1)
            {
                return Err(fault("branch-identity"));
            }
            require_dominance(function, definition.0, branch.0)?;
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][checked-compare-reuse/{reason}]")
}
fn check_entry_owner(
    builder: &MirBuilder,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<(), String> {
    let entry = builder
        .function_state
        .checked_compare_reuse
        .entry
        .as_ref()
        .and_then(Weak::upgrade)
        .ok_or_else(|| fault("entry-missing"))?;
    let entry = entry.try_borrow().map_err(|_| fault("entry-borrow"))?;
    if entry.checked_compare_entry_owner_v1()? != owner {
        return Err(fault("owner"));
    }
    Ok(())
}
fn check_owner(builder: &MirBuilder, record: &Record) -> Result<(), String> {
    check_entry_owner(builder, record.owner())?;
    if builder.function_state.type_ctx.get_type(record.value()) != Some(&MirType::Bool) {
        return Err(fault("bool-type"));
    }
    Ok(())
}

/// Exact tuple and sole destination definition in this function. This checks
/// physical identity only; source membership was checked by the original ledger.
fn exact_definition(
    function: &MirFunction,
    value: ValueId,
    original: &Binding,
) -> Result<(), String> {
    let block = function
        .blocks
        .get(&original.0)
        .ok_or_else(|| fault("definition-block"))?;
    if block
        .instructions
        .iter()
        .filter(|instruction| *instruction == &original.1)
        .count()
        != 1
        || function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|instruction| instruction.dst_value() == Some(value))
            .count()
            != 1
    {
        return Err(fault("definition-identity"));
    }
    Ok(())
}

fn require_dominance(
    function: &MirFunction,
    definition: BasicBlockId,
    consumer: BasicBlockId,
) -> Result<(), String> {
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    if !dominators.is_reachable(definition)
        || !dominators.is_reachable(consumer)
        || !dominators.dominates(definition, consumer)
    {
        return Err(fault("dominance"));
    }
    Ok(())
}

/// Existing copies must follow their defining Compare, including within a block.
fn require_definition_order(
    function: &MirFunction,
    source: &Binding,
    copy: &Binding,
) -> Result<(), String> {
    require_dominance(function, source.0, copy.0)?;
    if source.0 == copy.0 {
        let instructions = &function.blocks[&source.0].instructions;
        let source_index = instructions
            .iter()
            .position(|instruction| instruction == &source.1)
            .ok_or_else(|| fault("definition-identity"))?;
        let copy_index = instructions
            .iter()
            .position(|instruction| instruction == &copy.1)
            .ok_or_else(|| fault("definition-identity"))?;
        if source_index >= copy_index {
            return Err(fault("definition-order"));
        }
    }
    Ok(())
}

pub(in crate::mir::builder) fn install(
    builder: &mut MirBuilder,
    record: Rc<Record>,
) -> Result<(), String> {
    check_owner(builder, &record)?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    if builder.function_state.current_block != Some(record.original().0)
        || !matches!(record.original().1, MirInstruction::Compare { dst, .. } if dst == record.value())
    {
        return Err(fault("install-identity"));
    }
    exact_definition(function, record.value(), record.original())?;
    let state = &mut builder.function_state.checked_compare_reuse;
    if state.records.contains_key(&record.value()) || state.copies.contains_key(&record.value()) {
        return Err(fault("duplicate-install"));
    }
    state.records.insert(record.value(), record);
    Ok(())
}

/// Called before generic cache/pin paths. Cross-block reuse is a Copy of the
/// original post-Normal Bool; original checked operations are never cloned.
pub(super) fn materialize(builder: &mut MirBuilder, value: ValueId) -> Result<ValueId, Error> {
    if builder
        .function_state
        .checked_compare_reuse
        .literals
        .contains(value)
    {
        return literal::materialize(builder, value);
    }
    let (record, definition) = builder
        .function_state
        .checked_compare_reuse
        .source(value)
        .ok_or_else(|| Error::Contract(fault("record-missing")))?;
    check_owner(builder, &record).map_err(Error::Contract)?;
    let consumer = builder
        .function_state
        .current_block
        .ok_or_else(|| Error::Contract(fault("consumer-block")))?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| Error::Contract(fault("function-missing")))?;
    if function
        .blocks
        .get(&consumer)
        .is_none_or(|block| block.terminator.is_some())
    {
        return Err(Error::Contract(fault("consumer-closed")));
    }
    exact_definition(function, record.value(), record.original()).map_err(Error::Contract)?;
    exact_definition(function, value, &definition).map_err(Error::Contract)?;
    if value != record.value() {
        require_definition_order(function, record.original(), &definition)
            .map_err(Error::Contract)?;
    }
    require_dominance(function, record.original().0, consumer).map_err(Error::Contract)?;
    require_dominance(function, definition.0, consumer).map_err(Error::Contract)?;
    if definition.0 == consumer {
        // The consumer appends after existing instructions in this open block.
        return Ok(value);
    }
    let destination = builder.next_value_id();
    let prepared_type = PreparedLocalSsaPhysicalCopyTypeV1::prepare(
        &LocalSsaSourceTypeEntryV1::Exact(MirType::Bool),
        LocalSsaMaterializationKindV1::PhysicalCopy(
            LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
        ),
        builder.function_state.type_ctx.get_type(destination),
    )
    .map_err(|error| Error::Contract(error.to_string()))?;
    let instruction = MirInstruction::Copy {
        dst: destination,
        src: record.value(),
    };
    builder
        .emit_instruction(instruction.clone())
        .map_err(Error::InstructionEmission)?;
    prepared_type.commit(destination, &mut builder.function_state.type_ctx);
    builder.function_state.checked_compare_reuse.copies.insert(
        destination,
        BoolCopy {
            source: record,
            original: (consumer, instruction),
        },
    );
    Ok(destination)
}

/// Observe only the successfully sealed real conditional terminator.
pub(in crate::mir::builder) fn observe_branch(
    builder: &mut MirBuilder,
    block: BasicBlockId,
    condition: ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
) -> Result<(), String> {
    let Some((record, definition)) = builder
        .function_state
        .checked_compare_reuse
        .source(condition)
    else {
        return Ok(());
    };
    check_owner(builder, &record)?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?;
    let original = function
        .blocks
        .get(&block)
        .and_then(|block| block.terminator.clone())
        .ok_or_else(|| fault("branch-missing"))?;
    if !matches!(&original, MirInstruction::Branch { condition: actual, then_bb: actual_then, else_bb: actual_else, .. } if *actual == condition && *actual_then == then_bb && *actual_else == else_bb)
    {
        return Err(fault("branch-identity"));
    }
    exact_definition(function, condition, &definition)?;
    require_dominance(function, definition.0, block)?;
    let state = &mut builder.function_state.checked_compare_reuse;
    if state.branches.iter().any(|(_, binding)| binding.0 == block) {
        return Err(fault("duplicate-branch"));
    }
    state.branches.push((record, (block, original)));
    Ok(())
}

#[cfg(test)]
#[path = "checked_compare_tests.rs"]
mod tests;

pub(super) fn materialize_carrier(
    builder: &mut MirBuilder,
    value: ValueId,
) -> Result<ValueId, Error> {
    carrier::materialize(builder, value)
}
