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
        let owner = requirement.source_plan.owner();
        let source = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-text-entry/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("borrowed-text-entry/catalog-missing"))?;
        let mut declarations = catalog.iter().filter(|row| row.owner == owner);
        let declaration = declarations.next().ok_or_else(|| freeze("borrowed-text-entry/declaration-missing"))?;
        if declarations.next().is_some() || requirement.batch_slot != declaration.batch_slot
            || requirement.mode != declaration.mode
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
        let parameters: Vec<_> = declaration.parameters.iter().map(|row|
            (row.ordinal, row.binding, row.kind.clone())).collect();
        let structure = self.borrowed_entry_structure_for_contract(owner, &parameters)?
            .ok_or_else(|| {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[input-domain] read owner={owner:?} ordinal={} binding={:?} formal={:?} read={:?}",
                        requirement.ordinal, requirement.binding, requirement.formal, requirement.read);
                }
                freeze("borrowed-text-entry/finite-structure-required")
            })?;
        let entries = self.borrowed_ordinary_entry_values_v1(owner)?;
        structure.check_finite_entry_values_v1(&entries)?;
        let domain = structure.source_domain_for_formal_v1(requirement.ordinal, requirement.formal)?;
        let canonical = source.lend_source_value_sum_v1(requirement.formal)
            .ok_or_else(|| freeze("borrowed-text-entry/domain-missing"))?;
        if !Rc::ptr_eq(&domain, &canonical) { return Err(freeze("borrowed-text-entry/domain-source-drift")); }
        let mut formals = entries.iter().filter(|row| row.0 == requirement.ordinal && row.1 == requirement.formal);
        let entry_value = formals.next().ok_or_else(|| freeze("borrowed-text-entry/formal-value-missing"))?.2;
        if formals.next().is_some() { return Err(freeze("borrowed-text-entry/formal-value-duplicate")); }
        let carriers = function.metadata.physical_param_carriers.as_deref()
            .ok_or_else(|| freeze("borrowed-text-entry/carriers-missing"))?;
        // Static's existing source-only analysis law has no executable
        // signature requirement. Actual String reads separately demand the
        // canonical layout; this loan still issues no operand permission.
        let source_layout = |ordinal: u32, formal: BindingRefV1| match declaration.mode {
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::StaticBoxMethod =>
                Ok((ordinal as usize, declaration.parameters.len(), 0, None)),
            crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod =>
                self.original_entry_layout_v1(formal, formal, |kind| kind.is_ordinary_borrowed_handle())
                    .map(|(index, count, receivers, symbol)| (index, count, receivers, Some(symbol))),
        };
        let (index, count, receivers, symbol) = source_layout(requirement.ordinal, requirement.formal)?;
        let mut borrowed_lanes = BTreeSet::new();
        for (ordinal, formal, value) in entries.iter() {
            let (lane, lanes, receiver_count, name) = source_layout(*ordinal, *formal)?;
            if declaration.parameters.get(*ordinal as usize).is_none_or(|row|
                row.ordinal != *ordinal || row.binding != *formal || !row.kind.is_ordinary_borrowed_handle())
                || lanes != count || receiver_count != receivers || name != symbol
                || function.params.get(lane) != Some(value)
                || carriers.get(lane) != Some(&Carrier::BorrowedTaggedValue)
                || !matches!(function.signature.params.get(lane), Some(crate::mir::MirType::Integer | crate::mir::MirType::Unknown))
                || !borrowed_lanes.insert(lane) {
                return Err(freeze("borrowed-text-entry/physical-formal"));
            }
        }
        if symbol.as_ref().is_some_and(|symbol| &function.signature.name != symbol) || function.params.len() != count
            || function.signature.params.len() != count || carriers.len() != count
            || function.params.get(index) != Some(&entry_value)
            || receivers == 1 && function.metadata.declared_param_decls.len() != count
            || function.metadata.declared_param_decls.iter().enumerate().any(|(index, row)|
                row.implicit_receiver != (receivers == 1 && index == 0))
            || receivers == 1 && carriers.first() != Some(&Carrier::ExistingCallableI64)
            || carriers.iter().enumerate().filter_map(|(index, carrier)|
                (*carrier == Carrier::BorrowedTaggedValue).then_some(index)).collect::<BTreeSet<_>>() != borrowed_lanes
            || function.params.iter().copied().collect::<BTreeSet<_>>().len() != count
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
        Ok((entry_value, value, domain))
    }
}

impl OrdinaryNewClaimLedgerV1 {
/// Ephemeral source-to-physical correspondence from the SAME retained signature:
/// (formal lane, total physical lanes, receiver lanes, canonical symbol).
pub(in crate::mir) fn original_entry_layout_v1(&self, binding: crate::mir::resolved_semantics::BindingRefV1, formal: crate::mir::resolved_semantics::BindingRefV1,
    accepts: impl FnOnce(&crate::mir::callable_parameter_contract::CallableParameterContractKindV1) -> bool,
) -> Result<(usize, usize, usize, String), String> {
    use crate::mir::callable_parameter_contract::{CallableParameterDeclarationModeV1 as Mode};
    use crate::mir::normal_callable_semantic_package::PhysicalCallableLaneRoleV1 as Role;
    let ledger = self;
    let catalog = ledger.original_parameter_contracts.as_deref().ok_or_else(|| freeze("expression/scalar-entry-catalog"))?;
    let mut declarations = catalog.iter().filter(|row| row.owner == binding.owner());
    let declaration = declarations.next().ok_or_else(|| freeze("expression/scalar-entry-owner"))?;
    if declarations.next().is_some() { return Err(freeze("expression/scalar-entry-owner")); }
    let (receivers, namespace) = match declaration.mode {
        Mode::StaticBoxMethod => (0, hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod),
        Mode::InstanceBoxMethod => (1, hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod),
    };
    let mut rows = declaration.parameters.iter().filter(|row| row.binding == formal);
    let row = rows.next().ok_or_else(|| freeze("expression/scalar-entry-formal"))?;
    if rows.next().is_some() || declaration.parameters.get(row.ordinal as usize).is_none_or(|original| original.binding != formal)
        || !accepts(&row.kind) { return Err(freeze("expression/scalar-entry-contract")); }
    let (selected, signatures) = ledger.original_instance_input_issuance.as_ref()
        .ok_or_else(|| freeze("expression/scalar-entry-issuance"))?;
    let signature = signatures.row(declaration.batch_slot).ok_or_else(|| freeze("expression/scalar-entry-signature"))?;
    let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key)) = selected.key_for_batch_slot(declaration.batch_slot)
        else { return Err(freeze("expression/scalar-entry-key")) };
    if signature.owner() != binding.owner() || signature.mode() != declaration.mode
        || key.namespace() != namespace || key.arity() as usize != declaration.parameters.len()
        || !selected.identity_for_batch_slot(declaration.batch_slot).is_some_and(|identity| signature.identity().same_as(identity))
        || signature.source_logical_arity() as usize != declaration.parameters.len()
        || signature.receiver_lane_count() as usize != receivers
        || signature.lanes().iter().filter(|lane| lane.role() == Role::InstanceReceiver).count() != receivers
        || receivers == 1 && signature.lanes().first().is_none_or(|lane| lane.index() != 0
            || lane.role() != Role::InstanceReceiver || lane.logical_ordinal().is_some() || lane.binding().owner() != binding.owner()) {
        return Err(freeze("expression/scalar-entry-source-signature"));
    }
    let mut lanes = signature.lanes().iter().filter(|lane| lane.binding() == formal
        && lane.logical_ordinal() == Some(row.ordinal) && lane.role() == Role::OrdinaryScalar);
    let lane = lanes.next().ok_or_else(|| freeze("expression/scalar-entry-formal-lane"))?;
    if lanes.next().is_some() || signature.lanes().get(lane.index() as usize) != Some(lane) {
        return Err(freeze("expression/scalar-entry-formal-lane"));
    }
    Ok((lane.index() as usize, signature.physical_callable_lane_count() as usize, receivers, key.mir_symbol_projection()))
}

}
