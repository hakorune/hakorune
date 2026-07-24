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
    _seal: RawDrainedPhysicalSealV1,
}

#[derive(Debug)]
struct RawDrainedPhysicalSealV1;

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
