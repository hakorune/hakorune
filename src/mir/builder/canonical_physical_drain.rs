//! ROOT0-DRAIN0-PHYSICAL0-PREP0: mutation-free physical preflight.
//!
//! This module only joins the neutral source manifest to the real shell,
//! collector, and collector-issued receipt.  It does not publish functions;
//! the prepared products are consumed by the later I0 drain terminal.

use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};

use super::module_draft_collector::{
    CanonicalCollectorDrainErrorV1, CanonicalCollectorReceiptViewV1,
    CollectedDraftAdmissionReceiptV1, PreparedCanonicalCollectorDrainV1,
};
use super::module_invocation_brand0::{
    CollectedCanonicalCallablePhysicalV1, CollectedCanonicalSinglePhysicalV1,
};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1, InvocationBranded};
use super::module_draft_collector::CallableCollectorBatchReceiptV1;
use super::module_lowering_shell::ModuleLoweringShellV1;

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CanonicalPhysicalDrainPrepareErrorV1 {
    ForeignBrand,
    WrongFamily { family: ModuleInvocationFamilyV1 },
    PublishedShell { count: usize },
    ReceiptCollectorBrandMismatch,
    Collector(CanonicalCollectorDrainErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalSinglePhysicalDrainV1 {
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    error: CanonicalPhysicalDrainPrepareErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalCallablePhysicalDrainV1 {
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
    error: CanonicalPhysicalDrainPrepareErrorV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedCanonicalSinglePhysicalDrainV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: PreparedCanonicalCollectorDrainV1,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedCanonicalCallablePhysicalDrainV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: PreparedCanonicalCollectorDrainV1,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
}

impl CollectedCanonicalSinglePhysicalV1 {
    pub(in crate::mir) fn prepare_drain(
        self,
        manifest: &CanonicalPhysicalDrainManifestV1,
    ) -> Result<PreparedCanonicalSinglePhysicalDrainV1, RejectedCanonicalSinglePhysicalDrainV1>
    {
        let (shell, collector, receipt) = self.into_parts();
        let brand = manifest.brand();
        let family = manifest.family();
        if shell.brand() != brand || collector.brand() != brand || receipt.brand() != brand {
            return Err(reject_single(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::ForeignBrand,
            ));
        }
        if !matches!(family, ModuleInvocationFamilyV1::CanonicalAPlus
            | ModuleInvocationFamilyV1::BindingSsaTrivial)
        {
            return Err(reject_single(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::WrongFamily { family },
            ));
        }
        if shell.payload().has_published_functions() {
            let count = shell.payload().published_function_count();
            return Err(reject_single(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::PublishedShell { count },
            ));
        }
        if receipt.payload().collector_brand() != Some(brand) {
            return Err(reject_single(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::ReceiptCollectorBrandMismatch,
            ));
        }
        let prepared = prepare_collector(
            collector,
            manifest,
            CanonicalCollectorReceiptViewV1::Single(&receipt),
            brand,
        );
        match prepared {
            Ok(collector) => Ok(PreparedCanonicalSinglePhysicalDrainV1 {
                brand,
                family,
                shell,
                collector,
                receipt,
            }),
            Err((collector, error)) => Err(reject_single(shell, collector, receipt, error)),
        }
    }
}

impl CollectedCanonicalCallablePhysicalV1 {
    pub(in crate::mir) fn prepare_drain(
        self,
        manifest: &CanonicalPhysicalDrainManifestV1,
    ) -> Result<PreparedCanonicalCallablePhysicalDrainV1, RejectedCanonicalCallablePhysicalDrainV1>
    {
        let (shell, collector, receipt) = self.into_parts();
        let brand = manifest.brand();
        let family = manifest.family();
        if shell.brand() != brand || collector.brand() != brand || receipt.brand() != brand {
            return Err(reject_callable(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::ForeignBrand,
            ));
        }
        if !matches!(family, ModuleInvocationFamilyV1::BindingSsaAcyclic
            | ModuleInvocationFamilyV1::BindingSsaRecursive)
        {
            return Err(reject_callable(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::WrongFamily { family },
            ));
        }
        if shell.payload().has_published_functions() {
            let count = shell.payload().published_function_count();
            return Err(reject_callable(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::PublishedShell { count },
            ));
        }
        if receipt
            .payload()
            .admissions()
            .iter()
            .any(|admission| admission.collector_brand() != Some(brand))
        {
            return Err(reject_callable(
                shell,
                collector,
                receipt,
                CanonicalPhysicalDrainPrepareErrorV1::ReceiptCollectorBrandMismatch,
            ));
        }
        let prepared = prepare_collector(
            collector,
            manifest,
            CanonicalCollectorReceiptViewV1::Callable(&receipt),
            brand,
        );
        match prepared {
            Ok(collector) => Ok(PreparedCanonicalCallablePhysicalDrainV1 {
                brand,
                family,
                shell,
                collector,
                receipt,
            }),
            Err((collector, error)) => Err(reject_callable(shell, collector, receipt, error)),
        }
    }
}

fn prepare_collector(
    collector: BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>,
    manifest: &CanonicalPhysicalDrainManifestV1,
    receipt: CanonicalCollectorReceiptViewV1<'_>,
    brand: ModuleInvocationBrandV1,
) -> Result<PreparedCanonicalCollectorDrainV1, (BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>, CanonicalPhysicalDrainPrepareErrorV1)> {
    let collector_payload = collector.into_payload();
    match collector_payload.prepare_canonical_drain(manifest, receipt, brand) {
        Ok(prepared) => Ok(prepared),
        Err(rejected) => {
            let (collector, error) = rejected.into_parts();
            Err((
                InvocationBranded::from_source(brand, collector),
                CanonicalPhysicalDrainPrepareErrorV1::Collector(error),
            ))
        }
    }
}

fn reject_single(
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    error: CanonicalPhysicalDrainPrepareErrorV1,
) -> RejectedCanonicalSinglePhysicalDrainV1 {
    RejectedCanonicalSinglePhysicalDrainV1 {
        shell,
        collector,
        receipt,
        error,
    }
}

fn reject_callable(
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<super::module_draft_collector::ModuleDraftCollectorV1>,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
    error: CanonicalPhysicalDrainPrepareErrorV1,
) -> RejectedCanonicalCallablePhysicalDrainV1 {
    RejectedCanonicalCallablePhysicalDrainV1 {
        shell,
        collector,
        receipt,
        error,
    }
}
