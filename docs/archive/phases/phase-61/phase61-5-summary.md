# Phase 61-5: If PHI 削減計画策定 - サマリー

## 目的
Phase 61-3 で JoinIR 経路による If-in-Loop PHI 生成が完全動作したため、既存の If PHI 関数群を整理し、削減計画を策定する。

---

## 実施内容（4ステップ完了）

### Phase 61-5.1: If PHI 関数リスト作成 ✅
- **成果物**: [phase61-5-1-if-phi-function-list.md](phase61-5-1-if-phi-function-list.md)
- **内容**: If PHI 関連の全関数を 3 カテゴリに分類
  - PhiBuilderBox系: 4関数（set_if_context, generate_if_phis 等）
  - JoinIR Lowering系: 14関数（try_lower_if_to_joinir, find_if_pattern 等）
  - 補助系: 8関数（IfPhiContext, PhiSpec, Conservative 等）
- **合計**: 26関数をリスト化

### Phase 61-5.2: JoinIR カバレッジ調査 ✅
- **成果物**: [phase61-5-2-if-phi-coverage-table.md](phase61-5-2-if-phi-coverage-table.md)
- **内容**: 各関数の JoinIR カバレッジを調査
  - `in_loop`: 完全カバー済み（18関数）
  - `partial`: 部分カバー（5関数）
  - `none`: 未カバー（3関数）
- **重要発見**: If-in-Loop PHI は JoinIR で 100% カバー達成済み

### Phase 61-5.3: 優先度表作成 ✅
- **成果物**: [phase61-5-3-if-phi-priority-table.md](phase61-5-3-if-phi-priority-table.md)
- **内容**: カバレッジ調査を基に削減優先度を設定
  - **P1（最優先）**: 18関数 - 薄いラッパー、既に JoinIR で完全カバー
  - **P2（次期候補）**: 5関数 - JoinIR 拡張で統合可能
  - **P3（将来候補）**: 3関数 - 型システム統合待ち

### Phase 61-5.4: 次フェーズ候補選定 ✅
- **成果物**: [phase61-5.4-next-phase-candidates.md](phase61-5.4-next-phase-candidates.md)
- **内容**: Phase 61-6 で削減する 3 個の候補を選定
  1. **`set_if_context` 削除**（P1, 11行）- 最優先、最も安全
  2. **If Lowering dev フラグ削除**（P1, 15行）- デフォルト有効化
  3. **A/B テスト観察関数削除**（P2, 50行）- 最も削減効果が高い

---

## Phase 61-6 実装計画

### Wave 1: P1 薄いラッパー削除（26行削減）
1. `set_if_context` 削除
   - `loop_builder/if_lowering.rs:259-262` で直接 `IfPhiContext::for_loop_body()` 呼び出しに置き換え
   - リスク: 低
2. If Lowering dev フラグ削除
   - `joinir_if_select_enabled()` チェック削除、デフォルト有効化
   - リスク: 中（全テスト PASS 必須）

### Wave 2: P2 A/B テスト削除（50行削減）
1. `extract_phi_spec_from_builder`, `compare_and_log_phi_specs` 削除
   - if_lowering.rs の A/B 比較ロジック削除
   - if_phi_spec.rs は `compute_phi_spec_from_joinir()` のみ残す
   - リスク: 低（観察専用関数）

### 期待削減効果
- **合計**: 76行削減
- **JoinIR SSOT**: PhiBuilderBox 経路の観察コード完全削除
- **カバー率**: If-in-Loop PHI 100% JoinIR 化完了

---

## 保留候補（Phase 61-7+ で検討）

### P2 大型統合（145行削減見込み）
1. `compute_modified_names_if`: 変更変数検出を JoinIR に統合（~75行）
2. `get_conservative_if_values`: incoming 値解決を PhiSpec に移行（~70行）

### 保留理由
- Phase 61-6 では最小限の削除に集中（リスク最小化）
- P2 大型統合は Phase 61-7 で慎重に実装

---

## 技術的成果

### 1. If PHI 関数の完全インベントリ作成
- 26関数の全体像を把握
- 責務とカバレッジを明確化
- 削減優先度を定量的に決定

### 2. JoinIR カバレッジ 100% 達成確認
- If-in-Loop PHI は JoinIR で完全カバー
- PhiBuilderBox 経路は観察用のみ
- SSOT を JoinIR 側に完全移行済み

### 3. 段階的削減計画の確立
- P1（薄いラッパー）→ P2（統合）→ P3（型システム）の 3 段階
- Wave 1（26行）→ Wave 2（50行）→ Phase 61-7（145行）の段階的実装
- リスク最小化と削減効果のバランス

---

## 実装時の原則

1. **Fail-Fast**: エラーは即座に失敗、フォールバック禁止
2. **決定的順序**: BTreeSet/BTreeMap で ValueId 割り当ての決定性保証
3. **箱理論**: SSOT を JoinIR 側に寄せる、PhiBuilderBox は最小限に
4. **テスト**: 各ステップで全 JoinIR tests PASS を確認（268 tests）
5. **ドキュメント**: PHI_BOX_INVENTORY.md と CURRENT_TASK.md を更新

---

## 関連ドキュメント

### Phase 61-5 成果物
- [phase61-5-1-if-phi-function-list.md](phase61-5-1-if-phi-function-list.md)
- [phase61-5-2-if-phi-coverage-table.md](phase61-5-2-if-phi-coverage-table.md)
- [phase61-5-3-if-phi-priority-table.md](phase61-5-3-if-phi-priority-table.md)
- [phase61-5.4-next-phase-candidates.md](phase61-5.4-next-phase-candidates.md)

### 関連 Phase
- Phase 61-1: [if_phi_context.rs](../../../src/mir/join_ir/lowering/if_phi_context.rs) - If PHI Context 設計
- Phase 61-2: [if_phi_spec.rs](../../../src/mir/join_ir/lowering/if_phi_spec.rs) - PHI Spec 抽象化
- Phase 61-3: [if_in_loop/mod.rs](../../../src/mir/join_ir/frontend/ast_lowerer/if_in_loop/mod.rs) - If-in-Loop PHI Emitter
- Phase 61-4: [phase61-4-toplevel-if-design.md](phase61-4-toplevel-if-design.md) - Toplevel If PHI 設計
- PHI Inventory: [PHI_BOX_INVENTORY.md](../../../docs/private/roadmap2/phases/phase-30-final-joinir-world/PHI_BOX_INVENTORY.md)

---

## 次のステップ

### 即座に実装可能（Phase 61-6）
1. `set_if_context` 削除（11行、リスク低）
2. If Lowering dev フラグ削除（15行、リスク中）
3. A/B テスト観察関数削除（50行、リスク低）

### 合計削減見込み: 76行
### 期待成果: JoinIR SSOT 確立、PhiBuilderBox 観察コード完全削除
Status: Historical
