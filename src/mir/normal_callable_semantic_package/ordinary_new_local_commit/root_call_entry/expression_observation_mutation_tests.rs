//! Mutation evidence shared verbatim with the pre-replacement baseline.
use super::*;

pub(in super::super::super) fn check_observation_mutations(
    purpose: &str,
    row: ExpressionPhysicalProjectionRefV1<'_>,
    context: CallValidationContextV1<'_>,
    root: &FinalizedRootSourceHandoffV1,
    module: &crate::mir::MirModule,
) -> bool {
    if row.calls.len() < 2 {
        return false;
    }
    row.validate_source(context).unwrap();
    row.validate_recorded(row.original, None).unwrap();
    let refusal = |name: &str, result: Result<(), String>| {
        let error = result.unwrap_err();
        eprintln!("EXPR_MUTATION\t{purpose}\t{name}\t{error}");
    };
    let view = |calls| {
        ExpressionPhysicalProjectionRefV1::new(
            row.frame,
            row.values,
            calls,
            row.path_reads,
            row.reference_reads,
            row.original,
        )
        .with_inline_range_v1(row.inline_range)
        .with_root_fault_v1(row.root_fault)
    };
    let mut calls = row.calls.to_vec();
    calls.swap(0, 1);
    refusal("call-ordinal", view(&calls).validate_source(context));
    let mut calls = row.calls.to_vec();
    assert_ne!(calls[0].call_site(), calls[1].call_site());
    calls[0] = Rc::clone(&calls[1]);
    refusal(
        "equal-value-foreign-source",
        view(&calls).validate_source(context),
    );
    refusal("incomplete-observations", row.validate_recorded(&[], None));
    for duplicate in [false, true] {
        let mut changed = row.original.to_vec();
        let at = changed
            .iter()
            .position(|(_, i)| {
                matches!(
                    i,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::String(Operation::End { .. }),
                        ..
                    }
                )
            })
            .expect("expression owns a String End");
        if duplicate {
            changed.insert(at, changed[at].clone());
        } else {
            changed.remove(at);
        }
        refusal(
            if duplicate {
                "retained-end-duplicate"
            } else {
                "retained-end-missing"
            },
            row.validate_recorded(&changed, None),
        );
    }
    // Select effects from this expression, then use the captured finishing map.
    // An unrelated constructor producer must not stand in for expression evidence.
    for (kind, original) in [
        (
            "end",
            row.original
                .iter()
                .find(|(_, i)| {
                    matches!(
                        i,
                        MirInstruction::Invoke {
                            operation: InvokeOperation::String(Operation::End { .. }),
                            ..
                        }
                    )
                })
                .unwrap(),
        ),
        (
            "producer",
            row.original
                .iter()
                .find(|(_, i)| matches!(i, MirInstruction::InvokeNormalResult { .. }))
                .unwrap(),
        ),
    ] {
        let (symbol, (id, instruction)) = context
            .ledger()
            .finished_binding_for_owner(row.frame.source().owner(), original)
            .unwrap();
        for duplicate in [false, true] {
            if kind == "producer" && duplicate {
                continue;
            }
            let mut changed = module.clone();
            let block = changed
                .get_function_mut(&symbol)
                .unwrap()
                .blocks
                .get_mut(&id)
                .unwrap();
            assert_eq!(
                block
                    .all_instructions()
                    .filter(|i| **i == instruction)
                    .count(),
                1
            );
            if duplicate {
                block.instructions.push(instruction.clone());
            } else if block.terminator.as_ref() == Some(&instruction) {
                block.terminator = None;
            } else {
                let at = block
                    .instructions
                    .iter()
                    .position(|i| *i == instruction)
                    .unwrap();
                block.instructions.remove(at);
            }
            refusal(
                &format!(
                    "physical-{kind}-{}",
                    if duplicate { "duplicate" } else { "missing" }
                ),
                root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())),
            );
        }
    }
    true
}

#[test]
fn scalar_core_return_keeps_original_live_text_operand_order() {
    // Both operands remain live Text after exchange. Only the original source
    // correspondence, not a domain/lifetime check, distinguishes their roles.
    let text = r#"
static box Main {
    main() {
        local s = "hello"
        return s.indexOf("l")
    }
}
"#;
    let (completed, ledger) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, _, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
    let handoff = ledger
        .seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new())
        .unwrap();
    let root = handoff.root_source().unwrap();
    let context = CallValidationContextV1::Finalized(root);
    let mut observations = 0;
    for progress in ledger.root_exits.borrow().values() {
        let order = match progress {
            RootHomeExitProgress::Emitted { order, .. }
            | RootHomeExitProgress::Finalized(order) => order,
            _ => continue,
        };
        let Some(packet) = &order.return_expression else {
            continue;
        };
        let row = packet.expression();
        row.validate_source(context).unwrap();
        let mut changed = row.original.to_vec();
        let mut exchanged = 0;
        for (_, instruction) in &mut changed {
            if let MirInstruction::StringIndexOf { value, needle, .. } = instruction {
                assert_ne!(value, needle);
                std::mem::swap(value, needle);
                exchanged += 1;
            }
        }
        if exchanged == 0 {
            continue;
        }
        assert_eq!(exchanged, 1);
        let altered = ExpressionPhysicalProjectionRefV1 {
            original: &changed,
            ..row
        };
        assert!(altered.validate_source(context).is_err());
        packet.expression().validate_source(context).unwrap();
        observations += 1;
    }
    assert_eq!(observations, 1);
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
}
