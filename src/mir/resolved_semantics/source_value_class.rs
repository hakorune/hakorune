//! Source value alternatives and exact owner/flow class evidence.

use std::{collections::BTreeSet, sync::Arc};

use super::{BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1};
use super::source_value_flow::{SourceValueOriginV1, SourceValueRefV1, VerifiedSourceValueFlowV1};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir) enum SourceValueAlternativeV1 {
    Text,
    SignedI64,
    /// An unannotated formal with no source callers still has its declared
    /// tagged carrier. This is a runtime class, not a concrete payload tag.
    TaggedDynamic,
    Bool,
    Null,
    Other,
    Object(Box<str>),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceValueSumV1(pub(in crate::mir) BTreeSet<SourceValueAlternativeV1>);
impl SourceValueSumV1 {
    /// A closed nominal source may include null but must contain one class.
    /// This is a value-class fact, not a live or non-null read permission.
    pub(in crate::mir) fn nullable_object_class_v1(&self) -> Option<(&str, bool)> {
        Self::nullable_object_alternatives_v1(&self.0)
    }
    pub(in crate::mir) fn nullable_object_alternatives_v1(values: &BTreeSet<SourceValueAlternativeV1>) -> Option<(&str, bool)> {
        let class = values.iter().find_map(|value| match value {
            SourceValueAlternativeV1::Object(name) => Some(name.as_ref()),
            _ => None,
        })?;
        values.iter().all(|value| matches!(value, SourceValueAlternativeV1::Null)
            || matches!(value, SourceValueAlternativeV1::Object(actual) if actual.as_ref() == class))
            .then_some((class, values.contains(&SourceValueAlternativeV1::Null)))
    }
    pub(in crate::mir) fn object_class_v1(&self) -> Option<&str> {
        if self.0.len() != 1 { return None; }
        match self.0.first()? {
            SourceValueAlternativeV1::Object(class) => Some(class),
            _ => None,
        }
    }
    pub(in crate::mir) fn alternatives(&self) -> &BTreeSet<SourceValueAlternativeV1> {
        &self.0
    }
    pub(in crate::mir) fn signed_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::SignedI64)
    }
    pub(in crate::mir) fn text_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::Text)
    }
    pub(in crate::mir) fn tagged_dynamic_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::TaggedDynamic)
    }
    pub(in crate::mir) fn bool_only(&self) -> bool {
        self.0.len() == 1 && self.0.contains(&SourceValueAlternativeV1::Bool)
    }
    pub(in crate::mir) fn finite_input(&self) -> bool {
        !self.0.is_empty()
            && self.0.iter().all(|value| {
                matches!(
                    value,
                    SourceValueAlternativeV1::Text
                        | SourceValueAlternativeV1::SignedI64
                        | SourceValueAlternativeV1::Bool
                        | SourceValueAlternativeV1::Null
                )
            })
    }
    pub(in crate::mir) fn permits_object(&self, name: &str) -> bool {
        self.0.iter().all(|value| matches!(value, SourceValueAlternativeV1::Null)
            || matches!(value, SourceValueAlternativeV1::Object(actual) if actual.as_ref() == name))
    }
}

/// Source-only classes for exact formal and flow points in one complete
/// original SUM. The retained flow identity prevents these facts from being
/// reused by another callable; they grant no payload conversion or execution.
#[derive(Debug, Clone)]
pub(crate) struct SourceValueClassFactsV1 {
    owner: FunctionOwnerIdV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
    tagged_dynamic_formals: BTreeSet<BindingRefV1>,
    tagged_dynamic_points: BTreeSet<SourceValueRefV1>,
    signed_i64_formals: BTreeSet<BindingRefV1>,
}
impl SourceValueClassFactsV1 {
    pub(in crate::mir) fn new(
        owner: FunctionOwnerIdV1,
        flow: Arc<VerifiedSourceValueFlowV1>,
        tagged_dynamic_formals: BTreeSet<BindingRefV1>,
        tagged_dynamic_points: BTreeSet<SourceValueRefV1>,
        signed_i64_formals: BTreeSet<BindingRefV1>,
    ) -> Self {
        Self { owner, flow, tagged_dynamic_formals, tagged_dynamic_points, signed_i64_formals }
    }

    pub(in crate::mir) fn proves_formal(&self, binding: BindingRefV1) -> bool {
        binding.owner() == self.owner && self.tagged_dynamic_formals.contains(&binding)
    }
    pub(in crate::mir) fn proves_signed_i64_formal(&self, binding: BindingRefV1) -> bool {
        binding.owner() == self.owner && self.signed_i64_formals.contains(&binding)
    }
    pub(in crate::mir) fn proves_value(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        value: SourceValueRefV1,
    ) -> bool {
        self.owner == flow.owner()
            && Arc::ptr_eq(&self.flow, flow)
            && self.tagged_dynamic_points.contains(&value)
    }
    pub(in crate::mir) fn proves_read(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
    ) -> Option<SourceValueRefV1> {
        if binding.owner() != self.owner || !Arc::ptr_eq(&self.flow, flow) {
            return None;
        }
        let value = flow.read(site, binding)?;
        // A formal's direct read is already anchored by the exact Entry
        // origin and the formal's Dynamic SUM. It need not also appear as a
        // derived point: alias and computed reads still require point membership.
        let exact_formal_entry = self.proves_formal(binding)
            && matches!(
                flow.origin(value),
                Some(SourceValueOriginV1::Entry(
                    original,
                )) if *original == binding
            );
        (self.proves_value(flow, value) || exact_formal_entry).then_some(value)
    }
}
