//! LoopTrueCounterExtractorBox - loop(true) からの loop counter 抽出（Pattern2）
//!
//! 目的: `loop(true)` のように header condition から loop counter を抽出できないケースで、
//! body から loop counter（例: `i`）を一意に確定する。
//!
//! ## 責務（SSOT）
//! - Pattern2 の「loop(true) を扱う入口」を1箇所に集約する
//! - 曖昧な loop(true) を **通さない**（Fail-Fast で理由を返す）
//!
//! ## Contract（Fail-Fast）
//! 許可（loop(true) 系で必要な最小）:
//! - カウンタ候補が **ちょうど1つ**
//! - 更新が `i = i + 1` 形（定数 1 のみ） **または**
//!   `i = j + K` 形（`j = s.indexOf(..., i)` 由来、K は整数定数）
//! - `substring(i, ...)` が body のどこかに存在（誤マッチ防止）
//! - `i` が loop-outer var（`variable_map` に存在）である
//!
//! 禁止:
//! - 候補なし / 複数候補 / 更新形が曖昧（+1 以外）/ substring 形が無い
//! - loop-outer でない（variable_map にいない）

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::ValueId;
use std::collections::BTreeMap;

pub(crate) struct LoopTrueCounterExtractorBox;

impl LoopTrueCounterExtractorBox {
    pub(crate) fn is_loop_true(condition: &ASTNode) -> bool {
        matches!(
            condition,
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                ..
            }
        )
    }

    /// Extract a unique loop counter variable from loop(true) body.
    ///
    /// Current supported shape (minimal):
    /// - There exists an assignment `i = i + 1` somewhere in the body (including nested if).
    /// - There exists a substring read using that counter: `s.substring(i, i + 1)` (same `i`).
    ///
    /// Fail-Fast (returns Err) when:
    /// - No counter candidate found
    /// - Multiple different candidates found
    /// - Candidate not found in `variable_map` (loop-outer var required)
    /// - Substring pattern not found (guards against accidental matches)
    pub(crate) fn extract_loop_counter_from_body(
        body: &[ASTNode],
        variable_map: &BTreeMap<String, ValueId>,
    ) -> Result<(String, ValueId), String> {
        let mut candidates: Vec<String> = Vec::new();

        fn walk(node: &ASTNode, out: &mut Vec<String>) {
            match node {
                ASTNode::Assignment { target, value, .. } => {
                    if let (Some(name), true) = (
                        extract_var_name(target.as_ref()),
                        is_self_plus_const_one(value.as_ref(), target.as_ref()),
                    ) {
                        out.push(name);
                    }
                }
                ASTNode::If {
                    condition: _,
                    then_body,
                    else_body,
                    ..
                } => {
                    for s in then_body {
                        walk(s, out);
                    }
                    if let Some(eb) = else_body {
                        for s in eb {
                            walk(s, out);
                        }
                    }
                }
                ASTNode::Loop { body, .. } => {
                    for s in body {
                        walk(s, out);
                    }
                }
                _ => {}
            }
        }

        fn is_self_plus_const_one(value: &ASTNode, target: &ASTNode) -> bool {
            let target_name = match extract_var_name(target) {
                Some(n) => n,
                None => return false,
            };
            match value {
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left,
                    right,
                    ..
                } => {
                    let left_is_var = matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == &target_name);
                    let right_is_one = matches!(
                        right.as_ref(),
                        ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            ..
                        }
                    );
                    left_is_var && right_is_one
                }
                _ => false,
            }
        }

        for stmt in body {
            walk(stmt, &mut candidates);
        }

        candidates.sort();
        candidates.dedup();

        if candidates.len() > 1 {
            return Err(format!(
                "[loop_break/loop_true_counter/contract/multiple_candidates] Multiple counter candidates found in loop(true) body: {:?}",
                candidates
            ));
        }

        if candidates.len() == 1 {
            let loop_var_name = candidates[0].clone();
            let host_id = variable_map.get(&loop_var_name).copied().ok_or_else(|| {
                format!(
                    "[loop_break/loop_true_counter/contract/not_loop_outer] Counter '{}' not found in variable_map (loop-outer var required)",
                    loop_var_name
                )
            })?;

            if !has_substring_read(body, &loop_var_name) {
                return Err(format!(
                    "[loop_break/loop_true_counter/contract/missing_substring_guard] Counter '{}' found, but missing substring pattern `s.substring({}, {} + 1)`",
                    loop_var_name, loop_var_name, loop_var_name
                ));
            }

            return Ok((loop_var_name, host_id));
        }

        if let Some((loop_var_name, host_id)) =
            extract_loop_counter_from_indexof_pattern(body, variable_map)?
        {
            return Ok((loop_var_name, host_id));
        }

        Err(
            "[loop_break/loop_true_counter/contract/no_candidate] Cannot find unique counter update `i = i + 1` in loop(true) body"
                .to_string(),
        )
    }
}

fn extract_var_name(n: &ASTNode) -> Option<String> {
    match n {
        ASTNode::Variable { name, .. } => Some(name.clone()),
        _ => None,
    }
}

fn extract_loop_counter_from_indexof_pattern(
    body: &[ASTNode],
    variable_map: &BTreeMap<String, ValueId>,
) -> Result<Option<(String, ValueId)>, String> {
    let indexof_bindings = collect_indexof_bindings(body);
    if indexof_bindings.is_empty() {
        return Ok(None);
    }

    let mut candidates: Vec<String> = Vec::new();

    fn walk_assign(
        node: &ASTNode,
        indexof_bindings: &[(String, String)],
        candidates: &mut Vec<String>,
    ) {
        match node {
            ASTNode::Assignment { target, value, .. } => {
                if let (Some(target_name), Some((index_var, const_val))) =
                    (extract_var_name(target.as_ref()), extract_add_var_const(value.as_ref()))
                {
                    if const_val <= 0 {
                        return;
                    }
                    if indexof_bindings.iter().any(|(idx_var, start_var)| {
                        idx_var == &index_var && start_var == &target_name
                    }) {
                        candidates.push(target_name);
                    }
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    walk_assign(s, indexof_bindings, candidates);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        walk_assign(s, indexof_bindings, candidates);
                    }
                }
            }
            ASTNode::Loop { body, .. } => {
                for s in body {
                    walk_assign(s, indexof_bindings, candidates);
                }
            }
            _ => {}
        }
    }

    for stmt in body {
        walk_assign(stmt, &indexof_bindings, &mut candidates);
    }

    candidates.sort();
    candidates.dedup();

    if candidates.len() > 1 {
        return Err(format!(
            "[loop_break/loop_true_counter/contract/multiple_candidates] Multiple counter candidates found in loop(true) body: {:?}",
            candidates
        ));
    }

    if candidates.len() == 1 {
        let loop_var_name = candidates[0].clone();
        let host_id = variable_map.get(&loop_var_name).copied().ok_or_else(|| {
            format!(
                "[loop_break/loop_true_counter/contract/not_loop_outer] Counter '{}' not found in variable_map (loop-outer var required)",
                loop_var_name
            )
        })?;

        if !has_substring_read_with_start(body, &loop_var_name) {
            return Err(format!(
                "[loop_break/loop_true_counter/contract/missing_substring_guard] Counter '{}' found, but missing substring pattern `substring({}, ...)`",
                loop_var_name, loop_var_name
            ));
        }

        return Ok(Some((loop_var_name, host_id)));
    }

    Ok(None)
}

fn collect_indexof_bindings(body: &[ASTNode]) -> Vec<(String, String)> {
    fn extract_indexof_binding(node: &ASTNode) -> Option<(String, String)> {
        let (target_name, value_node) = match node {
            ASTNode::Local {
                variables,
                initial_values,
                ..
            } => {
                if variables.len() != 1 {
                    return None;
                }
                let value = initial_values.get(0).and_then(|v| v.as_ref())?;
                (variables[0].clone(), value.as_ref())
            }
            ASTNode::Assignment { target, value, .. } => {
                let target_name = extract_var_name(target.as_ref())?;
                (target_name, value.as_ref())
            }
            _ => return None,
        };

        if let ASTNode::MethodCall {
            method,
            arguments,
            ..
        } = value_node
        {
            if method == "indexOf" && arguments.len() == 2 {
                if let ASTNode::Variable { name, .. } = &arguments[1] {
                    return Some((target_name, name.clone()));
                }
            }
        }

        None
    }

    fn walk(node: &ASTNode, out: &mut Vec<(String, String)>) {
        if let Some(binding) = extract_indexof_binding(node) {
            out.push(binding);
        }

        match node {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for s in then_body {
                    walk(s, out);
                }
                if let Some(eb) = else_body {
                    for s in eb {
                        walk(s, out);
                    }
                }
            }
            ASTNode::Loop { body, .. } => {
                for s in body {
                    walk(s, out);
                }
            }
            _ => {}
        }
    }

    let mut bindings = Vec::new();
    for stmt in body {
        walk(stmt, &mut bindings);
    }
    bindings
}

fn extract_add_var_const(value: &ASTNode) -> Option<(String, i64)> {
    match value {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } => {
            if let ASTNode::Variable { name, .. } = left.as_ref() {
                if let ASTNode::Literal {
                    value: LiteralValue::Integer(i),
                    ..
                } = right.as_ref()
                {
                    return Some((name.clone(), *i));
                }
            }
            None
        }
        _ => None,
    }
}

fn has_substring_read(body: &[ASTNode], counter: &str) -> bool {
    fn walk(node: &ASTNode, counter: &str) -> bool {
        match node {
            ASTNode::Assignment { value, .. } => walk(value.as_ref(), counter),
            ASTNode::Local {
                initial_values, ..
            } => initial_values
                .iter()
                .filter_map(|v| v.as_ref())
                .any(|v| walk(v.as_ref(), counter)),
            ASTNode::MethodCall {
                object: _,
                method,
                arguments,
                ..
            } => {
                if method == "substring" && arguments.len() == 2 {
                    let a0 = &arguments[0];
                    let a1 = &arguments[1];
                    let a0_ok = matches!(a0, ASTNode::Variable { name, .. } if name == counter);
                    let a1_ok = matches!(
                        a1,
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left,
                            right,
                            ..
                        } if matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == counter)
                            && matches!(right.as_ref(), ASTNode::Literal { value: LiteralValue::Integer(1), .. })
                    );
                    if a0_ok && a1_ok {
                        return true;
                    }
                }
                // Search recursively in args
                arguments.iter().any(|a| walk(a, counter))
            }
            ASTNode::BinaryOp { left, right, .. } => walk(left.as_ref(), counter) || walk(right.as_ref(), counter),
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                walk(condition.as_ref(), counter)
                    || then_body.iter().any(|s| walk(s, counter))
                    || else_body
                        .as_ref()
                        .map(|eb| eb.iter().any(|s| walk(s, counter)))
                        .unwrap_or(false)
            }
            ASTNode::Loop { body, condition, .. } => {
                walk(condition.as_ref(), counter) || body.iter().any(|s| walk(s, counter))
            }
            _ => false,
        }
    }

    body.iter().any(|s| walk(s, counter))
}

fn has_substring_read_with_start(body: &[ASTNode], counter: &str) -> bool {
    fn walk(node: &ASTNode, counter: &str) -> bool {
        match node {
            ASTNode::Assignment { value, .. } => walk(value.as_ref(), counter),
            ASTNode::Local { initial_values, .. } => initial_values
                .iter()
                .filter_map(|v| v.as_ref())
                .any(|v| walk(v.as_ref(), counter)),
            ASTNode::MethodCall {
                method,
                arguments,
                ..
            } => {
                if method == "substring" && arguments.len() == 2 {
                    if matches!(
                        &arguments[0],
                        ASTNode::Variable { name, .. } if name == counter
                    ) {
                        return true;
                    }
                }
                arguments.iter().any(|a| walk(a, counter))
            }
            ASTNode::BinaryOp { left, right, .. } => {
                walk(left.as_ref(), counter) || walk(right.as_ref(), counter)
            }
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                walk(condition.as_ref(), counter)
                    || then_body.iter().any(|s| walk(s, counter))
                    || else_body
                        .as_ref()
                        .map(|eb| eb.iter().any(|s| walk(s, counter)))
                        .unwrap_or(false)
            }
            ASTNode::Loop { body, condition, .. } => {
                walk(condition.as_ref(), counter) || body.iter().any(|s| walk(s, counter))
            }
            _ => false,
        }
    }

    body.iter().any(|s| walk(s, counter))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn lit_i(n: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(n),
            span: span(),
        }
    }

    fn lit_s(s: &str) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::String(s.to_string()),
            span: span(),
        }
    }

    fn add(left: ASTNode, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left: Box::new(left),
            right: Box::new(right),
            span: span(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: span(),
        }
    }

    fn substring_i(s_var: &str, i_var: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(s_var)),
            method: "substring".to_string(),
            arguments: vec![var(i_var), add(var(i_var), lit_i(1))],
            span: span(),
        }
    }

    fn substring_ij(s_var: &str, i_var: &str, j_var: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(s_var)),
            method: "substring".to_string(),
            arguments: vec![var(i_var), var(j_var)],
            span: span(),
        }
    }

    fn indexof(s_var: &str, needle: &str, start_var: &str) -> ASTNode {
        ASTNode::MethodCall {
            object: Box::new(var(s_var)),
            method: "indexOf".to_string(),
            arguments: vec![lit_s(needle), var(start_var)],
            span: span(),
        }
    }

    fn local_one(name: &str, init: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(init))],
            span: span(),
        }
    }

    #[test]
    fn extract_single_candidate_ok() {
        let body = vec![
            local_one("ch", substring_i("s", "i")),
            assign(var("i"), add(var("i"), lit_i(1))),
        ];
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(123));

        let (name, host_id) =
            LoopTrueCounterExtractorBox::extract_loop_counter_from_body(&body, &variable_map)
                .unwrap();
        assert_eq!(name, "i");
        assert_eq!(host_id, ValueId(123));
    }

    #[test]
    fn extract_rejects_no_candidate() {
        let body = vec![local_one("ch", substring_i("s", "i"))];
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(1));
        let err =
            LoopTrueCounterExtractorBox::extract_loop_counter_from_body(&body, &variable_map)
                .unwrap_err();
        assert!(err.contains("no_candidate"));
    }

    #[test]
    fn extract_rejects_multiple_candidates() {
        let body = vec![
            local_one("ch", substring_i("s", "i")),
            assign(var("i"), add(var("i"), lit_i(1))),
            local_one("ch2", substring_i("s", "j")),
            assign(var("j"), add(var("j"), lit_i(1))),
        ];
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(1));
        variable_map.insert("j".to_string(), ValueId(2));
        let err =
            LoopTrueCounterExtractorBox::extract_loop_counter_from_body(&body, &variable_map)
                .unwrap_err();
        assert!(err.contains("multiple_candidates"));
    }

    #[test]
    fn extract_rejects_missing_substring_guard() {
        let body = vec![assign(var("i"), add(var("i"), lit_i(1)))];
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(1));
        let err =
            LoopTrueCounterExtractorBox::extract_loop_counter_from_body(&body, &variable_map)
                .unwrap_err();
        assert!(err.contains("missing_substring_guard"));
    }

    #[test]
    fn extract_indexof_candidate_ok() {
        let body = vec![
            local_one("j", indexof("table", "|||", "i")),
            local_one("seg", substring_ij("table", "i", "j")),
            assign(var("i"), add(var("j"), lit_i(3))),
        ];
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(7));

        let (name, host_id) =
            LoopTrueCounterExtractorBox::extract_loop_counter_from_body(&body, &variable_map)
                .unwrap();
        assert_eq!(name, "i");
        assert_eq!(host_id, ValueId(7));
    }
}
