//! Conditional Array requirements retained with the original incoming source.
//! Uniform class is source-only; successful slots, live reads and execution are owed.
use super::super::super::completion_index::ObjectReturnTeardownDescriptorV1;
use super::super::super::result_class_claim::{
    ObjectCallSourceLoanV1, OrdinaryNewResultClassClaimsV1,
};
use super::*;
use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::mir::normal_callable_semantic_package::{
    physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
};
use crate::mir::resolved_semantics::{
    NamedArrayAppendOnlySourceV1, NamedArrayConstructionSourceV1, NamedArrayRequirementIssueV1,
};
use std::cell::OnceCell;
use std::rc::Rc;

#[path = "ordinary_new_borrowed_array_home.rs"]
mod home_source;
pub(in crate::mir::normal_callable_semantic_package) use home_source::ArrayHolderLifecycleRecipeV1;


/// Derived whole-producer support on the original holder. This preserves the
/// canonical descriptor, not acquired slots, live-read or execution permission.
/// No source Rc back-edge: the original requirement owns this plan.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct ArrayProducerPlanV1 {
    descriptor: ObjectReturnTeardownDescriptorV1,
    selected: Rc<VerifiedSelectedCallableBatchMapV1>,
    signatures: Rc<VerifiedCallablePhysicalSignatureCohortV1>,
}

/// Original LOCAL holder identity for the source Home walk. Element support,
/// when selected, stays in the SAME requirement; no construction/End execution.
#[derive(Debug, Clone)]
pub(crate) struct NamedArrayHolderSourceLoanV1 {
    construction: NamedArrayConstructionSourceV1,
    requirement: Option<Rc<ArrayElementSourceRequirementV1>>,
    append_only: Option<Rc<NamedArrayAppendOnlySourceV1>>,
    empty_transfer: Option<crate::mir::resolved_semantics::NamedArrayEmptyTransferSourceV1>,
}
impl PartialEq for NamedArrayHolderSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.construction == other.construction
            && self.empty_transfer == other.empty_transfer
            && match (&self.requirement, &other.requirement) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
            && match (&self.append_only, &other.append_only) {
                (Some(a), Some(b)) => Rc::ptr_eq(a, b),
                (None, None) => true,
                _ => false,
            }
    }
}
impl Eq for NamedArrayHolderSourceLoanV1 {}
impl NamedArrayHolderSourceLoanV1 {
    fn from_requirement(requirement: Rc<ArrayElementSourceRequirementV1>) -> Self {
        Self { construction: requirement.holder().construction().clone(), requirement: Some(requirement), append_only: None, empty_transfer: None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn from_construction(
        construction: NamedArrayConstructionSourceV1,
        empty_transfer: Option<crate::mir::resolved_semantics::NamedArrayEmptyTransferSourceV1>,
    ) -> Self {
        Self { construction, requirement: None, append_only: None, empty_transfer }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn from_append_only(
        construction: NamedArrayConstructionSourceV1,
        append_only: Rc<NamedArrayAppendOnlySourceV1>,
    ) -> Self {
        Self { construction, requirement: None, append_only: Some(append_only), empty_transfer: None }
    }
    pub(crate) fn proves_empty_transfer(
        &self, assignment: &crate::mir::resolved_semantics::ResolvedAssignmentSourceV1,
        destination: BindingRefV1, field: &str,
    ) -> bool {
        self.requirement.is_none() && self.append_only.is_none() && self.empty_transfer.as_ref().is_some_and(|source|
            source.matches_transfer(&self.construction, assignment, destination, field))
    }
    pub(crate) fn has_element_contract(&self) -> bool { self.requirement.is_some() }
    pub(crate) fn is_dynamic_carrier_append_only_v1(&self) -> bool {
        self.requirement.is_none() && self.append_only.as_ref().is_some_and(|source|
            source.has_appends() && source.reads().is_empty())
    }
    pub(crate) fn original_append_sites_v1(&self) -> Vec<OwnedExprSiteV1> {
        let rows = self.append_only.as_ref().map(|source| source.appends())
            .or_else(|| self.requirement.as_ref().map(|row| row.holder().appends()));
        rows.into_iter().flatten().map(|row|
            OwnedExprSiteV1::new(self.binding().owner(), row.call().clone())).collect()
    }
    pub(crate) fn append_candidate_is_null_v1(&self, call: &OwnedExprSiteV1) -> Option<bool> {
        if call.owner() != self.binding().owner() { return None; }
        self.requirement.as_ref()
            .and_then(|row| row.holder().appends().iter().find(|append| append.call() == call.site())
                .map(|append| append.null_candidate()))
            .or_else(|| self.append_only.as_ref()?.append_candidate_is_null_v1(call.site()))
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.construction.binding()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn construction_site(&self) -> &SourceExprSiteV1 {
        self.construction.construction()
    }
    pub(crate) fn construction(&self) -> OwnedExprSiteV1 {
        OwnedExprSiteV1::new(self.construction.owner(), self.construction_site().clone())
    }
    pub(crate) fn append_candidate(&self, call: &OwnedExprSiteV1) -> Option<OwnedExprSiteV1> {
        if call.owner() != self.binding().owner() {
            return None;
        }
        let candidate = self.requirement.as_ref()
            .and_then(|row| row.holder().appends().iter().find(|append| append.call() == call.site())
                .map(|append| append.candidate()))
            .or_else(|| self.append_only.as_ref()?.append_candidate_v1(call.site()))?;
        Some(OwnedExprSiteV1::new(call.owner(), candidate.clone()))
    }
    pub(crate) fn append_receiver_v1(
        &self,
        call: &OwnedExprSiteV1,
    ) -> Option<SourceExprSiteV1> {
        if call.owner() != self.binding().owner() {
            return None;
        }
        self.requirement
            .as_ref()
            .and_then(|row| {
                row.holder()
                    .appends()
                    .iter()
                    .find(|append| append.call() == call.site())
                    .map(|append| append.receiver().clone())
            })
            .or_else(|| {
                self.append_only
                    .as_ref()?
                    .appends()
                    .iter()
                    .find(|append| append.call() == call.site())
                    .map(|append| append.receiver().clone())
            })
    }
}

impl PreparedBorrowedFormalIngressV1 {
    /// Original holder obligations require the same Home walk even when the
    /// owner has no ordinary-New, borrowed parameter or other incidental trigger.
    /// Structural/class refusal stays retained; membership grants no execution.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn requires_named_array_holder_walk_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> bool {
        self.array_sources.iter().any(|(read, row)| read.owner() == owner && row.construction.owner() == owner)
    }

    /// Source obligation only; initial Home analysis must not wait for Normal.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn named_array_holder_source_v1(
        &self,
        site: &OwnedExprSiteV1,
        binding: BindingRefV1,
        initializer: bool,
    ) -> Result<Option<NamedArrayHolderSourceLoanV1>, String> {
        let mut found: Option<NamedArrayHolderSourceLoanV1> = None;
        for (read, row) in self
            .array_sources
            .iter()
            .filter(|(read, _)| read.owner() == site.owner())
        {
            // Membership survives refusal in this same source row. An unrelated
            // holder's error is not authority for the requested construction.
            if row.construction.binding() != binding
                || (initializer
                    && (row.construction.owner() != site.owner()
                        || row.construction.construction() != site.site()))
            {
                continue;
            }
            let requirement = row.requirement.as_ref().map_err(Clone::clone)?;
            let current = self
                .array_element_source_v1(read)?
                .ok_or_else(|| freeze("array-holder/current-source-missing"))?;
            if !Rc::ptr_eq(requirement, current) {
                return Err(freeze("array-holder/current-source-drift"));
            }
            let loan = NamedArrayHolderSourceLoanV1::from_requirement(Rc::clone(requirement));
            let matches = if initializer {
                loan.binding() == binding && loan.construction() == *site
            } else {
                loan.binding() == binding && loan.append_candidate(site).is_some()
            };
            if matches {
                if found.as_ref().is_some_and(|previous| previous != &loan) {
                    return Err(freeze("array-holder/ambiguous-source"));
                }
                found = Some(loan);
            }
        }
        Ok(found)
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct ArrayElementSourceRequirementV1 {
    holder: Rc<NamedArrayAppendOnlySourceV1>,
    producers: Box<[(OwnedExprSiteV1, ObjectCallSourceLoanV1)]>,
    class: Box<str>,
    producer_plan: OnceCell<Rc<ArrayProducerPlanV1>>,
}
impl ArrayElementSourceRequirementV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn class(&self) -> &str {
        &self.class
    }
    pub(super) fn holder_source_v1(&self) -> &Rc<NamedArrayAppendOnlySourceV1> {
        &self.holder
    }
    pub(in crate::mir::normal_callable_semantic_package) fn holder(
        &self,
    ) -> &NamedArrayAppendOnlySourceV1 {
        &self.holder
    }
    pub(in crate::mir::normal_callable_semantic_package) fn producers(
        &self,
    ) -> &[(OwnedExprSiteV1, ObjectCallSourceLoanV1)] {
        &self.producers
    }

    pub(super) fn recheck(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        classes: &OrdinaryNewResultClassClaimsV1,
        calls: &BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
    ) -> Result<(), String> {
        for (site, loan) in self.producers.iter() {
            let source = calls
                .get(site)
                .ok_or_else(|| freeze("array-source/producer-missing"))?;
            classes
                .recheck_object_call_source_v1(batch, selected, source, loan)
                .map_err(|issue| format!("{}: {issue}", freeze("array-source/producer-drift")))?;
            if loan.class() != self.class() {
                return Err(freeze("array-source/class-drift"));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct ArraySourceRequirementRowV1 {
    pub(super) construction: NamedArrayConstructionSourceV1,
    /// Keep already-issued structural candidate membership even if class or
    /// producer qualification fails. It is not execution or slot permission.
    pub(super) holder: Result<Rc<NamedArrayAppendOnlySourceV1>, String>,
    pub(super) requirement: Result<Rc<ArrayElementSourceRequirementV1>, String>,
}
pub(super) type ArraySourceRequirementsV1 = BTreeMap<OwnedExprSiteV1, ArraySourceRequirementRowV1>;

/// The same read inventory before class qualification. Consumed at finish;
/// only the original structural issuer grants Append candidate membership.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) struct OriginalArraySourcesV1 {
    rows: BTreeMap<OwnedExprSiteV1, (NamedArrayConstructionSourceV1, Result<Rc<NamedArrayAppendOnlySourceV1>, String>)>,
}
impl OriginalArraySourcesV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn contains_candidate_v1(&self, site: &OwnedExprSiteV1) -> bool {
        self.rows.iter().any(|(read, (_, holder))| read.owner() == site.owner()
            && holder.as_ref().is_ok_and(|holder| holder.appends().iter()
                .any(|append| append.candidate() == site.site())))
    }
}

pub(super) fn collect_array_structure_sources_v1(
    brands: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    arguments: BTreeSet<OwnedExprSiteV1>,
) -> Result<OriginalArraySourcesV1, String> {
    let mut result = OriginalArraySourcesV1 { rows: BTreeMap::new() };
    extend_array_structure_sources_v1(&mut result, brands, batch, constructors, arguments)?;
    Ok(result)
}

/// Complete the same read map from retained census observations, including
/// Static arguments discovered there. Never change the completed caller census.
pub(super) fn extend_array_structure_sources_v1(
    result: &mut OriginalArraySourcesV1,
    brands: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    arguments: BTreeSet<OwnedExprSiteV1>,
) -> Result<(), String> {
    let Some(brands) = brands else { return Ok(()) };
    let mut holders: BTreeMap<_, _> = result.rows.values().map(|(construction, holder)|
        (construction.binding(), holder.clone())).collect();
    for site in arguments {
        if result.rows.contains_key(&site) { continue; }
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, site.owner(), |_, input| -> Result<(), String> {
            let ledger = input.forest().callable_source_ledger(input.owner())
                .map_err(|_| freeze("array-source/ledger"))?;
            let Some(read) = ledger.method_call(site.site()) else { return Ok(()) };
            if crate::mir::core_method_result_kind::lookup_core_method_result_row_v2("ArrayBox", read.selector(), read.arity())
                .is_none_or(|row| row.op != crate::mir::core_method_op::CoreMethodOp::ArrayGet) { return Ok(()); }
            let identity = match NamedArrayConstructionSourceV1::issue(&ledger, read, batch.ordinary_box_coverage(), brands) {
                Ok(identity) => identity,
                Err(NamedArrayRequirementIssueV1::NotNamedArray | NamedArrayRequirementIssueV1::UnsupportedReceiver
                    | NamedArrayRequirementIssueV1::InitializerMissing | NamedArrayRequirementIssueV1::ConstructionMissing) => return Ok(()),
                Err(issue) => return Err(format!("{}: {issue:?}", freeze("array-source/construction"))),
            };
            let holder = holders.entry(identity.binding()).or_insert_with(|| {
                NamedArrayAppendOnlySourceV1::issue(&ledger, read, batch.ordinary_box_coverage(), brands)
                    .map(Rc::new).map_err(|issue| format!("{}: {issue:?}", freeze("array-source/holder")))
            }).clone();
            result.rows.insert(site.clone(), (identity, holder));
            Ok(())
        })??;
    }
    Ok(())
}

/// Qualify once after Stored receivers have real final targets. Structural
/// holder Rc and selected refusals survive the phase transition unchanged.
pub(super) fn qualify_array_sources_v1(
    original: OriginalArraySourcesV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    classes: &OrdinaryNewResultClassClaimsV1,
    calls: &BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
) -> Result<ArraySourceRequirementsV1, String> {
    let mut result = BTreeMap::new();
    let mut qualified: BTreeMap<BindingRefV1, Result<Rc<ArrayElementSourceRequirementV1>, String>> = BTreeMap::new();
    for (read, (construction, holder)) in original.rows {
        let requirement = qualified.entry(construction.binding()).or_insert_with(|| {
            let holder = holder.as_ref().map_err(Clone::clone)?;
            let mut producers = Vec::new();
            let mut class: Option<Box<str>> = None;
            for append in holder.appends() {
                let site = OwnedExprSiteV1::new(read.owner(), append.candidate().clone());
                // The structural issuer permits exactly Null or a method Call.
                // Only an original method Call can need a producer loan.
                let is_null = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                    batch, constructors, read.owner(), |_, input| {
                        input.forest().callable_source_ledger(input.owner()).map(|ledger|
                            matches!(ledger.literal_source(append.candidate()), Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Null)))
                    })?.map_err(|_| freeze("array-source/ledger"))?;
                if is_null { continue; }
                let source = calls.get(&site).ok_or_else(|| freeze("array-source/producer-missing"))?;
                let loan = classes.object_call_source_v1(batch, selected, source)
                    .map_err(|issue| format!("{}: {issue}", freeze("array-source/producer")))?;
                if class.as_deref().is_some_and(|class| class != loan.class()) { return Err(freeze("array-source/mixed-class")); }
                class = Some(loan.class().into());
                producers.push((site, loan));
            }
            let source = ArrayElementSourceRequirementV1 { holder: Rc::clone(holder), producers: producers.into_boxed_slice(), producer_plan: OnceCell::new(),
                class: class.ok_or_else(|| freeze("array-source/class-unavailable"))? };
            source.recheck(batch, selected, classes, calls)?;
            Ok(Rc::new(source))
        }).clone();
        result.insert(read, ArraySourceRequirementRowV1 { construction, holder, requirement });
    }
    Ok(result)
}

/// One exact read of a shared conditional holder; no live slot or wire payload.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct ArrayElementSourceLoanV1 {
    read: OwnedExprSiteV1,
    index: SourceExprSiteV1,
    method: std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    requirement: Rc<ArrayElementSourceRequirementV1>,
}
impl PartialEq for ArrayElementSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.read == other.read
            && self.index == other.index
            && std::sync::Arc::ptr_eq(&self.method, &other.method)
            && Rc::ptr_eq(&self.requirement, &other.requirement)
    }
}
impl Eq for ArrayElementSourceLoanV1 {}
impl ArrayElementSourceLoanV1 {
    /// Nested actuals must retain the exact canonical resolver row.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_method_source_v1(
        &self, source: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>,
    ) -> bool {
        source.owner() == self.read.owner() && source.site() == self.read.site()
            && source.arguments().len() == 1 && source.arguments()[0].site() == &self.index
            && std::sync::Arc::ptr_eq(source, &self.method)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn same_holder_v1(
        &self,
        other: &Self,
    ) -> bool {
        Rc::ptr_eq(&self.requirement, &other.requirement)
    }
    /// Validate an existing normalized product against the SAME original issuer.
    /// Callers must first demand all current original inputs/result/Normal/End;
    /// a cached plan is never a substitute for those independent checks.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn check_producer_plan_v1(
        &self,
        descriptor: &ObjectReturnTeardownDescriptorV1,
        selected: &Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: &Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<(), String> {
        if self.requirement.producers.is_empty() {
            return Err(freeze("array-producer-plan/empty-original-producers"));
        }
        if let Some(plan) = self.requirement.producer_plan.get() {
            if plan.descriptor != *descriptor
                || !Rc::ptr_eq(&plan.selected, selected)
                || !Rc::ptr_eq(&plan.signatures, signatures)
            {
                return Err(freeze("array-producer-plan/original-issuance-drift"));
            }
        }
        Ok(())
    }

    /// The all-producer finisher seals only after every candidate holder has
    /// passed. Multiple Get loans keep the SAME source and normalized plan Rc.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn retain_checked_producer_plan_v1(
        &self,
        descriptor: ObjectReturnTeardownDescriptorV1,
        selected: &Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: &Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<&Rc<ArrayProducerPlanV1>, String> {
        self.check_producer_plan_v1(&descriptor, selected, signatures)?;
        Ok(self.requirement.producer_plan.get_or_init(|| {
            Rc::new(ArrayProducerPlanV1 {
                descriptor,
                selected: Rc::clone(selected),
                signatures: Rc::clone(signatures),
            })
        }))
    }

    /// Borrow every original producer, including those after this read.
    /// These remain source requirements, never acquisition or execution receipts.
    pub(in crate::mir::normal_callable_semantic_package) fn producer_sources_v1(
        &self,
    ) -> impl Iterator<Item = &ObjectCallSourceLoanV1> {
        self.requirement.producers().iter().map(|(_, loan)| loan)
    }

    /// Exact original input rows for ALL producers, never a prefix or Ready grant.
    pub(in crate::mir::normal_callable_semantic_package) fn original_producer_inputs_v1<'a>(
        &self, ingress: &'a PreparedBorrowedFormalIngressV1,
        catalog: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    ) -> Result<Vec<(&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    &'a crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1)>, String>{
        let mut inputs = Vec::new();
        for producer in self.producer_sources_v1() {
            let target = producer.source();
            let mut originals = ingress
                .source_incoming
                .exact_rows()
                .filter(|call| &call.call == target.call_site());
            let original = originals
                .next()
                .ok_or_else(|| freeze("array-producer-input/original-missing"))?;
            if originals.next().is_some()
                || original.callee != target.callee_owner()
                || original.source.require_instance()? != target
            {
                return Err(freeze("array-producer-input/original-identity"));
            }
            let mut contracts = catalog.iter().filter(|row| row.owner == original.callee);
            let contract = contracts
                .next()
                .ok_or_else(|| freeze("array-producer-input/contract-missing"))?;
            if contracts.next().is_some()
                || contract.batch_slot != target.target_batch_slot()
                || contract.parameters.len() != target.target().arity() as usize
                || original.arguments.iter().any(|(ordinal, site, formal)| {
                    target.argument_sites().get(*ordinal as usize) != Some(site)
                        || contract
                            .parameters
                            .get(*ordinal as usize)
                            .is_none_or(|row| row.binding != *formal)
                })
            {
                return Err(freeze("array-producer-input/contract-identity"));
            }
            inputs.push((original, contract));
        }
        Ok(inputs)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn read(&self) -> &OwnedExprSiteV1 {
        &self.read
    }
    pub(in crate::mir::normal_callable_semantic_package) fn index(&self) -> &SourceExprSiteV1 {
        &self.index
    }
    pub(in crate::mir::normal_callable_semantic_package) fn holder(&self) -> BindingRefV1 {
        self.requirement.holder().construction().binding()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        ingress: &PreparedBorrowedFormalIngressV1,
        read: &OwnedExprSiteV1,
    ) -> Result<Option<Self>, String> {
        let Some(requirement) = ingress.array_element_source_v1(read)? else {
            return Ok(None);
        };
        let mut rows = requirement
            .holder()
            .reads()
            .iter()
            .filter(|row| row.call() == read.site());
        let row = rows
            .next()
            .ok_or_else(|| freeze("array-source/read-missing"))?;
        if rows.next().is_some() {
            return Err(freeze("array-source/read-duplicate"));
        }
        Ok(Some(Self {
            read: read.clone(),
            index: row.index().clone(),
            method: std::sync::Arc::clone(row.source()),
            requirement: Rc::clone(requirement),
        }))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(
        &self,
        ingress: &PreparedBorrowedFormalIngressV1,
        classes: &OrdinaryNewResultClassClaimsV1,
        targets: &BTreeMap<OwnedExprSiteV1, &LexicalInstanceCallSourceTargetV1>,
    ) -> Result<(), String> {
        let current = Self::issue(ingress, self.read())?
            .ok_or_else(|| freeze("array-source/read-missing"))?;
        if &current != self {
            return Err(freeze("array-source/loan-drift"));
        }
        for (site, loan) in self.requirement.producers() {
            let source = targets
                .get(site)
                .ok_or_else(|| freeze("array-source/producer-missing"))?;
            classes
                .recheck_object_call_roots_v1(source, loan)
                .map_err(|issue| format!("{}: {issue}", freeze("array-source/producer-drift")))?;
        }
        Ok(())
    }
}
