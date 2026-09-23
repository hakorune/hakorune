//! Pre-wrapper Main0 derived-predicate route classification.
//!
//! Third bounded arm beside `main0_continue_route` and
//! `main0_in_body_step_route`: the observation borrows the same installed
//! input without consuming the one-shot loan; a decline returns `None` so
//! the legacy route keeps its loan, while a selection transports the
//! verified recipe product to the canonical draft handoff.

use crate::mir::compiler::main0_derived_predicate_recipe_coseal::VerifiedMain0DerivedPredicateRecipeProductV1;
use crate::mir::compiler::main0_derived_predicate_root_selection::{
    select_main0_derived_predicate_source_v1, Main0DerivedPredicateSourceSelectionV1,
};
use crate::mir::normal_callable_semantic_package::NormalCallableSemanticPackagePortV1;

use super::super::main_expansion::{VerifiedMainStaticChildV1, VerifiedRawRootExpansionV1};
use super::super::MirBuilder;

impl MirBuilder {
    /// Classify the source-backed App Main root for the Main0
    /// derived-predicate profile before the legacy wrapper opens. The
    /// observation borrows the same installed input without consuming the
    /// one-shot loan; a decline returns `None` so the legacy route keeps
    /// its loan.
    pub(super) fn select_main0_derived_predicate_root_product_v1(
        &mut self,
        package_port: &NormalCallableSemanticPackagePortV1<'_>,
        expansion: &VerifiedRawRootExpansionV1<'_>,
    ) -> Result<Option<VerifiedMain0DerivedPredicateRecipeProductV1>, String> {
        let VerifiedRawRootExpansionV1::App(main) = expansion else {
            return Ok(None);
        };
        let Some(identity) = main
            .callable_main_compat()
            .and_then(VerifiedMainStaticChildV1::parser_identity)
        else {
            return Ok(None);
        };
        let catalog_key = self
            .comp_ctx
            .callable_declaration_catalog()
            .map_err(|error| error.to_string())?
            .source_backed_app_main()
            .ok_or_else(|| {
                "[freeze:contract][mir/main0-selection/app-main-missing]".to_owned()
            })?
            .catalog_key()
            .clone();
        match select_main0_derived_predicate_source_v1(package_port, &catalog_key, identity) {
            Ok(Main0DerivedPredicateSourceSelectionV1::Selected(product)) => Ok(Some(product)),
            Ok(Main0DerivedPredicateSourceSelectionV1::Unselected) => Ok(None),
            Err(reject) => Err(format!(
                "[freeze:contract][mir/main0-selection] {reject:?}"
            )),
        }
    }
}
