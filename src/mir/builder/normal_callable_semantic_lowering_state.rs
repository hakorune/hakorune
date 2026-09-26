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

#[path = "normal_callable_semantic_lowering_state/loop_recipe_accounting.rs"]
mod loop_recipe_accounting;
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
    source_core_method_calls:
        BTreeMap<crate::mir::resolved_semantics::SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
    source_static_result_publications: BTreeMap<
        crate::mir::resolved_semantics::SourceExprSiteV1,
        VerifiedStaticCallResultPublicationHandoffV1,
    >,
    consumed_source_static_result_publications:
        BTreeSet<crate::mir::resolved_semantics::SourceExprSiteV1>,
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

    pub(super) fn from_exact_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source(input, None)
    }

    pub(super) fn from_exact_source_with_dynamic_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source_and_core_methods(
            input,
            dynamic_source,
            BTreeMap::new(),
        )
    }

    pub(super) fn from_exact_source_with_dynamic_source_and_core_methods(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
        source_core_method_calls: BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            SelectedSourceCoreMethodCallV1,
        >,
    ) -> Result<Self, String> {
        Self::from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
            input,
            dynamic_source,
            source_core_method_calls,
            None,
        )
    }

    pub(super) fn from_exact_source_with_dynamic_source_and_core_methods_and_loop_break_source(
        input: ResolvedFunctionLoweringInputV1<'_>,
        dynamic_source: Option<
            Rc<super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1>,
        >,
        source_core_method_calls: BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            SelectedSourceCoreMethodCallV1,
        >,
        loop_break_source: Option<LoopBreakSourcePackageLoanV1>,
    ) -> Result<Self, String> {
        let source_loop_bridge = source_loop_bridge::CallableLoopSourceBridgeV1::from_input(input)?;
        let dynamic_origins = match dynamic_source {
            Some(source) => CallableDynamicOriginLoweringStateV1::from_shared_source(source),
            None => {
                let source = SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input)
                    .map_err(|error| {
                        format!("[freeze:contract][callable-dynamic-source] {error:?}")
                    })?;
                CallableDynamicOriginLoweringStateV1::from_source(source)
            }
        }
        .map_err(|error| error.to_string())?;
        let forest = input.forest();
        let [root] = forest.roots() else {
            return Err(freeze("root-cardinality"));
        };
        let owner = forest.owner(*root).ok_or_else(|| freeze("root-owner"))?;
        let owner_id = owner.owner();
        let brand_constructors = super::brand_constructor_lowering_projection::BrandConstructorLoweringProjectionV1::from_verified_owner(
            owner_id,
            input.function().expression_sites(),
            input.function().brand_call_relations(),
        )
        .map_err(|error| format!("[freeze:contract][callable-brand-projection] {error:?}"))?;
        if dynamic_origins.owner() != owner_id {
            return Err(freeze("dynamic-origin-owner"));
        }
        let mut receiver = None;
        let mut parameters = BTreeMap::new();
        let mut locals = BTreeMap::<_, BTreeMap<_, _>>::new();
        let mut declared = BTreeSet::new();
        let mut binding_names = BTreeMap::new();

        for site in owner.declaration_sites() {
            let Some(binding) = owner.declaration_binding(site) else {
                return Err(freeze("missing-declaration-binding"));
            };
            let name = owner
                .binding(binding)
                .ok_or_else(|| freeze("missing-binding-record"))?
                .diagnostic_name();
            binding_names.insert(binding, Box::<str>::from(name));
            if binding.owner() != owner_id || !declared.insert(binding) {
                return Err(freeze("foreign-or-duplicate-declaration"));
            }
            match site {
                SourceBindingSiteV1::Receiver => {
                    if receiver.replace(binding).is_some() {
                        return Err(freeze("duplicate-receiver"));
                    }
                }
                SourceBindingSiteV1::Parameter { index } => {
                    if parameters.insert(*index, binding).is_some() {
                        return Err(freeze("duplicate-parameter"));
                    }
                }
                SourceBindingSiteV1::Local { statement, ordinal } => {
                    if locals
                        .entry(statement.node().clone())
                        .or_default()
                        .insert(*ordinal, binding)
                        .is_some()
                    {
                        return Err(freeze("duplicate-local"));
                    }
                }
                _ => {}
            }
        }

        let parameters = ordered_bindings(parameters, "parameter-ordinal-gap")?;
        let locals = locals
            .into_iter()
            .map(|(site, bindings)| {
                ordered_bindings(bindings, "local-ordinal-gap").map(|bindings| (site, bindings))
            })
            .collect::<Result<_, _>>()?;

        let mut variables = BTreeMap::new();
        for (site, reference) in owner.variable_refs() {
            let ResolvedLexicalRefV1::Local(binding) = reference else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-variable-binding"));
            }
            if variables.insert(site.node().clone(), *binding).is_some() {
                return Err(freeze("duplicate-variable-site"));
            }
        }

        let mut assignments = BTreeMap::new();
        let explicit_extern_calls = owner
            .explicit_extern_calls()
            .map(|(site, call)| (site.node().clone(), call.symbol().into()))
            .collect();
        for (site, target) in owner.assignment_targets() {
            let ResolvedAssignmentTargetV1::BindingRebind(binding) = target else {
                continue;
            };
            if binding.owner() != owner_id {
                return Err(freeze("foreign-assignment-binding"));
            }
            if assignments.insert(site.node().clone(), *binding).is_some() {
                return Err(freeze("duplicate-assignment-site"));
            }
        }

        let mut direct_lambda_captures = BTreeMap::new();
        for (child, _) in forest.owners() {
            let Some(edge) = forest.parent(child) else {
                continue;
            };
            if edge.parent_owner() != owner_id {
                continue;
            }
            let captures = forest
                .ordered_capture_demands(child)
                .iter()
                .map(|demand| {
                    let binding = demand.source_binding();
                    if binding.owner() != owner_id {
                        return Err(freeze("direct-lambda-foreign-capture"));
                    }
                    let name = owner
                        .binding(binding)
                        .ok_or_else(|| freeze("direct-lambda-missing-binding"))?
                        .diagnostic_name();
                    Ok((Box::<str>::from(name), binding))
                })
                .collect::<Result<Vec<_>, String>>()?
                .into_boxed_slice();
            if direct_lambda_captures
                .insert(edge.definition_site().site().node().clone(), captures)
                .is_some()
            {
                return Err(freeze("duplicate-direct-lambda-site"));
            }
        }

        let initializers = map_local::retain_initializers(input, &locals)?;
        Ok(Self {
            initializers,
            owner: owner_id,
            function_origin: input.function().function_origin(),
            source_kind: input.function().source_kind(),
            receiver,
            parameters,
            locals,
            variables,
            assignments,
            binding_names,
            explicit_extern_calls,
            brand_constructors,
            direct_lambda_captures,
            values: BTreeMap::new(),
            dynamic_origins,
            construction: construction::ConstructionState::NotConstruction,
            fault_frame: Some(
                crate::mir::builder::function_fault_frame::FunctionFaultFrameV1::borrowed(),
            ),
            entry_installed: false,
            materialized_locals: BTreeSet::new(),
            consumed_variables: BTreeSet::new(),
            consumed_assignments: BTreeSet::new(),
            consumed_direct_lambdas: BTreeSet::new(),
            consumed_brand_constructors: BTreeSet::new(),
            consumed_source_core_method_calls: BTreeSet::new(),
            source_loop_bridge,
            loop_break_source,
            source_core_method_calls,
            named_array_writes: Vec::new(),
            source_static_result_publications: BTreeMap::new(),
            consumed_source_static_result_publications: BTreeSet::new(),
        })
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

    pub(super) fn publish_source_loop_final_value(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<(), String> {
        if binding.owner() != self.owner {
            return Err(freeze("loop-final-binding-owner-mismatch"));
        }
        let previous = self
            .values
            .get(&binding)
            .copied()
            .ok_or_else(|| freeze("loop-final-binding-before-materialization"))?;
        self.dynamic_origins
            .invalidate_rebind(binding, previous)
            .map_err(|error| error.to_string())?;
        self.values.insert(binding, value);
        Ok(())
    }

    /// Publishes a loop-carrier physical value for the resolver-owned binding
    /// that carries `name`.  Loop producers collect carrier names from the
    /// co-sealed AST; this name index resolves the name to the single
    /// materialized binding so carrier publication and `read_variable` share
    /// `values` as the sole physical authority.  Unknown or ambiguous names
    /// freeze rather than guess a binding.
    pub(super) fn publish_source_loop_final_value_named(
        &mut self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        let mut candidates = self
            .binding_names
            .iter()
            .filter(|(binding, recorded)| {
                recorded.as_ref() == name && self.values.contains_key(*binding)
            })
            .map(|(binding, _)| *binding);
        let Some(binding) = candidates.next() else {
            return Err(freeze("loop-final-name-unknown"));
        };
        if candidates.next().is_some() {
            return Err(freeze("loop-final-name-ambiguous"));
        }
        self.publish_source_loop_final_value(binding, value)
    }

    /// Captures the `values` projection for a speculative branch lane.
    ///
    /// Source-bound `if` lowering produces branch bodies before the
    /// condition and rolls name-map state back between them; the ledger's
    /// `values` projection must follow the same transaction or the
    /// condition would read branch-produced rebinds.  The snapshot covers
    /// `values` only: consumption receipts and `active_origins` are facts
    /// about already-emitted values and stay monotone.
    pub(super) fn source_values_snapshot(&self) -> BTreeMap<BindingRefV1, ValueId> {
        self.values.clone()
    }

    /// Restores the `values` projection captured by `source_values_snapshot`.
    pub(super) fn restore_source_values(&mut self, snapshot: BTreeMap<BindingRefV1, ValueId>) {
        self.values = snapshot;
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
