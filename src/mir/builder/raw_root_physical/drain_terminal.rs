//! DRAIN-PHYSICAL0: one-shot Raw collector-to-shell drain.

use crate::mir::builder::module_draft_collector::{
    raw_collector_from_branded, DraftPublicationPolicyV1, FunctionDraftKeyV1,
    PreparedRawCollectorDrainV1, RawCollectorDrainErrorV1,
};
use crate::mir::builder::module_invocation_identity::{
    ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use crate::mir::builder::module_invocation_owner_chain::{
    BrandedCollectorV1, BrandedShellV1, InvocationBranded,
};
use crate::mir::builder::module_invocation_session::ModuleBuilderInvocationSessionV1;
use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1,
    PreparedModuleLoweringShellDrainV1,
};
use crate::mir::builder::raw_expansion_receipt_ledger::SealedRawExpansionReceiptLedgerV1;
use crate::mir::builder::raw_root_completion::{
    RawCompleteInvocationV1, RawInvocationRootWitnessV1,
};
use crate::mir::builder::raw_root_physical::drain_manifest::{
    project_raw_drain_manifest, RawDrainManifestErrorV1,
};
use crate::mir::raw_physical_drain::{
    RawPhysicalCallableMainDispositionV1, RawPhysicalDrainManifestV1, RawPhysicalDrainRoleV1,
    RawPhysicalDrainRouteV1,
};
use crate::mir::MirModule;

#[derive(Debug)]
pub(in crate::mir::builder) struct RawDrainPhysicalPartsV1 {
    pub(in crate::mir::builder) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir::builder) shell:
        BrandedShellV1<super::super::module_lowering_shell::ModuleLoweringShellV1>,
    pub(in crate::mir::builder) invocation: RawCompleteInvocationV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPhysicalDrainErrorV1 {
    NonRawFamily,
    ForeignBrand,
    PublishedShell { count: usize },
    Manifest(RawDrainManifestErrorV1),
    Collector(RawCollectorDrainErrorV1),
    Shell(ModuleLoweringShellErrorV1),
    RootWitnessMismatch,
}

#[derive(Debug)]
enum RejectedRawDrainOwnerV1 {
    Parts(RawDrainPhysicalPartsV1),
    Split {
        session: ModuleBuilderInvocationSessionV1,
        shell: BrandedShellV1<super::super::module_lowering_shell::ModuleLoweringShellV1>,
        token: ModuleInvocationTokenV1,
        collector: BrandedCollectorV1<super::super::module_draft_collector::ModuleDraftCollectorV1>,
        ledger: SealedRawExpansionReceiptLedgerV1,
        root: RawInvocationRootWitnessV1,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPhysicalDrainV1 {
    owner: RejectedRawDrainOwnerV1,
    error: RawPhysicalDrainErrorV1,
    _seal: RejectedRawPhysicalDrainSealV1,
}

#[derive(Debug)]
struct RejectedRawPhysicalDrainSealV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawPhysicalDrainV1 {
    token: ModuleInvocationTokenV1,
    session: ModuleBuilderInvocationSessionV1,
    shell: PreparedModuleLoweringShellDrainV1,
    collector: PreparedRawCollectorDrainV1,
    manifest: RawPhysicalDrainManifestV1,
    ledger: SealedRawExpansionReceiptLedgerV1,
    root: RawInvocationRootWitnessV1,
    _seal: PreparedRawPhysicalDrainSealV1,
}

#[derive(Debug)]
struct PreparedRawPhysicalDrainSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawUnfinalizedModuleV1 {
    module: MirModule,
    _seal: RawUnfinalizedModuleSealV1,
}

#[derive(Debug)]
struct RawUnfinalizedModuleSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawFinalizedModuleV1 {
    module: MirModule,
    _seal: RawFinalizedModuleSealV1,
}

#[derive(Debug)]
struct RawFinalizedModuleSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawDrainWitnessV1 {
    manifest: RawPhysicalDrainManifestV1,
    ledger: SealedRawExpansionReceiptLedgerV1,
    root: RawInvocationRootWitnessV1,
    _seal: RawDrainWitnessSealV1,
}

#[derive(Debug)]
struct RawDrainWitnessSealV1;

#[derive(Debug)]
pub(in crate::mir) struct RawDrainedPhysicalV1 {
    pub(in crate::mir) token: ModuleInvocationTokenV1,
    pub(in crate::mir) session: ModuleBuilderInvocationSessionV1,
    pub(in crate::mir) candidate: RawUnfinalizedModuleV1,
    pub(in crate::mir) witness: RawDrainWitnessV1,
    pub(in crate::mir::builder) _seal: RawDrainedPhysicalSealV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawDrainedPhysicalSealV1;

impl RawUnfinalizedModuleV1 {
    pub(in crate::mir::builder) fn name(&self) -> &str {
        &self.module.name
    }

    pub(in crate::mir::builder) fn function_count(&self) -> usize {
        self.module.functions.len()
    }

    pub(in crate::mir::builder) fn symbols(&self) -> impl Iterator<Item = &String> {
        self.module.functions.keys()
    }

    pub(in crate::mir::builder) fn function(
        &self,
        symbol: &str,
    ) -> Option<&crate::mir::MirFunction> {
        self.module.functions.get(symbol)
    }

    pub(in crate::mir::builder) fn finalize(self) -> RawFinalizedModuleV1 {
        RawFinalizedModuleV1 {
            module: self.module,
            _seal: RawFinalizedModuleSealV1,
        }
    }
}

impl RawFinalizedModuleV1 {
    pub(in crate::mir::builder) fn into_postprocess_module(self) -> MirModule {
        self.module
    }

    pub(in crate::mir) fn name(&self) -> &str {
        &self.module.name
    }

    pub(in crate::mir) fn function_count(&self) -> usize {
        self.module.functions.len()
    }

    pub(in crate::mir) fn symbols(&self) -> impl Iterator<Item = &String> {
        self.module.functions.keys()
    }

    pub(in crate::mir) fn function(&self, symbol: &str) -> Option<&crate::mir::MirFunction> {
        self.module.functions.get(symbol)
    }

    pub(in crate::mir) fn refresh_rune_plans(&mut self) {
        crate::mir::rune_plan_refresh::refresh_module_rune_plans(&mut self.module);
    }

    pub(in crate::mir) fn optimize(&mut self) -> crate::mir::optimizer_stats::OptimizationStats {
        crate::mir::optimizer::MirOptimizer::new().optimize_module(&mut self.module)
    }

    pub(in crate::mir) fn refresh_contracts(&mut self) -> Result<(), String> {
        crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
            &mut self.module,
            crate::mir::semantic_refresh::ContractRefreshBoundary::Verifier,
        )
        .map(|_| ())
    }

    pub(in crate::mir) fn verify(
        &mut self,
        verifier: &mut crate::mir::verification::MirVerifier,
    ) -> Result<(), Box<[crate::mir::verification_types::VerificationError]>> {
        verifier
            .verify_module(&mut self.module)
            .map_err(|errors| errors.into_boxed_slice())
    }

    pub(in crate::mir) fn insert_rc(&mut self) {
        crate::mir::passes::rc_insertion::insert_rc_instructions(&mut self.module);
    }

    pub(in crate::mir) fn refresh_semantic_metadata(&mut self) {
        crate::mir::semantic_refresh::refresh_module_semantic_metadata(&mut self.module);
    }

    pub(in crate::mir) fn canonicalize_callsites(&mut self) -> usize {
        crate::mir::passes::callsite_canonicalize::canonicalize_for_site(
            &mut self.module,
            crate::mir::passes::callsite_canonicalize::CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
        )
    }
}

impl RawDrainWitnessV1 {
    pub(in crate::mir) const fn brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.manifest.brand()
    }

    pub(in crate::mir::builder) fn manifest(&self) -> &RawPhysicalDrainManifestV1 {
        &self.manifest
    }

    pub(in crate::mir::builder) fn root(&self) -> &RawInvocationRootWitnessV1 {
        &self.root
    }

    pub(in crate::mir) fn ledger(&self) -> &SealedRawExpansionReceiptLedgerV1 {
        &self.ledger
    }

    pub(in crate::mir) fn vm_decode_plan(
        &self,
    ) -> Result<super::super::raw_root_body_exit::RawVmSourceEntryDecodeKindV1, ()> {
        self.root.exit().vm_decode_plan()
    }

    pub(in crate::mir) fn main_entry_target(
        &self,
    ) -> &super::super::root_batch_slot::RawMainEntryTargetV1 {
        self.root.main_entry_target()
    }
}

pub(in crate::mir::builder) fn prepare_from_parts(
    parts: RawDrainPhysicalPartsV1,
    route: RawPhysicalDrainRouteV1,
    callable_main: RawPhysicalCallableMainDispositionV1,
) -> Result<PreparedRawPhysicalDrainV1, RejectedRawPhysicalDrainV1> {
    let brand = parts.invocation.brand();
    if parts.session.family() != ModuleInvocationFamilyV1::Raw {
        return Err(reject(parts, RawPhysicalDrainErrorV1::NonRawFamily));
    }
    if parts.session.brand() != brand
        || parts.shell.brand() != brand
        || parts.invocation.ledger().brand() != brand
        || parts.invocation.root().brand() != brand
    {
        return Err(reject(parts, RawPhysicalDrainErrorV1::ForeignBrand));
    }
    if parts.shell.payload().has_published_functions() {
        let count = parts.shell.payload().published_function_count();
        return Err(reject(
            parts,
            RawPhysicalDrainErrorV1::PublishedShell { count },
        ));
    }
    if parts.invocation.root().callable_main() != map_callable_main(callable_main) {
        return Err(reject(parts, RawPhysicalDrainErrorV1::RootWitnessMismatch));
    }
    let manifest = match project_raw_drain_manifest(parts.invocation.ledger(), route, callable_main)
    {
        Ok(manifest) => manifest,
        Err(error) => return Err(reject(parts, RawPhysicalDrainErrorV1::Manifest(error))),
    };
    if !root_receipts_match_manifest(parts.invocation.root(), &manifest, brand) {
        return Err(reject(parts, RawPhysicalDrainErrorV1::RootWitnessMismatch));
    }
    let symbols = manifest
        .rows()
        .iter()
        .map(|row| row.symbol().to_owned())
        .collect::<Vec<_>>();
    let inventory = match ModuleLoweringShellDrainInventoryV1::from_symbols(symbols) {
        Ok(inventory) => inventory,
        Err(error) => return Err(reject(parts, RawPhysicalDrainErrorV1::Shell(error))),
    };

    let RawDrainPhysicalPartsV1 {
        session,
        shell,
        invocation,
    } = parts;
    let (token, collector, ledger, root) = invocation.into_parts();
    let collector = match raw_collector_from_branded(collector, &manifest, brand) {
        Ok(collector) => collector,
        Err((collector, error)) => {
            return Err(RejectedRawPhysicalDrainV1 {
                owner: RejectedRawDrainOwnerV1::Split {
                    session,
                    shell,
                    token,
                    collector,
                    ledger,
                    root,
                },
                error: RawPhysicalDrainErrorV1::Collector(error),
                _seal: RejectedRawPhysicalDrainSealV1,
            })
        }
    };
    let shell = shell.into_payload().prepare_drain(inventory);
    Ok(PreparedRawPhysicalDrainV1 {
        token,
        session,
        shell,
        collector,
        manifest,
        ledger,
        root,
        _seal: PreparedRawPhysicalDrainSealV1,
    })
}

impl PreparedRawPhysicalDrainV1 {
    pub(in crate::mir) fn drain(self) -> RawDrainedPhysicalV1 {
        let Self {
            token,
            session,
            shell,
            collector,
            manifest,
            ledger,
            root,
            _seal: _,
        } = self;
        let functions = collector.drain();
        let module = shell.commit_preflighted(functions);
        RawDrainedPhysicalV1 {
            token,
            session,
            candidate: RawUnfinalizedModuleV1 {
                module,
                _seal: RawUnfinalizedModuleSealV1,
            },
            witness: RawDrainWitnessV1 {
                manifest,
                ledger,
                root,
                _seal: RawDrainWitnessSealV1,
            },
            _seal: RawDrainedPhysicalSealV1,
        }
    }
}

impl RejectedRawPhysicalDrainV1 {
    pub(in crate::mir) fn error(&self) -> &RawPhysicalDrainErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

fn reject(
    parts: RawDrainPhysicalPartsV1,
    error: RawPhysicalDrainErrorV1,
) -> RejectedRawPhysicalDrainV1 {
    RejectedRawPhysicalDrainV1 {
        owner: RejectedRawDrainOwnerV1::Parts(parts),
        error,
        _seal: RejectedRawPhysicalDrainSealV1,
    }
}

fn map_callable_main(
    disposition: RawPhysicalCallableMainDispositionV1,
) -> crate::mir::builder::RawCallableMainCompatibilityDispositionV1 {
    match disposition {
        RawPhysicalCallableMainDispositionV1::NotSelected => {
            crate::mir::builder::RawCallableMainCompatibilityDispositionV1::NotSelected
        }
        RawPhysicalCallableMainDispositionV1::Selected => {
            crate::mir::builder::RawCallableMainCompatibilityDispositionV1::Selected
        }
    }
}

fn root_receipts_match_manifest(
    root: &RawInvocationRootWitnessV1,
    manifest: &RawPhysicalDrainManifestV1,
    brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
) -> bool {
    let Some(main) = manifest
        .rows()
        .iter()
        .find(|row| row.role() == RawPhysicalDrainRoleV1::RootMain)
    else {
        return false;
    };
    let Some(condition) = manifest
        .rows()
        .iter()
        .find(|row| row.role() == RawPhysicalDrainRoleV1::RequiredCondition)
    else {
        return false;
    };
    receipt_matches(root.main_receipt(), main, brand)
        && receipt_matches(root.condition_receipt(), condition, brand)
}

fn receipt_matches(
    receipt: &InvocationBranded<
        crate::mir::builder::module_draft_collector::CollectedDraftAdmissionReceiptV1,
    >,
    row: &crate::mir::raw_physical_drain::RawPhysicalDrainRowV1,
    brand: crate::mir::module_invocation_identity::ModuleInvocationBrandV1,
) -> bool {
    let payload = receipt.payload();
    let expected_key = match row.key() {
        crate::mir::raw_physical_drain::RawPhysicalDrainKeyV1::RootMain => FunctionDraftKeyV1::Main,
        crate::mir::raw_physical_drain::RawPhysicalDrainKeyV1::RequiredCondition => {
            FunctionDraftKeyV1::SyntheticConditionFn
        }
        crate::mir::raw_physical_drain::RawPhysicalDrainKeyV1::LegacySymbol(symbol) => {
            FunctionDraftKeyV1::LegacySymbol(symbol.to_string())
        }
    };
    let expected_policy = match row.policy() {
        crate::mir::raw_physical_drain::RawPhysicalDrainPolicyV1::LegacyReplaceWholePair => {
            DraftPublicationPolicyV1::LegacyReplaceWholePair
        }
        crate::mir::raw_physical_drain::RawPhysicalDrainPolicyV1::CanonicalRejectDuplicate => {
            DraftPublicationPolicyV1::CanonicalRejectDuplicate
        }
    };
    receipt.brand() == brand
        && payload.collector_brand() == Some(brand)
        && payload.key() == &expected_key
        && payload.symbol() == row.symbol()
        && payload.arity() == row.arity()
        && payload.policy() == expected_policy
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::main_pending_draft::{
        MainCompletionRequestV1, MainDraftIdentityV1, MainHeaderLoanV1, MainHeaderSourceV1,
    };
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_invocation_identity::{
        ModuleInvocationBrandV1, TestInvocationPreflightFactoryV1,
    };
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
    use crate::mir::builder::raw_root_completion_preflight::RawRootCompletionInputV1;
    use crate::mir::builder::root_body_completion::{
        RootBodyCompletionTrackerV1, RootBodyResultV1,
    };
    use crate::mir::builder::MirBuilder;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

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

    fn raw_complete() -> (ModuleInvocationBrandV1, RawCompleteInvocationV1) {
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
        let batch = crate::mir::builder::root_draft_batch::PreparedRootDraftBatchV1::prepare(
            main,
            Some(draft("condition_fn", 1)),
            crate::mir::builder::module_invocation_drain::ConditionFnPolicyV1::Required,
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
        let complete = complete_raw_root(RawRootCompletionInputV1::new(
            token,
            InvocationBranded::from_test(brand, ModuleDraftCollectorV1::with_brand(brand)),
            ledger,
            batch,
            main_reservation,
            condition_reservation,
            RawCallableMainCompatibilityDispositionV1::NotSelected,
        ))
        .unwrap();
        (brand, complete)
    }

    fn session(complete: &RawCompleteInvocationV1) -> ModuleBuilderInvocationSessionV1 {
        let live = MirBuilder::new();
        let config =
            BuilderInvocationConfigV1::snapshot_with_policy(&live, BuilderCoreSeedPolicyV1::Fresh);
        ModuleBuilderInvocationSessionV1::open_for_token(complete.token(), &live, config)
    }

    #[test]
    fn published_shell_rejection_is_owned_by_drain0() {
        let (brand, invocation) = raw_complete();
        let mut shell =
            ModuleLoweringShellV1::from_empty_module(MirModule::new("raw".into())).unwrap();
        shell.publish_function_for_test(draft("already/0", 0));
        let rejected = prepare_from_parts(
            RawDrainPhysicalPartsV1 {
                session: session(&invocation),
                shell: InvocationBranded::from_test(brand, shell),
                invocation,
            },
            RawPhysicalDrainRouteV1::Script,
            RawPhysicalCallableMainDispositionV1::NotSelected,
        )
        .expect_err("DRAIN0 must reject a published shell before projection");
        assert!(matches!(
            rejected.error(),
            RawPhysicalDrainErrorV1::PublishedShell { count: 1 }
        ));
        rejected.discard();
    }
}
