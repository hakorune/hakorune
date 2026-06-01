use nyash_rust::ast::{ASTNode, BuildPredicate};
use nyash_rust::parser::NyashParser;

#[test]
fn parses_top_level_build_test_when() {
    let ast = NyashParser::parse_from_string(
        r#"
when Build.test {
    import "HakoTest"
    function testOnly() {
        return 1
    }
} else {
    function releaseOnly() {
        return 2
    }
}
"#,
    )
    .expect("build cfg when should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    assert_eq!(statements.len(), 1);

    let ASTNode::BuildWhen {
        predicate,
        then_items,
        else_items,
        ..
    } = &statements[0]
    else {
        panic!("expected BuildWhen");
    };

    assert_eq!(predicate, &BuildPredicate::BuildFlag("test".to_string()));
    assert_eq!(then_items.len(), 2);
    assert_eq!(then_items[0].node_type(), "ImportStatement");
    assert_eq!(then_items[1].node_type(), "FunctionDeclaration");
    assert_eq!(
        else_items.as_ref().map(|items| items.len()),
        Some(1),
        "else branch should carry one item"
    );
}

#[test]
fn parses_feature_and_target_predicates() {
    let ast = NyashParser::parse_from_string(
        r#"
when all(Feature("alloc.fastpath"), Target.os == linux) {
    function enabled() {
        return 1
    }
}
"#,
    )
    .expect("compound build cfg predicate should parse");

    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    let ASTNode::BuildWhen { predicate, .. } = &statements[0] else {
        panic!("expected BuildWhen");
    };

    assert_eq!(
        predicate,
        &BuildPredicate::All(vec![
            BuildPredicate::Feature("alloc.fastpath".to_string()),
            BuildPredicate::TargetEq {
                key: "os".to_string(),
                value: "linux".to_string(),
            },
        ])
    );
}
