# Phase 29ai P11: Planner support + wiring（loop_break subset; historical Pattern2 label）

Date: 2025-12-29
Status: Historical reference (implemented)
Scope: loop_break route（break-only subset）を Facts→Planner の subset として吸収し、single_planner で planner-first を始めた slice（仕様不変）
Goal: 「入口は一本（Facts→Planner→historical plan payload）」を維持しつつ、loop_break route を段階的に plan 系へ寄せる

Historical note:
- `DomainPlan::Pattern2Break` と historical payload / file / fixture token 群は
  inventory SSOT に退避しているよ。
- current runtime では semantic vocabulary を使い、facts 側は `LoopBreakFacts`,
  `loop_break`, `loop_break_body_local` に揃っているよ。
- current semantic wrapper / fixture alias:
  - `tools/smokes/v2/profiles/integration/joinir/loop_break_plan_subset_vm.sh`
  - `apps/tests/loop_break_plan_subset_min.hako`
- exact historical basename が必要なら
  `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md`
  と
  `docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md`
  を見ること。

## Objective

loop_break route は当時 `DomainPlan::Pattern2Break` を持っていたが、抽出の多くは legacy JoinIR lowering に残っていた。
P11 では “PoC subset” の loop_break（break-only, 単純形）だけを Facts→Planner で historical plan payload まで到達させ、
`single_planner` の loop_break rule でも planner-first を開始する。

既存の loop_break（body-local promotion / NotApplicable / Freeze 等）の挙動は変えず、
planner が `Ok(None)` を返す限り従来経路に落とす。

## Non-goals（この P11 ではやらない）

- loop_break の全網羅（body-local promotion / derived slot / real-world seg 等）
- NotApplicable/Freeze の契約変更
- 既存 fixture の期待値（RC/ログ/エラー文字列）変更
- 新しい env var / debug トグル追加
- by-name ルールや特定 Box 名での分岐追加（禁止）

## Target Subset（PoC）

Historical execution note: 当時の extractor path token は `plan/extractors/pattern2_break.rs` だったよ。
P11 が固定した最小形は次の break-only loop_break subset だよ:

```hako
loop(i < N) {
    if (break_cond) { [carrier = expr;] break }
    carrier = carrier + expr
    i = i + 1
}
```

この形だけを Facts→Planner に吸収し、それ以外は `Ok(None)` で従来経路にフォールバックする。

## Implementation Steps（Critical Order）

### Step 1: loop_break facts を追加（Facts層）

Current semantic outcome:
- `src/mir/builder/control_flow/plan/facts/loop_break_core.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_types.rs`
- `src/mir/builder/control_flow/plan/facts/loop_types.rs`（`LoopFacts.loop_break` 接続）

方針:
- Facts は “構造的事実” だけを保持し、推測はしない（抽出できない場合は `Ok(None)`）。
- historical plan payload は Facts から直接作らない（planner の責務）。

最低限必要な情報（例）:
- `loop_var`, `carrier_var`
- `break_condition`（AST clone）
- `carrier_update_in_break`（Option<AST clone>）
- `carrier_update_in_body`（AST clone）
- `loop_increment`（AST clone）

### Step 2: planner で loop_break candidate を生成（historical payload token: `DomainPlan::Pattern2Break`）

ファイル:
- `src/mir/builder/control_flow/plan/planner/mod.rs`

やること:
- `facts.facts.loop_break` が `Some` のとき、CandidateSet に historical loop_break payload を push。
- 0/1/2+ の境界は CandidateSet の finalize に委譲（candidate-set 方式を崩さない）。

### Step 3: single_planner で loop_break を planner-first に

ファイル:
- `src/mir/builder/control_flow/plan/single_planner/rules.rs`

やること:
- scan_with_init / split_scan と同様に、loop_break rule でも先頭で planner を試す。
- `Ok(Some(DomainPlan::Pattern2Break(_)))` のときだけ採用（historical payload token）。
- `Ok(Some(other))` は採用せず（将来拡張用に一旦 skip）、`Ok(None)` は従来どおり legacy（plan/extractor 経由）へ。
- planner の `Ok(None)` で新規ログを出さない（観測差分を抑える）。

### Step 4: SSOT fixture/smoke を追加（PoC subset 固定）

追加（推奨）:
- current semantic fixture alias: `apps/tests/loop_break_plan_subset_min.hako`
- current semantic wrapper: `tools/smokes/v2/profiles/integration/joinir/loop_break_plan_subset_vm.sh`
- historical fixture pin / archive basename は inventory SSOT に退避

方針:
- 既存 loop_break 複雑系 fixture は変えない（回帰パック維持）。
- P11 は “subset が planner で採用される” ことだけを固定する。

### Step 5: Docs / Tracking 更新

更新:
- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

最低限:
- P11 完了の記録（目的/影響/検証コマンド）。
- Next（P12）候補を 1 行（例: loop_break body-local promotion の Facts 仕様化）。

## Verification Checklist（Acceptance）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile integration --filter "loop_break_plan_subset_vm"`

期待:
- quick/regression は既定挙動不変で PASS
- subset fixture は PASS（planner-first で採用される）

## Risk Notes

- loop_break は複雑なので、P11 は “subset のみ” を厳密にする（疑わしければ `Ok(None)`）。
- 既存の loop_break 実運用（body-local など）に影響を与えないことが最優先。
