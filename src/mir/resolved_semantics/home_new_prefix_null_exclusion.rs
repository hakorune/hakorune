//! Original terminating null arm, scoped to one covered surviving Prefix path.
use super::*;
use crate::mir::resolved_semantics::source_value_flow::{
    SourceValueRefV1, VerifiedSourceValueFlowV1,
};
use std::{rc::Rc, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq)]
enum NullRootV1 {
    Entry(BindingRefV1),
    Initializer {
        binding: BindingRefV1,
        site: SourceExprSiteV1,
        value: SourceValueRefV1,
    },
}
#[derive(Debug, Clone)]
pub(crate) struct SourceNullExclusionV1 {
    statement: SourceStmtSiteV1,
    bundle: crate::mir::resolved_semantics::ResolvedIfRegionBundleV1,
    condition: crate::mir::resolved_semantics::ResolvedBinaryExpressionSourceV1,
    guard_read: SourceExprSiteV1,
    binding: BindingRefV1,
    root: NullRootV1,
    value: SourceValueRefV1,
    flow: Arc<VerifiedSourceValueFlowV1>,
}
impl PartialEq for SourceNullExclusionV1 {
    fn eq(&self, other: &Self) -> bool {
        self.statement == other.statement
            && self.bundle == other.bundle
            && self.condition == other.condition
            && self.guard_read == other.guard_read
            && self.binding == other.binding
            && self.root == other.root
            && self.value == other.value
            && Arc::ptr_eq(&self.flow, &other.flow)
    }
}
impl Eq for SourceNullExclusionV1 {}
impl SourceNullExclusionV1 {
    /// Borrow only the original If identity; physical nonnull permission is
    /// checked independently by the existing Bool/Branch owner.
    pub(crate) fn condition_read_v1(
        &self,
        source: &crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1,
    ) -> Option<&SourceExprSiteV1> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
        (source.owner() == self.binding.owner()
            && matches!(source.root(), ExpressionPlanRootV1::IfCondition { statement, condition, bundle }
                if statement == &self.statement && condition == self.condition.site() && bundle == &self.bundle)
            && source.nodes().first().and_then(|node| node.binary()) == Some(&self.condition))
            .then_some(&self.guard_read)
    }
    pub(crate) fn root_binding(&self) -> BindingRefV1 {
        match self.root {
            NullRootV1::Entry(binding) | NullRootV1::Initializer { binding, .. } => binding,
        }
    }
    pub(crate) fn same_entry(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        formal: BindingRefV1,
    ) -> bool {
        Arc::ptr_eq(flow, &self.flow) && self.root == NullRootV1::Entry(formal)
    }
    pub(crate) fn corroborates_initializer(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        binding: BindingRefV1,
        initializer: &SourceExprSiteV1,
        read: &SourceExprSiteV1,
    ) -> bool {
        let NullRootV1::Initializer {
            binding: root,
            site,
            value: origin,
        } = &self.root
        else {
            return false;
        };
        let Some(flow) = input.function().expression_source().value_flow() else {
            return false;
        };
        *root == binding
            && site == initializer
            && Arc::ptr_eq(flow, &self.flow)
            && flow.read(&self.guard_read, self.binding) == Some(self.value)
            && flow.alias_origin(self.value, |site| input.function().variable_ref(site))
                == Some(*origin)
            && flow.read(read, binding).and_then(|value| {
                flow.alias_origin(value, |site| input.function().variable_ref(site))
            }) == Some(*origin)
    }
    pub(crate) fn corroborates(
        &self,
        flow: &Arc<VerifiedSourceValueFlowV1>,
        value: SourceValueRefV1,
        root: BindingRefV1,
        reference: impl FnMut(&SourceExprSiteV1) -> Option<ResolvedLexicalRefV1>,
    ) -> bool {
        Arc::ptr_eq(flow, &self.flow)
            && self.root == NullRootV1::Entry(root)
            && self.flow.read(&self.guard_read, self.binding) == Some(self.value)
            && flow.alias_entry_origin(value, reference) == Some(root)
    }
}

/// Only the real branch walker calls this after its selected null arm has
/// actually terminated and the surviving path is covered. No Object mark,
/// may-Fault call, raw condition or unwalked branch can issue this evidence.
pub(super) fn issue(
    input: ResolvedFunctionLoweringInputV1<'_>,
    statement: &SourceStmtSiteV1,
    condition: &SourceExprSiteV1,
    then_terminated: bool,
    else_terminated: bool,
) -> Option<Rc<SourceNullExclusionV1>> {
    let (binding, operator) = null_guarded_local(input.function(), condition)?;
    if !(operator == ResolvedBinaryOperatorV1::Equal && then_terminated && !else_terminated
        || operator == ResolvedBinaryOperatorV1::NotEqual && else_terminated && !then_terminated)
    {
        return None;
    }
    let bundle = input.function().if_region_bundle(statement).ok()?;
    let condition = input
        .function()
        .expression_source()
        .binary(condition)?
        .clone();
    let guard_read = [condition.lhs(), condition.rhs()]
        .into_iter()
        .find(|site| {
            input.function().variable_ref(site) == Some(ResolvedLexicalRefV1::Local(binding))
        })?
        .clone();
    let flow = input.function().expression_source().value_flow()?;
    let value = flow.read(&guard_read, binding)?;
    let root = if let Some(formal) =
        flow.alias_entry_origin(value, |site| input.function().variable_ref(site))
    {
        NullRootV1::Entry(formal)
    } else {
        if !flow.is_complete() {
            return None;
        }
        let origin = flow.alias_origin(value, |site| input.function().variable_ref(site))?;
        let crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1::Expression(
            site,
        ) = flow.origin(origin)?
        else {
            return None;
        };
        let mut declarations = input
            .function()
            .expression_source()
            .initializers()
            .filter(|row| row.binding() == binding && row.initializer_site() == Some(site));
        declarations.next()?;
        if declarations.next().is_some() {
            return None;
        }
        NullRootV1::Initializer {
            binding,
            site: site.clone(),
            value: origin,
        }
    };
    Some(Rc::new(SourceNullExclusionV1 {
        statement: statement.clone(),
        bundle: *bundle,
        condition,
        guard_read,
        binding,
        root,
        value,
        flow: Arc::clone(flow),
    }))
}
