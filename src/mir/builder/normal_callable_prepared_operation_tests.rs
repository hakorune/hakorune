use crate::mir::builder::callable_declaration_catalog::{
    SelectedNormalCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::builder::normal_callable_semantic_source::{
    NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1,
};
use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_from_ledger_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::NyashParser;

fn loop_program() -> crate::ast::ASTNode {
    NyashParser::parse_from_string(
        r#"
            static box StringHelpers {
                int_to_str(n) {
                    local value = me.to_i64(n)
                    local i = 0
                    loop(i < 1) { i = i + 1 }
                    return value
                }
                to_i64(x) { return x + 1 }
            }
        "#,
    )
    .expect("callable loop source")
}

fn loop_source<'a>(
    program: &'a crate::ast::ASTNode,
) -> (
    VerifiedNormalCallableSemanticSourceV1<'a>,
    SelectedNormalCallableKeyV1,
) {
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(program)
        .expect("callable catalog");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("callable semantic source deferred")
    };
    let key = source
        .keys()
        .find(|key| {
            matches!(
                key,
                SelectedNormalCallableKeyV1::Cataloged(key)
                    if key.owner() == "StringHelpers" && key.name() == "int_to_str"
            )
        })
        .expect("loop callable key")
        .clone();
    (source, key)
}

fn logical_product(
    receipt: &super::super::normal_callable_semantic_source::
        VerifiedNormalCallableSourceIngressReceiptV1<'_>,
) -> crate::mir::compiler::callable_single_loop_recipe_coseal::
VerifiedCallableSingleLoopRecipeProductV1{
    let syntax =
        issue_callable_single_loop_syntax_facts_from_ledger_v1(receipt.input(), receipt.ledger())
            .expect("syntax facts");
    let map =
        issue_callable_single_loop_source_map_v1(receipt.ledger(), syntax).expect("source map");
    issue_callable_single_loop_recipe_v1(receipt.ledger(), map).expect("logical product")
}

#[test]
fn prepared_ingress_runs_one_shot_callable_full_demand_preflight() {
    let program = loop_program();
    let (source, key) = loop_source(&program);
    let receipt = source.loan(&key).unwrap().into_source_ingress();
    let logical = logical_product(&receipt);
    let prepared = source
        .loan(&key)
        .unwrap()
        .prepare_loop_ingress(logical)
        .expect("prepared ingress");

    let prepared = prepared
        .prepare_full_demand()
        .expect("full callable demand");
    let (source, input, operation, prelude, tail) = prepared.into_parts();

    assert_eq!(operation.schedule().len(), 7);
    assert_eq!(operation.coverage().operation_count(), 7);
    assert_eq!(source.owner(), operation.demand().context().owner());
    assert_eq!(input.source_binding().owner(), source.owner());
    assert_eq!(prelude.owner(), source.owner());
    assert_eq!(tail.owner(), source.owner());
}
