# Phase 29ai P11: Planner support + wiring（Pattern2 break subset）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Pattern2（break）を Facts→Planner の subset として吸収し、single_planner で planner-first を開始する（仕様不変）  
Goal: 「入口は一本（Facts→Planner→DomainPlan）」を維持しつつ、Pattern2 を段階的に plan 系へ寄せる

## Objective

Pattern2 は現在でも `DomainPlan::Pattern2Break` を持っているが、抽出の多くは legacy JoinIR lowering に残っている。
P11 では “PoC subset” の Pattern2（break-only, 単純形）だけを Facts→Planner で `Ok(Some(DomainPlan::Pattern2Break))` まで到達させ、
`single_planner` の Pattern2 rule でも planner-first を開始する。

既存の Pattern2（LoopBodyLocal promotion / NotApplicable / Freeze 等）の挙動は変えず、planner が `Ok(None)` を返す限り従来経路に落とす。

## Non-goals（この P11 ではやらない）

- Pattern2 の全網羅（LoopBodyLocal promotion / derived slot / real-world seg 等）
- NotApplicable/Freeze の契約変更
- 既存 fixture の期待値（RC/ログ/エラー文字列）変更
- 新しい env var / debug トグル追加
- by-name ルールや特定 Box 名での分岐追加（禁止）

## Target Subset（PoC）

`plan/extractors/pattern2_break.rs` が扱っている最小形（例）:

```hako
loop(i < N) {
    if (break_cond) { [carrier = expr;] break }
    carrier = carrier + expr
    i = i + 1
}
```

この形だけを Facts→Planner に吸収し、それ以外は `Ok(None)` で従来経路にフォールバックする。

## Implementation Steps（Critical Order）

### Step 1: Pattern2BreakFacts を追加（Facts層）

ファイル（推奨）:
- `src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs` ✨ NEW
- `src/mir/builder/control_flow/plan/facts/mod.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`（接続）

方針:
- Facts は “構造的事実” だけを保持し、推測はしない（抽出できない場合は `Ok(None)`）。
- `DomainPlan` を直接作らない（planner の責務）。

最低限必要な情報（例）:
- `loop_var`, `carrier_var`
- `break_condition`（AST clone）
- `carrier_update_in_break`（Option<AST clone>）
- `carrier_update_in_body`（AST clone）
- `loop_increment`（AST clone）

### Step 2: planner で DomainPlan::Pattern2Break を生成

ファイル:
- `src/mir/builder/control_flow/plan/planner/build.rs`

やること:
- `facts.facts.pattern2_break` が `Some` のとき、CandidateSet に `DomainPlan::Pattern2Break(...)` を push。
- 0/1/2+ の境界は CandidateSet の finalize に委譲（candidate-set 方式を崩さない）。

### Step 3: single_planner で Pattern2 を planner-first に

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- Pattern6/7 と同様に、Pattern2 rule でも先頭で `planner::build_plan(ctx.condition, ctx.body)` を試す。
- `Ok(Some(DomainPlan::Pattern2Break(_)))` のときだけ採用。
- `Ok(Some(other))` は採用せず（将来拡張用に一旦 skip）、`Ok(None)` は従来どおり legacy（plan/extractor 経由）へ。
- planner の `Ok(None)` で新規ログを出さない（観測差分を抑える）。

### Step 4: SSOT fixture/smoke を追加（PoC subset 固定）

追加（推奨）:
- `apps/tests/phase29ai_pattern2_break_plan_subset_ok_min.hako`
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh`

方針:
- 既存 Pattern2 の複雑系 fixture は変えない（回帰パック維持）。
- P11 は “subset が planner で採用される” ことだけを固定する。

### Step 5: Docs / Tracking 更新

更新:
- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

最低限:
- P11 完了の記録（目的/影響/検証コマンド）。
- Next（P12）候補を 1 行（例: Pattern2 LoopBodyLocal promotion の Facts 仕様化）。

## Verification Checklist（Acceptance）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ai_pattern2_break_plan_subset_ok_min"`

期待:
- quick/regression は既定挙動不変で PASS
- subset fixture は PASS（planner-first で採用される）

## Risk Notes

- Pattern2 は複雑なので、P11 は “subset のみ” を厳密にする（疑わしければ `Ok(None)`）。
- 既存の Pattern2 実運用（LoopBodyLocal など）に影響を与えないことが最優先。

