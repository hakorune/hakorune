//! Fresh verifier evidence for eliding one exact local write in a physical artifact.
use super::{geometry::FunctionVerificationGeometry, scalar_origin::FinishedI64OriginsV1};
use crate::mir::type_contracts::local_slot::{
    permits_i64_write_elision, validate_local_slot_contracts,
};
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub(crate) struct VerifiedLocalSlotWrites<'module> {
    functions: HashMap<&'module str, FunctionWrites<'module>>,
}

#[derive(Debug)]
struct FunctionWrites<'module> {
    function: &'module MirFunction,
    rows: HashMap<(BasicBlockId, usize), &'module MirInstruction>,
}

impl<'module> VerifiedLocalSlotWrites<'module> {
    pub(crate) fn verify(module: &'module MirModule) -> Self {
        let mut proofs = Self::default();
        for (name, function) in &module.functions {
            if !function
                .metadata
                .local_slot_contracts
                .iter()
                .any(|c| c.proof_elision_allowed)
                || validate_local_slot_contracts(function).is_err()
            {
                continue;
            }
            let geometry = FunctionVerificationGeometry::new(function);
            if FinishedI64OriginsV1::new(&geometry, Some(module))
                .check_function()
                .is_err()
            {
                continue;
            }
            let mut rows = HashMap::new();
            for (&block, body) in &function.blocks {
                if !geometry.reachable().contains(&block) {
                    continue;
                }
                for (index, row) in body.instructions.iter().enumerate() {
                    let MirInstruction::LocalContractWrite {
                        local_slot_id, src, ..
                    } = row
                    else {
                        continue;
                    };
                    if permits_i64_write_elision(function, *local_slot_id)
                        && verify_i64_source(&geometry, *src)
                    {
                        rows.insert((block, index), row);
                    }
                }
            }
            if !rows.is_empty() {
                proofs
                    .functions
                    .insert(name, FunctionWrites { function, rows });
            }
        }
        proofs
    }

    pub(crate) fn copy_operands(
        &self,
        function: &MirFunction,
        block: BasicBlockId,
        index: usize,
        row: &MirInstruction,
    ) -> Option<(ValueId, ValueId)> {
        let proof = self.functions.get(function.signature.name.as_str())?;
        if !std::ptr::eq(proof.function, function)
            || !std::ptr::eq(*proof.rows.get(&(block, index))?, row)
        {
            return None;
        }
        match row {
            MirInstruction::LocalContractWrite { dst, src, .. } => Some((*dst, *src)),
            _ => None,
        }
    }
}

enum I64Producer {
    Literal,
    Forward(ValueId),
    Add(ValueId, ValueId),
}

fn scalar_producer(
    geometry: &FunctionVerificationGeometry<'_>,
    block: BasicBlockId,
    index: usize,
    row: &MirInstruction,
) -> Option<I64Producer> {
    match row {
        MirInstruction::Const {
            value: ConstValue::Integer(_),
            ..
        } => Some(I64Producer::Literal),
        MirInstruction::Copy { src, .. } => Some(I64Producer::Forward(*src)),
        // Follow the input, including for a reassignment. The destination's
        // local-contract fact is never the evidence for a proof.
        MirInstruction::LocalContractWrite {
            src, local_slot_id, ..
        } if permits_i64_write_elision(geometry.as_function(), *local_slot_id) => {
            Some(I64Producer::Forward(*src))
        }
        MirInstruction::BinOp {
            op: BinaryOp::Add,
            lhs,
            rhs,
            ..
        } if !geometry
            .metadata
            .exact_numeric_binary_op_route_facts
            .iter()
            .any(|fact| fact.block == block && fact.instruction_index == index) =>
        {
            // Both operands must be established independently by this query.
            // Current signed i64 Add preserves the domain with its existing
            // modulo law. An exact-width checked route needs separate coverage.
            Some(I64Producer::Add(*lhs, *rhs))
        }
        _ => None,
    }
}

fn is_add_recurrence(geometry: &FunctionVerificationGeometry<'_>, mut value: ValueId) -> bool {
    let mut seen = HashSet::new();
    while seen.insert(value) {
        let Some((block, index, row)) = geometry.definition(value) else {
            return false;
        };
        match scalar_producer(geometry, block, index, row) {
            Some(I64Producer::Forward(source)) => value = source,
            Some(I64Producer::Add(..)) => return true,
            _ => return false,
        }
    }
    false
}

/// Check all producers, including loop recurrences. Once classified backedges
/// are removed from the dependency graph, every node must be established from
/// ordinary edges ending at literals. This rejects unseeded or irreducible
/// cycles and never infers a loop's trip count.
fn verify_i64_source(geometry: &FunctionVerificationGeometry<'_>, source: ValueId) -> bool {
    let mut pending = vec![source];
    let mut needs = HashMap::<ValueId, usize>::new();
    let mut users = HashMap::<ValueId, Vec<ValueId>>::new();
    let mut established = Vec::new();

    while let Some(value) = pending.pop() {
        if needs.contains_key(&value) {
            continue;
        }
        let Some((block, index, row)) = geometry.definition(value) else {
            return false;
        };
        if geometry.definition_count(value) != 1 || !geometry.reachable().contains(&block) {
            return false;
        }
        let ordinary = match row {
            // Function verification already checks the full actual predecessor
            // set and dominance/order of each edge's source.
            MirInstruction::Phi { inputs, .. } => {
                let mut entries = Vec::new();
                for &(pred, input) in inputs {
                    if !geometry.reachable().contains(&pred) {
                        return false;
                    }
                    if geometry.dominators().dominates(block, pred) {
                        if !is_add_recurrence(geometry, input) {
                            return false;
                        }
                        pending.push(input);
                    } else {
                        entries.push(input);
                    }
                }
                if entries.is_empty() {
                    return false;
                }
                entries
            }
            _ => match scalar_producer(geometry, block, index, row) {
                Some(I64Producer::Literal) => Vec::new(),
                Some(I64Producer::Forward(input)) => vec![input],
                Some(I64Producer::Add(lhs, rhs)) => vec![lhs, rhs],
                None => return false,
            },
        };
        if ordinary.is_empty() {
            established.push(value);
        }
        needs.insert(value, ordinary.len());
        for input in ordinary {
            users.entry(input).or_default().push(value);
            pending.push(input);
        }
    }

    let mut proven = 0;
    while let Some(value) = established.pop() {
        proven += 1;
        for user in users.get(&value).into_iter().flatten() {
            let remaining = needs
                .get_mut(user)
                .expect("source graph contains its users");
            *remaining -= 1;
            if *remaining == 0 {
                established.push(*user);
            }
        }
    }
    proven == needs.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::LocalContractWriteKind;
    use crate::mir::type_contracts::local_slot::register_local_slot_contract;
    use crate::mir::{BasicBlock, BindingId, EffectMask, FunctionSignature, LocalSlotId, MirType};

    fn diamond(right: ConstValue) -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".into(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        );
        let slot = LocalSlotId::from(BindingId::new(0));
        register_local_slot_contract(&mut function, slot, "x", "i64").unwrap();
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId(0),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId(0),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        function.add_block(entry);
        for (id, value) in [(1, ConstValue::Integer(1)), (2, right)] {
            let mut block = BasicBlock::new(BasicBlockId(id));
            block.add_instruction(MirInstruction::Const {
                dst: ValueId(id),
                value,
            });
            block.set_terminator(MirInstruction::Jump {
                target: BasicBlockId(3),
                edge_args: None,
            });
            function.add_block(block);
        }
        let mut join = BasicBlock::new(BasicBlockId(3));
        join.add_instruction(MirInstruction::Phi {
            dst: ValueId(3),
            inputs: vec![(BasicBlockId(1), ValueId(1)), (BasicBlockId(2), ValueId(2))],
            type_hint: Some(MirType::Integer),
        });
        join.add_instruction(MirInstruction::LocalContractWrite {
            dst: ValueId(4),
            src: ValueId(3),
            local_slot_id: slot,
            write_kind: LocalContractWriteKind::Init,
        });
        join.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });
        function.add_block(join);
        function.update_cfg();
        for id in 1..=4 {
            function
                .metadata
                .value_types
                .insert(ValueId(id), MirType::Integer);
        }
        let mut module = MirModule::new("local-write".into());
        module.add_function(function);
        module
    }

    #[test]
    fn rejects_bool_phi_input_even_with_integer_metadata() {
        let module = diamond(ConstValue::Bool(false));
        let function = &module.functions["main"];
        let block = BasicBlockId(3);
        let geometry = FunctionVerificationGeometry::new(function);
        assert!(FinishedI64OriginsV1::new(&geometry, Some(&module))
            .check_function()
            .is_ok());
        let proof = VerifiedLocalSlotWrites::verify(&module);
        assert_eq!(
            proof.copy_operands(function, block, 1, &function.blocks[&block].instructions[1]),
            None
        );
    }

    #[test]
    fn proof_requires_the_exact_final_coordinate_and_row() {
        let module = diamond(ConstValue::Integer(2));
        let function = &module.functions["main"];
        let block = BasicBlockId(3);
        let row = &function.blocks[&block].instructions[1];
        let proof = VerifiedLocalSlotWrites::verify(&module);
        assert_eq!(
            proof.copy_operands(function, block, 1, row),
            Some((ValueId(4), ValueId(3)))
        );
        assert_eq!(proof.copy_operands(function, block, 0, row), None);
        assert_eq!(proof.copy_operands(function, block, 1, &row.clone()), None);
        let detached = function.clone();
        assert_eq!(
            proof.copy_operands(
                &detached,
                block,
                1,
                &detached.blocks[&block].instructions[1]
            ),
            None
        );
    }
}
