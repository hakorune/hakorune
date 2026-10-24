//! Implicit body-end loans use the existing source flow and root progress.
use super::*;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitV1;
use std::rc::Rc;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn implicit_root_home_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        completion: &Rc<VerifiedFunctionCompletionV1>,
    ) -> Result<&RootHomeExitV1, String> {
        let canonical = self
            .completion_source_for_owner_v1(owner)
            .ok_or_else(|| freeze("implicit-exit/completion-missing"))?;
        if !Rc::ptr_eq(canonical, completion)
            || completion.owner() != owner
            || completion.returns_value()
            || completion.implicit_body_end().is_none()
        {
            return Err(freeze("implicit-exit/completion-drift"));
        }
        let row = canonical
            .cleanup()
            .root_flow()
            .and_then(|flow| flow.implicit_exit())
            .ok_or_else(|| freeze("implicit-exit/source-missing"))?
            .map_err(|issue| {
                format!(
                    "{} owner={owner:?} reason={issue:?}",
                    freeze("implicit-exit/source-unavailable")
                )
            })?;
        if completion.implicit_body_end() != row.implicit_origin() {
            return Err(freeze("implicit-exit/source-end-drift"));
        }
        Ok(row)
    }

    /// Generic implicit families remain closed. A source-selected ordinary Unit
    /// joins its canonical implicit and explicit exits as one Home obligation.
    pub(in crate::mir::normal_callable_semantic_package) fn root_home_flow_ready_v1(
        &self,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<bool, String> {
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(false);
        };
        if flow.all_exits_ready() {
            return Ok(true);
        }
        let Some(selected) = self.implicit_unit_completion_source_v1(completion.owner())? else {
            return Ok(false);
        };
        if !std::ptr::eq(selected.as_ref(), completion) {
            return Err(freeze("implicit-exit/completion-drift"));
        }
        self.implicit_root_home_source_v1(completion.owner(), &selected)?;
        Ok(completion
            .explicit_sites()
            .iter()
            .all(|site| matches!(flow.exit_row(site), Some(Ok(_)))))
    }

    pub(crate) fn prepare_implicit_root_home_exit_v1(
        &self,
        owner: FunctionOwnerIdV1,
        completion: &Rc<VerifiedFunctionCompletionV1>,
    ) -> Result<(), String> {
        let row = self.implicit_root_home_source_v1(owner, completion)?;
        let order = RootHomeCleanupOrderV1::from_home_end_plans_for_exit(
            &self.local_commits.borrow(),
            &RootHomeExitKeyV1::ImplicitBodyEnd,
            row.homes(),
            row.homes(),
        )?
        .ok_or_else(|| freeze("implicit-exit/end-unavailable"))?;
        let mut exits = self.root_exits.borrow_mut();
        let progress = exits
            .entry((owner, RootHomeExitKeyV1::ImplicitBodyEnd))
            .or_insert(RootHomeExitProgress::Unprepared);
        if !matches!(progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        *progress = RootHomeExitProgress::Prepared(order);
        Ok(())
    }

    /// Source membership, original End order and the common graph verifier all
    /// apply before capture as well as after the finished block projection.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_implicit_root_graph_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
        order: &RootHomeCleanupOrderV1,
        origins: &[RootHomeReleaseEmissionV1],
        bindings: &[(BasicBlockId, MirInstruction)],
        entry: &RootHomeExitEntry,
    ) -> Result<(), String> {
        self.validate_implicit_root_graph_in_context_v1(
            owner,
            function,
            projection,
            order,
            origins,
            bindings,
            entry,
            CallValidationContextV1::Lowering(self),
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_implicit_root_graph_in_context_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
        order: &RootHomeCleanupOrderV1,
        origins: &[RootHomeReleaseEmissionV1],
        bindings: &[(BasicBlockId, MirInstruction)],
        entry: &RootHomeExitEntry,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        self.validate_implicit_root_entry_in_context_v1(
            owner,
            order,
            entry,
            bindings,
            Some(function),
            projection,
            context,
        )?;
        let (completion, _) = order
            .implicit_void
            .as_ref()
            .expect("validated implicit producer");
        let row = self.implicit_root_home_source_v1(owner, completion)?;
        let source_order = RootHomeCleanupOrderV1::from_home_end_plans_for_exit(
            &self.local_commits.borrow(),
            &RootHomeExitKeyV1::ImplicitBodyEnd,
            row.homes(),
            row.homes(),
        )?
        .ok_or_else(|| freeze("implicit-exit/end-unavailable"))?;
        if order.full() != source_order.full()
            || order.normal() != source_order.normal()
            || order.normal()
                != origins
                    .iter()
                    .map(|row| row.origin().clone())
                    .collect::<Vec<_>>()
        {
            return Err(freeze("root-exit-origin-order"));
        }
        for origin in origins {
            if !matches!(&origin.instruction, MirInstruction::Invoke { operation, .. }
                if operation == origin.origin.operation())
                || bindings
                    .iter()
                    .filter(|binding| binding.0 == origin.block && binding.1 == origin.instruction)
                    .count()
                    != 1
            {
                return Err(freeze("root-exit-origin-binding-drift"));
            }
        }
        for (block, instruction) in bindings {
            if !super::super::physical_boundary::check_binding(
                function,
                projection,
                *block,
                instruction,
            )? {
                return Err(freeze("root-exit-binding-drift"));
            }
        }
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
            )?;
        } else {
            super::super::root_cleanup_graph::ordered_structure::validate_original_with_wrapper(
                function,
                bindings,
                entry,
                order.ingress_result_kind(),
                &self.main_arguments_release_suffix_v1(owner, function)?,
            )?;
        }
        Ok(())
    }

    pub(in crate::mir) fn validate_implicit_callable_body_end_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_implicit_root_progress_v1(owner, function, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_implicit_root_progress_v1(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        // Selection follows the same retained original caller source, never MIR.
        let selected = self.implicit_unit_completion_source_v1(owner)?.is_some();
        let exits = self.root_exits.borrow();
        match exits.get(&(owner, RootHomeExitKeyV1::ImplicitBodyEnd)) {
            Some(RootHomeExitProgress::Emitted {
                order,
                origins,
                bindings,
                entry,
            }) => self.validate_implicit_root_graph_v1(
                owner, function, projection, order, origins, bindings, entry,
            ),
            None if !selected => Ok(()),
            _ => Err(freeze("implicit-exit/unconsumed")),
        }
    }

    pub(crate) fn record_implicit_root_home_exit_v1(
        &self,
        owner: FunctionOwnerIdV1,
        completion: Rc<VerifiedFunctionCompletionV1>,
        producer: (BasicBlockId, MirInstruction),
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        self.implicit_root_home_source_v1(owner, &completion)?;
        let exit = RootHomeExitKeyV1::ImplicitBodyEnd;
        let local_bindings = self.select_local_call_binding_groups_for_key_v1(owner, &exit)?;
        {
            let mut exits = self.root_exits.borrow_mut();
            let Some(RootHomeExitProgress::Emitting(order)) = exits.get_mut(&(owner, exit.clone()))
            else {
                return Err(freeze("root-exit-not-emitting"));
            };
            if order.implicit_void.is_some() {
                return Err(freeze("implicit-exit/duplicate-producer"));
            }
            order.implicit_void = Some((completion, producer));
        }
        self.record_root_home_exit_for_key_v1(
            owner,
            &exit,
            origins,
            bindings,
            RootHomeExitEntry::Plain { local_bindings },
        )
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_implicit_root_entry_v1(
        &self,
        owner: FunctionOwnerIdV1,
        order: &RootHomeCleanupOrderV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        function: Option<&MirFunction>,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_implicit_root_entry_in_context_v1(
            owner,
            order,
            entry,
            bindings,
            function,
            projection,
            CallValidationContextV1::Lowering(self),
        )
    }

    fn validate_implicit_root_entry_in_context_v1(
        &self,
        owner: FunctionOwnerIdV1,
        order: &RootHomeCleanupOrderV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        function: Option<&MirFunction>,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let (completion, producer) = order
            .implicit_void
            .as_ref()
            .ok_or_else(|| freeze("implicit-exit/producer-missing"))?;
        self.implicit_root_home_source_v1(owner, completion)?;
        let MirInstruction::Const {
            dst,
            value: crate::mir::ConstValue::Void,
        } = &producer.1
        else {
            return Err(freeze("implicit-exit/producer-kind"));
        };
        let RootHomeExitEntry::Plain { local_bindings } = entry else {
            return Err(freeze("implicit-exit/entry-kind"));
        };
        self.validate_local_call_binding_groups_for_key_v1(
            owner,
            &RootHomeExitKeyV1::ImplicitBodyEnd,
            local_bindings,
            &[],
        )?;
        let returns: Vec<_> = bindings
            .iter()
            .filter_map(|(_, instruction)| match instruction {
                MirInstruction::Return { value } => Some(value),
                _ => None,
            })
            .collect();
        if returns.as_slice() != [&Some(*dst)] {
            return Err(freeze("implicit-exit/return-producer-drift"));
        }
        if let Some(function) = function {
            if !super::super::physical_boundary::check_binding(
                function,
                projection,
                producer.0,
                &producer.1,
            )? {
                return Err(freeze("implicit-exit/producer-actual-drift"));
            }
            for group in local_bindings {
                group.validate_recorded_in_context(context, function, projection)?;
                group.check_physical_bindings_v1(function, projection)?;
            }
        }
        Ok(())
    }
}
