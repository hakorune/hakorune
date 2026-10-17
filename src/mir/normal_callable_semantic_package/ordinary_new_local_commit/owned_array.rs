//! Construction progress for the source-owned holder, inside the existing ledger.
use super::*;
use std::collections::BTreeMap;
use crate::mir::instruction::{OwnedArrayInvokeOperation as Array, OwnedArraySlotDescriptorV1};
use crate::mir::normal_callable_semantic_package::PreparedOwnedArrayConstructionV1;

#[derive(Debug)]
pub(super) struct OwnedArrayLocalProgress {
    source: PreparedOwnedArrayConstructionV1,
    progress: Progress,
    reads: BTreeMap<OwnedExprSiteV1, Rc<super::owned_array_read::EmittedOwnedArrayReadV1>>,
}
#[derive(Debug)]
enum Progress {
    Emitting,
    Emitted {
        array: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        phase: Phase,
    },
}
#[derive(Debug, PartialEq, Eq)]
enum Phase {
    ExpressionCompleted,
    Installed,
    Checked,
}
impl OwnedArrayLocalProgress {
    pub(super) fn binding(&self) -> BindingRefV1 {
        self.source.relation.binding()
    }
    pub(super) fn declaration(&self) -> &SourceBindingSiteV1 {
        self.source.relation.declaration_site()
    }
    pub(super) fn owner(&self) -> FunctionOwnerIdV1 {
        self.binding().owner()
    }
    pub(super) fn descriptor(&self) -> OwnedArraySlotDescriptorV1 {
        self.source.descriptor()
    }
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self.progress {
            Progress::Emitted {
                array,
                phase: Phase::ExpressionCompleted,
                ..
            } => Some(array),
            _ => None,
        }
    }
    pub(super) fn local(&self) -> Option<ValueId> {
        match self.progress {
            Progress::Emitted {
                array,
                phase: Phase::Installed | Phase::Checked,
                ..
            } => Some(array),
            _ => None,
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match &mut self.progress {
            Progress::Emitted { array, phase, .. }
                if *array == local && *phase == Phase::ExpressionCompleted =>
            {
                *phase = Phase::Installed
            }
            _ => unreachable!("owned Array local installation preflight"),
        }
    }
    pub(super) fn is_complete(&self) -> bool {
        matches!(
            self.progress,
            Progress::Emitted {
                phase: Phase::Checked,
                ..
            }
        )
    }
    pub(super) fn mark_checked(&mut self) {
        match &mut self.progress {
            Progress::Emitted { phase, .. }
                if matches!(*phase, Phase::Installed | Phase::Checked) =>
            {
                *phase = Phase::Checked
            }
            _ => unreachable!("owned Array construction validation preflight"),
        }
    }
    pub(super) fn emitted_bindings(&self) -> &[(BasicBlockId, MirInstruction)] {
        match &self.progress {
            Progress::Emitted { bindings, .. } => bindings,
            _ => &[],
        }
    }
    pub(super) fn checked_bindings(&self) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        match &self.progress {
            Progress::Emitted {
                phase: Phase::Checked,
                bindings,
                ..
            } => Ok(bindings),
            _ => Err(freeze("owned-array-unchecked")),
        }
    }
    pub(super) fn end_plan(&self) -> Box<[(RootHomeReleaseSubjectV1, InvokeOperation)]> {
        Box::new([(
            RootHomeReleaseSubjectV1::Binding(self.binding()),
            InvokeOperation::OwnedArray(Array::End {
                array: self.local().expect("installed owned Array"),
                descriptor: self.descriptor(),
            }),
        )])
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn has_owned_array_construction(&self, site: &OwnedExprSiteV1) -> bool {
        self.array_holder_lifecycles
            .borrow()
            .values()
            .any(|r| r.owner() == site.owner() && r.construction() == site.site())
    }
    pub(crate) fn owned_array_initializer_matches(
        &self,
        site: &OwnedExprSiteV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> bool {
        matches!(self.local_commits.borrow().get(site), Some(LocalCommitV1::OwnedArray(row))
            if row.binding() == binding && row.initializer() == Some(value))
    }
    pub(crate) fn begin_owned_array_construction(
        &self,
        source: PreparedOwnedArrayConstructionV1,
    ) -> Result<(), String> {
        let site = source.site();
        let recipes = self.array_holder_lifecycles.borrow();
        if !recipes
            .get(&source.relation.binding())
            .is_some_and(|r| Rc::ptr_eq(r, &source.recipe))
        {
            return Err(freeze("owned-array-foreign-source"));
        }
        self.owned_array_construction_unwind_for(&source)?;
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(&site) {
            return Err(freeze("owned-array-duplicate-construction"));
        }
        rows.insert(
            site,
            LocalCommitV1::OwnedArray(OwnedArrayLocalProgress {
                source,
                progress: Progress::Emitting,
                reads: BTreeMap::new(),
            }),
        );
        Ok(())
    }
    fn owned_array_construction_unwind_for(
        &self,
        source: &PreparedOwnedArrayConstructionV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        // The original New prefix is already newest-first. The existing lender
        // consumes oldest-first live sets and reverses exactly once.
        let live: Vec<_> = source
            .recipe
            .initializer_prefix_v1()?
            .prior_homes()
            .iter()
            .rev()
            .copied()
            .collect();
        self.prior_home_unwind_for(&live)
    }
    pub(crate) fn owned_array_construction_unwind(
        &self,
        site: &OwnedExprSiteV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::OwnedArray(row)) = rows.get(site) else {
            return Err(freeze("owned-array-progress-missing"));
        };
        self.owned_array_construction_unwind_for(&row.source)
    }
    pub(crate) fn record_owned_array_construction(
        &self,
        site: &OwnedExprSiteV1,
        array: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::OwnedArray(row)) = rows.get_mut(site) else {
            return Err(freeze("owned-array-progress-missing"));
        };
        if !matches!(row.progress, Progress::Emitting) {
            return Err(freeze("owned-array-duplicate-record"));
        }
        row.progress = Progress::Emitted {
            array,
            bindings,
            phase: Phase::ExpressionCompleted,
        };
        Ok(())
    }
    pub(super) fn validate_owned_array_construction(
        &self,
        site: &OwnedExprSiteV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::OwnedArray(row)) = rows.get(site) else {
            return Err(freeze("owned-array-progress-missing"));
        };
        if row.source.site() != *site
            || row.source.relation.initializer_site() != Some(site.site())
            || row.local().is_none()
        {
            return Err(freeze("owned-array-installation-drift"));
        }
        let Progress::Emitted {
            array, bindings, ..
        } = &row.progress
        else {
            return Err(freeze("owned-array-not-emitted"));
        };
        let mut allocation = None;
        let mut claim = None;
        for (block, instruction) in bindings {
            if !super::physical_boundary::check_binding(function, projection, *block, instruction)?
            {
                return Err(freeze("owned-array-binding-drift"));
            }
            match instruction {
                MirInstruction::Invoke {
                    operation: InvokeOperation::IntrinsicArrayNew,
                    fault_frame,
                    normal_landing,
                    fault_landing,
                } => {
                    if allocation
                        .replace((*block, *fault_frame, *normal_landing, *fault_landing))
                        .is_some()
                    {
                        return Err(freeze("owned-array-allocation-duplicate"));
                    }
                }
                MirInstruction::Invoke {
                    operation:
                        InvokeOperation::OwnedArray(Array::Claim {
                            array: actual,
                            descriptor,
                        }),
                    fault_frame,
                    normal_landing,
                    fault_landing,
                } => {
                    if actual != array
                        || *descriptor != row.descriptor()
                        || claim
                            .replace((*block, *fault_frame, *normal_landing, *fault_landing))
                            .is_some()
                    {
                        return Err(freeze("owned-array-claim-drift"));
                    }
                }
                _ => {}
            }
        }
        let (origin, frame, acquired, allocation_fault) =
            allocation.ok_or_else(|| freeze("owned-array-allocation-missing"))?;
        let (claim_origin, claim_frame, _claimed, claim_fault) =
            claim.ok_or_else(|| freeze("owned-array-claim-missing"))?;
        if acquired != claim_origin || frame != claim_frame || bindings.iter().filter(|(block, i)| matches!(i,
            MirInstruction::InvokeNormalResult { invoke_block, dst } if *block == acquired && *invoke_block == origin && dst == array)).count() != 1 {
            return Err(freeze("owned-array-normal-projection-drift"));
        }
        let expected = self.owned_array_construction_unwind_for(&row.source)?;
        check_fault_chain(bindings, allocation_fault, frame, None, &expected)?;
        check_fault_chain(bindings, claim_fault, frame, Some(*array), &expected)?;
        Ok(())
    }
}
/// Validate both outcomes of each cleanup step against the original release
/// sequence; a pre-Claim storage release is allowed only before that sequence.
pub(super) fn check_fault_chain(
    bindings: &[(BasicBlockId, MirInstruction)],
    mut at: BasicBlockId,
    frame: ValueId,
    unclaimed: Option<ValueId>,
    expected: &[InvokeOperation],
) -> Result<(), String> {
    // The common root writer gives each cleanup outcome its own Jump landing.
    // Follow only recorded empty forwarding blocks, then compare the common
    // successor. Distinct landing ids do not mean distinct cleanup outcomes.
    let skip = |mut block| -> Result<BasicBlockId, String> {
        let mut seen = std::collections::BTreeSet::new();
        loop {
            if !seen.insert(block) { return Err(freeze("owned-array-unwind-jump-cycle")); }
            let rows: Vec<_> = bindings.iter().filter(|(id, _)| *id == block)
                .map(|(_, instruction)| instruction).collect();
            match rows.as_slice() {
                [MirInstruction::Jump { target, edge_args: None }] => block = *target,
                _ => return Ok(block),
            }
        }
    };
    if let Some(array) = unclaimed {
        let block: Vec<_> = bindings
            .iter()
            .filter(|(b, _)| *b == at)
            .map(|(_, i)| i)
            .collect();
        let [MirInstruction::ArrayResidenceRelease { value }, MirInstruction::Jump { target, .. }] =
            block.as_slice()
        else {
            return Err(freeze("owned-array-unclaimed-release-missing"));
        };
        if *value != array {
            return Err(freeze("owned-array-unclaimed-value-drift"));
        }
        at = *target;
    }
    let mut visited = std::collections::BTreeSet::new();
    for expected in expected {
        if !visited.insert(at) {
            return Err(freeze("owned-array-unwind-cycle"));
        }
        let block: Vec<_> = bindings
            .iter()
            .filter(|(b, _)| *b == at)
            .map(|(_, i)| i)
            .collect();
        let [MirInstruction::Invoke {
            operation,
            fault_frame,
            normal_landing,
            fault_landing,
        }] = block.as_slice()
        else {
            return Err(freeze("owned-array-unwind-step"));
        };
        if operation != expected || *fault_frame != frame
            || skip(*normal_landing)? != skip(*fault_landing)? {
            return Err(freeze("owned-array-unwind-order"));
        }
        at = skip(*normal_landing)?;
    }
    let block: Vec<_> = bindings
        .iter()
        .filter(|(b, _)| *b == at)
        .map(|(_, i)| i)
        .collect();
    if !matches!(block.as_slice(), [MirInstruction::ReturnFault { fault_frame }] if *fault_frame == frame)
    {
        return Err(freeze("owned-array-unwind-outward"));
    }
    Ok(())
}

impl OrdinaryNewClaimLedgerV1 {
    /// The physical holder comes only from its exact source-selected Claim and
    /// installed LOCAL. A matching runtime type or raw value cannot substitute.
    pub(crate) fn live_owned_array_read_value_v1(
        &self,
        source: &crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1,
    ) -> Result<ValueId, String> {
        if source.recipe.original_read_v1(source.site()).is_none() {
            return Err(freeze("owned-array-read-foreign-source"));
        }
        self.live_owned_array_recipe_value_v1(&source.recipe)
    }
    pub(crate) fn live_owned_array_append_value_v1(
        &self, source: &crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
    ) -> Result<ValueId, String> {
        if source.recipe.original_append_v1(source.site()).is_none() {
            return Err(freeze("owned-array-append-foreign-source"));
        }
        self.live_owned_array_recipe_value_v1(&source.recipe)
    }
    pub(crate) fn has_owned_array_append_v1(&self, site: &OwnedExprSiteV1) -> bool {
        self.array_holder_lifecycles.borrow().values()
            .any(|recipe| recipe.original_append_v1(site).is_some())
    }
    pub(crate) fn owned_array_append_unwind_v1(
        &self, source: &crate::mir::normal_callable_semantic_package::OwnedArrayAppendSourceLoanV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        self.live_owned_array_append_value_v1(source)?;
        // Named Append observations are newest-first, whereas LocalCall inputs
        // to the existing unwind owner are oldest-first. Normalize once here.
        self.prior_home_unwind_for(&source.prior_homes()?.iter().rev().copied().collect::<Vec<_>>())
    }
    pub(super) fn live_owned_array_recipe_value_v1(
        &self, recipe: &Rc<super::super::ArrayHolderLifecycleRecipeV1>,
    ) -> Result<ValueId, String> {
        let recipes = self.array_holder_lifecycles.borrow();
        if !recipes.get(&recipe.holder()).is_some_and(|original| Rc::ptr_eq(original, recipe)) {
            return Err(freeze("owned-array-foreign-source"));
        }
        let rows = self.local_commits.borrow();
        let site = OwnedExprSiteV1::new(recipe.owner(), recipe.construction().clone());
        let Some(LocalCommitV1::OwnedArray(row)) = rows.get(&site) else {
            return Err(freeze("owned-array-holder-not-claimed"));
        };
        if !Rc::ptr_eq(&row.source.recipe, recipe) || row.descriptor() != recipe.slot_descriptor_v1() {
            return Err(freeze("owned-array-holder-drift"));
        }
        row.local().ok_or_else(|| freeze("owned-array-holder-not-installed"))
    }
    pub(crate) fn record_owned_array_read_v1(
        &self, read: Rc<super::owned_array_read::EmittedOwnedArrayReadV1>,
    ) -> Result<(), String> {
        let source = read.source();
        self.live_owned_array_read_value_v1(source)?;
        let site = OwnedExprSiteV1::new(source.holder().owner(), source.recipe.construction().clone());
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::OwnedArray(row)) = rows.get_mut(&site) else {
            return Err(freeze("owned-array-read-holder-missing"));
        };
        if row.reads.contains_key(source.site()) { return Err(freeze("owned-array-read-duplicate")); }
        row.reads.insert(source.site().clone(), read);
        Ok(())
    }
    pub(crate) fn owned_array_read_fault_unwind_v1(
        &self,
        call: &OwnedExprSiteV1,
        read: &crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1,
    ) -> Result<Vec<InvokeOperation>, String> {
        let observation = self.lexical_i64_call_source(call)
            .ok_or_else(|| freeze("owned-array-read-call-source-missing"))?;
        if call.owner() != read.site().owner()
            || observation.arguments().iter().filter(|argument| matches!(argument,
                crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { site, .. }
                    if site == read.site().site())).count() != 1
            || observation.prior_homes().iter().filter(|binding| **binding == read.holder()).count() != 1
        {
            return Err(freeze("owned-array-read-live-prefix-drift"));
        }
        self.live_owned_array_read_value_v1(read)?;
        self.prior_home_unwind_for(observation.prior_homes())
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Pre-install semantic reconstruction borrows the once-issued recipe; it
    /// never creates an installed package or substitutes an executable Get.
    pub(in crate::mir::normal_callable_semantic_package) fn retained_owned_array_read_source_v1(
        &self, original: &super::super::lexical_instance_call::ArrayElementSourceLoanV1,
    ) -> Result<Option<crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1>, String> {
        let recipes = self.array_holder_lifecycles.borrow();
        let Some(recipe) = recipes.get(&original.holder()) else { return Ok(None); };
        if recipe.original_read_v1(original.read()) != Some(original) { return Err(freeze("owned-array-read-original-loan-drift")); }
        crate::mir::normal_callable_semantic_package::OwnedArrayReadSourceLoanV1::from_original_recipe_v1(
            Rc::clone(recipe), original.read().clone(),
        ).map(Some)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn require_original_array_ingress_v1(
        &self, ingress: &super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
    ) -> Result<(), String> {
        let original = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("owned-array-original-ingress-missing"))?.as_ref().map_err(Clone::clone)?;
        if !std::ptr::eq(original, ingress) { return Err(freeze("owned-array-foreign-ingress")); }
        Ok(())
    }
}

#[path = "owned_array_holder.rs"]
mod holder;
pub(crate) use holder::EmittedOwnedArrayHolderV1;
