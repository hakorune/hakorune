//! Request-local BindingRef-to-ValueId materialization ledger.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::stmts::variable_stmt::OutboxBindingValueV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceNodeSiteV1};
use crate::mir::ValueId;

#[derive(Debug, Default)]
pub(super) struct ScriptSemanticLoweringState {
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    locals: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    nowaits: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    outboxes: BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
    materialized_outboxes: BTreeSet<SourceNodeSiteV1>,
}
impl ScriptSemanticLoweringState {
    pub(super) fn from_facts<Locals, Nowaits, Outboxes, OutboxBindings, Variables>(
        locals: Locals,
        nowaits: Nowaits,
        outboxes: Outboxes,
        variables: Variables,
    ) -> Self
    where
        Locals: IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        Nowaits: IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        Outboxes: IntoIterator<Item = (SourceNodeSiteV1, OutboxBindings)>,
        OutboxBindings: IntoIterator<Item = BindingRefV1>,
        Variables: IntoIterator<Item = (SourceExprSiteV1, BindingRefV1)>,
    {
        Self {
            variable_values: BTreeMap::new(),
            variables: variables
                .into_iter()
                .map(|(site, binding)| (site.node().clone(), binding))
                .collect(),
            locals: locals.into_iter().collect(),
            nowaits: nowaits.into_iter().collect(),
            outboxes: outboxes
                .into_iter()
                .map(|(site, bindings)| (site, bindings.into_iter().collect()))
                .collect(),
            materialized_outboxes: BTreeSet::new(),
        }
    }

    pub(super) fn variable_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.variables.get(site).copied()
    }

    pub(super) fn local_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.locals.get(site).copied()
    }

    pub(super) fn nowait_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.nowaits.get(site).copied()
    }

    pub(super) fn outbox_binding_count(&self, site: &SourceNodeSiteV1) -> Result<usize, String> {
        self.outboxes
            .get(site)
            .map(|bindings| bindings.len())
            .ok_or_else(|| "[freeze:contract][script-lexical/outbox-binding]".to_owned())
    }

    pub(super) fn value(&self, binding: BindingRefV1) -> Option<ValueId> {
        self.variable_values.get(&binding).copied()
    }

    pub(super) fn record(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.variable_values.insert(binding, value).is_some() {
            return Err("[freeze:contract][script-lexical/duplicate-value]".to_owned());
        }
        Ok(())
    }

    pub(super) fn record_outbox_receipt(
        &mut self,
        site: &SourceNodeSiteV1,
        emitted: &[OutboxBindingValueV1],
    ) -> Result<(), String> {
        let bindings = self
            .outboxes
            .get(site)
            .ok_or_else(|| "[freeze:contract][script-lexical/outbox-binding]".to_owned())?;
        if self.materialized_outboxes.contains(site) {
            return Err("[freeze:contract][script-lexical/outbox-duplicate]".to_owned());
        }
        if bindings.len() != emitted.len() {
            return Err("[freeze:contract][script-lexical/outbox-cardinality]".to_owned());
        }
        for (ordinal, (binding, row)) in bindings.iter().zip(emitted).enumerate() {
            if row.ordinal() != ordinal as u32 || self.variable_values.contains_key(binding) {
                return Err("[freeze:contract][script-lexical/outbox-receipt]".to_owned());
            }
        }
        for (binding, row) in bindings.iter().zip(emitted) {
            self.variable_values.insert(*binding, row.value());
        }
        self.materialized_outboxes.insert(site.clone());
        Ok(())
    }
}
