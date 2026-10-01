//! Source-target preparation inside the existing lexical-call owner.
//! Each per-need result retains declaration/source order until final issuance.
use super::*;
use std::collections::BTreeMap;

pub(in crate::mir::normal_callable_semantic_package) type PreparedLexicalInstanceCallSourceTargetsV1 =
    Result<Vec<Result<Option<LexicalInstanceCallSourceTargetV1>, String>>, String>;

pub(in crate::mir::normal_callable_semantic_package) fn prepare_lexical_source_targets_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    new_classes: &BTreeMap<OwnedExprSiteV1, Box<str>>,
    ordinary_box_names: &[Box<str>],
    field_write_claims: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
) -> PreparedLexicalInstanceCallSourceTargetsV1 {
    let source = provenance::LexicalReceiverClassSourceV1::prepared(
        new_classes,
        ordinary_box_names,
        field_write_claims,
        callable_result_classes,
    );
    let mut needs = Vec::new();
    for declaration in batch.declarations() {
        let slot = declaration.batch_slot();
        batch
            .with_lowering_input(slot, |input| {
                let owner = input.owner();
                for (site, call) in input.function().method_calls() {
                    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                        binding,
                    )) = call.receiver()
                    else {
                        continue;
                    };
                    if binding.owner() != owner {
                        continue;
                    }
                    let Some(record) = input.function().binding(binding) else {
                        continue;
                    };
                    let parameter_index = match record.kind() {
                        BindingKindV1::Parameter { index } => Some(index),
                        BindingKindV1::Local { .. } => None,
                        _ => continue,
                    };
                    let rebound = input.function().assignment_targets().any(|(_, target)| {
                        matches!(
                            target,
                            ResolvedAssignmentTargetV1::BindingRebind(rebound)
                                if *rebound == binding
                        )
                    });
                    needs.push(LexicalInstanceCallNeedV1 {
                        owner,
                        callee_slot: slot,
                        call_site: site.clone(),
                        receiver_site: call.receiver_site().clone(),
                        receiver_binding: binding,
                        parameter_index,
                        selector: call.selector().into(),
                        arity: call.arity(),
                        argument_sites: call
                            .arguments()
                            .iter()
                            .map(|argument| argument.site().clone())
                            .collect(),
                        rebound,
                    });
                }
            })
            .map_err(|_| freeze("lexical-instance-call/batch-loan"))?;
    }

    Ok(needs
        .into_iter()
        .map(|need| {
            (|| -> Result<Option<LexicalInstanceCallSourceTargetV1>, String> {
                let class = match need.parameter_index {
                    Some(index) => {
                        match source.prove_parameter_class(batch, selected, &need, index)? {
                            Some(class) => class,
                            None => return Ok(None),
                        }
                    }
                    None => match source.claim_local_class(batch, selected, &need)? {
                        Some(class) => class,
                        None => return Ok(None),
                    },
                };
                // Missing or contradictory evidence leaves the call unarmed:
                // outside an armed loop it keeps the existing dynamic path, and
                // inside one the route coverage names the uncovered site. Only
                // structural corruption (batch loan, duplicate issuance) is a
                // hard freeze.
                if need.rebound {
                    return Ok(None);
                }
                let Some((target, target_batch_slot)) = unique_instance_target(
                    selected,
                    class.as_ref(),
                    need.selector.as_ref(),
                    need.arity,
                ) else {
                    return Ok(None);
                };
                let call_site = OwnedExprSiteV1::new(need.owner, need.call_site.clone());
                let Some(callee_owner) = batch
                    .declarations()
                    .find(|declaration| declaration.batch_slot() == target_batch_slot)
                    .map(|declaration| declaration.owner())
                else {
                    return Ok(None);
                };

                Ok(Some(LexicalInstanceCallSourceTargetV1 {
                    call_site,
                    receiver_site: need.receiver_site,
                    receiver_binding: need.receiver_binding,
                    target,
                    target_batch_slot,
                    callee_owner,
                    argument_sites: need.argument_sites,
                }))
            })()
        })
        .collect())
}
