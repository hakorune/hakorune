//! Argument purpose uses original topology, not an enclosing Print/Return root.
use crate::ast::ASTNode;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::NyashParser;

use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1,
};

#[test]
fn original_call_argument_plan_keeps_outer_identity_ordinal_and_nearest_statement() {
    let source = "static box Helpers { value(q) { local x = Sink.take(q.substring(0,1), 7) if true { return Sink.take(q,8) } return 0 } }";
    let root = NyashParser::parse_from_string(source).unwrap();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&root, "Helpers", "value")).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut checked = 0;
    let mut statements = std::collections::BTreeSet::new();
    for (site, _) in input.function().method_calls().filter(|(_, call)| call.selector() == "take") {
        let call = input.function().method_call_source_v1(site).unwrap();
        for ordinal in 0..2 {
            let plan = call_argument_expression_source_plan_v1(input, call, ordinal).unwrap();
            let ExpressionPlanRootV1::CallArgument { statement, call: retained, ordinal: retained_ordinal } = plan.root() else { panic!("argument purpose") };
            assert!(std::sync::Arc::ptr_eq(call, retained));
            assert_eq!(*retained_ordinal, ordinal);
            assert_eq!(plan.nodes()[0].site(), call.arguments()[ordinal as usize].site());
            assert!(!plan.nodes().iter().any(|node| node.site() == site));
            assert_eq!(plan.ordered_postorder_v1().unwrap().len(), plan.nodes().len());
            assert_eq!(input.function().source_site_inventory().containing_statement(site), Some(statement));
            assert!(plan.root().initializer().is_none());
            statements.insert(statement.clone());
            checked += 1;
        }
        assert!(call_argument_expression_source_plan_v1(input, call, 2).is_err());
        let foreign = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&root, "Helpers", "value")).unwrap();
        assert!(call_argument_expression_source_plan_v1(foreign.root_function_input().unwrap(), call, 0).is_err());
    }
    assert_eq!(checked, 4);
    assert_eq!(statements.len(), 2, "nested Return belongs to its own registered statement");
}

fn method_declaration(root: &ASTNode, owner: &str, method: &str) -> ASTNode {
    let ASTNode::Program { statements, .. } = root else {
        panic!("program")
    };
    for statement in statements {
        let ASTNode::BoxDeclaration { name, methods, .. } = statement else {
            continue;
        };
        if name == owner {
            return methods
                .get_declaration(method)
                .expect("method declaration")
                .clone();
        }
    }
    panic!("missing method declaration")
}

#[test]
fn statement_expression_roots_keep_original_print_and_return_roles() {
    use super::expression_children::ExpressionChildIssueV1;
    use super::expression_root::{statement_expression_root_v1, StatementExpressionRoleV1};
    use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathSegmentV1 as Role, SourceStmtSiteV1};
    let root = NyashParser::parse_from_string(
        r#"static box Helpers { value() { print("x" + 1) return 2 } }"#,
    ).unwrap();
    let mut declaration = method_declaration(&root, "Helpers", "value");
    let ASTNode::FunctionDeclaration { body, .. } = &mut declaration else { unreachable!() };
    // Both are original source statements, but neither has a value-root role.
    body.push(ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(3),
        span: crate::ast::Span::unknown(),
    });
    body.push(ASTNode::Return { value: None, span: crate::ast::Span::unknown() });
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(declaration).unwrap();
    let input = unit.root_function_input().unwrap();
    let ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
    let shape = input.body_shape().unwrap();
    assert_eq!(shape.statements().len(), 4);
    for (index, role) in [StatementExpressionRoleV1::PrintValue, StatementExpressionRoleV1::ReturnValue]
        .into_iter().enumerate()
    {
        let statement = shape.statements()[index].site();
        let value = statement_expression_root_v1(&ledger, shape, statement).unwrap().unwrap();
        assert_eq!(value.role, role);
        assert_eq!(value.value.node().segments(), &[Role::Body(index as u32), Role::Value]);
        assert!(super::expression_children::expression_children_v1(
            &ledger, Some(shape), value.value,
        ).is_ok());
        let plan = super::expression_plan::statement_expression_source_plan_v1(
            &ledger, shape, statement,
        ).unwrap().unwrap();
        assert_eq!(plan.owner(), ledger.owner());
        assert_eq!(plan.nodes()[0].site(), value.value);
        assert!(!plan.has_opaque_obligations());
        assert!(matches!(plan.root(), super::expression_plan::ExpressionPlanRootV1::Statement {
            statement: original, role: actual_role, value: original_value,
        } if original == statement && *actual_role == role && original_value == value.value));
        if index == 0 {
            assert_eq!(plan.nodes().len(), 3);
            assert_eq!(plan.ordered_postorder_v1().unwrap(), vec![1, 2, 0]);
            let super::expression_children::ExpressionChildrenV1::Known(children) =
                plan.nodes()[0].children() else { panic!("original concat children") };
            assert_eq!(children[0].role, Role::Lhs);
            assert_eq!(children[1].role, Role::Rhs);
            assert_eq!(plan.nodes()[1].site(), &children[0].site);
            assert_eq!(plan.nodes()[2].site(), &children[1].site);
        } else {
            assert_eq!(plan.nodes().len(), 1);
        }
    }
    let nested = NyashParser::parse_from_string(
        r#"static box Helpers { value() { print(("a" + 1) + ("b" + 2)) return 0 } }"#,
    ).unwrap();
    let nested = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&nested, "Helpers", "value")).unwrap();
    let input = nested.root_function_input().unwrap();
    let nested_ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
    let nested_shape = input.body_shape().unwrap();
    let plan = super::expression_plan::statement_expression_source_plan_v1(
        &nested_ledger, nested_shape, nested_shape.statements()[0].site()).unwrap().unwrap();
    assert_eq!(plan.ordered_postorder_v1().unwrap(), vec![2, 3, 1, 5, 6, 4, 0],
        "left subtree completes before right subtree, then outer Add");
    for statement in &shape.statements()[2..] {
        assert!(statement_expression_root_v1(&ledger, shape, statement.site()).unwrap().is_none());
    }
    let missing = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![Role::Body(99)]));
    assert!(matches!(statement_expression_root_v1(&ledger, shape, &missing),
        Err(ExpressionChildIssueV1::MissingSource)));
    let foreign = VerifiedResolvedSourceUnitV1::resolve_function(
        method_declaration(&root, "Helpers", "value"),
    ).unwrap();
    let foreign_input = foreign.root_function_input().unwrap();
    assert!(matches!(statement_expression_root_v1(
        &ledger, foreign_input.body_shape().unwrap(), shape.statements()[0].site(),
    ), Err(ExpressionChildIssueV1::ForeignOwner)));
    // If conditions use their resolver-sealed bundle, never a Print/Return root.
    for (operator, truth) in [("&&", true), ("||", false)] {
        let root = NyashParser::parse_from_string(&format!(
            "static box Helpers {{ value(flag) {{ if flag {operator} true {{ return 1 }} return 0 }} }}",
        )).unwrap();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&root, "Helpers", "value")).unwrap();
        let input = unit.root_function_input().unwrap();
        let statement = input.function().if_region_sites().next().unwrap();
        let plan = super::expression_plan::condition_expression_source_plan_v1(input, statement).unwrap();
        assert_eq!(plan.owner(), input.owner());
        assert!(matches!(plan.root(), super::expression_plan::ExpressionPlanRootV1::IfCondition {
            statement: original, condition, bundle,
        } if original == statement && condition == plan.nodes()[0].site()
            && Some(bundle) == input.function().if_region_bundle(statement).ok()));
        assert_eq!(plan.nodes().len(), 3);
        assert_eq!(plan.ordered_postorder_v1().unwrap(), vec![1, 2, 0]);
        let super::expression_children::ExpressionChildrenV1::Known(children) = plan.nodes()[0].children()
            else { panic!("condition children") };
        assert_eq!(children[0].reach, super::expression_children::ExpressionChildReachV1::Always);
        assert_eq!(children[1].reach, super::expression_children::ExpressionChildReachV1::When {
            condition: children[0].site.clone(), value: truth,
        });
        assert!(super::expression_plan::condition_expression_source_plan_v1(input, &missing).is_err());
        let non_if = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![Role::Body(1)]));
        assert!(super::expression_plan::condition_expression_source_plan_v1(input, &non_if).is_err());
    }
    // Shape-independent Local profiles retain their original call receiver
    // qualification from the sealed method row, without a fabricated Me row.
    for expression in ["7 - 1", "me.scalar() - 1"] {
        let root = NyashParser::parse_from_string(&format!(
            "static box Helpers {{ value() {{ local n = {expression} return n }} scalar() {{ return 7 }} }}",
        )).unwrap();
        let declaration = method_declaration(&root, "Helpers", "value");
        let ASTNode::FunctionDeclaration { params, body, .. } = &declaration else { unreachable!() };
        // The enclosing original box is static. Resolve that real receiver
        // policy explicitly, as the callable catalog does in production.
        let syntax = crate::mir::resolved_semantics::FunctionSyntaxViewV1::from_borrowed_function_parts(
            params, body, crate::mir::resolved_semantics::ReceiverPolicyV1::StaticCurrentOwner,
        );
        let (forest, shapes) = FunctionSemanticResolverSessionV1::new(0).unwrap()
            .resolve_forest_with_body_shapes(syntax).unwrap();
        let owner = forest.roots()[0];
        let ledger = forest.callable_source_ledger(owner).unwrap();
        let initializer = ledger.initializer_relations().next().unwrap();
        let shaped = super::expression_plan::local_expression_source_plan_v1(
            &ledger, Some(&shapes[&owner]), initializer,
        ).unwrap();
        let without_shape = super::expression_plan::local_expression_source_plan_v1(
            &ledger, None, initializer,
        ).unwrap();
        assert_eq!(shaped, without_shape, "{expression}");
        assert!(!without_shape.has_opaque_obligations());
    }
}

#[test]
fn assignment_expression_plan_preserves_original_target_rhs_and_form() {
    use super::expression_children::ExpressionChildIssueV1;
    use super::expression_plan::{assignment_expression_source_plan_v1, ExpressionPlanRootV1};
    use crate::mir::resolved_semantics::{ResolvedAssignmentFormV1, SourcePathSegmentV1 as Role};
    let root = NyashParser::parse_from_string(
        "static box Helpers { value() { local n = 0 n = n + 1 n += 2 return n } }",
    ).unwrap();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        method_declaration(&root, "Helpers", "value"),
    ).unwrap();
    let input = unit.root_function_input().unwrap();
    let ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
    let shape = input.body_shape().unwrap();
    assert_eq!(shape.assignment_sources().len(), 2);
    for (row, form) in shape.assignment_sources().iter().zip([
        ResolvedAssignmentFormV1::Plain, ResolvedAssignmentFormV1::Compound,
    ]) {
        let plan = assignment_expression_source_plan_v1(&ledger, shape, row).unwrap();
        assert_eq!(plan.owner(), input.owner());
        assert_eq!(plan.root().statement(), row.statement_site());
        assert_eq!(plan.nodes()[0].site(), row.value_site());
        assert!(plan.root().initializer().is_none());
        assert!(matches!(plan.root(), ExpressionPlanRootV1::Assignment { assignment }
            if assignment == row && assignment.form() == form));
        assert_eq!(row.target_site().node().segments().last(), Some(&Role::Target));
        assert_eq!(row.value_site().node().segments().last(), Some(&Role::Value));
        assert!(!plan.has_opaque_obligations());
        assert_eq!(plan.nodes().len(), if form == ResolvedAssignmentFormV1::Plain { 3 } else { 1 });
        // A Compound RHS plan intentionally does not invent a target read or
        // operator: it is not the value ultimately stored by +=.
    }
    let other_root = NyashParser::parse_from_string(
        "static box Helpers { value() { local n = 0 n += 1 return n } }",
    ).unwrap();
    let other = VerifiedResolvedSourceUnitV1::resolve_function(
        method_declaration(&other_root, "Helpers", "value"),
    ).unwrap();
    let other_input = other.root_function_input().unwrap();
    assert!(matches!(assignment_expression_source_plan_v1(
        &ledger, other_input.body_shape().unwrap(), &shape.assignment_sources()[0],
    ), Err(ExpressionChildIssueV1::ForeignOwner)));
    assert!(matches!(assignment_expression_source_plan_v1(
        &ledger, shape, &other_input.body_shape().unwrap().assignment_sources()[0],
    ), Err(ExpressionChildIssueV1::ChildIdentity)), "same path with a different form is not the original row");
}

#[test]
fn original_field_paths_keep_root_and_order_without_direct_read_permission() {
    use super::expression_plan::{statement_expression_source_plan_v1, ExpressionNodeSourceV1};
    use crate::mir::resolved_semantics::BodyExpressionShapeV1;
    for (value, expected) in [("root.n", Some(vec!["n"])),
        ("root.child.n", Some(vec!["child", "n"])),
        ("root.child.next.n", Some(vec!["child", "next", "n"])),
        ("root.child.read().n", None), ("(root + 1).n", None)] {
        let source = format!("static box Helpers {{ value(root) {{ return {value} }} }}");
        let root = NyashParser::parse_from_string(&source).unwrap();
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(
            method_declaration(&root, "Helpers", "value")).unwrap();
        let input = unit.root_function_input().unwrap();
        let ledger = input.forest().callable_source_ledger(input.owner()).unwrap();
        let shape = input.body_shape().unwrap();
        let plan = statement_expression_source_plan_v1(&ledger, shape,
            shape.statements()[0].site()).unwrap().unwrap();
        let leaf = plan.nodes()[0].site();
        let path = plan.field_path_v1(leaf);
        let Some(names) = expected else { assert!(path.is_none(), "{value}"); continue; };
        let (binding, fields) = path.expect("original rooted field path");
        assert_eq!(binding.owner(), input.owner());
        let actual: Vec<_> = fields.iter().map(|index| match plan.nodes()[*index].source() {
            ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { field, .. }) => field.as_ref(),
            _ => panic!("field node"),
        }).collect();
        assert_eq!(actual, names);
        let ExpressionNodeSourceV1::Shape(BodyExpressionShapeV1::FieldAccess { object, .. }) =
            plan.nodes()[0].source() else { panic!("field leaf") };
        assert_eq!(plan.field_read_node_v1(leaf, object, binding, "n").is_some(), fields.len() == 1);
        assert!(plan.field_read_node_v1(leaf, leaf, binding, "n").is_none());
        assert!(plan.field_read_node_v1(leaf, object, binding, "other").is_none());
        assert!(plan.field_path_v1(object).is_some() == (fields.len() > 1));
    }
}
