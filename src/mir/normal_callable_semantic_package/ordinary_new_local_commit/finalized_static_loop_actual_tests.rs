//! Finalized canonical packet loans retain original actual and producer identity.
use super::*;
use crate::mir::compiler::{MirCompiler, NormalCompileRequestV1};
use crate::mir::instruction::InvokeOperation;

pub(super) fn selected_request() -> NormalCompileRequestV1 {
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("lang/src/hako_alloc/memory/size_class_box.hako"),
    )
    .unwrap();
    let text =
        format!("{text}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}");
    request_from_text(&text)
}

fn request_from_text(text: &str) -> NormalCompileRequestV1 {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("original source identity must survive");
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
        .for_closed_app_executable()
}

#[test]
fn finalized_mul_nonviews_keep_original_producers_and_reject_mutations() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for expression in [
            "p * 2",
            "2 * p",
            "p * -2",
            "-2 * p",
            "p * me.word_size()",
            "me.word_size() * p",
        ] {
            let original =
                include_str!("../../../../lang/src/hako_alloc/memory/size_class_box.hako");
            let source = original.replacen(
                "return bin * me.word_size()",
                &format!("return {}", expression.replace("p", "bin")),
                1,
            );
            let text = format!(
                "{source}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(9) }} }}"
            );
            for optimize in [false, true] {
                let mut compiler = MirCompiler::with_options(optimize);
                compiler.compile_normal_with_published(request_from_text(&text), |view, verification| {
                    assert!(verification.is_ok());
                    let source = view.retained_root_source().unwrap();
                    let owners: Vec<_> = source.ledger.borrowed_entry_values.borrow().keys().copied().collect();
                    let mut observed = 0;
                    for owner in owners {
                        let symbol = source.ledger.with_finished_projection(owner, |symbol, _| Ok(symbol.to_owned()))?;
                        let function = &view.module().functions[&symbol];
                        source.with_borrowed_ordinary_muls_v1(owner, function, |loan, mul, _| {
                            observed += 1;
                            let side = (0..2).find(|side| loan.integer_literal(*side).is_some()
                                || loan.integer_call_source(*side).is_some()).unwrap();
                            let raw = match &mul.1 {
                                MirInstruction::BinOp { lhs, rhs, .. } => [*lhs, *rhs][side],
                                _ => unreachable!(),
                            };
                            // The exact final original is lent by this same boundary;
                            // mutate its producer independently of source observations.
                            let original = source.ledger.with_finished_projection(owner, |_, projection| {
                                let mut records = Vec::new();
                                source.ledger.with_borrowed_ordinary_muls_v1(owner, |record, _| {
                                    if record.source().site() == loan.site() {
                                        let child = [record.children().0, record.children().1][side];
                                        let captured = projection.captured_definition(child)?;
                                        records.push(projection.relation_binding(function, captured)?.unwrap());
                                    }
                                    Ok(())
                                })?;
                                assert_eq!(records.len(), 1);
                                Ok(records.pop().unwrap())
                            })?;
                            if loan.integer_literal(side).is_some() {
                                let mut unreachable = None;
                                for (id, block) in &function.blocks {
                                    let Some(MirInstruction::Branch { then_bb, else_bb, .. }) = &block.terminator else { continue };
                                    for target in [*then_bb, *else_bb] {
                                        let mut changed = function.clone();
                                        let Some(MirInstruction::Branch { then_bb, else_bb, .. }) = &mut changed.blocks.get_mut(id).unwrap().terminator else { unreachable!() };
                                        *then_bb = target;
                                        *else_bb = target;
                                        let block = changed.blocks.get_mut(id).unwrap();
                                        block.set_terminator(block.terminator.clone().unwrap());
                                        if !crate::mir::verification::utils::compute_dominators(&changed).is_reachable(original.0) {
                                            unreachable = Some(changed);
                                            break;
                                        }
                                    }
                                    if unreachable.is_some() { break; }
                                }
                                let changed = unreachable.expect("guard edge makes original producer unreachable");
                                assert!(changed.blocks[&original.0].instructions.contains(&original.1));
                                assert!(changed.blocks[&mul.0].instructions.contains(&mul.1));
                                let error = source.with_borrowed_ordinary_muls_v1(owner, &changed, |_, _, _| Ok(())).unwrap_err();
                                assert!(error.contains("borrowed-mul/nonview-order"), "{error}");
                            } else {
                                let mut independent = MirCompiler::with_options(optimize);
                                independent.compile_normal_with_published(request_from_text(&text), |foreign_view, verification| {
                                    assert!(verification.is_ok());
                                    let foreign = foreign_view.retained_root_source().unwrap();
                                    let foreign_owners: Vec<_> = foreign.ledger.borrowed_entry_values.borrow().keys().copied().collect();
                                    let mut matched = 0;
                                    for foreign_owner in foreign_owners {
                                        foreign.ledger.with_borrowed_ordinary_muls_v1(foreign_owner, |record, _| {
                                            if record.source().site().site() != loan.site().site() { return Ok(()); }
                                            matched += 1;
                                            assert_ne!(record.source().owner(), owner, "independent invocation has another source owner");
                                            let error = source.verify_finished_mul_nonviews_v1(owner, function, record, mul, &[None, None]).unwrap_err();
                                            assert!(error.contains("borrowed-mul/nonview-owner"), "{error}");
                                            Ok(())
                                        })?;
                                    }
                                    assert_eq!(matched, 1);
                                    Ok::<_, String>(())
                                })?;
                            }
                            for mutation in 0..5 {
                                let mut changed = function.clone();
                                let block = changed.blocks.get_mut(&original.0).unwrap();
                                let index = block.instructions.iter().position(|row| row == &original.1).unwrap();
                                match mutation {
                                    0 => { block.instructions.remove(index); }
                                    1 => { block.add_instruction(original.1.clone()); }
                                    2 => {
                                        match &mut block.instructions[index] {
                                            MirInstruction::Const { value: crate::mir::ConstValue::Integer(value), .. } => *value += 1,
                                            MirInstruction::InvokeNormalResult { invoke_block, .. } => *invoke_block = BasicBlockId(999999),
                                            _ => panic!("original literal or Normal"),
                                        }
                                    }
                                    3 => {
                                        let mut substitute = original.1.clone();
                                        match &mut substitute {
                                            MirInstruction::Const { dst, .. } | MirInstruction::InvokeNormalResult { dst, .. } => *dst = ValueId(999999),
                                            _ => unreachable!(),
                                        }
                                        block.add_instruction(substitute);
                                        let mul_block = changed.blocks.get_mut(&mul.0).unwrap();
                                        let row = mul_block.instructions.iter_mut().find(|row| *row == &mul.1).unwrap();
                                        let MirInstruction::BinOp { lhs, rhs, .. } = row else { unreachable!() };
                                        assert_eq!([*lhs, *rhs][side], raw);
                                        if side == 0 { *lhs = ValueId(999999); } else { *rhs = ValueId(999999); }
                                    }
                                    _ => {
                                        let mul_block = changed.blocks.get_mut(&mul.0).unwrap();
                                        let row = mul_block.instructions.iter_mut().find(|row| *row == &mul.1).unwrap();
                                        let MirInstruction::BinOp { lhs, rhs, .. } = row else { unreachable!() };
                                        std::mem::swap(lhs, rhs);
                                    }
                                }
                                assert!(source.with_borrowed_ordinary_muls_v1(owner, &changed, |_, _, _| Ok(())).is_err(),
                                    "{expression}, optimize={optimize}, mutation={mutation}");
                            }
                            if loan.integer_call_source(side).is_some() {
                                let mut changed = function.clone();
                                let mut found = false;
                                for block in changed.blocks.values_mut() {
                                    if let Some(MirInstruction::Invoke { normal_landing, fault_landing, .. }) = &mut block.terminator {
                                        if *normal_landing == original.0 {
                                            *fault_landing = BasicBlockId(999999);
                                            found = true;
                                        }
                                    }
                                }
                                assert!(found);
                                assert!(source.with_borrowed_ordinary_muls_v1(owner, &changed, |_, _, _| Ok(())).is_err());
                            }
                            Ok(())
                        })?;
                    }
                    assert_eq!(observed, 1, "one original source Mul: {expression}");
                    view.issue_lifecycle_compiled_entry_contract()?;
                    Ok::<_, String>(())
                }).unwrap_or_else(|error| panic!("{expression}, optimize={optimize}: {error}"));
            }
        }
    });
}

#[test]
fn finalized_source_loop_rejects_initial_bin_and_normal_fault_target() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let original = include_str!("../../../../lang/src/hako_alloc/memory/size_class_box.hako");
        let text = format!(
            "{original}\nstatic box Main {{ main() {{ return SizeClassBox.size_to_bin(4194304) }} }}"
        );
        let mut compiler = MirCompiler::with_options(false);
        compiler
            .compile_normal_with_published(request_from_text(&text), |view, verification| {
                assert!(verification.is_ok(), "original MIR must verify");
                let source = view.retained_root_source().expect("original source");
                let target = crate::mir::Callee::Global(
                    crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                        "SizeClassBox",
                        "bin_size",
                        1,
                    )
                    .canonical_global_target_v1()
                    .unwrap(),
                );
                let mut visit_count = 0;
                source.visit_finalized_lexical_call_nodes_v1(
                    view.module(),
                    |owner, _, _, _, function, coordinate, _| {
                        let Some(MirInstruction::Invoke {
                            operation: InvokeOperation::Call { call, .. },
                            ..
                        }) = function.blocks[&coordinate.0].terminator.as_ref()
                        else {
                            return Ok(());
                        };
                        if call.callee != target {
                            return Ok(());
                        }
                        visit_count += 1;
                        let current = call.args[0];
                        let initial = function
                            .blocks
                            .values()
                            .flat_map(|block| block.all_instructions())
                            .find_map(|instruction| match instruction {
                                MirInstruction::Phi { dst, inputs, .. } if *dst == current => {
                                    Some(inputs)
                                }
                                _ => None,
                            })
                            .expect("the body reads the current header binding")
                            .iter()
                            .find_map(|(block, incoming)| {
                                let predecessor = &function.blocks[block];
                                let original = predecessor
                                    .all_instructions()
                                    .find_map(|row| match row {
                                        MirInstruction::Copy { dst, src } if dst == incoming => {
                                            Some(*src)
                                        }
                                        _ => None,
                                    })
                                    .unwrap_or(*incoming);
                                predecessor.all_instructions().find_map(|row| match row {
                                    MirInstruction::Const {
                                        dst,
                                        value: crate::mir::ConstValue::Integer(1),
                                    } if *dst == original => Some(*dst),
                                    _ => None,
                                })
                            })
                            .expect("find the real I64 initializer in the preheader");
                        assert_ne!(initial, current);
                        for value in [initial, current] {
                            assert_eq!(
                                function.metadata.value_types.get(&value),
                                Some(&crate::mir::MirType::Integer)
                            );
                        }
                        let mut original_seen = 0;
                        source.with_finished_bool_bindings_v1(owner, function, |at, _| {
                            original_seen += usize::from(at == coordinate);
                            Ok(())
                        })?;
                        assert_eq!(original_seen, 1, "original common Bool verifier");
                        for change in 0..3 {
                            let mut changed = function.clone();
                            let invoke = changed
                                .blocks
                                .get_mut(&coordinate.0)
                                .unwrap()
                                .terminator
                                .as_mut()
                                .unwrap();
                            let MirInstruction::Invoke {
                                operation: InvokeOperation::Call { call, .. },
                                normal_landing,
                                fault_landing,
                                ..
                            } = invoke
                            else {
                                unreachable!()
                            };
                            match change {
                                0 => call.args[0] = initial,
                                1 => {
                                    assert_ne!(normal_landing, fault_landing);
                                    *fault_landing = *normal_landing;
                                }
                                _ => {
                                    call.callee = crate::mir::Callee::Global(
                                        crate::mir::builder::CanonicalSameModuleCallableKeyV1::static_box_method(
                                            "SizeClassBox", "normalize_size", 1,
                                        ).canonical_global_target_v1().unwrap(),
                                    );
                                }
                            }
                            assert!(
                                source.with_finished_bool_bindings_v1(
                                    owner, &changed, |_, _| Ok(())
                                ).is_err(),
                                "changed argument, Fault edge, or target {change}"
                            );
                        }
                        Ok(())
                    },
                )?;
                assert_eq!(visit_count, 1, "must exercise the actual bin_size Invoke");
                view.issue_lifecycle_compiled_entry_contract()?;
                Ok(())
            })
            .unwrap();
    });
}
