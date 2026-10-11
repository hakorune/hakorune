//! Original selected batch checks before collector or ledger publication.
use super::*;
use crate::mir::instruction::InvokeCallResultKind;

impl PreparedSelectedStaticLoopEntryV1<'_, '_> {
    pub(in crate::mir::builder) fn check_unpublished_mutations_for_test(
        self,
        claims: &OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        let Self {
            pending,
            entry_group,
            body_retention,
            header_route,
            body_route,
            tail_route,
            publication,
            early_root_pending,
            tail_root_pending,
            ..
        } = self;
        let owner = entry_group.source_site().owner();
        let routes = [
            header_route.site().clone(),
            body_route.site().clone(),
            tail_route.site().clone(),
        ];
        let view = PendingSelectedStaticLoopViewV1::new(
            &entry_group,
            body_retention.packet_for_test(),
            [&early_root_pending, &tail_root_pending],
            &routes,
        );
        let unpublished = || {
            assert_eq!(claims.selected_static_entry_group_count_for_test(), 0);
            assert_eq!(claims.emitted_root_exit_count_for_test(), 0);
            assert_eq!(claims.selected_loop_body_packet_count_for_test(), 0);
            assert_eq!(claims.child_physical_validation_count_for_test(), 0);
        };
        unpublished();
        pending.complete_before_restore(|function| {
            // Reborrow the one production validator; successful preparation is
            // a pending receipt, never a committed state.
            drop(claims.validate_selected_static_loop_child_emissions(owner, &function, &view)?);
            unpublished();
            let reject = |changed: &MirFunction,
                          candidate: &PendingSelectedStaticLoopViewV1<'_, '_>,
                          label: &str| {
                let error = match claims
                    .validate_selected_static_loop_child_emissions(owner, changed, candidate)
                {
                    Ok(_) => panic!("{label}: malformed pending batch admitted"),
                    Err(error) => error,
                };
                assert!(error.contains("[freeze:contract]"), "{label}: {error}");
                unpublished();
            };
            let returns: Vec<_> = function
                .blocks
                .iter()
                .filter_map(|(id, block)| match block.terminator.as_ref() {
                    Some(MirInstruction::Return { value: Some(value) }) => Some((*id, *value)),
                    _ => None,
                })
                .collect();
            assert_eq!(returns.len(), 2);
            for (index, (block, _)) in returns.iter().enumerate() {
                for replacement in [
                    None,
                    Some(MirInstruction::Return {
                        value: Some(returns[1 - index].1),
                    }),
                ] {
                    let mut changed = function.clone();
                    changed.blocks.get_mut(block).unwrap().terminator = replacement;
                    reject(&changed, &view, "pending Return missing/foreign");
                }
            }
            let calls: Vec<_> = function
                .blocks
                .iter()
                .filter_map(|(id, block)| match block.terminator.as_ref() {
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call, .. },
                        ..
                    }) => Some((*id, call)),
                    _ => None,
                })
                .collect();
            let (_, _, original_calls) = publication.into_parts();
            let original_tail = original_calls[3].1.canonical_global_target_v1()?;
            let expected_callee =
                crate::mir::definitions::MirCall::global(None, original_tail, vec![]).callee;
            let tail_calls: Vec<_> = calls
                .iter()
                .filter(|(_, call)| call.callee == expected_callee)
                .collect();
            assert_eq!(tail_calls.len(), 1, "one original source tail target");
            let tail = tail_calls[0].0;
            let other_target = calls
                .iter()
                .find(|(_, call)| !call.args.is_empty())
                .expect("original prior call")
                .1
                .callee
                .clone();
            for wrong_target in [true, false] {
                let mut changed = function.clone();
                let MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, result },
                    ..
                } = changed
                    .blocks
                    .get_mut(&tail)
                    .unwrap()
                    .terminator
                    .as_mut()
                    .unwrap()
                else {
                    unreachable!()
                };
                if wrong_target {
                    call.callee = other_target.clone();
                } else {
                    *result = InvokeCallResultKind::Unit;
                }
                reject(&changed, &view, "pending tail target/result drift");
            }
            for missing in 0..routes.len() {
                let incomplete: Vec<_> = routes
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| *index != missing)
                    .map(|(_, site)| site.clone())
                    .collect();
                let changed_view = PendingSelectedStaticLoopViewV1::new(
                    &entry_group,
                    body_retention.packet_for_test(),
                    [&early_root_pending, &tail_root_pending],
                    &incomplete,
                );
                reject(&function, &changed_view, "pending route exclusion missing");
            }
            unpublished();
            Ok(())
        })
    }
}
