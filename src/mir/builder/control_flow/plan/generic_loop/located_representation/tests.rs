use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    VerifiedCallableResultActivationPlanV1, VerifiedCallableResultActivationRowsV1,
    VerifiedCallableResultLegacySourceViewV1, VerifiedSameModuleCallableResultCatalogV1,
};
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;

use super::*;
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::product::{
    VerifiedLocatedGenericLoopBodyModeV1, VerifiedLocatedRecipeItemV1,
};
use crate::mir::builder::control_flow::plan::generic_loop::located_representation::recipe_seal::{
    require_contract, seal_recipe_block, RecipeSealDomainV1,
};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BodyId, IfContractKind, IfMode, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::{refs::StmtRef, RecipeBody};

const SOURCE: &str = r#"
box Counter {
    run(pos) {
        loop(pos < 5) {
            pos = pos + 1
        }
        return pos
    }
}
"#;

const STRICT_SOURCE: &str = r#"
box Counter {
    run(pos) {
        loop(pos < 5) {
            if pos == 2 {
                return pos
            }
            if pos == 3 {
                local seen = 1
            } else {
                local seen = 0
            }
            pos = pos + 1
        }
        return pos
    }
}
"#;

fn activation_plan(source: &str) -> VerifiedCallableResultActivationPlanV1 {
    let root = NyashParser::parse_from_string(source).expect("O0-R0 fixture parses");
    let declarations = Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
            .expect("O0-R0 declarations seal"),
    );
    let imports =
        VerifiedStaticImportAliasViewV1::seal(&declarations, Vec::new()).expect("imports seal");
    let targets =
        VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
            .expect("target catalog seals");
    let results = VerifiedSameModuleCallableResultCatalogV1::verify(&declarations, &targets)
        .expect("result catalog verifies");
    let rows = VerifiedCallableResultActivationRowsV1::verify(&declarations, &targets, &results)
        .expect("activation rows verify");
    drop(results);
    drop(targets);
    drop(imports);
    VerifiedCallableResultActivationPlanV1::seal(declarations, rows).expect("activation plan seals")
}

fn caller(plan: &VerifiedCallableResultActivationPlanV1) -> CanonicalSameModuleCallableKeyV1 {
    plan.declaration_catalog()
        .declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            "Counter",
            "run",
            1,
        )
        .expect("caller exists")
        .key()
        .clone()
}

fn located_loop<'plan>(
    plan: &'plan VerifiedCallableResultActivationPlanV1,
    caller: &CanonicalSameModuleCallableKeyV1,
) -> (
    LocatedLoopPlanExpressionPortV1<'plan>,
    crate::mir::callable_result_representation::LegacyStmtInputV1<'plan>,
) {
    let view = VerifiedCallableResultLegacySourceViewV1::verify(plan, caller).expect("source view");
    let root = view.root_body();
    let statement = view.body_stmt(&root, 0).expect("loop statement");
    (LocatedLoopPlanExpressionPortV1::new(view), statement)
}

#[test]
fn direct_recipe_only_seals_exact_prefix_and_cleanup() {
    crate::test_support::with_env_vars(
        &[
            ("NYASH_JOINIR_DEV", None),
            ("HAKO_JOINIR_PLANNER_REQUIRED", None),
            ("HAKO_JOINIR_STRICT", None),
            ("NYASH_JOINIR_STRICT", None),
        ],
        || {
            let plan = activation_plan(SOURCE);
            let caller = caller(&plan);
            let (port, root) = located_loop(&plan, &caller);
            let sealed =
                VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, root)
                    .expect("direct representation seals");
            assert!(matches!(
                sealed.mode,
                VerifiedLocatedGenericLoopBodyModeV1::DirectRecipeOnly {
                    ref prefix,
                    ..
                } if prefix.is_empty()
            ));
        },
    );
}

#[test]
fn strict_exit_allowed_seals_explicit_and_wrapped_if_items() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_vars(
        &[
            ("NYASH_JOINIR_DEV", Some("1")),
            ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
            ("HAKO_JOINIR_STRICT", Some("1")),
            ("NYASH_JOINIR_STRICT", Some("1")),
        ],
        || {
            let plan = activation_plan(STRICT_SOURCE);
            let caller = caller(&plan);
            let (port, root) = located_loop(&plan, &caller);
            let sealed =
                VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, root)
                    .expect("strict representation seals");
            let VerifiedLocatedGenericLoopBodyModeV1::ExitAllowedRecipe { root, .. } = sealed.mode
            else {
                panic!("strict mode must use ExitAllowed recipe")
            };
            assert!(matches!(
                root.items.first(),
                Some(VerifiedLocatedRecipeItemV1::ExplicitIfV2 { .. })
            ));
            assert!(matches!(
                root.items.get(1),
                Some(VerifiedLocatedRecipeItemV1::StmtWrappedJoinIf { .. })
            ));
        },
    );
}

#[test]
fn foreign_and_unlocated_roots_reject_before_extraction() {
    let plan = activation_plan(SOURCE);
    let caller_key = caller(&plan);
    let (port, root) = located_loop(&plan, &caller_key);

    let foreign_plan = activation_plan(SOURCE);
    let foreign_caller = caller(&foreign_plan);
    let foreign_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&foreign_plan, &foreign_caller)
            .expect("foreign view");
    let foreign_root = foreign_view
        .body_stmt(&foreign_view.root_body(), 0)
        .expect("foreign root");
    assert!(matches!(
        VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, foreign_root),
        Err(LocatedGenericLoopRepresentationErrorV1::Port(_))
    ));

    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller_key).expect("view");
    let unlocated_expr = view.unlocated_expr(root.node());
    let unlocated_body = view
        .child_body(&unlocated_expr, BodyChildRoleV1::LoopBody)
        .expect("unlocated loop body");
    let unlocated_root = view
        .body_stmt(&unlocated_body, 0)
        .expect("unlocated statement");
    assert!(matches!(
        VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, unlocated_root),
        Err(LocatedGenericLoopRepresentationErrorV1::Port(_))
    ));
}

#[test]
fn non_loop_root_rejects_without_route_fallback() {
    let plan = activation_plan(SOURCE);
    let caller = caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).expect("view");
    let body = view.root_body();
    let non_loop = view.body_stmt(&body, 1).expect("return statement");
    let port = LocatedLoopPlanExpressionPortV1::new(view);
    assert!(matches!(
        VerifiedLocatedGenericLoopBodyRepresentationV1::verify_located_loop(&port, non_loop),
        Err(LocatedGenericLoopRepresentationErrorV1::NotLoopRoot)
    ));
}

#[test]
fn recipe_body_id_and_cardinality_are_co_sealed() {
    let plan = activation_plan(SOURCE);
    let caller = caller(&plan);
    let (port, root) = located_loop(&plan, &caller);
    let body = port
        .exact_child_body_from_stmt(&root, BodyChildRoleV1::LoopBody)
        .expect("exact loop body");
    let missing = RecipeBlock::new(BodyId(99), vec![RecipeItem::Stmt(StmtRef::new(0))]);
    assert!(matches!(
        seal_recipe_block(
            &port,
            &RecipeBodies::new(),
            &missing,
            &body,
            1,
            RecipeSealDomainV1::ExitAllowed,
        ),
        Err(LocatedGenericLoopRepresentationErrorV1::MissingRecipeBody)
    ));

    let mut arena = RecipeBodies::new();
    let body_id = arena.register(RecipeBody::new(Vec::new()));
    let wrong_len = RecipeBlock::new(body_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);
    assert!(matches!(
        seal_recipe_block(
            &port,
            &arena,
            &wrong_len,
            &body,
            1,
            RecipeSealDomainV1::ExitAllowed,
        ),
        Err(
            LocatedGenericLoopRepresentationErrorV1::RecipeBodyLengthMismatch {
                exact: 1,
                recipe: 0,
            }
        )
    ));
}

#[test]
fn recipe_contract_domains_do_not_overlap() {
    let exit_only = IfContractKind::ExitOnly {
        mode: IfMode::ExitIf,
    };
    assert!(require_contract(RecipeSealDomainV1::ExitAllowed, &exit_only).is_ok());
    assert!(require_contract(RecipeSealDomainV1::NoExit, &IfContractKind::Join).is_ok());
    assert!(matches!(
        require_contract(RecipeSealDomainV1::ExitAllowed, &IfContractKind::Join),
        Err(LocatedGenericLoopRepresentationErrorV1::RecipeContractMismatch)
    ));
    assert!(matches!(
        require_contract(RecipeSealDomainV1::NoExit, &exit_only),
        Err(LocatedGenericLoopRepresentationErrorV1::RecipeContractMismatch)
    ));
}
