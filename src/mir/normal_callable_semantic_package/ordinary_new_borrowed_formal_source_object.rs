//! Original all-incoming object agreement. This passive loan grants neither an
//! executable entry nor a live receiver; selected field-write packets own those.
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{ResolvedAssignmentTargetV1, SourceBindingSiteV1};

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct OriginalFormalObjectBasisV1 {
    domain: Rc<SourceObjectDomainV1>,
    binding: BindingRefV1,
    formal: BindingRefV1,
    ordinal: u32,
    site: OwnedExprSiteV1,
}
impl OriginalFormalObjectBasisV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn matches_source_receiver_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceExprSiteV1,
    ) -> bool {
        self.binding.owner() == owner && self.site == OwnedExprSiteV1::new(owner, site.clone())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str {
        self.domain.views[&self.formal].class()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn object(
        &self,
    ) -> hakorune_mir_defs::CanonicalObjectIdV1 {
        self.domain.views[&self.formal].object()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn ready(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
    ) -> bool {
        let seeds = &source.original_formal_seeds;
        seeds.corroborate_original_v1(&source.source_incoming)
            && seeds
                .source_objects
                .as_ref()
                .is_some_and(|current| Rc::ptr_eq(current, &self.domain))
            && self
                .domain
                .views
                .get(&self.formal)
                .is_some_and(|view| view.class() == self.class() && view.object() == self.object())
            && self.binding.owner() == self.formal.owner()
            && source
                .source_incoming
                .exact_rows()
                .filter(|row| row.callee == self.formal.owner())
                .all(|row| {
                    row.arguments.iter().any(|(ordinal, _, formal)| {
                        *ordinal == self.ordinal && *formal == self.formal
                    })
                })
    }
}
impl OriginalBorrowedFormalSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn formal_object_source_at_v1(
        &self,
        input: ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
        definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
        contracts: &[OwnedCallableParameterContractDeclarationV1],
    ) -> Option<OriginalFormalObjectBasisV1> {
        if !self.seeds.corroborate_original_v1(&self.inventory) {
            return None;
        }
        let binding = arg_site_binding(input, site)?;
        let formal = *definitions.get(&input.owner())?.origins.get(&binding)?;
        let mut declarations = contracts.iter().filter(|row| row.owner == input.owner());
        let declaration = declarations.next()?;
        if declarations.next().is_some()
            || declaration.mode != CallableParameterDeclarationModeV1::InstanceBoxMethod
            || binding.owner() != input.owner()
            || formal.owner() != input.owner()
        {
            return None;
        }
        let mut parameters = declaration
            .parameters
            .iter()
            .filter(|row| row.binding == formal);
        let parameter = parameters.next()?;
        if parameters.next().is_some() || !parameter.kind.is_ordinary_borrowed_handle()
            || input.function().declaration_binding(&SourceBindingSiteV1::Parameter { index: parameter.ordinal }) != Some(formal)
            || [binding, formal].iter().any(|binding|
                input.function().assignment_targets().any(|(_, target)| matches!(target,
                    ResolvedAssignmentTargetV1::BindingRebind(actual) if actual == binding))
                || input.forest().semantic_owners().any(|(_, function)| function.assignment_targets().any(|(_, target)|
                    matches!(target, ResolvedAssignmentTargetV1::UpvarRebind(upvar) if upvar.source() == *binding))))
        { return None; }
        let domain = self.seeds.source_objects.as_ref()?;
        domain.views.get(&formal)?;
        Some(OriginalFormalObjectBasisV1 {
            domain: Rc::clone(domain),
            binding,
            formal,
            ordinal: parameter.ordinal,
            site: OwnedExprSiteV1::new(input.owner(), site.clone()),
        })
    }
}

#[cfg(test)]
impl PreparedBorrowedFormalIngressV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_object_class_for_test(
        &self,
        formal: BindingRefV1,
    ) -> Option<&str> {
        self.original_formal_seeds
            .corroborate_original_v1(&self.source_incoming)
            .then(|| {
                self.original_formal_seeds
                    .source_objects
                    .as_ref()?
                    .views
                    .get(&formal)
                    .map(|view| view.class())
            })
            .flatten()
    }
}
