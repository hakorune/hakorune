use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(crate) fn build_scan_with_init_loop_condition(
    facts: &crate::mir::builder::control_flow::plan::facts::loop_types::ScanWithInitFacts,
) -> crate::ast::ASTNode {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    let span = Span::unknown();
    let loop_var = ASTNode::Variable {
        name: facts.loop_var.clone(),
        span,
    };

    if facts.step_lit < 0 {
        return ASTNode::BinaryOp {
            operator: BinaryOperator::GreaterEqual,
            left: Box::new(loop_var),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span,
            }),
            span,
        };
    }

    if facts.dynamic_needle {
        let left = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.haystack.clone(),
                span,
            }),
            method: "length".to_string(),
            arguments: vec![],
            span,
        };
        let right = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: facts.needle.clone(),
                span,
            }),
            method: "length".to_string(),
            arguments: vec![],
            span,
        };
        let bound = ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: Box::new(left),
            right: Box::new(right),
            span,
        };
        return ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left: Box::new(loop_var),
            right: Box::new(bound),
            span,
        };
    }

    let right = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.haystack.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left: Box::new(loop_var),
        right: Box::new(right),
        span,
    }
}

pub(crate) fn verify_no_exit_block_recipe(
    recipe: &crate::mir::builder::control_flow::plan::facts::no_exit_block::NoExitBlockRecipe,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;

    check_block_contract(&recipe.arena, &recipe.block, BlockContractKind::NoExit, context)
        .map(|_| ())
        .map_err(|e| Freeze::contract("scan_segment_no_exit_recipe_verification_failed").with_hint(&e))
}

pub(crate) fn verify_exit_allowed_block_recipe(
    recipe: &crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitAllowedBlockRecipe,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;

    check_block_contract(
        &recipe.arena,
        &recipe.block,
        BlockContractKind::ExitAllowed,
        context,
    )
    .map(|_| ())
    .map_err(|e| {
        Freeze::contract("scan_segment_exit_allowed_recipe_verification_failed").with_hint(&e)
    })
}

pub(crate) fn verify_stmt_only_block_recipe(
    recipe: &crate::mir::builder::control_flow::plan::facts::stmt_view::StmtOnlyBlockRecipe,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;

    check_block_contract(&recipe.arena, &recipe.block, BlockContractKind::StmtOnly, context)
        .map(|_| ())
        .map_err(|e| Freeze::contract("scan_segment_stmt_only_verification_failed").with_hint(&e))
}

pub(crate) fn verify_nested_loop_stmt_only_if_available(
    nested: &crate::mir::builder::control_flow::plan::scan_loop_segments::NestedLoopRecipe,
    context: &str,
) -> Result<(), Freeze> {
    let Some(body_stmt_only) = nested.body_stmt_only.as_ref() else {
        return Ok(());
    };
    verify_stmt_only_block_recipe(body_stmt_only, context)
}

pub(crate) fn verify_stmt_ref_in_bounds(
    stmt: crate::mir::builder::control_flow::plan::recipes::refs::StmtRef,
    len: usize,
    context: &str,
) -> Result<(), Freeze> {
    if stmt.index() >= len {
        return Err(Freeze::contract(&format!(
            "recipe index out of bounds: ctx={context} idx={} len={len}",
            stmt.index()
        )));
    }
    Ok(())
}

pub(crate) fn verify_stmt_pair_in_bounds(
    pair: crate::mir::builder::control_flow::plan::recipes::refs::StmtPair,
    len: usize,
    context: &str,
) -> Result<(), Freeze> {
    let a = pair.a.index();
    let b = pair.b.index();
    if a >= len || b >= len {
        return Err(Freeze::contract(&format!(
            "recipe index out of bounds: ctx={context} pair=({a},{b}) len={len}"
        )));
    }
    Ok(())
}

pub(crate) fn verify_stmt_span_len(
    span: crate::mir::builder::control_flow::plan::recipes::refs::StmtSpan,
    context: &str,
) -> Result<usize, Freeze> {
    let (start, end) = span.indices();
    if end < start {
        return Err(Freeze::contract(&format!(
            "recipe span invalid: ctx={context} start={start} end={end}"
        )));
    }
    Ok(end - start)
}

pub(crate) fn verify_exit_only_block_recipe(
    recipe: &crate::mir::builder::control_flow::plan::facts::exit_only_block::ExitOnlyBlockRecipe,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::recipe_tree::BlockContractKind;
    use crate::mir::builder::control_flow::plan::recipe_tree::verified::check_block_contract;

    check_block_contract(
        &recipe.arena,
        &recipe.block,
        BlockContractKind::ExitOnly,
        context,
    )
    .map(|_| ())
    .map_err(|e| Freeze::contract("scan_segment_exit_only_verification_failed").with_hint(&e))
}

pub(crate) fn verify_loop_cond_break_continue_recipe_items(
    recipe: &crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueRecipe,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_cond::break_continue_recipe::LoopCondBreakContinueItem;

    let body_len = recipe.body.len();
    for (idx, item) in recipe.items.iter().enumerate() {
        let item_ctx = format!("{context}_item_{idx}");
        match item {
            LoopCondBreakContinueItem::Stmt(stmt) => {
                verify_stmt_ref_in_bounds(*stmt, body_len, &item_ctx)?;
            }
            LoopCondBreakContinueItem::ProgramBlock { stmt, stmt_only } => {
                verify_stmt_ref_in_bounds(*stmt, body_len, &item_ctx)?;
                if let Some(stmt_only) = stmt_only {
                    verify_stmt_only_block_recipe(stmt_only, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ExitIf { if_stmt, block } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                if let Some(block) = block {
                    verify_exit_allowed_block_recipe(block, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ContinueIfWithElse {
                if_stmt,
                continue_prelude,
                fallthrough_body,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                if let Some(continue_prelude) = continue_prelude {
                    verify_no_exit_block_recipe(continue_prelude, &item_ctx)?;
                }
                if let Some(fallthrough_body) = fallthrough_body {
                    verify_no_exit_block_recipe(fallthrough_body, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ConditionalUpdateIf {
                if_stmt,
                then_body,
                else_body,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                if let Some(then_body) = then_body {
                    verify_no_exit_block_recipe(then_body, &item_ctx)?;
                }
                if let Some(else_body) = else_body {
                    verify_no_exit_block_recipe(else_body, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::GeneralIf(block) => {
                verify_no_exit_block_recipe(block, &item_ctx)?;
            }
            LoopCondBreakContinueItem::NestedLoopDepth1 { loop_stmt, nested } => {
                verify_stmt_ref_in_bounds(*loop_stmt, body_len, &item_ctx)?;
                if let Some(body) = nested.body.as_ref() {
                    verify_stmt_only_block_recipe(body, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::TailBreak { block } => {
                if let Some(block) = block {
                    verify_exit_allowed_block_recipe(block, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ElseOnlyReturnIf {
                if_stmt,
                then_no_exit,
                else_return_stmt,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                verify_if_branch_ref(&recipe.body, *if_stmt, "else", *else_return_stmt, &item_ctx)?;
                if let Some(then_no_exit) = then_no_exit {
                    verify_no_exit_block_recipe(then_no_exit, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ThenOnlyReturnIf {
                if_stmt,
                then_return_stmt,
                else_no_exit,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                verify_if_branch_ref(&recipe.body, *if_stmt, "then", *then_return_stmt, &item_ctx)?;
                if let Some(else_no_exit) = else_no_exit {
                    verify_no_exit_block_recipe(else_no_exit, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ElseOnlyBreakIf {
                if_stmt,
                then_no_exit,
                else_break_stmt,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                verify_if_branch_ref(&recipe.body, *if_stmt, "else", *else_break_stmt, &item_ctx)?;
                if let Some(then_no_exit) = then_no_exit {
                    verify_no_exit_block_recipe(then_no_exit, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ThenOnlyBreakIf {
                if_stmt,
                then_break_stmt,
                else_no_exit,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                verify_if_branch_ref(&recipe.body, *if_stmt, "then", *then_break_stmt, &item_ctx)?;
                if let Some(else_no_exit) = else_no_exit {
                    verify_no_exit_block_recipe(else_no_exit, &item_ctx)?;
                }
            }
            LoopCondBreakContinueItem::ElseGuardBreakIf {
                if_stmt,
                then_no_exit,
                then_body,
                else_body,
                else_exit_allowed,
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                if let Some(then_no_exit) = then_no_exit {
                    verify_no_exit_block_recipe(then_no_exit, &item_ctx)?;
                }
                if let Some(else_exit_allowed) = else_exit_allowed {
                    verify_exit_allowed_block_recipe(else_exit_allowed, &item_ctx)?;
                }
                verify_loop_cond_break_continue_recipe_items(then_body, &item_ctx)?;
                verify_loop_cond_break_continue_recipe_items(else_body, &item_ctx)?;
            }
            LoopCondBreakContinueItem::ExitLeaf { stmt, .. } => {
                verify_stmt_ref_in_bounds(*stmt, body_len, &item_ctx)?;
            }
            LoopCondBreakContinueItem::ExitIfTree {
                if_stmt,
                then_body,
                else_body,
                ..
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, body_len, &item_ctx)?;
                verify_exit_only_block_recipe(then_body, &item_ctx)?;
                if let Some(else_body) = else_body {
                    verify_exit_only_block_recipe(else_body, &item_ctx)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn verify_if_branch_ref(
    body: &crate::mir::builder::control_flow::plan::recipes::RecipeBody,
    if_stmt: crate::mir::builder::control_flow::plan::recipes::refs::StmtRef,
    branch: &str,
    stmt: crate::mir::builder::control_flow::plan::recipes::refs::StmtRef,
    context: &str,
) -> Result<(), Freeze> {
    use crate::ast::ASTNode;

    let if_node = body.get_ref(if_stmt).ok_or_else(|| {
        Freeze::contract(&format!(
            "recipe index out of bounds: ctx={context} idx={} len={}",
            if_stmt.index(),
            body.len()
        ))
    })?;
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = if_node
    else {
        return Err(Freeze::contract(&format!(
            "recipe if_stmt is not If: ctx={context}"
        )));
    };
    let branch_len = match branch {
        "then" => then_body.len(),
        "else" => else_body.as_ref().map(|b| b.len()).unwrap_or(0),
        _ => 0,
    };
    if stmt.index() >= branch_len {
        return Err(Freeze::contract(&format!(
            "recipe branch index out of bounds: ctx={context} branch={branch} idx={} len={}",
            stmt.index(),
            branch_len
        )));
    }
    Ok(())
}

pub(crate) fn verify_continue_only_recipe(
    recipe: &crate::mir::builder::control_flow::plan::loop_cond::continue_only_recipe::ContinueOnlyRecipe,
    context: &str,
) -> Result<(), Freeze> {
    let body_len = recipe.body.len();
    verify_continue_only_items_with_len(&recipe.items, body_len, context)
}

pub(crate) fn verify_continue_only_items_with_len(
    items: &[crate::mir::builder::control_flow::plan::loop_cond::continue_only_recipe::ContinueOnlyStmtRecipe],
    len: usize,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_cond::continue_only_recipe::ContinueOnlyStmtRecipe;

    for (idx, item) in items.iter().enumerate() {
        let item_ctx = format!("{context}_item_{idx}");
        match item {
            ContinueOnlyStmtRecipe::Stmt(stmt) => {
                verify_stmt_ref_in_bounds(*stmt, len, &item_ctx)?;
            }
            ContinueOnlyStmtRecipe::ContinueIf { if_stmt, prelude_span } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
                let _ = verify_stmt_span_len(*prelude_span, &item_ctx)?;
            }
            ContinueOnlyStmtRecipe::ContinueIfGroupPrelude {
                if_stmt,
                prelude_span,
                prelude_items,
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
                let prelude_len = verify_stmt_span_len(*prelude_span, &item_ctx)?;
                verify_continue_only_items_with_len(prelude_items, prelude_len, &item_ctx)?;
            }
            ContinueOnlyStmtRecipe::GroupIf {
                if_stmt,
                then_body,
                else_body,
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
                verify_continue_only_recipe(then_body, &item_ctx)?;
                if let Some(else_body) = else_body {
                    verify_continue_only_recipe(else_body, &item_ctx)?;
                }
            }
            ContinueOnlyStmtRecipe::ContinueIfNestedLoop {
                if_stmt,
                inner_loop_prelude_span,
                inner_loop_prelude_items,
                inner_loop_body: _,
                inner_loop_stmt,
                inner_loop_postlude_span,
                inner_loop_postlude_items,
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
                verify_stmt_ref_in_bounds(*inner_loop_stmt, len, &item_ctx)?;
                let prelude_len = verify_stmt_span_len(*inner_loop_prelude_span, &item_ctx)?;
                verify_continue_only_items_with_len(inner_loop_prelude_items, prelude_len, &item_ctx)?;
                let postlude_len = verify_stmt_span_len(*inner_loop_postlude_span, &item_ctx)?;
                verify_continue_only_items_with_len(inner_loop_postlude_items, postlude_len, &item_ctx)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn verify_continue_with_return_recipe(
    recipe: &crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnRecipe,
    context: &str,
) -> Result<(), Freeze> {
    let body_len = recipe.body.len();
    verify_continue_with_return_items_with_len(&recipe.items, body_len, context)
}

pub(crate) fn verify_continue_with_return_items_with_len(
    items: &[crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem],
    len: usize,
    context: &str,
) -> Result<(), Freeze> {
    use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_recipe::ContinueWithReturnItem;

    for (idx, item) in items.iter().enumerate() {
        let item_ctx = format!("{context}_item_{idx}");
        match item {
            ContinueWithReturnItem::Stmt(stmt) => {
                verify_stmt_ref_in_bounds(*stmt, len, &item_ctx)?;
            }
            ContinueWithReturnItem::ContinueIf {
                if_stmt,
                prelude_span,
                prelude_items,
            } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
                let prelude_len = verify_stmt_span_len(*prelude_span, &item_ctx)?;
                verify_continue_with_return_items_with_len(prelude_items, prelude_len, &item_ctx)?;
            }
            ContinueWithReturnItem::HeteroReturnIf { if_stmt } => {
                verify_stmt_ref_in_bounds(*if_stmt, len, &item_ctx)?;
            }
            ContinueWithReturnItem::IfAny(stmt) => {
                verify_stmt_ref_in_bounds(*stmt, len, &item_ctx)?;
            }
        }
    }
    Ok(())
}

pub(crate) fn build_split_scan_loop_condition(
    facts: &crate::mir::builder::control_flow::plan::facts::loop_types::SplitScanFacts,
) -> crate::ast::ASTNode {
    use crate::ast::{ASTNode, BinaryOperator, Span};

    let span = Span::unknown();
    let left = ASTNode::Variable {
        name: facts.i_var.clone(),
        span,
    };
    let s_len = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.s_var.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    let sep_len = ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: facts.sep_var.clone(),
            span,
        }),
        method: "length".to_string(),
        arguments: vec![],
        span,
    };
    let bound = ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left: Box::new(s_len),
        right: Box::new(sep_len),
        span,
    };

    ASTNode::BinaryOp {
        operator: BinaryOperator::LessEqual,
        left: Box::new(left),
        right: Box::new(bound),
        span,
    }
}
