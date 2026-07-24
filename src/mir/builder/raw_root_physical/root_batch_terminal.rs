//! ROOTBATCH0-S0c/S0d: paired Raw Main/condition physical handoff.
//!
//! All semantic checks borrow the complete BODY0 owner.  Only after the
//! collector and ledger plans agree does this terminal consume shell,
//! collector, and ledger and issue the unpublished root-batch product.

use super::{RawRootLedgerStateV1, RawRootPostBodyPhysicalStateV1};
use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_invocation_brand0::InvocationPhysicalStateV1;
use crate::mir::builder::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use crate::mir::builder::module_invocation_owner_chain::{
    BrandedCollectorV1, BrandedShellV1, InvocationBranded,
};
use crate::mir::builder::module_invocation_session::ModuleBuilderInvocationSessionV1;
use crate::mir::builder::module_lowering_shell::ModuleLoweringShellV1;
use crate::mir::builder::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionReceiptLedgerErrorV1,
    RawRootMainCommitDispositionV1,
};
use crate::mir::builder::raw_required_condition_draft::RawRequiredConditionDraftV1;
use crate::mir::builder::raw_root_body_exit::{
    RawRootBodyExitWitnessErrorV1, RawRootBodyExitWitnessV1,
};
use crate::mir::builder::raw_root_completion::RawCompleteInvocationV1;
use crate::mir::builder::root_body_completion::CompletedRootBodyV1;
use crate::mir::builder::root_draft_batch::PreparedRootDraftBatchV1;
use crate::mir::MirFunction;

/// Named handoff from BODY0.  The compiler never receives the session,
/// physical carrier, draft, and witness as a loose tuple.
#[derive(Debug)]
pub(in crate::mir) struct RawRootBatchPhysicalInputV1 {
    pub(in crate::mir::builder) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir::builder) physical: RawRootPostBodyPhysicalStateV1,
    pub(in crate::mir::builder) draft: MirFunction,
    pub(in crate::mir::builder) completion: CompletedRootBodyV1,
    pub(in crate::mir::builder) exit: RawRootBodyExitWitnessV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBatchPhysicalErrorV1 {
    NonRawFamily,
    ForeignBrand,
    MainIdentityMismatch { symbol: String, arity: usize },
    CompletionBrandMismatch,
    PublishedRootFunctions { count: usize },
    Ledger(RawExpansionReceiptLedgerErrorV1),
    Collector(String),
    MainDispositionMismatch,
    CallableDispositionMismatch,
    ExitWitness(RawRootBodyExitWitnessErrorV1),
}

#[derive(Debug)]
enum RejectedRawRootBatchOwnerV1 {
    BeforePrepare {
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootPostBodyPhysicalStateV1,
        draft: MirFunction,
        completion: CompletedRootBodyV1,
        exit: RawRootBodyExitWitnessV1,
    },
    Prepared {
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootPostBodyPhysicalStateV1,
        batch: PreparedRootDraftBatchV1,
        exit: RawRootBodyExitWitnessV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootBatchPhysicalV1 {
    token: ModuleInvocationTokenV1,
    owner: RejectedRawRootBatchOwnerV1,
    error: RawRootBatchPhysicalErrorV1,
    _seal: RejectedRawRootBatchPhysicalSealV1,
}

#[derive(Debug)]
struct RejectedRawRootBatchPhysicalSealV1;

#[derive(Debug)]
pub(in crate::mir) struct CompletedRawRootBatchPhysicalV1 {
    session: ModuleBuilderInvocationSessionV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    invocation: RawCompleteInvocationV1,
    _seal: CompletedRawRootBatchPhysicalSealV1,
}

#[derive(Debug)]
struct CompletedRawRootBatchPhysicalSealV1;

impl RawRootBatchPhysicalInputV1 {
    pub(in crate::mir) fn prepare_raw_root_batch(
        self,
        token: ModuleInvocationTokenV1,
    ) -> Result<CompletedRawRootBatchPhysicalV1, RejectedRawRootBatchPhysicalV1> {
        let Self {
            session,
            physical,
            draft,
            completion,
            exit,
        } = self;
        let brand = token.brand();
        let reject = |token, session, physical, draft, completion, exit, error| {
            Err(RejectedRawRootBatchPhysicalV1 {
                token,
                owner: RejectedRawRootBatchOwnerV1::BeforePrepare {
                    session,
                    physical,
                    draft,
                    completion,
                    exit,
                },
                error,
                _seal: RejectedRawRootBatchPhysicalSealV1,
            })
        };

        if token.family() != ModuleInvocationFamilyV1::Raw
            || session.family() != ModuleInvocationFamilyV1::Raw
        {
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::NonRawFamily,
            );
        }
        if session.brand() != brand || physical.brand() != brand {
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::ForeignBrand,
            );
        }
        if completion.brand() != brand {
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::CompletionBrandMismatch,
            );
        }
        if let Err(error) = exit.validate(&draft, &completion, brand) {
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::ExitWitness(error),
            );
        }
        let main_contract = super::super::root_batch_slot::RawRootBatchSlotV1::Main.contract();
        if draft.signature.name != main_contract.symbol()
            || draft.signature.params.len() != main_contract.arity()
        {
            let symbol = draft.signature.name.clone();
            let arity = draft.signature.params.len();
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::MainIdentityMismatch { symbol, arity },
            );
        }
        if !physical.shell_is_empty() {
            let count = physical.published_function_count();
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::PublishedRootFunctions { count },
            );
        }
        let ledger = match physical.open_ledger() {
            Some(ledger) => ledger,
            None => {
                return reject(
                    token,
                    session,
                    physical,
                    draft,
                    completion,
                    exit,
                    RawRootBatchPhysicalErrorV1::Ledger(
                        RawExpansionReceiptLedgerErrorV1::LedgerPoisoned,
                    ),
                )
            }
        };
        if ledger.callable_main() != physical.callable_main() {
            return reject(
                token,
                session,
                physical,
                draft,
                completion,
                exit,
                RawRootBatchPhysicalErrorV1::CallableDispositionMismatch,
            );
        }
        let ledger_plan = match ledger.prepare_required_root_pair() {
            Ok(plan) => plan,
            Err(error) => {
                return reject(
                    token,
                    session,
                    physical,
                    draft,
                    completion,
                    exit,
                    RawRootBatchPhysicalErrorV1::Ledger(error),
                )
            }
        };
        let collector = physical.collector();
        let condition = RawRequiredConditionDraftV1::build();
        let batch = PreparedRootDraftBatchV1::prepare_raw_required(draft, completion, condition);
        if let Err(error) = collector.payload().validate_root_batch(&batch, brand) {
            return Err(RejectedRawRootBatchPhysicalV1 {
                token,
                owner: RejectedRawRootBatchOwnerV1::Prepared {
                    session,
                    physical,
                    batch,
                    exit,
                },
                error: RawRootBatchPhysicalErrorV1::Collector(error.to_string()),
                _seal: RejectedRawRootBatchPhysicalSealV1,
            });
        }
        let collector_disposition = match collector.payload().raw_root_main_disposition() {
            Ok(disposition) => disposition,
            Err(error) => {
                return Err(RejectedRawRootBatchPhysicalV1 {
                    token,
                    owner: RejectedRawRootBatchOwnerV1::Prepared {
                        session,
                        physical,
                        batch,
                        exit,
                    },
                    error: RawRootBatchPhysicalErrorV1::Collector(error.to_string()),
                    _seal: RejectedRawRootBatchPhysicalSealV1,
                })
            }
        };
        if collector_disposition != *ledger_plan.main_disposition() {
            return Err(RejectedRawRootBatchPhysicalV1 {
                token,
                owner: RejectedRawRootBatchOwnerV1::Prepared {
                    session,
                    physical,
                    batch,
                    exit,
                },
                error: RawRootBatchPhysicalErrorV1::MainDispositionMismatch,
                _seal: RejectedRawRootBatchPhysicalSealV1,
            });
        }

        // From this point all semantic work is complete.  The remaining
        // operations are private invariant-preserving commits.
        let (invocation_physical, ledger_state, callable_main) = physical.into_parts();
        let (physical_brand, shell, collector) = invocation_physical.into_parts();
        debug_assert_eq!(physical_brand, brand);
        let ledger = match ledger_state {
            RawRootLedgerStateV1::Open(ledger) => ledger,
            RawRootLedgerStateV1::Aborted(_) | RawRootLedgerStateV1::AbortedPlaceholder => {
                unreachable!("clean Raw root ledger changed after preflight")
            }
        };
        let (ledger, main_reservation, condition_reservation) =
            ledger_plan.commit_reservations(ledger);
        let prepared_collector = collector
            .into_payload()
            .prepare_root_batch_preflighted(batch);
        let (collector, branded_batch) = prepared_collector
            .commit_branded()
            .unwrap_or_else(|_| unreachable!("collector brand drifted after preflight"));
        let (admissions, root_body, receipt_brand) = branded_batch.into_parts();
        debug_assert_eq!(receipt_brand, brand);
        let mut main = None;
        let mut condition = None;
        for receipt in admissions.into_vec() {
            match receipt.payload().key() {
                FunctionDraftKeyV1::Main => main = Some(receipt),
                FunctionDraftKeyV1::SyntheticConditionFn => condition = Some(receipt),
                _ => unreachable!("Raw root batch emitted a non-root receipt"),
            }
        }
        let main = main.unwrap_or_else(|| unreachable!("Raw root Main receipt disappeared"));
        let condition =
            condition.unwrap_or_else(|| unreachable!("Raw root condition receipt disappeared"));
        let mut ledger = ledger;
        ledger.commit_required_root_batch_preflighted(
            main_reservation,
            &main,
            condition_reservation,
            &condition,
        );
        let ledger = ledger
            .seal()
            .unwrap_or_else(|_| unreachable!("Raw root ledger drifted after preflight"));
        let invocation = RawCompleteInvocationV1::from_committed_parts(
            token,
            InvocationBranded::from_source(brand, collector),
            ledger,
            root_body,
            exit,
            main,
            condition,
            callable_main,
        );
        Ok(CompletedRawRootBatchPhysicalV1 {
            session,
            shell,
            invocation,
            _seal: CompletedRawRootBatchPhysicalSealV1,
        })
    }
}

impl CompletedRawRootBatchPhysicalV1 {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.invocation.brand()
    }

    pub(in crate::mir) fn prepare_raw_drain(
        self,
        route: crate::mir::raw_physical_drain::RawPhysicalDrainRouteV1,
        callable_main: crate::mir::raw_physical_drain::RawPhysicalCallableMainDispositionV1,
    ) -> Result<
        super::drain_terminal::PreparedRawPhysicalDrainV1,
        super::drain_terminal::RejectedRawPhysicalDrainV1,
    > {
        let Self {
            session,
            shell,
            invocation,
            _seal: _,
        } = self;
        super::drain_terminal::prepare_from_parts(
            super::drain_terminal::RawDrainPhysicalPartsV1 {
                session,
                shell,
                invocation,
            },
            route,
            callable_main,
        )
    }
}

impl RejectedRawRootBatchPhysicalV1 {
    pub(in crate::mir) fn error(&self) -> &RawRootBatchPhysicalErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}
