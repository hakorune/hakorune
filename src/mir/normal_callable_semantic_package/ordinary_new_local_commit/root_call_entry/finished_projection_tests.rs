//! Original source nodes remain the authority for finished physical coordinates.
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use std::collections::BTreeSet;

fn producers<'a>(
    packet: &'a EmittedLexicalCallProjectionV1,
    result: &mut Vec<(&'a OwnedExprSiteV1, &'a Binding)>,
) {
    result.push((packet.call_site(), &packet.invoke));
    result.push((packet.call_site(), &packet.projection));
    for argument in &packet.prepared.arguments {
        match argument {
            LexicalCallArgumentProjectionV1::Integer(binding) => {
                result.push((packet.call_site(), binding));
            }
            LexicalCallArgumentProjectionV1::CallResult(inner) => producers(inner, result),
            LexicalCallArgumentProjectionV1::Scalar(_) => {}
            _ => panic!("strict fixture contains no borrowed arguments"),
        }
    }
}

#[test]
fn original_root_and_child_nodes_lend_unique_finished_producers_after_simplify() {
    let (module, handoff) =
        crate::mir::builder::lexical_call_projection_finished_artifact_fixture();
    let source = handoff.root_source().unwrap();
    let mut owners = BTreeSet::new();
    let mut nested = 0;
    for (owner, group) in source.local_call_binding_groups() {
        let Some(packet) = group.lexical() else {
            continue;
        };
        owners.insert(owner);
        let mut nodes = Vec::new();
        producers(packet, &mut nodes);
        for (site, original) in nodes {
            nested += usize::from(site != group.site());
            let matches: Vec<_> = module
                .functions
                .values()
                .filter_map(|function| {
                    source
                        .finished_local_call_producer_v1(
                            owner,
                            group.site(),
                            site,
                            original,
                            function,
                        )
                        .ok()
                        .map(|coordinate| (function, coordinate))
                })
                .collect();
            assert_eq!(matches.len(), 1, "one original owner/function/producer");
            let (function, (block, index)) = matches[0];
            let actual = function.blocks[&block]
                .all_instructions()
                .nth(index)
                .unwrap();
            match (&original.1, actual) {
                (
                    MirInstruction::InvokeNormalResult { dst: expected, .. },
                    MirInstruction::InvokeNormalResult {
                        dst: actual,
                        invoke_block,
                    },
                ) => {
                    assert_eq!(expected, actual);
                    assert!(matches!(function.blocks[invoke_block].terminator,
                        Some(MirInstruction::Invoke { normal_landing, .. }) if normal_landing == block));
                }
                _ => assert_eq!(actual, &original.1),
            }
        }
    }
    assert_eq!(
        owners.len(),
        2,
        "root and ordinary child retain their own maps"
    );
    assert!(nested > 0, "nested node sites are checked separately");
}

#[test]
fn finished_lender_rejects_foreign_original_site_block_and_changed_actual() {
    let (module, handoff) =
        crate::mir::builder::lexical_call_projection_finished_artifact_fixture();
    let source = handoff.root_source().unwrap();
    let (owner, group) = source
        .local_call_binding_groups()
        .find(|(_, group)| group.lexical().is_some())
        .unwrap();
    let packet = group.lexical().unwrap();
    let inner = packet
        .prepared
        .arguments
        .iter()
        .find_map(|argument| match argument {
            LexicalCallArgumentProjectionV1::CallResult(inner) => Some(inner),
            _ => None,
        })
        .unwrap();
    let original = &inner.invoke;
    let (function, (block, _)) = module
        .functions
        .values()
        .find_map(|function| {
            source
                .finished_local_call_producer_v1(
                    owner,
                    group.site(),
                    inner.call_site(),
                    original,
                    function,
                )
                .ok()
                .map(|coordinate| (function, coordinate))
        })
        .unwrap();
    let reject = |site: &OwnedExprSiteV1, binding: &Binding, actual: &crate::mir::MirFunction| {
        source
            .finished_local_call_producer_v1(owner, group.site(), site, binding, actual)
            .unwrap_err()
    };
    assert!(reject(group.site(), original, function).contains("original-producer"));
    let wrong_block = (BasicBlockId(u32::MAX), original.1.clone());
    assert!(reject(inner.call_site(), &wrong_block, function).contains("original-producer"));
    let unrelated = group
        .bindings()
        .iter()
        .find(|binding| {
            !packet.has_producer_at(packet.call_site(), binding)
                && !packet.has_producer_at(inner.call_site(), binding)
        })
        .unwrap();
    assert!(reject(inner.call_site(), unrelated, function).contains("original-producer"));
    let mut changed = function.clone();
    changed.signature.name.push_str("-foreign");
    assert!(reject(inner.call_site(), original, &changed).contains("/function"));
    let mut missing = function.clone();
    missing.blocks.get_mut(&block).unwrap().terminator = None;
    assert!(reject(inner.call_site(), original, &missing).contains("producer-missing"));
    let mut duplicate = function.clone();
    let duplicate_block = duplicate.blocks.get_mut(&block).unwrap();
    duplicate_block
        .instructions
        .push(duplicate_block.terminator.clone().unwrap());
    assert!(reject(inner.call_site(), original, &duplicate).contains("producer-duplicate"));
}

#[test]
fn finalized_call_visitor_lends_original_nested_nodes_in_evaluation_order() {
    use crate::mir::normal_callable_semantic_package::FinalizedLexicalCallContextV1 as Context;
    let (module, handoff) =
        crate::mir::builder::lexical_call_projection_finished_artifact_fixture();
    let source = handoff.root_source().unwrap();
    let mut seen = Vec::new();
    source
        .visit_finalized_lexical_call_nodes_v1(
            &module,
            |owner, context, packet, arguments, function, (block, index), copies| {
                let Context::Local {
                    group_site,
                    declaration,
                    binding,
                } = context
                else {
                    panic!("fixture's original local destination");
                };
                assert_eq!(group_site.owner(), owner);
                assert_eq!(binding.owner(), owner);
                assert!(matches!(
                    declaration,
                    crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
                ));
                assert_eq!(
                    arguments.len(),
                    packet.original_row().unwrap().argument_sites().len()
                );
                assert!(matches!(
                    function.blocks[&block].all_instructions().nth(index),
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call {
                            result: InvokeCallResultKind::I64,
                            ..
                        },
                        ..
                    })
                ));
                for (original, finished) in copies {
                    assert_eq!(
                        function.blocks[&finished.0].instructions[finished.1],
                        original.1
                    );
                }
                seen.push((owner, group_site.clone(), packet.call_site().clone()));
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(seen.len(), 5);
    assert_eq!(
        seen.iter()
            .map(|(owner, _, _)| *owner)
            .collect::<BTreeSet<_>>()
            .len(),
        2
    );
    for (_, group, _) in &seen {
        let positions: Vec<_> = seen
            .iter()
            .enumerate()
            .filter(|(_, (_, outer, _))| outer == group)
            .collect();
        assert_eq!(
            &positions.last().unwrap().1 .2,
            group,
            "outer runs after its inner arguments"
        );
    }
}

#[test]
fn finalized_call_visitor_rejects_duplicate_source_and_changed_finished_function() {
    for mutation in 0..3 {
        let (mut module, mut handoff) =
            crate::mir::builder::lexical_call_projection_finished_artifact_fixture();
        // Source pool corruption is applied to an owned test handoff, never
        // to a second semantic inventory or cloned Taken disposition row.
        if mutation == 0 {
            let source = match &mut handoff {
                crate::mir::finalized_root_handoff::FinalizedRootHandoffV1::Births {
                    root_source: Some(source),
                    ..
                } => source,
                _ => panic!("original source"),
            };
            let groups = source.local_calls.values_mut().next().unwrap();
            groups.push(groups[0].clone());
            let error = source
                .visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
                .unwrap_err();
            assert!(error.contains("group-duplicate"), "{error}");
            continue;
        }
        if mutation == 1 {
            module.functions.clear();
        } else {
            for function in module.functions.values_mut() {
                for block in function.blocks.values_mut() {
                    for instruction in &mut block.instructions {
                        if let MirInstruction::Invoke {
                            operation: InvokeOperation::Call { call, .. },
                            ..
                        } = instruction
                        {
                            call.args.push(ValueId(999));
                        }
                    }
                    if let Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call, .. },
                        ..
                    }) = &mut block.terminator
                    {
                        call.args.push(ValueId(999));
                    }
                }
            }
        }
        let source = handoff.root_source().unwrap();
        let error = source
            .visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
            .unwrap_err();
        assert!(
            error.contains(if mutation == 1 {
                "function-missing"
            } else {
                "producer-missing"
            }),
            "{error}"
        );
    }
}
