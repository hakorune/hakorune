//! CUT0-I0-ROOT0-BRAND0: the first real invocation owner.
//!
//! This module is deliberately disconnected from compiler ingress.  It is the
//! one constructor that creates the actual Builder session and the actual
//! shell/collector payloads from one source-sealed token.

use crate::mir::MirModule;

use super::module_draft_collector::ModuleDraftCollectorV1;
use super::module_draft_collector::CollectedDraftAdmissionReceiptV1;
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationTokenV1};
use super::module_invocation_owner_chain::{BrandedCollectorV1, BrandedShellV1, InvocationBranded};
use super::module_invocation_session::{
    BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
};
use super::module_lowering_shell::{ModuleLoweringShellErrorV1, ModuleLoweringShellV1};
use super::MirBuilder;

#[derive(Debug)]
pub(in crate::mir::builder) struct InvocationPhysicalStateV1 {
    brand: ModuleInvocationBrandV1,
    shell: BrandedShellV1<ModuleLoweringShellV1>,
    collector: BrandedCollectorV1<ModuleDraftCollectorV1>,
}

impl InvocationPhysicalStateV1 {
    fn from_token(
        token: &ModuleInvocationTokenV1,
        module_name: String,
    ) -> Result<Self, ModuleLoweringShellErrorV1> {
        let brand = token.brand();
        let shell = ModuleLoweringShellV1::from_empty_module(MirModule::new(module_name))?;
        Ok(Self {
            brand,
            shell: InvocationBranded::from_source(brand, shell),
            collector: InvocationBranded::from_source(brand, ModuleDraftCollectorV1::default()),
        })
    }

    pub(in crate::mir::builder) fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) fn shell(&self) -> &BrandedShellV1<ModuleLoweringShellV1> {
        &self.shell
    }

    pub(in crate::mir::builder) fn collector(
        &self,
    ) -> &BrandedCollectorV1<ModuleDraftCollectorV1> {
        &self.collector
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
