//! Required source Text read, fulfilled only by a later finished borrowed entry.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedTextEntryReadRequirementV1 {
    source_plan: Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
    node: usize,
    read: SourceExprSiteV1,
    binding: BindingRefV1,
    formal: BindingRefV1,
    ordinal: u32,
    batch_slot: u32,
    mode: crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1,
    kind: CallableParameterContractKindV1,
}
impl BorrowedTextEntryReadRequirementV1 {
    pub(crate) fn source_plan(&self) -> &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1> { &self.source_plan }
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn formal(&self) -> (u32, BindingRefV1) { (self.ordinal, self.formal) }
}

pub(in crate::mir::normal_callable_semantic_package) fn issue_text_entry_read_requirement_v1(
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, read: &SourceExprSiteV1,
    binding: BindingRefV1, root: BindingRefV1,
) -> Result<BorrowedTextEntryReadRequirementV1, String> {
    let reject = || freeze("borrowed-text-entry/source-identity");
    if binding.owner() != plan.owner() || root.owner() != binding.owner() {
        return Err(reject());
    }
    let node = plan.nodes().iter().position(|node| node.site() == read
        && matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(original)) if *original == binding)).ok_or_else(reject)?;
    let mut matching = contracts.iter().filter(|row| row.owner == binding.owner());
    let declaration = matching.next().ok_or_else(reject)?;
    if matching.next().is_some() { return Err(reject()); }
    let mut formals = declaration.parameters.iter().filter(|row| row.binding == root);
    let formal = formals.next().ok_or_else(reject)?;
    if formals.next().is_some() || !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
        || declaration.parameters.get(formal.ordinal as usize).is_none_or(|row| row.binding != root) { return Err(reject()); }
    Ok(BorrowedTextEntryReadRequirementV1 { source_plan: Rc::clone(plan), node, read: read.clone(), binding,
        formal: root, ordinal: formal.ordinal, batch_slot: declaration.batch_slot,
        mode: declaration.mode, kind: formal.kind.clone() })
}
