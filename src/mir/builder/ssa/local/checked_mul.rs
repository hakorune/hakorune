//! Function-owned consumers of the existing borrowed Mul source record.
//! Source identity is issued by the ordinary-new ledger; this box checks the
//! appended MIR identity and carries the record to final reuse verification.
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::builder::MirBuilder;
use crate::mir::normal_callable_semantic_package::BorrowedMulMaterializationV1 as Record;
use crate::mir::{BasicBlockId, BinaryOp as BinOp, MirFunction, MirInstruction, ValueId};

type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct CheckedMulReuseV1 {
    records: BTreeMap<ValueId, Rc<Record>>,
    operand_copies: BTreeMap<ValueId, [Option<Binding>; 2]>,
}

impl CheckedMulReuseV1 {
    pub(in crate::mir::builder) fn is_empty(&self) -> bool {
        self.records.is_empty() && self.operand_copies.is_empty()
    }

    pub(in crate::mir::builder) fn clear(&mut self) {
        self.records.clear();
        self.operand_copies.clear();
    }

    pub(in crate::mir::builder) fn records(&self) -> impl Iterator<Item = &Rc<Record>> {
        self.records.values()
    }

    pub(in crate::mir::builder) fn observations(
        &self,
    ) -> impl Iterator<Item = (&Rc<Record>, &[Option<Binding>; 2])> {
        self.records
            .iter()
            .map(|(value, record)| (record, &self.operand_copies[value]))
    }

    pub(in crate::mir::builder) fn verify(&self, builder: &MirBuilder) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| freeze("function-missing"))?;
        for record in self.records.values() {
            exact_definition(function, record)?;
            if self.operand_copies.get(&record.value())
                != Some(&original_operand_copies(function, record)?)
            {
                return Err(freeze("operand-copy-drift"));
            }
        }
        if self.operand_copies.len() != self.records.len() {
            return Err(freeze("operand-copy-coverage"));
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
    let copies = original_operand_copies(function, &record)?;
    let records = &mut builder.function_state.checked_mul_reuse.records;
    if records.contains_key(&record.value())
        || builder
            .function_state
            .checked_mul_reuse
            .operand_copies
            .contains_key(&record.value())
    {
        return Err(freeze("duplicate-install"));
    }
    let value = record.value();
    records.insert(value, record);
    builder
        .function_state
        .checked_mul_reuse
        .operand_copies
        .insert(value, copies);
    Ok(())
}

fn original_operand_copies(
    function: &MirFunction,
    record: &Record,
) -> Result<[Option<Binding>; 2], String> {
    let (block_id, instruction) = record.original();
    let MirInstruction::BinOp {
        lhs,
        rhs,
        op: BinOp::Mul,
        ..
    } = instruction
    else {
        return Err(freeze("operation"));
    };
    let block = function
        .blocks
        .get(block_id)
        .ok_or_else(|| freeze("definition-block"))?;
    let rows: Vec<_> = block.all_instructions().collect();
    let mul_index = rows
        .iter()
        .position(|row| *row == instruction)
        .ok_or_else(|| freeze("definition-identity"))?;
    let children = [record.children().0, record.children().1];
    let operands = [*lhs, *rhs];
    let mut result = [None, None];
    for side in 0..2 {
        if operands[side] == children[side] {
            continue;
        }
        let mut copies = rows[..mul_index].iter().filter(|row| {
            matches!(row, MirInstruction::Copy { dst, src }
                if *dst == operands[side] && *src == children[side])
        });
        let copy = copies
            .next()
            .ok_or_else(|| freeze("operand-copy-missing"))?;
        if copies.next().is_some()
            || function
                .blocks
                .values()
                .flat_map(|row| row.all_instructions())
                .filter(|row| row.dst_value() == Some(operands[side]))
                .count()
                != 1
        {
            return Err(freeze("operand-copy-duplicate"));
        }
        result[side] = Some((*block_id, (*copy).clone()));
    }
    Ok(result)
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

/// Physical identity reuse only; the selected Mul source still owns typing.
pub(super) fn reuse_same_block_operand(
    function: &MirFunction,
    block: BasicBlockId,
    value: ValueId,
) -> Result<bool, String> {
    let body = function.blocks.get(&block).ok_or_else(|| freeze("operand-block"))?;
    let definitions = |value| {
        function.blocks.iter().flat_map(|(id, body)| {
            body.all_instructions().enumerate().filter_map(move |(index, row)| {
                (row.dst_value() == Some(value)).then_some((*id, index, row))
            })
        }).collect::<Vec<_>>()
    };
    let rows = definitions(value);
    if !rows.iter().any(|(id, _, row)| *id == block
        && matches!(row, MirInstruction::Const { .. } | MirInstruction::Copy { .. }))
    {
        return Ok(false);
    }
    if rows.len() != 1 || function.params.contains(&value) {
        return Err(freeze("operand-definition"));
    }
    if body.terminator.is_some() {
        return Err(freeze("operand-block-closed"));
    }
    let dominators = crate::mir::verification::utils::compute_dominators(function);
    let mut current = rows[0];
    let mut visited = std::collections::BTreeSet::new();
    while let MirInstruction::Copy { dst, src } = current.2 {
        if !visited.insert(*dst) {
            return Err(freeze("operand-copy-cycle"));
        }
        let sources = definitions(*src);
        let parameters = function.params.iter().filter(|value| **value == *src).count();
        if parameters == 1 && sources.is_empty() {
            break;
        }
        if parameters != 0 || sources.len() != 1 {
            return Err(freeze("operand-copy-source"));
        }
        let source = sources[0];
        if source.0 == current.0 {
            if source.1 >= current.1 {
                return Err(freeze("operand-copy-order"));
            }
        } else if !dominators.is_reachable(source.0)
            || !dominators.is_reachable(current.0)
            || !dominators.dominates(source.0, current.0)
        {
            return Err(freeze("operand-copy-dominance"));
        }
        current = source;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

    #[test]
    fn selected_mul_operand_identity_requires_unique_ordered_open_definition() {
        let mut function = MirFunction::new(FunctionSignature {
            name: "operand/1".into(), params: vec![MirType::Integer],
            return_type: MirType::Integer, effects: EffectMask::PURE,
        }, BasicBlockId(0));
        function.params = vec![ValueId(0)];
        function.blocks.get_mut(&BasicBlockId(0)).unwrap().instructions.extend([
            MirInstruction::Const { dst: ValueId(1), value: ConstValue::Integer(-2) },
            MirInstruction::Copy { dst: ValueId(2), src: ValueId(1) },
            MirInstruction::Copy { dst: ValueId(3), src: ValueId(0) },
        ]);
        for value in [1, 2, 3] {
            assert!(reuse_same_block_operand(&function, BasicBlockId(0), ValueId(value)).unwrap());
        }
        assert!(!reuse_same_block_operand(&function, BasicBlockId(0), ValueId(0)).unwrap());
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        assert!(!reuse_same_block_operand(&function, BasicBlockId(1), ValueId(1)).unwrap());
        for mutation in 0..5 {
            let mut changed = function.clone();
            let body = changed.blocks.get_mut(&BasicBlockId(0)).unwrap();
            let expected = match mutation {
                0 => {
                    body.instructions.push(body.instructions[0].clone());
                    "operand-definition"
                }
                1 => {
                    body.terminator = Some(MirInstruction::Return { value: Some(ValueId(1)) });
                    "operand-block-closed"
                }
                2 => {
                    body.instructions.swap(0, 1);
                    "operand-copy-order"
                }
                3 => {
                    body.instructions[1] = MirInstruction::Copy { dst: ValueId(2), src: ValueId(999) };
                    "operand-copy-source"
                }
                _ => {
                    body.instructions[1] = MirInstruction::Copy { dst: ValueId(2), src: ValueId(2) };
                    "operand-copy-order"
                }
            };
            let value = if mutation < 2 { 1 } else { 2 };
            let error = reuse_same_block_operand(&changed, BasicBlockId(0), ValueId(value)).unwrap_err();
            assert!(error.contains(expected), "mutation={mutation}: {error}");
        }
        function.blocks.get_mut(&BasicBlockId(0)).unwrap().set_terminator(
            MirInstruction::Jump { target: BasicBlockId(1), edge_args: None },
        );
        function.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions.push(
            MirInstruction::Copy { dst: ValueId(4), src: ValueId(1) },
        );
        assert!(reuse_same_block_operand(&function, BasicBlockId(1), ValueId(4)).unwrap());
        function.add_block(BasicBlock::new(BasicBlockId(2)));
        function.blocks.get_mut(&BasicBlockId(2)).unwrap().instructions.push(
            MirInstruction::Const { dst: ValueId(5), value: ConstValue::Integer(-2) },
        );
        function.blocks.get_mut(&BasicBlockId(1)).unwrap().instructions[0] =
            MirInstruction::Copy { dst: ValueId(4), src: ValueId(5) };
        assert!(reuse_same_block_operand(&function, BasicBlockId(1), ValueId(4))
            .unwrap_err().contains("operand-copy-dominance"));
    }
}
