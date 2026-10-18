//! Required source Text read, fulfilled only by a later finished borrowed entry.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BorrowedTextEntryReadRequirementV1 {
    original: super::super::borrowed_formal_uses::OriginalFormalReadLoanV1,
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
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn corroborates_original_uses_v1(
        &self, uses: &Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>,
    ) -> bool {
        uses.lend_read(&self.read, self.binding, self.formal).as_ref() == Some(&self.original)
    }
    pub(crate) fn source_plan(&self) -> &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1> { &self.source_plan }
    pub(crate) fn node(&self) -> usize { self.node }
    pub(crate) fn read(&self) -> &SourceExprSiteV1 { &self.read }
    pub(crate) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(crate) fn formal(&self) -> (u32, BindingRefV1) { (self.ordinal, self.formal) }
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn issue_text_entry_read_requirement_v1(
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>, read: &SourceExprSiteV1,
    binding: BindingRefV1, root: BindingRefV1,
    original: super::super::borrowed_formal_uses::OriginalFormalReadLoanV1,
) -> Result<BorrowedTextEntryReadRequirementV1, String> {
    let reject = || freeze("borrowed-text-entry/source-identity");
    if binding.owner() != plan.owner() || root.owner() != binding.owner()
        || !original.matches(read, binding, root) {
        return Err(reject());
    }
    let node = plan.nodes().iter().position(|node| node.site() == read
        && matches!(node.source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(original)) if *original == binding)).ok_or_else(reject)?;
    let mut matching = contracts.iter().filter(|row| row.owner == binding.owner());
    let declaration = matching.next().ok_or_else(reject)?;
    if matching.next().is_some() || !original.matches_contract(declaration) { return Err(reject()); }
    let mut formals = declaration.parameters.iter().filter(|row| row.binding == root);
    let formal = formals.next().ok_or_else(reject)?;
    if formals.next().is_some() || !matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
        || declaration.parameters.get(formal.ordinal as usize).is_none_or(|row| row.binding != root) { return Err(reject()); }
    Ok(BorrowedTextEntryReadRequirementV1 { original, source_plan: Rc::clone(plan), node, read: read.clone(), binding,
        formal: root, ordinal: formal.ordinal, batch_slot: declaration.batch_slot,
        mode: declaration.mode, kind: formal.kind.clone() })
}

impl OrdinaryNewClaimLedgerV1 {
    /// Corroborate a required Text read against the installed physical entry.
    /// Structural source domains avoid the caller-actual/completed-entry cycle.
    /// This lends only a ValueId correspondence, not a String operand, entry
    /// completion, backend marker or original-use coverage/publication proof.
    pub(in crate::mir) fn validate_prepared_text_entry_read_v1(
        &self, requirement: &BorrowedTextEntryReadRequirementV1,
        function: &crate::mir::MirFunction, read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<crate::mir::ValueId, String> {
        let (_, value, domain) = self.validate_prepared_finite_entry_read_v1(requirement, function, read)?;
        if !domain.text_only() { return Err(freeze("borrowed-text-entry/domain-not-text-only")); }
        Ok(value)
    }

    /// Original finite domain is preserved independently of a purpose's view.
    /// Keep formal and current values distinct: an alias may have a real Copy.
    pub(in crate::mir) fn validate_prepared_finite_entry_read_v1(
        &self, requirement: &BorrowedTextEntryReadRequirementV1,
        function: &crate::mir::MirFunction, read: &crate::mir::builder::ExactLexicalReadV1,
    ) -> Result<(crate::mir::ValueId, crate::mir::ValueId, Rc<super::super::borrowed_formal_source::SourceValueSumV1>), String> {
        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
        use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
        let owner = requirement.source_plan.owner();
        let source = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-text-entry/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("borrowed-text-entry/catalog-missing"))?;
        let mut declarations = catalog.iter().filter(|row| row.owner == owner);
        let declaration = declarations.next().ok_or_else(|| freeze("borrowed-text-entry/declaration-missing"))?;
        if declarations.next().is_some() || requirement.batch_slot != declaration.batch_slot
            || requirement.mode != declaration.mode || requirement.mode != Mode::StaticBoxMethod
            || !requirement.original.matches_contract(declaration)
            || source.original_formal_read_v1(owner, &requirement.read, requirement.binding, requirement.formal)? != requirement.original
            || declaration.parameters.get(requirement.ordinal as usize).is_none_or(|row|
                row.ordinal != requirement.ordinal || row.binding != requirement.formal || row.kind != requirement.kind)
            || !matches!(requirement.kind, CallableParameterContractKindV1::OpaqueHandle)
            || requirement.source_plan.nodes().get(requirement.node).is_none_or(|node|
                node.site() != &requirement.read || !matches!(node.source(), ExpressionNodeSourceV1::Reference(
                    crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == requirement.binding)) {
            return Err(freeze("borrowed-text-entry/original-read-identity"));
        }
        let structure = self.finite_text_entry_for_owner_v1(owner)?
            .ok_or_else(|| freeze("borrowed-text-entry/finite-structure-required"))?;
        let entries = self.borrowed_ordinary_entry_values_v1(owner)?;
        structure.check_finite_entry_values_v1(&entries)?;
        let domain = structure.finite_domain_for_formal_v1(requirement.ordinal, requirement.formal)?;
        let canonical = source.lend_source_value_sum_v1(requirement.formal)
            .ok_or_else(|| freeze("borrowed-text-entry/domain-missing"))?;
        if !Rc::ptr_eq(domain, &canonical) { return Err(freeze("borrowed-text-entry/domain-source-drift")); }
        let mut formals = entries.iter().filter(|row| row.0 == requirement.ordinal && row.1 == requirement.formal);
        let entry_value = formals.next().ok_or_else(|| freeze("borrowed-text-entry/formal-value-missing"))?.2;
        if formals.next().is_some() { return Err(freeze("borrowed-text-entry/formal-value-duplicate")); }
        let carriers = function.metadata.physical_param_carriers.as_deref()
            .ok_or_else(|| freeze("borrowed-text-entry/carriers-missing"))?;
        if function.params.len() != declaration.parameters.len()
            || function.signature.params.len() != function.params.len() || carriers.len() != function.params.len()
            || function.params.get(requirement.ordinal as usize) != Some(&entry_value)
            || entries.iter().any(|(ordinal, _, value)| function.params.get(*ordinal as usize) != Some(value)
                || carriers.get(*ordinal as usize) != Some(&Carrier::BorrowedTaggedValue)
                || !matches!(function.signature.params.get(*ordinal as usize), Some(crate::mir::MirType::Integer | crate::mir::MirType::Unknown)))
            || carriers.iter().enumerate().filter_map(|(index, carrier)|
                (*carrier == Carrier::BorrowedTaggedValue).then_some(index)).collect::<BTreeSet<_>>()
                != entries.iter().map(|(ordinal, _, _)| *ordinal as usize).collect::<BTreeSet<_>>()
            || function.metadata.declared_param_decls.iter().any(|row| row.implicit_receiver)
            || function.params.iter().copied().collect::<BTreeSet<_>>().len() != function.params.len()
            || function.blocks.values().flat_map(|block| block.all_instructions()).any(|row|
                row.dst_value().is_some_and(|dst| function.params.contains(&dst))) {
            return Err(freeze("borrowed-text-entry/physical-formal"));
        }
        let value = read.value_for(owner, requirement.read.node(), requirement.binding).map_err(|error| error.to_string())?;
        for (block, instruction) in read.loan_forwarded_copies(requirement.formal, entry_value)? {
            let dst = instruction.dst_value().ok_or_else(|| freeze("borrowed-text-entry/copy-dst"))?;
            let mut definitions = function.blocks.iter().flat_map(|(block, body)|
                body.all_instructions().map(move |row| (*block, row))).filter(|(_, row)| row.dst_value() == Some(dst));
            if definitions.next() != Some((*block, instruction)) || definitions.next().is_some() {
                return Err(freeze("borrowed-text-entry/physical-copy"));
            }
        }
        Ok((entry_value, value, Rc::clone(domain)))
    }
}
