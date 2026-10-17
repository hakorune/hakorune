//! Original initializer Text reads. Source proof only; current state is separate.
use super::*;
use std::collections::BTreeSet;
use super::super::super::borrowed_formal_source::{SourceValueSumV1, SourceValueAlternativeV1};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, ResolvedLexicalRefV1};
use crate::mir::resolved_semantics::source_value_flow::{VerifiedSourceValueFlowV1, SourceValueOriginV1, SourceValueRefV1};
use crate::mir::resolved_value_profile::expression_plan::{ExpressionSourcePlanV1, ExpressionPlanRootV1};
use crate::mir::normal_callable_semantic_package::BorrowedTextEntryReadRequirementV1;

#[derive(Debug, Clone)]
pub(crate) struct InitializerTextEntryReadV1 {
    requirement: BorrowedTextEntryReadRequirementV1,
    sum: Rc<SourceValueSumV1>,
    flow: std::sync::Arc<VerifiedSourceValueFlowV1>,
    value: SourceValueRefV1,
}
impl PartialEq for InitializerTextEntryReadV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.sum, &other.sum) && std::sync::Arc::ptr_eq(&self.flow, &other.flow)
            && self.value == other.value && self.requirement == other.requirement
            && Rc::ptr_eq(self.requirement.source_plan(), other.requirement.source_plan())
    }
}
impl Eq for InitializerTextEntryReadV1 {}
impl InitializerTextEntryReadV1 {
    pub(crate) fn requirement(&self) -> &BorrowedTextEntryReadRequirementV1 { &self.requirement }
}

// Follow only original resolver alias edges. This is provenance, not a second
// expression classifier. Joins, computed values and cycles need their own proof.
fn entry_origin(
    input: ResolvedFunctionLoweringInputV1<'_>, flow: &VerifiedSourceValueFlowV1,
    mut value: SourceValueRefV1, root: BindingRefV1,
) -> bool {
    let mut seen = BTreeSet::new();
    while seen.insert(value) {
        match flow.origin(value) {
            Some(SourceValueOriginV1::Entry(original)) => return *original == root,
            Some(SourceValueOriginV1::Expression(site)) => {
                let Some(ResolvedLexicalRefV1::Local(binding)) = input.function().variable_ref(site) else { return false };
                let Some(original) = flow.read(site, binding) else { return false };
                value = original;
            }
            _ => return false,
        }
    }
    false
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_initializer_text_entry_read_v1(
    input: ResolvedFunctionLoweringInputV1<'_>, prepared: &PreparedBorrowedFormalIngressV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    site: &OwnedExprSiteV1, plan: &Rc<ExpressionSourcePlanV1>,
    read: &SourceExprSiteV1, binding: BindingRefV1, root: BindingRefV1,
) -> Result<Option<InitializerTextEntryReadV1>, String> {
    let reject = || freeze("borrowed-text-entry/initializer-source-identity");
    let ExpressionPlanRootV1::LocalInitializer { initializer, .. } = plan.root() else { return Err(reject()) };
    if input.owner() != plan.owner() || site.owner() != input.owner()
        || initializer.initializer_site() != Some(site.site()) || root.owner() != input.owner()
        || binding.owner() != input.owner()
        || input.function().expression_source().initializers().filter(|row| *row == initializer).count() != 1
        || input.function().variable_ref(read) != Some(ResolvedLexicalRefV1::Local(binding)) {
        return Err(reject());
    }
    let Some(flow) = input.function().expression_source().value_flow() else { return Ok(None) };
    let Some(value) = flow.read(read, binding) else { return Ok(None) };
    if !entry_origin(input, flow, value, root) { return Ok(None); }
    let Some(sum) = prepared.lend_source_value_sum_v1(root) else { return Ok(None) };
    // Every original incoming alternative must be Text. Empty/unknown/mixed is
    // never narrowed using one caller, one literal or the local MIR carrier.
    if sum.alternatives().len() != 1 || !sum.alternatives().contains(&SourceValueAlternativeV1::Text) { return Ok(None); }
    let requirement = super::super::super::borrowed_formal_entry::issue_text_entry_read_requirement_v1(
        contracts, plan, read, binding, root)?;
    Ok(Some(InitializerTextEntryReadV1 { requirement, sum, flow: std::sync::Arc::clone(flow), value }))
}
