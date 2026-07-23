//! CUT0-I0-POST0-RAW-S0: retain the Raw physical owner for finalization.
//!
//! This is disconnected from all public ingress.  It closes the missing
//! bridge between Raw root evidence and a candidate module/session without
//! adapting through the legacy Main-only drained candidate.

use super::module_draft_collector::{CompletedDraftSignatureViewV1, ModuleDraftCollectorV1};
use super::module_invocation_identity::{
    ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1};
use super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::module_lowering_shell::{ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellV1};
use super::raw_expansion_receipt_ledger::SealedRawExpansionReceiptLedgerV1;
use super::raw_root_completion::{RawCompleteInvocationV1, RawInvocationRootWitnessV1};
use crate::mir::MirModule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPhysicalBindingErrorV1 {
    NonRawFamily,
    ForeignBrand,
    PublishedShell { count: usize },
}

#[derive(Debug)]
pub(in crate::mir) struct RawPhysicalCompleteInvocationV1 {
    token: ModuleInvocationTokenV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    ledger: SealedRawExpansionReceiptLedgerV1,
    root: RawInvocationRootWitnessV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPhysicalBindingV1 {
    owner: RawPhysicalCompleteInvocationV1,
    error: RawPhysicalBindingErrorV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPhysicalFinalizationErrorV1 {
    ForeignBrand,
    PublishedShell { count: usize },
    InventoryMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizationInputV1 {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) module: MirModule,
    pub(in crate::mir) ledger: SealedRawExpansionReceiptLedgerV1,
    pub(in crate::mir) root: RawInvocationRootWitnessV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawFinalizationV1 {
    pub(in crate::mir) owner: RawPhysicalCompleteInvocationV1,
    pub(in crate::mir) error: RawPhysicalFinalizationErrorV1,
}

impl RawCompleteInvocationV1 {
    pub(in crate::mir) fn bind_physical(
        self,
        token: ModuleInvocationTokenV1,
        session: ModuleBuilderInvocationSessionV1,
        shell: BrandedShellV1<ModuleLoweringShellV1>,
    ) -> Result<RawPhysicalCompleteInvocationV1, RejectedRawPhysicalBindingV1> {
        let (brand, collector, ledger, root) = self.into_parts();
        let expected = token.brand();
        if token.family() != ModuleInvocationFamilyV1::Raw {
            return Err(RejectedRawPhysicalBindingV1 {
                owner: RawPhysicalCompleteInvocationV1 {
                    token,
                    session,
                    shell,
                    collector,
                    ledger,
                    root,
                },
                error: RawPhysicalBindingErrorV1::NonRawFamily,
            });
        }
        if brand != expected || session.brand() != expected || shell.brand() != expected {
            return Err(RejectedRawPhysicalBindingV1 {
                owner: RawPhysicalCompleteInvocationV1 {
                    token,
                    session,
                    shell,
                    collector,
                    ledger,
                    root,
                },
                error: RawPhysicalBindingErrorV1::ForeignBrand,
            });
        }
        Ok(RawPhysicalCompleteInvocationV1 {
            token,
            session,
            shell,
            collector,
            ledger,
            root,
        })
    }
}

impl RawPhysicalCompleteInvocationV1 {
    pub(in crate::mir) fn prepare_finalization(
        self,
    ) -> Result<RawFinalizationInputV1, RejectedRawFinalizationV1> {
        let expected = self.token.brand();
        if self.shell.brand() != expected || self.collector.brand() != expected {
            return Err(RejectedRawFinalizationV1 {
                owner: self,
                error: RawPhysicalFinalizationErrorV1::ForeignBrand,
            });
        }
        if self.shell.payload().has_published_functions() {
            let count = self.shell.payload().published_function_count();
            return Err(RejectedRawFinalizationV1 {
                owner: self,
                error: RawPhysicalFinalizationErrorV1::PublishedShell { count },
            });
        }
        let mut symbols = Vec::new();
        self.collector
            .payload()
            .visit_symbols(&mut |symbol| symbols.push(symbol.to_owned()));
        symbols.sort();
        if symbols != ["condition_fn".to_owned(), "main".to_owned()] {
            return Err(RejectedRawFinalizationV1 {
                owner: self,
                error: RawPhysicalFinalizationErrorV1::InventoryMismatch,
            });
        }
        let inventory = match ModuleLoweringShellDrainInventoryV1::from_symbols(symbols) {
            Ok(inventory) => inventory,
            Err(_) => {
                return Err(RejectedRawFinalizationV1 {
                    owner: self,
                    error: RawPhysicalFinalizationErrorV1::InventoryMismatch,
                })
            }
        };
        let RawPhysicalCompleteInvocationV1 {
            token,
            session,
            shell,
            collector,
            ledger,
            root,
        } = self;
        let functions = collector.into_payload().into_draft_functions();
        let module = shell
            .into_payload()
            .prepare_drain(inventory)
            .commit_preflighted(functions);
        Ok(RawFinalizationInputV1 {
            token,
            session,
            module,
            ledger,
            root,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::main_pending_draft::{
        MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    };
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_invocation_identity::TestInvocationPreflightFactoryV1;
    use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
    use crate::mir::builder::module_invocation_session::{
        BuilderCoreSeedPolicyV1, BuilderInvocationConfigV1,
    };
    use crate::mir::builder::module_lowering_shell::ModuleLoweringShellV1;
    use crate::mir::builder::raw_expansion_receipt_ledger::{
        RawCallableMainCompatibilityDispositionV1, RawExpansionDraftRequestV1,
        RawExpansionReceiptLedgerV1,
    };
    use crate::mir::builder::raw_root_completion::complete_raw_root;
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::builder::root_draft_batch::PreparedRootDraftBatchV1;
    use crate::mir::builder::MirBuilder;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType,
    };
    use crate::mir::compiler::module_postprocess::ModulePostprocessOwnerV1;
    use crate::mir::compiler::MirCompiler;
    use crate::mir::verification::MirVerifier;

    fn draft(symbol: &str, arity: usize) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: vec![MirType::Integer; arity],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn raw_complete() -> (ModuleInvocationTokenV1, RawCompleteInvocationV1) {
        let mut factory = TestInvocationPreflightFactoryV1::new();
        let token = factory.mint(ModuleInvocationFamilyV1::Raw).unwrap();
        let brand = token.brand();
        let root_body = RootBodyCompletionTrackerV1::new_for_brand(brand)
            .complete(RootBodyResultV1::NoValue)
            .unwrap();
        let headers = MirModule::new("headers".into());
        let main = MainCompletionRequestV1::new(MainDraftIdentityV1::root(), root_body, false)
            .finish(
                draft("main", 0),
                MainHeaderLoanV1::new(&headers, MainHeaderSourceV1::InvocationCollector),
            )
            .unwrap();
        let batch = PreparedRootDraftBatchV1::prepare(
            main,
            Some(draft("condition_fn", 1)),
            super::super::module_invocation_drain::ConditionFnPolicyV1::Required,
        )
        .unwrap();
        let mut ledger = RawExpansionReceiptLedgerV1::new_for_token(
            &token,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        );
        let main_reservation = ledger
            .reserve(RawExpansionDraftRequestV1::root_main())
            .unwrap();
        let condition_reservation = ledger
            .reserve(RawExpansionDraftRequestV1::required_condition_fn())
            .unwrap();
        let complete = complete_raw_root(
            &token,
            InvocationBranded::from_test(brand, ModuleDraftCollectorV1::with_brand(brand)),
            ledger,
            batch,
            main_reservation,
            condition_reservation,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
        .unwrap();
        (token, complete)
    }

    fn session(token: &ModuleInvocationTokenV1) -> ModuleBuilderInvocationSessionV1 {
        let live = MirBuilder::new();
        let config =
            BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
        ModuleBuilderInvocationSessionV1::open_for_token(token, &live, config)
    }

    #[test]
    fn raw_physical_owner_retains_module_session_and_legacy_evidence() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let shell = InvocationBranded::from_test(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap(),
        );
        let active_session = session(&token);
        let input = complete
            .bind_physical(token, active_session, shell)
            .unwrap()
            .prepare_finalization()
            .unwrap();
        assert_eq!(input.token.family(), ModuleInvocationFamilyV1::Raw);
        assert_eq!(input.token.brand(), input.session.brand());
        assert_eq!(input.ledger.final_count(), 2);
        assert_eq!(input.root.brand(), brand);
        assert_eq!(input.module.functions.len(), 2);
        assert!(input
            .module
            .functions
            .keys()
            .any(|symbol| symbol.starts_with("main")));
        assert!(input
            .module
            .functions
            .keys()
            .any(|symbol| symbol.starts_with("condition_fn")));
    }

    #[test]
    fn raw_physical_prepare_rejects_published_shell_before_move() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let mut shell_payload =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap();
        shell_payload.publish_function_for_test(draft("already/0", 0));
        let shell = InvocationBranded::from_test(brand, shell_payload);
        let active_session = session(&token);
        let rejected = complete
            .bind_physical(token, active_session, shell)
            .unwrap()
            .prepare_finalization()
            .unwrap_err();
        assert!(matches!(
            rejected.error,
            RawPhysicalFinalizationErrorV1::PublishedShell { count: 1 }
        ));
    }

    #[test]
    fn raw_finalizer_consumes_physical_input_without_legacy_finalize() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let shell = InvocationBranded::from_test(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap(),
        );
        let active_session = session(&token);
        let input = complete
            .bind_physical(token, active_session, shell)
            .unwrap()
            .prepare_finalization()
            .unwrap();
        let prepared = crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::prepare(
            input,
        )
        .expect("raw finalization readiness must close the candidate session");
        assert_eq!(prepared.builder.brand(), brand);
        assert_eq!(prepared.token.brand(), brand);
        assert_eq!(prepared.module.functions.len(), 2);
        let finalized = crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::finalize(
            prepared,
        );
        assert_eq!(finalized.input.root.brand(), brand);
        let mut verifier = crate::mir::verification::MirVerifier::new();
        let postprocessed = crate::mir::compiler::module_postprocess::ModulePostprocessOwnerV1::new(
            &mut verifier,
            false,
        )
        .run_raw(finalized)
        .expect("Raw postprocess must retain reportable verifier evidence");
        assert_eq!(postprocessed.family(), ModuleInvocationFamilyV1::Raw);
        assert!(matches!(
            postprocessed.verification,
            crate::mir::compiler::module_postprocess::ModuleVerificationEvidenceV1::Raw { .. }
        ));
    }

    #[test]
    fn raw_finalizer_retains_readiness_failure_owner() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let shell = InvocationBranded::from_test(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap(),
        );
        let mut active_session = session(&token);
        active_session.builder_mut().current_module = Some(MirModule::new("open".into()));
        let input = complete
            .bind_physical(token, active_session, shell)
            .unwrap()
            .prepare_finalization()
            .unwrap();
        let rejected = crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::prepare(
            input,
        )
        .expect_err("open Builder state must reject before finalization");
        assert!(matches!(
            rejected.error,
            crate::mir::compiler::raw_finalization::RawFinalizationErrorV1::BuilderReadiness(
                crate::mir::builder::BuilderCommitReadinessErrorV1::CurrentModuleOpen
            )
        ));
        assert_eq!(rejected.owner.token.brand(), brand);
    }

    #[test]
    fn p0_r1_raw_verifier_error_remains_reportable() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let shell = InvocationBranded::from_test(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap(),
        );
        let active_session = session(&token);
        let input = complete
            .bind_physical(token, active_session, shell)
            .unwrap()
            .prepare_finalization()
            .unwrap();
        let mut finalized =
            crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::finalize(
                crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::prepare(input)
                    .unwrap(),
            );
        let function = finalized
            .input
            .module
            .functions
            .values_mut()
            .next()
            .expect("Raw verifier fixture function");
        let entry = function.entry_block;
        function
            .get_block_mut(entry)
            .expect("Raw verifier fixture entry block")
            .set_terminator(MirInstruction::Jump {
                target: BasicBlockId::new(9999),
                edge_args: None,
            });
        let mut verifier = MirVerifier::new();
        let processed = ModulePostprocessOwnerV1::new(&mut verifier, false)
            .run_raw(finalized)
            .expect("Raw verifier errors remain reportable");
        let crate::mir::compiler::module_postprocess::ModuleVerificationEvidenceV1::Raw {
            pre_transform,
        } = processed.verification
        else {
            panic!("Raw route must retain reportable pre-transform evidence")
        };
        assert!(pre_transform.is_err());
    }

    #[test]
    fn p0_r1_raw_real_authority_chain() {
        let (token, complete) = raw_complete();
        let brand = token.brand();
        let shell = InvocationBranded::from_test(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("p0_r1_raw".into()))
                .unwrap(),
        );
        let active_session = session(&token);
        let physical = complete.bind_physical(token, active_session, shell).unwrap();
        let physical = physical.prepare_finalization().unwrap();
        let finalized = crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::prepare(
            physical,
        )
        .unwrap();
        let finalized = crate::mir::compiler::raw_finalization::RawModuleFinalizerV1::finalize(
            finalized,
        );
        let mut verifier = MirVerifier::new();
        let processed = ModulePostprocessOwnerV1::new(&mut verifier, false)
            .run_raw(finalized)
            .unwrap();
        let mut compiler = MirCompiler::with_options(false);
        let prepared = compiler.prepare_module_external_commit(processed).unwrap();
        let result = compiler.commit_prepared_module(prepared);

        assert!(result.verification_result.is_ok());
        assert!(result
            .module
            .functions
            .keys()
            .any(|symbol| symbol.starts_with("main")));
        assert!(result
            .module
            .functions
            .keys()
            .any(|symbol| symbol.starts_with("condition_fn")));
    }
}
