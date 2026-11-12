use super::super::expression_projection::mutation_evidence::check_observation_mutations;
use super::*;
#[test]
fn shared_expression_records_reject_print_and_return_mutations() {
    let text = "box Counter { n: i64 birth() { me.n = 7 }
        read() { return me.n }
        }
        static box Main { main() { local c = new Counter()
            print((\"n=\" + c.read()) + c.read()) return (\"n=\" + c.read()).length() + c.read() } }";
    let (completed, ledger) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, same, keys, cohort) =
        completed.document_preflight_parts_for_test(true).unwrap();
    assert!(Rc::ptr_eq(&same, &ledger));
    let handoff = ledger
        .seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new())
        .unwrap();
    let root = handoff.root_source().unwrap();
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
    let context = CallValidationContextV1::Finalized(root);
    let mut counts = [0, 0];
    let mut return_key = None;
    for groups in root.local_calls.values() {
        for group in groups {
            if let PrefixPhysicalProofV1::PrintExpression(row) = &group.proof {
                if check_observation_mutations("Print", row.expression(), context, root, &module) {
                    counts[0] += 1;
                }
            }
        }
    }
    for (key, progress) in ledger.root_exits.borrow().iter() {
        let order = match progress {
            RootHomeExitProgress::Emitted { order, .. }
            | RootHomeExitProgress::Finalized(order) => order,
            _ => continue,
        };
        if let Some(row) = &order.return_expression {
            if check_observation_mutations("Return", row.expression(), context, root, &module) {
                counts[1] += 1;
                return_key = Some(key.clone());
            }
        }
    }
    assert_eq!(
        counts,
        [1, 1],
        "both purpose contracts must reach the shared record"
    );
    // Correct MIR with incomplete purpose retention must still be refused.
    let mut print_count = 0;
    for groups in root.local_calls.values() {
        for group in groups {
            if let PrefixPhysicalProofV1::PrintExpression(row) = &group.proof {
                if row.calls().count() < 2 {
                    continue;
                }
                let partial = group.with_bindings(vec![]);
                let owner = row.frame().source().owner();
                let (symbol, _) = ledger
                    .finished_binding_for_owner(owner, &group.bindings()[0])
                    .unwrap();
                let error = partial
                    .validate_recorded_in_context(
                        context,
                        module.get_function(&symbol).unwrap(),
                        None,
                    )
                    .unwrap_err();
                eprintln!("EXPR_MUTATION\tPrint\tincomplete-retention\t{error}");
                print_count += 1;
            }
        }
    }
    assert_eq!(print_count, 1);
    let key = return_key.unwrap();
    let saved = {
        let mut exits = ledger.root_exits.borrow_mut();
        let order = match exits.get_mut(&key).unwrap() {
            RootHomeExitProgress::Emitted { order, .. }
            | RootHomeExitProgress::Finalized(order) => order,
            _ => unreachable!(),
        };
        order.return_expression.take().unwrap()
    };
    let result = root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()));
    {
        let mut exits = ledger.root_exits.borrow_mut();
        let order = match exits.get_mut(&key).unwrap() {
            RootHomeExitProgress::Emitted { order, .. }
            | RootHomeExitProgress::Finalized(order) => order,
            _ => unreachable!(),
        };
        order.return_expression = Some(saved);
    }
    let error = result.unwrap_err();
    eprintln!("EXPR_MUTATION\tReturn\tincomplete-retention\t{error}");
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
}
