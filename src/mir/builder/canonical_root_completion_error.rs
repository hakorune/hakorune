//! Shared error vocabulary for disconnected canonical root completion.

use super::module_draft_collector::FunctionDraftKeyV1;
use super::module_invocation_identity::ModuleInvocationFamilyV1;
use super::module_lowering_shell::ModuleLoweringShellErrorV1;
use crate::mir::builder::module_invocation_callable_batch::CallableBatchSourceErrorV1;
use crate::mir::compiler::capability::ResolvedOwnerHeaderSealErrorV1;
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalSourceBindingErrorV1 {
    FamilyMismatch {
        expected: ModuleInvocationFamilyV1,
        actual: ModuleInvocationFamilyV1,
    },
    Header(ResolvedOwnerHeaderSealErrorV1),
    RoutePolicy,
    CallablePlan(CallableBatchSourceErrorV1),
}

impl std::fmt::Display for CanonicalSourceBindingErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][canonical_source_binding] {self:?}"
        )
    }
}

impl std::error::Error for CanonicalSourceBindingErrorV1 {}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CanonicalCompletionErrorV1 {
    Shell(ModuleLoweringShellErrorV1),
    ForeignBrand {
        expected: u64,
        actual: u64,
    },
    CollectorCardinality {
        expected: usize,
        actual: usize,
    },
    MissingReceipt,
    SyntheticRoot(FunctionDraftKeyV1),
    KeyMismatch,
    SymbolMismatch {
        expected: String,
        actual: String,
    },
    ArityMismatch {
        expected: usize,
        actual: usize,
    },
    PolicyMismatch,
    ReplacementForbidden,
    CallableCardinality {
        expected: usize,
        actual: usize,
    },
    CallableMissing(CanonicalCallableKeyV1),
    CallableKeyMismatch,
    CallableSymbolMismatch,
    CallableArityMismatch,
    CallablePolicyMismatch,
    CallableReplacementForbidden,
    RecursiveCapability(&'static str),
    CapabilityFamilyMismatch(ModuleInvocationFamilyV1),
    CapabilityBrandMismatch {
        expected: u64,
        actual: u64,
    },
    CapabilityWitnessFamilyMismatch {
        expected: ModuleInvocationFamilyV1,
        actual: ModuleInvocationFamilyV1,
    },
}

impl std::fmt::Display for CanonicalCompletionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][canonical_completion] {self:?}"
        )
    }
}

impl std::error::Error for CanonicalCompletionErrorV1 {}
