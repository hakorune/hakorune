//! Work-plan-owned role tickets for selected-normal instance constructors.
//!
//! Source identity comes from the parser-backed semantic package.  This
//! module only records which physical demand is allowed to consume that
//! identity; it does not issue semantic rows or choose a constructor.

use crate::parser::ConstructorSourceIdV1;

use super::NormalInstanceConstructorSourceBatchV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstanceConstructorDemandRoleV1 {
    ImmediateDeclaration,
    ScriptRuntimePrefix,
    ScriptRuntimeFullLifecycle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct InstanceConstructorDemandExpectationV1 {
    source_id: ConstructorSourceIdV1,
    role: InstanceConstructorDemandRoleV1,
}

impl InstanceConstructorDemandExpectationV1 {
    pub(in crate::mir::builder) fn new(
        source_id: &ConstructorSourceIdV1,
        role: InstanceConstructorDemandRoleV1,
    ) -> Self {
        Self {
            source_id: source_id.clone(),
            role,
        }
    }

    pub(in crate::mir::builder) fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(in crate::mir::builder) const fn role(&self) -> InstanceConstructorDemandRoleV1 {
        self.role
    }

    pub(in crate::mir::builder) fn same_as(&self, other: &Self) -> bool {
        self.role == other.role && self.source_id.same_as(&other.source_id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct InstanceConstructorDemandTicketV1 {
    source_id: ConstructorSourceIdV1,
    role: InstanceConstructorDemandRoleV1,
}

impl InstanceConstructorDemandTicketV1 {
    pub(in crate::mir::builder) fn issue(
        source_id: &ConstructorSourceIdV1,
        role: InstanceConstructorDemandRoleV1,
    ) -> Self {
        Self {
            source_id: source_id.clone(),
            role,
        }
    }

    pub(in crate::mir::builder) fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(in crate::mir::builder) const fn role(&self) -> InstanceConstructorDemandRoleV1 {
        self.role
    }

    pub(in crate::mir::builder) fn same_as(&self, other: &Self) -> bool {
        self.role == other.role && self.source_id.same_as(&other.source_id)
    }
}

#[derive(Debug, Default)]
pub struct InstanceConstructorDemandManifestBuilderV1 {
    expectations: Vec<InstanceConstructorDemandExpectationV1>,
}

#[derive(Debug)]
pub struct VerifiedInstanceConstructorPhysicalDemandManifestV1 {
    expectations: Box<[InstanceConstructorDemandExpectationV1]>,
}

impl InstanceConstructorDemandManifestBuilderV1 {
    pub fn issue_batch(
        &mut self,
        batch: &NormalInstanceConstructorSourceBatchV1,
    ) -> Result<(), String> {
        for expectation in batch.demand_expectations() {
            if self
                .expectations
                .iter()
                .any(|existing| existing.same_as(&expectation))
            {
                return Err(
                    "[freeze:contract][mir/instance-constructor-demand/duplicate-ticket]"
                        .to_owned(),
                );
            }
            self.expectations.push(expectation);
        }
        Ok(())
    }

    pub fn finish(self) -> VerifiedInstanceConstructorPhysicalDemandManifestV1 {
        VerifiedInstanceConstructorPhysicalDemandManifestV1 {
            expectations: self.expectations.into_boxed_slice(),
        }
    }
}

impl VerifiedInstanceConstructorPhysicalDemandManifestV1 {
    pub(in crate::mir::builder) fn expectations(
        &self,
    ) -> &[InstanceConstructorDemandExpectationV1] {
        &self.expectations
    }

    pub fn validate_exact(
        &self,
        actual: &[InstanceConstructorDemandExpectationV1],
    ) -> Result<(), String> {
        if self.expectations.len() != actual.len() {
            return Err("[freeze:contract][mir/instance-constructor-demand/count]".to_owned());
        }
        for expected in &self.expectations {
            if actual
                .iter()
                .filter(|candidate| candidate.same_as(expected))
                .count()
                != 1
            {
                return Err(
                    "[freeze:contract][mir/instance-constructor-demand/coverage]".to_owned(),
                );
            }
        }
        for candidate in actual {
            if self
                .expectations
                .iter()
                .filter(|expected| expected.same_as(candidate))
                .count()
                != 1
            {
                return Err("[freeze:contract][mir/instance-constructor-demand/foreign]".to_owned());
            }
        }
        Ok(())
    }
}
