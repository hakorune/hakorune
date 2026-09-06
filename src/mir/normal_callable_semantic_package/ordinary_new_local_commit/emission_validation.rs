//! Validation of recorded physical New emission remains owned by local commit.

use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn validate_new_emissions(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        for (site, row) in self
            .local_commits
            .borrow()
            .iter()
            .filter(|(_, row)| row.binding.owner() == owner)
        {
            match &row.emission {
                NewEmissionProgress::RetainedUnavailable => {}
                NewEmissionProgress::Emitted {
                    result,
                    reclaim,
                    bindings,
                    ..
                } => {
                    if row.initializer != Some(*result) || row.local.is_none() {
                        return Err(freeze("emission-local-result-drift"));
                    }
                    let local = row.local.expect("checked local installation");
                    let mut copies = function
                        .blocks
                        .values()
                        .flat_map(|block| block.all_instructions())
                        .filter(|instruction| {
                            matches!(instruction, MirInstruction::Copy { dst, .. } if *dst == local)
                        });
                    if !matches!(copies.next(), Some(MirInstruction::Copy { src, .. }) if src == result)
                        || copies.next().is_some()
                    {
                        return Err(freeze("emission-local-copy-drift"));
                    }
                    let expected_reclaim = match (&row.birth_target, &row.construction) {
                        (None, Ok(plan)) if plan.constructor().is_none() => None,
                        (Some(_), Ok(plan)) => {
                            let (constructor_source, constructor_owner) = plan
                                .constructor()
                                .ok_or_else(|| freeze("reclaim-origin-constructor-missing"))?;
                            if !plan.reclaims_unpublished_outer_storage()
                                || plan.object() != row.object
                            {
                                return Err(freeze("reclaim-origin-source-drift"));
                            }
                            Some((constructor_source, constructor_owner))
                        }
                        _ => return Err(freeze("reclaim-origin-source-drift")),
                    };
                    match (expected_reclaim, reclaim) {
                        (None, None) => {}
                        (Some((constructor_source, constructor_owner)), Some(emitted)) => {
                            if emitted.origin.site != *site
                                || emitted.origin.object != row.object
                                || !emitted
                                    .origin
                                    .constructor_source
                                    .same_as(constructor_source)
                                || emitted.origin.constructor_owner != *constructor_owner
                            {
                                return Err(freeze("reclaim-origin-source-drift"));
                            }
                            if !matches!(
                                emitted.instruction,
                                MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
                                        object,
                                        value,
                                    },
                                    ..
                                } if object == emitted.origin.object && value == *result
                            ) {
                                return Err(freeze("reclaim-origin-operation-drift"));
                            }
                            let matching = function
                                .blocks
                                .values()
                                .flat_map(|block| block.all_instructions())
                                .filter(|actual| matches!(
                                    actual,
                                    MirInstruction::Invoke {
                                        operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
                                            object,
                                            value,
                                        },
                                        ..
                                    } if *object == emitted.origin.object && *value == *result
                                ))
                                .count();
                            if matching != 1 {
                                return Err(freeze(if matching == 0 {
                                    "reclaim-origin-operation-drift"
                                } else {
                                    "reclaim-origin-duplicate"
                                }));
                            }
                            if !function.blocks.get(&emitted.block).is_some_and(|block| {
                                block
                                    .all_instructions()
                                    .any(|actual| actual == &emitted.instruction)
                            }) {
                                return Err(freeze("reclaim-origin-binding-drift"));
                            }
                        }
                        _ => return Err(freeze("reclaim-origin-presence-drift")),
                    }
                    for (block, expected) in bindings {
                        if !function.blocks.get(block).is_some_and(|block| {
                            block.all_instructions().any(|actual| actual == expected)
                        }) {
                            return Err(freeze("emission-binding-drift"));
                        }
                    }
                }
                _ => return Err(freeze("emission-residual")),
            }
        }
        Ok(())
    }
}
