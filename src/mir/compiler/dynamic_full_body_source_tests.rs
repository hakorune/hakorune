use crate::ast::ASTNode;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;
use crate::parser::NyashParser;

use super::dynamic_full_body_source::{
    verify_iteration_local_source_closure, DynamicFullBodyBindingRoleV1,
    DynamicFullBodySourceIssueV1, DynamicFullBodySourceIssuerV1, DynamicFullBodySourceRoleV1,
    DynamicFullBodySourceSiteV1, VerifiedDynamicLoopFullBodySourceInventoryV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;

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

fn input_for(function: ASTNode) -> ResolvedFunctionLoweringInputV1<'static> {
    let unit = Box::leak(Box::new(
        super::VerifiedResolvedSourceUnitV1::resolve_function(function).expect("fixture resolves"),
    ));
    unit.root_function_input().expect("root input")
}

fn issue(
    function: ASTNode,
) -> Result<VerifiedDynamicLoopFullBodySourceInventoryV1, DynamicFullBodySourceIssueV1> {
    let input = input_for(function);
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("source ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let completion = verify_function_completion_v1(input).expect("completion");
    DynamicFullBodySourceIssuerV1::issue(input, membership, completion)
}

fn production_skip_while() -> ASTNode {
    parsed_method(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        "ParserScanLoopBox",
        "skip_while",
    )
}

#[test]
fn unchanged_skip_while_issues_complete_ast_free_source_inventory() {
    let product = issue(production_skip_while()).expect("full source inventory");

    assert_eq!(product.bindings().len(), 6);
    assert_eq!(product.rows().len(), 28);
    assert_eq!(product.completion().explicit_sites().len(), 2);
    assert_eq!(
        product.loop_membership().source().site().node().segments(),
        &[crate::mir::resolved_semantics::SourcePathSegmentV1::Body(1)]
    );
    for role in [
        DynamicFullBodyBindingRoleV1::Src,
        DynamicFullBodyBindingRoleV1::Pos,
        DynamicFullBodyBindingRoleV1::End,
        DynamicFullBodyBindingRoleV1::PredChars,
        DynamicFullBodyBindingRoleV1::Induction,
        DynamicFullBodyBindingRoleV1::IterationLocalCh,
    ] {
        assert_eq!(
            product
                .bindings()
                .iter()
                .filter(|row| row.role() == role)
                .count(),
            1
        );
    }
    for role in [
        DynamicFullBodySourceRoleV1::SubstringCall,
        DynamicFullBodySourceRoleV1::IndexOfCall,
        DynamicFullBodySourceRoleV1::InnerIf,
        DynamicFullBodySourceRoleV1::InnerReturn,
        DynamicFullBodySourceRoleV1::OuterReturn,
    ] {
        assert_eq!(
            product
                .rows()
                .iter()
                .filter(|row| row.role() == role)
                .count(),
            1
        );
    }
}

#[test]
fn iteration_local_closure_is_exactly_one_read_in_the_loop_body_scope() {
    let input = input_for(production_skip_while());
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("source ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let completion = verify_function_completion_v1(input).expect("completion");
    let product = DynamicFullBodySourceIssuerV1::issue(input, membership, completion)
        .expect("full source inventory");
    let binding = product
        .bindings()
        .iter()
        .find(|row| row.role() == DynamicFullBodyBindingRoleV1::IterationLocalCh)
        .expect("iteration-local binding")
        .binding();
    let read = profile_expr_site(&product, DynamicFullBodySourceRoleV1::IndexOfArgumentCh);

    assert_eq!(
        verify_iteration_local_source_closure(
            input,
            product.loop_membership().scope_region().scope(),
            binding,
            read,
        ),
        Ok(())
    );
    assert_eq!(
        verify_iteration_local_source_closure(
            input,
            input.function().function_scope(),
            binding,
            read,
        ),
        Err(DynamicFullBodySourceIssueV1::IterationLocalScopeMismatch)
    );
    assert_eq!(
        verify_iteration_local_source_closure(
            input,
            product.loop_membership().scope_region().scope(),
            binding,
            profile_expr_site(
                &product,
                DynamicFullBodySourceRoleV1::IndexOfReceiverPredChars,
            ),
        ),
        Err(DynamicFullBodySourceIssueV1::IterationLocalUseClosureMismatch)
    );
}

#[test]
fn unchanged_skip_while_profile_call_sites_match_the_neutral_method_call_ledger() {
    let input = input_for(production_skip_while());
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("source ledger");
    let calls = ledger
        .method_calls()
        .map(|(_, row)| row)
        .collect::<Vec<_>>();
    let product = issue(production_skip_while()).expect("full source inventory");

    assert_eq!(calls.len(), 2);
    for (selector, arity, call_role, receiver_role, argument_roles) in [
        (
            "substring",
            2,
            DynamicFullBodySourceRoleV1::SubstringCall,
            DynamicFullBodySourceRoleV1::SubstringReceiverSrc,
            vec![
                DynamicFullBodySourceRoleV1::SubstringStartI,
                DynamicFullBodySourceRoleV1::SubstringEndAdd,
            ],
        ),
        (
            "indexOf",
            1,
            DynamicFullBodySourceRoleV1::IndexOfCall,
            DynamicFullBodySourceRoleV1::IndexOfReceiverPredChars,
            vec![DynamicFullBodySourceRoleV1::IndexOfArgumentCh],
        ),
    ] {
        let call_site = profile_expr_site(&product, call_role);
        let neutral = calls
            .iter()
            .find(|row| row.site() == call_site)
            .expect("neutral MethodCall row");
        assert_eq!(neutral.selector(), selector);
        assert_eq!(neutral.arity(), arity);
        assert_eq!(neutral.arguments().len(), arity as usize);
        assert_eq!(call_site, neutral.result_site());
        assert_eq!(
            profile_expr_site(&product, receiver_role),
            neutral.receiver_site()
        );
        for (argument, role) in neutral.arguments().iter().zip(argument_roles) {
            assert_eq!(profile_expr_site(&product, role), argument.site());
        }
    }
}

fn profile_expr_site(
    product: &VerifiedDynamicLoopFullBodySourceInventoryV1,
    role: DynamicFullBodySourceRoleV1,
) -> &crate::mir::resolved_semantics::SourceExprSiteV1 {
    product
        .rows()
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Expression(site) => site,
                DynamicFullBodySourceSiteV1::Statement(_) => {
                    panic!("expected expression role: {role:?}")
                }
            })
        })
        .expect("profile expression site")
}

#[test]
fn extra_loop_statement_rejects_instead_of_narrowing_source() {
    let function = parsed_method(
        "static box Scan {\n\
         skip_while(src, pos, end, pred_chars) {\n\
           local i = pos\n\
           loop(i < end) {\n\
             local ch = src.substring(i, i + 1)\n\
             if pred_chars.indexOf(ch) < 0 { return i }\n\
             print(ch)\n\
             i = i + 1\n\
           }\n\
           return i\n\
         }\n\
         }",
        "Scan",
        "skip_while",
    );
    assert!(matches!(
        issue(function),
        Err(DynamicFullBodySourceIssueV1::BodyShape)
    ));
}

#[test]
fn different_dynamic_selector_is_not_reclassified_as_the_canary_shape() {
    let function = parsed_method(
        "static box Scan {\n\
         skip_while(src, pos, end, pred_chars) {\n\
           local i = pos\n\
           loop(i < end) {\n\
             local ch = src.slice(i, i + 1)\n\
             if pred_chars.indexOf(ch) < 0 { return i }\n\
             i = i + 1\n\
           }\n\
           return i\n\
         }\n\
         }",
        "Scan",
        "skip_while",
    );
    assert!(matches!(
        issue(function),
        Err(DynamicFullBodySourceIssueV1::ExpressionShape)
    ));
}

#[test]
fn foreign_completion_rejects_before_any_builder_effect() {
    let input = input_for(production_skip_while());
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("source ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let foreign = input_for(production_skip_while());
    let completion = verify_function_completion_v1(foreign).expect("foreign completion");

    assert!(matches!(
        DynamicFullBodySourceIssuerV1::issue(input, membership, completion),
        Err(DynamicFullBodySourceIssueV1::ForeignOwner)
    ));
}
