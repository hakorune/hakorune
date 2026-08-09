use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourcePathV1,
};
use crate::parser::NyashParser;

use super::{
    DynamicLoopComparisonKindV1, DynamicLoopOperationResultClassV1,
    DynamicLoopOperationSourceIssueV1, DynamicLoopOperationSourceIssuerV1,
};
use crate::mir::builder::normal_callable_dynamic_source::SourceBackedDynamicCallableIssuerV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;

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
) -> Result<super::VerifiedDynamicLoopOperationSourceSetV1, DynamicLoopOperationSourceIssueV1> {
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(mut forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .unwrap()
    else {
        panic!("source deferred")
    };
    let forest = forests.into_vec().pop().unwrap();
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        function,
        &forest,
        syntax.function().root_profile(),
    )
    .unwrap();
    let state_input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        function,
        &forest,
        &projection,
    )
    .unwrap();
    let state = CallableSemanticLoweringState::from_exact_source(state_input).unwrap();
    let schedule = state
        .loop_binding_source_projection()
        .project(SourcePathV1::root_body(1).node())
        .unwrap();
    let source_input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        function,
        &forest,
        &projection,
    )
    .unwrap();
    let dynamic =
        SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(source_input).unwrap();
    let operation_input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        function,
        &forest,
        &projection,
    )
    .unwrap();
    DynamicLoopOperationSourceIssuerV1::issue(operation_input, &dynamic, &schedule)
}

#[test]
fn production_skip_while_issues_exact_dynamic_compare_and_add_rebind() {
    let function = parsed_method(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        "ParserScanLoopBox",
        "skip_while",
    );
    let product = issue(&function).expect("source-only Dynamic operation relations");
    assert_eq!(
        product.comparison().kind(),
        DynamicLoopComparisonKindV1::Less
    );
    assert_eq!(
        product.comparison().result(),
        DynamicLoopOperationResultClassV1::Bool
    );
    assert_eq!(
        product.add_rebind().result(),
        DynamicLoopOperationResultClassV1::Dynamic
    );
    assert_eq!(product.add_rebind().delta(), 1);
    assert_eq!(
        product.comparison().carrier(),
        product.add_rebind().carrier()
    );
    assert_ne!(
        product.comparison().carrier(),
        product.comparison().operand()
    );
    assert_eq!(product.owner(), product.add_rebind().carrier().owner());
    assert_eq!(
        product.loop_site().segments(),
        &[crate::mir::resolved_semantics::SourcePathSegmentV1::Body(1)]
    );
}

#[test]
fn typed_comparison_operand_is_not_relabelled_dynamic() {
    let function = parsed_method(
        "static box Scan { skip(pos, end: i64) { local i = pos loop(i < end) { i = i + 1 } return i } }",
        "Scan",
        "skip",
    );
    assert!(matches!(
        issue(&function),
        Err(DynamicLoopOperationSourceIssueV1::ComparisonOperandNotDynamic)
    ));
}

#[test]
fn subtraction_and_reversed_add_are_no_safe_slice_not_repaired() {
    for update in ["i = i - 1", "i = 1 + i"] {
        let source = format!(
            "static box Scan {{ skip(pos, end) {{ local i = pos loop(i < end) {{ {update} }} return i }} }}"
        );
        let function = parsed_method(&source, "Scan", "skip");
        assert!(matches!(
            issue(&function),
            Err(DynamicLoopOperationSourceIssueV1::RebindNoSafeSlice)
        ));
    }
}
