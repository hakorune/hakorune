//! Request-local physical projection for one resolved callable owner.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::stmts::CompletedLocalStatementV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceNodeSiteV1, VerifiedSemanticOwnerForestV1,
};
use crate::mir::ValueId;

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
    direct_lambda_captures: BTreeMap<SourceNodeSiteV1, Box<[(Box<str>, BindingRefV1)]>>,
    values: BTreeMap<BindingRefV1, ValueId>,
    entry_installed: bool,
    materialized_locals: BTreeSet<SourceNodeSiteV1>,
    consumed_variables: BTreeSet<SourceNodeSiteV1>,
    consumed_assignments: BTreeSet<SourceNodeSiteV1>,
    consumed_direct_lambdas: BTreeSet<SourceNodeSiteV1>,
}

impl CallableSemanticLoweringState {
    pub(super) fn from_forest(forest: &VerifiedSemanticOwnerForestV1) -> Result<Self, String> {
        let [root] = forest.roots() else {
            return Err(freeze("root-cardinality"));
        };
        let owner = forest.owner(*root).ok_or_else(|| freeze("root-owner"))?;
        let owner_id = owner.owner();
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
            direct_lambda_captures,
            values: BTreeMap::new(),
            entry_installed: false,
            materialized_locals: BTreeSet::new(),
            consumed_variables: BTreeSet::new(),
            consumed_assignments: BTreeSet::new(),
            consumed_direct_lambdas: BTreeSet::new(),
        })
    }

    pub(super) fn loop_binding_source_projection(
        &self,
    ) -> super::normal_callable_loop_handoff::CallableLoopSourceProjectionV1<'_> {
        super::normal_callable_loop_handoff::CallableLoopSourceProjectionV1::new(
            self.owner,
            &self.variables,
            &self.assignments,
        )
    }

    pub(super) fn install_entry_values(
        &mut self,
        receiver: Option<ValueId>,
        parameters: &[ValueId],
    ) -> Result<(), String> {
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
        if bindings.len() != completed.values().len()
            || !self.materialized_locals.insert(site.clone())
        {
            return Err(freeze("local-materialization-mismatch"));
        }
        for (binding, value) in bindings
            .iter()
            .copied()
            .zip(completed.values().iter().copied())
        {
            self.insert_value(binding, value)?;
        }
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
        let slot = self
            .values
            .get_mut(&binding)
            .ok_or_else(|| freeze("rebind-before-materialization"))?;
        *slot = value;
        Ok(())
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
        if !self.entry_installed
            || self.materialized_locals.len() != self.locals.len()
            || self.consumed_variables.len() != self.variables.len()
            || self.consumed_assignments.len() != self.assignments.len()
            || self.consumed_direct_lambdas.len() != self.direct_lambda_captures.len()
        {
            return Err(format!(
                "{} entry={} locals={}/{} variables={}/{} assignments={}/{} lambdas={}/{}",
                freeze("incomplete-consumption"),
                self.entry_installed,
                self.materialized_locals.len(),
                self.locals.len(),
                self.consumed_variables.len(),
                self.variables.len(),
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
