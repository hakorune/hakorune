//! Source-issued Birth formal lanes retained until the final backend handoff.
//!
//! This relation records source bindings and their explicit ABI lanes.  It has
//! no `ValueId` or function-parameter authority: the final view validates the
//! already-published physical layout before a transport projects values.

use super::super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticRowV1;
use super::super::instance_constructor_semantic::BirthFormalContractV1;
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BirthAbiHandoffV1 {
    source_id: crate::parser::ConstructorSourceIdV1,
    object: CanonicalObjectIdV1,
    target: CanonicalSameModuleCallableKeyV1,
    owner: FunctionOwnerIdV1,
    abi: InstanceConstructorAbiV1,
    receiver: BirthFormalLaneV1,
    parameters: Box<[BirthFormalLaneV1]>,
    formal_contracts: Box<[BirthFormalContractV1]>,
    result: BirthResultAbiV1,
}

impl BirthAbiHandoffV1 {
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
    }
}
