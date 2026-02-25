use std::collections::BTreeSet;

use crate::mir::ValueId;

/// 推定したループ引数の並び。
#[derive(Debug, Clone)]
pub(crate) struct ParamGuess {
    pub loop_var: (String, ValueId),
    pub acc: (String, ValueId),
    pub len: Option<(String, ValueId)>,
}

/// Break パターン向けのパラメータ推定テーブル。
/// - ループ変数優先順: p → i → 先頭の var
/// - アキュムレータ優先順: num_str → acc → result → ループ変数
pub(crate) fn compute_param_guess(ctx: &super::super::context::ExtractCtx) -> ParamGuess {
    let loop_var = ctx
        .get_var("p")
        .map(|v| ("p".to_string(), v))
        .or_else(|| ctx.get_var("i").map(|v| ("i".to_string(), v)))
        .or_else(|| {
            ctx.var_map
                .iter()
                .find(|(name, _)| name.as_str() != "me")
                .map(|(name, v)| (name.clone(), *v))
        })
        .unwrap_or_else(|| panic!("[joinir/frontend] break_pattern: loop variable missing"));

    let acc = if let Some(v) = ctx.get_var("num_str") {
        ("num_str".to_string(), v)
    } else if let Some(v) = ctx.get_var("acc") {
        ("acc".to_string(), v)
    } else if let Some(v) = ctx.get_var("result") {
        ("result".to_string(), v)
    } else {
        loop_var.clone()
    };

    let len = ctx
        .get_var("n")
        .map(|v| ("n".to_string(), v))
        .or_else(|| ctx.get_var("len").map(|v| ("len".to_string(), v)));

    ParamGuess { loop_var, acc, len }
}

/// 推定結果と ExtractCtx からパラメータの並びを構成する。
pub(crate) fn build_param_order(
    guess: &ParamGuess,
    entry_ctx: &super::super::context::ExtractCtx,
) -> Vec<(String, ValueId)> {
    let mut order = Vec::new();
    order.push(guess.loop_var.clone());
    if guess.acc.0 != guess.loop_var.0 {
        order.push(guess.acc.clone());
    }
    if let Some(len) = &guess.len {
        if len.0 != guess.loop_var.0 && len.0 != guess.acc.0 {
            order.push(len.clone());
        }
    }

    let mut seen: BTreeSet<String> = order.iter().map(|(n, _)| n.clone()).collect();
    for (name, var_id) in &entry_ctx.var_map {
        if !seen.contains(name) {
            order.push((name.clone(), *var_id));
            seen.insert(name.clone());
        }
    }
    order
}
