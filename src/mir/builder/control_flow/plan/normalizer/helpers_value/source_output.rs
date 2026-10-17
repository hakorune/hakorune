//! Output correspondence retained by the sole value normalizer. Source class
//! proof remains with the original Home expression, never with a ValueId/type.
use crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1};
use crate::mir::ValueId;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::builder) struct NormalizedSourceValueV1 {
    site: OwnedExprSiteV1,
    value: ValueId,
    kind: NormalizedSourceValueKindV1,
}
#[derive(Debug)]
enum NormalizedSourceValueKindV1 {
    Literal(i64),
    Variable(Rc<ExactLexicalReadV1>),
    Add(Rc<NormalizedSourceValueV1>, Rc<NormalizedSourceValueV1>),
}
impl NormalizedSourceValueV1 {
    pub(super) fn literal(site: OwnedExprSiteV1, value: ValueId, literal: i64) -> Rc<Self> {
        Rc::new(Self { site, value, kind: NormalizedSourceValueKindV1::Literal(literal) })
    }
    pub(super) fn variable(read: ExactLexicalReadV1) -> Rc<Self> {
        let (site, _, value) = read.source_value();
        Rc::new(Self { site, value, kind: NormalizedSourceValueKindV1::Variable(Rc::new(read)) })
    }
    pub(super) fn add(site: OwnedExprSiteV1, value: ValueId, lhs: Rc<Self>, rhs: Rc<Self>) -> Rc<Self> {
        Rc::new(Self { site, value, kind: NormalizedSourceValueKindV1::Add(lhs, rhs) })
    }
    fn values(&self, values: &mut BTreeMap<ValueId, ValueId>) {
        values.insert(self.value, self.value);
        if let NormalizedSourceValueKindV1::Add(lhs, rhs) = &self.kind {
            lhs.values(values); rhs.values(values);
        }
    }
    fn matches_source(&self, owner: FunctionOwnerIdV1,
        source: &crate::mir::builder::VerifiedNoExitIntegerExpressionV1,
    ) -> bool {
        if self.site.owner() != owner || self.site.site() != source.site() { return false; }
        match &self.kind {
            NormalizedSourceValueKindV1::Literal(value) => source.literal() == Some(*value),
            NormalizedSourceValueKindV1::Variable(read) => source.binding().is_some_and(|binding|
                read.value_for(owner, source.site().node(), binding) == Ok(self.value)),
            NormalizedSourceValueKindV1::Add(lhs, rhs) => source.operands().is_some_and(|(left, right)|
                lhs.matches_source(owner, left) && rhs.matches_source(owner, right)),
        }
    }
}

/// Each speculative CorePlan clone owns its physical projection. Freshening
/// never edits the immutable source output or the shared affine emission port.
#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SourceValueProjectionV1 {
    original: Rc<NormalizedSourceValueV1>,
    values: BTreeMap<ValueId, ValueId>,
}
impl SourceValueProjectionV1 {
    pub(in crate::mir::builder) fn from_output(original: Rc<NormalizedSourceValueV1>) -> Self {
        let mut values = BTreeMap::new(); original.values(&mut values);
        Self { original, values }
    }
    pub(in crate::mir::builder) fn root_value(&self) -> ValueId { self.values[&self.original.value] }
    pub(in crate::mir::builder) fn matches_source(&self, owner: FunctionOwnerIdV1,
        source: &crate::mir::builder::VerifiedNoExitIntegerExpressionV1,
    ) -> bool { self.original.matches_source(owner, source) }
    pub(in crate::mir::builder::control_flow::plan::normalizer) fn remap(
        &mut self, map: &super::super::cond_lowering_freshen::FresheningValuesV1<'_>,
    ) {
        for value in self.values.values_mut() { *value = map.project(*value); }
    }
    pub(in crate::mir::builder) fn exact_variable_read_v1(
        &self, owner: FunctionOwnerIdV1, site: &crate::mir::resolved_semantics::SourceExprSiteV1,
        binding: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Result<Rc<ExactLexicalReadV1>, String> {
        let NormalizedSourceValueKindV1::Variable(read) = &self.original.kind else {
            return Err("[freeze:contract][ordinary-field-read/plan-receiver-not-variable]".into());
        };
        if self.original.site.owner() != owner || self.original.site.site() != site
            || read.value_for(owner, site.node(), binding).map_err(|issue| issue.to_string())? != self.original.value {
            return Err("[freeze:contract][ordinary-field-read/plan-receiver-source-drift]".into());
        }
        Ok(Rc::clone(read))
    }
    pub(in crate::mir::builder) fn projected_values(&self) -> impl Iterator<Item = ValueId> + '_ {
        self.values.values().copied()
    }
}

mod observed;
pub(in crate::mir) use observed::ObservedSourceValueV1;
