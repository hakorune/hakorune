//! Source-target preparation inside the existing lexical-call owner.
//! Each per-need result retains declaration/source order until final issuance.
use super::*;
use std::collections::BTreeMap;

pub(in crate::mir::normal_callable_semantic_package) type PreparedLexicalInstanceCallSourceTargetsV1 =
    Result<Vec<Result<Option<LexicalInstanceCallSourceTargetV1>, String>>, String>;

/// Passive receiver source facts. This is neither an owned-child lease nor inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::normal_callable_semantic_package) struct StoredReceiverSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) parent_binding: BindingRefV1,
    pub(in crate::mir::normal_callable_semantic_package) parent_site: SourceExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package) parent_class: Box<str>,
    pub(in crate::mir::normal_callable_semantic_package) field_name: Box<str>,
    pub(in crate::mir::normal_callable_semantic_package) child_class: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CallTargetReferenceV1 {
    pub(super) call_site: OwnedExprSiteV1,
    pub(super) receiver_site: SourceExprSiteV1,
    pub(super) target: CanonicalSameModuleCallableKeyV1,
    pub(super) target_batch_slot: u32,
    pub(super) callee_owner: FunctionOwnerIdV1,
    pub(super) argument_sites: Box<[SourceExprSiteV1]>,
}

impl CallTargetReferenceV1 {
    pub(super) fn from_target(row: &LexicalInstanceCallSourceTargetV1) -> Self {
        Self {
            call_site: row.call_site.clone(), receiver_site: row.receiver_site.clone(),
            target: row.target.clone(), target_batch_slot: row.target_batch_slot,
            callee_owner: row.callee_owner, argument_sites: row.argument_sites.clone(),
        }
    }
}

#[derive(Debug)]
pub(super) enum PreparedSourceCallNeedV1 {
    Lexical(LexicalInstanceCallSourceTargetV1),
    Stored { reference: CallTargetReferenceV1, receiver: StoredReceiverSourceV1 },
}

impl PreparedSourceCallNeedV1 {
    pub(super) fn reference(&self) -> CallTargetReferenceV1 {
        match self {
            Self::Lexical(row) => CallTargetReferenceV1::from_target(row),
            Self::Stored { reference, .. } => reference.clone(),
        }
    }
    pub(super) fn stored(&self) -> Option<&StoredReceiverSourceV1> {
        match self { Self::Stored { receiver, .. } => Some(receiver), _ => None }
    }
}

pub(super) type PreparedSourceNeedsV1 =
    Result<Vec<Result<Option<PreparedSourceCallNeedV1>, String>>, String>;

enum SourceReceiverNeedV1 {
    Lexical(LexicalInstanceCallNeedV1),
    Stored(Result<Option<PreparedSourceCallNeedV1>, String>),
}

pub(in crate::mir::normal_callable_semantic_package) fn prepare_lexical_source_targets_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    new_classes: &BTreeMap<OwnedExprSiteV1, Box<str>>,
    ordinary_box_names: &[Box<str>],
    field_write_claims: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    stored_receiver: &mut impl FnMut(
        u32,
        crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ) -> Result<
        Option<StoredReceiverSourceV1>,
        String,
    >,
) -> PreparedSourceNeedsV1 {
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
                        let row = stored_receiver(slot, input, call).map(|proof| {
                            let receiver = proof?;
                            let (target, target_batch_slot) = unique_instance_target(
                                selected,
                                receiver.child_class.as_ref(),
                                call.selector(),
                                call.arity(),
                            )?;
                            let callee_owner = batch
                                .declarations()
                                .find(|row| row.batch_slot() == target_batch_slot)?
                                .owner();
                            Some(PreparedSourceCallNeedV1::Stored {
                              reference: CallTargetReferenceV1 {
                                call_site: OwnedExprSiteV1::new(owner, site.clone()),
                                receiver_site: call.receiver_site().clone(),
                                target,
                                target_batch_slot,
                                callee_owner,
                                argument_sites: call
                                    .arguments()
                                    .iter()
                                    .map(|argument| argument.site().clone())
                                    .collect(),
                              }, receiver,
                            })
                        });
                        needs.push(SourceReceiverNeedV1::Stored(row));
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
                    needs.push(SourceReceiverNeedV1::Lexical(LexicalInstanceCallNeedV1 {
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
                    }));
                }
            })
            .map_err(|_| freeze("lexical-instance-call/batch-loan"))?;
    }

    Ok(needs
        .into_iter()
        .map(|need| {
            let SourceReceiverNeedV1::Lexical(need) = need else {
                let SourceReceiverNeedV1::Stored(row) = need else {
                    unreachable!()
                };
                return row;
            };
            (|| -> Result<Option<PreparedSourceCallNeedV1>, String> {
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

                Ok(Some(PreparedSourceCallNeedV1::Lexical(LexicalInstanceCallSourceTargetV1 {
                    call_site,
                    receiver_site: need.receiver_site,
                    receiver: LexicalInstanceCallReceiverV1::Lexical(need.receiver_binding),
                    target,
                    target_batch_slot,
                    callee_owner,
                    argument_sites: need.argument_sites,
                })))
            })()
        })
        .collect())
}
