//! Pre-wrapper Main0 root selection dispatch.
//!
//! The installed App Main root is classified against each bounded Main0
//! source profile in a fixed order before the legacy wrapper opens. At most
//! one profile may admit a given source: a profile that declines leaves the
//! one-shot loan unconsumed for the next arm, and a full decline returns
//! `None` so the legacy wrapper path keeps its loan.

use crate::mir::compiler::main0_continue_recipe_coseal::VerifiedMain0ContinueRecipeProductV1;
use crate::mir::compiler::main0_in_body_step_recipe_coseal::VerifiedMain0InBodyStepRecipeProductV1;
use crate::mir::normal_callable_semantic_package::NormalCallableSemanticPackagePortV1;

use super::super::main_expansion::VerifiedRawRootExpansionV1;
use super::super::MirBuilder;

/// One selected Main0 profile product transported from pre-wrapper
/// selection to the canonical draft handoff. Exactly one arm is carried;
/// a declined observation never produces a product.
#[derive(Debug)]
pub(in crate::mir::builder) enum SelectedMain0RootProductV1 {
    Continue(VerifiedMain0ContinueRecipeProductV1),
    InBodyStep(VerifiedMain0InBodyStepRecipeProductV1),
}

impl MirBuilder {
    /// Classify the source-backed App Main root against the bounded Main0
    /// profiles before the legacy wrapper opens. The observation borrows
    /// the same installed input without consuming the one-shot loan; a full
    /// decline returns `None` so the legacy route keeps its loan.
    pub(super) fn select_main0_root_product_v1(
        &mut self,
        package_port: &NormalCallableSemanticPackagePortV1<'_>,
        expansion: &VerifiedRawRootExpansionV1<'_>,
    ) -> Result<Option<SelectedMain0RootProductV1>, String> {
        if let Some(product) =
            self.select_main0_continue_root_product_v1(package_port, expansion)?
        {
            return Ok(Some(SelectedMain0RootProductV1::Continue(product)));
        }
        if let Some(product) =
            self.select_main0_in_body_step_root_product_v1(package_port, expansion)?
        {
            return Ok(Some(SelectedMain0RootProductV1::InBodyStep(product)));
        }
        Ok(None)
    }
}
