//! The independent literal validator reads each original owner's scalar book.
use super::*;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirType};

fn function(value: ValueId, literal: ConstValue, returned: ValueId) -> MirFunction {
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "literal_storage_validator".into(),
            params: vec![],
            return_type: if matches!(literal, ConstValue::Bool(_)) { MirType::Bool } else { MirType::Integer },
            effects: EffectMask::CONTROL,
        },
        entry,
    );
    let block = function.blocks.get_mut(&entry).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: value,
        value: literal,
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(returned),
    });
    function
}

#[test]
fn indexed_root_literal_validation_keeps_missing_and_physical_drift_refusals() {
    for is_bool in [false, true] {
    let root_body = if is_bool { "true" } else { "7" };
    let child_body = if is_bool { "false" } else { "30" };
    let annotation = if is_bool { "" } else { ": i64" };
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &format!("box Page {{}} static box Main {{ main() {{ local page = new Page() local m = %{{\"v\" => 0}} return {root_body} }} helper(value: i64){annotation} {{ local page = new Page() local m = %{{\"v\" => value}} return {child_body} }} unused() {{ return }} }}")
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let root = ledger.root_owner().unwrap();
    let child = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .find(|owner| {
            *owner != root
                && ledger.terminal_relations_for_owner(*owner).iter().any(|row| matches!(row, TerminalRelationV1::IntegerLiteral(_) | TerminalRelationV1::BoolLiteral(_)))
        })
        .unwrap();
    let site = ledger.terminal_relations_for_owner(root).into_iter().next().unwrap().return_site()
        .clone();
    assert_eq!(
        &site,
        ledger.terminal_relations_for_owner(child).into_iter().next().unwrap().return_site()
    );
    assert!(ledger.terminal_relation_is_indexed(root, &site));
    for (owner, value) in [(root, ValueId(900)), (child, ValueId(901))] {
        ledger
            .record_terminal_scalar_literal_return(owner, site.node(), value)
            .unwrap();
        let literal = if owner == root { if is_bool { ConstValue::Bool(true) } else { ConstValue::Integer(7) } }
            else if is_bool { ConstValue::Bool(false) } else { ConstValue::Integer(30) };
        let physical = function(value, literal.clone(), value);
        let block = &physical.blocks[&BasicBlockId(0)];
        assert!(ledger.complete_terminal_literal_return_v1(owner, &site,
            &[(block.id, block.terminator.as_ref().unwrap().clone())]).unwrap_err()
            .contains("literal-return-drift"));
        let foreign = package.batch().declarations().map(|row| row.owner()).find(|candidate|
            ledger.terminal_relations_for_owner(*candidate).iter().all(|relation|
                !matches!(relation, TerminalRelationV1::IntegerLiteral(_) | TerminalRelationV1::BoolLiteral(_))))
            .expect("original non-literal owner");
        assert!(ledger.complete_terminal_literal_return_v1(foreign, &site,
            &[(block.id, block.terminator.as_ref().unwrap().clone())]).unwrap_err()
            .contains("literal-source-drift"));
        ledger.record_terminal_literal_producer_v1(owner, &site,
            (block.id, block.instructions[0].clone()), &physical).unwrap();
        ledger.complete_terminal_literal_return_v1(owner, &site,
            &[(block.id, block.terminator.as_ref().unwrap().clone())]).unwrap();
    }
    for (owner, value, literal) in [(root, ValueId(900), if is_bool { ConstValue::Bool(true) } else { ConstValue::Integer(7) }), (child, ValueId(901), if is_bool { ConstValue::Bool(false) } else { ConstValue::Integer(30) })] {
        let original = function(value, literal.clone(), value);
        ledger
            .validate_terminal_scalar_literal_return(owner, &original)
            .unwrap();
        if owner == root {
            ledger
                .terminal_scalar_literal_value
                .borrow_mut()
                .remove(&site);
        } else {
            ledger
                .terminal_scalar_literal_values
                .borrow_mut()
                .remove(&(owner, site.clone()));
        }
        assert_eq!(
            ledger
                .validate_terminal_scalar_literal_return(owner, &original)
                .unwrap_err(),
            "[freeze:contract][ordinary-new/local-commit/literal-unconsumed]"
        );
        let (other, other_value, other_literal) = if owner == root {
            (child, ValueId(901), if is_bool { ConstValue::Bool(false) } else { ConstValue::Integer(30) })
        } else {
            (root, ValueId(900), if is_bool { ConstValue::Bool(true) } else { ConstValue::Integer(7) })
        };
        ledger
            .validate_terminal_scalar_literal_return(
                other,
                &function(other_value, other_literal, other_value),
            )
            .unwrap();
        ledger
            .record_terminal_scalar_literal_return(owner, site.node(), value)
            .unwrap();
        let literal = if owner == root { if is_bool { ConstValue::Bool(true) } else { ConstValue::Integer(7) } }
            else if is_bool { ConstValue::Bool(false) } else { ConstValue::Integer(30) };
        let physical = function(value, literal.clone(), value);
        let block = &physical.blocks[&BasicBlockId(0)];
        ledger.record_terminal_literal_producer_v1(owner, &site,
            (block.id, block.instructions[0].clone()), &physical).unwrap();
        ledger.complete_terminal_literal_return_v1(owner, &site,
            &[(block.id, block.terminator.as_ref().unwrap().clone())]).unwrap();
        for changed in [
            function(value, match literal { ConstValue::Bool(value) => ConstValue::Bool(!value), ConstValue::Integer(value) => ConstValue::Integer(value + 1), _ => unreachable!() }, value),
            function(value, literal.clone(), ValueId(999)),
        ] {
            assert_eq!(
                ledger
                    .validate_terminal_scalar_literal_return(owner, &changed)
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/local-commit/literal-physical-drift]"
            );
        }
        for duplicate in [
            MirInstruction::Const { dst: value, value: literal.clone() },
            MirInstruction::Return { value: Some(value) },
        ] {
            let mut changed = original.clone();
            if matches!(duplicate, MirInstruction::Return { .. }) {
                let mut sibling = crate::mir::BasicBlock::new(BasicBlockId(1));
                sibling.set_terminator(duplicate);
                changed.add_block(sibling);
            } else {
                changed.blocks.get_mut(&BasicBlockId(0)).unwrap()
                    .add_instruction_before_terminator(duplicate);
            }
            assert_eq!(ledger.validate_terminal_scalar_literal_return(owner, &changed).unwrap_err(),
                "[freeze:contract][ordinary-new/local-commit/literal-physical-drift]");
        }
        let mut relocated = original.clone();
        let mut block = relocated.blocks.remove(&BasicBlockId(0)).unwrap();
        block.id = BasicBlockId(7);
        relocated.entry_block = block.id;
        relocated.add_block(block);
        assert!(ledger.validate_terminal_scalar_literal_return(owner, &relocated).unwrap_err()
            .contains("literal-physical-drift"));
        ledger
            .validate_terminal_scalar_literal_return(owner, &original)
            .unwrap();
    }
}
}
