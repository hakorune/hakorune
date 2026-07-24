//! PUBLICATION0 target quiescence and assignment receipt.

use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use super::module_invocation_session::BuilderCommitReadinessErrorV1;
use super::MirBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct BuilderPublicationReceiptV1 {
    pub(in crate::mir::builder) brand: ModuleInvocationBrandV1,
    pub(in crate::mir::builder) family: ModuleInvocationFamilyV1,
    pub(in crate::mir::builder) _seal: BuilderPublicationReceiptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct BuilderPublicationReceiptSealV1;

impl BuilderPublicationReceiptV1 {
    pub(in crate::mir) const fn brand(self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn family(self) -> ModuleInvocationFamilyV1 {
        self.family
    }
}

pub(in crate::mir) fn check_builder_external_commit_quiescence(
    builder: &MirBuilder,
) -> Result<(), BuilderCommitReadinessErrorV1> {
    if builder.current_module.is_some() {
        return Err(BuilderCommitReadinessErrorV1::CurrentModuleOpen);
    }
    if builder.function_state.current_function.is_some() {
        return Err(BuilderCommitReadinessErrorV1::CurrentFunctionOpen);
    }
    if builder.function_state.current_block.is_some() {
        return Err(BuilderCommitReadinessErrorV1::CurrentBlockOpen);
    }
    if !builder.function_state.is_closed_for_external_commit() {
        return Err(BuilderCommitReadinessErrorV1::FunctionStateOpen);
    }
    if builder.comp_ctx.current_slot_registry.is_some() {
        return Err(BuilderCommitReadinessErrorV1::SlotRegistryOpen);
    }
    if builder.comp_ctx.compilation_context.is_some() {
        return Err(BuilderCommitReadinessErrorV1::CompilationContextOpen);
    }
    if builder.recursion_depth != 0 {
        return Err(BuilderCommitReadinessErrorV1::RecursionDepthOpen);
    }
    Ok(())
}
