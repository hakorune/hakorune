//! Exercise the full child finishing validator against the original source ledger.
use super::*;
use crate::mir::compiler::{MirCompiler, NormalCompileRequestV1};
use crate::mir::{BasicBlock, ConstValue};

#[test]
fn stored_child_finishing_rejects_missing_duplicate_and_drifted_receiver_reads() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Item { value: i64 birth() { me.value = 5 } }
            box Leaf { flag: i64 birth() { me.flag = 0 }
                read(p: Item) { if p == null { return 7 } return p.value } }
            box Parent { left: Leaf right: Leaf
                birth() { me.left = new Leaf() me.right = new Leaf() }
                first(p: Item) { return me.left.read(p) }
                second(p: Item) { return me.right.read(p) } }
            static box Main { main() { local parent = new Parent() local item = new Item()
                local ignored = parent.second(item) return parent.first(item) } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text,
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("original source")
        };
        let request =
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
        MirCompiler::with_options(false).compile_normal_with_published(request, |view, _| -> Result<(), String> {
            let source = view.retained_root_source().unwrap();
            let ledger = &source.ledger;
            let original = view.module().clone();
            rearm_children(ledger, &original);
            ledger.validate_finalized_child_functions(&original, false)?;
            let reader = &original.functions["Parent.first/1"];
            let (read_block, read_index, read) = reader.blocks.iter().find_map(|(id, block)| {
                block.instructions.iter().enumerate().find_map(|(index, instruction)| {
                    matches!(instruction, MirInstruction::ObjectFieldGet { .. })
                        .then(|| (*id, index, instruction.clone()))
                })
            }).expect("healthy original receiver read");
            let MirInstruction::ObjectFieldGet { dst, base, field } = read else { unreachable!() };
            for change in 0..6 {
                let mut changed = original.clone();
                let reader = changed.functions.get_mut("Parent.first/1").unwrap();
                match change {
                    0 => {
                        let block = reader.blocks.get_mut(&read_block).unwrap();
                        block.instructions.remove(read_index);
                        block.instruction_spans.remove(read_index);
                    }
                    1 => reader.blocks.get_mut(&read_block).unwrap().add_instruction(read.clone()),
                    2..=4 => {
                        reader.blocks.get_mut(&read_block).unwrap().instructions[read_index] =
                            MirInstruction::ObjectFieldGet {
                                dst: if change == 2 { ValueId(9999) } else { dst },
                                base: if change == 3 { dst } else { base },
                                field: if change == 4 {
                                    hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(field.object(), 1).unwrap()
                                } else { field },
                            };
                    }
                    _ => {
                        let block = reader.blocks.get_mut(&read_block).unwrap();
                        let instruction = block.instructions.remove(read_index);
                        block.instruction_spans.remove(read_index);
                        let mut orphan = BasicBlock::new(BasicBlockId(99998));
                        orphan.add_instruction(instruction);
                        orphan.set_terminator(MirInstruction::Return { value: Some(dst) });
                        reader.add_block(orphan);
                    }
                }
                rearm_children(ledger, &original);
                let error = ledger.validate_finalized_child_functions(&changed, false)
                    .expect_err("changed stored receiver cannot finish");
                if change == 5 {
                    assert!(error.contains("ordinary-new/local-commit/call-binding-drift"), "{error}");
                } else {
                    assert!(error.contains("physical-boundary/")
                        || error.contains("ordinary-field-read/"), "change={change}: {error}");
                }
            }
            Ok(())
        }).unwrap();
    });
}

fn request(text: Option<&str>) -> NormalCompileRequestV1 {
    let text = text.unwrap_or("box Item { value: i64 other: i64 birth() { me.value = 5 me.other = 9 } }
        box Transport { flag: i64 birth() { me.flag = 0 }
            read(p: Item): i64 { if p == null { return 7 } return p.value } }
        static box Main { main() { local recv = new Transport() return recv.read(null) } }");
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    else {
        panic!("original source")
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
}

/// Re-arm only the test's finishing checkpoint from the pristine source-issued
/// functions. Semantic facts, emission rows and entry proofs stay unchanged.
fn rearm_children(ledger: &OrdinaryNewClaimLedgerV1, original: &crate::mir::MirModule) {
    let rows: Vec<_> = ledger
        .child_physical_validation
        .borrow()
        .iter()
        .map(|(owner, state)| {
            let symbol = match state {
                ChildPhysicalValidation::Checked { symbol, .. }
                | ChildPhysicalValidation::FinishingChecked { symbol, .. } => symbol.clone(),
            };
            (*owner, symbol)
        })
        .collect();
    for (owner, symbol) in rows {
        let function = &original.functions[&symbol];
        let bindings = ledger.lifecycle_bindings(owner).unwrap();
        let copies = ledger.source_local_copies(owner).unwrap();
        let aliases = ledger.borrowed_ordinary_alias_bindings_v1(owner).unwrap();
        let boundary =
            super::super::physical_boundary::PhysicalBoundary::capture_with_source_copies(
                function, &bindings, &copies, &aliases,
            )
            .unwrap();
        ledger
            .child_physical_validation
            .borrow_mut()
            .insert(owner, ChildPhysicalValidation::Checked { symbol, boundary });
    }
}

#[test]
fn full_child_finishing_rejects_foreign_field_positions_and_masked_exits() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (text, symbol, published) in [(None, "Transport.read/1", true),
            (Some(include_str!("../../../../apps/typed-object-method-min/main.hako")), "Pair.sum/0", true),
            (Some("box Counter { n: i64 other: i64 birth(capacity) { me.n = capacity me.other = 9 } read() { return me.n } }
                static box Main { main() { local c = new Counter(7) return c.read() } }"), "Counter.read/0", true),
            (Some("box Counter { n other: i64 birth(capacity) { me.n = capacity me.other = 9 } read() { return me.n } }
                static box Main { main() { local c = new Counter(7) return c.read() } }"), "Counter.read/0", false)] {
        let outcome = MirCompiler::with_options(false).compile_normal_with_published(
            request(text), |view, _| -> Result<(), String> {
                let source = view.retained_root_source().expect("same original ledger");
                let ledger = &source.ledger;
                let original = view.module().clone();
                rearm_children(ledger, &original);
                ledger.validate_finalized_child_functions(&original, false)?;
                let reader = &original.functions[symbol];
                if symbol == "Counter.read/0" {
                    assert_eq!(reader.signature.return_type, crate::mir::MirType::Integer);
                    assert_eq!(reader.params.len(), 1, "original implicit receiver ABI");
                    let reads = ledger.field_reads.borrow();
                    let (site, _) = reads.iter().find(|(_, row)| matches!(row.value_source, crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::TerminalFieldValueSourceV1::Numeric(_)))
                        .expect("same result-owner numeric field loan");
                    let target = ledger.terminal_relation.values().find_map(|relation| {
                        let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Call(call) = relation else { return None };
                        call.source_scalar().map(|loan| loan.source()).filter(|target| target.callee_owner() == site.owner())
                    }).expect("original getter incoming target");
                    let (selected, signatures) = ledger.original_instance_input_issuance.as_ref().unwrap();
                    let arguments = ledger.checked_completed_original_instance_target_arguments_v1(
                        target, selected, ledger.original_parameter_contracts.as_ref().unwrap(), signatures,
                    )?.expect("whole original empty input cohort finished");
                    assert!(arguments.is_empty());
                }
                let (read_block, read_index, read) = reader.blocks.iter().find_map(|(id, block)| {
                    block.instructions.iter().enumerate().find_map(|(index, instruction)| {
                        matches!(instruction, MirInstruction::ObjectFieldGet { .. })
                            .then(|| (*id, index, instruction.clone()))
                    })
                }).expect("original exact read");
                let MirInstruction::ObjectFieldGet { dst, base, field } = read else { unreachable!() };
                if symbol == "Pair.sum/0" {
                    let (add_block, add_index, add) = reader.blocks.iter().find_map(|(block, row)|
                        row.instructions.iter().enumerate().find_map(|(index, instruction)|
                            matches!(instruction, MirInstruction::BinOp { op: crate::mir::BinaryOp::Add, .. })
                                .then(|| (*block, index, instruction.clone())))).expect("original Add");
                    let MirInstruction::BinOp { dst: result, lhs, rhs, .. } = add else { unreachable!() };
                    for mutation in 0..5 {
                        let mut changed = original.clone();
                        let reader = changed.functions.get_mut(symbol).unwrap();
                        match mutation {
                            0 => {
                                let block = reader.blocks.get_mut(&add_block).unwrap();
                                block.instructions.remove(add_index);
                                block.instruction_spans.remove(add_index);
                            }
                            1 => reader.blocks.get_mut(&add_block).unwrap().instructions[add_index] =
                                MirInstruction::BinOp { dst: result, op: crate::mir::BinaryOp::Add, lhs: rhs, rhs: lhs },
                            2 => reader.blocks.get_mut(&add_block).unwrap().add_instruction(add.clone()),
                            3 | 4 => {
                                for block in reader.blocks.values_mut() {
                                    if matches!(block.terminator, Some(MirInstruction::Return { value: Some(value) }) if value == result) {
                                        block.set_terminator(MirInstruction::Return { value: Some(lhs) });
                                    }
                                }
                                if mutation == 4 {
                                    let mut orphan = BasicBlock::new(BasicBlockId(99998));
                                    orphan.set_terminator(MirInstruction::Return { value: Some(result) });
                                    reader.add_block(orphan);
                                }
                            }
                            _ => unreachable!(),
                        }
                        rearm_children(ledger, &original);
                        let issue = ledger.validate_finalized_child_functions(&changed, false)
                            .expect_err("same child Add and exact Return cannot drift");
                        assert!(issue.contains("[freeze:contract]"), "{mutation}: {issue}");
                        assert!(!issue.contains("duplicate-child"), "{issue}");
                    }
                }
                let orphan = BasicBlockId(99998);
                assert!(!reader.blocks.contains_key(&orphan));
                for change in 0..6 {
                    let mut changed = original.clone();
                    let reader = changed.functions.get_mut(symbol).unwrap();
                    match change {
                        0 => {
                            let block = reader.blocks.get_mut(&read_block).unwrap();
                            let instruction = block.instructions.remove(read_index);
                            block.instruction_spans.remove(read_index);
                            let mut isolated = BasicBlock::new(orphan);
                            isolated.add_instruction(instruction);
                            isolated.set_terminator(MirInstruction::Return { value: Some(dst) });
                            reader.add_block(isolated);
                        }
                        1..=3 => {
                            reader.blocks.get_mut(&read_block).unwrap().instructions[read_index] =
                                MirInstruction::ObjectFieldGet {
                                    dst: if change == 1 { ValueId(9999) } else { dst },
                                    base: if change == 2 { dst } else { base },
                                    field: if change == 3 {
                                        hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(field.object(), if field.declaration_ordinal() == 0 { 1 } else { 0 }).unwrap()
                                    } else { field },
                                };
                        }
                        4 => reader.blocks.get_mut(&read_block).unwrap().add_instruction(
                            MirInstruction::ObjectFieldGet { dst, base, field },
                        ),
                        5 => {
                            let replacement = if symbol == "Counter.read/0" || symbol == "Pair.sum/0" {
                                let replacement = ValueId(99997);
                                reader.blocks.get_mut(&reader.entry_block).unwrap().add_instruction(MirInstruction::Const {
                                    dst: replacement, value: ConstValue::Integer(7),
                                });
                                replacement
                            } else { reader.blocks.values().flat_map(|b| &b.instructions)
                                .find_map(|i| match i {
                                    MirInstruction::Const { value: ConstValue::Integer(7), .. } => i.dst_value(),
                                    _ => None,
                                }).expect("existing null-arm value") };
                            for block in reader.blocks.values_mut() {
                                if matches!(block.terminator, Some(MirInstruction::Return { value: Some(value) }) if value == dst || symbol == "Pair.sum/0") {
                                    block.set_terminator(MirInstruction::Return { value: Some(replacement) });
                                }
                            }
                            let mut isolated = BasicBlock::new(orphan);
                            isolated.set_terminator(MirInstruction::Return { value: Some(dst) });
                            reader.add_block(isolated);
                        }
                        _ => unreachable!(),
                    }
                    rearm_children(ledger, &original);
                    let error = ledger.validate_finalized_child_functions(&changed, false)
                        .expect_err(&format!("mutation {change} must fail child finishing"));
                    assert!(error.contains("[freeze:contract]"), "{change}: {error}");
                    assert!(!error.contains("duplicate-child"), "must exercise the final semantic boundary: {error}");
                }
                if symbol == "Counter.read/0" {
                    // Matching source-row and physical corruption must still
                    // lose the immutable original field-source loan.
                    let site = ledger.field_reads.borrow().iter()
                        .find(|(_, row)| matches!(row.value_source, crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_reads::TerminalFieldValueSourceV1::Numeric(_))).unwrap().0.clone();
                    let foreign = hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(field.object(), 1).unwrap();
                    ledger.field_reads.borrow_mut().get_mut(&site).unwrap().field = foreign;
                    let mut changed = original.clone();
                    changed.functions.get_mut(symbol).unwrap().blocks.get_mut(&read_block).unwrap()
                        .instructions[read_index] = MirInstruction::ObjectFieldGet { dst, base, field: foreign };
                    rearm_children(ledger, &original);
                    let outcome = ledger.validate_finalized_child_functions(&changed, false);
                    ledger.field_reads.borrow_mut().get_mut(&site).unwrap().field = field;
                    assert!(outcome.unwrap_err().contains("integer-field/source-identity"));
                }
                Ok(())
            },
        );
        if published {
            outcome.unwrap_or_else(|issue| panic!("{symbol}: {issue}"));
        } else {
            let issue = outcome.err().expect("unannotated source class remains unproved");
            assert!(issue.contains("borrowed-result/source-not-i64"), "{issue}");
        }
        }
    });
}
