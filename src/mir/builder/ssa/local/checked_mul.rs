//! Function-owned consumers of the existing borrowed Mul source record.
//! Source identity is issued by the ordinary-new ledger; this box checks the
//! appended MIR identity and carries the record to final reuse verification.
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::builder::MirBuilder;
use crate::mir::normal_callable_semantic_package::BorrowedMulMaterializationV1 as Record;
use crate::mir::{BinaryOp as BinOp, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct CheckedMulReuseV1 {
    records: BTreeMap<ValueId, Rc<Record>>,
}

impl CheckedMulReuseV1 {
    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub(in crate::mir::builder) fn clear(&mut self) {
        self.records.clear();
    }

    pub(in crate::mir::builder) fn records(&self) -> impl Iterator<Item = &Rc<Record>> {
        self.records.values()
    }

    pub(in crate::mir::builder) fn verify(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("function-missing"))?;
        for record in self.records.values() {
            exact_definition(function, record)?;
        }
        Ok(())
    }
}

pub(in crate::mir::builder) fn install(
    builder: &mut MirBuilder,
    record: Rc<Record>,
) -> Result<(), String> {
    if builder.function_state.current_block != Some(record.original().0) {
        return Err(freeze("install-block"));
    }
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("function-missing"))?;
    exact_definition(function, &record)?;
    let records = &mut builder.function_state.checked_mul_reuse.records;
    if records.contains_key(&record.value()) {
        return Err(freeze("duplicate-install"));
    }
    records.insert(record.value(), record);
    Ok(())
}

fn exact_definition(function: &MirFunction, record: &Record) -> Result<(), String> {
    let value = record.value();
    let original = record.original();
    if !matches!(original.1, MirInstruction::BinOp { dst, op: BinOp::Mul, .. } if dst == value) {
        return Err(freeze("operation"));
    }
    let block = function
        .blocks
        .get(&original.0)
        .ok_or_else(|| freeze("definition-block"))?;
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
        return Err(freeze("definition-identity"));
    }
    Ok(())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][borrowed-mul/{reason}]")
}
