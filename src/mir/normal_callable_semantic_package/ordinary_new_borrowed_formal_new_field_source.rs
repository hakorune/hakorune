//! A numeric dependency loan from the sole original New census. This never
//! promotes Conflict in the cached formal domain or supplies executable input.
use super::*;
use std::rc::Rc;

type FieldKeyV1 = (Box<str>, Box<str>);

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) struct OriginalNewI64InputsLoanV1 {
    original: Rc<OriginalNewSeedsV1>,
    constructor: ConstructorSourceIdV1,
    formal: BindingRefV1,
    ordinal: usize,
    rows: Box<[usize]>,
    field_rows: Box<[usize]>,
    array_rows: Box<[usize]>,
    origins: Box<[BindingRefV1]>,
}
impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn lend_i64_inputs_v1(
        self: &Rc<Self>,
        constructor: &ConstructorSourceIdV1,
        formal: BindingRefV1,
        constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
        current_i64: &BTreeSet<BindingRefV1>,
    ) -> Option<OriginalNewI64InputsLoanV1> {
        if self.incomplete || !self.current_provider_sources_match(constructors) { return None; }
        let mut targets = constructors.rows().iter().filter(|row| row.source_id() == constructor);
        let target = targets.next()?;
        if targets.next().is_some() || target.kind() != ConstructorSourceKindV1::Birth
            || target.forest().roots() != [formal.owner()] { return None; }
        let ordinal = target.formal_contracts().iter().position(|row| row.binding() == formal)?;
        if target.formal_contracts()[ordinal].ordinal() as usize != ordinal { return None; }
        let mut rows = Vec::new();
        let mut origins = Vec::new();
        let mut field_rows = Vec::new();
        let mut array_rows = Vec::new();
        // Filter only by source class first: an unavailable/other constructor
        // of this class is a veto, never an omitted incoming construction.
        for (index, row) in self.rows.iter().enumerate().filter(|(_, row)| row.original.class() == target.box_name()) {
            let OriginalNewBirthV1::Birth { source, owner, formals } = &row.birth else { return None; };
            if source != constructor || *owner != formal.owner()
                || row.original.site() != row.site.site()
                || !row.original.field_initializers().is_empty()
                || formals.iter().copied().ne(target.formal_contracts().iter().map(|row| row.binding()))
                || formals.get(ordinal) != Some(&formal)
                || row.seeds.len() != formals.len() || row.fields.len() != formals.len() || row.array_reads.len() != formals.len()
                || row.original.arguments().len() != formals.len() { return None; }
            match &row.seeds[ordinal] {
                OriginalFormalActualSeedV1::ProviderStatic(loan)
                    if loan.matches_original(&row.site, &row.original, ordinal)
                        && loan.matches_constructor(constructors) => {},
                seed => match seed.project_source_integer_v1(true) {
                    FormalActualSeedV1::Integer => {},
                    FormalActualSeedV1::Forward(origin) | FormalActualSeedV1::ForwardIntegerOnly(origin)
                        if current_i64.contains(&origin) => origins.push(origin),
                    FormalActualSeedV1::Conflict => {
                        match (&row.fields[ordinal], &row.array_reads[ordinal]) {
                            (Some(field), None) => {
                                if row.original.arguments().get(ordinal) != Some(&field.site)
                                    || field.receiver.owner() != row.site.owner() { return None; }
                                field_rows.push(index);
                            }
                            (None, Some(read)) if read.binding().owner() == row.site.owner()
                                && read.read().call().owner() == row.site.owner() => array_rows.push(index),
                            _ => return None,
                        }
                    }
                    _ => return None,
                },
            }
            rows.push(index);
        }
        if rows.is_empty() { return None; }
        Some(OriginalNewI64InputsLoanV1 { original: Rc::clone(self), constructor: constructor.clone(),
            formal, ordinal, rows: rows.into_boxed_slice(), field_rows: field_rows.into_boxed_slice(), array_rows: array_rows.into_boxed_slice(), origins: origins.into_boxed_slice() })
    }
}
impl OriginalNewI64InputsLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn fields(&self) -> impl Iterator<Item = FieldKeyV1> + '_ {
        self.field_rows.iter().filter_map(|index| self.original.rows[*index].fields[self.ordinal].as_ref())
            .map(|field| (field.class.clone(), field.field.clone()))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn ready(&self, source: &PreparedBorrowedFormalIngressV1) -> bool {
        let seeds = &source.original_formal_seeds;
        Rc::ptr_eq(&self.original, &seeds.new_calls)
            && seeds.corroborate_original_v1(&source.source_incoming)
            && self.origins.iter().all(|origin| source.candidate_i64_source_agreement(*origin))
            && self.array_rows.iter().all(|index| self.original.rows[*index].array_reads[self.ordinal].as_ref()
                .is_some_and(|read| read.binding().owner() == self.original.rows[*index].site.owner()
                    && read.read().call().owner() == read.binding().owner()))
            && self.rows.iter().all(|index| matches!(&self.original.rows[*index].birth,
                OriginalNewBirthV1::Birth { source, formals, .. }
                    if *source == self.constructor && formals.get(self.ordinal) == Some(&self.formal)))
    }
}

impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn array_get_at_v1(
        &self, call: &OwnedExprSiteV1,
    ) -> Result<Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>, String> {
        let mut original = None;
        for read in self.rows.iter().flat_map(|row| row.array_reads.iter()).flatten()
            .filter(|read| read.read().call() == call) {
            if let Some(held) = &original {
                if !Rc::ptr_eq(held, read.read()) { return Err(freeze("array-read/original-new-source-conflict")); }
            } else { original = Some(Rc::clone(read.read())); }
        }
        Ok(original)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn argument_array_read_v1(
        &self, new_site: &OwnedExprSiteV1, ordinal: u32, site: &SourceExprSiteV1, binding: BindingRefV1,
    ) -> Result<Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>, String> {
        let mut rows = self.rows.iter().filter(|row| &row.site == new_site);
        let row = rows.next().ok_or_else(|| freeze("array-read/original-new-missing"))?;
        let ordinal = ordinal as usize;
        if rows.next().is_some() || row.original.arguments().get(ordinal) != Some(site)
            || row.array_reads.len() != row.original.arguments().len()
            || binding.owner() != new_site.owner() { return Err(freeze("array-read/original-new-argument-drift")); }
        let read = row.array_reads[ordinal].as_ref();
        if read.is_some_and(|read| read.binding() != binding || read.read().call().owner() != new_site.owner()) {
            return Err(freeze("array-read/original-new-binding-drift"));
        }
        Ok(read)
    }
}

/// Passive original source lender, issued before transport selection. A failed
/// transport or result seal cannot erase the original New/operand identities.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct OriginalNewSourceLoanV1(Rc<OriginalNewSeedsV1>);
impl OriginalNewSeedsV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn source_loan_v1(self: &Rc<Self>) -> OriginalNewSourceLoanV1 { OriginalNewSourceLoanV1(Rc::clone(self)) }
}
impl OriginalNewSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn argument_array_read_v1(
        &self, new_site: &OwnedExprSiteV1, ordinal: u32, site: &SourceExprSiteV1, binding: BindingRefV1,
    ) -> Result<Option<&Rc<crate::mir::normal_callable_semantic_package::ArgumentFieldArrayReadSourceV1>>, String> {
        self.0.argument_array_read_v1(new_site, ordinal, site, binding)
    }
}
