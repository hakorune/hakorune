# Phase 236-EX: ExprLowerer/ScopeManager 本番導入（Pattern2 条件）

目的: Phase 231/235 でパイロット実装・テストしてきた ExprLowerer/ScopeManager を、Pattern2 の break 条件 lowering に実際に使ってみて、既存の condition_to_joinir ベース経路と挙動が一致することを確認するフェーズだよ。影響範囲は Pattern2 の「break 条件」だけに限定する。

---

## 1. 現状の経路（Task 236-1 メモ）

### 1.1 関連ファイル

- `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`
  - Pattern 2 のルーティングと、`lower_loop_with_break_minimal` 呼び出しの入口。
  - Phase 231 で ExprLowerer による **validation-only** 呼び出しが挿入されている。
- `src/mir/join_ir/lowering/loop_with_break_minimal.rs`
  - Pattern2 本体の JoinIR ライン。
  - `lower_loop_with_break_minimal()` の中で、`condition_to_joinir::lower_condition_to_joinir` を用いて
    - ループ条件
    - break 条件
    の両方を JoinIR に lower している。
- `src/mir/join_ir/lowering/expr_lowerer.rs`
  - Phase 231/235: ExprLowerer 本体。ScopeManager から ConditionEnv を組み立てて `condition_lowerer::lower_condition_to_joinir` に委譲する薄い箱。
- `src/mir/join_ir/lowering/scope_manager.rs`
  - Phase 231: `Pattern2ScopeManager` 実装。ConditionEnv / LoopBodyLocalEnv / CapturedEnv / CarrierInfo を束ねて、名前→ValueId / VarScopeKind を返す。
- `src/mir/join_ir/lowering/condition_to_joinir.rs`
  - 既存の condition lowering オーケストレータ。`ConditionEnv` + `lower_condition_to_joinir` + 変数抽出をまとめた API。

### 1.2 現在の break 条件 lowering の流れ

1. `pattern2_with_break.rs` で:
   - LoopScopeShape や LoopBodyLocalEnv を構築。
   - ConditionEnvBuilder v2 相当で `env: ConditionEnv` を用意。
   - `carrier_info` / `carrier_updates` など Pattern2 メタ情報を集約。
2. Phase 231 追加分:
   - `Pattern2ScopeManager` を上記 env + body_local_env + captured_env + carrier_info から組み立てる。
   - ExprLowerer を `ExprContext::Condition` で生成し、`effective_break_condition` を **一度 lower してみるだけ**（結果 ValueId は捨てる）。
   - UnsupportedNode / VariableNotFound などはログ出力にとどめ、実際の lowering には関与しない。
3. 本番 lowering:
   - `lower_loop_with_break_minimal(...)` を呼び出し、
   - その内部で `condition_to_joinir::lower_condition_to_joinir` により
     - ループ条件 AST
     - break 条件 AST
     をそれぞれ `env: ConditionEnv` を使って JoinIR 命令列 & ValueId に変換している。

### 1.3 Phase 236 の狙い

- `loop_with_break_minimal.rs` 内の break 条件 lowering を、
  - 「ConditionEnv → lower_condition_to_joinir」から、
  - 「ScopeManager + ExprLowerer（内部は既存 lower_condition_to_joinir を利用）」に置き換える。
- ループ条件（自然終了条件）の lowering は従来どおり condition_to_joinir 経路のまま保持し、差分を break 条件のみに限定する。

---

## 2. 次フェーズとのつながり（Phase 237-EX 予告）

- Phase 237-EX では JsonParser/selfhost の条件パターンを棚卸しし、ExprLowerer/ScopeManager が今後どの条件を優先的に扱うかのカタログを作る予定だよ。
- 参照: `phase237-exprlowerer-condition-catalog.md`（条件パターン一覧と ExprLowerer サポート状況の SSOT）。

---

## 3. その先（Phase 238-EX との接続）

- Phase 238-EX で Scope/Env の境界ルールを文書化し、ExprLowerer/ScopeManager/ConditionEnv/LoopBodyLocalEnv/UpdateEnv の責務を固定する予定。
- 参照: `phase238-exprlowerer-scope-boundaries.md`（境界ルールと将来のガード案）。***
Status: Active  
Scope: ExprLowerer 統合（JoinIR/ExprLowerer ライン）
