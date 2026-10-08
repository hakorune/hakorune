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
}
