//! Source-issued Birth formal lanes retained until the final backend handoff.
//!
//! This relation records source bindings and their explicit ABI lanes.  It has
//! no `ValueId` or function-parameter authority: the final view validates the
//! already-published physical layout before a transport projects values.

use super::super::instance_constructor_semantic::BirthFormalContractV1;
use super::super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticRowV1;
use super::super::instance_construction::{ConstructionEligibilityV1, ConstructionPlanV1};
use crate::mir::instance_constructor_abi::InstanceConstructorAbiV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, FunctionOwnerIdV1, ReceiverPolicyV1, SemanticOwnerRootProfileV1,
};
use hakorune_mir_defs::{CanonicalObjectIdV1, CanonicalSameModuleCallableKeyV1};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BirthResultAbiV1 {
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BirthFormalLaneV1 {
    binding: BindingRefV1,
    source_ordinal: Option<u32>,
    physical_lane: u32,
}

impl BirthFormalLaneV1 {
    pub(crate) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn source_ordinal(self) -> Option<u32> {
        self.source_ordinal
    }

    pub(crate) const fn physical_lane(self) -> u32 {
        self.physical_lane
    }
}

/// Immutable source-to-ABI relation for one exact declared instance Birth.
#[derive(Debug, Clone)]
pub(crate) struct BirthAbiHandoffV1 {
    source_id: crate::parser::ConstructorSourceIdV1,
    object: CanonicalObjectIdV1,
    target: CanonicalSameModuleCallableKeyV1,
    owner: FunctionOwnerIdV1,
    abi: InstanceConstructorAbiV1,
    receiver: BirthFormalLaneV1,
    parameters: Box<[BirthFormalLaneV1]>,
    formal_contracts: Box<[BirthFormalContractV1]>,
    construction: ConstructionEligibilityV1,
    result: BirthResultAbiV1,
}

impl PartialEq for BirthAbiHandoffV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source_id == other.source_id && self.object == other.object
            && self.target == other.target && self.owner == other.owner
            && self.abi == other.abi && self.receiver == other.receiver
            && self.parameters == other.parameters && self.formal_contracts == other.formal_contracts
            && self.result == other.result && self.matches_construction_v1(&other.construction)
    }
}
impl Eq for BirthAbiHandoffV1 {}

impl BirthAbiHandoffV1 {
    pub(in crate::mir) fn construction_plan_v1(&self) -> Option<&ConstructionPlanV1> {
        self.construction.as_ref().ok()
    }

    /// Borrow the sealed Text demand without rewriting the original formal's
    /// declaration, use census or deferred physical disposition. Every original
    /// store of this binding must be covered by the SAME execution projection.
    pub(in crate::mir) fn text_formal_source_v1(
        &self, ordinal: u32,
    ) -> Result<Option<&std::rc::Rc<super::super::SourceValueSumV1>>, String> {
        use super::super::instance_constructor_semantic::{BirthFormalDeclarationClassV1 as D, BirthFormalUseCoverageV1 as U};
        use super::super::instance_construction::ConstructionStoreRhsV1;
        use crate::mir::instruction::FieldSetModeV1;
        let lane = self.parameters.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        let formal = self.formal_contracts.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        if lane.source_ordinal() != Some(ordinal) || lane.binding() != formal.binding()
            || formal.ordinal() != ordinal || formal.binding().owner() != self.owner {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        if !matches!(formal.declaration(), D::Unannotated | D::ExactText) { return Ok(None); }
        let U::UnqualifiedFieldStores { sites } = formal.uses() else { return Ok(None) };
        let Ok(plan) = &self.construction else { return Ok(None) };
        if plan.object() != self.object || !matches!(plan.constructor(), Some((source, owner))
            if source.same_as(&self.source_id) && *owner == self.owner) {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        let mut covered = std::collections::BTreeSet::new();
        let mut input = None;
        for (index, store) in plan.stores().iter().enumerate() {
            let ConstructionStoreRhsV1::Parameter { site, binding, provided } = store.rhs() else { continue };
            if *binding != formal.binding() { continue; }
            let execution = plan.store_execution_v1(index).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
            let Some(sum) = execution.text_input_v1() else { return Ok(None) };
            if provided.is_some() || !sites.contains(site) || !covered.insert(site)
                || execution.mode() != FieldSetModeV1::DynamicTextInitialize || !sum.text_only()
                || input.is_some_and(|prior| !std::rc::Rc::ptr_eq(prior, sum)) {
                return Ok(None);
            }
            input = Some(sum);
        }
        if covered.len() != sites.len() || sites.is_empty() { return Ok(None); }
        Ok(input)
    }

    /// Exact Dynamic Object field formal selected by the same construction
    /// plan. The child identity and nullable tag come from its source-issued
    /// DynamicValue contract, not the ABI carrier or physical value type.
    pub(in crate::mir) fn dynamic_object_formal_source_v1(
        &self,
        ordinal: u32,
    ) -> Result<Option<(CanonicalObjectIdV1, bool)>, String> {
        use super::super::instance_constructor_semantic::{
            BirthFormalDeclarationClassV1 as D, BirthFormalUseCoverageV1 as U,
        };
        use super::super::instance_construction::ConstructionStoreRhsV1;
        use crate::mir::instruction::FieldSetModeV1;
        let lane = self.parameters.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        let formal = self.formal_contracts.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        if lane.source_ordinal() != Some(ordinal) || lane.binding() != formal.binding()
            || formal.ordinal() != ordinal || formal.binding().owner() != self.owner {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        if formal.declaration() != D::Unannotated { return Ok(None); }
        let U::UnqualifiedFieldStores { sites } = formal.uses() else { return Ok(None) };
        let Ok(plan) = &self.construction else { return Ok(None) };
        if plan.object() != self.object || !matches!(plan.constructor(), Some((source, owner))
            if source.same_as(&self.source_id) && *owner == self.owner) {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        let mut covered = std::collections::BTreeSet::new();
        let mut child = None;
        let mut nullable = false;
        for (index, store) in plan.stores().iter().enumerate() {
            let ConstructionStoreRhsV1::Parameter { site, binding, provided } = store.rhs() else { continue };
            if *binding != formal.binding() { continue; }
            let execution = plan.store_execution_v1(index).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
            let (candidate, candidate_nullable) = match execution.mode() {
                FieldSetModeV1::DynamicValue(contract) => {
                    let Some((candidate, candidate_nullable)) = dynamic_object_contract(contract) else { return Ok(None) };
                    (candidate, candidate_nullable)
                }
                _ => return Ok(None),
            };
            if provided.is_some() || !sites.contains(site) || !covered.insert(site)
                || child.is_some_and(|prior| prior != candidate)
                || (child.is_some() && nullable != candidate_nullable) {
                return Ok(None);
            }
            child = Some(candidate);
            nullable = candidate_nullable;
        }
        if covered.len() != sites.len() || sites.is_empty() { return Ok(None); }
        Ok(child.map(|child| (child, nullable)))
    }

    /// Exact signed-I64 Dynamic field formal. The selected mode can only be
    /// issued after the original Birth input source sum proves signed I64.
    pub(in crate::mir) fn dynamic_i64_formal_source_v1(
        &self,
        ordinal: u32,
    ) -> Result<bool, String> {
        use super::super::instance_constructor_semantic::{
            BirthFormalDeclarationClassV1 as D, BirthFormalUseCoverageV1 as U,
        };
        use super::super::instance_construction::ConstructionStoreRhsV1;
        use crate::mir::instruction::FieldSetModeV1;
        let lane = self.parameters.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        let formal = self.formal_contracts.get(ordinal as usize).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
        if lane.source_ordinal() != Some(ordinal) || lane.binding() != formal.binding()
            || formal.ordinal() != ordinal || formal.binding().owner() != self.owner {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        if formal.declaration() != D::Unannotated { return Ok(false); }
        let U::UnqualifiedFieldStores { sites } = formal.uses() else { return Ok(false) };
        let Ok(plan) = &self.construction else { return Ok(false) };
        if plan.object() != self.object || !matches!(plan.constructor(), Some((source, owner))
            if source.same_as(&self.source_id) && *owner == self.owner) {
            return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
        }
        let mut covered = std::collections::BTreeSet::new();
        for (index, store) in plan.stores().iter().enumerate() {
            let ConstructionStoreRhsV1::Parameter { site, binding, provided } = store.rhs() else { continue };
            if *binding != formal.binding() { continue; }
            let execution = plan.store_execution_v1(index).ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
            if provided.is_some() || !sites.contains(site) || !covered.insert(site)
                || execution.mode() != FieldSetModeV1::DynamicI64Initialize {
                return Ok(false);
            }
        }
        Ok(!sites.is_empty() && covered.len() == sites.len())
    }

    fn matches_construction_v1(&self, other: &ConstructionEligibilityV1) -> bool {
        match (&self.construction, other) {
            (Ok(left), Ok(right)) => left.same_execution_v1(right),
            (Err(left), Err(right)) => left == right,
            _ => false,
        }
    }

    /// Prepare a replacement before any ledger entry is changed. The raw
    /// formal-use census and ABI lanes remain the original declaration loan.
    pub(super) fn with_late_construction_plan_v1(
        &self, row: &VerifiedInstanceConstructorSemanticRowV1, plan: &ConstructionPlanV1,
    ) -> Result<Self, &'static str> {
        if !self.source_id.same_as(row.source_id()) || self.object != row.object()
            || row.published_birth_key() != Some(&self.target)
            || self.formal_contracts.as_ref() != row.formal_contracts()
            || !self.matches_construction_v1(row.construction())
            || plan.object() != self.object
            || !matches!(plan.constructor(), Some((source, owner))
                if source.same_as(&self.source_id) && *owner == self.owner)
            || !row.construction_source().as_ref().is_ok_and(|source|
                std::rc::Rc::ptr_eq(source, plan.source_v1())) {
            return Err("[freeze:contract][constructor-unit/new-source-drift]");
        }
        let mut updated = self.clone();
        updated.construction = Ok(plan.clone());
        Ok(updated)
    }

    pub(crate) fn issue(
        row: &VerifiedInstanceConstructorSemanticRowV1,
        target: CanonicalSameModuleCallableKeyV1,
        abi: InstanceConstructorAbiV1,
    ) -> Result<Self, &'static str> {
        let source_arity = usize::try_from(row.source_arity()).map_err(|_| "source-arity")?;
        abi.validate(
            source_arity,
            source_arity.checked_add(1).ok_or("physical-arity")?,
        )
        .map_err(|_| "abi")?;
        if row.published_birth_key() != Some(&target) {
            return Err("target");
        }
        let [owner] = row.forest().roots() else {
            return Err("root-count");
        };
        let owner = row.forest().semantic_owner(*owner).ok_or("root-missing")?;
        if !matches!(
            owner.root_profile(),
            SemanticOwnerRootProfileV1::DeclaredFunction {
                receiver_policy: ReceiverPolicyV1::DeclaredInstance
            }
        ) {
            return Err("root-profile");
        }
        if matches!(row.construction(), Ok(plan)
            if plan.object() != row.object()
                || !matches!(plan.constructor(), Some((source, plan_owner))
                    if source.same_as(row.source_id()) && *plan_owner == owner.owner()))
        {
            return Err("construction-object");
        }
        let mut receiver = None;
        let mut parameters = BTreeMap::new();
        for (binding, record) in owner.bindings() {
            if binding.owner() != owner.owner() {
                return Err("foreign-binding");
            }
            match record.kind() {
                BindingKindV1::Receiver => {
                    if receiver.replace(binding).is_some() {
                        return Err("duplicate-receiver");
                    }
                }
                BindingKindV1::Parameter { index } => {
                    if parameters.insert(index, binding).is_some() {
                        return Err("duplicate-parameter");
                    }
                }
                _ => {}
            }
        }
        let receiver = receiver.ok_or("receiver-missing")?;
        if parameters.len() != source_arity || parameters.keys().copied().ne(0..row.source_arity())
        {
            return Err("parameter-ordinal");
        }
        let receiver = BirthFormalLaneV1 {
            binding: receiver,
            source_ordinal: None,
            physical_lane: 0,
        };
        let parameters = parameters
            .into_iter()
            .map(|(source_ordinal, binding)| {
                Ok(BirthFormalLaneV1 {
                    binding,
                    source_ordinal: Some(source_ordinal),
                    physical_lane: source_ordinal.checked_add(1).ok_or("physical-lane")?,
                })
            })
            .collect::<Result<Vec<_>, &str>>()?
            .into_boxed_slice();
        if row.formal_contracts().len() != parameters.len()
            || row
                .formal_contracts()
                .iter()
                .zip(parameters.iter())
                .any(|(contract, lane)| {
                    lane.source_ordinal() != Some(contract.ordinal())
                        || contract.binding() != lane.binding()
                })
        {
            return Err("formal-contract");
        }
        Ok(Self {
            source_id: row.source_id().clone(),
            object: row.object(),
            target,
            owner: owner.owner(),
            abi,
            receiver,
            parameters,
            formal_contracts: row.formal_contracts().to_vec().into_boxed_slice(),
            construction: row.construction().clone(),
            result: BirthResultAbiV1::Unit,
        })
    }

    pub(crate) fn source_id(&self) -> &crate::parser::ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn abi(&self) -> InstanceConstructorAbiV1 {
        self.abi
    }

    pub(crate) const fn receiver(&self) -> BirthFormalLaneV1 {
        self.receiver
    }

    pub(crate) fn parameters(&self) -> &[BirthFormalLaneV1] {
        &self.parameters
    }

    pub(crate) fn formal_contracts(&self) -> &[BirthFormalContractV1] {
        &self.formal_contracts
    }

    pub(crate) const fn result(&self) -> BirthResultAbiV1 {
        self.result
    }
}

fn dynamic_object_contract(
    contract: crate::mir::instruction::DynamicFieldSetContractV1,
) -> Option<(CanonicalObjectIdV1, bool)> {
    use crate::mir::instruction::{DynamicFieldResidenceV1 as Residence,
        DynamicFieldValueTagV1 as Tag, DynamicFieldWriteIntentV1 as Intent};
    if contract.intent() != Intent::Initialize { return None; }
    match (contract.value_tag(), contract.residence()) {
        (Tag::Object, Residence::ObjectHome(child)) => Some((child, false)),
        (Tag::NullableObject, Residence::ObjectHome(child)) => Some((child, true)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_issued_birth_lanes_are_receiver_then_ordered_parameters() {
        let package = super::super::super::brand_catalog_tests::issue_with_brand_catalog(
            "box Pair { birth(left, right) { } } static box Main { main() { local pair = new Pair(10, 20) return pair.left + pair.right } }",
        )
        .expect("source package");
        let row = package
            .instance_constructors()
            .rows()
            .iter()
            .find(|row| row.box_name() == "Pair")
            .expect("Pair Birth row");
        let target = row
            .published_birth_key()
            .expect("published Birth key")
            .clone();
        let handoff = BirthAbiHandoffV1::issue(
            row,
            target.clone(),
            InstanceConstructorAbiV1::issue(2).expect("N+1 ABI"),
        )
        .expect("source-issued ABI relation");
        assert_eq!(handoff.target(), &target);
        assert_eq!(handoff.object(), row.object());
        assert_eq!(handoff.formal_contracts(), row.formal_contracts());
        assert_eq!(handoff.receiver().source_ordinal(), None);
        assert_eq!(handoff.receiver().physical_lane(), 0);
        assert_eq!(
            handoff
                .parameters()
                .iter()
                .map(|formal| (formal.source_ordinal(), formal.physical_lane()))
                .collect::<Vec<_>>(),
            vec![(Some(0), 1), (Some(1), 2)]
        );
        assert_eq!(handoff.result(), BirthResultAbiV1::Unit);
        // Reissuing from this same source row lends its existing execution;
        // the final provider/claim join must not reconstruct that permission.
        assert_eq!(handoff, BirthAbiHandoffV1::issue(
            row, target, InstanceConstructorAbiV1::issue(2).expect("same ABI"),
        ).expect("same source loan"));
    }
}
