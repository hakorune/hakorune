# Legacy 候補一覧

Phase 256+ で特定された、将来削除可能な legacy コード候補を記録する。

---

## Phase 256 P1.7 で特定された legacy

### `join_func_name()` (src/mir/join_ir_vm_bridge/mod.rs)

**状態**: 未使用になった可能性あり（要精査）

**理由**:
- Phase 256 P1.7 で `continuation_func_ids` が `String` ベースに変更
- `join_func_name()` による `JoinFuncId → String` 変換が不要になった可能性
- JoinFunction の `name` フィールドが直接利用可能

**残利用箇所**: 11箇所 + 定義1箇所 = 計12箇所

#### 実利用箇所（10箇所）

1. **src/mir/join_ir_vm_bridge/runner.rs:55**
   ```rust
   let entry_name = join_func_name(entry_func);
   ```
   - 用途: VM実行時のエントリーポイント関数名取得
   - 状態: 実利用（Phase 27-shortterm S-4.3）

2. **src/mir/join_ir_vm_bridge/joinir_block_converter.rs:162**
   ```rust
   let func_name = join_func_name(*func);
   ```
   - 用途: JoinIR ブロック変換時の関数名取得
   - 状態: 実利用（Phase 190）

3. **src/mir/join_ir_vm_bridge/normalized_bridge/direct.rs:94**
   ```rust
   name: join_func_name(JoinFuncId(func.id.0)),
   ```
   - 用途: Normalized → MIR 変換時の関数署名生成
   - 状態: 実利用（Phase 141+）
   - 条件: `#[cfg(feature = "normalized_dev")]`

4. **src/mir/join_ir_vm_bridge/normalized_bridge/direct.rs:182**
   ```rust
   let func_name = join_func_name(JoinFuncId(target.0));
   ```
   - 用途: Normalized 関数ターゲット名取得
   - 状態: 実利用（Phase 141+）
   - 条件: `#[cfg(feature = "normalized_dev")]`

5. **src/mir/join_ir_vm_bridge/joinir_function_converter.rs:39**
   ```rust
   .insert(join_func_name(*func_id), mir_func);
   ```
   - 用途: MIR モジュールへの関数登録
   - 状態: 実利用（Phase 190）

6. **src/mir/builder/control_flow/joinir/merge/tail_call_lowering_policy.rs:100**
   ```rust
   let k_exit_name = join_func_name(JoinFuncId::new(2));
   ```
   - 用途: テストコード内での k_exit 関数名生成
   - 状態: テスト専用（`#[cfg(test)]`）

7. **src/mir/control_tree/normalized_shadow/loop_true_break_once.rs:63**
   ```rust
   use crate::mir::join_ir_vm_bridge::join_func_name;
   ```
   - 用途: テストコード内での関数名取得
   - 状態: テスト専用（`#[cfg(test)]`）

#### コメント参照箇所（3箇所）

8. **src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs**
   ```rust
   // No need to convert with join_func_name() - use directly
   ```
   - 用途: コメント（変換不要の説明）
   - 状態: ドキュメント

9. **src/mir/join_ir_vm_bridge/meta.rs**
   ```rust
   // Phase 256 P1.7: Use actual function name instead of join_func_name()
   // join_func_name() produces "join_func_{id}" but JoinFunction.name contains
   ```
   - 用途: コメント（Phase 256 P1.7 の設計変更説明）
   - 状態: ドキュメント

10. **src/mir/builder/control_flow/joinir/patterns/pattern7_split_scan.rs**
    ```rust
    // The bridge uses JoinFunction.name as the MirModule function key, not join_func_name(id)
    ```
    - 用途: コメント（Pattern 7 の設計説明）
    - 状態: ドキュメント

#### 定義箇所（1箇所）

11. **src/mir/join_ir_vm_bridge/mod.rs**
    ```rust
    pub(crate) fn join_func_name(id: JoinFuncId) -> String {
    ```
    - 用途: 関数定義
    - 状態: pub(crate) で公開

---

## 推奨アクション

### 短期（Phase 256 P1.7 スコープ外）
- ✅ このドキュメントに記録完了
- 実装変更は行わない（記録のみ）

### 中期（Phase 257+ で検討）
1. **段階的移行案**:
   - Step 1: 新規コードでは `JoinFunction.name` を直接使用
   - Step 2: 既存の10実利用箇所を段階的に置換
   - Step 3: テストコード（2箇所）を置換
   - Step 4: コメント更新（3箇所）
   - Step 5: `join_func_name()` 削除

2. **影響範囲**:
   - Bridge 層: 7箇所（実利用6 + テスト1）
   - Normalized 層: 2箇所（feature gated）
   - Control flow builder: 1箇所（テストのみ）
   - Control tree: 1箇所（テストのみ）

3. **リスク評価**:
   - 低: テストコードのみ（2箇所）
   - 中: Bridge 層の実利用（7箇所）
   - 高: Normalized 層（feature gated, 現在開発中）

4. **優先度**: 低（現在問題なし、技術的負債削減として）

---

## メンテナンス

- **作成日**: 2025-12-20
- **最終更新**: 2025-12-20
- **関連 Phase**: 256 P1.7
- **関連コミット**: (Phase 256 P1.7 完了後に記入)

---

## 参考リンク

- Phase 256 P1.7 設計: [phase-256/README.md](../phases/phase-256/README.md)
- JoinIR 設計マップ: [design/joinir-design-map.md](../design/joinir-design-map.md)
- JoinIR アーキテクチャ: [joinir-architecture-overview.md](../joinir-architecture-overview.md)

---

## Phase 29bq / 2026-02-02 inventory（legacy/compat 候補）

### 1) JoinIR legacy routing/binding
- `src/mir/builder/control_flow/joinir/legacy/routing_legacy_binding.rs`
- `src/mir/builder/control_flow/joinir/patterns/legacy.rs`
- `src/mir/builder/control_flow/joinir/routing.rs`（legacy whitelist）
- 理由: legacy 経路の互換維持。SSOT 化が進んだら段階的に削除候補。

### 2) Normalized shadow の legacy 経路
- `src/mir/control_tree/normalized_shadow/legacy/mod.rs`
- `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs`
- 理由: 旧挙動互換・テスト用途。削除可否の判断材料として残存。

### 3) Legacy call emission 経路
- `src/mir/builder/calls/emit.rs`
- `src/mir/builder/calls/unified_emitter.rs`
- `src/mir/builder/ops/comparison.rs`
- 理由: 旧パスの名残。現行 SSOT へ寄せる対象。

### 4) JoinIR bridge の legacy jump-args
- `src/mir/join_ir_vm_bridge/handlers/call.rs`
- `src/mir/join_ir_vm_bridge/handlers/jump.rs`
- `src/mir/join_ir_vm_bridge/joinir_block_converter.rs`（legacy jump args）
- 理由: 旧引数形式の互換保持。削除順序の検討対象。

### 5) Legacy env flags（棚卸し対象）
- `src/config/env/joinir_dev.rs`
- `src/config/env/parser_flags.rs`
- `src/config/env/mir_flags.rs`
- 理由: 旧フラグの整理候補。使用実績と撤去計画の棚卸し対象。

### 6) TODO クラスタ（旧仕様の残り）
- `src/mir/builder/emit_guard/mod.rs`（BlockScheduleBox TODO）
- `src/mir/loop_pattern_detection/mod.rs`
- `src/mir/join_ir_vm_bridge/meta.rs`
- `src/mir/join_ir/lowering/loop_patterns/mod.rs`
- 理由: legacy/暫定の TODO が集中。段階的に解消方針を決める。

### 7) direct Copy emission（SSOT 逸脱）
- `src/runner/mir_json_v0.rs`
- `src/runner/json_v1_bridge/parse.rs`
- `src/tests/mir_joinir_if_select.rs`
- 理由: CopyEmitter を迂回。削除または許可範囲の明確化が必要。

**メモ**: 実装変更は行わず、削除順序の候補を固定するための棚卸し。
