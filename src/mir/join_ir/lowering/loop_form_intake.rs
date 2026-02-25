//! LoopForm intake helpers for generic JoinIR lowering.
//!
//! 役割:
//! - LoopForm + MirQuery + MirFunction から、JoinIR 降下に必要な情報を抽出する。
//! - pinned/carrier 推定と ValueId マッピング、header/exit スナップショット収集を一元化。
//! - generic_case_a/B など複数ロワーで再利用するための「入口箱」だよ。
//!
//! Phase 70-1: Trio (LocalScopeInspectorBox, LoopVarClassBox) 依存削除
//! - 変数分類は LoopScopeShape::from_loop_form() に一本化（二重分類問題解消）

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirQuery, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// 抽出結果
pub(crate) struct LoopFormIntake {
    pub pinned_ordered: Vec<String>,
    pub carrier_ordered: Vec<String>,
    pub header_snapshot: BTreeMap<String, ValueId>,
    pub exit_snapshots: Vec<(BasicBlockId, BTreeMap<String, ValueId>)>,
    pub exit_preds: Vec<BasicBlockId>,
}

/// LoopForm + MIR から pinned/carrier とスナップショットを抽出する。
///
/// Phase 70-1: var_classes 引数削除（Trio 依存排除）
/// 実際の変数分類は呼び出し側が LoopScopeShape::from_loop_form() で実施
///
/// 失敗時は None（フォールバック用）。
pub(crate) fn intake_loop_form(
    loop_form: &crate::mir::loop_form::LoopForm,
    query: &impl MirQuery,
    mir_func: &MirFunction,
) -> Option<LoopFormIntake> {
    // preheader のパラメータを名前付け
    let mut value_to_name: BTreeMap<ValueId, String> = BTreeMap::new();
    let mut preheader_names: BTreeMap<String, ValueId> = BTreeMap::new();
    for (idx, val) in mir_func.params.iter().copied().enumerate() {
        let name = if idx == 0 {
            "s".to_string()
        } else {
            format!("param{}", idx)
        };
        value_to_name.insert(val, name.clone());
        preheader_names.insert(name, val);
    }

    let mut string_name: Option<String> = value_to_name.get(mir_func.params.get(0)?).cloned();
    let mut len_name: Option<String> = None;
    let mut index_name: Option<String> = None;

    for inst in query.insts_in_block(loop_form.preheader) {
        match inst {
            MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(crate::mir::Callee::Method {
                        method,
                        receiver: Some(recv),
                        ..
                    }),
                ..
            } if method == "length" => {
                let s_name = value_to_name
                    .get(recv)
                    .cloned()
                    .unwrap_or_else(|| "s".to_string());
                value_to_name.entry(*recv).or_insert(s_name.clone());
                string_name.get_or_insert(s_name.clone());
                value_to_name.entry(*dst).or_insert_with(|| "n".to_string());
                len_name.get_or_insert_with(|| value_to_name[dst].clone());
                preheader_names
                    .entry(value_to_name[dst].clone())
                    .or_insert(*dst);
            }
            MirInstruction::Const {
                dst,
                value: crate::mir::types::ConstValue::Integer(0),
            } if index_name.is_none() => {
                value_to_name.entry(*dst).or_insert_with(|| "i".to_string());
                index_name = Some(value_to_name[dst].clone());
                preheader_names
                    .entry(value_to_name[dst].clone())
                    .or_insert(*dst);
            }
            MirInstruction::Copy { dst, src } => {
                if let Some(existing) = value_to_name.get(src).cloned() {
                    value_to_name.entry(*dst).or_insert(existing.clone());
                    preheader_names.entry(existing).or_insert(*dst);
                }
            }
            _ => {}
        }
    }

    // header φ を読む
    let mut header_vals_mir: BTreeMap<String, ValueId> = BTreeMap::new();
    let mut snapshot_maps: BTreeMap<BasicBlockId, BTreeMap<String, ValueId>> = BTreeMap::new();
    let mut pinned_hint: Vec<String> = Vec::new();
    let mut carrier_hint: Vec<String> = Vec::new();

    for inst in query.insts_in_block(loop_form.header) {
        if let MirInstruction::Phi { dst, inputs, .. } = inst {
            let name = inputs
                .iter()
                .find_map(|(_, v)| value_to_name.get(v).cloned())
                .unwrap_or_else(|| format!("v{}", dst.0));
            value_to_name.entry(*dst).or_insert(name.clone());
            header_vals_mir.insert(name.clone(), *dst);

            let pre_val = inputs
                .iter()
                .find(|(bb, _)| *bb == loop_form.preheader)
                .map(|(_, v)| *v);
            let latch_val = inputs
                .iter()
                .find(|(bb, _)| *bb == loop_form.latch)
                .map(|(_, v)| *v);
            let is_carrier = match (pre_val, latch_val) {
                (Some(p), Some(l)) => p != l,
                (None, Some(_)) => true,
                _ => false,
            };
            if is_carrier {
                carrier_hint.push(name.clone());
            } else {
                pinned_hint.push(name.clone());
            }

            for (bb, val) in inputs {
                let entry = snapshot_maps.entry(*bb).or_default();
                entry.insert(name.clone(), *val);
                value_to_name.entry(*val).or_insert(name.clone());
            }
        }
    }

    // header で読まれている値も拾う
    for inst in query.insts_in_block(loop_form.header) {
        for used in query.reads_of(inst) {
            if let Some(name) = value_to_name.get(&used).cloned() {
                header_vals_mir.entry(name).or_insert(used);
            }
        }
    }

    // preheader の既知を補完（s/n/i）
    for (name, val) in &preheader_names {
        header_vals_mir.entry(name.clone()).or_insert(*val);
    }

    // exit preds を CFG から集める
    let mut exit_preds: Vec<BasicBlockId> = mir_func
        .blocks
        .iter()
        .filter(|(_, bb)| bb.successors.contains(&loop_form.exit))
        .map(|(id, _)| *id)
        .collect();
    exit_preds.sort_by_key(|b| b.0);

    if exit_preds.contains(&loop_form.header) {
        snapshot_maps
            .entry(loop_form.header)
            .or_insert_with(|| header_vals_mir.clone());
    }

    let mut exit_snapshots: Vec<(BasicBlockId, BTreeMap<String, ValueId>)> = Vec::new();
    for pred in &exit_preds {
        if let Some(snap) = snapshot_maps.get(pred) {
            exit_snapshots.push((*pred, snap.clone()));
        }
    }

    // Phase 70-1: Trio 分類を削除し、pinned_hint/carrier_hint をそのまま返す
    // 実際の分類は LoopScopeShape::from_loop_form() 内部で実施される（二重分類問題解消）
    let ordered_pinned: Vec<String> = pinned_hint
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let ordered_carriers: Vec<String> = carrier_hint
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    if ordered_pinned.is_empty() || ordered_carriers.is_empty() {
        return None;
    }

    Some(LoopFormIntake {
        pinned_ordered: ordered_pinned,
        carrier_ordered: ordered_carriers,
        header_snapshot: header_vals_mir,
        exit_snapshots,
        exit_preds,
    })
}
