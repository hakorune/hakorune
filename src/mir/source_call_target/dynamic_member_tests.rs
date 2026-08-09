use crate::mir::builder::{
    NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::NyashParser;

use super::{
    DynamicMemberSourceIssueV1, DynamicMemberSourceRejectV1, VerifiedSourceCallTargetCatalogV1,
    VerifiedSourceCallTargetV1, VerifiedStaticImportAliasViewV1,
};

fn seal_source<'source>(
    program: &'source crate::ast::ASTNode,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedNormalCallableSemanticSourceV1<'source> {
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
        panic!("full parser scan source must not be narrowed to Deferred")
    };
    source
}

fn empty_targets<'catalog>(
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceCallTargetCatalogV1<'catalog> {
    let imports = VerifiedStaticImportAliasViewV1::seal(catalog, std::iter::empty()).unwrap();
    VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty()).unwrap()
}

#[test]
fn full_parser_scan_method_resolution_census() {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let crate::ast::ASTNode::Program { statements, .. } = program else {
        unreachable!()
    };
    let mut failures = Vec::new();
    for statement in statements {
        if let crate::ast::ASTNode::BoxDeclaration { methods, .. } = statement {
            for entry in methods.iter_selected_declaration_order() {
                if let Err(error) =
                    crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(
                        entry.declaration().clone(),
                    )
                {
                    failures.push((entry.name().to_owned(), format!("{error:?}")));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn unchanged_full_parser_scan_source_issues_exact_dynamic_member_rows() {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let source = seal_source(&program, &catalog);
    let targets = empty_targets(&catalog)
        .extend_complete_dynamic_sources(&source)
        .expect("the compiler must accept the unchanged full method bodies");

    let caller = catalog
        .declaration_for(
            crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod,
            "ParserScanLoopBox",
            "skip_while",
            4,
        )
        .unwrap()
        .key();
    let rows = targets
        .all_rows()
        .filter_map(|((row_caller, _), target)| (row_caller == caller).then_some(target))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);
    let mut dispatches = rows
        .into_iter()
        .map(|target| match target {
            VerifiedSourceCallTargetV1::DynamicMember(target) => (
                target.dispatch().selector().to_owned(),
                target.dispatch().arity(),
                target.arguments().len(),
                target.call_site().clone(),
                target.result_site().clone(),
            ),
            VerifiedSourceCallTargetV1::Static(_) => panic!("unexpected static projection"),
        })
        .collect::<Vec<_>>();
    dispatches.sort_by(|left, right| left.0.cmp(&right.0));
    assert_eq!(dispatches[0].0, "indexOf");
    assert_eq!((dispatches[0].1, dispatches[0].2), (1, 1));
    assert_eq!(dispatches[1].0, "substring");
    assert_eq!((dispatches[1].1, dispatches[1].2), (2, 2));
    assert!(dispatches.iter().all(|row| row.3 == row.4));
    assert_eq!(
        targets.rows().count(),
        0,
        "static projection stays disjoint"
    );
}

#[test]
fn equal_looking_foreign_catalog_is_rejected_before_target_issue() {
    let text = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let program = NyashParser::parse_from_string(text).unwrap();
    let foreign_program = NyashParser::parse_from_string(text).unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let foreign_catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&foreign_program).unwrap();
    let source = seal_source(&program, &catalog);

    assert!(matches!(
        empty_targets(&foreign_catalog).extend_complete_dynamic_sources(&source),
        Err(DynamicMemberSourceIssueV1::Rejected(
            DynamicMemberSourceRejectV1::CatalogCallableOwnerLink(_)
        ))
    ));
}

#[test]
fn typed_non_dynamic_receiver_is_a_valid_unselected_row() {
    let program = NyashParser::parse_from_string(
        "static box TextUse { scan(src: String) { return src.length() } }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let source = seal_source(&program, &catalog);
    let targets = empty_targets(&catalog)
        .extend_complete_dynamic_sources(&source)
        .expect("a fully observed non-Dynamic receiver must not reject the batch");
    assert_eq!(targets.len(), 0);
}

#[test]
fn a_second_dynamic_arm_for_the_same_exact_site_is_rejected() {
    let program = NyashParser::parse_from_string(
        "static box Calls { run(src) { return src.substring(0, 1) } }",
    )
    .unwrap();
    let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let source = seal_source(&program, &catalog);
    let targets = empty_targets(&catalog)
        .extend_complete_dynamic_sources(&source)
        .unwrap();
    assert!(matches!(
        targets.extend_complete_dynamic_sources(&source),
        Err(DynamicMemberSourceIssueV1::Rejected(
            DynamicMemberSourceRejectV1::DuplicateOrCollidingTarget { .. }
        ))
    ));
}
