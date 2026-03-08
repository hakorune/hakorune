---
Status: Ready
Scope: code+tests+docs（仕様不変・strict/devのみ観測強化）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_vm.sh
  - tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_seg_vm.sh
  - src/mir/builder/control_flow/plan/facts/loop_break_core.rs
  - src/mir/builder/control_flow/plan/facts/loop_break_body_local_facts.rs
  - src/mir/builder/control_flow/plan/planner/build.rs
  - src/mir/builder/control_flow/plan/composer/shadow_adopt.rs
---

# Phase 29ao P33: LoopBreak body-local（phase29ab / historical fixture token / label 2）を planner 由来に引き上げ、shadow adopt タグを回帰で固定する

Date: 2025-12-30  
Status: Ready for execution  
Goal: `loop_break_body_local_{min,seg_min}.hako`（historical fixture token: `phase29ab_pattern2_loopbodylocal_{min,seg_min}`）が strict/dev で **planner 由来の `DomainPlan::LoopBreak`** になり、既存の historical debug tag
`[coreplan/shadow_adopt:pattern2_break_subset]` が必ず出るようにして回帰で固定する。

## 背景

- `loop_break_body_local*` current semantic lane に対応する historical fixture family `phase29ab_pattern2_loopbodylocal_*` は現状、promotion hint タグ（`[plan/loop_break/promotion_hint:*]`）は出るが、
  `outcome.plan` が `LoopBreak` ではないため shadow adopt が走らず、CorePlan 経路の差分検知ができない。
- Phase 29ao の “段階1（strict/dev）完了” は、「回帰パックに含まれる代表ケースが shadow adopt タグで検知できる」状態。
  historical label 2 の realworld（phase263）は P32 で埋めたので、次は phase29ab の LoopBodyLocal を埋めるのが自然。

## 非目的

- release 既定挙動の変更
- NotApplicable/Freeze ケースの “無理な planner 化”
- 新しい env var 追加
- タグ名の増殖（既存タグを流用する）

## 受け入れ基準（Acceptance）

- `cargo build --release` が通る
- `./tools/smokes/v2/run.sh --profile quick` が通る（既定挙動不変）
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が通る
- 次の 2 つの current integration smoke が strict/dev の raw output で shadow adopt タグを必須で満たす:
  - `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_vm.sh`
  - `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_seg_vm.sh`

## 実装手順（安全順）

### Step 1: Facts 抽出を “phase29ab の LoopBodyLocal 形状” に対応させる

対象:
- `src/mir/builder/control_flow/plan/facts/loop_break_core.rs`

方針:
- 誤マッチ防止優先で、`Ok(None)` を維持しつつ “この2 fixture” を拾える最小条件を追加する。
- `loop_break_body_local_facts` は既に promotion hint を出しているので、
  **break/loop_increment 側の facts 取りこぼし**が主因になっている可能性が高い。

最低限の SSOT:
- `apps/tests/loop_break_body_local_min.hako`
- `apps/tests/loop_break_body_local_seg_min.hako`
- historical fixture token:
  - historical fixture token: `phase29ab_pattern2_loopbodylocal_min.hako`
  - historical fixture token: `phase29ab_pattern2_loopbodylocal_seg_min.hako`

### Step 2: planner で LoopBreak を候補に出す（planner 由来にする）

対象:
- `src/mir/builder/control_flow/plan/planner/build.rs`

方針:
- `facts.facts.loop_break.is_some()` のとき、`DomainPlan::LoopBreak(..)` の candidate を push する。
- `facts.facts.loop_break_body_local` があれば `promotion` を乗せる（既存の composer と同じ）。

注意:
- `loop_break` が取れないケース（freeze/notapplicable）は candidate を出さない（Ok(None) を維持）。

### Step 3: shadow adopt の gate は既存のまま（タグは流用）

確認対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

このファイルは既に
- `DomainPlan::LoopBreak` かつ `outcome.plan` が `LoopBreak` のときだけ adopt
- タグは historical debug token `[coreplan/shadow_adopt:pattern2_break_subset]`
なので、P33 では増殖しない。

### Step 4: 既存 smoke を “タグ必須” に昇格（filter_noise を避ける）

対象:
- `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_vm.sh`
- `tools/smokes/v2/profiles/integration/joinir/loop_break_body_local_seg_vm.sh`

方針:
- `filter_noise` は `[coreplan/shadow_adopt:*]` を落とすので、raw `OUTPUT` でタグを検証する。
- 既存の promotion hint タグ検証（DigitPos/TrimSeg）と output=2 の期待は維持する。

例（概念）:
- `if ! echo "$OUTPUT" | grep -qF '[coreplan/shadow_adopt:pattern2_break_subset]'; then fail; fi`
- その後 `OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise)` を使って従来の output/hint を検証

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p33): planner-derive loop-break body-local smokes"`
