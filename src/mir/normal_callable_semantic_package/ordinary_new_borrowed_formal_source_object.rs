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

impl PreparedBorrowedFormalIngressV1 {
    /// Original field evidence selects input work only. It cannot lend a live
    /// operand, acquire a Home or qualify the callee's result.
    pub(in crate::mir::normal_callable_semantic_package) fn original_map_field_argument_request_v1(
        &self, call: &OwnedExprSiteV1,
    ) -> Result<bool, String> {
        self.map_field_input_request_v1(call, None)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_map_field_actual_request_v1(
        &self, call: &OwnedExprSiteV1, ordinal: u32, site: &SourceExprSiteV1,
    ) -> Result<bool, String> {
        self.map_field_input_request_v1(call, Some((ordinal, site)))
    }
    fn map_field_input_request_v1(
        &self, call: &OwnedExprSiteV1, actual: Option<(u32, &SourceExprSiteV1)>,
    ) -> Result<bool, String> {
        let seeds = &self.original_formal_seeds;
        let mut rows = seeds.rows.iter().filter(|row| &row.incoming.call == call);
        let Some(row) = rows.next() else { return Ok(false); };
        if !row.seeds.iter().any(|seed| matches!(seed, OriginalFormalActualSeedV1::MapField { .. })) { return Ok(false); }
        if rows.next().is_some() || !seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("borrowed-view/original-seed-identity"));
        }
        let target = row.incoming.source.original_instance_reference_v1()
            .ok_or_else(|| freeze("borrowed-object/source-actual-identity"))?;
        if &target.call_site != call || target.callee_owner != row.incoming.callee {
            return Err(freeze("borrowed-object/source-actual-identity"));
        }
        let mut selected = false;
        for ((ordinal, site, formal), seed) in row.incoming.arguments.iter().zip(&row.seeds) {
            let OriginalFormalActualSeedV1::MapField { read, get, source } = seed else { continue; };
            if actual.is_some_and(|(expected, _)| expected != *ordinal) { continue; }
            if actual.is_some_and(|(_, expected)| expected != site) {
                return Err(freeze("borrowed-object/source-actual-identity"));
            }
            if read != &OwnedExprSiteV1::new(call.owner(), site.clone())
                || target.argument_sites.get(*ordinal as usize) != Some(site)
                || formal.owner() != row.incoming.callee || get.get().owner() != call.owner()
                || source.field().object() != get.payload_source_v1().payload_object_v1() {
                return Err(freeze("borrowed-object/source-actual-identity"));
            }
            // An unknown source projection may request no executable work.
            selected |= seeds.source_objects.as_ref().is_some_and(|domain| domain.views.contains_key(formal));
        }
        Ok(selected)
    }
    /// Merge corroborated passive class evidence from selected source inputs
    /// and the original all-incoming domain. Neither is an execution capability.
    /// If both are known they must name the same canonical object.
    pub(in crate::mir::normal_callable_semantic_package) fn field_source_object_view_v1(
        &self, formal: BindingRefV1,
    ) -> Result<Option<&BorrowedFormalObjectViewV1>, String> {
        if self.source_definition_for(formal.owner()).is_none() { return Ok(None); }
        let selected = self.source_object_view_v1(formal)?;
        let seeds = &self.original_formal_seeds;
        if !seeds.corroborate_original_v1(&self.source_incoming) {
            return Err(freeze("borrowed-view/original-seed-identity"));
        }
        let original = seeds.source_objects.as_ref().and_then(|domain| domain.views.get(&formal));
        match (selected, original) {
            (Some(a), Some(b)) if a.class() != b.class() || a.object() != b.object() =>
                Err(freeze("borrowed-view/declared-object-class")),
            (Some(view), _) | (_, Some(view)) => Ok(Some(view)),
            (None, None) => Ok(None),
        }
    }
}
