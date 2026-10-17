//! Passive original local-New identity, shared by field and stored-call roots.
//! This lends neither construction eligibility, liveness, nor ownership.
use super::*;

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct LocalNewRootSourceV1 {
    acquisition: OwnedExprSiteV1,
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    object: CanonicalObjectIdV1,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn recheck_stored_local_new_parent_v1(
        &self, target: &super::super::LexicalInstanceCallSourceTargetV1,
    ) -> Result<(), String> {
        let ingress = self.borrowed_formal_source.as_ref().ok_or("[freeze:contract][stored-child/source-missing]")?
            .as_ref().map_err(Clone::clone)?;
        let Some((slot, root)) = ingress.stored_local_new_root_v1(target)? else { return Ok(()); };
        let mut callers = self.original_parameter_contracts.as_ref().ok_or("[freeze:contract][stored-child/caller-catalog-missing]")?
            .iter().filter(|row| row.owner == target.call_site().owner());
        let caller = callers.next().ok_or("[freeze:contract][stored-child/caller-missing]")?;
        if callers.next().is_some() || caller.batch_slot != slot
            || self.original_parameter_contracts.as_ref().unwrap().iter().filter(|row| row.batch_slot == slot).count() != 1 {
            return Err("[freeze:contract][stored-child/caller-drift]".into());
        }
        root.recheck(self)
    }
}
impl PartialEq for LocalNewRootSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        self.acquisition == other.acquisition && self.binding == other.binding
            && self.declaration == other.declaration && self.object == other.object
            && self.box_source.same_source_as(&other.box_source)
    }
}
impl Eq for LocalNewRootSourceV1 {}
impl LocalNewRootSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn from_candidate(candidate: &OrdinaryNewCandidate) -> Rc<Self> {
        Rc::new(Self { acquisition: candidate.site.clone(), binding: candidate.destination,
            declaration: candidate.declaration.clone(), box_source: candidate.box_source.clone(), object: candidate.object })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn binding(&self) -> BindingRefV1 { self.binding }
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str { self.box_source.name() }
    pub(in crate::mir::normal_callable_semantic_package) fn object(&self) -> CanonicalObjectIdV1 { self.object }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_definition(&self,
        source: &crate::parser::ParserOrdinaryBoxSourceRowV1, object: CanonicalObjectIdV1) -> bool {
        self.box_source.same_source_as(source) && self.object == object
    }
    pub(in crate::mir::normal_callable_semantic_package) fn corroborate_input(
        &self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        parent: &SourceExprSiteV1,
    ) -> Result<(), String> {
        let reject = || "[freeze:contract][local-new-source/original-input]".to_owned();
        let function = input.function();
        let mut rows = function.expression_source().initializers().filter(|row| row.binding() == self.binding);
        let row = rows.next().ok_or_else(reject)?;
        if input.owner() != self.acquisition.owner() || self.binding.owner() != input.owner()
            || function.variable_ref(parent) != Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(self.binding))
            || !matches!(function.binding(self.binding).map(|row| row.kind()), Some(crate::mir::resolved_semantics::BindingKindV1::Local { .. }))
            || rows.next().is_some() || row.initializer_site() != Some(self.acquisition.site())
            || row.declaration_site() != &self.declaration
            || function.declaration_binding(&self.declaration) != Some(self.binding)
            || function.assignment_targets().any(|(_, target)| matches!(target,
                crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(binding) if *binding == self.binding)) {
            return Err(reject());
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        ledger.with_local_new_source_v1(&self.acquisition, self.binding, |source| {
            if !source.box_source.same_source_as(&self.box_source) || source.object != self.object || source.declaration != &self.declaration {
                return Err("[freeze:contract][local-new-source/original-row-drift]".into());
            }
            Ok(())
        })?.ok_or_else(|| "[freeze:contract][local-new-source/original-row-missing]".to_owned())
    }
}
