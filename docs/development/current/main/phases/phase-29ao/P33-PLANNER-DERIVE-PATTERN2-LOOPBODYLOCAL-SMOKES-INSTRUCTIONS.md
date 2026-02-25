---
Status: Ready
Scope: code+tests+docs（仕様不変・strict/devのみ観測強化）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_min_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh
  - src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs
  - src/mir/builder/control_flow/plan/facts/pattern2_loopbodylocal_facts.rs
  - src/mir/builder/control_flow/plan/planner/build.rs
  - src/mir/builder/control_flow/plan/composer/shadow_adopt.rs
---

# Phase 29ao P33: Pattern2 LoopBodyLocal（phase29ab）を planner 由来に引き上げ、shadow adopt タグを回帰で固定する

Date: 2025-12-30  
Status: Ready for execution  
Goal: `phase29ab_pattern2_loopbodylocal_{min,seg_min}` が strict/dev で **planner 由来の `DomainPlan::Pattern2Break`** になり、既存の shadow adopt タグ
`[coreplan/shadow_adopt:pattern2_break_subset]` が必ず出るようにして回帰で固定する。

## 背景

- `phase29ab_pattern2_loopbodylocal_*` は現状、promotion hint タグ（`[plan/pattern2/promotion_hint:*]`）は出るが、
  `outcome.plan` が `Pattern2Break` ではないため shadow adopt が走らず、CorePlan 経路の差分検知ができない。
- Phase 29ao の “段階1（strict/dev）完了” は、「回帰パックに含まれる代表ケースが shadow adopt タグで検知できる」状態。
  Pattern2 の realworld（phase263）は P32 で埋めたので、次は phase29ab の LoopBodyLocal を埋めるのが自然。

## 非目的

- release 既定挙動の変更
- NotApplicable/Freeze ケースの “無理な planner 化”
- 新しい env var 追加
- タグ名の増殖（既存タグを流用する）

## 受け入れ基準（Acceptance）

- `cargo build --release` が通る
- `./tools/smokes/v2/run.sh --profile quick` が通る（既定挙動不変）
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が通る
- 次の 2 つの既存 integration smoke が strict/dev の raw output で shadow adopt タグを必須で満たす:
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_min_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`

## 実装手順（安全順）

### Step 1: Facts 抽出を “phase29ab の LoopBodyLocal 形状” に対応させる

対象:
- `src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs`

方針:
- 誤マッチ防止優先で、`Ok(None)` を維持しつつ “この2 fixture” を拾える最小条件を追加する。
- `pattern2_loopbodylocal_facts` は既に promotion hint を出しているので、
  **break/loop_increment 側の facts 取りこぼし**が主因になっている可能性が高い。

最低限の SSOT:
- `phase29ab_pattern2_loopbodylocal_min.hako`
- `phase29ab_pattern2_loopbodylocal_seg_min.hako`

### Step 2: planner で Pattern2Break を候補に出す（planner 由来にする）

対象:
- `src/mir/builder/control_flow/plan/planner/build.rs`

方針:
- `facts.facts.pattern2_break.is_some()` のとき、`DomainPlan::Pattern2Break(..)` の candidate を push する。
- `facts.facts.pattern2_loopbodylocal` があれば `promotion` を乗せる（既存の composer と同じ）。

注意:
- `pattern2_break` が取れないケース（freeze/notapplicable）は candidate を出さない（Ok(None) を維持）。

### Step 3: shadow adopt の gate は既存のまま（タグは流用）

確認対象:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

このファイルは既に
- `DomainPlan::Pattern2Break` かつ `outcome.plan` が `Pattern2Break` のときだけ adopt
- タグは `[coreplan/shadow_adopt:pattern2_break_subset]`
なので、P33 では増殖しない。

### Step 4: 既存 smoke を “タグ必須” に昇格（filter_noise を避ける）

対象:
- `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_min_vm.sh`
- `tools/smokes/v2/profiles/integration/apps/phase29ab_pattern2_loopbodylocal_seg_min_vm.sh`

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
- `git commit -m "phase29ao(p33): planner-derive pattern2 loopbodylocal smokes"`

