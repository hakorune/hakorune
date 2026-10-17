//! Original terminating null arm, scoped to one covered surviving Prefix path.
use super::*;
use crate::mir::resolved_semantics::source_value_flow::{SourceValueRefV1, VerifiedSourceValueFlowV1};
use std::{rc::Rc, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct EntryNullExclusionV1 {
    statement: SourceStmtSiteV1,
    bundle: crate::mir::resolved_semantics::ResolvedIfRegionBundleV1,
    condition: crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
    guard_read: SourceExprSiteV1,
    binding: BindingRefV1,
    formal: BindingRefV1,
    value: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
}
impl PartialEq for EntryNullExclusionV1 {
    fn eq(&self, other: &Self) -> bool {
        self.statement == other.statement && self.bundle == other.bundle
            && self.condition == other.condition && self.guard_read == other.guard_read
            && self.binding == other.binding && self.formal == other.formal && self.value == other.value
            && Arc::ptr_eq(&self.flow, &other.flow)
    }
}
impl Eq for EntryNullExclusionV1 {}
impl EntryNullExclusionV1 {
    pub(crate) fn formal(&self) -> BindingRefV1 { self.formal }
    pub(crate) fn same_entry(&self, flow: &Arc<VerifiedSourceValueFlowV1>, formal: BindingRefV1) -> bool {
        Arc::ptr_eq(flow, &self.flow) && self.formal == formal
    }
    pub(crate) fn corroborates(
        &self, flow: &Arc<VerifiedSourceValueFlowV1>, value: SourceValueRefV1,
        root: BindingRefV1, reference: impl FnMut(&SourceExprSiteV1) -> Option<ResolvedLexicalRefV1>,
    ) -> bool {
        Arc::ptr_eq(flow, &self.flow) && root == self.formal
            && self.flow.read(&self.guard_read, self.binding) == Some(self.value)
            && flow.alias_entry_origin(value, reference) == Some(self.formal)
    }
}

/// Only the real branch walker calls this after its selected null arm has
/// actually terminated and the surviving path is covered. No Object mark,
/// may-Fault call, raw condition or unwalked branch can issue this evidence.
pub(super) fn issue(
    input: ResolvedFunctionLoweringInputV1<'_>, statement: &SourceStmtSiteV1,
    condition: &SourceExprSiteV1, then_terminated: bool, else_terminated: bool,
) -> Option<Rc<EntryNullExclusionV1>> {
    let (binding, operator) = null_guarded_local(input.function(), condition)?;
    if !(operator == ResolvedBinaryOperatorV1::Equal && then_terminated && !else_terminated
        || operator == ResolvedBinaryOperatorV1::NotEqual && else_terminated && !then_terminated) { return None; }
    let bundle = input.function().if_region_bundle(statement).ok()?;
    let condition = input.function().expression_source().binary(condition)?.clone();
    let guard_read = [condition.lhs(), condition.rhs()].into_iter().find(|site|
        input.function().variable_ref(site) == Some(ResolvedLexicalRefV1::Local(binding)))?.clone();
    let flow = input.function().expression_source().value_flow()?;
    let value = flow.read(&guard_read, binding)?;
    let formal = flow.alias_entry_origin(value, |site| input.function().variable_ref(site))?;
    Some(Rc::new(EntryNullExclusionV1 { statement: statement.clone(), bundle: *bundle, condition,
        guard_read, binding, formal, value, flow: Arc::clone(flow) }))
}
