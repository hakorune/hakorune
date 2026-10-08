//! Nullable outer membership never grants an I64 result to argument calls.
use super::*;
use crate::ast::ASTNode;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::parser::NyashParser;

fn source(arguments: &str) -> VerifiedResolvedSourceUnitV1 {
    let parsed = NyashParser::parse_from_string(&format!(
        "function probe(token) {{ local item = token.make({arguments}) return item }}"
    ))
    .unwrap();
    let ASTNode::Program { statements, .. } = parsed else {
        panic!("Program")
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .unwrap();
    VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap()
}

fn call(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: bool,
    nested: &mut impl FnMut(&OwnedExprSiteV1) -> Result<bool, String>,
    borrowed: &mut impl FnMut(
        &OwnedExprSiteV1,
        BorrowedCallActualRequestV1<'_>,
    ) -> Result<Option<BorrowedCallArgumentsV1>, String>,
) -> Result<Option<LocalCallObservationV1>, String> {
    let original = input
        .function()
        .expression_source()
        .initializers()
        .next()
        .unwrap();
    let SourceBindingSiteV1::Local { statement, .. } = original.declaration_site() else {
        panic!("original local")
    };
    let site = OwnedExprSiteV1::new(input.owner(), original.initializer_site().unwrap().clone());
    issue_lexical_nullable_local_call(
        input,
        statement,
        &site,
        original.declaration_site().clone(),
        original.binding(),
        &[],
        &PrefixLocalFlow::new(input),
        &mut |candidate| Ok(selected && candidate == &site),
        nested,
        borrowed,
    )
}

#[test]
fn nullable_outer_literal_and_zero_inputs_do_not_demand_i64_result() {
    for arguments in ["7", ""] {
        let source = source(arguments);
        let input = source.root_function_input().unwrap();
        let row = call(
            input,
            true,
            &mut |_| panic!("the outer Nullable result cannot require I64"),
            &mut |_, request| {
                assert!(matches!(
                    request,
                    BorrowedCallActualRequestV1::ScalarArguments
                ));
                Ok(None)
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(row.result(), LocalCallResultClassV1::Nullable);
        let expected = if arguments.is_empty() {
            vec![]
        } else {
            vec![LocalCallArgumentV1::Integer(7)]
        };
        assert_eq!(row.arguments(), expected);
    }
}

#[test]
fn nullable_outer_preserves_exact_nested_i64_result_membership() {
    let source = source("token.number(11)");
    let input = source.root_function_input().unwrap();
    let (nested_site, _) = input
        .function()
        .method_calls()
        .find(|(_, call)| call.selector() == "number")
        .unwrap();
    let nested_site = OwnedExprSiteV1::new(input.owner(), nested_site.clone());
    for admitted in [false, true] {
        let mut consulted = Vec::new();
        let row = call(
            input,
            true,
            &mut |candidate| {
                consulted.push(candidate.clone());
                Ok(admitted && candidate == &nested_site)
            },
            &mut |candidate, request| {
                if candidate == &nested_site {
                    assert!(matches!(
                        request,
                        BorrowedCallActualRequestV1::I64ResultArguments
                    ));
                }
                Ok(None)
            },
        )
        .unwrap();
        assert_eq!(consulted, vec![nested_site.clone()]);
        if admitted {
            let row = row.unwrap();
            let [LocalCallArgumentV1::CallResult(inner)] = row.arguments() else {
                panic!("original nested call")
            };
            assert_eq!(inner.site(), &nested_site);
            assert_eq!(inner.arguments(), &[LocalCallArgumentV1::Integer(11)]);
        } else {
            assert!(
                row.is_none(),
                "an unproved or non-I64 argument call stays unavailable"
            );
        }
    }
}

#[test]
fn nullable_outer_nonmember_does_not_request_arguments() {
    let source = source("7");
    assert!(call(
        source.root_function_input().unwrap(),
        false,
        &mut |_| panic!("nonmember must not consult nested calls"),
        &mut |_, _| panic!("nonmember must not demand borrowed arguments"),
    )
    .unwrap()
    .is_none());
}

#[test]
fn nullable_outer_borrowed_error_propagates_before_strict_sealing() {
    let source = source("token.number(11)");
    assert_eq!(
        call(
            source.root_function_input().unwrap(),
            true,
            &mut |_| panic!("a borrowed error cannot retry strict sealing"),
            &mut |_, _| Err("original borrowed refusal".into()),
        )
        .unwrap_err(),
        "original borrowed refusal"
    );
}

#[test]
fn nullable_outer_noninteger_literals_remain_unavailable() {
    for arguments in ["null", "true", "3.5", "\"text\""] {
        let source = source(arguments);
        assert!(call(
            source.root_function_input().unwrap(),
            true,
            &mut |_| panic!("a literal is not an argument call"),
            &mut |_, _| Ok(None),
        )
        .unwrap()
        .is_none());
    }
}
