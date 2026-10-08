use super::*;
use crate::ast::ASTNode;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::parser::NyashParser;

#[test]
fn received_handle_observer_rejects_initializer_argument_and_response_drift_without_retry() {
    let ASTNode::Program { statements, .. } = NyashParser::parse_from_string(
        "function probe() { local maker = new Maker() local item = maker.make(7) return item }",
    )
    .unwrap() else {
        panic!("Program")
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .unwrap();
    let source = VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
    let input = source.root_function_input().unwrap();
    let rows: Vec<_> = input
        .function()
        .expression_source()
        .initializers()
        .collect();
    let row = rows
        .iter()
        .find(|row| {
            input
                .function()
                .method_call(row.initializer_site().unwrap())
                .is_some()
        })
        .unwrap();
    let declaration = row.declaration_site();
    let SourceBindingSiteV1::Local { statement, .. } = declaration else {
        panic!("local")
    };
    let site = OwnedExprSiteV1::new(input.owner(), row.initializer_site().unwrap().clone());
    let receiver = rows
        .iter()
        .find(|candidate| candidate.binding() != row.binding())
        .unwrap()
        .binding();
    let argument = input
        .function()
        .method_call(site.site())
        .unwrap()
        .arguments()[0]
        .site()
        .clone();
    for mutation in 0..10 {
        let mut demands = 0;
        let mut old_predicate = 0;
        let result = issue_lexical_local_call(
            input,
            statement,
            &site,
            if mutation == 1 {
                rows[0].declaration_site().clone()
            } else {
                declaration.clone()
            },
            if mutation == 2 {
                receiver
            } else {
                row.binding()
            },
            &[receiver],
            LocalCallResultClassV1::Handle,
            &mut |_| {
                old_predicate += 1;
                Ok::<_, String>(true)
            },
            &mut |owned, request| {
                demands += 1;
                assert_eq!(owned, &site);
                assert!(
                    matches!(request, BorrowedCallActualRequestV1::ReceivedHandleArguments(destination) if destination == row.binding())
                );
                if mutation == 3 {
                    return Err("selected-original-error".into());
                }
                if mutation == 8 {
                    return Ok(Some(BorrowedCallArgumentsV1::Scalar(Box::new([]))));
                }
                let arguments = match mutation {
                    4 => vec![LocalCallArgumentV1::BorrowedActual {
                        ordinal: 1,
                        site: argument.clone(),
                    }],
                    5 => vec![LocalCallArgumentV1::BorrowedActual {
                        ordinal: 0,
                        site: site.site().clone(),
                    }],
                    6 => vec![LocalCallArgumentV1::Integer(8)],
                    7 => vec![LocalCallArgumentV1::Scalar(receiver)],
                    9 => Vec::new(),
                    _ => vec![LocalCallArgumentV1::BorrowedActual {
                        ordinal: 0,
                        site: argument.clone(),
                    }],
                };
                Ok(Some(BorrowedCallArgumentsV1::HandleSource(
                    arguments.into_boxed_slice(),
                )))
            },
        );
        assert_eq!(
            old_predicate, 0,
            "selected demand/refusal never retries the old predicate"
        );
        assert_eq!(demands, usize::from(!matches!(mutation, 1 | 2)));
        match mutation {
            0 => {
                let observation = result.unwrap().unwrap();
                assert_eq!(
                    observation.local_binding(),
                    Some((declaration, row.binding()))
                );
                assert_eq!(observation.prior_homes(), [receiver]);
            }
            3 => assert_eq!(result.unwrap_err(), "selected-original-error"),
            _ => assert!(result.unwrap().is_none()),
        }
    }
    // Producer source keeps Nullable/Handle distinct and never retries a
    // selected response, including Unavailable and wrong argument identity.
    for kind in [
        LocalCallResultClassV1::Nullable,
        LocalCallResultClassV1::Handle,
    ] {
        for mutation in 0..10 {
            let mut demands = 0;
            let result = issue_received_producer_local_call(
                input,
                statement,
                &site,
                if mutation == 7 {
                    rows[0].declaration_site().clone()
                } else {
                    declaration.clone()
                },
                row.binding(),
                &[receiver],
                &mut |owned, request| {
                    demands += 1;
                    assert_eq!(owned, &site);
                    let BorrowedCallActualRequestV1::ReceivedObjectArguments(
                        destination,
                        requested,
                    ) = request
                    else {
                        panic!("producer request only")
                    };
                    assert_eq!(destination, row.binding());
                    if mutation == 9 || requested != kind {
                        return Ok(None);
                    }
                    if mutation == 6 {
                        return Err("producer-original-error".to_string());
                    }
                    if mutation == 4 {
                        return Ok(Some(BorrowedCallArgumentsV1::Scalar(Box::new([]))));
                    }
                    let args = vec![match mutation {
                        5 => LocalCallArgumentV1::BorrowedActual {
                            ordinal: 1,
                            site: argument.clone(),
                        },
                        8 => LocalCallArgumentV1::Integer(8),
                        _ => LocalCallArgumentV1::BorrowedActual {
                            ordinal: 0,
                            site: argument.clone(),
                        },
                    }]
                    .into_boxed_slice();
                    Ok(Some(BorrowedCallArgumentsV1::SourceObject {
                        result: if mutation == 2 {
                            LocalCallResultClassV1::I64
                        } else {
                            kind
                        },
                        arguments: match mutation {
                            1 => ObjectCallSourceSupportV1::Observed(args),
                            3 => ObjectCallSourceSupportV1::Unavailable,
                            _ => ObjectCallSourceSupportV1::SourceOnly(args),
                        },
                    }))
                },
            );
            assert_eq!(
                demands,
                if mutation == 7 {
                    0
                } else if mutation == 9 || kind == LocalCallResultClassV1::Handle {
                    2
                } else {
                    1
                }
            );
            match mutation {
                0 | 1 => {
                    let observation = result.unwrap().unwrap().unwrap();
                    assert_eq!(observation.result(), kind);
                    assert_eq!(
                        observation.local_binding(),
                        Some((declaration, row.binding()))
                    );
                    assert_eq!(observation.prior_homes(), [receiver]);
                }
                6 => assert_eq!(result.unwrap_err(), "producer-original-error"),
                9 => assert!(result.unwrap().is_none()),
                _ => assert_eq!(
                    result.unwrap().unwrap().unwrap_err(),
                    HomePrefixUnavailableV1::SourceMismatch
                ),
            }
        }
    }
    assert!(issue_received_producer_local_call(
        input,
        statement,
        &site,
        declaration.clone(),
        row.binding(),
        &[],
        &mut |_, _| -> Result<Option<BorrowedCallArgumentsV1>, String> {
            panic!("non-Home local never demands a sibling ingress error")
        },
    )
    .unwrap()
    .is_none());
}
