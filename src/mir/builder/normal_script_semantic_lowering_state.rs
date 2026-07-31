//! Request-local BindingRef-to-ValueId materialization ledger.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceNodeSiteV1};
use crate::mir::ValueId;

#[derive(Debug, Default)]
pub(super) struct ScriptSemanticLoweringState {
    variable_values: BTreeMap<BindingRefV1, ValueId>,
    variables: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    locals: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
    nowaits: BTreeMap<SourceNodeSiteV1, BindingRefV1>,
}
impl ScriptSemanticLoweringState {
    pub(super) fn from_facts(
        locals: impl IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        nowaits: impl IntoIterator<Item = (SourceNodeSiteV1, BindingRefV1)>,
        variables: impl IntoIterator<Item = (SourceExprSiteV1, BindingRefV1)>,
    ) -> Self {
        Self {
            variable_values: BTreeMap::new(),
            variables: variables
                .into_iter()
                .map(|(site, binding)| (site.node().clone(), binding))
                .collect(),
            locals: locals.into_iter().collect(),
            nowaits: nowaits.into_iter().collect(),
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

    pub(super) fn value(&self, binding: BindingRefV1) -> Option<ValueId> {
        self.variable_values.get(&binding).copied()
    }

    pub(super) fn record(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.variable_values.insert(binding, value).is_some() {
            return Err("[freeze:contract][script-lexical/duplicate-value]".to_owned());
        }
        Ok(())
    }
}
