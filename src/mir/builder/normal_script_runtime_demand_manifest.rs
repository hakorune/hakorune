//! Runtime-side view of the work-plan-issued constructor role tickets.

use super::PreparedNormalScriptRuntimeWorkV1;
use crate::mir::builder::normal_instance_constructor_admission::InstanceConstructorDemandTicketV1;

impl PreparedNormalScriptRuntimeWorkV1 {
    pub(in crate::mir::builder) fn constructor_demand_tickets(
        &self,
    ) -> Vec<InstanceConstructorDemandTicketV1> {
        self.admissions
            .iter()
            .filter_map(|entry| match &entry.admission {
                super::NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility {
                    constructor_sources: Some(sources),
                    ..
                }
                | super::NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle {
                    constructor_sources: Some(sources),
                    ..
                } => Some(sources.demand_tickets()),
                _ => None,
            })
            .flatten()
            .collect()
    }
}
