//! Semantic outcome contracts for explicitly accepted extern operations.

use super::route_spec::ExternCallRouteKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternSuccessOutcome {
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternResultPolicy {
    NoPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternValueUsePolicy {
    StatementOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternOutcomeSpec {
    pub contract_id: &'static str,
    pub route_id: &'static str,
    pub source_site: &'static str,
    pub success_outcome: ExternSuccessOutcome,
    pub result_policy: ExternResultPolicy,
    pub value_use_policy: ExternValueUsePolicy,
    pub required_capability: &'static str,
}

pub const HAKO_MEM_FREE_OUTCOME: ExternOutcomeSpec = ExternOutcomeSpec {
    contract_id: "extern-outcome:hako-mem-free:v1",
    route_id: "extern.hako_mem.free",
    source_site: "runtime.hako_mem.hako_mem_free.success",
    success_outcome: ExternSuccessOutcome::Unit,
    result_policy: ExternResultPolicy::NoPayload,
    value_use_policy: ExternValueUsePolicy::StatementOnly,
    required_capability: "extern_unit_no_payload_hako_mem_free_v1",
};

pub fn extern_outcome_spec(kind: ExternCallRouteKind) -> Option<&'static ExternOutcomeSpec> {
    match kind {
        ExternCallRouteKind::HakoMemFree => Some(&HAKO_MEM_FREE_OUTCOME),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hako_mem_free_contract_is_unit_no_payload_statement_only() {
        let spec = extern_outcome_spec(ExternCallRouteKind::HakoMemFree)
            .expect("hako_mem_free outcome contract");
        assert_eq!(spec.success_outcome, ExternSuccessOutcome::Unit);
        assert_eq!(spec.result_policy, ExternResultPolicy::NoPayload);
        assert_eq!(spec.value_use_policy, ExternValueUsePolicy::StatementOnly);
        assert_eq!(spec.route_id, "extern.hako_mem.free");
    }

    #[test]
    fn unselected_extern_routes_have_no_semantic_activation_contract() {
        assert!(extern_outcome_spec(ExternCallRouteKind::EnvGet).is_none());
    }
}
