use super::*;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::home_new_prefix::local_flow::PrefixLocalFlow;
#[test]
fn complete_expression_installation_publishes_only_after_the_final_field_batch() {
    let parsed = crate::parser::NyashParser::parse_from_string(
        "function probe() { print(\"猫\") return 0 }",
    )
    .unwrap();
    let crate::ast::ASTNode::Program { statements, .. } = parsed else {
        panic!("Program")
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
        .unwrap();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap();
    let input = unit.root_function_input().unwrap();
    let shape = input.body_shape().unwrap();
    let statement = shape
        .statements()
        .iter()
        .find(|statement| {
            matches!(
                statement,
                crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. }
            )
        })
        .unwrap();
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .unwrap();
    let source =
        crate::mir::resolved_value_profile::expression_plan::statement_expression_source_plan_v1(
            &ledger,
            shape,
            statement.site(),
        )
        .unwrap()
        .unwrap();
    let flow = PrefixLocalFlow::new(input);
    let sibling = flow.clone();
    for batch_succeeds in [false, true] {
        let mut installation = flow.expression_installation();
        let mut batches = 0;
        let plan = qualify_print_expression_v1::<()>(
                Rc::clone(&source),
                &[],
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &std::collections::BTreeMap::new(),
                &mut |_| Ok(None),
                &mut |plan| installation.approve(plan),
                &mut |fields: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], _| {
                    assert!(fields.is_empty());
                    batches += 1;
                    Ok(batch_succeeds)
                },
            )
            .unwrap();
        assert_eq!(batches, 1);
        assert_eq!(plan.is_some(), batch_succeeds);
        if let Some(plan) = plan {
            let original_frame = Rc::clone(plan.print_frame().unwrap());
            installation.commit();
            let installed = sibling.expression_plans();
            assert_eq!(installed.len(), 1);
            assert!(Rc::ptr_eq(installed[0].source(), &source));
            assert!(Rc::ptr_eq(
                installed[0].print_frame().unwrap(),
                &original_frame
            ));
            assert!(installed[0]
                .print_frame()
                .unwrap()
                .owned_result_nodes()
                .is_empty());
            // Membership comes from the installed source frame even when
            // no call, field, temporary or physical observation exists.
            use crate::mir::resolved_semantics::home_new_prefix::{RootHomeExitV1, RootHomeFlow};
            let root = RootHomeFlow {
                named_arrays: Vec::new(),
                exits: Default::default(),
                maps: Vec::new(),
                local_calls: Vec::new(),
                expression_plans: installed,
            };
            let expected = root.lifecycle_prefixes(&[]).unwrap();
            assert_eq!(expected.len(), 1);
            let covered = RootHomeExitV1::issue(
                Box::new([]),
                Box::new([]),
                vec![source.root().statement().clone()].into_boxed_slice(),
            );
            let sibling = RootHomeExitV1::issue(Box::new([]), Box::new([]), Box::new([]));
            assert!(expected[0].covered_by(&covered));
            assert!(!expected[0].covered_by(&sibling));
        } else {
            drop(installation);
            assert!(
                sibling.expression_plans().is_empty(),
                "failed field batch publishes no frame"
            );
        }
    }
    let mut installation = sibling.expression_installation();
    let mut batches = 0;
    assert!(qualify_print_expression_v1::<()>(
        source,
        &[],
        &mut |_| None,
        &mut |_| None,
        &mut |_| None,
        &mut |_| None,
        &mut |_| None,
        &std::collections::BTreeMap::new(),
        &mut |_| Ok(None),
        &mut |plan| installation.approve(plan),
        &mut |_, _| {
            batches += 1;
            Ok(true)
        }
    )
    .unwrap()
    .is_none());
    assert_eq!(
        batches, 0,
        "duplicate source slot rejects before a field batch"
    );
    // LOCAL i64 demand validates the whole RHS even without Sub/Div.
    // This is expression preparation, not LOCAL store/publication permission.
    for (rhs, integer_reference, eligible) in [
        ("7", false, true),
        ("2 + 3", false, true),
        ("n", true, true),
        ("n", false, false),
        ("true", false, false),
        ("\"7\"", false, false),
        ("\"text\" + 1", false, false),
        ("null", false, false),
        ("1 - 2", false, true),
        ("6 / 2", false, true),
        ("6 / 0", false, false),
        ("2 * 3", false, true),
        ("6 % 2", false, true),
        ("6 % 0", false, true),
        ("3 & 1", false, true),
        ("2 << 1", false, true),
    ] {
        for batch_succeeds in [false, true] {
            let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(p, n) {{ p.value = {rhs} return p }}"
            ))
            .unwrap();
            let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
                panic!("Program")
            };
            let unit =
                VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
            let input = unit.root_function_input().unwrap();
            let shape = input.body_shape().unwrap();
            let ledger = input
                .forest()
                .callable_source_ledger(input.owner())
                .unwrap();
            let source = crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1(
                    &ledger, shape, &shape.assignment_sources()[0]).unwrap();
            let flow = PrefixLocalFlow::new(input);
            let mut installation = flow.expression_installation();
            let mut batches = 0;
            let plan = qualify_field_write_expression_v1::<()>(Rc::clone(&source), true, None,
                    &mut |_| integer_reference.then_some(crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer),
                    &mut |_| None, &mut |_| None, &mut |_| None, &mut |plan| installation.approve(plan),
                    &mut |requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], _| { assert!(requests.is_empty()); batches += 1; Ok(batch_succeeds) }).unwrap();
            assert_eq!(plan.is_some(), eligible && batch_succeeds, "RHS {rhs}");
            assert_eq!(
                batches,
                usize::from(eligible),
                "reject wrong RHS before field staging"
            );
            if plan.is_some() {
                installation.commit();
            } else {
                drop(installation);
            }
            let installed = flow.expression_plans();
            assert_eq!(installed.len(), usize::from(eligible && batch_succeeds));
            if let Some(plan) = installed.first() {
                assert!(Rc::ptr_eq(plan.source(), &source));
            }
        }
    }
    // A borrowed write retains its original assignment and allocation-order
    // snapshot. This frame alone never issues a physical store capability.
    {
        let parsed = crate::parser::NyashParser::parse_from_string(
            "function probe(p, n) { p.value = n return p }",
        )
        .unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let shape = input.body_shape().unwrap();
        let assignment = &shape.assignment_sources()[0];
        let crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { receiver } =
            input
                .function()
                .assignment_target(assignment.target_site())
                .unwrap()
        else {
            panic!("field write")
        };
        let binding = |site| match input.function().variable_ref(site).unwrap() {
            crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) => binding,
            _ => panic!("local source"),
        };
        let homes = [
            LiveHomeV1::Stable(binding(receiver)),
            LiveHomeV1::Stable(binding(assignment.value_site())),
        ];
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let source = crate::mir::resolved_value_profile::expression_plan::assignment_expression_source_plan_v1(
                &ledger, shape, assignment).unwrap();
        assert!(HomeExpressionPlanV1::borrowed_field_write(
            Rc::clone(&source),
            &[homes[0].clone(), homes[0].clone()]
        )
        .is_none());
        let plan = HomeExpressionPlanV1::borrowed_field_write(Rc::clone(&source), &homes).unwrap();
        let frame = Rc::clone(plan.borrowed_field_write_frame().unwrap());
        assert!(Rc::ptr_eq(frame.source(), &source));
        assert_eq!(frame.prior_homes(), &homes);
        assert!(plan.print_frame().is_none() && plan.condition_frame().is_none());
        let flow = PrefixLocalFlow::new(input);
        let mut installation = flow.expression_installation();
        assert!(installation.approve(&plan));
        drop(installation);
        assert!(flow.expression_plans().is_empty());
        let mut installation = flow.expression_installation();
        assert!(installation.approve(&plan));
        installation.commit();
        let installed = flow.expression_plans();
        assert!(Rc::ptr_eq(
            installed[0].borrowed_field_write_frame().unwrap(),
            &frame
        ));
        use crate::mir::resolved_semantics::home_new_prefix::{
            LifecyclePrefixSourceV1, RootHomeExitV1, RootHomeFlow,
        };
        let root = RootHomeFlow {
            named_arrays: Vec::new(),
            exits: Default::default(),
            maps: Vec::new(),
            local_calls: Vec::new(),
            expression_plans: installed,
        };
        let prefixes = root.lifecycle_prefixes(&[]).unwrap();
        assert_eq!(
            prefixes.len(),
            1,
            "an original write requires a packet without any calls"
        );
        assert!(
            matches!(&prefixes[0], LifecyclePrefixSourceV1::BorrowedFieldWrite { frame: original, .. }
                if Rc::ptr_eq(original, &frame))
        );
        assert!(prefixes[0].covered_by(&RootHomeExitV1::issue(
            Box::new([]),
            Box::new([]),
            vec![assignment.statement_site().clone()].into_boxed_slice()
        )));
        assert!(!prefixes[0].covered_by(&RootHomeExitV1::issue(
            Box::new([]),
            Box::new([]),
            Box::new([])
        )));
        let mut installation = flow.expression_installation();
        assert!(!installation.approve(&plan));
    }
    // Bool roots share the transaction and source inventory, while their
    // conditional reach and primitive result laws stay a separate frame.
    for expression in [
        "true && false",
        "false || true",
        "(1 == 1) && (2 < 3)",
        "(1 != 2) || (3 >= 4)",
        "(\"a\" == \"a\") && true",
        "(\"a\" != \"b\") || false",
        "(\"a\" < \"b\") && true",
        "(\"a\" <= \"b\") && true",
        "(\"b\" > \"a\") || false",
        "(\"b\" >= \"a\") && true",
    ] {
        let parsed = crate::parser::NyashParser::parse_from_string(&format!(
            "function probe() {{ if {expression} {{ return 1 }} return 0 }}"
        ))
        .unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let statement = input.function().if_region_sites().next().unwrap();
        let source = crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(input, statement).unwrap();
        let flow = PrefixLocalFlow::new(input);
        let sibling = flow.clone();
        for succeeds in [false, true] {
            let mut installation = flow.expression_installation();
            let mut batches = 0;
            let plan = qualify_bool_condition_v1::<()>(
                    Rc::clone(&source),
                    &[],
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| None,
                    &mut |_| Ok(None),
                    &mut |plan| installation.approve(plan),
                    &mut |fields: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], _| {
                        assert!(fields.is_empty());
                        batches += 1;
                        Ok(succeeds)
                    },
                )
                .unwrap();
            assert_eq!(batches, 1, "{expression}");
            assert_eq!(plan.is_some(), succeeds);
            if let Some(plan) = plan {
                assert!(plan.print_frame().is_none());
                let original = Rc::clone(plan.condition_frame().unwrap());
                assert!(Rc::ptr_eq(original.source(), &source));
                assert_eq!(original.logical_nodes().len(), 1);
                assert!(original.calls().is_empty());
                let portable = crate::mir::loop_recipe_contract::project_source_loop_bool_v1(
                    Rc::clone(&original),
                )
                .unwrap();
                assert!(portable.matches_frame(&original));
                assert!(portable.imports().is_empty());
                assert_eq!(portable.region().may_fault(), expression.contains('"'));
                installation.commit();
                let installed = sibling.expression_plans();
                assert_eq!(installed.len(), 1);
                assert!(Rc::ptr_eq(
                    installed[0].condition_frame().unwrap(),
                    &original
                ));
                use crate::mir::resolved_semantics::home_new_prefix::{
                    LifecyclePrefixSourceV1, RootHomeExitV1, RootHomeFlow,
                };
                let root = RootHomeFlow {
                    named_arrays: Vec::new(),
                    exits: Default::default(),
                    maps: Vec::new(),
                    local_calls: Vec::new(),
                    expression_plans: installed,
                };
                let prefixes = root.lifecycle_prefixes(&[]).unwrap();
                assert_eq!(
                    prefixes.len(),
                    1,
                    "even a call-free Bool condition is mandatory"
                );
                assert!(
                    matches!(&prefixes[0], LifecyclePrefixSourceV1::BoolCondition { frame, .. }
                        if Rc::ptr_eq(frame, &original))
                );
                assert!(prefixes[0].covered_by(&RootHomeExitV1::issue(
                    Box::new([]),
                    Box::new([]),
                    vec![statement.clone()].into_boxed_slice()
                )));
                assert!(!prefixes[0].covered_by(&RootHomeExitV1::issue(
                    Box::new([]),
                    Box::new([]),
                    Box::new([])
                )));
            } else {
                drop(installation);
                assert!(sibling.expression_plans().is_empty());
            }
        }
        let mut installation = sibling.expression_installation();
        let mut batches = 0;
        assert!(qualify_bool_condition_v1::<()>(
            source,
            &[],
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| Ok(None),
            &mut |plan| installation.approve(plan),
            &mut |_, _| {
                batches += 1;
                Ok(true)
            }
        )
        .unwrap()
        .is_none());
        assert_eq!(batches, 0);
    }
    for (expression, selected, accepted) in [
        ("true && false", true, true),
        ("\"a\" < \"b\"", true, true),
        ("(\"a\" < \"b\") || (\"z\" > \"x\")", true, true),
        ("(1 == 1) && (2 < 3)", true, true),
        ("false && obj.missing()", true, false),
        ("1 && true", true, false),
        ("true", false, false),
        ("obj != null && true", false, false),
        ("(3 - 1 == 2) && true", false, false),
    ] {
        let parsed = crate::parser::NyashParser::parse_from_string(&format!(
            "function probe(obj) {{ if {expression} {{ return 1 }} return 0 }}"
        ))
        .unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let statement = input.function().if_region_sites().next().unwrap();
        for field_ok in [false, true] {
            let flow = PrefixLocalFlow::new(input);
            let mut calls = Vec::new();
            let mut path_calls = Default::default();
            let mut batches = 0;
            let mut demands = 0;
            use crate::mir::resolved_semantics::home_new_prefix::condition::{
                observe, ConditionInstallationV1,
            };
            let result = observe::<()>(
                    input,
                    statement,
                    &flow,
                    &[],
                    true,
                    &mut calls,
                    &mut path_calls,
                    &mut |requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1], _| {
                        assert!(requests.is_empty());
                        batches += 1;
                        Ok(field_ok.then(Vec::new))
                    },
                    &mut |_, _| {
                        demands += 1;
                        Ok(None)
                    },
                )
                .unwrap();
            assert_eq!(
                matches!(result, ConditionInstallationV1::Unselected),
                !selected,
                "{expression}"
            );
            assert_eq!(
                matches!(result, ConditionInstallationV1::Installed),
                accepted && field_ok,
                "{expression}"
            );
            assert_eq!(batches, usize::from(accepted), "{expression}");
            assert_eq!(demands, usize::from(expression == "false && obj.missing()"));
            assert_eq!(
                flow.expression_plans().len(),
                usize::from(accepted && field_ok)
            );
            assert!(calls.is_empty() && path_calls.is_empty());
        }
    }
    for expression in [
        "1 && true",
        "false && obj.missing()",
        "\"x\" == 1",
        "\"x\" < 1",
        "\"x\" == null",
        "(\"a\" + \"b\") == \"ab\"",
    ] {
        let parsed = crate::parser::NyashParser::parse_from_string(&format!(
            "function probe(obj) {{ if {expression} {{ return 1 }} return 0 }}"
        ))
        .unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let source = crate::mir::resolved_value_profile::expression_plan::condition_expression_source_plan_v1(
                input, input.function().if_region_sites().next().unwrap()).unwrap();
        let mut batches = 0;
        assert!(qualify_bool_condition_v1::<()>(
            source,
            &[],
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| Ok(None),
            &mut |_| { panic!("unproved condition cannot install") },
            &mut |_, _| {
                batches += 1;
                Ok(true)
            }
        )
        .unwrap()
        .is_none());
        assert_eq!(batches, 0);
    }
}

#[test]
fn initializer_scope_keeps_its_original_profile_and_home_snapshot() {
    let parsed = crate::parser::NyashParser::parse_from_string(
        "function probe() { local prior = \"p\" + \"q\" local out = \"a\" + \"b\" return 0 }",
    )
    .unwrap();
    let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
        panic!("Program")
    };
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
    let input = unit.root_function_input().unwrap();
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .unwrap();
    let initializers: Vec<_> = input
        .function()
        .expression_source()
        .initializers()
        .collect();
    let source =
        crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(
            &ledger,
            input.body_shape(),
            initializers[1],
        )
        .unwrap();
    for profile in [
        leaves::ExpressionLeafProfileV1::Print,
        leaves::ExpressionLeafProfileV1::LocalInitializer,
    ] {
        let homes = [LiveHomeV1::Stable(initializers[0].binding())];
        let prepared = leaves::prepare_expression_calls_v1::<()>(
            &source,
            profile,
            &Default::default(),
            &homes,
            &mut |_| panic!("literal initializer has no Call"),
        )
        .unwrap()
        .unwrap();
        let mut batches = 0;
        let frame = qualify_initializer_expression_v1::<()>(
            prepared,
            &[],
            &[],
            &mut |_| None,
            None,
            false,
            &mut |_| None,
            &mut |_, _, _| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| None,
            &mut |_| true,
            &mut |_, _| {
                batches += 1;
                Ok(true)
            },
        )
        .unwrap();
        assert_eq!(
            frame.is_some(),
            profile == leaves::ExpressionLeafProfileV1::LocalInitializer
        );
        assert_eq!(batches, usize::from(frame.is_some()));
        if let Some(plan) = frame {
            assert_eq!(plan.initializer_frame().unwrap().prior_homes(), homes);
        }
    }
}

#[test]
fn original_loop_condition_keeps_frame_and_conditional_reach_without_if_permission() {
    use crate::mir::resolved_value_profile::{
        expression_children::{ExpressionChildReachV1, ExpressionChildrenV1},
        expression_plan::{
            condition_expression_source_plan_v1, loop_condition_expression_source_plan_v1,
            ExpressionPlanRootV1,
        },
    };
    for (operator, truth) in [("&&", true), ("||", false)] {
        let parsed = crate::parser::NyashParser::parse_from_string(&format!(
                "function probe(flag) {{ loop(flag {operator} true) {{ break }} if true {{ return 1 }} return 0 }}")).unwrap();
        let crate::ast::ASTNode::Program { mut statements, .. } = parsed else {
            panic!("Program")
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(statements.remove(0)).unwrap();
        let input = unit.root_function_input().unwrap();
        let site = input.function().only_loop_site().unwrap();
        let source = loop_condition_expression_source_plan_v1(input, &site).unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let membership = ledger.resolved_loop_source(&site).unwrap();
        assert!(
            matches!(source.root(), ExpressionPlanRootV1::LoopCondition { statement, condition, bundle, frame }
                if statement == &site && condition == source.nodes()[0].site()
                    && Some(bundle) == input.function().loop_region_bundle(&site).ok()
                    && frame.matches(membership.frame()))
        );
        assert_eq!(source.root().statement(), &site);
        assert!(source.root().initializer().is_none());
        assert_eq!(source.ordered_postorder_v1().unwrap(), vec![1, 2, 0]);
        let ExpressionChildrenV1::Known(children) = source.nodes()[0].children() else {
            panic!("original children")
        };
        assert_eq!(children[0].reach, ExpressionChildReachV1::Always);
        assert_eq!(
            children[1].reach,
            ExpressionChildReachV1::When {
                condition: children[0].site.clone(),
                value: truth
            }
        );
        assert!(condition_expression_source_plan_v1(input, &site).is_err());
        let if_site = input.function().if_region_sites().next().unwrap();
        assert!(loop_condition_expression_source_plan_v1(input, if_site).is_err());
        let mut batches = 0;
        assert!(
            qualify_bool_condition_v1::<()>(
                source,
                &[],
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| None,
                &mut |_| Ok(None),
                &mut |_| true,
                &mut |_, _| {
                    batches += 1;
                    Ok(true)
                }
            )
            .unwrap()
            .is_none(),
            "source purpose cannot borrow If-only Home permission"
        );
        assert_eq!(batches, 0);
    }
}
