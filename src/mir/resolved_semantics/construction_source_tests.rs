use super::*;
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};
use crate::parser::NyashParser;

fn parsed_function(source: &str) -> ASTNode {
    let ASTNode::Program { statements, .. } =
        NyashParser::parse_from_string(source).expect("natural source parses")
    else {
        panic!("program expected")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("function")
}

#[test]
fn natural_new_rows_keep_initializer_and_nested_argument_sites() {
    let tree = parsed_function("function build() {\n local arr = new ArrayBox()\n local item = new UserBox(new OtherBox(), 7)\n return arr\n}");
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let ledger = forest.callable_source_ledger(forest.roots()[0]).unwrap();
    let initializers = ledger.initializer_relations().collect::<Vec<_>>();
    assert_eq!(initializers.len(), 2);
    let arr = ledger
        .construction_source(initializers[0].initializer_site().unwrap())
        .unwrap();
    assert_eq!(arr.class(), "ArrayBox");
    assert!(arr.arguments().is_empty());
    assert!(arr.field_initializers().is_empty());
    assert_eq!(arr.site(), initializers[0].initializer_site().unwrap());
    assert!(ledger.binding(initializers[0].binding()).is_some());
    let item = ledger
        .construction_source(initializers[1].initializer_site().unwrap())
        .unwrap();
    assert_eq!(item.class(), "UserBox");
    assert_eq!(item.arguments().len(), 2);
    assert_eq!(
        ledger
            .construction_source(&item.arguments()[0])
            .unwrap()
            .class(),
        "OtherBox"
    );
    assert_eq!(
        ledger.literal_source(&item.arguments()[1]),
        Some(&ResolvedLiteralSourceV1::Integer(7))
    );
    for site in item.arguments() {
        assert!(ledger.source_site_inventory().contains_expression(site));
    }
    assert!(ledger.construction_source(&item.arguments()[1]).is_none());
    // User names remain observations, with exactly the same shape as builtin names.
    // This test intentionally claims no builtin classification or physical admission.
}

#[test]
fn new_field_children_remain_ordered_and_resolved() {
    let mut tree = parsed_function("function build() { local item = new UserBox() return item }");
    let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
        panic!()
    };
    let ASTNode::Local { initial_values, .. } = &mut body[0] else {
        panic!()
    };
    let ASTNode::New {
        field_initializers, ..
    } = initial_values[0].as_deref_mut().unwrap()
    else {
        panic!()
    };
    for (name, number) in [("right", 9), ("left", 4)] {
        field_initializers.push((
            name.into(),
            ASTNode::Literal {
                value: LiteralValue::Integer(number),
                span: crate::ast::Span::unknown(),
            },
        ));
    }
    let mut session = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let forest = session
        .resolve_forest(FunctionSyntaxViewV1::from_ast(&tree).unwrap())
        .unwrap();
    let ledger = forest.callable_source_ledger(forest.roots()[0]).unwrap();
    let initializer = ledger.initializer_relations().next().unwrap();
    let row = ledger
        .construction_source(initializer.initializer_site().unwrap())
        .unwrap();
    assert_eq!(row.field_initializers().len(), 2);
    for ((name, site), (expected_name, value)) in row
        .field_initializers()
        .iter()
        .zip([("right", 9), ("left", 4)])
    {
        assert_eq!(name.as_ref(), expected_name);
        assert!(ledger.source_site_inventory().contains_expression(site));
        assert_eq!(
            ledger.literal_source(site),
            Some(&ResolvedLiteralSourceV1::Integer(value))
        );
    }
}

#[test]
fn duplicate_construction_site_rejects_at_seal() {
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
    ]));
    let row = ResolvedNewExpressionSourceV1 {
        site,
        class: "ArrayBox".into(),
        arguments: Box::new([]),
        field_initializers: Box::new([]),
    };
    let mut draft = ShadowExpressionSourceDraftV1::default();
    draft.constructions.extend([row.clone(), row]);
    assert_eq!(
        seal_shadow_expression_source_v1(draft, |_| unreachable!()).unwrap_err(),
        "duplicate construction expression source relation"
    );
}
