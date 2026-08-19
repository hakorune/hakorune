//! Request-local physical projection for one resolved callable owner.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::mir::builder::stmts::CompletedLocalStatementV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceNodeSiteV1,
};
use crate::mir::ValueId;

use super::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use super::normal_callable_dynamic_origin::{
    CallableDynamicOriginLoweringStateV1, CurrentDynamicBindingReceiptV1,
    PreparedDynamicOriginRebindV1,
};
use super::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;

/// Physical values materialized while lowering one callable body.
///
/// Semantic identity remains owned by `VerifiedResolvedFunctionV1`; this state
/// only projects that identity onto the `ValueId`s allocated by existing Lower.
#[derive(Debug)]
pub(super) struct CallableSemanticLoweringState {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    receiver: Option<BindingRefV1>,
    parameters: Box<[BindingRefV1]>,
    locals: BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    assignments: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
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

        for site in owner.declaration_sites() {
            let Some(binding) = owner.declaration_binding(site) else {
                return Err(freeze("missing-declaration-binding"));
            };
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

        Ok(Self {
            owner: owner_id,
            receiver,
            parameters,
            locals,
            variables,
            assignments,
            explicit_extern_calls,
            brand_constructors,
            direct_lambda_captures,
            values: BTreeMap::new(),
            dynamic_origins,
            entry_installed: false,
            materialized_locals: BTreeSet::new(),
            consumed_variables: BTreeSet::new(),
            consumed_assignments: BTreeSet::new(),
            consumed_direct_lambdas: BTreeSet::new(),
        })
    }

    pub(super) fn explicit_extern_symbol(&self, site: &SourceNodeSiteV1) -> Option<&str> {
        self.explicit_extern_calls.get(site).map(Box::as_ref)
    }

    pub(super) fn brand_constructor_disposition(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Result<
        super::brand_constructor_lowering_projection::BrandConstructorDispositionRefV1<'_>,
        super::brand_constructor_lowering_projection::BrandConstructorProjectionErrorV1,
    > {
        self.brand_constructors.disposition(site)
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
        self.values
            .get(&binding)
            .copied()
            .ok_or_else(|| freeze("variable-before-materialization"))
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

    pub(super) fn finish(self) -> Result<(), String> {
        self.dynamic_origins
            .finish()
            .map_err(|error| error.to_string())?;
        let missing_variables = self
            .variables
            .keys()
            .filter(|site| !self.consumed_variables.contains(*site))
            .collect::<Vec<_>>();
        if !self.entry_installed
            || self.materialized_locals.len() != self.locals.len()
            || self.consumed_variables.len() != self.variables.len()
            || self.consumed_assignments.len() != self.assignments.len()
            || self.consumed_direct_lambdas.len() != self.direct_lambda_captures.len()
        {
            return Err(format!(
                "{} owner={:?} entry={} locals={}/{} variables={}/{} missing_variables={:?} assignments={}/{} lambdas={}/{}",
                freeze("incomplete-consumption"),
                self.owner,
                self.entry_installed,
                self.materialized_locals.len(),
                self.locals.len(),
                self.consumed_variables.len(),
                self.variables.len(),
                missing_variables,
                self.consumed_assignments.len(),
                self.assignments.len(),
                self.consumed_direct_lambdas.len(),
                self.direct_lambda_captures.len(),
            ));
        }
        Ok(())
    }

    fn insert_value(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.values.insert(binding, value).is_some() {
            return Err(freeze("duplicate-value"));
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn dynamic_current_origin_for_test(
        &self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Option<BindingRefV1> {
        self.dynamic_origins.current_origin(binding, value)
    }

    #[cfg(test)]
    pub(super) fn dynamic_value_origin_for_test(&self, value: ValueId) -> Option<BindingRefV1> {
        self.dynamic_origins.value_origin(value)
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
