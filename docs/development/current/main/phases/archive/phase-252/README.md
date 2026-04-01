Status: Completed
Scope: Phase 252 (JoinIR LoopBreak 条件: `this.methodcall(...)` 対応 + policy SSOT)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/phases/phase-251/README.md

# Phase 252: LoopBreak 条件の `this.methodcall(...)` 対応

## 目的

- `--profile quick` の `json_lint_vm` で露出した JoinIR LoopBreak route（historical label `2`）の回帰を潰す。
- 具体的には `if not this.is_whitespace(s.substring(i, i + 1)) { break }` のような
  `this.methodcall(...)` を break 条件として lowering できるようにする。

## 実装（P0/P1: 完了）

### 1) ユーザー定義メソッドの許可ポリシー（SSOT）

ファイル:
- `src/mir/join_ir/lowering/user_method_policy.rs`

要点:
- CoreMethodId（builtin）とは別に、`this.methodcall(...)` の「許可」を一箇所に集約する。
- by-name の if 分岐で散らさず、ポリシーテーブルとして SSOT 化する。

### 2) ConditionLowerer: `ASTNode::MethodCall(object: Me, ...)` の受理

ファイル:
- `src/mir/join_ir/lowering/condition_lowerer.rs`

要点:
- break 条件のトップレベルが `MethodCall(Me, ...)` の場合に lowering できる分岐を追加。
- `this` の所属 box 名は `current_static_box_name` を経由して受け取る（固定名分岐しない）。

### 3) `current_static_box_name` の配線（LoopBreak まで）

変更点:
- `ConditionContext` に `current_static_box_name` を追加
- LoopBreak lowering 入力（inputs）から break/header 条件 lowering まで `current_static_box_name` を伝搬

注:
- ここは “構造” による情報伝達であり、特定関数名での回避分岐（ハードコード）ではない。

### 4) 局所リファクタ（DebugOutputBox 統一）

ファイル:
- `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs`

要点:
- 無条件/散在ログを追加しない方針を維持しつつ、出力 API を `DebugOutputBox` に統一する。
- デフォルトでは出力ゼロ（smoke の期待出力を壊さない）。

### 5) テスト/fixture の追加

- unit tests を追加（`this.methodcall(...)` 条件の lowering 回帰固定）
- v2 smoke fixture を追加（integration profile）

## 検証状況（Phase 252 終点）

- `cargo check` が通る（0 errors、warnings のみ）
- `--profile quick` の最初の FAIL は次に切り出し（Phase 253）:
  - `[joinir/mutable-acc-spec] Assignment form not accumulator pattern (required: target = target + x)`

## 次の作業（Phase 253）

次の SSOT: `docs/development/current/main/phases/archive/phase-253/README.md`
