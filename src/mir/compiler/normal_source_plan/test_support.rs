use crate::ast::{ASTNode, Span};

use super::{
    NormalMainFunctionPreflightV1, NormalSourcePlanClassifierV1, PreparedNormalSourcePlanInputV1,
    SealedNormalScalarRootV1, SealedNormalSourcePlanV1, VerifiedNormalMainThunkPlanV1,
};

pub(crate) fn with_main_thunk_for_test<R>(
    program: ASTNode,
    inspect: impl FnOnce(VerifiedNormalMainThunkPlanV1<'_>) -> R,
) -> R {
    let input = PreparedNormalSourcePlanInputV1::new(program, "main-thunk-shared-test");
    let plan = NormalSourcePlanClassifierV1::seal(input).expect("valid Main0");
    let SealedNormalSourcePlanV1::ScalarRoot(SealedNormalScalarRootV1::Main0(main)) = plan else {
        panic!("expected Main0");
    };
    let source = main.prepare_function_source().expect("exact Main source");
    let resolved = source
        .prepare_embedded_resolved_main()
        .expect("embedded Main resolution");
    let main = NormalMainFunctionPreflightV1::seal(&resolved).expect("Main F1 plan");
    inspect(VerifiedNormalMainThunkPlanV1::seal(main).expect("Main thunk plan"))
}

pub(super) fn integer_local_return_body(
    local_name: &str,
    declared_type: Option<&str>,
    initializer: Option<ASTNode>,
    returned_name: &str,
) -> Vec<ASTNode> {
    vec![
        ASTNode::Local {
            variables: vec![local_name.to_owned()],
            initial_values: vec![initializer.map(Box::new)],
            declared_type_names: vec![declared_type.map(str::to_owned)],
            span: Span::unknown(),
        },
        ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: returned_name.to_owned(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        },
    ]
}
