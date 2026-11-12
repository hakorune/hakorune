//! Rebind observation, replacement and receipt contract evidence.
use super::*;
#[test]
fn rebind_record_rejects_shared_and_replacement_mutations() {
    use super::super::expression_projection::mutation_evidence::check_observation_mutations;
    let text = "static box Values { read(a: i64): i64 { return copy a } }
        static box Main { main() { local text = \"old\" + \"value\"
            text = (\"n=\" + Values.read(7)) + Values.read(7)
            print(text) return 0 } }";
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
    let mut count = 0;
    for group in root.local_calls.values().flatten() {
        let Some(packet) = group.rebind() else {
            continue;
        };
        assert!(check_observation_mutations(
            "Rebind",
            packet.expression(),
            context,
            root,
            &module
        ));
        let old = ledger
            .text_rebind_displaced_end_v1(packet.frame())
            .unwrap()
            .unwrap();
        let (symbol, (install_id, install)) = ledger
            .finished_binding_for_owner(
                packet.frame().source().owner(),
                packet.contract.install.as_ref().unwrap(),
            )
            .unwrap();
        let function = module.get_function(&symbol).unwrap();
        assert!(matches!(
            install,
            MirInstruction::StringResidenceInstall { .. }
        ));
        assert_eq!(
            function.blocks[&install_id]
                .instructions
                .iter()
                .filter(|row| **row == install)
                .count(),
            1
        );
        // Recording order also contains RHS Fault cleanup. Select execution
        // order instead: both Ends must follow this install on its Normal path.
        let mut current = install_id;
        let mut visited = std::collections::BTreeSet::new();
        let mut ends = Vec::new();
        while ends.len() < 2 {
            assert!(visited.insert(current), "acyclic replacement cleanup");
            match function.blocks[&current].terminator.as_ref().unwrap() {
                MirInstruction::Jump { target, .. } => current = *target,
                instruction @ MirInstruction::Invoke {
                    operation,
                    normal_landing,
                    ..
                } => {
                    assert!(
                        matches!(
                            operation,
                            InvokeOperation::String(
                                crate::mir::instruction::StringInvokeOperation::End { .. }
                                    | crate::mir::instruction::StringInvokeOperation::EndIfOwned { .. }
                            )
                        ),
                        "only End after install"
                    );
                    ends.push((current, instruction.clone()));
                    current = *normal_landing;
                }
                _ => panic!("two sequential Ends after install"),
            }
        }
        let (id, instruction) = &ends[0];
        let MirInstruction::Invoke { operation, .. } = instruction else {
            unreachable!()
        };
        assert_eq!(operation, &old, "displaced old value ends first");
        let id = *id;
        let instruction = instruction.clone();
        for duplicate in [false, true] {
            let mut changed = module.clone();
            let block = changed
                .get_function_mut(&symbol)
                .unwrap()
                .blocks
                .get_mut(&id)
                .unwrap();
            assert_eq!(block.terminator.as_ref(), Some(&instruction));
            if duplicate {
                block.instructions.push(instruction.clone());
            } else {
                block.terminator = None;
            }
            let error = root
                .visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
                .unwrap_err();
            eprintln!(
                "EXPR_MUTATION\tRebind\told-end-{}\t{error}",
                if duplicate { "duplicate" } else { "missing" }
            );
        }
        let mut changed = module.clone();
        let function = changed.get_function_mut(&symbol).unwrap();
        let block = function.blocks.get_mut(&install_id).unwrap();
        let position = block
            .instructions
            .iter()
            .position(|row| row == &install)
            .unwrap();
        block.instructions.remove(position);
        let MirInstruction::Invoke {
            normal_landing,
            fault_landing,
            ..
        } = &instruction
        else {
            unreachable!()
        };
        assert_ne!(*normal_landing, install_id);
        assert_ne!(normal_landing, fault_landing);
        function
            .blocks
            .get_mut(normal_landing)
            .unwrap()
            .instructions
            .push(install.clone());
        assert_eq!(
            function
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .filter(|row| **row == install)
                .count(),
            1,
            "move one exact install without duplication"
        );
        let error = root
            .visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
            .unwrap_err();
        eprintln!("EXPR_MUTATION\tRebind\tinstall-after-old-end\t{error}");
        let (other_id, other_instruction) = ends[1].clone();
        let MirInstruction::Invoke {
            operation: other_operation,
            ..
        } = other_instruction
        else {
            panic!("temporary End")
        };
        assert_ne!(
            other_operation, old,
            "the second End owns a different value"
        );
        let mut changed = module.clone();
        let function = changed.get_function_mut(&symbol).unwrap();
        for (block_id, replacement) in [(id, other_operation), (other_id, old)] {
            let MirInstruction::Invoke { operation, .. } = function
                .blocks
                .get_mut(&block_id)
                .unwrap()
                .terminator
                .as_mut()
                .unwrap()
            else {
                panic!("End terminator")
            };
            *operation = replacement;
        }
        let error = root
            .visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
            .unwrap_err();
        eprintln!("EXPR_MUTATION\tRebind\treplacement-end-order\t{error}");
        let absent = RootLifecyclePrefixBindingGroupV1::new(
            group.site().clone(),
            group.bindings().to_vec(),
            None,
        )
        .unwrap();
        let expected = LifecyclePrefixSourceV1::BindingRebind {
            site: group.site().clone(),
            frame: Rc::clone(packet.frame()),
        };
        assert!(group.matches_source(&expected));
        assert!(!absent.matches_source(&expected));
        eprintln!("EXPR_MUTATION\tRebind\trecord-absent\tsource-membership-rejected");
        let error = group
            .with_bindings(vec![])
            .validate_recorded_in_context(context, module.get_function(&symbol).unwrap(), None)
            .unwrap_err();
        eprintln!("EXPR_MUTATION\tRebind\tincomplete-retention\t{error}");
        count += 1;
    }
    assert_eq!(count, 1, "selected root Rebind, not an unrelated child");
}

#[test]
fn scalar_rebind_keeps_assignment_and_alias_provenance_through_both_final_consumers() {
    for rhs in ["x", "x + 1", "7", "(\"a\" + \"b\").length()"] {
        let text = format!(
            "static box Main {{ main() {{ local x = 0 local before = x
        x = {rhs} local alias = x x = x local after = x local y = 0 y = x
        local text = \"\" + before + alias + after print(text) return 0 }} }}"
        );
        let (completed, ledger) =
            crate::mir::builder::lexical_call_projection_document_completion_fixture(&text);
        let (key, module, same, keys, cohort) =
            completed.document_preflight_parts_for_test(true).unwrap();
        assert!(Rc::ptr_eq(&same, &ledger));
        let mut handoff = ledger
            .seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort, Vec::new())
            .unwrap();
        let root = handoff.root_source().unwrap();
        root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
            .unwrap();
        let packets: Vec<_> = root
            .local_calls
            .get(&root.owner())
            .unwrap()
            .iter()
            .filter_map(|group| group.rebind())
            .filter(|packet| packet.contract.receipt.target() == "x")
            .collect();
        assert_eq!(
            packets.len(),
            2,
            "both original rebinds remain mandatory: {rhs}"
        );
        assert_ne!(
            packets[0].frame().source().root().statement(),
            packets[1].frame().source().root().statement()
        );
        assert!(!packets[0]
            .contract
            .receipt
            .matches_rebind_frame_v1(packets[1].frame()));
        assert!(!packets[1]
            .contract
            .receipt
            .matches_rebind_frame_v1(packets[0].frame()));
        assert_eq!(
            packets[0].value(),
            packets[1].value(),
            "self-assignment reuses RHS, never a fresh definition"
        );
        assert!(
            !module
                .functions
                .get(&key)
                .unwrap()
                .blocks
                .values()
                .flat_map(|block| block.all_instructions())
                .any(|row| matches!(row, MirInstruction::ReleaseStrong { .. })),
            "a scalar rebind owns no Home to release"
        );
        let mut changed = module.clone();
        let mut count = 0;
        for block in changed.functions.get_mut(&key).unwrap().blocks.values_mut() {
            for row in &mut block.instructions {
                if let MirInstruction::Const {
                    value: crate::mir::ConstValue::Integer(value),
                    ..
                } = row
                {
                    *value += 1;
                    count += 1;
                }
            }
        }
        assert!(count > 0);
        assert!(
            root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
                .is_err(),
            "changed producer: {rhs}"
        );
        let original_frame = Rc::clone(packets[0].frame());
        let foreign_packet = root.local_calls[&root.owner()]
            .iter()
            .filter_map(|group| group.rebind())
            .find(|packet| packet.contract.receipt.target() == "y")
            .unwrap();
        assert_ne!(
            packets[0].contract.receipt.target(),
            foreign_packet.contract.receipt.target()
        );
        assert_ne!(packets[0].frame().target(), foreign_packet.frame().target());
        assert_eq!(
            packets[0].value(),
            foreign_packet.value(),
            "only the destination differs"
        );
        let foreign_frame = Rc::clone(foreign_packet.frame());
        let original_site = root.local_calls[&root.owner()]
            .iter()
            .find(|g| {
                g.rebind()
                    .is_some_and(|p| Rc::ptr_eq(p.frame(), &original_frame))
            })
            .unwrap()
            .site()
            .clone();
        let source = match &mut handoff {
            crate::mir::finalized_root_handoff::FinalizedRootHandoffV1::Births {
                root_source: Some(source),
                ..
            }
            | crate::mir::finalized_root_handoff::FinalizedRootHandoffV1::NoBirth {
                root_source: Some(source),
                ..
            } => source,
            _ => panic!("retained original source"),
        };
        super::super::local_binding_group::expression_emission_tests::mutate_rebind_observation(
            source,
            &original_site,
            |packet| packet.contract.frame = foreign_frame,
        );
        let packet = source.local_calls[&source.owner]
            .iter()
            .find(|g| g.site() == &original_site)
            .unwrap()
            .rebind()
            .unwrap();
        ledger
            .with_finished_projection(source.owner, |symbol, projection| {
                let error = packet
                    .validate_paths(
                        CallValidationContextV1::Finalized(source),
                        module.get_function(symbol).unwrap(),
                        Some(projection),
                    )
                    .unwrap_err();
                assert!(error.contains("rebind-expression/target-name"));
                eprintln!("EXPR_MUTATION\tRebind\twrong-receipt-target-{rhs}\t{error}");
                Ok(())
            })
            .unwrap();
        super::super::local_binding_group::expression_emission_tests::mutate_rebind_observation(
            source,
            &original_site,
            |packet| packet.contract.frame = original_frame,
        );
        source
            .visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
            .unwrap();
    }
}
