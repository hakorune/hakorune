use super::normal_root_execution::{
    ParserNormalRootExecutionRoleV1, ParserNormalRootExecutionSourceDispositionV1,
    ParserNormalRootExecutionTestTerminalV1, ParserRetainedCallableSemanticSourceTestLoanV1,
};
use crate::ast::ASTNode;
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

fn with_retained<R>(
    source: &str,
    callback: impl for<'source> FnOnce(ParserRetainedCallableSemanticSourceTestLoanV1<'source>) -> R,
) -> R {
    let retained = NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .unwrap()
    .into_retained_source()
    .unwrap();
    ParserNormalRootExecutionTestTerminalV1::consume_retained_once(retained, callback).unwrap()
}

#[test]
fn one_scoped_loan_retains_the_exact_declaration_rows() {
    let observed = with_retained("static box Api { run(value) { return value } }", |loan| {
        let catalog = loan.callable_parameter_source();
        let syntax = loan.callable_declaration_syntax();
        assert_eq!(catalog.declarations().len(), 1);
        assert_eq!(syntax.declarations().len(), 1);
        let row = &syntax.declarations()[0];
        let ASTNode::FunctionDeclaration { name, .. } = row.declaration() else {
            unreachable!("exact loan retains one function declaration")
        };
        (row.source_row_index(), name.clone())
    });

    assert_eq!(observed.0, 0);
    assert_eq!(observed.1, "run");
}

#[test]
fn retained_source_keeps_total_app_relation_and_script_sibling() {
    with_retained("static box Main { main() { return 1 } }", |loan| {
        assert!(matches!(
            loan.normal_root_execution(),
            ParserNormalRootExecutionSourceDispositionV1::Ready(root)
                if root.role() == ParserNormalRootExecutionRoleV1::App
        ));
        assert!(loan.retains_script_source_rows());
        assert!(loan.source_authority_is_ready());
    });
}

#[test]
fn retained_source_does_not_turn_main_arity_into_parser_policy() {
    with_retained(
        "static box Main { main(argument) { return argument } }",
        |loan| {
            assert!(matches!(
                loan.normal_root_execution(),
                ParserNormalRootExecutionSourceDispositionV1::Ready(root)
                    if root.role() == ParserNormalRootExecutionRoleV1::App
            ));
        },
    );
}

#[test]
fn equal_source_text_does_not_merge_parser_authority() {
    let witness = || {
        with_retained("static box Api { run(value) { return value } }", |loan| {
            loan.parser_invocation_witness()
                .expect("retained Ready root")
                .clone()
        })
    };
    let first = witness();
    let second = witness();
    assert!(!first.same_as(&second));
}

#[test]
fn rejected_retention_keeps_the_atomic_parser_owner_until_named_discard() {
    let rejected = NyashParser::parse_from_string_with_callable_parameter_source(
        "gate Build.test { box Enabled { run(value) { return value } } } else { box Disabled { run(value) { return value } } }",
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("selected gate parser product")
    .into_retained_source()
    .expect_err("selected gate cannot become a complete retained source");
    assert_eq!(
        rejected.error(),
        super::ParserCallableSourceRetentionErrorV1::ParameterSourceUnavailable
    );
    rejected.discard();
}
