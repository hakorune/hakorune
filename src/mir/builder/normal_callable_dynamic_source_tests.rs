use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1,
};
use crate::parser::NyashParser;

use super::{
    SourceBackedDynamicCallableIssueV1, SourceBackedDynamicCallableIssuerV1,
    VerifiedSourceBackedDynamicCallableV1,
};

fn parsed_method(source: &str, box_name: &str, method_name: &str) -> ASTNode {
    let program = NyashParser::parse_from_string(source).expect("source parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == box_name => {
                methods.get_declaration(method_name).cloned()
            }
            _ => None,
        })
        .expect("exact method declaration")
}

fn issue(
    function: &ASTNode,
) -> Result<VerifiedSourceBackedDynamicCallableV1, SourceBackedDynamicCallableIssueV1> {
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(function).expect("function syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("resolved forest")
    else {
        panic!("selected callable unexpectedly deferred")
    };
    let forest = forests
        .into_vec()
        .pop()
        .expect("one selected callable forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    SourceBackedDynamicCallableIssuerV1::issue(function, &forest, &projection)
}

#[test]
fn skip_while_source_seals_complete_dynamic_formals_and_exact_carrier() {
    let function = parsed_method(
        "static box ParserScanLoopBox {\n\
             skip_while(text, pos, end, pred) {\n\
                 local i = pos\n\
                 loop(i < end) {\n\
                     i = i + 1\n\
                 }\n\
                 return i\n\
             }\n\
         }",
        "ParserScanLoopBox",
        "skip_while",
    );

    let product = issue(&function).expect("source-backed dynamic callable");
    assert_eq!(
        product
            .formals()
            .iter()
            .map(|row| row.parameter_ordinal())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(product.local_initializations().len(), 1);
    assert_eq!(product.loops().len(), 1);
    assert_eq!(product.loops()[0].carriers().len(), 1);
    assert_eq!(
        product.loops()[0].carriers()[0].local(),
        product.local_initializations()[0].local()
    );
    assert_eq!(product.loops()[0].carriers()[0].condition_reads().len(), 1);
    assert_eq!(product.loops()[0].carriers()[0].body_rebinds().len(), 1);
    assert_eq!(
        product.formals()[1].binding(),
        product.local_initializations()[0].formal()
    );
    assert_eq!(product.owner(), product.formals()[0].binding().owner());
    assert_eq!(
        product.loops()[0]
            .membership()
            .source()
            .site()
            .node()
            .segments(),
        &[crate::mir::resolved_semantics::SourcePathSegmentV1::Body(1)]
    );
}

#[test]
fn typed_formal_is_not_relabelled_dynamic_or_used_as_dynamic_local_origin() {
    let function = parsed_method(
        "static box Scan {\n\
             skip(text, pos: i64, end) {\n\
                 local i = pos\n\
                 loop(i < end) { i = i + 1 }\n\
                 return i\n\
             }\n\
         }",
        "Scan",
        "skip",
    );

    let product = issue(&function).expect("complete source catalog");
    assert_eq!(
        product
            .formals()
            .iter()
            .map(|row| row.parameter_ordinal())
            .collect::<Vec<_>>(),
        vec![0, 2]
    );
    assert!(product.local_initializations().is_empty());
    assert!(product.loops()[0].carriers().is_empty());
}

#[test]
fn multi_loop_callable_keeps_nearest_loop_carriers_separate() {
    let function = parsed_method(
        "static box Scan {\n\
             walk(a0, b0) {\n\
                 local a = a0\n\
                 local b = b0\n\
                 loop(a < 9) {\n\
                     loop(b < 4) { b = b + 1 }\n\
                     a = a + 1\n\
                 }\n\
                 return a\n\
             }\n\
         }",
        "Scan",
        "walk",
    );

    let product = issue(&function).expect("multi-loop dynamic source");
    assert_eq!(product.loops().len(), 2);
    assert_eq!(product.loops()[0].carriers().len(), 1);
    assert_eq!(product.loops()[1].carriers().len(), 1);
    assert_ne!(
        product.loops()[0].carriers()[0].local(),
        product.loops()[1].carriers()[0].local()
    );
}

#[test]
fn foreign_projection_and_forest_cannot_be_repaired_by_matching_shape() {
    let first = parsed_method(
        "static box A { f(x) { local y = x loop(y < 2) { y = y + 1 } return y } }",
        "A",
        "f",
    );
    let second = parsed_method(
        "static box B { f(x) { local y = x loop(y < 2) { y = y + 1 } return y } }",
        "B",
        "f",
    );
    let first_syntax = CallableFunctionSyntaxViewV1::from_function_ast(&first).unwrap();
    let second_syntax = CallableFunctionSyntaxViewV1::from_function_ast(&second).unwrap();
    let mut first_resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(first_forests) = first_resolver
        .resolve_selected_callable_forests(&[first_syntax.function()])
        .unwrap()
    else {
        panic!("first source deferred")
    };
    let first_forest = first_forests.into_vec().pop().unwrap();
    let first_projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &first,
        &first_forest,
        first_syntax.function().root_profile(),
    )
    .unwrap();
    let mut second_resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(second_forests) = second_resolver
        .resolve_selected_callable_forests(&[second_syntax.function()])
        .unwrap()
    else {
        panic!("second source deferred")
    };
    let second_forest = second_forests.into_vec().pop().unwrap();

    assert!(matches!(
        SourceBackedDynamicCallableIssuerV1::issue(&first, &second_forest, &first_projection),
        Err(SourceBackedDynamicCallableIssueV1::SourceProjection(_))
    ));
}

#[test]
fn names_only_compatibility_params_do_not_become_verified_dynamic_source() {
    let function = ASTNode::FunctionDeclaration {
        name: "legacy".into(),
        params: vec!["x".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: "x".into(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };

    assert!(matches!(
        issue(&function),
        Err(
            SourceBackedDynamicCallableIssueV1::ParameterDeclarationCardinality {
                names: 1,
                declarations: 0
            }
        )
    ));
}

#[test]
fn explicit_typed_and_untyped_param_decls_remain_distinct_in_manual_ast() {
    let function = ASTNode::FunctionDeclaration {
        name: "manual".into(),
        params: vec!["typed".into(), "dynamic".into()],
        param_decls: vec![
            ParamDecl {
                name: "typed".into(),
                declared_type_name: Some("i64".into()),
            },
            ParamDecl {
                name: "dynamic".into(),
                declared_type_name: None,
            },
        ],
        return_type_name: None,
        body: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: "dynamic".into(),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };

    let product = issue(&function).expect("manual exact declaration");
    assert_eq!(product.formals().len(), 1);
    assert_eq!(product.formals()[0].parameter_ordinal(), 1);
}
