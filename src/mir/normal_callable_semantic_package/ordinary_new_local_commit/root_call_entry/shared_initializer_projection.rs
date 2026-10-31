//! Shared (`Shared(move x)`) Local initializer: source selection and local-commit validation.
use super::*;
use crate::mir::instruction::StringInvokeOperand as Operand;
use crate::mir::normal_callable_semantic_package::CallValidationContextV1;
use crate::mir::normal_callable_semantic_package::EvaluatedExpressionValueV1;
use crate::mir::resolved_semantics::home_new_prefix::CheckedInitializerExpressionFrameV1;
use std::rc::Rc;

type Binding = (BasicBlockId, MirInstruction);

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_shared_initializer_bindings_v1(
        &self,
        source: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        payload_source: crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1,
        prior_homes: Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>,
        descriptor: crate::mir::instruction::SharedDescriptorV1,
        payload: ValueId,
        local: ValueId,
        bindings: Vec<Binding>,
        function: &MirFunction,
    ) -> Result<PreparedLifecyclePrefixBindingGroupV1<'_>, String> {
        let initializer = source.root().initializer().ok_or_else(|| freeze("shared-initializer/source-missing"))?;
        let packet = Rc::new(SharedLocalCommitV1 {
            binding: initializer.binding(), declaration: initializer.declaration_site().clone(),
            source, payload_source, prior_homes, descriptor, payload, local, bindings,
        });
        packet.validate_shared_v1(self, function, None)?;
        self.prepare_local_call_binding_group(packet.binding.owner(),
            RootLifecyclePrefixBindingGroupV1::new_shared_initializer(packet)?)
    }

    /// Borrow the already installed payload and its existing complete End;
    /// neither source spelling nor the handle bits can invent a descriptor.
    pub(crate) fn prepare_shared_move_payload_v1(
        &self, from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: &OwnedExprSiteV1,
        prior: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    ) -> Result<(crate::mir::instruction::SharedDescriptorV1, ValueId, Vec<InvokeOperation>), String> {
        let mut matching = prior.iter().filter(|home| home.binding() == from);
        let home = matching.next().ok_or_else(|| freeze("shared-initializer/prior-home-missing"))?;
        if matching.next().is_some() { return Err(freeze("shared-initializer/prior-home-duplicate")); }
        let (descriptor, payload) = {
            let rows = self.local_commits.borrow();
            let row = installed_source_home(&rows, home).map_err(|_| freeze("shared-initializer/payload-home-missing"))?;
            if !row.end_available() || rows.get(acquisition).is_none_or(|original| !std::ptr::eq(original, row)) {
                return Err(freeze("shared-initializer/payload-source-drift"));
            }
            let object = row.ordinary_object().ok_or_else(|| freeze("shared-initializer/payload-not-ordinary"))?;
            let payload = row.local().ok_or_else(|| freeze("shared-initializer/payload-not-installed"))?;
            if !matches!(row.end_operations().as_slice(), [InvokeOperation::HomeRelease { object: actual, value }] if *actual == object && *value == payload) {
                return Err(freeze("shared-initializer/payload-end-not-supported"));
            }
            (crate::mir::instruction::SharedDescriptorV1 { payload: object,
                end: vec![crate::mir::instruction::ObjectEndStepV1::ReleaseOuter].into_boxed_slice() }, payload)
        };
        Ok((descriptor, payload, self.prior_home_unwind_for(prior)?))
    }

    pub(crate) fn prepare_shared_clone_handle_v1(
        &self, from: crate::mir::resolved_semantics::BindingRefV1, acquisition: &OwnedExprSiteV1,
        prior: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    ) -> Result<(crate::mir::instruction::SharedDescriptorV1, ValueId, Vec<InvokeOperation>), String> {
        let mut matching = prior.iter().filter(|home| home.binding() == from);
        let home = matching.next().ok_or_else(|| freeze("shared-clone/prior-home-missing"))?;
        if matching.next().is_some() { return Err(freeze("shared-clone/prior-home-duplicate")); }
        let (descriptor, handle) = {
            let rows = self.local_commits.borrow();
            let row = installed_source_home(&rows, home).map_err(|_| freeze("shared-clone/handle-home-missing"))?;
            if rows.get(acquisition).is_none_or(|original| !std::ptr::eq(original, row)) {
                return Err(freeze("shared-clone/handle-source-drift"));
            }
            let LocalCommitV1::Shared(shared) = row else { return Err(freeze("shared-clone/handle-not-shared")); };
            (shared.descriptor.clone(), shared.local)
        };
        Ok((descriptor, handle, self.prior_home_unwind_for(prior)?))
    }

    /// Borrow the completed original New result and reuse its existing End
    /// plan on SharedNew Fault, before unwinding the preceding Homes.
    pub(crate) fn prepare_shared_fresh_payload_v1(
        &self, site: &OwnedExprSiteV1, shared_site: &OwnedExprSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        prior: &[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1],
    ) -> Result<(crate::mir::instruction::SharedDescriptorV1, ValueId, Vec<InvokeOperation>), String> {
        use crate::mir::resolved_semantics::home_new_prefix::ResultNewPurposeV1;
        let (descriptor, payload, end) = {
            let rows = self.local_commits.borrow();
            let Some(LocalCommitV1::Result(row)) = rows.get(site) else { return Err(freeze("shared-fresh/construction-missing")); };
            if row.home_prefix.as_ref().ok().map(|prefix| prefix.purpose()) !=
                Some(&ResultNewPurposeV1::SharedPayload { shared_site: shared_site.clone(), binding })
                || !matches!(&row.construction, Ok(plan) if plan.object() == row.object)
                || row.argument_rows.is_err() || row.destruction != crate::mir::function::ObjectDestructionDispositionV1::NUMERIC_NO_HOOK {
                return Err(freeze("shared-fresh/source-drift"));
            }
            let NewEmissionProgress::Emitted { result, progress, .. } = &row.emission else { return Err(freeze("shared-fresh/not-emitted")); };
            if !matches!(progress, EmittedLocalProgress::ExpressionCompleted | EmittedLocalProgress::Checked { .. }) {
                return Err(freeze("shared-fresh/not-completed"));
            }
            let end: Vec<_> = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::object_end::operations_v1(row.object, false, &[], *result).map(|(_, op)| op).collect();
            if !matches!(end.as_slice(), [InvokeOperation::HomeRelease { object, value }] if *object == row.object && *value == *result) {
                return Err(freeze("shared-fresh/end-unsupported"));
            }
            (crate::mir::instruction::SharedDescriptorV1 { payload: row.object,
                end: vec![crate::mir::instruction::ObjectEndStepV1::ReleaseOuter].into_boxed_slice() }, *result, end)
        };
        let mut unwind = end;
        unwind.extend(self.prior_home_unwind_for(prior)?);
        Ok((descriptor, payload, unwind))
    }

    pub(crate) fn shared_fresh_initializer_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<(Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, OwnedExprSiteV1, Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>)>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None); };
        let mut rows = flow.expression_plans().iter().filter_map(|plan| plan.shared_fresh_initializer_v1())
            .filter(|(source, _, _)| source.root().statement().node() == statement);
        let row = rows.next().map(|(source, payload, prior)| (Rc::clone(source), payload.clone(), prior.into()));
        if rows.next().is_some() || row.as_ref().is_some_and(|(source, payload, _)| source.owner() != owner || payload.owner() != owner) {
            return Err(freeze("shared-fresh/source-owner-or-duplicate"));
        }
        Ok(row)
    }

    pub(crate) fn shared_clone_initializer_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<(Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, crate::mir::resolved_semantics::BindingRefV1, OwnedExprSiteV1, Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>)>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else { return Ok(None); };
        let mut rows = flow.expression_plans().iter().filter_map(|plan| plan.shared_clone_initializer_v1())
            .filter(|(source, _, _, _)| source.root().statement().node() == statement);
        let row = rows.next().map(|(source, from, acquisition, prior)| (Rc::clone(source), from, acquisition.clone(), prior.into()));
        if rows.next().is_some() || row.as_ref().is_some_and(|(source, from, acquisition, _)|
            source.owner() != owner || from.owner() != owner || acquisition.owner() != owner) {
            return Err(freeze("shared-clone/source-owner-or-duplicate"));
        }
        Ok(row)
    }

    /// Retain the exact Shared expression and its pre-New Fault Homes; this
    /// projection does not itself authorize a physical Shared operation.
    pub(crate) fn shared_move_initializer_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
    ) -> Result<Option<(Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, crate::mir::resolved_semantics::BindingRefV1, OwnedExprSiteV1, Box<[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1]>)>, String> {
        let Some(flow) = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) else {
            return Ok(None);
        };
        let mut rows = flow.expression_plans().iter()
            .filter_map(|plan| plan.shared_move_initializer_v1())
            .filter(|(source, _, _, _)| source.root().statement().node() == statement);
        let row = rows.next().map(|(source, from, acquisition, prior)| (Rc::clone(source), from, acquisition.clone(), prior.into()));
        if rows.next().is_some() || row.as_ref().is_some_and(|(source, from, acquisition, _)|
            source.owner() != owner || from.owner() != owner || acquisition.owner() != owner) {
            return Err(freeze("shared-initializer/source-owner-or-duplicate"));
        }
        Ok(row)
    }
}

impl SharedLocalCommitV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_shared_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
        projection: Option<&super::super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1;
        let (descriptor, payload, unwind, is_clone) = match &self.payload_source {
            SharedPayloadSourceV1::Existing { from, acquisition, is_clone } => {
                let selected = if *is_clone {
                    ledger.shared_clone_initializer_v1(self.binding.owner(), self.source.root().statement().node())?
                } else { ledger.shared_move_initializer_v1(self.binding.owner(), self.source.root().statement().node())? };
                let (source, actual_from, actual_acquisition, prior) = selected.ok_or_else(|| freeze("shared-initializer/source-missing"))?;
                if !Rc::ptr_eq(&source, &self.source) || actual_from != *from || actual_acquisition != *acquisition || prior != self.prior_homes {
                    return Err(freeze("shared-initializer/source-drift"));
                }
                let (descriptor, payload, unwind) = if *is_clone {
                    ledger.prepare_shared_clone_handle_v1(*from, acquisition, &prior)?
                } else { ledger.prepare_shared_move_payload_v1(*from, acquisition, &prior)? };
                (descriptor, payload, unwind, *is_clone)
            }
            SharedPayloadSourceV1::Fresh { site } => {
                let (source, payload_site, prior) = ledger.shared_fresh_initializer_v1(self.binding.owner(), self.source.root().statement().node())?
                    .ok_or_else(|| freeze("shared-fresh/source-missing"))?;
                if !Rc::ptr_eq(&source, &self.source) || &payload_site != site || prior != self.prior_homes {
                    return Err(freeze("shared-fresh/source-drift"));
                }
                let shared_site = OwnedExprSiteV1::new(self.binding.owner(), source.nodes().first().ok_or_else(|| freeze("shared-fresh/source-missing"))?.site().clone());
                let (descriptor, payload, unwind) = ledger.prepare_shared_fresh_payload_v1(site, &shared_site, self.binding, &prior)?;
                (descriptor, payload, unwind, false)
            }
        };
        if self.source.root().initializer().is_none_or(|row| row.binding() != self.binding || row.declaration_site() != &self.declaration) {
            return Err(freeze("shared-initializer/source-drift"));
        }
        if descriptor != self.descriptor || payload != self.payload {
            return Err(freeze("shared-initializer/payload-drift"));
        }
        let mut invocations = self.bindings.iter().filter(|(_, row)| matches!(row,
            MirInstruction::Invoke { operation: InvokeOperation::Shared(crate::mir::instruction::SharedInvokeOperation::New { .. } | crate::mir::instruction::SharedInvokeOperation::Clone { .. }), .. }));
        let invoke = invocations.next().ok_or_else(|| freeze("shared-initializer/invoke-missing"))?;
        if invocations.next().is_some() { return Err(freeze("shared-initializer/invoke-duplicate")); }
        let mapped = match projection {
            Some(projection) => projection.binding(invoke.0, &invoke.1)?.ok_or_else(|| freeze("shared-initializer/invoke-removed"))?,
            None => invoke.clone(),
        };
        let MirInstruction::Invoke { operation: InvokeOperation::Shared(operation), fault_frame, normal_landing, fault_landing } = &mapped.1 else {
            return Err(freeze("shared-initializer/invoke-drift"));
        };
        let exact = match operation {
            crate::mir::instruction::SharedInvokeOperation::New { descriptor: actual, payload: value } if !is_clone => actual == &descriptor && *value == payload,
            crate::mir::instruction::SharedInvokeOperation::Clone { descriptor: actual, handle } if is_clone => actual == &descriptor && *handle == payload,
            _ => false,
        };
        if !exact || normal_landing == fault_landing {
            return Err(freeze("shared-initializer/invoke-drift"));
        }
        let normal = function.blocks.get(normal_landing).ok_or_else(|| freeze("shared-initializer/normal-missing"))?;
        if normal.instructions.iter().filter(|row| matches!(row,
            MirInstruction::InvokeNormalResult { invoke_block, dst } if *invoke_block == mapped.0 && *dst == self.local)).count() != 1 {
            return Err(freeze("shared-initializer/normal-result"));
        }
        super::super::root_cleanup_graph::ordered_paths::pending_operations(function, *fault_landing, &unwind, Some(*fault_frame))?;
        Ok(())
    }
}
