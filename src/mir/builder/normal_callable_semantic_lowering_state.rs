//! Request-local physical projection for one resolved callable owner.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::mir::builder::stmts::CompletedLocalStatementV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SemanticOwnerSourceKindV1, SourceBindingSiteV1, SourceNodeSiteV1,
};
use crate::mir::ValueId;

use super::control_flow::plan::expression_port::ExactSourceMethodCallV1;
use super::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use super::normal_callable_dynamic_origin::{
    CallableDynamicOriginLoweringStateV1, CurrentDynamicBindingReceiptV1,
    PreparedDynamicOriginRebindV1,
};
use super::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;
use crate::mir::normal_callable_semantic_package::LoopBreakSourcePackageLoanV1;
use crate::mir::normal_callable_semantic_package::SelectedSourceCoreMethodCallV1;

#[path = "normal_callable_construction_state.rs"]
pub(super) mod construction;
#[path = "normal_callable_fault_state.rs"]
mod fault;
#[path = "normal_callable_semantic_receiver_crosswalk.rs"]
mod normal_callable_semantic_receiver_crosswalk;
#[path = "normal_callable_semantic_observation.rs"]
mod observation;

#[path = "normal_callable_semantic_lowering_state/source_prepare.rs"]
mod source_prepare;

#[path = "normal_callable_semantic_lowering_state/loop_recipe_accounting.rs"]
mod loop_recipe_accounting;
/// Loop-carrier physical value publication and `values` projection
/// transactions for speculative branch lanes.
#[path = "normal_callable_semantic_lowering_state/loop_value_publication.rs"]
mod loop_value_publication;
/// Physical values materialized while lowering one callable body.
///
/// Semantic identity remains owned by `VerifiedResolvedFunctionV1`; this state
/// only projects that identity onto the `ValueId`s allocated by existing Lower.
#[path = "normal_callable_semantic_lowering_state/map_local.rs"]
mod map_local;
#[path = "normal_callable_semantic_lowering_state/source_call_publication.rs"]
mod source_call_publication;
#[path = "normal_callable_semantic_lowering_state/source_loop_bridge.rs"]
mod source_loop_bridge;
pub(in crate::mir) use map_local::validate_map_local_annotation;
pub(in crate::mir::builder) use source_loop_bridge::CallableLoopSourceBridgeTakeV1;

#[derive(Debug)]
pub(super) struct CallableSemanticLoweringState {
    fault_frame: Option<crate::mir::builder::function_fault_frame::FunctionFaultFrameV1>,
    construction: construction::ConstructionState,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    receiver: Option<BindingRefV1>,
    parameters: Box<[BindingRefV1]>,
    locals: BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
    initializers: BTreeMap<
        crate::mir::resolved_semantics::SourceBindingSiteV1,
        crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
    >,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    assignments: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    binding_names: BTreeMap<BindingRefV1, Box<str>>,
    explicit_extern_calls: BTreeMap<SourceNodeSiteV1, Box<str>>,
    brand_constructors:
        super::brand_constructor_lowering_projection::BrandConstructorLoweringProjectionV1,
    direct_lambda_captures: BTreeMap<SourceNodeSiteV1, Box<[(Box<str>, BindingRefV1)]>>,
    values: BTreeMap<BindingRefV1, ValueId>,
    dynamic_origins: CallableDynamicOriginLoweringStateV1,
    entry_installed: bool,
    materialized_locals: BTreeSet<SourceNodeSiteV1>,
    consumed_variables: BTreeSet<SourceNodeSiteV1>,
    consumed_assignments: BTreeSet<SourceNodeSiteV1>,
    consumed_direct_lambdas: BTreeSet<SourceNodeSiteV1>,
    consumed_brand_constructors: BTreeSet<SourceNodeSiteV1>,
    consumed_source_core_method_calls: BTreeSet<crate::mir::resolved_semantics::SourceExprSiteV1>,
    source_loop_bridge: Option<source_loop_bridge::CallableLoopSourceBridgeV1>,
    loop_break_source: Option<LoopBreakSourcePackageLoanV1>,
    named_array_writes:
        Vec<crate::mir::normal_callable_semantic_package::NamedArrayWriteEmissionPortV1>,
    /// Birth-side provider sites claimed by field-resident named-array
    /// requirements: `me.<field> = new ArrayBox()` inside this constructor.
    /// Each claimed site must be consumed exactly once by the physical `new`.
    named_array_field_providers: BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        hakorune_mir_defs::CanonicalFieldRefV1,
    >,
    named_array_field_allocations: BTreeSet<crate::mir::resolved_semantics::SourceExprSiteV1>,
    source_core_method_calls:
        BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
    source_static_result_publications: BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        VerifiedStaticCallResultPublicationHandoffV1,
    >,
    consumed_source_static_result_publications:
        BTreeSet<crate::mir::resolved_semantics::SourceExprSiteV1>,
    /// Package-scoped ordinary-new claim ledger borrowed for the active
    /// callable scope. It carries the co-sealed lexical instance-call
    /// dispositions (claim-local and parameter receiver provenance) issued
    /// beside the declared-instance locator. The state only re-lends it;
    /// ownership stays with the invocation scope.
    ordinary_new_claim_ledger:
        Option<Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>>,
}

#[derive(Debug)]
pub(super) struct PreparedCallableDynamicRebindV1 {
    site: SourceNodeSiteV1,
    binding: BindingRefV1,
    result: ValueId,
    consumes_target_read: bool,
    origin: PreparedDynamicOriginRebindV1,
}

impl CallableSemanticLoweringState {
    pub(super) const fn owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(super) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    /// Lend the package-scoped ordinary-new claim ledger to this callable
    /// state for the duration of one source scope. Installed once by the
    /// canonical scope owner; the ledger is shared, never re-issued here.
    pub(super) fn lend_ordinary_new_claim_ledger(
        &mut self,
        ledger: Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    ) {
        self.ordinary_new_claim_ledger = Some(ledger);
    }

    pub(super) fn ordinary_new_claim_ledger(
        &self,
    ) -> Option<&Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>> {
        self.ordinary_new_claim_ledger.as_ref()
    }

    pub(super) fn loop_binding_source_projection(
        &self,
    ) -> super::normal_callable_loop_handoff::CallableLoopSourceProjectionV1<'_> {
        super::normal_callable_loop_handoff::CallableLoopSourceProjectionV1::new(
            self.owner,
            &self.locals,
            &self.variables,
            &self.assignments,
        )
    }

    /// Lend one resolver-issued forest projection to the active source Loop.
    /// The projection is move-only and remains owned by this callable state
    /// until the exact source site consumes it.  Resolver-cataloged sites the
    /// forest projection deliberately left unarmed report `Unarmed`; a site
    /// the bridge never cataloged stays a contract violation.
    pub(super) fn take_source_loop_bridge(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<CallableLoopSourceBridgeTakeV1, String> {
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        match self.source_loop_bridge.as_mut() {
            Some(bridge) => bridge.take_for(&statement_site),
            None => Ok(CallableLoopSourceBridgeTakeV1::BridgeAbsent),
        }
    }

    pub(super) fn source_loop_items(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<Box<[
        crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceItemBindingV1
    ]>>{
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        self.source_loop_bridge
            .as_ref()
            .and_then(|bridge| bridge.source_items_for(&statement_site))
    }

    /// Consume one package-issued LoopBreak candidate by exact source site.
    /// Missing candidates are typed absence for this route; they do not
    /// reclassify an unrelated loop or synthesize a fallback product.
    pub(super) fn take_loop_break_source_candidate(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<Option<crate::mir::builder::VerifiedCallableLoopBreakSourceCandidateV1>, String>
    {
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        self.loop_break_source
            .as_mut()
            .map(|loan| Ok(loan.take_candidate_for_site(&statement_site)))
            .unwrap_or(Ok(None))
    }

    pub(super) fn has_loop_break_source_candidate(&self, site: &SourceNodeSiteV1) -> bool {
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        self.loop_break_source
            .as_ref()
            .map(|loan| loan.has_candidate_for_site(&statement_site))
            .unwrap_or(false)
    }

    /// Consume one composite LoopBreak candidate by its exact source site.
    /// Direct candidates retain their singleton physical contract; a missing
    /// composite row is typed absence and never re-enters the generic route.
    pub(super) fn take_loop_break_composite_source_candidate(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<
        Option<crate::mir::builder::VerifiedCallableLoopBreakCompositeSourceCandidateV1>,
        String,
    > {
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        self.loop_break_source
            .as_mut()
            .map(|loan| Ok(loan.take_composite_candidate_for_site(&statement_site)))
            .unwrap_or(Ok(None))
    }

    pub(super) fn has_loop_break_composite_source_candidate(
        &self,
        site: &SourceNodeSiteV1,
    ) -> bool {
        let statement_site =
            crate::mir::resolved_semantics::SourceStmtSiteV1::from_node(site.clone());
        self.loop_break_source
            .as_ref()
            .map(|loan| loan.has_composite_candidate_for_site(&statement_site))
            .unwrap_or(false)
    }

    /// Return the resolver-issued CoreMethod item rows that cover one exact
    /// loop root. This is an applicability view only; physical consumption
    /// still happens once through `take_source_core_method_call`.
    pub(super) fn source_core_method_items(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Box<
        [crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceItemBindingV1],
    > {
        let Some(items) = self.source_loop_items(site) else {
            return Box::new([]);
        };
        items
            .into_vec()
            .into_iter()
            .filter(|item| self.source_core_method_calls.contains_key(item.call_site()))
            .collect()
    }

    pub(super) fn prepare_source_backed_dynamic_loop_ingress(
        &self,
        schedule: super::normal_callable_loop_handoff::VerifiedCallableSemanticLoopBindingScheduleV1,
        operations: super::normal_callable_dynamic_operation_source::VerifiedDynamicLoopOperationSourceSetV1,
        parent_site: &SourceNodeSiteV1,
        condition_site: &SourceNodeSiteV1,
        body_site: &SourceNodeSiteV1,
    ) -> Result<
        super::normal_callable_dynamic_loop_prepare::PreparedSourceBackedDynamicLoopIngressV1,
        super::normal_callable_dynamic_loop_prepare::DynamicLoopPrepareIssueV1,
    > {
        super::normal_callable_dynamic_loop_prepare::DynamicLoopPrepareIssuerV1::issue(
            schedule,
            operations,
            &self.dynamic_origins,
            parent_site,
            condition_site,
            body_site,
        )
    }

    pub(super) fn install_entry_values(
        &mut self,
        entry: &PreparedCallableEntryValuesV1,
    ) -> Result<(), String> {
        let receiver = entry.receiver();
        let parameters = entry.parameters();
        if self.entry_installed {
            return Err(freeze("duplicate-entry-install"));
        }
        if self.receiver.is_some() != receiver.is_some()
            || self.parameters.len() != parameters.len()
        {
            return Err(freeze("entry-shape-mismatch"));
        }
        if let (Some(binding), Some(value)) = (self.receiver, receiver) {
            self.insert_value(binding, value)?;
        }
        for index in 0..self.parameters.len() {
            self.insert_value(self.parameters[index], parameters[index])?;
        }
        self.dynamic_origins
            .install_entry(&self.parameters, entry)
            .map_err(|error| error.to_string())?;
        self.entry_installed = true;
        Ok(())
    }

    pub(super) fn record_completed_local(
        &mut self,
        site: &SourceNodeSiteV1,
        completed: &CompletedLocalStatementV1,
    ) -> Result<(), String> {
        let bindings = self
            .locals
            .get(site)
            .cloned()
            .ok_or_else(|| freeze("missing-local-site"))?;
        if bindings.len() != completed.bindings().len()
            || !self.materialized_locals.insert(site.clone())
        {
            return Err(freeze("local-materialization-mismatch"));
        }
        for (binding, value) in bindings
            .iter()
            .copied()
            .zip(completed.bindings().iter().map(|row| row.local()))
        {
            self.insert_value(binding, value)?;
        }
        self.dynamic_origins
            .record_local(site, &bindings, completed.bindings())
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    pub(super) fn read_variable(&mut self, site: &SourceNodeSiteV1) -> Result<ValueId, String> {
        let binding = if let Some(binding) = self.variables.get(site).copied() {
            if !self.consumed_variables.insert(site.clone()) {
                return Err(freeze("duplicate-variable-consumption"));
            }
            binding
        } else if let Some(binding) = self.assignments.get(site).copied() {
            // Existing assignment lowering reads its target before publishing
            // the successful write.  The assignment receipt, not a name,
            // authorizes this physical read; rebind() consumes it afterwards.
            binding
        } else {
            return Err(format!(
                "{} site={:?}",
                freeze("missing-variable-site"),
                site.segments()
            ));
        };
        self.value_for_exact_binding(self.owner, binding)
            .map_err(|error| match error {
                normal_callable_semantic_receiver_crosswalk::ExactBindingValueErrorV1::EntryNotInstalled
                | normal_callable_semantic_receiver_crosswalk::ExactBindingValueErrorV1::ValueUnavailable => {
                    freeze("variable-before-materialization")
                }
                other => other.to_string(),
            })
    }

    pub(super) fn source_read_binding(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<BindingRefV1, String> {
        self.variables
            .get(site)
            .copied()
            .or_else(|| self.assignments.get(site).copied())
            .ok_or_else(|| {
                format!(
                    "{} site={:?}",
                    freeze("missing-variable-site"),
                    site.segments()
                )
            })
    }

    pub(super) fn rebind(&mut self, site: &SourceNodeSiteV1, value: ValueId) -> Result<(), String> {
        let binding = self
            .assignments
            .get(site)
            .copied()
            .ok_or_else(|| freeze("missing-assignment-site"))?;
        if !self.consumed_assignments.insert(site.clone()) {
            return Err(freeze("duplicate-assignment-consumption"));
        }
        if let Some(read_binding) = self.variables.get(site).copied() {
            if read_binding != binding || !self.consumed_variables.insert(site.clone()) {
                return Err(freeze("assignment-target-read-mismatch"));
            }
        }
        let previous = self
            .values
            .get(&binding)
            .copied()
            .ok_or_else(|| freeze("rebind-before-materialization"))?;
        self.dynamic_origins
            .invalidate_rebind(binding, previous)
            .map_err(|error| error.to_string())?;
        self.values.insert(binding, value);
        Ok(())
    }

    pub(super) fn prepare_source_backed_dynamic_rebind(
        &self,
        site: &SourceNodeSiteV1,
        expected_binding: BindingRefV1,
        expected_previous: ValueId,
        result: ValueId,
        expected_origin: BindingRefV1,
    ) -> Result<PreparedCallableDynamicRebindV1, String> {
        let binding = self
            .assignments
            .get(site)
            .copied()
            .ok_or_else(|| freeze("missing-assignment-site"))?;
        if binding != expected_binding || self.consumed_assignments.contains(site) {
            return Err(freeze("dynamic-rebind-assignment-mismatch"));
        }
        let consumes_target_read = match self.variables.get(site).copied() {
            Some(read_binding)
                if read_binding == binding && !self.consumed_variables.contains(site) =>
            {
                true
            }
            Some(_) => return Err(freeze("assignment-target-read-mismatch")),
            None => false,
        };
        if self.values.get(&binding).copied() != Some(expected_previous) {
            return Err(freeze("dynamic-rebind-current-mismatch"));
        }
        let origin = self
            .dynamic_origins
            .prepare_current_rebind(binding, expected_previous, result, expected_origin)
            .map_err(|error| error.to_string())?;
        Ok(PreparedCallableDynamicRebindV1 {
            site: site.clone(),
            binding,
            result,
            consumes_target_read,
            origin,
        })
    }

    pub(super) fn commit_source_backed_dynamic_rebind(
        &mut self,
        prepared: PreparedCallableDynamicRebindV1,
    ) -> CurrentDynamicBindingReceiptV1 {
        debug_assert_eq!(
            self.values.get(&prepared.binding),
            Some(
                &self
                    .dynamic_origins
                    .current_binding(prepared.binding)
                    .expect("prepared Dynamic origin")
                    .0
            )
        );
        let inserted_assignment = self.consumed_assignments.insert(prepared.site.clone());
        debug_assert!(inserted_assignment);
        if prepared.consumes_target_read {
            let inserted_read = self.consumed_variables.insert(prepared.site.clone());
            debug_assert!(inserted_read);
        }
        self.values.insert(prepared.binding, prepared.result);
        self.dynamic_origins.commit_current_rebind(prepared.origin)
    }

    pub(super) fn direct_lambda_captures(
        &mut self,
        site: &SourceNodeSiteV1,
    ) -> Result<Vec<(String, ValueId)>, String> {
        let captures = self
            .direct_lambda_captures
            .get(site)
            .ok_or_else(|| freeze("missing-direct-lambda-site"))?;
        if !self.consumed_direct_lambdas.insert(site.clone()) {
            return Err(freeze("duplicate-direct-lambda-consumption"));
        }
        captures
            .iter()
            .map(|(name, binding)| {
                let value = self
                    .values
                    .get(binding)
                    .copied()
                    .ok_or_else(|| freeze("direct-lambda-capture-before-materialization"))?;
                Ok((name.to_string(), value))
            })
            .collect()
    }

    fn insert_value(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.values.insert(binding, value).is_some() {
            return Err(freeze("duplicate-value"));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn install_single_local_for_test(
        &mut self,
        site: &SourceNodeSiteV1,
        binding: BindingRefV1,
        ordinal: u32,
        initializer: ValueId,
        local: ValueId,
    ) -> Result<(), String> {
        if self.locals.get(site).map(|rows| rows.as_ref()) != Some(&[binding])
            || !self.materialized_locals.insert(site.clone())
        {
            return Err(freeze("test-local-shape"));
        }
        self.insert_value(binding, local)?;
        self.dynamic_origins
            .record_local(
                site,
                &[binding],
                &[super::stmts::CompletedLocalBindingV1::new(
                    ordinal,
                    initializer,
                    local,
                )],
            )
            .map_err(|error| error.to_string())
    }
}

fn ordered_bindings(
    bindings: BTreeMap<u32, BindingRefV1>,
    gap: &'static str,
) -> Result<Box<[BindingRefV1]>, String> {
    if bindings.keys().copied().ne(0..bindings.len() as u32) {
        return Err(freeze(gap));
    }
    Ok(bindings.into_values().collect())
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][callable-semantic-lowering/{reason}]")
}

mod named_array;

#[cfg(test)]
#[path = "normal_callable_semantic_lowering_state/named_array_tests.rs"]
mod named_array_tests;
