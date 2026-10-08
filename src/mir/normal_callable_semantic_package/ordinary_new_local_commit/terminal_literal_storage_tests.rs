//! The independent literal validator reads each original owner's scalar book.
use super::*;
use crate::mir::{BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirType};

fn function(value: ValueId, literal: i64, returned: ValueId) -> MirFunction {
    let entry = BasicBlockId(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "literal_storage_validator".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        entry,
    );
    let block = function.blocks.get_mut(&entry).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: value,
        value: ConstValue::Integer(literal),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(returned),
    });
    function
}

#[test]
fn indexed_root_literal_validation_keeps_missing_and_physical_drift_refusals() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Page {} static box Main {
           main() { local page = new Page() local m = %{\"v\" => 0} return 7 }
           helper(value: i64): i64 { local page = new Page() local m = %{\"v\" => value} return 30 }
         }"
    ).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let root = ledger.root_owner().unwrap();
    let child = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .find(|owner| {
            *owner != root
                && ledger
                    .terminal_integer_literal_return_for_owner(*owner)
                    .is_some()
        })
        .unwrap();
    let site = ledger
        .terminal_integer_literal_return_for_owner(root)
        .unwrap()
        .return_site()
        .clone();
    assert_eq!(
        &site,
        ledger
            .terminal_integer_literal_return_for_owner(child)
            .unwrap()
            .return_site()
    );
    assert!(ledger.terminal_relation_is_indexed(root, &site));
    for (owner, value) in [(root, ValueId(900)), (child, ValueId(901))] {
        ledger
            .record_terminal_integer_literal_return(owner, site.node(), value)
            .unwrap();
    }
    for (owner, value, literal) in [(root, ValueId(900), 7), (child, ValueId(901), 30)] {
        let original = function(value, literal, value);
        ledger
            .validate_terminal_integer_literal_return(owner, &original)
            .unwrap();
        if owner == root {
            ledger
                .terminal_integer_literal_value
                .borrow_mut()
                .remove(&site);
        } else {
            ledger
                .terminal_integer_literal_values
                .borrow_mut()
                .remove(&(owner, site.clone()));
        }
        assert_eq!(
            ledger
                .validate_terminal_integer_literal_return(owner, &original)
                .unwrap_err(),
            "[freeze:contract][ordinary-new/local-commit/literal-unconsumed]"
        );
        let (other, other_value, other_literal) = if owner == root {
            (child, ValueId(901), 30)
        } else {
            (root, ValueId(900), 7)
        };
        ledger
            .validate_terminal_integer_literal_return(
                other,
                &function(other_value, other_literal, other_value),
            )
            .unwrap();
        ledger
            .record_terminal_integer_literal_return(owner, site.node(), value)
            .unwrap();
        for changed in [
            function(value, literal + 1, value),
            function(value, literal, ValueId(999)),
        ] {
            assert_eq!(
                ledger
                    .validate_terminal_integer_literal_return(owner, &changed)
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/local-commit/literal-physical-drift]"
            );
        }
        ledger
            .validate_terminal_integer_literal_return(owner, &original)
            .unwrap();
    }
}
