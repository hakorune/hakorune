//! Original scalar source loans and independent finished-MIR mutations.
use super::*;
use crate::mir::compiler::MirCompiler;

use super::selected_loop_actual_tests::selected_request;

#[test]
fn original_scalar_append_observer_retains_three_source_operations() {
    use crate::mir::resolved_semantics::OwnedExprSiteV1;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(
                selected_request(),
                |view, verification| -> Result<(), String> {
                    assert!(verification.is_ok(), "final module verification");
                    let source = view
                        .retained_root_source()
                        .expect("original source retained");
                    let mut count = 0;
                    let mut originals = Vec::new();
                    let mut owners = std::collections::BTreeSet::new();
                    source.visit_scalar_operation_originals_v1(|row| {
                        let proof = row.source();
                        let site =
                            OwnedExprSiteV1::new(proof.owner(), proof.binary().site().clone());
                        let MirInstruction::BinOp { op, .. } = &row.original().1 else {
                            panic!("original Binary append");
                        };
                        let original = source
                            .ledger
                            .prepare_scalar_operation_source_v1(proof.owner(), &site, Some(*op))?
                            .expect("same original source membership");
                        assert!(std::rc::Rc::ptr_eq(proof, &original));
                        if proof.positive_divisor().is_some() {
                            assert!(row.divisor().is_some());
                        }
                        assert_ne!(row.children().0, row.children().1);
                        let _actual_ordered_copies = row.operand_relations();
                        owners.insert(proof.owner());
                        originals.push(std::rc::Rc::clone(row));
                        count += 1;
                        Ok(())
                    })?;
                    assert_eq!(
                        count, 3,
                        "one original append per Sub/Div source, not derived coverage"
                    );
                    let mut derived_divisions = 0;
                    for owner in owners {
                        source
                            .ledger
                            .with_finished_projection(owner, |symbol, projection| {
                                let function = view
                                    .module()
                                    .functions
                                    .get(symbol)
                                    .expect("same finished owner");
                                source.ledger.verify_finished_scalar_operations_v1(
                                    owner,
                                    function,
                                    |binding| projection.relation_binding(function, binding),
                                )?;
                                let mut derived_probe = None;
                                source.ledger.visit_scalar_operation_consumers_v1(
                                    owner,
                                    |original, relation| {
                                        assert!(originals
                                            .iter()
                                            .any(|row| std::rc::Rc::ptr_eq(row, original)));
                                        relation.verify_projected(function, &mut |binding| {
                                            projection.relation_binding(function, binding)
                                        })?;
                                        if matches!(
                                            original.original().1,
                                            MirInstruction::BinOp {
                                                op: crate::mir::BinaryOp::Div,
                                                ..
                                            }
                                        ) {
                                            derived_divisions += 1;
                                            let value = relation.values().1;
                                            let actual = function
                                                .blocks
                                                .values()
                                                .flat_map(|block| block.all_instructions())
                                                .find(|row| row.dst_value() == Some(value))
                                                .expect("derived producer");
                                            let MirInstruction::BinOp {
                                                op: crate::mir::BinaryOp::Div,
                                                rhs,
                                                ..
                                            } = actual
                                            else {
                                                panic!("derived Div actual writer");
                                            };
                                            derived_probe.get_or_insert((value, *rhs));
                                        }
                                        Ok(())
                                    },
                                )?;
                                if let Some((value, divisor)) = derived_probe {
                                    for mutation in ["missing", "ordered", "zero-divisor"] {
                                        let mut changed = function.clone();
                                        let mut edits = 0;
                                        for block in changed.blocks.values_mut() {
                                            if mutation == "missing" {
                                                let before = block.instructions.len();
                                                block
                                                    .instructions
                                                    .retain(|row| row.dst_value() != Some(value));
                                                edits += before - block.instructions.len();
                                            } else {
                                                for row in &mut block.instructions {
                                                    match row {
                                                        MirInstruction::BinOp {
                                                            dst,
                                                            lhs,
                                                            rhs,
                                                            ..
                                                        } if mutation == "ordered"
                                                            && *dst == value =>
                                                        {
                                                            std::mem::swap(lhs, rhs);
                                                            edits += 1;
                                                        }
                                                        MirInstruction::Const {
                                                            dst,
                                                            value: constant,
                                                        } if mutation == "zero-divisor"
                                                            && *dst == divisor =>
                                                        {
                                                            assert_eq!(
                                                                *constant,
                                                                crate::mir::ConstValue::Integer(4)
                                                            );
                                                            *constant =
                                                                crate::mir::ConstValue::Integer(0);
                                                            edits += 1;
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                        assert_eq!(
                                            edits, 1,
                                            "one actual final producer mutation: {mutation}"
                                        );
                                        let error = source
                                            .ledger
                                            .verify_finished_scalar_operations_v1(
                                                owner,
                                                &changed,
                                                |binding| {
                                                    projection.relation_binding(&changed, binding)
                                                },
                                            )
                                            .unwrap_err();
                                        assert!(
                                            error.contains(
                                                "physical-boundary/relation-finished-producer"
                                            ),
                                            "{mutation}: {error}"
                                        );
                                    }
                                }
                                Ok(())
                            })?;
                    }
                    assert!(
                        derived_divisions > 0,
                        "real source has repeated Div materialization"
                    );
                    Ok(())
                },
            )
            .unwrap();
    });
}
