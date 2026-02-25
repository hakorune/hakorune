//! Phase 29ca P1: Generic structured loop v0 facts (ExitIf-capable, no carriers)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::generic_loop::canon::canon_condition_for_generic_loop_v0;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    extract_loop_increment_plan,
};
use crate::mir::builder::control_flow::plan::coreloop_body_contract::is_effect_only_stmt;
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV0Facts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopV1Facts {
    pub loop_var: String,
    pub condition: ASTNode,
    pub loop_increment: ASTNode,
    pub body: Vec<ASTNode>,
}

pub(in crate::mir::builder) fn try_extract_generic_loop_v0_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<GenericLoopV0Facts>, Freeze> {
    let flat_body = flatten_scope_boxes(body);
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let debug = crate::config::env::is_joinir_debug();

    let Some(canon) = canon_condition_for_generic_loop_v0(condition) else {
        if debug {
            eprintln!("[generic_loop_v0] reject: condition_not_subset");
        }
        return Ok(None);
    };

    let mut matches = Vec::new();
    for loop_var in &canon.loop_var_candidates {
        let loop_increment = match extract_loop_increment_plan(&flat_body, loop_var) {
            Ok(Some(inc)) => inc,
            _ => continue,
        };

        let step_placement =
            match classify_step_placement(&flat_body, loop_var, &loop_increment, strict)? {
                Some(placement) => placement,
                None => continue,
            };

        let step_ok = match step_placement {
            StepPlacement::InBody(idx) => {
                validate_in_body_step(&flat_body, idx, loop_var, &loop_increment, strict)?
            }
            StepPlacement::InContinueIf(idx) => validate_continue_if_step(&flat_body, idx, strict)?,
            StepPlacement::InBreakElseIf(idx) => validate_break_else_if_step(&flat_body, idx, strict)?,
            _ => true,
        };
        if !step_ok {
            continue;
        }

        if !body_is_generic_v1(&flat_body, loop_var, &loop_increment) {
            continue;
        }

        matches.push(GenericLoopV0Facts {
            loop_var: loop_var.clone(),
            condition: condition.clone(),
            loop_increment,
            body: flat_body.clone(),
        });
    }

    if matches.is_empty() {
        if debug {
            eprintln!(
                "[generic_loop_v0] reject: no_valid_loop_var_candidates candidates={:?}",
                canon.loop_var_candidates
            );
        }
        return Ok(None);
    }

    if matches.len() > 1 {
        return reject_or_none(
            strict,
            "generic loop v0.3: multiple loop_var candidates matched (ambiguous)",
        );
    }

    Ok(Some(matches.remove(0)))
}

pub(in crate::mir::builder) fn try_extract_generic_loop_v1_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<GenericLoopV1Facts>, Freeze> {
    let flat_body = flatten_scope_boxes(body);
    let strict = crate::config::env::joinir_dev::strict_enabled();
    let debug = crate::config::env::is_joinir_debug();

    let Some(canon) = canon_condition_for_generic_loop_v0(condition) else {
        if debug {
            eprintln!("[generic_loop_v1] reject: condition_not_subset");
        }
        return Ok(None);
    };

    let mut matches = Vec::new();
    for loop_var in &canon.loop_var_candidates {
        let loop_increment = match extract_loop_increment_plan(&flat_body, loop_var) {
            Ok(Some(inc)) => inc,
            _ => continue,
        };

        let step_placement =
            match classify_step_placement(&flat_body, loop_var, &loop_increment, strict)? {
                Some(placement) => placement,
                None => continue,
            };

        let step_ok = match step_placement {
            StepPlacement::InBody(idx) => {
                validate_in_body_step_v1(&flat_body, idx, loop_var, &loop_increment, strict)?
            }
            StepPlacement::InContinueIf(idx) => validate_continue_if_step(&flat_body, idx, strict)?,
            StepPlacement::InBreakElseIf(idx) => validate_break_else_if_step(&flat_body, idx, strict)?,
            _ => true,
        };
        if !step_ok {
            continue;
        }

        if !body_is_generic_v0(&flat_body, loop_var, &loop_increment) {
            continue;
        }

        matches.push(GenericLoopV1Facts {
            loop_var: loop_var.clone(),
            condition: condition.clone(),
            loop_increment,
            body: flat_body.clone(),
        });
    }

    if matches.is_empty() {
        if debug {
            eprintln!(
                "[generic_loop_v1] reject: no_valid_loop_var_candidates candidates={:?}",
                canon.loop_var_candidates
            );
        }
        return Ok(None);
    }

    if matches.len() > 1 {
        return reject_or_none(
            strict,
            "generic loop v1: multiple loop_var candidates matched (ambiguous)",
        );
    }

    Ok(Some(matches.remove(0)))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum StepPlacement {
    Last,
    InBody(usize),
    InContinueIf(usize),
    InBreakElseIf(usize),
}

fn classify_step_placement(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    strict: bool,
) -> Result<Option<StepPlacement>, Freeze> {
    let mut indices = Vec::new();
    for (idx, stmt) in body.iter().enumerate() {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            indices.push(idx);
        }
    }
    if indices.is_empty() {
        let mut continue_indices = Vec::new();
        let mut break_else_indices = Vec::new();
        for (idx, stmt) in body.iter().enumerate() {
            if is_continue_if_with_increment(stmt, loop_var, loop_increment) {
                continue_indices.push(idx);
            }
            if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
                break_else_indices.push(idx);
            }
        }
        if continue_indices.is_empty() && break_else_indices.is_empty() {
            return Ok(None);
        }
        if continue_indices.len() + break_else_indices.len() > 1 {
            return reject_or_none(
                strict,
                "generic loop v0.2: multiple conditional step assignments in body",
            );
        }
        if let Some(idx) = continue_indices.first().copied() {
            return Ok(Some(StepPlacement::InContinueIf(idx)));
        }
        if let Some(idx) = break_else_indices.first().copied() {
            return Ok(Some(StepPlacement::InBreakElseIf(idx)));
        }
        return Ok(None);
    }
    if indices.len() > 1 {
        return reject_or_none(
            strict,
            "generic loop v0.2: multiple step assignments in body",
        );
    }
    let idx = indices[0];
    if idx + 1 == body.len() {
        Ok(Some(StepPlacement::Last))
    } else {
        Ok(Some(StepPlacement::InBody(idx)))
    }
}

fn validate_in_body_step(
    body: &[ASTNode],
    step_index: usize,
    loop_var: &str,
    loop_increment: &ASTNode,
    strict: bool,
) -> Result<bool, Freeze> {
    if body_has_continue(body) {
        return reject_or_false(
            strict,
            "generic loop v0.2: in-body step + continue is not allowed",
        );
    }
    for stmt in body.iter().skip(step_index + 1) {
        if is_exit_if(stmt) || is_effect_if(stmt, loop_var, loop_increment) {
            return reject_or_false(
                strict,
                "generic loop v0.2: control flow after in-body step",
            );
        }
        if matches!(stmt, ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. })
        {
            return reject_or_false(
                strict,
                "generic loop v0.2: exit after in-body step",
            );
        }
        if stmt_uses_loop_var(stmt, loop_var) {
            return reject_or_false(
                strict,
                "generic loop v0.2: loop var used after in-body step",
            );
        }
        if is_simple_assignment(stmt, loop_var)
            || is_local_init(stmt, loop_var)
            || is_effect_only_stmt(stmt)
        {
            continue;
        }
        return reject_or_false(
            strict,
            "generic loop v0.2: unsupported stmt after in-body step",
        );
    }
    Ok(true)
}

fn validate_in_body_step_v1(
    body: &[ASTNode],
    step_index: usize,
    loop_var: &str,
    loop_increment: &ASTNode,
    strict: bool,
) -> Result<bool, Freeze> {
    for stmt in body.iter().skip(step_index + 1) {
        if is_exit_if(stmt) || is_effect_if(stmt, loop_var, loop_increment) {
            return reject_or_false(
                strict,
                "generic loop v1: control flow after in-body step",
            );
        }
        if matches!(stmt, ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. })
        {
            return reject_or_false(
                strict,
                "generic loop v1: exit after in-body step",
            );
        }
        if stmt_uses_loop_var(stmt, loop_var) {
            return reject_or_false(
                strict,
                "generic loop v1: loop var used after in-body step",
            );
        }
        if is_simple_assignment(stmt, loop_var)
            || is_local_init(stmt, loop_var)
            || is_effect_only_stmt(stmt)
        {
            continue;
        }
        return reject_or_false(
            strict,
            "generic loop v1: unsupported stmt after in-body step",
        );
    }
    Ok(true)
}

fn validate_continue_if_step(
    body: &[ASTNode],
    step_index: usize,
    strict: bool,
) -> Result<bool, Freeze> {
    let tail = if step_index + 1 >= body.len() {
        &[]
    } else {
        &body[step_index + 1..]
    };
    if tail.is_empty() {
        return Ok(true);
    }
    if tail.len() == 1
        && matches!(tail[0], ASTNode::Break { .. } | ASTNode::Return { .. })
    {
        return Ok(true);
    }
    reject_or_false(
        strict,
        "generic loop v0.2: continue-if step requires trailing break/return",
    )
}

fn is_continue_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() {
        return false;
    }
    if !matches!(then_body.last(), Some(ASTNode::Continue { .. })) {
        return false;
    }
    let mut saw_increment = false;
    for inner in then_body {
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if saw_increment {
                return false;
            }
            saw_increment = true;
        }
    }
    saw_increment
}

fn validate_break_else_if_step(
    body: &[ASTNode],
    step_index: usize,
    strict: bool,
) -> Result<bool, Freeze> {
    if step_index + 1 == body.len() {
        return Ok(true);
    }
    reject_or_false(
        strict,
        "generic loop v0.2: break-else step must be final stmt",
    )
}

fn is_break_else_if_with_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    let Some(else_body) = else_body else {
        return false;
    };
    if else_body.len() != 1 || !matches!(else_body[0], ASTNode::Break { .. }) {
        return false;
    }
    if then_body.is_empty() {
        return false;
    }
    let mut saw_increment = false;
    for inner in then_body {
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if saw_increment {
                return false;
            }
            saw_increment = true;
            continue;
        }
        return false;
    }
    saw_increment
}

fn body_is_generic_v0(body: &[ASTNode], loop_var: &str, loop_increment: &ASTNode) -> bool {
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_simple_assignment(stmt, loop_var) {
            continue;
        }
        if is_local_init(stmt, loop_var) {
            continue;
        }
        if is_exit_if(stmt) {
            continue;
        }
        if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if matches!(
            stmt,
            ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::Loop { .. }
        ) {
            continue;
        }
        return false;
    }
    true
}

fn body_is_generic_v1(body: &[ASTNode], loop_var: &str, loop_increment: &ASTNode) -> bool {
    for stmt in body {
        if matches_loop_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_simple_assignment(stmt, loop_var) {
            continue;
        }
        if is_local_init(stmt, loop_var) {
            continue;
        }
        if is_exit_if(stmt) {
            continue;
        }
        if is_continue_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_if_with_increment(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_break_else_effect_if(stmt, loop_var, loop_increment) {
            continue;
        }
        if is_effect_if_pure(stmt) {
            continue;
        }
        if is_conditional_update_if(stmt, loop_var) {
            continue;
        }
        if matches!(
            stmt,
            ASTNode::Break { .. }
                | ASTNode::Continue { .. }
                | ASTNode::Return { .. }
                | ASTNode::MethodCall { .. }
                | ASTNode::FunctionCall { .. }
                | ASTNode::Loop { .. }
        ) {
            continue;
        }
        return false;
    }
    true
}

fn is_break_else_effect_if(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !is_supported_bool_expr(condition) {
        return false;
    }
    if then_body.len() != 1 || !matches!(then_body[0], ASTNode::Break { .. }) {
        return false;
    }
    let Some(else_body) = else_body else {
        return false;
    };
    if else_body.is_empty() {
        return false;
    }
    else_body
        .iter()
        .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment))
}

fn matches_loop_increment(
    stmt: &ASTNode,
    loop_var: &str,
    loop_increment: &ASTNode,
) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name != loop_var {
        return false;
    }
    value.as_ref() == loop_increment
}

fn is_simple_assignment(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Assignment { target, value, .. } = stmt else {
        return false;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return false;
    };
    if name == loop_var {
        return false;
    }
    is_supported_value_expr(value)
}

fn is_local_init(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return false;
    };
    if variables.len() != initial_values.len() {
        return false;
    }
    for (name, init) in variables.iter().zip(initial_values.iter()) {
        if name == loop_var {
            return false;
        }
        let Some(init) = init.as_ref() else {
            return false;
        };
        if !is_supported_value_expr(init) {
            return false;
        }
    }
    true
}

fn is_effect_stmt(stmt: &ASTNode, loop_var: &str, loop_increment: &ASTNode) -> bool {
    if is_simple_assignment(stmt, loop_var) || is_local_init(stmt, loop_var) {
        return true;
    }
    if is_effect_only_stmt(stmt) {
        return true;
    }
    if is_effect_if(stmt, loop_var, loop_increment) {
        return true;
    }
    if is_break_else_effect_if(stmt, loop_var, loop_increment) {
        return true;
    }
    false
}

fn is_exit_if(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if else_body.is_some() || then_body.len() != 1 {
        return false;
    }
    if !is_supported_bool_expr(condition) {
        return false;
    }
    matches!(
        then_body[0],
        ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Return { .. }
    )
}

fn is_effect_if(stmt: &ASTNode, loop_var: &str, loop_increment: &ASTNode) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if then_body.is_empty() {
        return false;
    }
    if !is_supported_bool_expr(condition) {
        return false;
    }

    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        return then_body
            .iter()
            .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment))
            && else_body
                .iter()
                .all(|stmt| is_effect_stmt(stmt, loop_var, loop_increment));
    }

    let last_is_continue = matches!(then_body.last(), Some(ASTNode::Continue { .. }));
    let mut saw_increment = false;

    for (idx, inner) in then_body.iter().enumerate() {
        let is_last = idx + 1 == then_body.len();
        if last_is_continue && is_last && matches!(inner, ASTNode::Continue { .. }) {
            continue;
        }
        if matches_loop_increment(inner, loop_var, loop_increment) {
            if !last_is_continue || idx + 2 != then_body.len() || saw_increment {
                return false;
            }
            saw_increment = true;
            continue;
        }
        if is_effect_stmt(inner, loop_var, loop_increment) {
            continue;
        }
        return false;
    }

    true
}

fn is_effect_if_pure(stmt: &ASTNode) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if then_body.is_empty() {
        return false;
    }
    if !is_supported_bool_expr(condition) {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.is_empty() {
            return false;
        }
        return then_body.iter().all(is_effect_only_stmt)
            && else_body.iter().all(is_effect_only_stmt);
    }
    then_body.iter().all(is_effect_only_stmt)
}

fn is_conditional_update_if(stmt: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return false;
    };
    if !is_supported_bool_expr(condition) {
        return false;
    }

    let mut saw_assignment = false;
    if !is_conditional_update_branch(then_body, loop_var, &mut saw_assignment) {
        return false;
    }
    if let Some(else_body) = else_body {
        if !is_conditional_update_branch(else_body, loop_var, &mut saw_assignment) {
            return false;
        }
    }

    saw_assignment
}

fn is_conditional_update_branch(
    body: &[ASTNode],
    loop_var: &str,
    saw_assignment: &mut bool,
) -> bool {
    let mut saw_exit = false;
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                if name == loop_var {
                    return false;
                }
                if !is_pure_value_expr(value) {
                    return false;
                }
                *saw_assignment = true;
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last {
                    return false;
                }
                if saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    true
}

fn is_supported_value_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall { .. } => true,
        ASTNode::BinaryOp { operator, left: _, right: _, .. } => matches!(
            operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
        ),
        _ => false,
    }
}

pub(super) fn is_pure_value_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr(operand),
        ASTNode::BinaryOp { operator, left, right, .. } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr(left)
                && is_pure_value_expr(right)
        }
        _ => false,
    }
}

fn is_supported_bool_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr(left) && is_supported_bool_expr(right)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr(left) && is_supported_value_expr(right)
            }
            _ => false,
        },
        _ => false,
    }
}

fn body_has_continue(body: &[ASTNode]) -> bool {
    body.iter().any(stmt_has_continue)
}

fn stmt_has_continue(stmt: &ASTNode) -> bool {
    match stmt {
        ASTNode::Continue { .. } => true,
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            then_body.iter().any(stmt_has_continue)
                || else_body.as_ref().map_or(false, |body| {
                    body.iter().any(stmt_has_continue)
                })
        }
        ASTNode::ScopeBox { body, .. } => body.iter().any(stmt_has_continue),
        _ => false,
    }
}

fn stmt_uses_loop_var(stmt: &ASTNode, loop_var: &str) -> bool {
    match stmt {
        ASTNode::Variable { name, .. } => name == loop_var,
        ASTNode::Assignment { target, value, .. } => {
            stmt_uses_loop_var(target, loop_var) || stmt_uses_loop_var(value, loop_var)
        }
        ASTNode::BinaryOp { left, right, .. } => {
            stmt_uses_loop_var(left, loop_var) || stmt_uses_loop_var(right, loop_var)
        }
        ASTNode::UnaryOp { operand, .. } => stmt_uses_loop_var(operand, loop_var),
        ASTNode::MethodCall {
            object,
            arguments,
            ..
        } => {
            stmt_uses_loop_var(object, loop_var)
                || arguments.iter().any(|arg| stmt_uses_loop_var(arg, loop_var))
        }
        ASTNode::FunctionCall { arguments, .. } => {
            arguments
                .iter()
                .any(|arg| stmt_uses_loop_var(arg, loop_var))
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            stmt_uses_loop_var(condition, loop_var)
                || then_body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
                || else_body.as_ref().map_or(false, |body| {
                    body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
                })
        }
        ASTNode::Local {
            initial_values,
            ..
        } => initial_values.iter().any(|init| {
            init.as_ref()
                .map_or(false, |node| stmt_uses_loop_var(node, loop_var))
        }),
        ASTNode::Return { value, .. } => value
            .as_ref()
            .map_or(false, |node| stmt_uses_loop_var(node, loop_var)),
        ASTNode::Loop { condition, body, .. } => {
            stmt_uses_loop_var(condition, loop_var)
                || body.iter().any(|node| stmt_uses_loop_var(node, loop_var))
        }
        ASTNode::ScopeBox { body, .. } => body.iter().any(|node| stmt_uses_loop_var(node, loop_var)),
        ASTNode::Break { .. }
        | ASTNode::Continue { .. }
        | ASTNode::Literal { .. } => false,
        _ => true,
    }
}

fn flatten_scope_boxes(body: &[ASTNode]) -> Vec<ASTNode> {
    let mut out = Vec::new();
    fn push_node(node: &ASTNode, out: &mut Vec<ASTNode>) {
        match node {
            ASTNode::ScopeBox { body, .. } => {
                for inner in body {
                    push_node(inner, out);
                }
            }
            _ => out.push(node.clone()),
        }
    }
    for stmt in body {
        push_node(stmt, &mut out);
    }
    out
}

fn reject_or_none<T>(strict: bool, message: &str) -> Result<Option<T>, Freeze> {
    if strict {
        Err(Freeze::ambiguous(message))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn lit_i(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: Span::unknown(),
        }
    }

    fn bin(op: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    fn assign(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(var(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn generic_loop_v0_allows_loop_var_from_add_expr_in_condition() {
        let cond = bin(
            BinaryOperator::LessEqual,
            bin(BinaryOperator::Add, var("j"), var("m")),
            var("n"),
        );
        let body = vec![assign("j", bin(BinaryOperator::Add, var("j"), lit_i(1)))];

        let facts = try_extract_generic_loop_v0_facts(&cond, &body)
            .expect("no freeze")
            .expect("should match");
        assert_eq!(facts.loop_var, "j");
    }
}

fn reject_or_false(strict: bool, message: &str) -> Result<bool, Freeze> {
    if strict {
        Err(Freeze::unsupported(message))
    } else {
        Ok(false)
    }
}
