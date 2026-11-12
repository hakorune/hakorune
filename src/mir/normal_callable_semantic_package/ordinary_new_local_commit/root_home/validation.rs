//! Independent validation of emitted root Home exits.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    pub(in super::super) fn validate_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
        pending: Option<&super::super::PendingSelectedStaticLoopViewV1<'_, '_>>,
    ) -> Result<(), String> {
        self.validate_implicit_root_progress_v1(owner, function, projection)?;
        if !self.object_return_construction_ready_v1(owner)? {
            return Ok(());
        }
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(());
        };
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(());
        };
        if !self.root_home_flow_ready_v1(completion)? {
            return Ok(());
        }
        let exits = self.root_exits.borrow();
        for expected_exit in completion.explicit_sites() {
            let Some(exit_row) = self.normal_exit_projection_v1(owner, expected_exit)? else {
                return Err(format!("{}: owner={owner:?} exit={expected_exit:?} dispositions={} flow_exit={} terminal={:?}",
                    freeze("root-exit-source-missing"), self.normal_return_dispositions.is_some(),
                    flow.exit_row(expected_exit).is_some(), self.terminal_relation_for_owner_at(owner, expected_exit)));
            };
            let expected_homes = exit_row.homes();
            let pending_row = pending.and_then(|pending| pending.root(owner, expected_exit));
            let progress = exits.get(&(owner, expected_exit.clone().into()));
            let (order, origins, bindings, entry) = match (pending_row, progress) {
                (Some(row), None | Some(RootHomeExitProgress::Unprepared)) => {
                    (row.order(), row.origins(), row.bindings(), row.entry())
                }
                (None, Some(RootHomeExitProgress::Unavailable)) => continue,
                (
                    None,
                    Some(RootHomeExitProgress::Emitted {
                        order,
                        origins,
                        bindings,
                        entry,
                    }),
                ) => (order, origins.as_slice(), bindings.as_slice(), entry),
                _ => return Err(freeze("root-exit-unconsumed")),
            };
            {
                if order.normal()
                    != origins
                        .iter()
                        .map(|row| row.origin.clone())
                        .collect::<Vec<_>>()
                {
                    return Err(freeze("root-exit-origin-order"));
                }
                self.validate_call_entry(
                    owner,
                    expected_exit,
                    function,
                    projection,
                    None,
                    pending.map_or(&[][..], |pending| pending.excluded_routes()),
                    entry,
                    bindings,
                )?;
                // Call-argument maps precede the binding origins: they are
                // emitted inside the terminal expression (youngest), in
                // descending argument order. Their expected sequence comes
                // from the sealed terminal Call, never from the MIR.
                let expected_argument_maps = self
                    .call_source_completion_for_owner_at(owner, expected_exit)
                    .filter(|(completion, _)| completion.owner() == owner)
                    .map(|(_, terminal)| {
                        terminal
                            .arguments()
                            .iter()
                            .enumerate()
                            .filter_map(|(ordinal, argument)| {
                                argument
                                    .map_site()
                                    .map(|site| (ordinal as u32, site.clone()))
                            })
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let mut expected_argument_maps = expected_argument_maps;
                expected_argument_maps.sort_by_key(|(ordinal, _)| std::cmp::Reverse(*ordinal));
                if projection
                    .is_some_and(|p| bindings.iter().any(|(id, _)| p.destination(*id).is_none()))
                {
                    return Err(freeze("root-cleanup-graph/residual-node"));
                }
                let mapped = projection.map(|p| p.bindings(bindings)).transpose()?;
                if let (RootHomeExitEntry::Plain { .. }, Some(projection), Some(mapped)) =
                    (entry, projection, mapped.as_ref())
                {
                    if let Some((entry, _)) = bindings.last() {
                        let borrowed = order
                            .borrowed_expression_fault_v1()
                            .map(|(fault, original)| {
                                Ok::<_, String>((
                                    projection.destination(fault).ok_or_else(|| {
                                        freeze("root-cleanup-graph/residual-node")
                                    })?,
                                    projection.bindings(original)?,
                                ))
                            })
                            .transpose()?;
                        super::super::root_cleanup_graph::validate_projected_ingress(
                            function,
                            mapped,
                            projection
                                .destination(*entry)
                                .expect("checked root mapping"),
                            borrowed
                                .as_ref()
                                .map(|(entry, rows)| (*entry, rows.as_slice())),
                        )?;
                    }
                }
                let rows = self.local_commits.borrow();
                let mut expected_subjects: Vec<RootHomeReleaseSubjectV1> = expected_argument_maps
                    .iter()
                    .map(|(ordinal, site)| RootHomeReleaseSubjectV1::ArgumentMap {
                        site: site.clone(),
                        ordinal: *ordinal,
                    })
                    .collect();
                // Each installed Home expands to its sealed teardown
                // plan's subjects: owned `ArrayBox` residences precede
                // the Home's own Binding release.
                let expected_plans = source_home_end_plans_v1(self, &rows, expected_homes)?
                    .ok_or_else(|| freeze("root-home-end-unavailable"))?;
                expected_subjects
                    .extend(expected_plans.iter().map(|(_, subject, _)| subject.clone()));
                if let Some((prior, ends)) = self
                    .loop_return_text_ends
                    .borrow()
                    .get(&(owner, expected_exit.clone().into()))
                {
                    let at = expected_subjects.len() - expected_plans.len()
                        + expected_plans
                            .iter()
                            .position(|(home, _, _)| prior.contains(&home.binding()))
                            .unwrap_or(expected_plans.len());
                    expected_subjects
                        .splice(at..at, ends.iter().map(|origin| origin.subject().clone()));
                }
                if origins.len() != expected_subjects.len() {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[root-home] origin-count exit={expected_exit:?} emitted={:?} expected={expected_subjects:?}",
                                origins.iter().map(|emitted| emitted.origin.subject()).collect::<Vec<_>>());
                    }
                    return Err(freeze("root-exit-origin-count"));
                }
                let expected_subjects = expected_subjects.into_iter();
                for (emitted, expected_subject) in origins.iter().zip(expected_subjects) {
                    if emitted.origin.subject() != &expected_subject
                        || expected_plans
                            .iter()
                            .find(|(_, subject, _)| subject == &expected_subject)
                            .is_some_and(|(_, _, operation)| {
                                emitted.origin.operation() != operation
                            })
                        || emitted.origin.exit() != expected_exit
                    {
                        return Err(freeze("root-exit-origin-drift"));
                    }
                    if !matches!(
                        &emitted.instruction,
                        MirInstruction::Invoke { operation, .. }
                            if operation == emitted.origin.operation()
                    ) {
                        return Err(freeze("root-exit-operation-drift"));
                    }
                    if bindings
                        .iter()
                        .filter(|(id, instruction)| {
                            *id == emitted.block && *instruction == emitted.instruction
                        })
                        .count()
                        != 1
                    {
                        return Err(freeze("root-exit-origin-binding-drift"));
                    }
                    let (actual_block, expected_instruction) = match projection {
                        Some(p) => p
                            .binding(emitted.block, &emitted.instruction)?
                            .ok_or_else(|| freeze("root-exit-origin-binding-drift"))?,
                        None => (emitted.block, emitted.instruction.clone()),
                    };
                    if !function.blocks.get(&actual_block).is_some_and(|block| {
                        block.all_instructions().any(|actual| {
                            matches!(
                                actual,
                                MirInstruction::Invoke { operation, .. }
                                    if operation == emitted.origin.operation()
                            )
                        })
                    }) {
                        return Err(freeze("root-exit-operation-drift"));
                    }
                    if !function.blocks.get(&actual_block).is_some_and(|block| {
                        block
                            .all_instructions()
                            .any(|actual| actual == &expected_instruction)
                    }) {
                        return Err(freeze("root-exit-origin-binding-drift"));
                    }
                }
                for (id, expected) in bindings {
                    if !super::super::physical_boundary::check_binding(
                        function, projection, *id, expected,
                    )? {
                        return Err(freeze("root-exit-binding-drift"));
                    }
                }
                order.validate_direct_entry(
                    self,
                    owner,
                    expected_exit,
                    entry,
                    bindings,
                    projection,
                )?;
                order.validate_null_return(entry, bindings, projection, projection)?;
                if let Some(producers) = self.validate_return_expression_producer_v1(
                    CallValidationContextV1::Lowering(self),
                    owner,
                    expected_exit,
                    order,
                    entry,
                    bindings,
                    projection,
                    projection,
                    Some(function),
                )? {
                    for producer in producers {
                        if !super::super::physical_boundary::check_binding(
                            function,
                            None,
                            producer.0,
                            &producer.1,
                        )? {
                            return Err(freeze("root-exit-binding-drift"));
                        }
                    }
                }
                if let Some(producer) = self.validate_received_return_producer_v1(
                    owner,
                    expected_exit,
                    entry,
                    bindings,
                    projection,
                    projection,
                )? {
                    // Projection precedes validate_complete's registration.
                    // Final collector checks mandatory membership after that
                    // existing owner has installed the recorded bindings.
                    if !super::super::physical_boundary::check_binding(
                        function,
                        None,
                        producer.0,
                        &producer.1,
                    )? {
                        return Err(freeze("received-return/producer-actual-drift"));
                    }
                }
                self.validate_fresh_return_producer_v1(
                    owner,
                    expected_exit,
                    entry,
                    bindings,
                    projection,
                    projection,
                    Some(function),
                )?;
                super::super::root_cleanup_graph::ordered_paths::validate(
                    function, bindings, entry, order, projection,
                )?;
                if let Some(projection) = projection {
                    super::super::root_cleanup_graph::ordered_structure::validate_projected(
                        function,
                        bindings,
                        entry,
                        projection,
                        order.ingress_result_kind(),
                        order.borrowed_expression_fault_v1(),
                    )?;
                }
                let _ = mapped;
            }
        }
        Ok(())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Every recorded lifecycle local-call binding group must be claimed by
    /// at least one emitted exit entry, or sit on paths whose exits are all
    /// retained `Unavailable`. A recorded-but-never-claimed site is physical
    /// evidence nobody consumed — a violation, not a benign leftover.
    pub(in crate::mir::normal_callable_semantic_package) fn local_call_bindings_consumed(
        &self,
    ) -> bool {
        let pending = self.root_local_call_bindings.borrow();
        let exits = self.root_exits.borrow();
        pending.iter().all(|(owner, groups)| {
            let _source_scope = self.branch_source_scope_v1(*owner);
            groups.iter().all(|group| {
                let site = group.site();
                let Some(completion) = self.completion_for_owner(*owner) else {
                    return false;
                };
                let Some(flow) = completion.cleanup().root_flow() else {
                    return false;
                };
                let expected = match self.expected_local_call_binding_sources(*owner) {
                    Ok(expected) => expected,
                    Err(_) => return false,
                };
                let Some(source) = expected.iter().find(|source| group.matches_source(source))
                else {
                    return false;
                };
                let exits_to_check = completion
                    .explicit_sites()
                    .iter()
                    .cloned()
                    .map(RootHomeExitKeyV1::Explicit)
                    .chain(
                        completion
                            .implicit_body_end()
                            .map(|_| RootHomeExitKeyV1::ImplicitBodyEnd),
                    )
                    .collect::<Vec<_>>();
                let consumed = exits_to_check.iter().any(|exit| {
                    let covers = match &exit {
                        RootHomeExitKeyV1::Explicit(site) => flow.exit_row(site),
                        RootHomeExitKeyV1::ImplicitBodyEnd => flow.implicit_exit(),
                    }
                    .and_then(|row| row.ok())
                    .is_some_and(|row| source.covered_by(row));
                    if !covers {
                        return false;
                    }
                    match exits.get(&(*owner, exit.clone())) {
                        Some(RootHomeExitProgress::Emitted { entry, .. }) => entry
                            .local_call_groups()
                            .iter()
                            .any(|group| group.site() == site),
                        Some(RootHomeExitProgress::Unavailable)
                        | Some(RootHomeExitProgress::Finalized(_)) => true,
                        _ => false,
                    }
                });
                consumed
            })
        })
    }
}
