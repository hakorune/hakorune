//! CUT0-I0-ROOT0-CANON0: source-bound canonical completion products.
//!
//! This module is disconnected from public compiler ingress.  It consumes a
//! preflight package once, keeps the real BRAND0 shell/collector owner, and
//! emits route-specific completion products without reusing Raw Main state.
use super::module_draft_collector::{
    CallableCollectorBatchPrepareErrorV1, CallableCollectorBatchReceiptV1,
    CallableCollectorDraftEntryV1, CollectedCallableCollectorBatchV1,
    CollectedDraftAdmissionProductV1, CollectedDraftAdmissionReceiptV1,
    CollectedDraftReplacementDispositionV1, CompletedDraftSignatureViewV1,
    DraftPublicationPolicyV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
    RejectedCollectedDraftAdmissionV1,
};
use super::module_invocation_brand0::ActiveModuleInvocationV1;
use super::module_invocation_callable_batch::CallableBatchShellFactV1;
use super::module_invocation_identity::{ModuleInvocationFamilyV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1, InvocationBranded};
use super::module_invocation_session::{
    BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use super::module_lowering_shell::{
    AcyclicCapabilityAbsenceWitnessV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    RecursiveCapabilityInstallReceiptV1,
};
use super::route_owned_invocation_inventory::RouteOwnedInvocationInventoryV2;
use crate::mir::builder::module_invocation_callable_batch::CallableBatchSourceErrorV1;
use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1,
    VerifiedResolvedOwnerHeaderV1,
};
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::compiler::{
    acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1,
    recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1,
};
use crate::mir::function::MirFunction;
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

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalSingleSourceContinuationV1 {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    header: VerifiedResolvedOwnerHeaderV1,
    policy: RouteOwnedInvocationInventoryV2,
    _seal: CanonicalSingleSourceContinuationSealV1,
}
#[derive(Debug)]
struct CanonicalSingleSourceContinuationSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCanonicalSingleSourceV1<'a> {
    token: ModuleInvocationTokenV1,
    plan: Option<CanonicalFirstFamilyPlanV1<'a>>,
    continuation: CanonicalSingleSourceContinuationV1,
    _seal: PreparedCanonicalSingleSourceSealV1,
}
#[derive(Debug)]
struct PreparedCanonicalSingleSourceSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct BrandedCanonicalSingleLoweringPlanV1<'a> {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    plan: CanonicalFirstFamilyPlanV1<'a>,
    _seal: BrandedCanonicalSingleLoweringPlanSealV1,
}
#[derive(Debug)]
struct BrandedCanonicalSingleLoweringPlanSealV1;

impl<'a> PreparedCanonicalSingleSourceV1<'a> {
    pub(in crate::mir::builder) fn prepare(
        token: ModuleInvocationTokenV1,
        plan: CanonicalFirstFamilyPlanV1<'a>,
    ) -> Result<Self, CanonicalSourceBindingErrorV1> {
        let header = plan
            .seal_resolved_owner_header_v1()
            .map_err(CanonicalSourceBindingErrorV1::Header)?;
        let expected = match token.family() {
            ModuleInvocationFamilyV1::CanonicalAPlus => {
                ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus
            }
            ModuleInvocationFamilyV1::BindingSsaTrivial => {
                ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
            }
            actual => {
                return Err(CanonicalSourceBindingErrorV1::FamilyMismatch {
                    expected: ModuleInvocationFamilyV1::CanonicalAPlus,
                    actual,
                })
            }
        };
        if header.family() != expected {
            return Err(CanonicalSourceBindingErrorV1::FamilyMismatch {
                expected: token.family(),
                actual: match header.family() {
                    ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus => {
                        ModuleInvocationFamilyV1::CanonicalAPlus
                    }
                    ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa => {
                        ModuleInvocationFamilyV1::BindingSsaTrivial
                    }
                },
            });
        }
        let policy = RouteOwnedInvocationInventoryV2::derive(token.family())
            .map_err(|_| CanonicalSourceBindingErrorV1::RoutePolicy)?;
        let brand = token.brand();
        Ok(Self {
            token,
            plan: Some(plan),
            continuation: CanonicalSingleSourceContinuationV1 {
                brand,
                header,
                policy,
                _seal: CanonicalSingleSourceContinuationSealV1,
            },
            _seal: PreparedCanonicalSingleSourceSealV1,
        })
    }

    pub(in crate::mir::builder) fn split(
        mut self,
    ) -> (
        ModuleInvocationTokenV1,
        BrandedCanonicalSingleLoweringPlanV1<'a>,
        CanonicalSingleSourceContinuationV1,
    ) {
        let plan = self.plan.take().expect("source package split once");
        (
            self.token,
            BrandedCanonicalSingleLoweringPlanV1 {
                brand: self.continuation.brand,
                plan,
                _seal: BrandedCanonicalSingleLoweringPlanSealV1,
            },
            self.continuation,
        )
    }
}
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

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalSingleRootWitnessV1 {
    source: CanonicalSingleSourceContinuationV1,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    _seal: CanonicalSingleRootWitnessSealV1,
}
#[derive(Debug)]
struct CanonicalSingleRootWitnessSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CanonicalSingleDrainPlanV1 {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
    _seal: CanonicalSingleDrainPlanSealV1,
}
#[derive(Debug)]
struct CanonicalSingleDrainPlanSealV1;

pub(in crate::mir::builder) struct CanonicalSingleCompleteInvocationV1 {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    root: CanonicalSingleRootWitnessV1,
    drain_plan: CanonicalSingleDrainPlanV1,
    _seal: CanonicalSingleCompleteInvocationSealV1,
}
#[derive(Debug)]
struct CanonicalSingleCompleteInvocationSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableBatchSourceContinuationV1<'a> {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    source: &'a VerifiedResolvedCallableModuleV1,
    policy: RouteOwnedInvocationInventoryV2,
    _seal: CallableBatchSourceContinuationSealV1,
}

#[derive(Debug)]
struct CallableBatchSourceContinuationSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) enum BrandedCallableBatchLoweringPlanV1<'a> {
    Acyclic {
        brand: super::module_invocation_identity::ModuleInvocationBrandV1,
        plan: VerifiedAcyclicCallableModulePlanV1<'a>,
    },
    Recursive {
        brand: super::module_invocation_identity::ModuleInvocationBrandV1,
        plan: VerifiedRecursiveCallableModulePlanV1<'a>,
    },
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedCallableBatchSourceV1<'a> {
    token: ModuleInvocationTokenV1,
    plan: Option<BrandedCallableBatchLoweringPlanV1<'a>>,
    continuation: CallableBatchSourceContinuationV1<'a>,
    _seal: PreparedCallableBatchSourceSealV1,
}

#[derive(Debug)]
struct PreparedCallableBatchSourceSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) enum CallableBatchCapabilityDispositionV1 {
    Acyclic(AcyclicCapabilityAbsenceWitnessV1),
    Recursive(RecursiveCapabilityInstallReceiptV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableBatchRootWitnessV1<'a> {
    source: CallableBatchSourceContinuationV1<'a>,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
    capability: CallableBatchCapabilityDispositionV1,
    _seal: CallableBatchRootWitnessSealV1,
}

#[derive(Debug)]
struct CallableBatchRootWitnessSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CallableBatchDrainPlanV1<'a> {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    source: &'a VerifiedResolvedCallableModuleV1,
    _seal: CallableBatchDrainPlanSealV1,
}

#[derive(Debug)]
struct CallableBatchDrainPlanSealV1;

pub(in crate::mir::builder) struct CallableBatchCompleteInvocationV1<'a> {
    brand: super::module_invocation_identity::ModuleInvocationBrandV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    root: CallableBatchRootWitnessV1<'a>,
    drain_plan: CallableBatchDrainPlanV1<'a>,
    _seal: CallableBatchCompleteInvocationSealV1,
}

#[derive(Debug)]
struct CallableBatchCompleteInvocationSealV1;

impl<'a> PreparedCallableBatchSourceV1<'a> {
    pub(in crate::mir::builder) fn prepare_acyclic(
        token: ModuleInvocationTokenV1,
        plan: VerifiedAcyclicCallableModulePlanV1<'a>,
    ) -> Result<Self, CanonicalSourceBindingErrorV1> {
        let brand = token.brand();
        Self::prepare_callable(
            token,
            ModuleInvocationFamilyV1::BindingSsaAcyclic,
            plan.module(),
            BrandedCallableBatchLoweringPlanV1::Acyclic { brand, plan },
        )
    }

    pub(in crate::mir::builder) fn prepare_recursive(
        token: ModuleInvocationTokenV1,
        plan: VerifiedRecursiveCallableModulePlanV1<'a>,
    ) -> Result<Self, CanonicalSourceBindingErrorV1> {
        let brand = token.brand();
        Self::prepare_callable(
            token,
            ModuleInvocationFamilyV1::BindingSsaRecursive,
            plan.module(),
            BrandedCallableBatchLoweringPlanV1::Recursive { brand, plan },
        )
    }

    fn prepare_callable(
        token: ModuleInvocationTokenV1,
        family: ModuleInvocationFamilyV1,
        source: &'a VerifiedResolvedCallableModuleV1,
        plan: BrandedCallableBatchLoweringPlanV1<'a>,
    ) -> Result<Self, CanonicalSourceBindingErrorV1> {
        if token.family() != family {
            return Err(CanonicalSourceBindingErrorV1::FamilyMismatch {
                expected: family,
                actual: token.family(),
            });
        }
        let policy = RouteOwnedInvocationInventoryV2::derive(family)
            .map_err(|_| CanonicalSourceBindingErrorV1::RoutePolicy)?;
        let brand = token.brand();
        Ok(Self {
            token,
            plan: Some(plan),
            continuation: CallableBatchSourceContinuationV1 {
                brand,
                family,
                source,
                policy,
                _seal: CallableBatchSourceContinuationSealV1,
            },
            _seal: PreparedCallableBatchSourceSealV1,
        })
    }

    pub(in crate::mir::builder) fn split(
        mut self,
    ) -> (
        ModuleInvocationTokenV1,
        BrandedCallableBatchLoweringPlanV1<'a>,
        CallableBatchSourceContinuationV1<'a>,
    ) {
        (
            self.token,
            self.plan
                .take()
                .expect("callable source package split once"),
            self.continuation,
        )
    }
}

pub(in crate::mir::builder) struct CanonicalSingleActiveInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCanonicalSingleLoweringPlanV1<'a>>,
    source: CanonicalSingleSourceContinuationV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
}

pub(in crate::mir::builder) struct RejectedCanonicalSingleCollectionV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCanonicalSingleLoweringPlanV1<'a>>,
    source: CanonicalSingleSourceContinuationV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    rejected: RejectedCollectedDraftAdmissionV1,
}
pub(in crate::mir::builder) struct CanonicalSingleCollectedInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCanonicalSingleLoweringPlanV1<'a>>,
    source: CanonicalSingleSourceContinuationV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collected: CollectedDraftAdmissionProductV1,
}
impl<'a> CanonicalSingleActiveInvocationV1<'a> {
    pub(in crate::mir::builder) fn open(
        prepared: PreparedCanonicalSingleSourceV1<'a>,
        current: &super::MirBuilder,
        config: BuilderInvocationConfigV1,
        module_name: String,
    ) -> Result<Self, CanonicalCompletionErrorV1> {
        let (token, lowering, source) = prepared.split();
        let active = ActiveModuleInvocationV1::open(token, current, config, module_name)
            .map_err(CanonicalCompletionErrorV1::Shell)?;
        let (token, session, physical) = active.into_parts();
        let (_brand, shell, collector) = physical.into_parts();
        Ok(Self {
            token,
            lowering: Some(lowering),
            source,
            session,
            shell,
            collector,
        })
    }

    pub(in crate::mir::builder) fn collect(
        self,
        draft: MirFunction,
    ) -> Result<CanonicalSingleCollectedInvocationV1<'a>, RejectedCanonicalSingleCollectionV1<'a>>
    {
        let Self {
            token,
            lowering,
            source,
            session,
            shell,
            collector,
        } = self;
        let key = FunctionDraftKeyV1::CanonicalResolvedOwner(source.header.owner());
        let symbol = source.header.symbol().as_mir_name().to_owned();
        match collector.collect_canonical_single(key, symbol, source.header.arity(), draft) {
            Ok(collected) => Ok(CanonicalSingleCollectedInvocationV1 {
                token,
                lowering,
                source,
                session,
                shell,
                collected,
            }),
            Err(rejected) => Err(RejectedCanonicalSingleCollectionV1 {
                token,
                lowering,
                source,
                session,
                shell,
                rejected,
            }),
        }
    }
}

impl<'a> CanonicalSingleCollectedInvocationV1<'a> {
    pub(in crate::mir::builder) fn complete(
        self,
    ) -> Result<CanonicalSingleCompleteInvocationV1, CanonicalCompletionErrorV1> {
        let Self {
            token,
            mut lowering,
            source,
            session,
            shell,
            collected,
        } = self;
        let (collector, receipt) = collected.into_parts();
        let brand = token.brand();
        if receipt.brand() != brand {
            return Err(CanonicalCompletionErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: receipt.brand().ordinal(),
            });
        }
        let physical = receipt.payload();
        let symbol = source.header.symbol().as_mir_name().to_owned();
        let expected_key = FunctionDraftKeyV1::CanonicalResolvedOwner(source.header.owner());
        if matches!(
            physical.key(),
            FunctionDraftKeyV1::Main | FunctionDraftKeyV1::SyntheticConditionFn
        ) {
            return Err(CanonicalCompletionErrorV1::SyntheticRoot(
                physical.key().clone(),
            ));
        }
        if physical.collector_brand() != Some(brand) || physical.key() != &expected_key {
            return Err(CanonicalCompletionErrorV1::KeyMismatch);
        }
        if physical.symbol() != symbol {
            return Err(CanonicalCompletionErrorV1::SymbolMismatch {
                expected: symbol.clone(),
                actual: physical.symbol().to_owned(),
            });
        }
        if physical.arity() != source.header.arity() {
            return Err(CanonicalCompletionErrorV1::ArityMismatch {
                expected: source.header.arity(),
                actual: physical.arity(),
            });
        }
        if physical.policy() != DraftPublicationPolicyV1::CanonicalRejectDuplicate {
            return Err(CanonicalCompletionErrorV1::PolicyMismatch);
        }
        if !matches!(
            physical.replacement(),
            CollectedDraftReplacementDispositionV1::Inserted
        ) || collector.payload().symbol_count() != 1
            || collector.payload().key_for_symbol(&symbol) != Some(&expected_key)
        {
            return Err(CanonicalCompletionErrorV1::ReplacementForbidden);
        }
        let _lowering = lowering
            .take()
            .expect("canonical lowering plan consumed once");
        Ok(CanonicalSingleCompleteInvocationV1 {
            brand,
            session,
            shell,
            collector,
            drain_plan: CanonicalSingleDrainPlanV1 {
                brand,
                family: source.policy.policy().family(),
                key: expected_key,
                symbol: symbol.into(),
                arity: source.header.arity(),
                _seal: CanonicalSingleDrainPlanSealV1,
            },
            root: CanonicalSingleRootWitnessV1 {
                source,
                receipt,
                _seal: CanonicalSingleRootWitnessSealV1,
            },
            _seal: CanonicalSingleCompleteInvocationSealV1,
        })
    }
}

pub(in crate::mir::builder) struct CallableBatchActiveInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCallableBatchLoweringPlanV1<'a>>,
    source: CallableBatchSourceContinuationV1<'a>,
    capability: CallableBatchCapabilityDispositionV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
}

pub(in crate::mir::builder) struct RejectedCallableBatchCollectionV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCallableBatchLoweringPlanV1<'a>>,
    source: CallableBatchSourceContinuationV1<'a>,
    capability: CallableBatchCapabilityDispositionV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    error: CallableBatchCollectionErrorV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableBatchCollectionErrorV1 {
    Prepare(CallableCollectorBatchPrepareErrorV1),
    CollectorUnbranded,
}

pub(in crate::mir::builder) struct CallableBatchCollectedInvocationV1<'a> {
    token: ModuleInvocationTokenV1,
    lowering: Option<BrandedCallableBatchLoweringPlanV1<'a>>,
    source: CallableBatchSourceContinuationV1<'a>,
    capability: CallableBatchCapabilityDispositionV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collected: CollectedCallableCollectorBatchV1,
}
impl<'a> CallableBatchActiveInvocationV1<'a> {
    pub(in crate::mir::builder) fn open(
        prepared: PreparedCallableBatchSourceV1<'a>,
        current: &super::MirBuilder,
        config: BuilderInvocationConfigV1,
        module_name: String,
    ) -> Result<Self, CanonicalCompletionErrorV1> {
        let (token, lowering, source) = prepared.split();
        let family = source.family;
        let active = ActiveModuleInvocationV1::open(token, current, config, module_name)
            .map_err(CanonicalCompletionErrorV1::Shell)?;
        let (token, session, physical) = active.into_parts();
        let (_brand, mut shell, collector) = physical.into_parts();
        let brand = token.brand();
        let installed = shell
            .install_callable_batch_capability(family)
            .map_err(CanonicalCompletionErrorV1::RecursiveCapability)?;
        let capability = match (family, installed) {
            (ModuleInvocationFamilyV1::BindingSsaAcyclic, Err(absence)) => {
                if absence.family() != family {
                    return Err(
                        CanonicalCompletionErrorV1::CapabilityWitnessFamilyMismatch {
                            expected: family,
                            actual: absence.family(),
                        },
                    );
                }
                if absence.brand() != brand {
                    return Err(CanonicalCompletionErrorV1::CapabilityBrandMismatch {
                        expected: brand.ordinal(),
                        actual: absence.brand().ordinal(),
                    });
                }
                CallableBatchCapabilityDispositionV1::Acyclic(absence)
            }
            (ModuleInvocationFamilyV1::BindingSsaRecursive, Ok(receipt)) => {
                if receipt.family() != family {
                    return Err(
                        CanonicalCompletionErrorV1::CapabilityWitnessFamilyMismatch {
                            expected: family,
                            actual: receipt.family(),
                        },
                    );
                }
                if receipt.brand() != brand {
                    return Err(CanonicalCompletionErrorV1::CapabilityBrandMismatch {
                        expected: brand.ordinal(),
                        actual: receipt.brand().ordinal(),
                    });
                }
                CallableBatchCapabilityDispositionV1::Recursive(receipt)
            }
            _ => return Err(CanonicalCompletionErrorV1::CapabilityFamilyMismatch(family)),
        };
        Ok(Self {
            token,
            lowering: Some(lowering),
            source,
            capability,
            session,
            shell,
            collector,
        })
    }

    pub(in crate::mir::builder) fn collect(
        self,
        entries: Vec<CallableCollectorDraftEntryV1>,
    ) -> Result<CallableBatchCollectedInvocationV1<'a>, RejectedCallableBatchCollectionV1<'a>> {
        let Self {
            token,
            lowering,
            source,
            capability,
            session,
            shell,
            collector,
        } = self;
        let brand = token.brand();
        if collector.payload().receipt_brand() != Some(brand) {
            return Err(RejectedCallableBatchCollectionV1 {
                token,
                lowering,
                source,
                capability,
                session,
                shell,
                collector,
                error: CallableBatchCollectionErrorV1::CollectorUnbranded,
            });
        }
        let raw_collector = collector.into_payload();
        let prepared = match raw_collector.prepare_callable_batch(entries) {
            Ok(prepared) => prepared,
            Err(rejected) => {
                let (collector, error) = rejected.into_parts();
                return Err(RejectedCallableBatchCollectionV1 {
                    token,
                    lowering,
                    source,
                    capability,
                    session,
                    shell,
                    collector: InvocationBranded::from_source(brand, collector),
                    error: CallableBatchCollectionErrorV1::Prepare(error),
                });
            }
        };
        let collected = prepared
            .collect_all_branded()
            .expect("collector brand was preflighted before batch collection");
        Ok(CallableBatchCollectedInvocationV1 {
            token,
            lowering,
            source,
            capability,
            session,
            shell,
            collected,
        })
    }
}
impl<'a> CallableBatchCollectedInvocationV1<'a> {
    pub(in crate::mir::builder) fn complete(
        self,
    ) -> Result<CallableBatchCompleteInvocationV1<'a>, CanonicalCompletionErrorV1> {
        let Self {
            token,
            mut lowering,
            source,
            capability,
            session,
            shell,
            collected,
        } = self;
        let (collector, receipt) = collected.into_parts();
        let brand = token.brand();
        if receipt.brand() != brand {
            return Err(CanonicalCompletionErrorV1::ForeignBrand {
                expected: brand.ordinal(),
                actual: receipt.brand().ordinal(),
            });
        }
        let expected = source.source.functions_by_key().len();
        if receipt.payload().len() != expected || collector.payload().symbol_count() != expected {
            return Err(CanonicalCompletionErrorV1::CollectorCardinality {
                expected,
                actual: receipt.payload().len(),
            });
        }
        for admission in receipt.payload().admissions() {
            if admission.collector_brand() != Some(brand) {
                return Err(CanonicalCompletionErrorV1::ForeignBrand {
                    expected: brand.ordinal(),
                    actual: admission
                        .collector_brand()
                        .map_or(0, |value| value.ordinal()),
                });
            }
            if admission.policy() != DraftPublicationPolicyV1::CanonicalRejectDuplicate {
                return Err(CanonicalCompletionErrorV1::PolicyMismatch);
            }
            if !matches!(
                admission.replacement(),
                CollectedDraftReplacementDispositionV1::Inserted
            ) {
                return Err(CanonicalCompletionErrorV1::ReplacementForbidden);
            }
        }
        for (key, _) in source.source.functions_by_key() {
            let header = source
                .source
                .source()
                .catalog()
                .index()
                .lookup(key)
                .ok_or(CanonicalCompletionErrorV1::MissingReceipt)?;
            let symbol = header.symbol().as_mir_name();
            let expected_key = FunctionDraftKeyV1::CanonicalCallable(key.clone());
            if collector.payload().key_for_symbol(symbol) != Some(&expected_key) {
                return Err(CanonicalCompletionErrorV1::KeyMismatch);
            }
            let admission = receipt
                .payload()
                .admissions()
                .iter()
                .find(|entry| entry.symbol() == symbol)
                .ok_or(CanonicalCompletionErrorV1::MissingReceipt)?;
            if admission.key() != &expected_key || admission.arity() != header.signature().arity() {
                return Err(CanonicalCompletionErrorV1::KeyMismatch);
            }
        }
        let _lowering = lowering
            .take()
            .expect("callable lowering plan consumed once");
        let source_ref = source.source;
        let family = source.family;
        Ok(CallableBatchCompleteInvocationV1 {
            brand,
            session,
            shell,
            collector,
            root: CallableBatchRootWitnessV1 {
                source,
                receipt,
                capability,
                _seal: CallableBatchRootWitnessSealV1,
            },
            drain_plan: CallableBatchDrainPlanV1 {
                brand,
                family,
                source: source_ref,
                _seal: CallableBatchDrainPlanSealV1,
            },
            _seal: CallableBatchCompleteInvocationSealV1,
        })
    }
}
