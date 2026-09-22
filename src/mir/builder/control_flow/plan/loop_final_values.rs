//! Final-value transport; source identity travels with values through freshening.
//! This product carries no publication permission. The active loop scope owns it.
use crate::mir::builder::normal_callable_loop_source_facts::{
    CallableGenericLoopSourceRelationViewV1, CallableLoopCarrierSlotV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) enum CoreLoopFinalValuesV1 {
    Raw(Vec<(String, ValueId)>),
    Source(SourceLoopFinalValuesV1),
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SourceLoopFinalValuesV1 {
    owner: FunctionOwnerIdV1,
    site: SourceNodeSiteV1,
    rows: Vec<SourceLoopFinalValueV1>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SourceLoopFinalValueV1 {
    slot: CallableLoopCarrierSlotV1,
    binding: BindingRefV1,
    label: String,
    value: ValueId,
}

impl From<Vec<(String, ValueId)>> for CoreLoopFinalValuesV1 {
    fn from(rows: Vec<(String, ValueId)>) -> Self {
        Self::Raw(rows)
    }
}
impl CoreLoopFinalValuesV1 {
    pub(in crate::mir::builder) fn from_source_view(
        view: &CallableGenericLoopSourceRelationViewV1<'_>,
        values: &[(CallableLoopCarrierSlotV1, ValueId)],
    ) -> Result<Self, String> {
        let mut by_slot = BTreeMap::new();
        for &(slot, value) in values {
            if by_slot.insert(slot, value).is_some() {
                return Err(reject("duplicate-slot"));
            }
        }
        let relation = view.carrier_relation();
        if by_slot.len() != relation.carriers().len() {
            return Err(reject("slot-coverage"));
        }
        let mut bindings = BTreeSet::new();
        let mut rows = Vec::new();
        for carrier in relation.carriers() {
            if carrier.binding().owner() != view.owner() || !bindings.insert(carrier.binding()) {
                return Err(reject("binding-identity"));
            }
            let value = by_slot
                .remove(&carrier.slot())
                .ok_or_else(|| reject("slot-coverage"))?;
            rows.push(SourceLoopFinalValueV1 {
                slot: carrier.slot(),
                binding: carrier.binding(),
                label: carrier.label().to_owned(),
                value,
            });
        }
        if !by_slot.is_empty() {
            return Err(reject("slot-coverage"));
        }
        Ok(Self::Source(SourceLoopFinalValuesV1 {
            owner: view.owner(),
            site: view.pre_effect().loop_site().clone(),
            rows,
        }))
    }
    pub(in crate::mir::builder) fn raw_rows(&self) -> Result<&[(String, ValueId)], String> {
        match self {
            Self::Raw(rows) => Ok(rows),
            Self::Source(_) => Err(reject("source-at-raw-consumer")),
        }
    }
    pub(in crate::mir::builder) fn raw_rows_mut(
        &mut self,
    ) -> Result<&mut Vec<(String, ValueId)>, String> {
        match self {
            Self::Raw(rows) => Ok(rows),
            Self::Source(_) => Err(reject("source-at-raw-consumer")),
        }
    }
    pub(in crate::mir::builder) fn len(&self) -> usize {
        match self {
            Self::Raw(rows) => rows.len(),
            Self::Source(source) => source.rows.len(),
        }
    }
    pub(in crate::mir::builder) fn diagnostic_values(
        &self,
    ) -> Box<dyn Iterator<Item = (&str, ValueId)> + '_> {
        match self {
            Self::Raw(rows) => Box::new(rows.iter().map(|(label, value)| (label.as_str(), *value))),
            Self::Source(source) => Box::new(
                source
                    .rows
                    .iter()
                    .map(|row| (row.label.as_str(), row.value)),
            ),
        }
    }
    pub(in crate::mir::builder) fn remap_values(
        &mut self,
        mut remap: impl FnMut(ValueId) -> ValueId,
    ) {
        match self {
            Self::Raw(rows) => {
                for (_, value) in rows {
                    *value = remap(*value);
                }
            }
            Self::Source(source) => {
                for row in &mut source.rows {
                    row.value = remap(row.value);
                }
            }
        }
    }
}
impl SourceLoopFinalValuesV1 {
    pub(in crate::mir::builder) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(in crate::mir::builder) fn site(&self) -> &SourceNodeSiteV1 {
        &self.site
    }
    pub(in crate::mir::builder) fn rows(&self) -> &[SourceLoopFinalValueV1] {
        &self.rows
    }
}
impl SourceLoopFinalValueV1 {
    pub(in crate::mir::builder) fn slot(&self) -> CallableLoopCarrierSlotV1 {
        self.slot
    }
    pub(in crate::mir::builder) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(in crate::mir::builder) fn value(&self) -> ValueId {
        self.value
    }
}
fn reject(reason: &str) -> String {
    format!("[freeze:contract][loop-final-values/{reason}]")
}
