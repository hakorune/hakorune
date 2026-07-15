//! MP0-TX0 unpublished callable draft set and atomic candidate publication.
//!
//! Every function is lowered and verified before this box mutates the
//! candidate module. Source lookup, sibling-call admission, and backend
//! activation remain outside this transaction.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalTrivialBindingSsaPlanV1,
};
use crate::mir::compiler::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use crate::mir::compiler::resolved_callable_module_preflight::VerifiedCallableModulePreflightV1;
use crate::mir::function::{FunctionPublicationErrorV1, MirFunction, MirModule};
use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::{CanonicalResolvedBuildErrorV1, MirBuilder};

#[derive(Debug)]
pub(in crate::mir) enum CallableModuleTransactionErrorV1 {
    UnsupportedPlan(CanonicalCallableKeyV1),
    FunctionDraft {
        key: CanonicalCallableKeyV1,
        source: CanonicalResolvedBuildErrorV1,
    },
    MissingHeader(CanonicalCallableKeyV1),
    SymbolMismatch {
        key: CanonicalCallableKeyV1,
        expected: String,
        actual: String,
    },
    SignatureArityMismatch {
        key: CanonicalCallableKeyV1,
        expected: usize,
        actual: usize,
    },
    DuplicateDraftKey(CanonicalCallableKeyV1),
    DuplicateDraftSymbol(String),
    CardinalityMismatch {
        catalog: usize,
        functions: usize,
        plans: usize,
        drafts: usize,
    },
    Publication(FunctionPublicationErrorV1),
    BuilderContract(String),
}

/// Complete, individually verified drafts which are still absent from MIR.
pub(super) struct VerifiedUnpublishedCallableDraftSetV1<'a> {
    source: &'a VerifiedResolvedCallableModuleV1,
    drafts_by_key: BTreeMap<CanonicalCallableKeyV1, MirFunction>,
}

impl<'a> VerifiedUnpublishedCallableDraftSetV1<'a> {
    pub(super) fn collect_with(
        preflight: VerifiedCallableModulePreflightV1<'a>,
        mut lower: impl FnMut(
            &CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'a>,
        ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>,
    ) -> Result<Self, CallableModuleTransactionErrorV1> {
        let (source, plans) = preflight.into_parts();
        let plan_count = plans.len();
        let mut drafts_by_key = BTreeMap::new();
        let mut symbols = BTreeSet::new();

        for (key, plan) in plans {
            let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
                return Err(CallableModuleTransactionErrorV1::UnsupportedPlan(key));
            };
            let draft = lower(&key, plan).map_err(|source| {
                CallableModuleTransactionErrorV1::FunctionDraft {
                    key: key.clone(),
                    source,
                }
            })?;
            let header = source
                .source()
                .catalog()
                .index()
                .lookup(&key)
                .ok_or_else(|| CallableModuleTransactionErrorV1::MissingHeader(key.clone()))?;
            let expected_symbol = header.symbol().as_mir_name();
            if draft.signature.name != expected_symbol {
                return Err(CallableModuleTransactionErrorV1::SymbolMismatch {
                    key,
                    expected: expected_symbol.to_string(),
                    actual: draft.signature.name,
                });
            }
            let expected_arity = header.signature().arity();
            if draft.signature.params.len() != expected_arity {
                return Err(CallableModuleTransactionErrorV1::SignatureArityMismatch {
                    key,
                    expected: expected_arity,
                    actual: draft.signature.params.len(),
                });
            }
            if !symbols.insert(draft.signature.name.clone()) {
                return Err(CallableModuleTransactionErrorV1::DuplicateDraftSymbol(
                    draft.signature.name,
                ));
            }
            if drafts_by_key.insert(key.clone(), draft).is_some() {
                return Err(CallableModuleTransactionErrorV1::DuplicateDraftKey(key));
            }
        }

        let catalog = source.source().catalog().len();
        let functions = source.functions_by_key().len();
        let drafts = drafts_by_key.len();
        if catalog != functions || functions != plan_count || plan_count != drafts {
            return Err(CallableModuleTransactionErrorV1::CardinalityMismatch {
                catalog,
                functions,
                plans: plan_count,
                drafts,
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
            .map_err(CallableModuleTransactionErrorV1::Publication)
    }
}

impl MirBuilder {
    pub(in crate::mir) fn build_resolved_callable_module_candidate(
        &mut self,
        preflight: VerifiedCallableModulePreflightV1<'_>,
    ) -> Result<MirModule, CallableModuleTransactionErrorV1> {
        self.build_resolved_callable_module_candidate_with(preflight, |builder, _key, plan| {
            builder.lower_resolved_trivial_function_draft(plan)
        })
    }

    pub(super) fn build_resolved_callable_module_candidate_with<'a>(
        &mut self,
        preflight: VerifiedCallableModulePreflightV1<'a>,
        mut lower: impl FnMut(
            &mut MirBuilder,
            &CanonicalCallableKeyV1,
            CanonicalTrivialBindingSsaPlanV1<'a>,
        ) -> Result<MirFunction, CanonicalResolvedBuildErrorV1>,
    ) -> Result<MirModule, CallableModuleTransactionErrorV1> {
        self.prepare_module()
            .map_err(CallableModuleTransactionErrorV1::BuilderContract)?;
        let drafts =
            VerifiedUnpublishedCallableDraftSetV1::collect_with(preflight, |key, plan| {
                lower(self, key, plan)
            })?;
        let module = self.current_module.as_mut().ok_or_else(|| {
            CallableModuleTransactionErrorV1::BuilderContract(
                "[freeze:contract][callable_module_transaction/module_missing]".to_string(),
            )
        })?;
        drafts.publish_into(module)?;

        let entry = crate::mir::builder::emission::constant::emit_void(self)
            .map_err(CallableModuleTransactionErrorV1::BuilderContract)?;
        self.finalize_module(entry)
            .map_err(CallableModuleTransactionErrorV1::BuilderContract)
    }
}
