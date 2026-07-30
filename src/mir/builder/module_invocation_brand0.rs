//! CUT0-I0-ROOT0-BRAND0: the first real invocation owner.
//!
//! This module is deliberately disconnected from compiler ingress.  It is the
//! one constructor that creates the actual Builder session and the actual
//! shell/collector payloads from one source-sealed token.

use crate::mir::MirModule;

use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_draft_collector::{
    CallableCollectorBatchPrepareErrorV1, CollectedCallableCollectorBatchV1,
    CollectedDraftAdmissionProductErrorV1, CollectedDraftAdmissionProductV1,
    CollectedDraftAdmissionReceiptV1, FunctionDraftKeyV1, ModuleDraftCollectorV1,
};
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1, InvocationBranded};
use super::module_invocation_session::{
    BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use super::module_lowering_shell::{
    AcyclicCapabilityAbsenceWitnessV1, RecursiveCapabilityInstallReceiptV1,
};
use super::module_lowering_shell::{ModuleLoweringShellErrorV1, ModuleLoweringShellV1};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use super::MirBuilder;
use crate::mir::builder::resolved_lowering::VerifiedUnpublishedCallableDraftSetV1;
use crate::mir::compiler::capability::VerifiedResolvedOwnerHeaderV1;

#[derive(Debug)]
pub(in crate::mir) struct CollectedCanonicalSinglePhysicalV1 {
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collected: CollectedDraftAdmissionProductV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CollectedCanonicalCallablePhysicalV1 {
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collected: CollectedCallableCollectorBatchV1,
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalCallableCapabilityWitnessV1 {
    Acyclic(AcyclicCapabilityAbsenceWitnessV1),
    Recursive(RecursiveCapabilityInstallReceiptV1),
}

impl CanonicalCallableCapabilityWitnessV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        match self {
            Self::Acyclic(witness) => witness.brand(),
            Self::Recursive(receipt) => receipt.brand(),
        }
    }

    pub(in crate::mir) fn family(
        &self,
    ) -> super::module_invocation_identity::ModuleInvocationFamilyV1 {
        match self {
            Self::Acyclic(witness) => witness.family(),
            Self::Recursive(receipt) => receipt.family(),
        }
    }
}

#[derive(Debug)]
pub(in crate::mir) enum CanonicalPhysicalCollectionErrorV1 {
    Single(CollectedDraftAdmissionProductErrorV1),
    Batch(CallableCollectorBatchPrepareErrorV1),
    CollectorUnbranded,
    CapabilityMissing,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedCanonicalPhysicalCollectionV1 {
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
    error: CanonicalPhysicalCollectionErrorV1,
}

impl CollectedCanonicalSinglePhysicalV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.shell.brand()
    }

    pub(in crate::mir) fn receipt_brand(&self) -> ModuleInvocationBrandV1 {
        self.collected.receipt_brand()
    }

    /// The only sibling-terminal unpack authority for the collected physical
    /// owner.  The shell, collector, and receipt remain one consuming move.
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        BrandedShellV1<ModuleLoweringShellV1>,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        InvocationBranded<CollectedDraftAdmissionReceiptV1>,
    ) {
        let (collector, receipt) = self.collected.into_parts();
        (self.shell, collector, receipt)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(
        shell: BrandedShellV1<ModuleLoweringShellV1>,
        collected: CollectedDraftAdmissionProductV1,
    ) -> Self {
        Self { shell, collected }
    }

    #[cfg(test)]
    pub(in crate::mir) fn publish_probe_for_test(&mut self) {
        self.shell
            .payload_mut()
            .publish_function_for_test(crate::mir::MirFunction::new(
                crate::mir::FunctionSignature {
                    name: "published/0".to_owned(),
                    params: Vec::new(),
                    return_type: crate::mir::MirType::Integer,
                    effects: crate::mir::EffectMask::PURE,
                },
                crate::mir::BasicBlockId::new(0),
            ));
    }
}

impl CollectedCanonicalCallablePhysicalV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.shell.brand()
    }

    pub(in crate::mir) fn receipt_brand(&self) -> ModuleInvocationBrandV1 {
        self.collected.receipt_brand()
    }

    /// The only sibling-terminal unpack authority for the collected physical
    /// owner.  The shell, collector, and batch receipt move together.
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        BrandedShellV1<ModuleLoweringShellV1>,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
        InvocationBranded<super::module_draft_collector::CallableCollectorBatchReceiptV1>,
    ) {
        let (collector, receipt) = self.collected.into_parts();
        (self.shell, collector, receipt)
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(
        shell: BrandedShellV1<ModuleLoweringShellV1>,
        collected: CollectedCallableCollectorBatchV1,
    ) -> Self {
        Self { shell, collected }
    }
}

impl RejectedCanonicalPhysicalCollectionV1 {
    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.shell.brand()
    }

    pub(in crate::mir) fn error(&self) -> &CanonicalPhysicalCollectionErrorV1 {
        &self.error
    }
}

#[derive(Debug)]
pub(in crate::mir) struct InvocationPhysicalStateV1 {
    brand: ModuleInvocationBrandV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
}

impl InvocationPhysicalStateV1 {
    pub(in crate::mir) fn from_token(
        token: &ModuleInvocationTokenV1,
        module_name: String,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let brand = token.brand();
        let shell = ModuleLoweringShellV1::from_empty_module(MirModule::new(module_name))?;
        Ok(Self {
            brand,
            shell: InvocationBranded::from_source(brand, shell),
            collector: InvocationBranded::from_source(
                brand,
                ModuleDraftCollectorV1::with_brand(brand),
            ),
        })
    }

    pub(in crate::mir) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) fn install_callable_capability(
        &mut self,
        family: super::module_invocation_identity::ModuleInvocationFamilyV1,
    ) -> Result<CanonicalCallableCapabilityWitnessV1, &'static str> {
        let installed = self.shell.install_callable_batch_capability(family)?;
        match (family, installed) {
            (
                super::module_invocation_identity::ModuleInvocationFamilyV1::BindingSsaAcyclic,
                Err(witness),
            ) => Ok(CanonicalCallableCapabilityWitnessV1::Acyclic(witness)),
            (
                super::module_invocation_identity::ModuleInvocationFamilyV1::BindingSsaRecursive,
                Ok(receipt),
            ) => Ok(CanonicalCallableCapabilityWitnessV1::Recursive(receipt)),
            _ => Err("[freeze:contract][canonical_bridge/capability_family]"),
        }
    }

    pub(in crate::mir::builder) fn shell(&self) -> &BrandedShellV1<ModuleLoweringShellV1> {
        &self.shell
    }

    pub(in crate::mir::builder) fn environment_lanes_are_vacant(&self) -> bool {
        self.shell.payload().environment_lanes_are_vacant()
    }

    pub(in crate::mir::builder) fn install_environment_preflighted(
        self,
        facts: SealedModuleDeclarationFactsV1,
        source_file: Option<Box<str>>,
    ) -> Self {
        let Self {
            brand,
            shell,
            collector,
        } = self;
        let shell = shell.into_payload();
        let prepared = shell.prepare_declaration_fact_commit_preflighted(facts);
        let shell = prepared.commit_with_source_file(source_file);
        Self {
            brand,
            shell: InvocationBranded::from_source(brand, shell),
            collector,
        }
    }

    pub(in crate::mir::builder) fn collector(&self) -> &BrandedCollectorV1<ModuleDraftCollectorV1> {
        &self.collector
    }

    pub(in crate::mir::builder) fn complete_raw_static_child(
        &mut self,
        builder: &mut MirBuilder,
        work: super::PreparedRawRootStaticChildDraftV1,
    ) -> Result<InvocationBranded<CollectedDraftAdmissionReceiptV1>, ModuleLoweringPortChildErrorV1>
    {
        let mut port = ModuleLoweringPortV1::from_collector(self.collector.payload_mut());
        let mut child = RawInvocationChildPortV1::new(&mut port);
        child.complete_raw_root_static_child_branded(builder, work)
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleInvocationBrandV1,
        BrandedShellV1<ModuleLoweringShellV1>,
        BrandedCollectorV1<ModuleDraftCollectorV1>,
    ) {
        (self.brand, self.shell, self.collector)
    }

    pub(in crate::mir) fn collect_single(
        self,
        header: &VerifiedResolvedOwnerHeaderV1,
        draft: crate::mir::MirFunction,
    ) -> Result<CollectedCanonicalSinglePhysicalV1, RejectedCanonicalPhysicalCollectionV1> {
        let (brand, shell, collector) = self.into_parts();
        let key = FunctionDraftKeyV1::CanonicalResolvedOwner(header.owner());
        let symbol = header.symbol().as_mir_name().to_owned();
        match collector.collect_canonical_single(key, symbol, header.arity(), draft) {
            Ok(collected) => Ok(CollectedCanonicalSinglePhysicalV1 { shell, collected }),
            Err(rejected) => {
                let (collector, error) = rejected.into_parts();
                Err(RejectedCanonicalPhysicalCollectionV1 {
                    shell,
                    collector,
                    error: CanonicalPhysicalCollectionErrorV1::Single(error),
                })
            }
        }
    }

    pub(in crate::mir) fn collect_callable_batch(
        self,
        drafts: VerifiedUnpublishedCallableDraftSetV1<'_>,
    ) -> Result<CollectedCanonicalCallablePhysicalV1, RejectedCanonicalPhysicalCollectionV1> {
        let (brand, shell, collector) = self.into_parts();
        let entries = drafts.into_canonical_entries();
        let raw_collector = collector.into_payload();
        if raw_collector.receipt_brand() != Some(brand) {
            return Err(RejectedCanonicalPhysicalCollectionV1 {
                shell,
                collector: InvocationBranded::from_source(brand, raw_collector),
                error: CanonicalPhysicalCollectionErrorV1::CollectorUnbranded,
            });
        }
        let prepared = match raw_collector.prepare_callable_batch(entries) {
            Ok(prepared) => prepared,
            Err(rejected) => {
                let (collector, error) = rejected.into_parts();
                return Err(RejectedCanonicalPhysicalCollectionV1 {
                    shell,
                    collector: InvocationBranded::from_source(brand, collector),
                    error: CanonicalPhysicalCollectionErrorV1::Batch(error),
                });
            }
        };
        let collected = prepared
            .collect_all_branded()
            .expect("collector brand was preflighted before callable collection");
        Ok(CollectedCanonicalCallablePhysicalV1 { shell, collected })
    }

    pub(in crate::mir) fn reject_capability_missing(self) -> RejectedCanonicalPhysicalCollectionV1 {
        let (brand, shell, collector) = self.into_parts();
        RejectedCanonicalPhysicalCollectionV1 {
            shell,
            collector,
            error: CanonicalPhysicalCollectionErrorV1::CapabilityMissing,
        }
    }
}

impl InvocationBranded<ModuleDraftCollectorV1> {
    /// Issue a receipt only through the branded physical collector owner.
    /// Callers cannot attach a brand to a receipt without holding this owner.
    pub(in crate::mir::builder) fn issue_collected_receipt(
        &self,
        receipt: CollectedDraftAdmissionReceiptV1,
    ) -> InvocationBranded<CollectedDraftAdmissionReceiptV1> {
        InvocationBranded::from_source(self.brand(), receipt)
    }
}

pub(in crate::mir::builder) struct ActiveModuleInvocationV1 {
    token: ModuleInvocationTokenV1,
    session: ModuleBuilderInvocationSessionV1,
    physical: InvocationPhysicalStateV1,
}

impl ActiveModuleInvocationV1 {
    pub(in crate::mir::builder) fn open(
        token: ModuleInvocationTokenV1,
        current: &MirBuilder,
        config: BuilderInvocationConfigV1,
        module_name: String,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let session = ModuleBuilderInvocationSessionV1::open_for_token(&token, current, config);
        let physical = InvocationPhysicalStateV1::from_token(&token, module_name)?;
        debug_assert_eq!(session.brand(), token.brand());
        debug_assert_eq!(session.family(), token.family());
        debug_assert_eq!(physical.brand(), token.brand());
        Ok(Self {
            token,
            session,
            physical,
        })
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir::builder) fn session(&self) -> &ModuleBuilderInvocationSessionV1 {
        &self.session
    }

    pub(in crate::mir::builder) fn physical(&self) -> &InvocationPhysicalStateV1 {
        &self.physical
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        ModuleInvocationTokenV1,
        ModuleBuilderInvocationSessionV1,
        InvocationPhysicalStateV1,
    ) {
        (self.token, self.session, self.physical)
    }
}
