---
Status: SSOT
Scope: Facts の reject 判定を構造化し、handoff 先を 1 箇所に集約
Related:
- plan-reject-handoff-gap-taxonomy-ssot.md
- src/mir/builder/control_flow/plan/facts/reject_reason.rs
---

# Planner Entry Guards SSOT

## 目的

Facts の `Ok(None)` 黙殺を減らし、reject_reason → handoff を構造化。
「どのレゴを足すべきか」がログ 1 行で分かる状態にする。

## RejectReason enum

実装:

- vocabulary / handoff / log format:
  `src/mir/builder/control_flow/plan/facts/reject_reason.rs`
- last reject-detail diagnostic state:
  `src/mir/builder/control_flow/verify/diagnostics/planner_reject_detail.rs`

```rust
pub enum RejectReason {
    ReturnInBody,
    ReturnInBranch,
    ReturnInBothBranches,
    BreakPresent,
    ContinueInIfWithElse,
    TopLevelNestedLoop,
    NoContinue,
    UnsupportedCondition,
    // ... 拡張可能
}
```

## HandoffTarget enum

```rust
pub enum HandoffTarget {
    LoopCondBreakContinue,
    LoopCondReturnInBody,
    LoopCondContinueWithReturn,
    OutOfScope,  // 現時点で受け皿なし
}
```

## Per-box Handoff Tables

同じ `RejectReason` でも箱によって `handoff` が異なり得るため、箱ごとにテーブルを持つ。

```rust
pub mod handoff_tables {
    /// loop_cond_continue_only 用
    pub fn for_loop_cond_continue_only(reason: RejectReason) -> HandoffTarget { ... }

    /// loop_cond_break_continue 用
    pub fn for_loop_cond_break_continue(reason: RejectReason) -> HandoffTarget { ... }

    /// generic_loop 用（現状は OutOfScope へ）
    pub fn for_generic_loop(reason: RejectReason) -> HandoffTarget { ... }

    /// match_return_facts 用（現状は OutOfScope へ）
    pub fn for_match_return_facts(reason: RejectReason) -> HandoffTarget { ... }
}
```

## ログ契約

`HAKO_JOINIR_DEBUG=1` 時のみ出力:

```
[plan/reject] box=loop_cond_continue_only reason=return_in_body handoff=loop_cond_break_continue
[plan/reject] box=generic_loop_v0 reason=no_valid_loop_var_candidates handoff=out_of_scope
[plan/reject] box=generic_loop_v0 reason=control_flow_after_in_body_step handoff=out_of_scope
[plan/reject] box=generic_loop_v0 reason=exit_after_in_body_step handoff=out_of_scope
[plan/reject] box=generic_loop_v0 reason=break_else_step_must_be_final_stmt handoff=out_of_scope
[plan/reject] box=match_return_facts reason=match_return_else_not_literal handoff=out_of_scope
```

フォーマット: `[plan/reject] box=<box_name> reason=<reason> handoff=<target>`

**挙動不変**: `log_reject` はログ出力のみを担当する。strict/non-strict の `Freeze` / `Ok(None)` / `Ok(false)` 判定は従来の `reject_or_none` / `reject_or_false` 関数が担当し、`log_reject` は実行フローに影響しない。

### Reject Detail 診断 state

`[joinir/reject_detail]` に使う last-detail state は diagnostics 側が所有する。

実装: `src/mir/builder/control_flow/verify/diagnostics/planner_reject_detail.rs`

契約:

- `cf_loop` の先頭で clear する。
- facts-level `log_reject` は detail を上書きできる。
- route exhaustion / whitelist miss は `set_if_absent` 相当で、既存の facts-level detail を潰さない。
- planner-required freeze / final JoinIR freeze は `take` で消費する。

### [plan/accept] ログ

`HAKO_JOINIR_DEBUG=1` 時のみ出力:

```
[plan/accept] box=loop_cond_break_continue accept=return_in_exit_if
[plan/accept] box=loop_cond_break_continue accept=return_only_body
```

フォーマット: `[plan/accept] box=<box_name> accept=<tag>`

**挙動不変**: `log_accept` はログ出力のみを担当する。accept/reject の判定ロジックには影響しない。

## Freeze メッセージ SSOT

`RejectReason::as_freeze_message()` で Freeze 文言を統一。

| RejectReason | Freeze message |
|-------------|----------------|
| NoValidLoopVarCandidates | "no valid loop_var candidates found" |
| AmbiguousLoopVarCandidates | "multiple loop_var candidates matched (ambiguous)" |
| MultipleConditionalStepAssignments | "generic loop v0.2: multiple conditional step assignments in body" |
| MultipleStepAssignments | "generic loop v0.2: multiple step assignments in body" |
| MatchReturnScrutineeNotSupported | "match return scrutinee must be var or int literal" |
| MatchReturnTooFewArms | "match return requires >= 2 arms" |
| MatchReturnElseNotLiteral | "match return else must be literal" |
| MatchReturnArmLabelNotSupported | "match return arm label must be int/bool literal" |
| MatchReturnArmNotLiteral | "match return arm must be literal" |
| MatchReturnArmLiteralTypeUnsupported | "match return arm literal type unsupported" |

**注**: NoValidLoopVarCandidates は現在 `Ok(None)` 経路のため未使用。将来の strict freeze 化に備えた登録。

**canon 層対応** (14/N): `canon/generic_loop.rs` の `StepPlacementDecision.reject_reason` も `Option<RejectReason>` に型変更済み。canon 層からも構造化された reject reason を返すようになった。

**契約**: `as_freeze_message()` は対応済み reason のみ呼び出すこと。未対応は debug_assert で検出。

### Freeze 文字列ポリシー（回帰防止）

- Facts 側で `Freeze::unsupported("...")` の生文字列を新規追加しない。
- strict で Freeze する場合は、`RejectReason` を選び、Freeze 文言は `RejectReason::as_freeze_message()` を使う。
- 例外が必要なら、このSSOTに「理由」と「撤去条件」を明記してから追加する（silent/temporary禁止）。

## 使用方法

```rust
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    RejectReason, log_reject, handoff_tables,
};

// reject 時に呼び出す
log_reject(
    "loop_cond_continue_only",
    RejectReason::ReturnInBody,
    handoff_tables::for_loop_cond_continue_only,
);
return Ok(None);
```

## 拡張手順

### 新しい RejectReason を追加する場合

1. `reject_reason.rs` の `RejectReason` enum に variant 追加
2. `Display` impl に対応する文字列追加
3. 対象箱の `handoff_tables::for_*()` に match arm 追加

### 新しい箱に適用する場合

1. `handoff_tables` モジュールに `for_<box_name>()` 関数追加
2. 対象箱の `facts.rs` で `log_reject()` 呼び出しに置換

## スコープ外のログ種別

以下のログ種別は本 SSOT のスコープ外とし、別途管理する:

- `[plan/trace]`: ai-handoff SSOT に従う（ここでは増やさない）
- `MATCHED` / `detected`: 開発用の一時ログとして当面維持。整理対象は reject/accept を優先する
