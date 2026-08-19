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

#[derive(Debug, PartialEq, Eq)]
pub struct InstanceConstructorDemandTicketV1 {
    source_id: ConstructorSourceIdV1,
    role: InstanceConstructorDemandRoleV1,
}

impl InstanceConstructorDemandTicketV1 {
    pub fn new(source_id: &ConstructorSourceIdV1, role: InstanceConstructorDemandRoleV1) -> Self {
        Self {
            source_id: source_id.clone(),
            role,
        }
    }

    pub fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }

    pub const fn role(&self) -> InstanceConstructorDemandRoleV1 {
        self.role
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.role == other.role && self.source_id.same_as(&other.source_id)
    }
}

#[derive(Debug, Default)]
pub struct InstanceConstructorDemandManifestBuilderV1 {
    tickets: Vec<InstanceConstructorDemandTicketV1>,
}

#[derive(Debug)]
pub struct VerifiedInstanceConstructorPhysicalDemandManifestV1 {
    tickets: Box<[InstanceConstructorDemandTicketV1]>,
}

impl InstanceConstructorDemandManifestBuilderV1 {
    pub fn issue_batch(
        &mut self,
        batch: &NormalInstanceConstructorSourceBatchV1,
    ) -> Result<(), String> {
        for source in batch.sources() {
            let ticket = InstanceConstructorDemandTicketV1::new(source.source_id(), batch.role());
            if self
                .tickets
                .iter()
                .any(|existing| existing.same_as(&ticket))
            {
                return Err(
                    "[freeze:contract][mir/instance-constructor-demand/duplicate-ticket]"
                        .to_owned(),
                );
            }
            self.tickets.push(ticket);
        }
        Ok(())
    }

    pub fn finish(self) -> VerifiedInstanceConstructorPhysicalDemandManifestV1 {
        VerifiedInstanceConstructorPhysicalDemandManifestV1 {
            tickets: self.tickets.into_boxed_slice(),
        }
    }
}

impl VerifiedInstanceConstructorPhysicalDemandManifestV1 {
    pub fn tickets(&self) -> &[InstanceConstructorDemandTicketV1] {
        &self.tickets
    }

    pub fn validate_exact(
        &self,
        actual: &[InstanceConstructorDemandTicketV1],
    ) -> Result<(), String> {
        if self.tickets.len() != actual.len() {
            return Err("[freeze:contract][mir/instance-constructor-demand/count]".to_owned());
        }
        for expected in &self.tickets {
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
                .tickets
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
