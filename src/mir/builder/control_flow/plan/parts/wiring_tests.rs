#[cfg(test)]
mod tests {
    use super::super::loop_::lower_loop_v0;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
    use crate::mir::builder::control_flow::plan::recipe_tree::{
        BlockContractKind, ExitKind, IfContractKind, LoopKindV0, LoopV0Features, RecipeBodies,
        RecipeBlock, RecipeItem,
    };
    use crate::mir::builder::control_flow::plan::recipes::refs::StmtRef;
    use crate::mir::builder::control_flow::plan::{CoreExitPlan, CorePlan};
    use crate::mir::builder::stmts::variable_stmt::build_local_statement;
    use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
    use crate::mir::builder::MirBuilder;
    use crate::mir::ValueId;
    use std::collections::BTreeMap;

    fn span() -> Span {
        Span::unknown()
    }

    fn lit_bool(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: span(),
        }
    }

    fn lit_int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn bin(op: BinaryOperator, lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: span(),
        }
    }

    fn assign(name: &str, expr: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(expr),
            span: span(),
        }
    }

    #[test]
    fn test_joinir_wiring_then_only_loop_uses_join_dst_for_carrier() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("joinir_wiring_loop_if_loop".to_string());

        let _scope = LexicalScopeGuard::new(&mut builder);
        let _sum_id = build_local_statement(
            &mut builder,
            vec!["sum".to_string()],
            vec![Some(Box::new(lit_int(0)))],
        )
        .expect("declare sum");

        let mut current_bindings: BTreeMap<String, crate::mir::ValueId> =
            builder.variable_ctx.variable_map.clone();

        let sum_noop = assign("sum", var("sum"));
        let sum_inc = assign("sum", bin(BinaryOperator::Add, var("sum"), lit_int(1)));

        let inner_loop_ast = ASTNode::Loop {
            condition: Box::new(lit_bool(true)),
            body: vec![sum_inc.clone()],
            span: span(),
        };

        let then_body_ast = vec![sum_noop.clone(), inner_loop_ast.clone()];
        let if_ast = ASTNode::If {
            condition: Box::new(lit_bool(true)),
            then_body: then_body_ast.clone(),
            else_body: None,
            span: span(),
        };

        let mut arena = RecipeBodies::new();
        let inner_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            vec![sum_inc.clone()],
        ));
        let inner_body_block = RecipeBlock::new(
            inner_body_id,
            vec![RecipeItem::Stmt(StmtRef::new(0))],
        );

        let then_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            then_body_ast.clone(),
        ));
        let then_block = RecipeBlock::new(
            then_body_id,
            vec![
                RecipeItem::Stmt(StmtRef::new(0)),
                RecipeItem::LoopV0 {
                    loop_stmt: StmtRef::new(1),
                    kind: LoopKindV0::WhileLike,
                    cond_view: CondBlockView::from_expr(&lit_bool(true)),
                    body_block: Box::new(inner_body_block),
                    body_contract: BlockContractKind::NoExit,
                    features: LoopV0Features::default(),
                },
            ],
        );

        let outer_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            vec![if_ast],
        ));
        let outer_body_block = RecipeBlock::new(
            outer_body_id,
            vec![RecipeItem::IfV2 {
                if_stmt: StmtRef::new(0),
                cond_view: CondBlockView::from_expr(&lit_bool(true)),
                contract: IfContractKind::Join,
                then_block: Box::new(then_block),
                else_block: None,
            }],
        );

        let loop_plan = lower_loop_v0(
            &mut builder,
            &mut current_bindings,
            &CondBlockView::from_expr(&lit_bool(true)),
            BlockContractKind::NoExit,
            &arena,
            &outer_body_block,
            "test_loop_if_loop",
        )
        .expect("lower loop plan");

        let CorePlan::Loop(loop_plan) = loop_plan else {
            panic!("expected CorePlan::Loop");
        };

        let if_plan = loop_plan
            .body
            .iter()
            .find_map(|plan| match plan {
                CorePlan::If(plan) => Some(plan),
                _ => None,
            })
            .expect("expected if plan in loop body");

        let join_sum = if_plan
            .joins
            .iter()
            .find(|join| join.name == "sum")
            .expect("expected sum join entry");

        let continue_phi_args = loop_plan
            .body
            .iter()
            .rev()
            .find_map(|plan| match plan {
                CorePlan::Exit(CoreExitPlan::ContinueWithPhiArgs { phi_args, .. }) => {
                    Some(phi_args)
                }
                _ => None,
            })
            .expect("expected ContinueWithPhiArgs in loop body");

        assert!(
            continue_phi_args
                .iter()
                .any(|(_, value)| *value == join_sum.dst),
            "expected loop backedge to use join dst for sum carrier"
        );

        builder.exit_function_for_test();
    }

    #[test]
    fn test_joinir_obligation_allows_then_only_local_branch_scoped() {
        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let then_body_ast = vec![ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(lit_int(1)))],
            span: span(),
        }];
        let if_ast = ASTNode::If {
            condition: Box::new(lit_bool(true)),
            then_body: then_body_ast.clone(),
            else_body: None,
            span: span(),
        };

        let mut arena = RecipeBodies::new();
        let then_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            then_body_ast,
        ));
        let then_block = RecipeBlock::new(then_body_id, vec![RecipeItem::Stmt(StmtRef::new(0))]);

        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            vec![if_ast],
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::IfV2 {
                if_stmt: StmtRef::new(0),
                cond_view: CondBlockView::from_expr(&lit_bool(true)),
                contract: IfContractKind::Join,
                then_block: Box::new(then_block),
                else_block: None,
            }],
        );

        let result = super::super::entry::verify_no_exit_block_with_pre(
            &arena,
            &block,
            "test_if_join_obligation",
            None,
        );

        assert!(
            result.is_ok(),
            "expected then-only local intro to be branch-scoped (ok), got: {result:?}"
        );
    }

    #[test]
    fn test_joinir_obligation_freezes_loop_carrier_missing_in_pre() {
        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let loop_body = vec![assign("sum", bin(BinaryOperator::Add, var("sum"), lit_int(1)))];
        let loop_ast = ASTNode::Loop {
            condition: Box::new(lit_bool(true)),
            body: loop_body.clone(),
            span: span(),
        };

        let mut arena = RecipeBodies::new();
        let inner_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            loop_body,
        ));
        let inner_body_block = RecipeBlock::new(
            inner_body_id,
            vec![RecipeItem::Stmt(StmtRef::new(0))],
        );

        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            vec![loop_ast],
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::LoopV0 {
                loop_stmt: StmtRef::new(0),
                kind: LoopKindV0::WhileLike,
                cond_view: CondBlockView::from_expr(&lit_bool(true)),
                body_block: Box::new(inner_body_block),
                body_contract: BlockContractKind::NoExit,
                features: LoopV0Features::default(),
            }],
        );

        let pre_bindings: BTreeMap<String, ValueId> = BTreeMap::new();
        let err = super::super::entry::verify_no_exit_block_with_pre(
            &arena,
            &block,
            "test_loop_carrier_pre",
            Some(&pre_bindings),
        )
        .expect_err("expected missing carrier to freeze");

        assert!(
            err.contains("loop_carrier_missing_in_pre"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_joinir_obligation_freezes_exit_obligation() {
        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let loop_body = vec![
            assign("sum", bin(BinaryOperator::Add, var("sum"), lit_int(1))),
            ASTNode::Continue { span: span() },
        ];
        let loop_ast = ASTNode::Loop {
            condition: Box::new(lit_bool(true)),
            body: loop_body.clone(),
            span: span(),
        };

        let mut arena = RecipeBodies::new();
        let inner_body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            loop_body,
        ));
        let inner_body_block = RecipeBlock::new(
            inner_body_id,
            vec![RecipeItem::Stmt(StmtRef::new(0))],
        );

        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            vec![loop_ast],
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::LoopV0 {
                loop_stmt: StmtRef::new(0),
                kind: LoopKindV0::WhileLike,
                cond_view: CondBlockView::from_expr(&lit_bool(true)),
                body_block: Box::new(inner_body_block),
                body_contract: BlockContractKind::NoExit,
                features: LoopV0Features::default(),
            }],
        );

        let pre_bindings: BTreeMap<String, ValueId> = BTreeMap::new();
        let err = super::super::entry::verify_no_exit_block_with_pre(
            &arena,
            &block,
            "test_exit_obligation",
            Some(&pre_bindings),
        )
        .expect_err("expected exit obligation freeze");

        assert!(
            err.contains("port_sig_exit_not_defined")
                || err.contains("loop_carrier_missing_in_pre"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_joinir_obligation_freezes_return_obligation() {
        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let body_ast = vec![ASTNode::Return {
            value: Some(Box::new(lit_int(0))),
            span: span(),
        }];

        let mut arena = RecipeBodies::new();
        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            body_ast,
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::Exit {
                kind: ExitKind::Return,
                stmt: StmtRef::new(0),
            }],
        );

        let pre_bindings: BTreeMap<String, ValueId> = BTreeMap::new();
        let verified = super::super::entry::verify_exit_allowed_block_with_pre(
            &arena,
            &block,
            "test_return_obligation",
            Some(&pre_bindings),
        )
        .expect("verify exit-allowed block");

        super::super::verify::verify_port_sig_obligations_if_enabled(
            &verified,
            "test_return_obligation",
        )
        .expect("expected empty return obligations to be allowed");
    }

    #[test]
    fn test_joinir_obligation_exit_allowed_port_sig_is_seeded() {
        use crate::mir::builder::control_flow::plan::recipe_tree::verified::PortType;

        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let body_ast = vec![ASTNode::Return {
            value: Some(Box::new(var("sum"))),
            span: span(),
        }];

        let mut arena = RecipeBodies::new();
        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            body_ast,
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::Exit {
                kind: ExitKind::Return,
                stmt: StmtRef::new(0),
            }],
        );

        let mut pre_bindings: BTreeMap<String, ValueId> = BTreeMap::new();
        pre_bindings.insert("sum".to_string(), ValueId::new(0));

        let verified = super::super::entry::verify_exit_allowed_block_with_pre(
            &arena,
            &block,
            "test_exit_allowed_port_sig",
            Some(&pre_bindings),
        )
        .expect("verify exit-allowed block");

        assert!(
            verified.port_sig().port(PortType::Return).contains_key("sum"),
            "expected return port to include pre-bound var"
        );
        assert!(
            verified.port_sig().port(PortType::Break).contains_key("sum"),
            "expected break port to include pre-bound var"
        );
        assert!(
            verified.port_sig().port(PortType::Continue).contains_key("sum"),
            "expected continue port to include pre-bound var"
        );

        super::super::verify::verify_port_sig_obligations_if_enabled(
            &verified,
            "test_exit_allowed_port_sig",
        )
        .expect("expected exit-allowed port sig to be valid");
    }

    #[test]
    fn test_joinir_obligation_exit_only_port_sig_is_seeded() {
        use crate::mir::builder::control_flow::plan::recipe_tree::verified::PortType;

        std::env::set_var("HAKO_JOINIR_STRICT", "1");
        std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", "1");

        let body_ast = vec![ASTNode::Return {
            value: Some(Box::new(var("sum"))),
            span: span(),
        }];

        let mut arena = RecipeBodies::new();
        let body_id = arena.register(crate::mir::builder::control_flow::plan::recipes::RecipeBody::new(
            body_ast,
        ));
        let block = RecipeBlock::new(
            body_id,
            vec![RecipeItem::Exit {
                kind: ExitKind::Return,
                stmt: StmtRef::new(0),
            }],
        );

        let mut pre_bindings: BTreeMap<String, ValueId> = BTreeMap::new();
        pre_bindings.insert("sum".to_string(), ValueId::new(0));

        let verified = super::super::entry::verify_exit_only_block_with_pre(
            &arena,
            &block,
            "test_exit_only_port_sig",
            Some(&pre_bindings),
        )
        .expect("verify exit-only block");

        assert!(
            verified.port_sig().port(PortType::Return).contains_key("sum"),
            "expected return port to include pre-bound var"
        );
        assert!(
            verified.port_sig().port(PortType::Break).contains_key("sum"),
            "expected break port to include pre-bound var"
        );
        assert!(
            verified.port_sig().port(PortType::Continue).contains_key("sum"),
            "expected continue port to include pre-bound var"
        );

        super::super::verify::verify_port_sig_obligations_if_enabled(
            &verified,
            "test_exit_only_port_sig",
        )
        .expect("expected exit-only port sig to be valid");
    }
}
