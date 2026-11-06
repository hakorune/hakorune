//! MP0-TX0 unpublished callable draft set and atomic candidate publication.
//!
//! Every function is lowered and verified before this box mutates the
//! candidate module. Source lookup, sibling-call admission, and backend
//! activation remain outside this transaction.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::compiler::acyclic_callable_module_plan::VerifiedAcyclicCallableModulePlanV1;
use crate::mir::compiler::capability::CanonicalTrivialBindingSsaPlanV1;
use crate::mir::compiler::recursive_callable_module_plan::VerifiedRecursiveCallableModulePlanV1;
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::function::{FunctionPublicationErrorV1, MirFunction, MirModule};
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::{CanonicalResolvedBuildErrorV1, MirBuilder};
use crate::mir::builder::module_draft_collector::CallableCollectorDraftEntryV1;

#[derive(Debug)]
pub(in crate::mir) enum CallableModuleTransactionErrorV1 {
    FunctionDraft {
        key: CanonicalCallableKeyV1,
        _source: CanonicalResolvedBuildErrorV1,
    },
    MissingHeader {
        _key: CanonicalCallableKeyV1,
    },
    SymbolMismatch {
        _key: CanonicalCallableKeyV1,
        _expected: String,
        _actual: String,
    },
    SignatureArityMismatch {
        _key: CanonicalCallableKeyV1,
        _expected: usize,
        _actual: usize,
    },
    DuplicateDraftKey {
        _key: CanonicalCallableKeyV1,
    },
    DuplicateDraftSymbol {
        _symbol: String,
    },
    CardinalityMismatch {
        _catalog: usize,
        _functions: usize,
        _plans: usize,
        _drafts: usize,
    },
    Publication {
        _error: FunctionPublicationErrorV1,
    },
    BuilderContract {
        _detail: String,
    },
}

/// Complete, individually verified drafts which are still absent from MIR.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedUnpublishedCallableDraftSetV1<'a> {
    source: &'a VerifiedResolvedCallableModuleV1,
    drafts_by_key: BTreeMap<CanonicalCallableKeyV1, MirFunction>,
}

impl<'a> VerifiedUnpublishedCallableDraftSetV1<'a> {
    /// Project the already verified source/catalog into canonical collector
    /// entries.  No caller supplies key, symbol, arity, or publication policy.
    pub(in crate::mir) fn into_canonical_entries(self) -> Vec<CallableCollectorDraftEntryV1> {
        let source = self.source;
        self.drafts_by_key
            .into_iter()
            .map(|(key, draft)| {
                let header = source
                    .source()
                    .catalog()
                    .index()
                    .lookup(&key)
                    .expect("verified callable draft set has an exact catalog header");
                CallableCollectorDraftEntryV1::new(
                    crate::mir::builder::module_draft_collector::FunctionDraftKeyV1::CanonicalCallable(
                        key,
                    ),
                    header.symbol().as_mir_name().to_owned(),
                    header.signature().arity(),
                    draft,
                )
            })
            .collect()
    }

    fn collect_acyclic_with(
        plan: VerifiedAcyclicCallableModulePlanV1<'a>,
        lower: impl FnMut(
            &CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'a>,
        ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>,
    ) -> Result<Self, CallableModuleTransactionErrorV1> {
        let (source, _graph, plans) = plan.into_parts();
        Self::collect_typed_with(source, plans, lower)
    }

    fn collect_recursive_with(
        plan: VerifiedRecursiveCallableModulePlanV1<'a>,
        lower: impl FnMut(
            &CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'a>,
        ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>,
    ) -> Result<Self, CallableModuleTransactionErrorV1> {
        let (source, _partition, plans) = plan.into_parts();
        Self::collect_typed_with(source, plans, lower)
    }

    fn collect_typed_with(
        source: &'a VerifiedResolvedCallableModuleV1,
        plans: BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'a>>,
        mut lower: impl FnMut(
            &CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'a>,
        ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>,
    ) -> Result<Self, CallableModuleTransactionErrorV1> {
        let plan_count = plans.len();
        let mut drafts_by_key = BTreeMap::new();
        let mut symbols = BTreeSet::new();

        for (key, plan) in plans {
            let draft = lower(&key, plan).map_err(|source| {
                CallableModuleTransactionErrorV1::FunctionDraft {
                    key: key.clone(),
                    _source: source,
                }
            })?;
            let header = source
                .source()
                .catalog()
                .index()
                .lookup(&key)
                .ok_or_else(|| CallableModuleTransactionErrorV1::MissingHeader {
                    _key: key.clone(),
                })?;
            let expected_symbol = header.symbol().as_mir_name();
            if draft.signature.name != expected_symbol {
                return Err(CallableModuleTransactionErrorV1::SymbolMismatch {
                    _key: key,
                    _expected: expected_symbol.to_string(),
                    _actual: draft.signature.name,
                });
            }
            let expected_arity = header.signature().arity();
            if draft.signature.params.len() != expected_arity {
                return Err(CallableModuleTransactionErrorV1::SignatureArityMismatch {
                    _key: key,
                    _expected: expected_arity,
                    _actual: draft.signature.params.len(),
                });
            }
            if !symbols.insert(draft.signature.name.clone()) {
                return Err(CallableModuleTransactionErrorV1::DuplicateDraftSymbol {
                    _symbol: draft.signature.name,
                });
            }
            if drafts_by_key.insert(key.clone(), draft).is_some() {
                return Err(CallableModuleTransactionErrorV1::DuplicateDraftKey { _key: key });
            }
        }

        let catalog = source.source().catalog().len();
        let functions = source.functions_by_key().len();
        let drafts = drafts_by_key.len();
        if catalog != functions || functions != plan_count || plan_count != drafts {
            return Err(CallableModuleTransactionErrorV1::CardinalityMismatch {
                _catalog: catalog,
                _functions: functions,
                _plans: plan_count,
                _drafts: drafts,
            });
        }
        Ok(Self {
            source,
            drafts_by_key,
        })
    }

    pub(super) fn publish_into(
        self,
        module: &mut MirModule,
    ) -> Result<(), CallableModuleTransactionErrorV1> {
        let _source = self.source;
        module
            .try_add_functions_atomic(self.drafts_by_key.into_values().collect())
            .map_err(|_error| CallableModuleTransactionErrorV1::Publication { _error })
    }
}

impl MirBuilder {
    /// LOWER0 draft-only callable consumer.  It lowers every verified plan
    /// but does not prepare a module, install a recursive marker, or publish.
    pub(in crate::mir) fn lower_acyclic_callable_drafts<'a>(
        &mut self,
        plan: VerifiedAcyclicCallableModulePlanV1<'a>,
    ) -> Result<VerifiedUnpublishedCallableDraftSetV1<'a>, CallableModuleTransactionErrorV1> {
        VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with(plan, |_key, plan| {
            self.lower_resolved_trivial_function_draft(plan)
        })
    }

    /// LOWER0 draft-only recursive consumer.  Capability installation stays
    /// with the later RECURSIVE0 row.
    pub(in crate::mir) fn lower_recursive_callable_drafts<'a>(
        &mut self,
        plan: VerifiedRecursiveCallableModulePlanV1<'a>,
    ) -> Result<VerifiedUnpublishedCallableDraftSetV1<'a>, CallableModuleTransactionErrorV1> {
        VerifiedUnpublishedCallableDraftSetV1::collect_recursive_with(plan, |_key, plan| {
            self.lower_resolved_trivial_function_draft(plan)
        })
    }

    pub(in crate::mir) fn build_recursive_callable_module_candidate(
        &mut self,
        plan: VerifiedRecursiveCallableModulePlanV1<'_>,
    ) -> Result<MirModule, CallableModuleTransactionErrorV1> {
        self.prepare_module()
            .map_err(|_detail| CallableModuleTransactionErrorV1::BuilderContract { _detail })?;
        let drafts =
            VerifiedUnpublishedCallableDraftSetV1::collect_recursive_with(plan, |_key, plan| {
                self.lower_resolved_trivial_function_draft(plan)
            })?;
        self.publish_recursive_callable_drafts(drafts)
    }

    fn publish_recursive_callable_drafts(
        &mut self,
        drafts: VerifiedUnpublishedCallableDraftSetV1<'_>,
    ) -> Result<MirModule, CallableModuleTransactionErrorV1> {
        let module = self.current_module.as_mut().ok_or_else(|| {
            CallableModuleTransactionErrorV1::BuilderContract {
                _detail: "[freeze:contract][callable_module_transaction/module_missing]"
                    .to_string(),
            }
        })?;
        if module
            .metadata
            .canonical_recursive_callable_module_capability
            .is_some()
        {
            return Err(CallableModuleTransactionErrorV1::BuilderContract {
                _detail: "[freeze:contract][canonical_recursive_module/capability_preexisting]"
                    .to_string(),
            });
        }
        drafts.publish_into(module)?;
        crate::mir::canonical_recursive_callable_module_capability::CanonicalRecursiveCallableModuleCapabilityV1::install_for_module(
            &mut module.metadata.canonical_recursive_callable_module_capability,
            true,
        )
        .map_err(|error| CallableModuleTransactionErrorV1::BuilderContract {
            _detail: error.to_string(),
        })?;

        let entry = crate::mir::builder::emission::constant::emit_void(self)
            .map_err(|_detail| CallableModuleTransactionErrorV1::BuilderContract { _detail })?;
        self.finalize_module(entry)
            .map_err(|_detail| CallableModuleTransactionErrorV1::BuilderContract { _detail })
    }


}

#[cfg(test)]
#[path = "callable_module_transaction_p0d_tests.rs"]
mod p0d_tests;
