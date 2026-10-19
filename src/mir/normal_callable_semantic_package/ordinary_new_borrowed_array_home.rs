//! Borrow the current holder Home obligations beside the SAME producer plan.
//! Completion stays borrowed: storing it in the source-owned plan would cycle.
use super::*;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, NamedArrayHomeObservationV1, RootHomeExitV1,
};
use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourceStmtSiteV1};

/// Source/producer preparation only. Acquisition, commit, live-read, physical
/// End and independent final correspondence are still separate obligations.
pub(in crate::mir::normal_callable_semantic_package) struct CheckedArrayHomeSourceV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) source: &'a ArrayElementSourceLoanV1,
    pub(in crate::mir::normal_callable_semantic_package) producer: &'a Rc<ArrayProducerPlanV1>,
    pub(in crate::mir::normal_callable_semantic_package) completion:
        &'a VerifiedFunctionCompletionV1,
    pub(in crate::mir::normal_callable_semantic_package) initializer: &'a CallerNewHomePrefixV1,
    pub(in crate::mir::normal_callable_semantic_package) appends:
        Box<[&'a NamedArrayHomeObservationV1]>,
    pub(in crate::mir::normal_callable_semantic_package) ends:
        Box<[(&'a SourceStmtSiteV1, &'a RootHomeExitV1)]>,
}

impl ArrayElementSourceLoanV1 {
    /// Accepted holders are direct top-level LOCAL constructions and cannot
    /// rebind/escape. Original explicit exit order decides whether a Home is
    /// owed; an omitted post-construction Home is never a pre-construction path.
    pub(in crate::mir::normal_callable_semantic_package) fn validate_holder_exit_source_v1(
        &self,
        exit: &SourceStmtSiteV1,
        count: usize,
    ) -> Result<bool, String> {
        let [SourcePathSegmentV1::Body(construction), SourcePathSegmentV1::Initializer(_)] = self
            .requirement
            .holder()
            .construction()
            .construction()
            .node()
            .segments()
        else {
            return Err(freeze("array-home/construction-order"));
        };
        let Some(SourcePathSegmentV1::Body(returned)) = exit.node().segments().first() else {
            return Err(freeze("array-home/exit-order"));
        };
        if count > 1 {
            return Err(freeze("array-home/duplicate-exit-holder"));
        }
        if returned == construction {
            return Err(freeze("array-home/exit-order"));
        }
        let acquired = returned > construction;
        if acquired != (count == 1) {
            return Err(freeze("array-home/exit-holder-coverage"));
        }
        Ok(acquired)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn checked_home_source_v1<'a>(
        &'a self,
        completion: &'a VerifiedFunctionCompletionV1,
        selected: &Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: &Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<Option<CheckedArrayHomeSourceV1<'a>>, String> {
        let Some(producer) = self.requirement.producer_plan.get() else {
            return Ok(None);
        };
        self.check_producer_plan_v1(&producer.descriptor, selected, signatures)?;
        let holder = self.requirement.holder();
        let binding = holder.construction().binding();
        if completion.owner() != binding.owner() || self.read.owner() != binding.owner() {
            return Err(freeze("array-home/owner-drift"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(None);
        };
        let loan = NamedArrayHolderSourceLoanV1::from_requirement(Rc::clone(&self.requirement));
        let mut initializer = None;
        let mut appends = Vec::new();
        for observation in flow.named_arrays() {
            match observation {
                NamedArrayHomeObservationV1::Initializer { source, prefix }
                    if source.binding() == binding =>
                {
                    if source != &loan || initializer.is_some() {
                        return Err(freeze("array-home/initializer-identity"));
                    }
                    let prefix = prefix.as_ref().map_err(|issue| {
                        format!("{}: {issue:?}", freeze("array-home/initializer-prefix"))
                    })?;
                    if prefix.destination() != binding
                        || prefix.required_unwind() != &loan.construction()
                    {
                        return Err(freeze("array-home/destination-drift"));
                    }
                    initializer = Some(prefix);
                }
                NamedArrayHomeObservationV1::Append {
                    source,
                    call,
                    candidate,
                    availability,
                    prior_homes,
                    ..
                } if source.binding() == binding => {
                    if source != &loan
                        || loan.append_candidate(call).as_ref() != Some(candidate)
                        || prior_homes.iter().filter(|home| home.binding() == binding).count() != 1
                    {
                        return Err(freeze("array-home/append-identity"));
                    }
                    availability.as_ref().map_err(|issue| {
                        format!("{}: {issue:?}", freeze("array-home/append-prefix"))
                    })?;
                    appends.push(observation);
                }
                _ => {}
            }
        }
        let Some(initializer) = initializer else {
            return Ok(None);
        };
        if appends.len() != holder.appends().len() {
            return Err(freeze("array-home/append-coverage"));
        }
        for original in holder.appends() {
            if appends.iter().filter(|observation| matches!(observation,
                NamedArrayHomeObservationV1::Append { call, candidate, .. }
                if call.site() == original.call() && candidate.site() == original.candidate()
            )).count() != 1 { return Err(freeze("array-home/append-coverage")); }
        }
        if !flow.all_exits_ready() {
            return Ok(None);
        }
        let mut ends = Vec::new();
        for site in completion.explicit_sites() {
            let exit = flow
                .exit_row(site)
                .ok_or_else(|| freeze("array-home/exit-missing"))?
                .map_err(|issue| format!("{}: {issue:?}", freeze("array-home/exit")))?;
            let count = exit.homes().iter().filter(|home| home.binding() == binding).count();
            if self.validate_holder_exit_source_v1(site, count)? {
                ends.push((site, exit));
            }
        }
        Ok(Some(CheckedArrayHomeSourceV1 {
            source: self,
            producer,
            completion,
            initializer,
            appends: appends.into_boxed_slice(),
            ends: ends.into_boxed_slice(),
        }))
    }
}

/// Package-owned source obligations, shared with the physical coordinator.
/// Neither the original requirement nor Completion may point back to this row.
#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct ArrayHolderLifecycleRecipeV1 {
    holder: NamedArrayHolderSourceLoanV1,
    completion: Rc<VerifiedFunctionCompletionV1>,
    caller: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    elements: ArrayHolderElementRecipeV1,
}

/// Element operations share their original requirement/producer; neither a
/// selected read nor its representative becomes the holder's construction identity.
#[derive(Debug)]
struct ArrayHolderElementRecipeV1 {
    source: ArrayElementSourceLoanV1,
    producer: Rc<ArrayProducerPlanV1>,
    reads: Box<[ArrayElementSourceLoanV1]>,
}
impl ArrayHolderLifecycleRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_append_sites_v1(&self) -> impl Iterator<Item = OwnedExprSiteV1> + '_ {
        self.elements.source.requirement.holder().appends().iter()
            .map(|row| OwnedExprSiteV1::new(self.owner(), row.call().clone()))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_read_sites_v1(&self) -> impl Iterator<Item = &OwnedExprSiteV1> {
        self.elements.reads.iter().map(|row| row.read())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_end_sites_v1(&self) -> Result<Vec<&SourceStmtSiteV1>, String> {
        let flow = self.completion.cleanup().root_flow().ok_or_else(|| freeze("array-holder/root-flow-missing"))?;
        let mut ends = Vec::new();
        for exit in self.completion.explicit_sites() {
            let row = flow.exit_row(exit).ok_or_else(|| freeze("array-holder/exit-missing"))?
                .map_err(|issue| format!("{}: {issue:?}", freeze("array-holder/exit-unavailable")))?;
            if self.elements.source.validate_holder_exit_source_v1(exit, row.homes().iter().filter(|home| home.binding() == self.holder()).count())? {
                ends.push(exit);
            }
        }
        Ok(ends)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn holder_source_loan_v1(&self) -> NamedArrayHolderSourceLoanV1 {
        self.holder.clone()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        source: &ArrayElementSourceLoanV1,
        ingress: &PreparedBorrowedFormalIngressV1,
        completion: &Rc<VerifiedFunctionCompletionV1>,
        caller: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        selected: &Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: &Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<Option<Rc<Self>>, String> {
        let Some(home) = source.checked_home_source_v1(completion, selected, signatures)?
            else { return Ok(None); };
        let mut reads = Vec::new();
        for original in source.requirement.holder().reads() {
            let site = OwnedExprSiteV1::new(completion.owner(), original.call().clone());
            let read = ArrayElementSourceLoanV1::issue(ingress, &site)?
                .ok_or_else(|| freeze("array-lifecycle/original-read-missing"))?;
            if !source.same_holder_v1(&read) || reads.contains(&read) {
                return Err(freeze("array-lifecycle/original-read-identity"));
            }
            reads.push(read);
        }
        if reads.is_empty() || !reads.contains(source) {
            return Err(freeze("array-lifecycle/original-read-coverage"));
        }
        Ok(Some(Rc::new(Self {
            holder: NamedArrayHolderSourceLoanV1::from_requirement(Rc::clone(&source.requirement)),
            completion: Rc::clone(completion), caller: caller.clone(),
            elements: ArrayHolderElementRecipeV1 {
                source: reads[0].clone(), producer: Rc::clone(home.producer), reads: reads.into(),
            },
        })))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn owner(&self) -> FunctionOwnerIdV1 { self.completion.owner() }
    pub(in crate::mir::normal_callable_semantic_package) fn holder(&self) -> BindingRefV1 { self.holder.binding() }
    pub(in crate::mir::normal_callable_semantic_package) fn caller(&self) -> &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1 { &self.caller }
    pub(in crate::mir::normal_callable_semantic_package) fn construction(&self) -> &SourceExprSiteV1 {
        self.holder.construction_site()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn source(&self) -> &ArrayElementSourceLoanV1 { &self.elements.source }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        self.holder == other.holder && self.elements.source == other.elements.source
            && self.elements.reads == other.elements.reads && self.caller == other.caller
            && Rc::ptr_eq(&self.elements.producer, &other.elements.producer) && Rc::ptr_eq(&self.completion, &other.completion)
    }
    /// Structural/Home re-demand only. The ledger additionally re-demands the
    /// current whole original producer inputs, Normal results and End descriptor.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(
        &self, ingress: &PreparedBorrowedFormalIngressV1,
        completion: &Rc<VerifiedFunctionCompletionV1>,
        selected: &Rc<VerifiedSelectedCallableBatchMapV1>,
        signatures: &Rc<VerifiedCallablePhysicalSignatureCohortV1>,
    ) -> Result<(), String> {
        if !Rc::ptr_eq(completion, &self.completion) {
            return Err(freeze("array-lifecycle/original-completion-drift"));
        }
        let Some(home) = self.elements.source.checked_home_source_v1(completion, selected, signatures)?
            else { return Err(freeze("array-lifecycle/home-unavailable")); };
        if !Rc::ptr_eq(home.producer, &self.elements.producer)
            || self.elements.reads.len() != self.elements.source.requirement.holder().reads().len()
        { return Err(freeze("array-lifecycle/original-source-drift")); }
        for (read, original) in self.elements.reads.iter().zip(self.elements.source.requirement.holder().reads()) {
            if read.read().site() != original.call()
                || ArrayElementSourceLoanV1::issue(ingress, read.read())?.as_ref() != Some(read)
            { return Err(freeze("array-lifecycle/current-read-drift")); }
        }
        Ok(())
    }
}

impl ArrayHolderLifecycleRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn slot_descriptor_v1(&self) -> crate::mir::instruction::OwnedArraySlotDescriptorV1 {
        let holder = self.elements.source.requirement.holder();
        let nullable = self.elements.source.requirement.producers().len() != holder.appends().len()
            || self.elements.source.producer_sources_v1().any(|p| matches!(p.result_class(),
                crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1::NullableObject(_)));
        crate::mir::instruction::OwnedArraySlotDescriptorV1::new(self.elements.producer.descriptor.object(), nullable)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn initializer_prefix_v1(&self) -> Result<&CallerNewHomePrefixV1, String> {
        let flow = self.completion.cleanup().root_flow().ok_or_else(|| freeze("array-lifecycle/root-flow-missing"))?;
        let mut matches = flow.named_arrays().iter().filter_map(|row| match row {
            NamedArrayHomeObservationV1::Initializer { source, prefix } if source.binding() == self.holder() => Some((source, prefix)),
            _ => None,
        });
        let (source, prefix) = matches.next().ok_or_else(|| freeze("array-lifecycle/initializer-missing"))?;
        if matches.next().is_some() || source.construction().site() != self.construction() {
            return Err(freeze("array-lifecycle/initializer-drift"));
        }
        prefix.as_ref().map_err(|issue| format!("{}: {issue:?}", freeze("array-lifecycle/initializer-unavailable")))
    }
}

impl ArrayHolderLifecycleRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_append_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Option<(&SourceExprSiteV1, &SourceExprSiteV1, bool)> {
        if site.owner() != self.owner() { return None; }
        self.elements.source.requirement.holder().appends().iter().find(|row| row.call() == site.site())
            .map(|row| (row.receiver(), row.candidate(), row.null_candidate()))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn append_prior_homes_v1(
        &self, site: &OwnedExprSiteV1,
    ) -> Result<&[crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1], String> {
        let (_, candidate, _) = self.original_append_v1(site)
            .ok_or_else(|| freeze("array-append/original-missing"))?;
        let flow = self.completion.cleanup().root_flow()
            .ok_or_else(|| freeze("array-append/root-flow-missing"))?;
        let mut rows = flow.named_arrays().iter().filter_map(|row| match row {
            NamedArrayHomeObservationV1::Append { source, call, candidate: observed, availability, prior_homes, .. }
                if call == site => Some((source, observed, availability, prior_homes)),
            _ => None,
        });
        let (holder, observed, available, homes) = rows.next()
            .ok_or_else(|| freeze("array-append/home-missing"))?;
        if rows.next().is_some() || holder.binding() != self.holder()
            || holder.construction().site() != self.construction()
            || holder.append_candidate(site).as_ref() != Some(observed)
            || observed.site() != candidate
            || homes.iter().filter(|binding| binding.binding() == self.holder()).count() != 1
        { return Err(freeze("array-append/home-drift")); }
        available.as_ref().map_err(|issue| format!("{}: {issue:?}", freeze("array-append/unavailable")))?;
        Ok(homes)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn check_append_candidate_v1(
        &self, site: &OwnedExprSiteV1, row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<(), String> {
        let (_, candidate, null) = self.original_append_v1(site)
            .ok_or_else(|| freeze("array-append/original-missing"))?;
        let mut sources = self.elements.source.producer_sources_v1()
            .filter(|source| source.source().call_site().site() == candidate);
        let original = sources.next().ok_or_else(|| freeze("array-append/producer-missing"))?;
        if null || sources.next().is_some() || original.source() != row.source_target()
            || original.class() != self.element_class_v1()
        { return Err(freeze("array-append/producer-source-drift")); }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_read_v1(&self, site: &OwnedExprSiteV1) -> Option<&ArrayElementSourceLoanV1> {
        self.elements.reads.iter().find(|read| read.read() == site)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn element_class_v1(&self) -> &str {
        self.elements.source.requirement.class()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn original_read_integer_v1(&self, site: &OwnedExprSiteV1) -> Result<i64, String> {
        let loan = self.original_read_v1(site).ok_or_else(|| freeze("array-read/original-missing"))?;
        let mut rows = self.elements.source.requirement.holder().reads().iter().filter(|row| row.call() == site.site());
        let row = rows.next().ok_or_else(|| freeze("array-read/original-index-missing"))?;
        if rows.next().is_some() || row.index() != loan.index() {
            return Err(freeze("array-read/original-index-drift"));
        }
        Ok(row.integer())
    }
}

impl ArrayHolderLifecycleRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn original_read_receiver_v1(&self, site: &OwnedExprSiteV1) -> Result<&SourceExprSiteV1, String> {
        self.original_read_integer_v1(site)?;
        self.elements.source.requirement.holder().reads().iter().find(|row| row.call() == site.site())
            .map(|row| row.receiver()).ok_or_else(|| freeze("array-read/original-receiver-missing"))
    }
}
