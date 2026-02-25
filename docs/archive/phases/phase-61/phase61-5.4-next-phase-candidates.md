# Phase 61-5.4: 次フェーズ候補選定（2〜3個）

## 目的
Phase 61-5.3 の優先度表から、Phase 61-6（If PHI JoinIR 化 第2弾）で触る候補を 2〜3 個だけ選定する。

## 選定基準

### P1 候補（薄いラッパー削除）
- コード量が少ない（50行以下）
- 依存関係が少ない
- 削除による影響範囲が明確
- テストが既存のもので十分

### P2 候補（JoinIR 拡張）
- PhiSpec や IfMerge との 1:1 対応が明確
- 既存の JoinIR 構造に自然に統合できる
- 削減効果が高い（100行以上）

---

## Phase 61-6 削減候補（2〜3個）

### Phase 61-6.1: P1 薄いラッパー削除

#### 1. **`set_if_context` 削除**
   - **ファイル**: `src/mir/phi_core/phi_builder_box.rs:143-152`
   - **削減見込み**: 10行（関数本体）+ 呼び出し側1行 = **11行**
   - **実装方針**: `loop_builder/if_lowering.rs:259-262` で直接 `IfPhiContext::for_loop_body(true, carrier_names.clone())` を生成してフィールドに代入
   - **リスク**: **低** - 単純なラッパー削除、既存ロジックに影響なし
   - **備考**: 現在は `set_if_context()` → `IfPhiContext { in_loop_body, loop_carrier_names }` 構造体生成のみ

#### 2. **If Lowering ルーティング関数のデフォルト有効化**
   - **ファイル**: `src/mir/join_ir/lowering/mod.rs:132-141`
   - **削減見込み**: dev フラグチェック削除 8行 + 環境変数関数削除 = **15行**
   - **実装方針**: `joinir_if_select_enabled()` チェックを削除、デフォルトで有効化
   - **リスク**: **中** - 全関数でデフォルト有効化されるため、テスト全通過が必須
   - **備考**: 現在は `NYASH_JOINIR_IF_SELECT=1` で制御、Phase 33-8 で導入済み

---

### Phase 61-6.2: P2 JoinIR 拡張

#### 3. **A/B テスト観察用関数の削除**
   - **関数**: `extract_phi_spec_from_builder`, `compare_and_log_phi_specs`
   - **ファイル**: `src/mir/join_ir/lowering/if_phi_spec.rs:97-140`
   - **削減見込み**: 2関数 + テスト削除 = **50行**
   - **実装方針**:
     1. Phase 61-3 で JoinIR 経路が完全動作確認済み
     2. `if_lowering.rs:273-290` の A/B 比較ロジック削除
     3. `extract_phi_spec_from_builder()` と `compare_and_log_phi_specs()` 削除
     4. if_phi_spec.rs は `compute_phi_spec_from_joinir()` のみ残す
   - **リスク**: **低** - 観察専用関数、実行経路に影響なし
   - **備考**: Phase 61-2 で導入、Phase 61-3 で検証完了済み

---

## 期待される効果

| Wave | 削減対象 | 見込み削減行数 | 累積削減 |
|------|---------|--------------|---------|
| 61-6.1 | P1 薄いラッパー削除 (set_if_context) | 11 | 11 |
| 61-6.1 | P1 dev フラグ削除 | 15 | 26 |
| 61-6.2 | P2 A/B テスト削除 | 50 | **76** |

### JoinIR カバー率
- **現在**: If-in-Loop PHI 生成は JoinIR で 100% カバー（Phase 61-3 完了）
- **61-6 後**: PhiBuilderBox 経路の観察コード削除、JoinIR SSOT 確立

---

## 実装順序

### Wave 1: P1 削除（Phase 61-6.1）
1. **`set_if_context` 削除** - 最優先（最も安全）
2. **dev フラグ削除** - 全テストPASS確認後

### Wave 2: P2 削除（Phase 61-6.2）
1. **A/B テスト観察関数削除** - 最も削減効果が高い

### 各 Wave の完了条件
- Wave 1: 全 JoinIR tests PASS（268 tests）
- Wave 2: if_phi_spec.rs が `compute_phi_spec_from_joinir()` のみに（観察コード完全削除）

---

## 実装時の注意事項

1. **Fail-Fast原則**: エラーは即座に失敗、フォールバック禁止
2. **決定的順序**: BTreeSet/BTreeMap で ValueId 割り当ての決定性保証
3. **箱理論**: SSOT を JoinIR 側に寄せる、PhiBuilderBox は最小限に
4. **テスト**: 各ステップで全 JoinIR tests PASS を確認
5. **ドキュメント**: PHI_BOX_INVENTORY.md と CURRENT_TASK.md を更新

---

## 保留候補（Phase 61-7+ で検討）

### P2 候補（Phase 61-7 で検討）
1. **`compute_modified_names_if`** - 変更変数検出を JoinIR の modified 変数集合解析に統合（~75行削減）
2. **`get_conservative_if_values`** - incoming 値解決を PhiSpec に移行、void fallback 削除（~70行削減）

### 保留理由
- Phase 61-6 では最小限の削除に集中（76行削減）
- P2 大型統合は Phase 61-7 で慎重に実装（145行削減見込み）

---

## 関連ドキュメント

- Phase 61-5.3: [if-phi-priority-table.md](phase61-5-3-if-phi-priority-table.md)
- PHI Inventory: [PHI_BOX_INVENTORY.md](../../../docs/private/roadmap2/phases/phase-30-final-joinir-world/PHI_BOX_INVENTORY.md)
- If PHI Context: [if_phi_context.rs](../../../src/mir/join_ir/lowering/if_phi_context.rs)
- If PHI Spec: [if_phi_spec.rs](../../../src/mir/join_ir/lowering/if_phi_spec.rs)
Status: Historical
