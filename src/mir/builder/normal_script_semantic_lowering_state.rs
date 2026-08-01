//! Request-local BindingRef-to-ValueId materialization ledger.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::stmts::variable_stmt::OutboxBindingValueV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, EnumVariantAdmissionV1, SourceExprSiteV1, SourceNodeSiteV1,
};
use crate::mir::ValueId;

#[derive(Debug, Default)]
pub(super) struct ScriptSemanticLoweringState {
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    assignments: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    locals: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    nowaits: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    outboxes: BTreeMap<SourceNodeSiteV1, Box<[BindingRefV1]>>,
    lambda_captures: BTreeMap<SourceNodeSiteV1, Box<[(Box<str>, BindingRefV1)]>>,
    record_literal_demands: BTreeMap<SourceNodeSiteV1, u32>,
    enum_variant_demands: BTreeMap<SourceNodeSiteV1, EnumVariantAdmissionV1>,
    enum_match_scrutinee_receipts: BTreeSet<SourceNodeSiteV1>,
    qmark_propagation_receipts: BTreeSet<SourceNodeSiteV1>,
    materialized_outboxes: BTreeSet<SourceNodeSiteV1>,
}
impl ScriptSemanticLoweringState {
    pub(super) fn from_facts<Locals, Nowaits, Outboxes, OutboxBindings, Variables, Assignments>(
        locals: Locals,
        nowaits: Nowaits,
        outboxes: Outboxes,
        variables: Variables,
        assignments: Assignments,
    ) -> Self
    where
        Locals: IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        Nowaits: IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        Outboxes: IntoIterator<Item = (SourceNodeSiteV1, OutboxBindings)>,
        OutboxBindings: IntoIterator<Item = BindingRefV1>,
        Variables: IntoIterator<Item = (SourceExprSiteV1, BindingRefV1)>,
        Assignments: IntoIterator<Item = (SourceExprSiteV1, BindingRefV1)>,
    {
        Self {
            variable_values: BTreeMap::new(),
            variables: variables
                .into_iter()
                .map(|(site, binding)| (site.node().clone(), binding))
                .collect(),
            assignments: assignments
                .into_iter()
                .map(|(site, binding)| (site.node().clone(), binding))
                .collect(),
            locals: locals.into_iter().collect(),
            nowaits: nowaits.into_iter().collect(),
            outboxes: outboxes
                .into_iter()
                .map(|(site, bindings)| (site, bindings.into_iter().collect()))
                .collect(),
            lambda_captures: BTreeMap::new(),
            record_literal_demands: BTreeMap::new(),
            enum_variant_demands: BTreeMap::new(),
            enum_match_scrutinee_receipts: BTreeSet::new(),
            qmark_propagation_receipts: BTreeSet::new(),
            materialized_outboxes: BTreeSet::new(),
        }
    }

    pub(super) fn install_lambda_captures<Captures>(
        &mut self,
        captures: Captures,
    ) -> Result<(), String>
    where
        Captures: IntoIterator<Item = (SourceNodeSiteV1, Box<[(Box<str>, BindingRefV1)]>)>,
    {
        for (site, capture_bindings) in captures {
            if self
                .lambda_captures
                .insert(site, capture_bindings)
                .is_some()
            {
                return Err("[freeze:contract][script-lambda/duplicate-receipt]".to_owned());
            }
        }
        Ok(())
    }

    pub(super) fn lambda_captures(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<Result<Vec<(String, ValueId)>, String>> {
        self.lambda_captures.get(site).map(|captures| {
            captures
                .iter()
                .map(|(name, binding)| {
                    self.value(*binding)
                        .map(|value| (name.to_string(), value))
                        .ok_or_else(|| {
                            "[freeze:contract][script-lambda/capture-before-materialization]"
                                .to_owned()
                        })
                })
                .collect()
        })
    }

    pub(super) fn variable_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.variables.get(site).copied()
    }

    pub(super) fn assignment_binding(&self, site: &SourceNodeSiteV1) -> Option<BindingRefV1> {
        self.assignments.get(site).copied()
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

    pub(super) fn install_record_literal_demands<Demands>(
        &mut self,
        demands: Demands,
    ) -> Result<(), String>
    where
        Demands: IntoIterator<Item = (SourceNodeSiteV1, u32)>,
    {
        for (site, count) in demands {
            if self.record_literal_demands.insert(site, count).is_some() {
                return Err("[freeze:contract][script-record/duplicate-demand]".to_owned());
            }
        }
        Ok(())
    }

    pub(super) fn record_literal_explicit_field_count(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<u32> {
        self.record_literal_demands.get(site).copied()
    }

    pub(super) fn install_enum_variant_demands<Demands>(
        &mut self,
        demands: Demands,
    ) -> Result<(), String>
    where
        Demands: IntoIterator<Item = (SourceNodeSiteV1, EnumVariantAdmissionV1)>,
    {
        for (site, admission) in demands {
            if self.enum_variant_demands.insert(site, admission).is_some() {
                return Err("[freeze:contract][script-enum/duplicate-demand]".to_owned());
            }
        }
        Ok(())
    }

    pub(super) fn enum_variant_demand(
        &self,
        site: &SourceNodeSiteV1,
    ) -> Option<&EnumVariantAdmissionV1> {
        self.enum_variant_demands.get(site)
    }

    pub(super) fn install_enum_match_scrutinee_receipts<Receipts>(
        &mut self,
        receipts: Receipts,
    ) -> Result<(), String>
    where
        Receipts: IntoIterator<Item = SourceNodeSiteV1>,
    {
        for site in receipts {
            if !self.enum_match_scrutinee_receipts.insert(site) {
                return Err("[freeze:contract][script-enum-match/duplicate-receipt]".to_owned());
            }
        }
        Ok(())
    }

    pub(super) fn has_enum_match_scrutinee_receipt(&self, site: &SourceNodeSiteV1) -> bool {
        self.enum_match_scrutinee_receipts.contains(site)
    }

    pub(super) fn install_qmark_propagation_receipts<Receipts>(
        &mut self,
        receipts: Receipts,
    ) -> Result<(), String>
    where
        Receipts: IntoIterator<Item = SourceNodeSiteV1>,
    {
        for site in receipts {
            if !self.qmark_propagation_receipts.insert(site) {
                return Err("[freeze:contract][script-qmark/duplicate-receipt]".to_owned());
            }
        }
        Ok(())
    }

    pub(super) fn has_qmark_propagation_receipt(&self, site: &SourceNodeSiteV1) -> bool {
        self.qmark_propagation_receipts.contains(site)
    }

    pub(super) fn record(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.variable_values.insert(binding, value).is_some() {
            return Err("[freeze:contract][script-lexical/duplicate-value]".to_owned());
        }
        Ok(())
    }

    pub(super) fn rebind(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        let Some(slot) = self.variable_values.get_mut(&binding) else {
            return Err(
                "[freeze:contract][script-lexical/rebind-before-materialization]".to_owned(),
            );
        };
        *slot = value;
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

#[path = "normal_script_binding_materialization.rs"]
mod binding_materialization;
