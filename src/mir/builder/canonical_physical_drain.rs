//! ROOT0-DRAIN0-PHYSICAL0-PREP0: mutation-free physical preflight.
//!
//! This module only joins the neutral source manifest to the real shell,
//! collector, and collector-issued receipt.  It does not publish functions;
//! the prepared products are consumed by the later I0 drain terminal.

use crate::mir::canonical_physical_drain::CanonicalPhysicalDrainManifestV1;
use crate::mir::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use crate::mir::MirModule;

use super::module_draft_collector::{
    CanonicalCollectorDrainErrorV1, CanonicalCollectorReceiptViewV1,
    CollectedDraftAdmissionReceiptV1, PreparedCanonicalCollectorDrainV1,
};
use super::module_invocation_brand0::{
    CollectedCanonicalCallablePhysicalV1, CollectedCanonicalSinglePhysicalV1,
};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1, InvocationBranded};
use super::module_draft_collector::CallableCollectorBatchReceiptV1;
use super::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl RejectedCanonicalSinglePhysicalDrainV1 {
    pub(in crate::mir) fn error(&self) -> &CanonicalPhysicalDrainPrepareErrorV1 {
        &self.error
    }
}

impl RejectedCanonicalCallablePhysicalDrainV1 {
    pub(in crate::mir) fn error(&self) -> &CanonicalPhysicalDrainPrepareErrorV1 {
        &self.error
    }
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedCanonicalSinglePhysicalDrainV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: PreparedCanonicalCollectorDrainV1,
    receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    manifest: CanonicalPhysicalDrainManifestV1,
}

#[derive(Debug)]
pub(in crate::mir) struct PreparedCanonicalCallablePhysicalDrainV1 {
    brand: ModuleInvocationBrandV1,
    family: ModuleInvocationFamilyV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: PreparedCanonicalCollectorDrainV1,
    receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
    manifest: CanonicalPhysicalDrainManifestV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalDrainedSinglePhysicalV1 {
    pub(in crate::mir) brand: ModuleInvocationBrandV1,
    pub(in crate::mir) family: ModuleInvocationFamilyV1,
    pub(in crate::mir) module: MirModule,
    pub(in crate::mir) receipt: InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    pub(in crate::mir) inventory: CanonicalPhysicalDrainManifestV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CanonicalDrainedCallablePhysicalV1 {
    pub(in crate::mir) brand: ModuleInvocationBrandV1,
    pub(in crate::mir) family: ModuleInvocationFamilyV1,
    pub(in crate::mir) module: MirModule,
    pub(in crate::mir) receipt: InvocationBranded<CallableCollectorBatchReceiptV1>,
    pub(in crate::mir) inventory: CanonicalPhysicalDrainManifestV1,
}

impl CollectedCanonicalSinglePhysicalV1 {
    pub(in crate::mir) fn prepare_drain(
        self,
        manifest: CanonicalPhysicalDrainManifestV1,
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
            &manifest,
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
                manifest,
            }),
            Err((collector, error)) => Err(reject_single(shell, collector, receipt, error)),
        }
    }
}

impl CollectedCanonicalCallablePhysicalV1 {
    pub(in crate::mir) fn prepare_drain(
        self,
        manifest: CanonicalPhysicalDrainManifestV1,
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
            &manifest,
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
                manifest,
            }),
            Err((collector, error)) => Err(reject_callable(shell, collector, receipt, error)),
        }
    }
}

impl PreparedCanonicalSinglePhysicalDrainV1 {
    /// The keyed collector proof has completed; this is the only physical
    /// publication move and cannot fail.
    pub(in crate::mir) fn drain(self) -> CanonicalDrainedSinglePhysicalV1 {
        let Self {
            brand,
            family,
            shell,
            collector,
            receipt,
            manifest,
        } = self;
        let functions = collector.drain();
        let symbols = functions
            .iter()
            .map(|function| function.signature.name.clone())
            .collect::<Vec<_>>();
        let inventory = ModuleLoweringShellDrainInventoryV1::from_symbols(symbols)
            .expect("collector proof guarantees unique canonical symbols");
        let module = shell
            .into_payload()
            .prepare_drain(inventory)
            .commit_preflighted(functions);
        CanonicalDrainedSinglePhysicalV1 {
            brand,
            family,
            module,
            receipt,
            inventory: manifest,
        }
    }
}

impl PreparedCanonicalCallablePhysicalDrainV1 {
    /// The keyed collector proof has completed; this is the only physical
    /// publication move and cannot fail.
    pub(in crate::mir) fn drain(self) -> CanonicalDrainedCallablePhysicalV1 {
        let Self {
            brand,
            family,
            shell,
            collector,
            receipt,
            manifest,
        } = self;
        let functions = collector.drain();
        let symbols = functions
            .iter()
            .map(|function| function.signature.name.clone())
            .collect::<Vec<_>>();
        let inventory = ModuleLoweringShellDrainInventoryV1::from_symbols(symbols)
            .expect("collector proof guarantees unique canonical symbols");
        let module = shell
            .into_payload()
            .prepare_drain(inventory)
            .commit_preflighted(functions);
        CanonicalDrainedCallablePhysicalV1 {
            brand,
            family,
            module,
            receipt,
            inventory: manifest,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::canonical_physical_drain::{
        CanonicalInsertedDispositionV1, CanonicalPhysicalCallableRowV1,
        CanonicalPhysicalSingleRowV1,
    };
    use crate::mir::builder::module_draft_collector::{FunctionDraftKeyV1, ModuleDraftCollectorV1};
    use crate::mir::builder::module_invocation_identity::ModuleInvocationFamilyV1;
    use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIssuerV1};
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

    fn draft(symbol: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.to_owned(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn single_fixture(
        brand: ModuleInvocationBrandV1,
        published: bool,
    ) -> (CollectedCanonicalSinglePhysicalV1, CanonicalPhysicalDrainManifestV1) {
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = owners.issue().unwrap();
        let collected = InvocationBranded::from_source(
            brand,
            ModuleDraftCollectorV1::with_brand(brand),
        )
        .collect_canonical_single(
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            "owner/0".to_owned(),
            0,
            draft("owner/0"),
        )
        .unwrap();
        let mut shell = ModuleLoweringShellV1::from_empty_module(MirModule::new("fixture".into()))
            .unwrap();
        if published {
            shell.publish_function_for_test(draft("already/0"));
        }
        let shell = InvocationBranded::from_source(brand, shell);
        let physical = CollectedCanonicalSinglePhysicalV1::from_test(shell, collected);
        let manifest = CanonicalPhysicalDrainManifestV1::single(
            brand,
            ModuleInvocationFamilyV1::BindingSsaTrivial,
            CanonicalPhysicalSingleRowV1::new(
                owner,
                "owner/0".into(),
                0,
                CanonicalInsertedDispositionV1::from_canonical_source(),
            ),
        );
        (physical, manifest)
    }

    #[test]
    fn published_shell_rejects_before_collector_prepare() {
        let (physical, manifest) = single_fixture(ModuleInvocationBrandV1::test_with_ordinal(91), true);
        let rejected = physical.prepare_drain(manifest).unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::PublishedShell { count: 1 }
        ));
    }

    #[test]
    fn manifest_symbol_drift_rejects_before_shell_mutation() {
        let (physical, manifest) = single_fixture(ModuleInvocationBrandV1::test_with_ordinal(92), false);
        let row = manifest.single_row().expect("single fixture row");
        let manifest = CanonicalPhysicalDrainManifestV1::single(
            manifest.brand(),
            manifest.family(),
            CanonicalPhysicalSingleRowV1::new(
                row.owner(),
                "wrong/0".into(),
                row.arity(),
                CanonicalInsertedDispositionV1::from_canonical_source(),
            ),
        );
        let rejected = physical.prepare_drain(manifest).unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::Collector(
                CanonicalCollectorDrainErrorV1::SymbolMismatch { .. }
            )
        ));
    }

    #[test]
    fn collector_payload_receipt_brand_mismatch_rejects_before_shell_mutation() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(98);
        let foreign = ModuleInvocationBrandV1::test_with_ordinal(99);
        let (physical, manifest) = single_fixture(brand, false);
        let (shell, collector, _receipt) = physical.into_parts();

        // The wrapper carries the active invocation brand, while its payload
        // receipt was issued by a different collector.  This is distinct from
        // a foreign wrapper-brand mismatch and must fail before shell/collector
        // mutation.
        let mut owners = FunctionOwnerIssuerV1::new_for_compilation().unwrap();
        let owner = owners.issue().unwrap();
        let foreign_product = InvocationBranded::from_source(
            foreign,
            ModuleDraftCollectorV1::with_brand(foreign),
        )
        .collect_canonical_single(
            FunctionDraftKeyV1::CanonicalResolvedOwner(owner),
            "owner/0".to_owned(),
            0,
            draft("owner/0"),
        )
        .unwrap();
        let (_foreign_collector, foreign_receipt) = foreign_product.into_parts();
        let mismatched_receipt = InvocationBranded::from_test(brand, foreign_receipt.into_payload());
        let collected =
            super::super::module_draft_collector::CollectedDraftAdmissionProductV1::from_test_parts(
                collector,
                mismatched_receipt,
            );
        let physical = CollectedCanonicalSinglePhysicalV1::from_test(shell, collected);

        let rejected = physical.prepare_drain(manifest).unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::ReceiptCollectorBrandMismatch
        ));
    }

    #[test]
    fn foreign_manifest_brand_rejects_before_collector_prepare() {
        let (physical, manifest) = single_fixture(ModuleInvocationBrandV1::test_with_ordinal(96), false);
        let foreign_manifest = CanonicalPhysicalDrainManifestV1::single(
            ModuleInvocationBrandV1::test_with_ordinal(97),
            manifest.family(),
            {
                let row = manifest.single_row().expect("single fixture row");
                CanonicalPhysicalSingleRowV1::new(
                    row.owner(),
                    row.symbol().into(),
                    row.arity(),
                    CanonicalInsertedDispositionV1::from_canonical_source(),
                )
            },
        );
        let rejected = physical.prepare_drain(foreign_manifest).unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::ForeignBrand
        ));
    }

    fn callable_fixture(
        brand: ModuleInvocationBrandV1,
    ) -> (CollectedCanonicalCallablePhysicalV1, Vec<CanonicalCallableKeyV1>) {
        let keys = vec![
            CanonicalCallableKeyV1::free_static_for_test("alpha", 0),
            CanonicalCallableKeyV1::free_static_for_test("zeta", 0),
        ];
        let entries = keys
            .iter()
            .map(|key| {
                super::super::module_draft_collector::CallableCollectorDraftEntryV1::new(
                    FunctionDraftKeyV1::CanonicalCallable(key.clone()),
                    format!("{}/0", key.name()),
                    0,
                    draft(&format!("{}/0", key.name())),
                )
            })
            .collect();
        let collector = InvocationBranded::from_source(
            brand,
            ModuleDraftCollectorV1::with_brand(brand),
        );
        let collected = collector
            .into_payload()
            .prepare_callable_batch(entries)
            .unwrap()
            .collect_all_branded()
            .unwrap();
        let shell = InvocationBranded::from_source(
            brand,
            ModuleLoweringShellV1::from_empty_module(MirModule::new("callable".into())).unwrap(),
        );
        (
            CollectedCanonicalCallablePhysicalV1::from_test(shell, collected),
            keys,
        )
    }

    fn callable_manifest(
        brand: ModuleInvocationBrandV1,
        keys: &[CanonicalCallableKeyV1],
    ) -> CanonicalPhysicalDrainManifestV1 {
        CanonicalPhysicalDrainManifestV1::callable(
            brand,
            ModuleInvocationFamilyV1::BindingSsaAcyclic,
            keys.iter()
                .map(|key| {
                    CanonicalPhysicalCallableRowV1::new(
                        key.clone(),
                        format!("{}/0", key.name()).into_boxed_str(),
                        0,
                        CanonicalInsertedDispositionV1::from_canonical_source(),
                    )
                })
                .collect(),
        )
    }

    #[test]
    fn callable_row_cardinality_rejects_missing_and_surplus_manifest_rows() {
        let brand = ModuleInvocationBrandV1::test_with_ordinal(93);
        let (physical, keys) = callable_fixture(brand);
        let missing = callable_manifest(brand, &keys[..1]);
        let rejected = physical.prepare_drain(missing).unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::Collector(
                CanonicalCollectorDrainErrorV1::ReceiptCountMismatch { .. }
            )
        ));

        let (physical, keys) = callable_fixture(ModuleInvocationBrandV1::test_with_ordinal(94));
        let mut surplus_keys = keys.clone();
        surplus_keys.push(CanonicalCallableKeyV1::free_static_for_test("extra", 0));
        let rejected = physical
            .prepare_drain(callable_manifest(
                ModuleInvocationBrandV1::test_with_ordinal(94),
                &surplus_keys,
            ))
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::Collector(
                CanonicalCollectorDrainErrorV1::ReceiptCountMismatch { .. }
            )
        ));

        let (physical, keys) = callable_fixture(ModuleInvocationBrandV1::test_with_ordinal(95));
        let duplicate_keys = vec![keys[0].clone(), keys[0].clone()];
        let rejected = physical
            .prepare_drain(callable_manifest(
                ModuleInvocationBrandV1::test_with_ordinal(95),
                &duplicate_keys,
            ))
            .unwrap_err();
        assert!(matches!(
            rejected.error(),
            CanonicalPhysicalDrainPrepareErrorV1::Collector(
                CanonicalCollectorDrainErrorV1::SurplusKey(_)
            )
        ));
    }
}
