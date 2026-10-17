//! Immutable exact Integer field-read source, retained by the existing result facts.
//! The wholebox census remains the sole store authority; no physical read permission.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CheckedFieldArrayReadSourceV1 {
    read: Rc<super::super::super::super::CheckedFieldArrayGetSourceV1>,
    return_sites: Box<[OwnedExprSiteV1]>,
    local_origin: Option<(BindingRefV1, SourceBindingSiteV1)>,
}
impl CheckedFieldArrayReadSourceV1 {
    pub(crate) fn read(&self) -> &Rc<super::super::super::super::CheckedFieldArrayGetSourceV1> { &self.read }
    pub(crate) fn return_sites(&self) -> &[OwnedExprSiteV1] { &self.return_sites }
}
impl std::ops::Deref for CheckedFieldArrayReadSourceV1 {
    type Target = super::super::super::super::CheckedFieldArrayGetSourceV1;
    fn deref(&self) -> &Self::Target { &self.read }
}
impl CheckedFieldArrayReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_census(
        &self, owner: FunctionOwnerIdV1, returns: &[OwnedExprSiteV1],
        census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    ) -> Result<(), String> {
        self.read.recheck_census(owner, census)?;
        if self.return_sites.iter().any(|site| site.owner() != owner)
            || self.return_sites.is_empty() || self.return_sites.iter().any(|site| !returns.contains(site))
        {
            return Err(freeze("borrowed-result/array-read-source-drift"));
        }
        Ok(())
    }
}

/// One physical read may ground several original Return expressions.
/// Normalize by exact call while retaining every Return relation.
pub(super) fn normalize_reads(
    reads: Vec<Rc<CheckedFieldArrayReadSourceV1>>,
) -> Result<Box<[Rc<CheckedFieldArrayReadSourceV1>]>, String> {
    let mut unique = BTreeMap::<OwnedExprSiteV1, Rc<CheckedFieldArrayReadSourceV1>>::new();
    for read in reads {
        match unique.entry(read.call().clone()) {
            std::collections::btree_map::Entry::Vacant(slot) => { slot.insert(read); }
            std::collections::btree_map::Entry::Occupied(mut slot) => {
                let mut comparable = read.as_ref().clone();
                comparable.return_sites = slot.get().return_sites.clone();
                if &comparable != slot.get().as_ref() {
                    return Err(freeze("borrowed-result/array-read-source-conflict"));
                }
                let current = Rc::make_mut(slot.get_mut());
                let mut sites = current.return_sites.to_vec();
                for site in read.return_sites.iter() {
                    if sites.contains(site) {
                        return Err(freeze("borrowed-result/array-read-return-duplicate"));
                    }
                    sites.push(site.clone());
                }
                current.return_sites = sites.into_boxed_slice();
            }
        }
    }
    Ok(unique.into_values().collect())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn capture_field_array_get(
    input: ResolvedFunctionLoweringInputV1<'_>,
    contract: &OwnedCallableParameterContractDeclarationV1,
    site: &SourceExprSiteV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    proven_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    original: Option<&super::super::super::borrowed_formal_source::OriginalBorrowedFormalSourceV1>,
) -> Result<Option<Rc<CheckedFieldArrayReadSourceV1>>, String> {
    let function = input.function();
    // The return source is either the `get` itself or a local bound by a
    // single, never-rebound initializer — the same shape `capture_call`
    // demands of a local source.
    let mut local_origin = None;
    let call_site = if function.method_calls().any(|(actual, _)| actual == site) {
        site.clone()
    } else {
        let Some(ResolvedLexicalRefV1::Local(binding)) = function.variable_ref(site) else {
            return Ok(None);
        };
        let Some((declaration, call)) = super::super::super::super::field_array_read_source::local_initializer(input, binding)
            else { return Ok(None) };
        local_origin = Some((binding, declaration));
        call
    };
    let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller)) =
        selected.key_for_batch_slot(contract.batch_slot) else { return Ok(None) };
    let held = original.map(|original| original.original_new_array_get_v1(
        &OwnedExprSiteV1::new(input.owner(), call_site.clone()))).transpose()?.flatten();
    let read = match held {
        Some(read) => Some(read),
        None => super::super::super::super::field_array_read_source::capture_exact_get(
            input, caller, &call_site, selected, batch, constructors, proven_fields)?,
    };
    let Some(read) = read else { return Ok(None) };
    Ok(Some(Rc::new(CheckedFieldArrayReadSourceV1 {
        read, return_sites: Box::new([OwnedExprSiteV1::new(input.owner(), site.clone())]), local_origin,
    })))
}

// Home observation consumes only original source evidence, before Completion exists.
impl BorrowedI64ResultSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_field_array_return_v1(
        &self, input: ResolvedFunctionLoweringInputV1<'_>, site: &OwnedExprSiteV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
        census: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    ) -> Result<bool, String> {
        let mut reads = self.field_array_reads.iter().filter(|read| read.return_sites().contains(site));
        let Some(read) = reads.next() else { return Ok(false) };
        if reads.next().is_some() { return Err(freeze("borrowed-result/array-return-duplicate")); }
        self.require_source_i64_v1()?;
        let expected = crate::mir::resolved_semantics::SourcePathV1::from_node(exit.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Value).expr();
        if site.owner() != input.owner() || &expected != site.site()
            || !input.body_shape().and_then(|shape|
                super::super::super::super::verified_value_return_sites(input, shape))
                .is_some_and(|sites| sites.contains(site.site()))
        { return Err(freeze("borrowed-result/array-return-identity")); }
        read.recheck_census(input.owner(), &self.returns, census)?;
        Ok(true)
    }
}
