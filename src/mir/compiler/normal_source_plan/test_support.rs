use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl, Span};

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

pub(super) fn input(source: ASTNode) -> PreparedNormalSourcePlanInputV1 {
    PreparedNormalSourcePlanInputV1::new(source, "normal-source-plan0-test")
}

pub(super) fn program(statements: Vec<ASTNode>) -> ASTNode {
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

pub(super) fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(super) fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

pub(super) fn function(name: &str, arity: usize, is_static: bool) -> ASTNode {
    let params = (0..arity)
        .map(|index| format!("p{index}"))
        .collect::<Vec<_>>();
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        param_decls: params
            .iter()
            .map(|name| ParamDecl {
                name: name.to_owned(),
                declared_type_name: None,
            })
            .collect(),
        params,
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

pub(super) fn function_with_body(name: &str, body: Vec<ASTNode>, is_static: bool) -> ASTNode {
    let mut function = function(name, 0, is_static);
    let ASTNode::FunctionDeclaration {
        body: function_body,
        ..
    } = &mut function
    else {
        unreachable!()
    };
    *function_body = body;
    function
}

pub(super) fn value_return(value: ASTNode) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    }
}

pub(super) fn integer_return_function(name: &str, value: i64) -> ASTNode {
    function_with_body(name, vec![value_return(literal(value))], false)
}

pub(super) fn i64_parameter_return_function(
    name: &str,
    declared_type_name: Option<&str>,
    returned_name: &str,
) -> ASTNode {
    let mut function = function(name, 1, false);
    let ASTNode::FunctionDeclaration {
        param_decls, body, ..
    } = &mut function
    else {
        unreachable!()
    };
    param_decls[0].declared_type_name = declared_type_name.map(str::to_owned);
    *body = vec![value_return(variable(returned_name))];
    function
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
