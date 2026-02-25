/*!
 * LoopSnapshotMergeBox - Exit PHI merge utility (pinned/carrier + availability)
 *
 * - Exit PHI inputs are merged using pinned/carrier hints and availability
 *   reconstructed from header/exit snapshots (Option C guard)。
 * - Header/exit PHI の仕様や最適化は LoopFormBuilder / LoopScopeShape 側が SSOT。
 * - 将来の縮退・移行計画は docs 側（Phase 37+ メモ）を参照してね。
 */

use crate::mir::{BasicBlockId, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// Phase 36: Pure static utility for exit PHI merging (no state)
///
/// # Responsibility
///
/// - Exit PHI input merging with variable classification (Option C)
/// - PHI pred mismatch prevention via availability checking
///
/// # Design: Pure Static Utility
///
/// This struct has no fields and provides only static methods.
/// All state is passed as function arguments.
pub struct LoopSnapshotMergeBox;

impl LoopSnapshotMergeBox {
    /// Option C: exit_merge with variable classification
    ///
    /// # Phase 36 Essential Logic
    ///
    /// This method is the SSOT for exit PHI input merging with:
    /// 1. 変数分類（Pinned/Carrier/BodyLocalExit/BodyLocalInternal）
    /// 2. スナップショットに基づく availability チェック
    /// 3. PHI pred mismatch prevention (Option C)
    ///
    /// # Future Migration Path (Phase 37+)
    ///
    /// - Expand JoinIR Exit lowering to cover complex patterns
    /// - Move classification logic to LoopScopeShape
    /// - Reduce this to thin adapter or remove entirely
    ///
    /// ## 目的
    ///
    /// PHI pred mismatch バグを防ぐため、body-local変数が全exit predecessorで
    /// 定義されているかチェックし、定義されていない変数はexit PHIを生成しない。
    ///
    /// ## 引数
    ///
    /// - `header_id`: header ブロックのID（fallthrough元）
    /// - `header_vals`: header での変数値
    /// - `exit_snapshots`: 各 break 文での変数スナップショット
    /// - `exit_preds`: Exit block の実際のCFG predecessors（重要！snapshotだけではない）
    /// - `pinned_vars`: Loop-crossing parameters（常にPHI生成）
    /// - `carrier_vars`: Loop-modified variables（常にPHI生成）
    ///
    /// ## 戻り値
    ///
    /// 各変数ごとの PHI 入力（predecessor, value）のリスト
    ///
    /// ## Option C ロジック
    ///
    /// 1. スナップショットから変数定義位置を再構築
    /// 2. pinned/carrier hint を優先し、残りは availability で BodyLocalExit/BodyLocalInternal を判定
    /// 3. BodyLocalInternal（全exit predsで定義されていない）変数は skip
    ///
    /// ## 例（skip_whitespace バグ修正）
    ///
    /// ```
    /// loop(1 == 1) {
    ///     if i >= n { break }       // exit pred 1: ch doesn't exist
    ///     local ch = s.substring()  // ch defined here
    ///     if ch == " " { ... } else { break }  // exit pred 2: ch exists
    /// }
    /// // ch は BodyLocalInternal → exit PHI 生成しない（バグ修正！）
    /// ```
    pub fn merge_exit_with_classification(
        header_id: BasicBlockId,
        header_vals: &BTreeMap<String, ValueId>,
        exit_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
        exit_preds: &[BasicBlockId],
        pinned_vars: &[String],
        carrier_vars: &[String],
    ) -> Result<BTreeMap<String, Vec<(BasicBlockId, ValueId)>>, String> {
        let mut result: BTreeMap<String, Vec<(BasicBlockId, ValueId)>> = BTreeMap::new();

        let debug = std::env::var("NYASH_OPTION_C_DEBUG").is_ok();

        let definitions = build_definitions(header_id, header_vals, exit_snapshots);
        if debug {
            crate::runtime::get_global_ring0()
                .log
                .debug("[Option C] merge_exit_with_classification called");
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[Option C]   exit_preds: {:?}", exit_preds));
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[Option C]   pinned_vars: {:?}", pinned_vars));
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[Option C]   carrier_vars: {:?}", carrier_vars));
        }

        // [LoopForm] Case A/B 分岐:
        // - header_id ∈ exit_preds → header fallthrough を exit PHI 入力に含める（Case A）
        // - header_id ∉ exit_preds → break-only, header 由来の値を PHI に入れない（Case B）
        //
        // Header is a valid predecessor only when CFG actually branches to exit from the header.
        // 例: loop(1 == 1) { ... break } のように header→exit のエッジが無い場合は、
        // header 値を exit PHI 入力に含めると「非支配ブロックからの値参照」で壊れる。
        let header_in_exit_preds = exit_preds.contains(&header_id);

        // すべての変数名を収集（決定的順序のためBTreeSet使用）
        let mut all_vars: BTreeSet<String> = BTreeSet::new();
        all_vars.extend(header_vals.keys().cloned());
        for (_, snap) in exit_snapshots {
            all_vars.extend(snap.keys().cloned());
        }

        // 各変数を分類して、exit PHI が必要なもののみ処理（アルファベット順で決定的）
        for var_name in all_vars {
            let class = classify_exit_var(
                &var_name,
                pinned_vars,
                carrier_vars,
                exit_preds,
                &definitions,
            );
            let needs_exit_phi = class_needs_exit_phi(class);

            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[Option C]   var '{}': {:?} needs_exit_phi={}",
                    var_name, class, needs_exit_phi
                ));
                if let Some(defining_blocks) = definitions.get(&var_name) {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[Option C]     defining_blocks: {:?}",
                        defining_blocks
                    ));
                }
            }

            // Option C: Additional check - even Carrier/Pinned need definition check!
            // Carrier/Pinned だからといって、全 exit preds で定義されているとは限らない
            let is_in_all_preds = is_available_in_all(&var_name, exit_preds, &definitions);

            // exit PHI が不要な場合は skip
            if !needs_exit_phi || !is_in_all_preds {
                if debug {
                    if !needs_exit_phi {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Option C]   → SKIP exit PHI for '{}' (class={:?})",
                            var_name, class
                        ));
                    } else {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Option C]   → SKIP exit PHI for '{}' (NOT in all preds, class={:?})",
                            var_name, class
                        ));
                    }
                }
                continue;
            }

            // exit PHI が必要な変数のみ入力を集約
            let mut inputs: Vec<(BasicBlockId, ValueId)> = Vec::new();

            // Header fallthrough（header に存在する変数のみ）
            if header_in_exit_preds {
                if let Some(&val) = header_vals.get(&var_name) {
                    inputs.push((header_id, val));
                }
            } else if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[Option C]   header not a predecessor → skip header input for '{}'",
                    var_name
                ));
            }

            // Break snapshots
            for (bb, snap) in exit_snapshots {
                // Step 5-5-H: CRITICAL - Skip phantom exit preds
                if !exit_preds.contains(bb) {
                    if debug {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[Option C] ⚠️ SKIP phantom exit pred (not in CFG): {:?} for var '{}'",
                            bb, var_name
                        ));
                    }
                    continue;
                }

                if let Some(&val) = snap.get(&var_name) {
                    inputs.push((*bb, val));
                }
            }

            if !inputs.is_empty() {
                result.insert(var_name, inputs);
            }
        }

        Ok(result.into_iter().collect())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExitVarClass {
    Pinned,
    Carrier,
    BodyLocalExit,
    BodyLocalInternal,
}

fn classify_exit_var(
    var_name: &str,
    pinned_vars: &[String],
    carrier_vars: &[String],
    exit_preds: &[BasicBlockId],
    definitions: &BTreeMap<String, BTreeSet<BasicBlockId>>,
) -> ExitVarClass {
    if is_pin_temp(var_name) {
        return ExitVarClass::BodyLocalInternal;
    }

    if pinned_vars.iter().any(|p| p == var_name) {
        return ExitVarClass::Pinned;
    }

    if carrier_vars.iter().any(|c| c == var_name) {
        return ExitVarClass::Carrier;
    }

    if is_available_in_all(var_name, exit_preds, definitions) {
        ExitVarClass::BodyLocalExit
    } else {
        ExitVarClass::BodyLocalInternal
    }
}

fn class_needs_exit_phi(class: ExitVarClass) -> bool {
    matches!(
        class,
        ExitVarClass::Pinned | ExitVarClass::Carrier | ExitVarClass::BodyLocalExit
    )
}

fn build_definitions(
    header_id: BasicBlockId,
    header_vals: &BTreeMap<String, ValueId>,
    exit_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
) -> BTreeMap<String, BTreeSet<BasicBlockId>> {
    let mut definitions: BTreeMap<String, BTreeSet<BasicBlockId>> = BTreeMap::new();

    for name in header_vals.keys() {
        definitions
            .entry(name.clone())
            .or_default()
            .insert(header_id);
    }

    for (bb, snap) in exit_snapshots {
        for name in snap.keys() {
            definitions.entry(name.clone()).or_default().insert(*bb);
        }
    }

    definitions
}

fn is_available_in_all(
    var_name: &str,
    required_blocks: &[BasicBlockId],
    definitions: &BTreeMap<String, BTreeSet<BasicBlockId>>,
) -> bool {
    if let Some(defining_blocks) = definitions.get(var_name) {
        required_blocks
            .iter()
            .all(|block| defining_blocks.contains(block))
    } else {
        false
    }
}

fn is_pin_temp(var_name: &str) -> bool {
    var_name.starts_with("__pin$") && var_name.contains("$@")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test merge_exit_with_classification with simple carrier variable
    ///
    /// Phase 36: This is the ONLY remaining test (merge_continue tests removed as dead code)
    #[test]
    fn test_merge_exit_with_classification_simple_carrier() {
        let header_id = BasicBlockId::new(0);
        let break_bb = BasicBlockId::new(1);

        let mut header_vals = BTreeMap::new();
        header_vals.insert("i".to_string(), ValueId::new(1));

        let mut break_snap = BTreeMap::new();
        break_snap.insert("i".to_string(), ValueId::new(10));
        let exit_snapshots = vec![(break_bb, break_snap)];

        let exit_preds = vec![header_id, break_bb];
        let pinned_vars = vec![];
        let carrier_vars = vec!["i".to_string()];

        // Phase 69-2: inspector は merge_exit_with_classification 内部で構築
        let result = LoopSnapshotMergeBox::merge_exit_with_classification(
            header_id,
            &header_vals,
            &exit_snapshots,
            &exit_preds,
            &pinned_vars,
            &carrier_vars,
        )
        .unwrap();

        // "i" should have exit PHI with inputs from header and break
        assert_eq!(result.len(), 1);
        let i_inputs = result.get("i").unwrap();
        assert_eq!(i_inputs.len(), 2);
        assert!(i_inputs.contains(&(header_id, ValueId::new(1))));
        assert!(i_inputs.contains(&(break_bb, ValueId::new(10))));
    }

    /// Test merge_exit_with_classification skips BodyLocalInternal variables
    ///
    /// Phase 36: Tests Option C logic (PHI pred mismatch prevention)
    #[test]
    fn test_merge_exit_with_classification_skips_body_local_internal() {
        let header_id = BasicBlockId::new(0);
        let break1_bb = BasicBlockId::new(1);
        let break2_bb = BasicBlockId::new(2);

        let header_vals = BTreeMap::new(); // ch not in header

        let break1_snap = BTreeMap::new();
        // break1: ch not defined yet (early exit)
        let mut break2_snap = BTreeMap::new();
        break2_snap.insert("ch".to_string(), ValueId::new(20)); // ch defined in break2
        let exit_snapshots = vec![(break1_bb, break1_snap), (break2_bb, break2_snap)];

        let exit_preds = vec![header_id, break1_bb, break2_bb];
        let pinned_vars = vec![];
        let carrier_vars = vec![];

        // Phase 69-2: inspector は merge_exit_with_classification 内部で構築
        // ch は break2_snap にのみ存在するので、内部で正しく検出される
        let result = LoopSnapshotMergeBox::merge_exit_with_classification(
            header_id,
            &header_vals,
            &exit_snapshots,
            &exit_preds,
            &pinned_vars,
            &carrier_vars,
        )
        .unwrap();

        // "ch" should NOT have exit PHI (BodyLocalInternal - not in all exit preds)
        assert_eq!(
            result.len(),
            0,
            "Expected no exit PHI for BodyLocalInternal variable 'ch'"
        );
    }

    /// Test merge_exit_with_classification with header not in exit_preds (break-only loop)
    ///
    /// Phase 36: Tests Case B (header not in exit preds)
    #[test]
    fn test_merge_exit_with_classification_break_only_loop() {
        let header_id = BasicBlockId::new(0);
        let break_bb = BasicBlockId::new(1);

        let mut header_vals = BTreeMap::new();
        header_vals.insert("i".to_string(), ValueId::new(1));

        let mut break_snap = BTreeMap::new();
        break_snap.insert("i".to_string(), ValueId::new(10));
        let exit_snapshots = vec![(break_bb, break_snap)];

        // Note: header_id NOT in exit_preds (break-only loop, no header fallthrough)
        let exit_preds = vec![break_bb];
        let pinned_vars = vec![];
        let carrier_vars = vec!["i".to_string()];

        // Phase 69-2: inspector は merge_exit_with_classification 内部で構築
        let result = LoopSnapshotMergeBox::merge_exit_with_classification(
            header_id,
            &header_vals,
            &exit_snapshots,
            &exit_preds,
            &pinned_vars,
            &carrier_vars,
        )
        .unwrap();

        // "i" should have exit PHI with ONLY break input (no header input)
        assert_eq!(result.len(), 1);
        let i_inputs = result.get("i").unwrap();
        assert_eq!(i_inputs.len(), 1);
        assert!(i_inputs.contains(&(break_bb, ValueId::new(10))));
        assert!(
            !i_inputs.contains(&(header_id, ValueId::new(1))),
            "Header input should be excluded when header not in exit_preds"
        );
    }
}
