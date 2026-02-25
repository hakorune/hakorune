Status: Historical

# PHI生成コードの箱理論リファクタリング計画

**作成日**: 2025-11-20
**最終更新**: 2025-11-20 (Phase 26-B完了)
**ステータス**: ✅ Phase 1完了 / Phase 2-3実施中
**優先度**: High
**目標**: PHI生成コードの保守性・テスト容易性・可読性の向上

---

## 📋 **Executive Summary**

現在のPHI生成コードは約2,916行に及び、複数の責任が混在しています。本計画では、箱理論（Box-First）原則に基づき、責任を明確に分離した6つのBoxを提案します。

**期待される効果:**
- 🎯 保守性向上: 責任の明確化により変更影響範囲が限定される
- ⚡ テスト容易性: 各Boxを独立してテストできる
- 📚 可読性向上: 各Boxの役割が一目で分かる
- 🔄 再利用性: 他のPHI生成文脈でも使える

---

## 📊 **現状分析レポート**

### 既存のBox実装（完成済み✅）

| Box名 | 行数 | 責任 | 状態 |
|-------|------|------|------|
| **LocalScopeInspectorBox** | 152 | 変数定義位置の追跡 | ✅ 完全箱化済み |
| **LoopVarClassBox** | 516 | 変数4カテゴリ分類 | ✅ 完全箱化済み |
| **LoopSnapshotMergeBox** | 578 | スナップショットマージ統一管理 | ✅ 完全箱化済み |

**合計**: 1,246行（既に箱化済み）

---

### 箱化されていない責任領域

#### 1. **loopform_builder.rs** (1,075行) - 複数責任が混在

| 責任 | 行数 | 複雑度 | 箱化優先度 |
|------|------|--------|-----------|
| ValueId割り当て管理 | ~100 | 低 | Medium |
| Header PHI生成 | ~50 | 中 | High |
| Latch PHI更新 | ~120 | 高 | High |
| Exit PHI生成 | ~170 | **最高** | **Critical** |
| Preheader Copy生成 | ~50 | 低 | Low |
| 変数分類 | ~100 | 中 | Medium |

**問題点:**
- ❌ Exit PHI生成が最も複雑（173行の`build_exit_phis`）
- ❌ `seal_phis`が複数の責任を持つ（Header PHI + Latch値更新）
- ❌ LocalScopeInspectorBox/LoopVarClassBoxとの連携が散在

#### 2. **if_phi.rs** (298行) - 部分的箱化

| 責任 | 状態 | 箱化優先度 |
|------|------|-----------|
| If/Else PHI生成 | 🟡 Trait依存 | Medium |
| 変数変更検出 | ✅ 関数化済み | Low |
| PHI merge処理 | 🟡 Trait依存 | Medium |

#### 3. **loop_phi.rs** (265行) - レガシー scaffold

**ステータス**: Phase 31.x 以降で削除予定
**推奨**: 新規実装には使用しない

---

## 🎯 **箱化候補の詳細設計**

### 優先度マトリックス

```
         ┌─────────────────────────────────────────┐
High     │ PhiInputCollector  BodyLocalPhiBuilder  │
Effect   │ LoopSnapshotManager HeaderPhiBuilder    │
         │                                          │
         │         ExitPhiBuilder (最重要)         │
Medium   │                                          │
Effect   │         ValueIdAllocator                │
         │                                          │
         └─────────────────────────────────────────┘
           Low Risk ←────────────────────→ High Risk
```

---

### **Option 1: PhiInputCollector** ⭐ 最優先候補

#### 責任
- PHI入力の収集
- 重複predecessor削除（sanitize）
- 同値縮約最適化（optimize）

#### 設計
```rust
/// PHI入力収集専門Box
///
/// 複数のpredecessorからPHI入力を収集し、最適化を適用する。
pub struct PhiInputCollector {
    /// 収集された入力: (predecessor, value)
    inputs: Vec<(BasicBlockId, ValueId)>,
}

impl PhiInputCollector {
    /// 新しいcollectorを作成
    pub fn new() -> Self {
        Self { inputs: Vec::new() }
    }

    /// Preheader入力を追加
    pub fn add_preheader(&mut self, block: BasicBlockId, value: ValueId) {
        self.inputs.push((block, value));
    }

    /// Continue/Break snapshot入力を追加
    pub fn add_snapshot(&mut self, snapshot: &[(BasicBlockId, ValueId)]) {
        self.inputs.extend_from_slice(snapshot);
    }

    /// Latch入力を追加
    pub fn add_latch(&mut self, block: BasicBlockId, value: ValueId) {
        self.inputs.push((block, value));
    }

    /// 入力をサニタイズ（重複削除、ソート）
    pub fn sanitize(&mut self) {
        // 既存のsanitize_inputsロジックを使用
        let mut seen: BTreeMap<BasicBlockId, ValueId> = BTreeMap::new();
        for (bb, val) in self.inputs.iter() {
            seen.insert(*bb, *val);
        }
        self.inputs = seen.into_iter().collect();
        self.inputs.sort_by_key(|(bb, _)| bb.0);
    }

    /// 同値最適化: 全て同じ値ならPHI不要
    pub fn optimize_same_value(&self) -> Option<ValueId> {
        if self.inputs.is_empty() {
            return None;
        }
        if self.inputs.len() == 1 {
            return Some(self.inputs[0].1);
        }
        let first_val = self.inputs[0].1;
        if self.inputs.iter().all(|(_, val)| *val == first_val) {
            Some(first_val)
        } else {
            None
        }
    }

    /// 最終的な入力を取得
    pub fn finalize(self) -> Vec<(BasicBlockId, ValueId)> {
        self.inputs
    }
}
```

#### 評価
- 🎯 **影響範囲**: 中 (loopform_builder.rs + loop_snapshot_merge.rs)
- ⚡ **効果**: 中 (PHI入力収集ロジックの統一)
- ⚠️ **リスク**: **低** (既存関数を統合するだけ)
- 📅 **実装時間**: **小** (1-2時間)

#### 削減見込み
- loopform_builder.rs: ~50行削減
- loop_snapshot_merge.rs: ~30行削減
- **合計**: ~80行削減

---

### **Option 2: BodyLocalPhiBuilder** ⭐ 第2優先候補

#### 責任
- BodyLocal変数のPHI生成判定
- BodyLocalInternal変数のスキップ
- Exit PHI候補のフィルタリング

#### 設計
```rust
/// BodyLocal変数PHI生成専門Box
///
/// BodyLocalExit/BodyLocalInternal変数を判定し、
/// Exit PHI生成の要否を決定する。
pub struct BodyLocalPhiBuilder {
    classifier: LoopVarClassBox,
    inspector: LocalScopeInspectorBox,
}

impl BodyLocalPhiBuilder {
    /// 新しいbuilderを作成
    pub fn new(
        classifier: LoopVarClassBox,
        inspector: LocalScopeInspectorBox,
    ) -> Self {
        Self { classifier, inspector }
    }

    /// 変数がExit PHIを必要とするか判定
    ///
    /// # Returns
    /// - `Some(PhiSpec)`: PHI生成が必要
    /// - `None`: PHI不要（BodyLocalInternal）
    pub fn should_generate_exit_phi(
        &self,
        var_name: &str,
        pinned_vars: &[String],
        carrier_vars: &[String],
        exit_preds: &[BasicBlockId],
    ) -> bool {
        let class = self.classifier.classify(
            var_name,
            pinned_vars,
            carrier_vars,
            &self.inspector,
            exit_preds,
        );

        // BodyLocalInternal → Skip exit PHI
        class.needs_exit_phi()
    }

    /// Exit PHI候補をフィルタリング
    pub fn filter_exit_phi_candidates(
        &self,
        all_vars: &[String],
        pinned_vars: &[String],
        carrier_vars: &[String],
        exit_preds: &[BasicBlockId],
    ) -> Vec<String> {
        all_vars
            .iter()
            .filter(|var_name| {
                self.should_generate_exit_phi(
                    var_name,
                    pinned_vars,
                    carrier_vars,
                    exit_preds,
                )
            })
            .cloned()
            .collect()
    }

    /// Inspector参照を取得（スナップショット記録用）
    pub fn inspector_mut(&mut self) -> &mut LocalScopeInspectorBox {
        &mut self.inspector
    }
}
```

#### 評価
- 🎯 **影響範囲**: 中 (loopform_builder.rsのbuild_exit_phisの一部)
- ⚡ **効果**: 中 (BodyLocal処理の明確化)
- ⚠️ **リスク**: **低** (既存Boxの組み合わせ)
- 📅 **実装時間**: **小** (2-3時間)

#### 削減見込み
- loopform_builder.rs: ~60行削減（build_exit_phis内の分類ロジック）
- **合計**: ~60行削減

---

### **Option 3: LoopSnapshotManager** - データ構造統一

#### 責任
- Snapshot保存・取得
- Preheader/Exit/Continue snapshotの一元管理
- Snapshot比較（変数変更検出）

#### 設計
```rust
/// ループSnapshotの一元管理Box
///
/// preheader/exit/continue時のvariable mapを保存し、
/// PHI生成時に必要なsnapshotを提供する。
pub struct LoopSnapshotManager {
    /// Preheader時点の変数状態
    preheader_snapshot: HashMap<String, ValueId>,

    /// Exit predecessorごとのsnapshot
    exit_snapshots: Vec<(BasicBlockId, HashMap<String, ValueId>)>,

    /// Continue predecessorごとのsnapshot
    continue_snapshots: Vec<(BasicBlockId, HashMap<String, ValueId>)>,
}

impl LoopSnapshotManager {
    /// 新しいmanagerを作成
    pub fn new() -> Self {
        Self {
            preheader_snapshot: HashMap::new(),
            exit_snapshots: Vec::new(),
            continue_snapshots: Vec::new(),
        }
    }

    /// Preheader snapshotを保存
    pub fn save_preheader(&mut self, vars: HashMap<String, ValueId>) {
        self.preheader_snapshot = vars;
    }

    /// Exit snapshotを追加
    pub fn add_exit_snapshot(
        &mut self,
        block: BasicBlockId,
        vars: HashMap<String, ValueId>,
    ) {
        self.exit_snapshots.push((block, vars));
    }

    /// Continue snapshotを追加
    pub fn add_continue_snapshot(
        &mut self,
        block: BasicBlockId,
        vars: HashMap<String, ValueId>,
    ) {
        self.continue_snapshots.push((block, vars));
    }

    /// Preheader snapshotを取得
    pub fn preheader(&self) -> &HashMap<String, ValueId> {
        &self.preheader_snapshot
    }

    /// Exit snapshotsを取得
    pub fn exit_snapshots(&self) -> &[(BasicBlockId, HashMap<String, ValueId>)] {
        &self.exit_snapshots
    }

    /// Continue snapshotsを取得
    pub fn continue_snapshots(&self) -> &[(BasicBlockId, HashMap<String, ValueId>)] {
        &self.continue_snapshots
    }

    /// 変数がpreheaderから変更されたかチェック
    pub fn is_modified(&self, var_name: &str, current_value: ValueId) -> bool {
        self.preheader_snapshot
            .get(var_name)
            .map(|&preheader_val| preheader_val != current_value)
            .unwrap_or(true) // 新規変数は変更扱い
    }
}
```

#### 評価
- 🎯 **影響範囲**: 大 (builder.rs + loopform_builder.rs + loop_snapshot_merge.rs)
- ⚡ **効果**: 高 (Snapshot管理の一元化)
- ⚠️ **リスク**: 中 (データ構造の変更が必要)
- 📅 **実装時間**: 中 (4-6時間)

#### 削減見込み
- builder.rs: ~50行削減（loop_header_stack/loop_exit_stack関連）
- loopform_builder.rs: ~40行削減（preheader_vars管理）
- **合計**: ~90行削減

---

### **Option 4: HeaderPhiBuilder** - Header PHI専門化

#### 責任
- Header PHI生成
- Preheader入力設定
- Latch値更新（seal時）

#### 設計
```rust
/// Header PHI生成専門Box
///
/// Loop headerでのPHI nodeを生成し、sealする。
pub struct HeaderPhiBuilder {
    /// Pinned変数のPHI情報
    pinned_phis: Vec<PinnedPhiInfo>,
    /// Carrier変数のPHI情報
    carrier_phis: Vec<CarrierPhiInfo>,
}

#[derive(Debug, Clone)]
struct PinnedPhiInfo {
    var_name: String,
    phi_id: ValueId,
    param_value: ValueId,
    preheader_copy: ValueId,
}

#[derive(Debug, Clone)]
struct CarrierPhiInfo {
    var_name: String,
    phi_id: ValueId,
    init_value: ValueId,
    preheader_copy: ValueId,
}

impl HeaderPhiBuilder {
    /// 新しいbuilderを作成
    pub fn new() -> Self {
        Self {
            pinned_phis: Vec::new(),
            carrier_phis: Vec::new(),
        }
    }

    /// Pinned変数のPHIを準備
    pub fn prepare_pinned_phi(
        &mut self,
        var_name: String,
        phi_id: ValueId,
        param_value: ValueId,
        preheader_copy: ValueId,
    ) {
        self.pinned_phis.push(PinnedPhiInfo {
            var_name,
            phi_id,
            param_value,
            preheader_copy,
        });
    }

    /// Carrier変数のPHIを準備
    pub fn prepare_carrier_phi(
        &mut self,
        var_name: String,
        phi_id: ValueId,
        init_value: ValueId,
        preheader_copy: ValueId,
    ) {
        self.carrier_phis.push(CarrierPhiInfo {
            var_name,
            phi_id,
            init_value,
            preheader_copy,
        });
    }

    /// Header PHIを発行
    pub fn emit_header_phis<O: LoopFormOps>(
        &self,
        ops: &mut O,
        header_id: BasicBlockId,
        preheader_id: BasicBlockId,
    ) -> Result<(), String> {
        ops.set_current_block(header_id)?;

        // Pinned PHIs
        for phi in &self.pinned_phis {
            ops.emit_phi(
                phi.phi_id,
                vec![(preheader_id, phi.preheader_copy)],
            )?;
            ops.update_var(phi.var_name.clone(), phi.phi_id);
        }

        // Carrier PHIs
        for phi in &self.carrier_phis {
            ops.emit_phi(
                phi.phi_id,
                vec![(preheader_id, phi.preheader_copy)],
            )?;
            ops.update_var(phi.var_name.clone(), phi.phi_id);
        }

        Ok(())
    }

    /// Seal PHIs（latch + continue入力を追加）
    pub fn seal_phis<O: LoopFormOps>(
        &self,
        ops: &mut O,
        header_id: BasicBlockId,
        preheader_id: BasicBlockId,
        latch_id: BasicBlockId,
        continue_snapshots: &[(BasicBlockId, HashMap<String, ValueId>)],
    ) -> Result<(), String> {
        // Seal pinned PHIs
        for phi in &self.pinned_phis {
            let mut collector = PhiInputCollector::new();
            collector.add_preheader(preheader_id, phi.preheader_copy);

            // Continue inputs
            for (cid, snapshot) in continue_snapshots {
                if let Some(&value) = snapshot.get(&phi.var_name) {
                    collector.add_snapshot(&[(*cid, value)]);
                }
            }

            // Latch input
            let latch_value = ops
                .get_variable_at_block(&phi.var_name, latch_id)
                .unwrap_or(phi.phi_id);
            collector.add_latch(latch_id, latch_value);

            collector.sanitize();

            // Optimize same-value PHI
            if let Some(same_value) = collector.optimize_same_value() {
                // Skip PHI update - loop-invariant
                continue;
            }

            let inputs = collector.finalize();
            ops.update_phi_inputs(header_id, phi.phi_id, inputs)?;
        }

        // Seal carrier PHIs (同様のロジック)
        for phi in &self.carrier_phis {
            // ... 同様の処理 ...
        }

        Ok(())
    }
}
```

#### 評価
- 🎯 **影響範囲**: 中 (loopform_builder.rsの一部)
- ⚡ **効果**: 中 (Header PHI生成ロジックの分離)
- ⚠️ **リスク**: 中 (seal_phisロジックとの統合が必要)
- 📅 **実装時間**: 中 (4-5時間)

#### 削減見込み
- loopform_builder.rs: ~150行削減（emit_header_phis + seal_phisの一部）
- **合計**: ~150行削減

---

### **Option 5: ExitPhiBuilder** 🔥 最重要・最複雑

#### 責任
- Exit PHI生成
- Exit predecessors検証
- Phantom block除外
- Body-local変数の処理

#### 設計
```rust
/// Exit PHI生成専門Box
///
/// Loop exit時のPHI nodeを生成する。最も複雑な責任を持つ。
pub struct ExitPhiBuilder {
    snapshot_merger: LoopSnapshotMergeBox,
    body_local_builder: BodyLocalPhiBuilder,
}

impl ExitPhiBuilder {
    /// 新しいbuilderを作成
    pub fn new(
        snapshot_merger: LoopSnapshotMergeBox,
        body_local_builder: BodyLocalPhiBuilder,
    ) -> Self {
        Self {
            snapshot_merger,
            body_local_builder,
        }
    }

    /// Exit PHIsを生成
    pub fn build_exit_phis<O: LoopFormOps>(
        &mut self,
        ops: &mut O,
        exit_id: BasicBlockId,
        header_id: BasicBlockId,
        branch_source_block: BasicBlockId,
        header_vals: &HashMap<String, ValueId>,
        exit_snapshots: &[(BasicBlockId, HashMap<String, ValueId>)],
        pinned_vars: &[String],
        carrier_vars: &[String],
    ) -> Result<(), String> {
        ops.set_current_block(exit_id)?;

        // 1. Exit predecessorsを取得（CFG検証）
        let exit_preds_set = ops.get_block_predecessors(exit_id);
        let exit_preds: Vec<BasicBlockId> = exit_preds_set.iter().copied().collect();

        // 2. Phantom blockをフィルタリング
        let filtered_snapshots = self.filter_phantom_blocks(
            exit_snapshots,
            &exit_preds_set,
            ops,
        );

        // 3. Inspectorに定義を記録
        let inspector = self.body_local_builder.inspector_mut();
        for pinned_name in pinned_vars {
            inspector.record_definition(pinned_name, header_id);
        }
        for carrier_name in carrier_vars {
            inspector.record_definition(carrier_name, header_id);
        }
        for (block_id, snapshot) in &filtered_snapshots {
            inspector.record_snapshot(*block_id, snapshot);
        }
        if exit_preds_set.contains(&branch_source_block) {
            inspector.record_snapshot(branch_source_block, header_vals);
        }

        // 4. LoopSnapshotMergeBoxでPHI入力を生成
        let all_vars = self.snapshot_merger.merge_exit_with_classification(
            header_id,
            header_vals,
            &filtered_snapshots,
            &exit_preds,
            pinned_vars,
            carrier_vars,
            inspector,
        )?;

        // 5. PHI生成（optimize + sanitize適用）
        for (var_name, mut inputs) in all_vars {
            if let Some(same_val) = self.snapshot_merger.optimize_same_value(&inputs) {
                // 同値PHI → 直接バインド
                ops.update_var(var_name, same_val);
            } else {
                // 異なる値 → PHI生成
                self.snapshot_merger.sanitize_inputs(&mut inputs);
                let phi_id = ops.new_value();
                ops.emit_phi(phi_id, inputs)?;
                ops.update_var(var_name, phi_id);
            }
        }

        Ok(())
    }

    /// Phantom blockをフィルタリング
    fn filter_phantom_blocks<O: LoopFormOps>(
        &self,
        exit_snapshots: &[(BasicBlockId, HashMap<String, ValueId>)],
        exit_preds: &std::collections::HashSet<BasicBlockId>,
        ops: &O,
    ) -> Vec<(BasicBlockId, HashMap<String, ValueId>)> {
        let mut filtered = Vec::new();
        for (block_id, snapshot) in exit_snapshots {
            if !ops.block_exists(*block_id) {
                continue; // Non-existent block
            }
            if !exit_preds.contains(block_id) {
                continue; // Not a CFG predecessor
            }
            filtered.push((*block_id, snapshot.clone()));
        }
        filtered
    }
}
```

#### 評価
- 🎯 **影響範囲**: **大** (loopform_builder.rs + loop_snapshot_merge.rs)
- ⚡ **効果**: **高** (Exit PHI生成の複雑ロジック集約)
- ⚠️ **リスク**: **高** (複数ファイルにまたがる変更)
- 📅 **実装時間**: **大** (6-8時間)

#### 削減見込み
- loopform_builder.rs: ~170行削減（build_exit_phis全体）
- loop_snapshot_merge.rs: ~50行削減（merge_exit_with_classification簡略化）
- **合計**: ~220行削減

---

### **Option 6: ValueIdAllocator** - 優先度低

#### 責任
- ValueId割り当て
- カウンター管理
- パラメータ予約

#### 設計
```rust
/// ValueId割り当て専門Box
///
/// LoopForm構築時のValueId管理を抽象化。
pub struct ValueIdAllocator {
    next_value_id: u32,
    param_count: usize,
}

impl ValueIdAllocator {
    /// 新しいallocatorを作成
    pub fn new(param_count: usize, existing_vars: &HashMap<String, ValueId>) -> Self {
        let min_from_params = param_count as u32;
        let min_from_vars = existing_vars.values().map(|v| v.0 + 1).max().unwrap_or(0);

        Self {
            next_value_id: min_from_params.max(min_from_vars),
            param_count,
        }
    }

    /// 新しいValueIdを割り当て
    pub fn allocate(&mut self) -> ValueId {
        let id = ValueId(self.next_value_id);
        self.next_value_id += 1;
        id
    }

    /// カウンターを特定のValueId以降に設定
    pub fn ensure_after(&mut self, max_id: u32) {
        if self.next_value_id <= max_id {
            self.next_value_id = max_id + 1;
        }
    }
}
```

#### 評価
- 🎯 **影響範囲**: 小 (loopform_builder.rsのみ)
- ⚡ **効果**: 低 (既にLoopFormContextがある程度抽象化)
- ⚠️ **リスク**: 低
- 📅 **実装時間**: 小 (1-2時間)

#### 削減見込み
- loopform_builder.rs: ~20行削減（LoopFormContextの一部）
- **合計**: ~20行削減

---

## 📋 **リファクタリング実行計画**

### **Phase 1: Quick Wins（低リスク・即効性）**

**期間**: 1週間
**目標**: テスト容易性の向上、コード削減80-140行

#### タスク
1. **PhiInputCollector実装** (Priority: High, Risk: Low)
   - [ ] `src/mir/phi_core/phi_input_collector.rs`作成
   - [ ] PhiInputCollector struct実装
   - [ ] add_preheader/add_snapshot/add_latch実装
   - [ ] sanitize/optimize_same_value実装
   - [ ] 単体テスト作成（テストカバレッジ>90%）
   - [ ] loopform_builder.rsで使用
   - [ ] loop_snapshot_merge.rsで使用
   - [ ] 既存のsanitize_inputs/optimize_same_value削除

2. **BodyLocalPhiBuilder実装** (Priority: High, Risk: Low)
   - [ ] `src/mir/phi_core/body_local_phi_builder.rs`作成
   - [ ] BodyLocalPhiBuilder struct実装
   - [ ] should_generate_exit_phi実装
   - [ ] filter_exit_phi_candidates実装
   - [ ] 単体テスト作成（skip_whitespaceシナリオ含む）
   - [ ] loopform_builder.rsのbuild_exit_phisで使用
   - [ ] 既存の分類ロジック削除

**成果物:**
- ✅ 2つの新しいBox（合計~300行）
- ✅ 既存コード削減: ~140行
- ✅ テストカバレッジ: >90%
- ✅ ドキュメント: 各Boxのdocコメント完備

---

### **Phase 2: 構造改善（中リスク・効果大）**

**期間**: 2週間
**目標**: データ構造の一元化、コード削減240行

#### タスク
3. **LoopSnapshotManager実装** (Priority: Medium, Risk: Medium)
   - [ ] `src/mir/phi_core/loop_snapshot_manager.rs`作成
   - [ ] LoopSnapshotManager struct実装
   - [ ] save_preheader/add_exit_snapshot/add_continue_snapshot実装
   - [ ] is_modified実装
   - [ ] 単体テスト作成
   - [ ] builder.rsのloop_header_stack/loop_exit_stack削除
   - [ ] loopform_builder.rsのpreheader_vars削除
   - [ ] 全ループ構築箇所で使用

4. **HeaderPhiBuilder実装** (Priority: Medium, Risk: Medium)
   - [ ] `src/mir/phi_core/header_phi_builder.rs`作成
   - [ ] HeaderPhiBuilder struct実装
   - [ ] prepare_pinned_phi/prepare_carrier_phi実装
   - [ ] emit_header_phis実装
   - [ ] seal_phis実装（PhiInputCollector使用）
   - [ ] 単体テスト作成
   - [ ] loopform_builder.rsのemit_header_phis/seal_phis削除

**成果物:**
- ✅ 2つの新しいBox（合計~400行）
- ✅ 既存コード削減: ~240行
- ✅ データ構造の一元化完了
- ✅ Header PHI生成の完全分離

---

### **Phase 3: 最難関攻略（高リスク・最大効果）**

**期間**: 2-3週間
**目標**: Exit PHI生成の完全分離、コード削減220行

#### タスク
5. **ExitPhiBuilder実装** (Priority: Critical, Risk: High)
   - [ ] `src/mir/phi_core/exit_phi_builder.rs`作成
   - [ ] ExitPhiBuilder struct実装
   - [ ] build_exit_phis実装
   - [ ] filter_phantom_blocks実装
   - [ ] 包括的テストスイート作成
     - [ ] 正常系: 2 exit preds, 3 exit preds
     - [ ] 異常系: Phantom block, CFG不整合
     - [ ] skip_whitespaceシナリオ
     - [ ] BodyLocalExit vs BodyLocalInternal
   - [ ] loopform_builder.rsのbuild_exit_phis削除（173行）
   - [ ] loop_snapshot_merge.rsのmerge_exit_with_classification簡略化
   - [ ] 全ループ構築箇所で動作確認

**成果物:**
- ✅ 1つの新しいBox（合計~250行）
- ✅ 既存コード削減: ~220行
- ✅ Exit PHI生成の完全分離
- ✅ 最も複雑なロジックのテストカバレッジ>95%

---

### **Phase 4: 仕上げ（オプショナル）**

**期間**: 1週間
**目標**: 残存部分の最適化、ドキュメント完備

#### タスク
6. **ValueIdAllocator実装** (Priority: Low, Risk: Low)
   - [ ] `src/mir/phi_core/value_id_allocator.rs`作成
   - [ ] ValueIdAllocator struct実装
   - [ ] allocate/ensure_after実装
   - [ ] loopform_builder.rsのLoopFormContext置き換え

7. **ドキュメント整備**
   - [ ] 箱理論適用前後の比較図作成
   - [ ] 依存関係図作成（Before/After）
   - [ ] 実装ガイド作成
   - [ ] 各Boxのユースケース集作成

**成果物:**
- ✅ ValueIdAllocator実装（~80行）
- ✅ 既存コード削減: ~20行
- ✅ 完全なドキュメント
- ✅ 実装ガイド

---

## 📊 **削減効果まとめ**

### コード削減見込み

| Phase | Box実装 | 削減行数 | 純削減 | 累計削減 |
|-------|---------|---------|--------|---------|
| **Phase 1** | PhiInputCollector<br>BodyLocalPhiBuilder | +300 | -140 | **+160** |
| **Phase 2** | LoopSnapshotManager<br>HeaderPhiBuilder | +400 | -240 | **+160** |
| **Phase 3** | ExitPhiBuilder | +250 | -220 | **+30** |
| **Phase 4** | ValueIdAllocator | +80 | -20 | **+60** |
| **合計** | **1,030行** | **-620行** | **+410行** |

**注記:**
- 純削減が正の値 = コード量増加（構造化による一時的増加）
- しかし、テスト・ドキュメント・保守性は劇的向上
- Phase 3完了後、さらなる最適化で追加削減可能

### 保守性向上指標

| 指標 | 現状 | 目標 | 改善率 |
|------|------|------|--------|
| **最大関数サイズ** | 173行 | 50行以下 | **71%削減** |
| **責任の分離** | 混在 | 完全分離 | **100%改善** |
| **テストカバレッジ** | ~60% | >90% | **+50%向上** |
| **循環的複雑度** | 高 | 低 | **推定50%改善** |

---

## 🎯 **テスト戦略**

### Phase 1テスト

#### PhiInputCollector
```rust
#[cfg(test)]
mod phi_input_collector_tests {
    use super::*;

    #[test]
    fn test_single_input_optimization() {
        let mut collector = PhiInputCollector::new();
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(10));

        assert_eq!(collector.optimize_same_value(), Some(ValueId::new(10)));
    }

    #[test]
    fn test_same_value_optimization() {
        let mut collector = PhiInputCollector::new();
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(10));
        collector.add_latch(BasicBlockId::new(1), ValueId::new(10));
        collector.add_snapshot(&[(BasicBlockId::new(2), ValueId::new(10))]);

        assert_eq!(collector.optimize_same_value(), Some(ValueId::new(10)));
    }

    #[test]
    fn test_different_values_no_optimization() {
        let mut collector = PhiInputCollector::new();
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(10));
        collector.add_latch(BasicBlockId::new(1), ValueId::new(20));

        assert_eq!(collector.optimize_same_value(), None);
    }

    #[test]
    fn test_sanitize_duplicates() {
        let mut collector = PhiInputCollector::new();
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(10));
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(20)); // Duplicate

        collector.sanitize();
        let inputs = collector.finalize();

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0], (BasicBlockId::new(0), ValueId::new(20))); // Latest wins
    }

    #[test]
    fn test_sanitize_sorting() {
        let mut collector = PhiInputCollector::new();
        collector.add_latch(BasicBlockId::new(2), ValueId::new(20));
        collector.add_preheader(BasicBlockId::new(0), ValueId::new(10));
        collector.add_snapshot(&[(BasicBlockId::new(1), ValueId::new(15))]);

        collector.sanitize();
        let inputs = collector.finalize();

        // Should be sorted by BasicBlockId
        assert_eq!(inputs[0].0, BasicBlockId::new(0));
        assert_eq!(inputs[1].0, BasicBlockId::new(1));
        assert_eq!(inputs[2].0, BasicBlockId::new(2));
    }
}
```

#### BodyLocalPhiBuilder
```rust
#[cfg(test)]
mod body_local_phi_builder_tests {
    use super::*;

    #[test]
    fn test_skip_whitespace_scenario() {
        // Reproduce the exact skip_whitespace bug scenario
        let mut inspector = LocalScopeInspectorBox::new();

        let block_2 = BasicBlockId::new(2); // header / break path 1
        let block_5 = BasicBlockId::new(5); // break path 2

        // i, n, s in all blocks
        for var in &["i", "n", "s"] {
            inspector.record_definition(var, block_2);
            inspector.record_definition(var, block_5);
        }

        // ch only in block 5
        inspector.record_definition("ch", block_5);

        let classifier = LoopVarClassBox::new();
        let builder = BodyLocalPhiBuilder::new(classifier, inspector);

        let exit_preds = vec![block_2, block_5];
        let pinned = vec!["n".to_string(), "s".to_string()];
        let carrier = vec!["i".to_string()];

        // i, n, s should need exit PHI
        assert!(builder.should_generate_exit_phi("i", &pinned, &carrier, &exit_preds));
        assert!(builder.should_generate_exit_phi("n", &pinned, &carrier, &exit_preds));
        assert!(builder.should_generate_exit_phi("s", &pinned, &carrier, &exit_preds));

        // ch should NOT need exit PHI (BodyLocalInternal)
        assert!(!builder.should_generate_exit_phi("ch", &pinned, &carrier, &exit_preds));
    }

    #[test]
    fn test_filter_exit_phi_candidates() {
        let mut inspector = LocalScopeInspectorBox::new();

        let block_2 = BasicBlockId::new(2);
        let block_5 = BasicBlockId::new(5);

        inspector.record_definition("i", block_2);
        inspector.record_definition("i", block_5);
        inspector.record_definition("ch", block_5); // Only block 5

        let classifier = LoopVarClassBox::new();
        let builder = BodyLocalPhiBuilder::new(classifier, inspector);

        let all_vars = vec!["i".to_string(), "ch".to_string()];
        let exit_preds = vec![block_2, block_5];

        let candidates = builder.filter_exit_phi_candidates(
            &all_vars,
            &[],
            &["i".to_string()],
            &exit_preds,
        );

        assert_eq!(candidates.len(), 1);
        assert!(candidates.contains(&"i".to_string()));
        assert!(!candidates.contains(&"ch".to_string()));
    }
}
```

### Phase 3 Critical Tests

#### ExitPhiBuilder
```rust
#[cfg(test)]
mod exit_phi_builder_tests {
    use super::*;

    #[test]
    fn test_phantom_block_filtering() {
        // Test that phantom blocks (from stale snapshots) are correctly filtered
        // This is critical for preventing CFG inconsistencies
    }

    #[test]
    fn test_exit_phi_with_two_predecessors() {
        // Standard case: loop with 2 exit paths
    }

    #[test]
    fn test_exit_phi_with_three_predecessors() {
        // Complex case: loop with 3 exit paths
    }

    #[test]
    fn test_body_local_exit_variable() {
        // Variable defined in all exit paths → should get PHI
    }

    #[test]
    fn test_body_local_internal_variable() {
        // Variable defined in some (not all) exit paths → should NOT get PHI
    }

    #[test]
    fn test_exit_phi_optimization() {
        // Same-value optimization: all exit paths have same value → direct bind
    }
}
```

---

## 📚 **ドキュメント案**

### 箱理論適用前後の比較図

#### **Before: 責任混在（現状）**

```
┌─────────────────────────────────────────────────────────────┐
│ loopform_builder.rs (1,075 lines)                          │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ prepare_structure()                                   │  │
│  │  - ValueId割り当て ────────────┐                      │  │
│  │  - Carrier/Pinned分類 ────────┼─┐                    │  │
│  │  - Snapshot保存 ──────────────┼─┼─┐                  │  │
│  └──────────────────────────────────────────────────────┘  │
│                                      │ │ │                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ emit_header_phis()                │ │ │               │  │
│  │  - Header PHI生成 ───────────────┘ │ │               │  │
│  └──────────────────────────────────────────────────────┘  │
│                                        │ │                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ seal_phis()                         │ │               │  │
│  │  - Continue入力収集 ────────────────┘ │               │  │
│  │  - Latch値更新 ──────────────────────┘               │  │
│  │  - PHI sanitize/optimize (重複!)                      │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ build_exit_phis() (173 lines!)                        │  │
│  │  - Exit predecessors検証 ───────┐                     │  │
│  │  - Phantom block除外 ───────────┼─┐                   │  │
│  │  - Body-local分類 ─────────────┼─┼─┐                 │  │
│  │  - Inspector記録 ──────────────┼─┼─┼─┐               │  │
│  │  - Snapshot merge ─────────────┼─┼─┼─┼─┐             │  │
│  │  - PHI sanitize/optimize (重複!)│ │ │ │ │             │  │
│  └──────────────────────────────────────────────────────┘  │
│                                      ↓ ↓ ↓ ↓ ↓             │
│  ❌ 複雑度: 高                                               │
│  ❌ テスト困難                                               │
│  ❌ 保守性: 低                                               │
└─────────────────────────────────────────────────────────────┘
```

#### **After: 責任分離（目標）**

```
┌───────────────────────────────────────────────────────────────┐
│ PHI Generation Ecosystem (Box-First Architecture)            │
│                                                                │
│  ┌──────────────────┐  ┌──────────────────┐                  │
│  │ LocalScope       │  │ LoopVarClass     │                  │
│  │ InspectorBox     │  │ Box              │                  │
│  │ (152 lines)      │  │ (516 lines)      │                  │
│  │                  │  │                  │                  │
│  │ ✅ 定義位置追跡   │  │ ✅ 4カテゴリ分類  │                  │
│  └────────┬─────────┘  └────────┬─────────┘                  │
│           │                     │                             │
│           └──────────┬──────────┘                             │
│                      ↓                                        │
│           ┌──────────────────────┐                            │
│           │ BodyLocalPhiBuilder  │                            │
│           │ (~150 lines)         │                            │
│           │                      │                            │
│           │ ✅ Exit PHI判定      │                            │
│           │ ✅ フィルタリング     │                            │
│           └──────────┬───────────┘                            │
│                      │                                        │
│  ┌───────────────────┼───────────────────┐                   │
│  │ PhiInputCollector │                   │                   │
│  │ (~100 lines)      │                   │                   │
│  │                   │                   │                   │
│  │ ✅ 入力収集        │                   │                   │
│  │ ✅ Sanitize       │                   │                   │
│  │ ✅ Optimize       │                   │                   │
│  └───────────────────┘                   │                   │
│                                          │                   │
│  ┌──────────────────────────────────────┼─────────────────┐ │
│  │ HeaderPhiBuilder                     │                 │ │
│  │ (~200 lines)                         │                 │ │
│  │                                      │                 │ │
│  │ ✅ Header PHI生成  ──────────────────┘                 │ │
│  │ ✅ Seal処理                                             │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ ExitPhiBuilder                                          │ │
│  │ (~250 lines)                                            │ │
│  │                                                          │ │
│  │ ✅ Exit PHI生成                                          │ │
│  │ ✅ Phantom block除外                                     │ │
│  │ ✅ 完全分離                                              │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │ LoopSnapshotManager                                     │ │
│  │ (~150 lines)                                            │ │
│  │                                                          │ │
│  │ ✅ Snapshot一元管理                                      │ │
│  └─────────────────────────────────────────────────────────┘ │
│                                                                │
│  ✅ 複雑度: 低（各Box<100行）                                   │
│  ✅ テスト容易（独立テスト可能）                                │
│  ✅ 保守性: 高（責任明確）                                      │
└───────────────────────────────────────────────────────────────┘
```

### 依存関係図（After）

```
                    ┌─────────────────────┐
                    │ LocalScopeInspector │
                    │ Box                 │
                    └──────────┬──────────┘
                               │
                    ┌──────────┴──────────┐
                    │ LoopVarClassBox     │
                    └──────────┬──────────┘
                               │
                    ┌──────────┴──────────┐
                    │BodyLocalPhiBuilder  │
                    └──────────┬──────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
┌───────┴────────┐   ┌─────────┴────────┐   ┌────────┴───────┐
│PhiInputCollector│   │HeaderPhiBuilder  │   │ExitPhiBuilder  │
└────────────────┘   └──────────────────┘   └────────────────┘
        │                      │                      │
        └──────────────────────┼──────────────────────┘
                               │
                    ┌──────────┴──────────┐
                    │LoopSnapshotManager  │
                    └─────────────────────┘
```

**依存関係の原則:**
- ✅ 上位Boxは下位Boxに依存できる
- ❌ 下位Boxは上位Boxに依存しない（循環依存防止）
- ✅ 同レベルBoxは独立（水平依存なし）

---

## 💡 **実装ガイド（次のセッション用）**

### Phase 1 実装手順

#### Step 1: PhiInputCollector実装

```bash
# 1. ファイル作成
touch src/mir/phi_core/phi_input_collector.rs

# 2. mod.rsに追加
echo "pub mod phi_input_collector;" >> src/mir/phi_core/mod.rs

# 3. テンプレート作成（上記の設計をコピー）
# 4. テスト実装
# 5. loopform_builder.rsで使用
# 6. 既存コード削除
```

#### Step 2: BodyLocalPhiBuilder実装

```bash
# 同様の手順
touch src/mir/phi_core/body_local_phi_builder.rs
```

### 検証チェックリスト

#### Phase 1完了時 ✅ **Phase 26-B完了 (2025-11-20)**
- [x] PhiInputCollectorの単体テスト全通過 ✅ (33186e1e)
- [x] BodyLocalPhiBuilderの単体テスト全通過 ✅ (54f6ce84)
- [x] skip_whitespaceシナリオで動作確認 ✅ (既知の問題は別Issue)
- [x] 既存の全スモークテスト通過 ✅ (mir_loopform_exit_phi 4/4 PASS)
- [~] コード削減: 目標140行達成 → **実績+1行** (Phase 2-3で大幅削減予定)
- [x] ドキュメント: docコメント完備 ✅

**実装成果:**
- PhiInputCollector: 統一PHI入力収集 (BTreeMap決定性確保)
- BodyLocalPhiBuilder: BodyLocal変数PHI生成専門化
- 統合完了: loopform_builder/loop_builder/json_v0_bridge全対応
- コミット: 33186e1e, 54f6ce84, 05953387, 26288b54

#### Phase 2完了時
- [ ] LoopSnapshotManagerの単体テスト全通過
- [ ] HeaderPhiBuilderの単体テスト全通過
- [ ] データ構造の一元化確認
- [ ] 既存の全スモークテスト通過
- [ ] コード削減: 累計380行達成
- [ ] 依存関係図の正確性確認

#### Phase 3完了時
- [ ] ExitPhiBuilderの包括的テスト全通過
- [ ] Phantom block除外の動作確認
- [ ] BodyLocalExit/BodyLocalInternal区別確認
- [ ] 既存の全スモークテスト通過
- [ ] コード削減: 累計600行達成
- [ ] 複雑度測定: 最大関数50行以下確認

---

## 🎓 **箱理論との整合性確認**

### 箱理論4原則

#### 1. **分離 (Separation)**
- ✅ PhiInputCollector: PHI入力収集専門
- ✅ BodyLocalPhiBuilder: BodyLocal判定専門
- ✅ HeaderPhiBuilder: Header PHI専門
- ✅ ExitPhiBuilder: Exit PHI専門
- ✅ 各Boxは単一責任のみ

#### 2. **境界 (Boundary)**
- ✅ 明確なtrait定義（PhiMergeOps, LoopFormOps）
- ✅ pub/privateの適切な使い分け
- ✅ 依存関係の一方向性確保
- ✅ 横方向の依存なし（水平分離）

#### 3. **可逆性 (Reversibility)**
- ✅ 各Phaseは独立（Phase 1失敗でもPhase 2可能）
- ✅ Feature flagで段階的有効化可能
- ✅ 既存実装との並行運用可能（移行期間）
- ✅ ロールバック容易（Gitベース）

#### 4. **テスト容易性 (Testability)**
- ✅ 各Boxの単体テスト可能
- ✅ Mockable trait設計
- ✅ テストカバレッジ>90%目標
- ✅ 統合テストも独立実行可能

---

## 📝 **リスク分析と軽減策**

### リスク一覧

| リスク | 影響度 | 発生確率 | 軽減策 |
|-------|--------|---------|--------|
| **Phase 3失敗** | 高 | 中 | Phase 1-2で基盤確立、段階的移行 |
| **パフォーマンス劣化** | 中 | 低 | ベンチマーク測定、最適化 |
| **既存バグの顕在化** | 中 | 中 | 包括的テスト、段階的リリース |
| **実装時間超過** | 低 | 中 | Phase毎にタイムボックス設定 |

### Phase 3特有のリスク

**問題**: ExitPhiBuilderが最も複雑（173行のロジック移行）

**軽減策:**
1. **段階的実装**
   - Step 1: Phantom block除外のみ実装
   - Step 2: Body-local処理追加
   - Step 3: 完全移行

2. **並行運用**
   - 環境変数で新旧切り替え可能に
   - `NYASH_USE_EXIT_PHI_BUILDER=1`で新実装有効化
   - デフォルトは旧実装（安全性優先）

3. **包括的テスト**
   - skip_whitespaceシナリオ必須
   - Phantom blockシナリオ必須
   - 3+ exit predsシナリオ必須

---

## 🚀 **まとめ**

### 提案内容
6つのBoxによるPHI生成コードの完全分離を提案します：

1. **PhiInputCollector** - PHI入力収集統一
2. **BodyLocalPhiBuilder** - BodyLocal変数処理
3. **LoopSnapshotManager** - Snapshot一元管理
4. **HeaderPhiBuilder** - Header PHI専門化
5. **ExitPhiBuilder** - Exit PHI専門化（最重要）
6. **ValueIdAllocator** - ValueId管理（オプショナル）

### 期待効果
- 🎯 **保守性**: 責任明確化、最大関数173行→50行以下
- ⚡ **テスト容易性**: 独立テスト可能、カバレッジ60%→90%
- 📚 **可読性**: 各Boxの役割一目瞭然
- 🔄 **再利用性**: 他のPHI文脈でも使用可能

### 実装優先順位
1. **Phase 1**: PhiInputCollector + BodyLocalPhiBuilder（即効性）
2. **Phase 2**: LoopSnapshotManager + HeaderPhiBuilder（構造改善）
3. **Phase 3**: ExitPhiBuilder（最難関・最大効果）
4. **Phase 4**: ValueIdAllocator + ドキュメント（仕上げ）

### 次のステップ
- [ ] 本計画のレビュー（ユーザー確認）
- [ ] Phase 1着手判断
- [ ] Phase 1実装（1週間以内）
- [ ] Phase 2実装（2週間以内）
- [ ] Phase 3実装（3週間以内）

---

**箱理論原則に基づく段階的リファクタリングで、確実に保守性を向上させます！** 🎉
