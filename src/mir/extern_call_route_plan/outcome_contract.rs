//! Semantic outcomes, activation, and backend support for selected externs.

use super::route_spec::ExternCallRouteKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternOutcomeContractId {
    HakoMemFreeV1,
}

impl ExternOutcomeContractId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HakoMemFreeV1 => "extern-outcome:hako-mem-free:v1",
        }
    }
}

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
    pub contract_id: ExternOutcomeContractId,
    pub route_id: &'static str,
    pub source_site: &'static str,
    pub success_outcome: ExternSuccessOutcome,
    pub result_policy: ExternResultPolicy,
    pub value_use_policy: ExternValueUsePolicy,
}

pub const HAKO_MEM_FREE_OUTCOME: ExternOutcomeSpec = ExternOutcomeSpec {
    contract_id: ExternOutcomeContractId::HakoMemFreeV1,
    route_id: "extern.hako_mem.free",
    source_site: "runtime_backend.extern.hako_mem_free.success",
    success_outcome: ExternSuccessOutcome::Unit,
    result_policy: ExternResultPolicy::NoPayload,
    value_use_policy: ExternValueUsePolicy::StatementOnly,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternOutcomeActivation {
    pub contract_id: ExternOutcomeContractId,
}

pub const ACTIVE_EXTERN_OUTCOMES: &[ExternOutcomeActivation] = &[ExternOutcomeActivation {
    contract_id: ExternOutcomeContractId::HakoMemFreeV1,
}];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternOutcomeConsumer {
    NyLlvmObject,
    ReferenceVm,
    Wasm,
    PyVm,
}

impl ExternOutcomeConsumer {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NyLlvmObject => "ny-llvmc-object",
            Self::ReferenceVm => "reference-vm",
            Self::Wasm => "wasm",
            Self::PyVm => "pyvm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternOutcomeCapability {
    UnitNoPayloadHakoMemFreeV1,
}

impl ExternOutcomeCapability {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitNoPayloadHakoMemFreeV1 => "extern_unit_no_payload_hako_mem_free_v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternOutcomeProjection {
    CVoidNoResultDiscardOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternOutcomeBackendSupport {
    pub contract_id: ExternOutcomeContractId,
    pub consumer: ExternOutcomeConsumer,
    pub capability: ExternOutcomeCapability,
    pub projection: ExternOutcomeProjection,
}

pub const HAKO_MEM_FREE_LLVM_OBJECT_SUPPORT: ExternOutcomeBackendSupport =
    ExternOutcomeBackendSupport {
        contract_id: ExternOutcomeContractId::HakoMemFreeV1,
        consumer: ExternOutcomeConsumer::NyLlvmObject,
        capability: ExternOutcomeCapability::UnitNoPayloadHakoMemFreeV1,
        projection: ExternOutcomeProjection::CVoidNoResultDiscardOnly,
    };

pub const EXTERN_OUTCOME_BACKEND_SUPPORT: &[ExternOutcomeBackendSupport] =
    &[HAKO_MEM_FREE_LLVM_OBJECT_SUPPORT];

pub fn extern_outcome_spec(kind: ExternCallRouteKind) -> Option<&'static ExternOutcomeSpec> {
    match kind {
        ExternCallRouteKind::HakoMemFree => Some(&HAKO_MEM_FREE_OUTCOME),
        _ => None,
    }
}

pub fn extern_outcome_activation(
    kind: ExternCallRouteKind,
) -> Option<&'static ExternOutcomeActivation> {
    let spec = extern_outcome_spec(kind)?;
    ACTIVE_EXTERN_OUTCOMES
        .iter()
        .find(|activation| activation.contract_id == spec.contract_id)
}

pub fn extern_outcome_backend_support(
    kind: ExternCallRouteKind,
    consumer: ExternOutcomeConsumer,
) -> Option<&'static ExternOutcomeBackendSupport> {
    let spec = extern_outcome_spec(kind)?;
    EXTERN_OUTCOME_BACKEND_SUPPORT
        .iter()
        .find(|support| support.contract_id == spec.contract_id && support.consumer == consumer)
}

pub fn validate_extern_outcome_backend_support(
    kind: ExternCallRouteKind,
    consumer: ExternOutcomeConsumer,
) -> Result<&'static ExternOutcomeBackendSupport, String> {
    if extern_outcome_activation(kind).is_none() {
        return Err(format!(
            "[failure/outcome_activation_missing] route={}",
            kind.route_id()
        ));
    }
    extern_outcome_backend_support(kind, consumer).ok_or_else(|| {
        format!(
            "[failure/outcome_unit_backend_unsupported] route={} consumer={}",
            kind.route_id(),
            consumer.as_str()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hako_mem_free_contract_is_backend_independent_unit_no_payload() {
        let spec = extern_outcome_spec(ExternCallRouteKind::HakoMemFree)
            .expect("hako_mem_free outcome contract");
        assert_eq!(spec.contract_id.as_str(), "extern-outcome:hako-mem-free:v1");
        assert_eq!(spec.success_outcome, ExternSuccessOutcome::Unit);
        assert_eq!(spec.result_policy, ExternResultPolicy::NoPayload);
        assert_eq!(spec.value_use_policy, ExternValueUsePolicy::StatementOnly);
        assert!(extern_outcome_activation(ExternCallRouteKind::HakoMemFree).is_some());
    }

    #[test]
    fn llvm_support_is_separate_from_semantic_activation() {
        let support = extern_outcome_backend_support(
            ExternCallRouteKind::HakoMemFree,
            ExternOutcomeConsumer::NyLlvmObject,
        )
        .expect("LLVM/object support");
        assert_eq!(
            support.capability.as_str(),
            "extern_unit_no_payload_hako_mem_free_v1"
        );
        assert_eq!(support.consumer.as_str(), "ny-llvmc-object");
        assert!(extern_outcome_backend_support(
            ExternCallRouteKind::HakoMemFree,
            ExternOutcomeConsumer::ReferenceVm,
        )
        .is_none());
        assert!(extern_outcome_backend_support(
            ExternCallRouteKind::HakoMemFree,
            ExternOutcomeConsumer::Wasm,
        )
        .is_none());
    }

    #[test]
    fn unselected_extern_routes_have_no_semantic_activation_contract() {
        assert!(extern_outcome_spec(ExternCallRouteKind::EnvGet).is_none());
        assert!(extern_outcome_activation(ExternCallRouteKind::EnvGet).is_none());
    }
}
