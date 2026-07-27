//! Focused fixtures for the stack-local Stage-B function ingress.

use std::sync::Arc;

use crate::mir::builder::preloop_stageb_context_install::PreparedPreloopStageBAliasInstallV1;
use crate::mir::builder::{MirBuilder, VerifiedSameModuleCallableDeclarationCatalogV1};
use crate::mir::resolved_semantics::SourcePathSegmentV1;
use crate::mir::source_call_target::{
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use crate::parser::NyashParser;

use super::{
    inventory_preloop_stageb_candidates_v1, seal_preloop_stageb_candidate_selection_v1,
    VerifiedPreloopStageBCandidateSelectionV1,
};

const SOURCE: &str = r#"
static box Carrier {
  keep(left, right) { return right }
}
box Caller {
  inner(value) { return 1 }
  run(text, pos) { pos = Carrier.keep(text, me.inner(pos)) }
}
"#;

fn selected_ingress() -> (
    Arc<VerifiedSameModuleCallableDeclarationCatalogV1>,
    super::PreparedPreloopStageBFunctionIngressV1,
) {
    let ast = NyashParser::parse_from_string(SOURCE).expect("Stage-B ingress source");
    let catalog = Arc::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&ast)
            .expect("Stage-B ingress catalog"),
    );
    let aliases = VerifiedStaticImportAliasViewV1::seal(catalog.as_ref(), std::iter::empty())
        .expect("empty aliases");
    let calls = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(catalog.as_ref(), &aliases)
        .expect("whole-source calls");
    let inventory =
        inventory_preloop_stageb_candidates_v1(&calls).expect("exact candidate inventory");
    let selection = seal_preloop_stageb_candidate_selection_v1(Arc::clone(&catalog), inventory)
        .expect("candidate selection");
    let VerifiedPreloopStageBCandidateSelectionV1::One(selected) = selection else {
        panic!("fixture must select exactly one Stage-B candidate");
    };
    let prepared = selected
        .into_activation()
        .into_module_install_parts_v1()
        .attach_aliases(PreparedPreloopStageBAliasInstallV1::None);
    let mut builder = MirBuilder::new();
    let installed = prepared
        .commit(&mut builder)
        .expect("candidate context install");
    let ingress = installed.into_ledger_parts().prepare_function_ingress();
    (catalog, ingress)
}

#[test]
fn selected_activation_reconstructs_one_stack_local_located_argument() {
    let (catalog, ingress) = selected_ingress();
    let caller = ingress.recipe().caller().clone();
    let outer = ingress.recipe().outer_call_site().clone();
    let inner = ingress.recipe().inner_call_site().clone();
    let target = ingress.recipe().outer_target().clone();
    let selected_index = ingress.recipe().selected_argument_index();

    let observed = ingress
        .with_prepared_located_argument(|located, recipe| {
            assert!(std::ptr::eq(
                located.selected().parent().view().catalog(),
                catalog.as_ref()
            ));
            assert_eq!(located.selected().index(), selected_index);
            assert_eq!(located.selected().parent().site(), &outer);
            assert_eq!(located.selected().child().site(), &inner);
            assert_eq!(recipe.caller(), &caller);
            assert_eq!(recipe.outer_target(), &target);
            assert!(recipe.result().is_integer());
            assert_eq!(
                recipe.body_handoff().selected_statement().node().segments(),
                &[SourcePathSegmentV1::Body(0)]
            );
            located.discard();
            true
        })
        .expect("stack-local located ingress");
    assert!(observed);
}
