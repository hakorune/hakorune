//! Source-backed pure scalar roots consume exact field ports in existing operators.
use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::mir::{MirInstruction, MirType};
use crate::parser::ParserBuildConfig;

fn lower(body: &str) -> crate::mir::MirModule {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        &format!(
            "box Pool {{ size: i64 = 0 birth() {{ }} }}
        static box Main {{ main() {{ local pool = new Pool() {body} }} }}"
        ),
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed scalar root must lower");
    completed.into_parts().1
}

#[test]
fn scalar_expression_emits_typed_field_reads_and_arithmetic() {
    let module = lower("local n = pool.size + 1 return n");
    let main = module
        .functions
        .values()
        .find(|f| f.signature.name == "main")
        .expect("main");
    let reads: Vec<_> = main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .filter_map(|i| match i {
            MirInstruction::ObjectFieldGet { dst, field, .. } => Some((*dst, *field)),
            _ => None,
        })
        .collect();
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].1.declaration_ordinal(), 0);
    assert_eq!(
        main.metadata.value_types.get(&reads[0].0),
        Some(&MirType::Integer)
    );
    let instructions: Vec<_> = main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .collect();
    let copy_origin = |mut value| {
        for _ in 0..=instructions.len() {
            let Some(source) = instructions.iter().find_map(|i| match i {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            }) else {
                return value;
            };
            value = source;
        }
        panic!("copy cycle in source-backed scalar arithmetic");
    };
    assert!(
        instructions.iter().any(|i| matches!(i,
            MirInstruction::BinOp { op: crate::mir::BinaryOp::Add, lhs, rhs, .. }
                if copy_origin(*lhs) == reads[0].0 || copy_origin(*rhs) == reads[0].0
        )),
        "Add must consume the exact read, possibly through source copies: {instructions:#?}"
    );
    assert!(!main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .any(|i| matches!(i, MirInstruction::FieldGet { .. })));

    // Same original nested source path, through the real finishing handoff.
    // A same-typed sibling or missing ancestor is never a receiver substitute.
    for mutation in ["none", "ancestor-base", "leaf-base", "sibling-base", "missing-leaf", "missing-ancestor"] {
        let source = callable_source(
            "box Leaf { n: usize = 3 birth() { } }
             box Parent { left: Leaf = new Leaf() right: Leaf = new Leaf()
                 birth() { } total() { return me.left.n + me.right.n } }
             static box Main { main() { local parent = new Parent() return parent.total() } }",
            ParserBuildConfig::default(),
        );
        let completed = session().complete_normal_default_program_root_catalog_lifecycle(
            source, CallableMainMaterializationPolicyV1::Omitted, NormalRuntimeInputSnapshotV1::empty(),
        ).expect("original nested scalar path must lower without dynamic field retry");
        let (_, mut module, validate) = completed.into_artifact_parts();
        let total = module.functions.get_mut("Parent.total/0").expect("source-selected callee");
        assert_eq!(total.signature.return_type, MirType::Integer);
        let reads: Vec<_> = total.blocks.iter().flat_map(|(id, block)| block.instructions.iter().enumerate()
            .filter_map(move |(index, instruction)| match instruction {
                MirInstruction::ObjectFieldGet { dst, base, field } => Some((*id, index, *dst, *base, *field)),
                _ => None,
            })).collect();
        assert_eq!(reads.len(), 4);
        assert!(!total.blocks.values().flat_map(|block| block.all_instructions())
            .any(|instruction| matches!(instruction, MirInstruction::FieldGet { .. })));
        let leaf = reads.iter().rev().find(|read| total.metadata.value_types.get(&read.2) == Some(&MirType::Integer))
            .expect("numeric leaf retains Integer from original writer proof");
        assert!(reads.iter().any(|read| read.2 == leaf.3), "leaf base must be original object edge result");
        match mutation {
            "none" => { validate(&module).expect("source path survives final consumer validation"); continue; }
            "missing-leaf" | "missing-ancestor" => {
                let chosen = if mutation == "missing-leaf" { leaf } else {
                    reads.iter().find(|read| read.2 == leaf.3).unwrap()
                };
                let block = total.blocks.get_mut(&chosen.0).unwrap();
                block.instructions.remove(chosen.1); block.instruction_spans.remove(chosen.1);
            }
            "sibling-base" => {
                let sibling = reads.iter().find(|read| read.2 != leaf.3 &&
                    total.metadata.value_types.get(&read.2) == Some(&MirType::Box("Leaf".into()))).unwrap().2;
                let MirInstruction::ObjectFieldGet { base, .. } = &mut total.blocks.get_mut(&leaf.0).unwrap().instructions[leaf.1]
                    else { unreachable!() };
                *base = sibling;
            }
            _ => {
                let chosen = if mutation == "leaf-base" { leaf } else {
                    reads.iter().find(|read| read.2 == leaf.3).unwrap()
                };
                let MirInstruction::ObjectFieldGet { base, .. } = &mut total.blocks.get_mut(&chosen.0).unwrap().instructions[chosen.1]
                    else { unreachable!() };
                *base = crate::mir::ValueId(90001);
            }
        }
        assert!(validate(&module).is_err(), "original path drift {mutation} must refuse");
    }
}

#[test]
fn scalar_expression_short_circuit_rhs_read_stays_in_deferred_block() {
    for body in [
        "local b = pool.size == 0 && pool.size != 1 if b { return 1 } return 0",
        "if pool.size == 0 && pool.size != 1 { return 1 } return 0",
        "if pool.size != 0 || pool.size == 1 { return 1 } return 0",
    ] {
    let module = lower(body);
    let main = module
        .functions
        .values()
        .find(|f| f.signature.name == "main")
        .expect("main");
    let blocks: Vec<_> = main
        .blocks
        .iter()
        .flat_map(|(id, b)| {
            b.all_instructions().filter_map(move |i| {
                matches!(i, MirInstruction::ObjectFieldGet { .. }).then_some(*id)
            })
        })
        .collect();
    assert_eq!(blocks.len(), 2);
    assert_ne!(
        blocks[0], blocks[1],
        "RHS must not be hoisted into LHS block"
    );
    assert!(!main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .any(|i| matches!(i, MirInstruction::FieldGet { .. })));
    }
}

#[test]
fn text_condition_keeps_short_circuit_and_final_source_operation() {
    use crate::mir::instruction::{InvokeOperation, StringInvokeOperation as Operation, StringInvokeOperand as Operand};
    use crate::mir::{BasicBlockId, ValueId};
    for (prefix, condition, residence, in_method, in_loop) in [
        ("", "\"0\" < \"5\" || \"9\" < \"x\"", false, true, false),
        ("", "\"0\" < \"5\" || \"9\" < \"x\"", false, false, false),
        ("local text = \"5\" local alias = text", "alias < \"0\" || alias > \"9\"", false, false, false),
        ("local text = \"\" + \"5\" local alias = text", "alias < \"0\" || alias > \"9\"", true, false, false),
        ("local a = \"5\" local b = \"9\"", "a < b && b >= a", false, false, false),
        ("local text = \"5\"", "text.substring(0, 1) < \"0\" || text.substring(0, 1) > \"9\"", false, false, false),
        ("local text = \"5\" local m = text.length() if m == 0 {}", "text.substring(i, i + 1) < \"0\" || text.substring(i, i + 1) > \"9\"", false, false, true),
    ] {
        let text = if in_loop {
            format!("box Pool {{ size: i64 = 0 birth() {{}} }} static box Main {{ main() {{
                local pool = new Pool() {prefix} local i = 0 loop(i < 2) {{
                if {condition} {{ return i }} i = i + 1 }} return i }} }}")
        } else if in_method {
            format!("box Pool {{ size: i64 = 0 birth() {{}} compare(): i64 {{
                if {condition} {{ return 1 }} return 0 }} }} static box Main {{ main() {{
                local pool = new Pool() return pool.compare() }} }}")
        } else {
            let pool = if prefix.is_empty() { "" } else { "local pool = new Pool()" };
            format!("box Pool {{ size: i64 = 0 birth() {{}} }} static box Main {{ main() {{
                {pool} {prefix} if {condition} {{ return 1 }} return 0 }} }}")
        };
        for simplify in [false, true] {
        let (completed, ledger) = crate::mir::builder::lexical_call_projection_document_completion_fixture(&text);
        let (key, module, _, keys, cohort) = completed.document_preflight_parts_for_test(simplify).unwrap();
        let main_key = if in_method { "Pool.compare/0".to_owned() } else { key.clone() };
        let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new()).unwrap();
        let source = handoff.root_source().unwrap();
        source.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(())).unwrap();
        let main = &module.functions[&main_key];
        // Document fixtures do not install declared method-return carriers.
        // Main still exercises the independent physical lifetime/SSA verifier.
        if !in_method {
            let physical = crate::mir::verification::MirVerifier::new_strict().verify_function(main);
            assert!(physical.is_ok(), "{prefix}: {physical:?}\n{main:#?}");
        }
        let comparisons: Vec<_> = main.blocks.iter().filter_map(|(id, block)| match block.terminator.as_ref() {
            Some(row @ MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Compare { .. }), .. }) => Some((*id, row.clone())),
            _ => None,
        }).collect();
        assert_eq!(comparisons.len(), 2);
        assert_ne!(comparisons[0].0, comparisons[1].0, "RHS comparison is not eagerly emitted in LHS block");
        let rhs = main.blocks.values().filter_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Branch { then_bb, else_bb, then_edge_args: None, else_edge_args: None, .. }) =>
                Some([*then_bb, *else_bb]), _ => None,
        }).flatten().any(|mut target| {
            let mut seen = std::collections::BTreeSet::new();
            while seen.insert(target) {
                if comparisons.iter().any(|(id, _)| *id == target) { return true; }
                target = match main.blocks[&target].terminator.as_ref() {
                    Some(MirInstruction::Jump { target, edge_args: None }) => *target,
                    Some(MirInstruction::Invoke { normal_landing, .. }) => *normal_landing,
                    _ => return false,
                };
            }
            false
        });
        assert!(rhs, "original short-circuit branch reaches the second comparison");
        let rows: Vec<_> = main.blocks.values().flat_map(|block| block.all_instructions()).collect();
        let mut roots: std::collections::BTreeSet<_> = rows.iter().filter_map(|row| match row {
            MirInstruction::Const { dst, value: crate::mir::ConstValue::String(_) } => Some(*dst), _ => None,
        }).collect();
        for (_, row) in &comparisons {
            if let MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Compare { left, right, .. }), .. } = row {
                for operand in [left, right] { if let Operand::OwnedString(value) = operand { roots.insert(*value); } }
            }
        }
        let text_copy = rows.iter().find_map(|row| match row {
            MirInstruction::Copy { dst, src } => {
                let mut value = *src;
                for _ in 0..rows.len() {
                    if roots.contains(&value) { return Some((*dst, *src)); }
                    let next = rows.iter().find_map(|row| match row {
                        MirInstruction::Copy { dst, src } if *dst == value => Some(*src), _ => None,
                    });
                    let Some(next) = next else { break; }; value = next;
                }
                None
            }, _ => None,
        });
        let mut faults = std::collections::BTreeSet::new();
        let mut pending: Vec<_> = comparisons.iter().filter_map(|(_, row)| match row {
            MirInstruction::Invoke { fault_landing, .. } => Some(*fault_landing), _ => None,
        }).collect();
        while let Some(id) = pending.pop() {
            if !faults.insert(id) { continue; }
            match main.blocks[&id].terminator.as_ref().unwrap() {
                MirInstruction::Invoke { normal_landing, fault_landing, .. } => pending.extend([*normal_landing, *fault_landing]),
                MirInstruction::Jump { target, .. } => pending.push(*target),
                MirInstruction::ReturnFault { .. } => {}, _ => panic!("comparison Fault escaped its unwind"),
            }
        }
        for mutation in ["operator", "operand", "normal", "fault", "frame", "projection", "logical-edge", "copy-source", "text-end", "residence-producer", "return-value"] {
            if mutation == "return-value" && !in_loop { continue; }
            if mutation == "copy-source" && text_copy.is_none() { continue; }
            if mutation == "residence-producer" && !residence { continue; }
            if mutation == "text-end" && !residence && !condition.contains("substring") { continue; }
            let mut changed = module.clone();
            let function = changed.functions.get_mut(&main_key).unwrap();
            let mut count = 0;
            for (block_id, block) in function.blocks.iter_mut() {
                for row in block.instructions.iter_mut().chain(block.terminator.iter_mut()) {
                    if count != 0 { break; }
                    match (mutation, row) {
                        ("return-value", MirInstruction::Return { value }) => { *value = Some(ValueId(90006)); count += 1; }
                        ("operator" | "operand" | "normal" | "fault" | "frame", MirInstruction::Invoke {
                            operation: InvokeOperation::String(Operation::Compare { op, left, .. }), normal_landing, fault_landing, fault_frame,
                        }) => {
                            match mutation {
                                "operator" => *op = if *op == crate::mir::CompareOp::Eq { crate::mir::CompareOp::Ne } else { crate::mir::CompareOp::Eq },
                                "operand" => *left = Operand::StaticText { utf8: "foreign".into() },
                                "normal" => *normal_landing = BasicBlockId(90001),
                                "fault" => *fault_landing = *normal_landing,
                                "frame" => *fault_frame = ValueId(90002),
                                _ => unreachable!(),
                            }
                            count += 1;
                        }
                        ("residence-producer", MirInstruction::Invoke { operation: InvokeOperation::String(Operation::Add { left, .. }), .. }) => {
                            *left = Operand::StaticText { utf8: "foreign".into() }; count += 1;
                        }
                        ("projection", MirInstruction::InvokeNormalResult { invoke_block, .. })
                            if comparisons.iter().any(|(id, _)| id == invoke_block) => { *invoke_block = BasicBlockId(90003); count += 1; }
                        ("logical-edge", MirInstruction::Branch { then_bb, else_bb, then_edge_args: None, else_edge_args: None, .. }) => {
                            std::mem::swap(then_bb, else_bb); count += 1;
                        }
                        ("copy-source", MirInstruction::Copy { dst, src }) if Some((*dst, *src)) == text_copy => { *src = ValueId(90004); count += 1; }
                        ("text-end", MirInstruction::Invoke { operation: InvokeOperation::String(Operation::End { value }), .. })
                            if faults.contains(block_id) && roots.contains(value) => {
                            *value = ValueId(90005); count += 1;
                        }
                        _ => {}
                    }
                }
            }
            assert_eq!(count, 1, "{prefix}: {mutation}");
            assert!(source.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(())).is_err(),
                "Text condition final consumer admitted {mutation}: {prefix}");
        }
        }
    }
}
