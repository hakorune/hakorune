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
