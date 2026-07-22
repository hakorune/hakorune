//! CUT0-I0-COLLECT0-BATCH0: callable source/collector co-seal.
//!
//! The verified callable module remains the source authority. This terminal
//! compares its catalog headers with one branded physical collector and one
//! branded whole-batch receipt. It is disconnected from all production
//! callable ingress until the later atomic CUT0.

use super::module_draft_collector::{
    CallableCollectorBatchReceiptV1, CompletedDraftSignatureViewV1, DraftPublicationPolicyV1,
    FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, InvocationBranded};
use crate::mir::canonical_recursive_callable_module_capability::
    CanonicalRecursiveCallableModuleCapabilityV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::compiler::{
    acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1,
    recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1,
};
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableBatchSourcePayloadV1<'a> {
    token: ModuleInvocationTokenV1,
    source: &'a VerifiedResolvedCallableModuleV1,
    shell_fact: CallableBatchShellFactV1,
}

pub(in crate::mir::builder) type CallableBatchSourceProofV1<'a> =
    InvocationBranded<CallableBatchSourcePayloadV1<'a>>;
pub(in crate::mir::builder) type CallableBatchPhysicalReceiptV1 =
    InvocationBranded<CallableCollectorBatchReceiptV1>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableBatchShellFactV1 {
    Acyclic,
    Recursive {
        capability: CanonicalRecursiveCallableModuleCapabilityV1,
    },
}

impl CallableBatchShellFactV1 {
    pub(in crate::mir::builder) const fn is_recursive(self) -> bool {
        matches!(self, Self::Recursive { .. })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableBatchSourceErrorV1 {
    UnsupportedFamily(ModuleInvocationFamilyV1),
    SourcePlanMismatch { family: ModuleInvocationFamilyV1 },
    RecursiveCapabilityMissing,
    RecursiveCapabilityUnexpected,
    RecursiveCapabilityInvalid,
}

impl std::fmt::Display for CallableBatchSourceErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][callable_batch/source] {self:?}")
    }
}

impl std::error::Error for CallableBatchSourceErrorV1 {}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableBatchSealErrorV1 {
    ForeignOwner { expected: u64, actual: u64 },
    CardinalityMismatch { expected: usize, actual: usize },
    MissingRow { key: CanonicalCallableKeyV1 },
    SurplusRow { symbol: String },
    KeyMismatch { symbol: String },
    SymbolMismatch { expected: String, actual: String },
    ArityMismatch { symbol: String, expected: usize, actual: usize },
    PolicyMismatch { symbol: String },
    ReplacementForbidden { symbol: String },
}

impl std::fmt::Display for CallableBatchSealErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][callable_batch/seal] {self:?}")
    }
}

impl std::error::Error for CallableBatchSealErrorV1 {}

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableBatchCollectedInvocationDraftSetV1<'a> {
    source: CallableBatchSourceProofV1<'a>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    receipt: CallableBatchPhysicalReceiptV1,
    _seal: CallableBatchCollectedSealV1,
}

#[derive(Debug)]
struct CallableBatchCollectedSealV1;

impl CallableBatchCollectedInvocationDraftSetV1<'_> {
    pub(in crate::mir::builder) fn receipt_count(&self) -> usize {
        self.receipt.payload().len()
    }

    pub(in crate::mir::builder) fn is_recursive(&self) -> bool {
        self.source.payload().shell_fact.is_recursive()
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn source_from_test<'a>(
    token: ModuleInvocationTokenV1,
    source: &'a VerifiedResolvedCallableModuleV1,
    recursive_capability: Option<CanonicalRecursiveCallableModuleCapabilityV1>,
) -> Result<CallableBatchSourceProofV1<'a>, CallableBatchSourceErrorV1> {
    let shell_fact = match token.family() {
        ModuleInvocationFamilyV1::BindingSsaAcyclic => {
            if VerifiedAcyclicCallableModulePlanV1::verify(source).is_err() {
                return Err(CallableBatchSourceErrorV1::SourcePlanMismatch {
                    family: ModuleInvocationFamilyV1::BindingSsaAcyclic,
                });
            }
            if recursive_capability.is_some() {
                return Err(CallableBatchSourceErrorV1::RecursiveCapabilityUnexpected);
            }
            CallableBatchShellFactV1::Acyclic
        }
        ModuleInvocationFamilyV1::BindingSsaRecursive => {
            if VerifiedRecursiveCallableModulePlanV1::verify(source).is_err() {
                return Err(CallableBatchSourceErrorV1::SourcePlanMismatch {
                    family: ModuleInvocationFamilyV1::BindingSsaRecursive,
                });
            }
            let capability = recursive_capability
                .ok_or(CallableBatchSourceErrorV1::RecursiveCapabilityMissing)?;
            CanonicalRecursiveCallableModuleCapabilityV1::verify_required(Some(&capability))
                .map_err(|_| CallableBatchSourceErrorV1::RecursiveCapabilityInvalid)?;
            CallableBatchShellFactV1::Recursive { capability }
        }
        family => return Err(CallableBatchSourceErrorV1::UnsupportedFamily(family)),
    };
    Ok(InvocationBranded::from_test(
        token.brand(),
        CallableBatchSourcePayloadV1 {
            token,
            source,
            shell_fact,
        },
    ))
}

#[cfg(test)]
pub(in crate::mir::builder) fn physical_receipt_from_test(
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    receipt: CallableCollectorBatchReceiptV1,
) -> CallableBatchPhysicalReceiptV1 {
    InvocationBranded::from_test(brand, receipt)
}

#[cfg(test)]
pub(in crate::mir::builder) fn shell_fact_from_test(
    family: ModuleInvocationFamilyV1,
    capability: Option<CanonicalRecursiveCallableModuleCapabilityV1>,
) -> Result<CallableBatchShellFactV1, CallableBatchSourceErrorV1> {
    match family {
        ModuleInvocationFamilyV1::BindingSsaAcyclic if capability.is_none() => {
            Ok(CallableBatchShellFactV1::Acyclic)
        }
        ModuleInvocationFamilyV1::BindingSsaRecursive => {
            let capability = capability.ok_or(CallableBatchSourceErrorV1::RecursiveCapabilityMissing)?;
            CanonicalRecursiveCallableModuleCapabilityV1::verify_required(Some(&capability))
                .map_err(|_| CallableBatchSourceErrorV1::RecursiveCapabilityInvalid)?;
            Ok(CallableBatchShellFactV1::Recursive { capability })
        }
        ModuleInvocationFamilyV1::BindingSsaAcyclic => {
            Err(CallableBatchSourceErrorV1::RecursiveCapabilityUnexpected)
        }
        family => Err(CallableBatchSourceErrorV1::UnsupportedFamily(family)),
    }
}

pub(in crate::mir::builder) fn seal_callable_batch<'a>(
    source: CallableBatchSourceProofV1<'a>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    receipt: CallableBatchPhysicalReceiptV1,
) -> Result<CallableBatchCollectedInvocationDraftSetV1<'a>, CallableBatchSealErrorV1> {
    let brand = source.brand();
    if brand != collector.brand() {
        return Err(CallableBatchSealErrorV1::ForeignOwner {
            expected: brand.ordinal(),
            actual: collector.brand().ordinal(),
        });
    }
    if brand != receipt.brand() {
        return Err(CallableBatchSealErrorV1::ForeignOwner {
            expected: brand.ordinal(),
            actual: receipt.brand().ordinal(),
        });
    }
    let expected = source.payload().source.functions_by_key().len();
    if expected != source.payload().source.source().catalog().len()
        || expected != collector.payload().symbol_count()
        || expected != receipt.payload().len()
    {
        return Err(CallableBatchSealErrorV1::CardinalityMismatch {
            expected,
            actual: receipt.payload().len(),
        });
    }
    for (key, _) in source.payload().source.functions_by_key() {
        let header = source
            .payload()
            .source
            .source()
            .catalog()
            .index()
            .lookup(key)
            .ok_or_else(|| CallableBatchSealErrorV1::MissingRow { key: key.clone() })?;
        let symbol = header.symbol().as_mir_name().to_owned();
        let expected_key = FunctionDraftKeyV1::CanonicalCallable(key.clone());
        if collector.payload().key_for_symbol(&symbol) != Some(&expected_key) {
            return Err(CallableBatchSealErrorV1::KeyMismatch { symbol });
        }
        let physical = receipt
            .payload()
            .admissions()
            .iter()
            .find(|admission| admission.symbol() == symbol)
            .ok_or_else(|| CallableBatchSealErrorV1::MissingRow { key: key.clone() })?;
        if physical.key() != &expected_key {
            return Err(CallableBatchSealErrorV1::KeyMismatch { symbol });
        }
        if physical.symbol() != symbol {
            return Err(CallableBatchSealErrorV1::SymbolMismatch {
                expected: symbol,
                actual: physical.symbol().to_owned(),
            });
        }
        if physical.arity() != header.signature().arity() {
            return Err(CallableBatchSealErrorV1::ArityMismatch {
                symbol,
                expected: header.signature().arity(),
                actual: physical.arity(),
            });
        }
        if physical.policy() != DraftPublicationPolicyV1::CanonicalRejectDuplicate {
            return Err(CallableBatchSealErrorV1::PolicyMismatch { symbol });
        }
        if !matches!(
            physical.replacement(),
            super::module_draft_collector::CollectedDraftReplacementDispositionV1::Inserted
        ) {
            return Err(CallableBatchSealErrorV1::ReplacementForbidden { symbol });
        }
    }
    Ok(CallableBatchCollectedInvocationDraftSetV1 {
        source,
        collector,
        receipt,
        _seal: CallableBatchCollectedSealV1,
    })
}
