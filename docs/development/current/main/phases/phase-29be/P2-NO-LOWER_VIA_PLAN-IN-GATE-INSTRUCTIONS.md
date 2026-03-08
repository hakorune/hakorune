---
Status: Ready
Scope: code + docs
---

# Phase 29be P2: Ensure gate never relies on `lower_via_plan(domain_plan)` (domain-plan-free gate)

## Goal

Phase 29be P0 inventory の “router final fallback” を解消する。

- current `src/mir/builder/control_flow/joinir/route_entry/router.rs`
  （historical path token: `joinir/patterns/router.rs`）に残っている
  `lower_via_plan(builder, domain_plan, ctx)` は **最終fallbackとして残してよい**が、
  **gate 対象（phase-29ae regression pack）がその経路を踏まない**ことを SSOT と smoke で固定する。

## Non-goals

- release 既定挙動の変更（fallback 経路の削除はしない）
- 新しい env var の追加
- 新しい永続ログの追加（strict/dev の FlowBox タグは既存語彙のみ）

## SSOT

- Gate: `docs/development/current/main/phases/phase-29ae/README.md`
- FlowBox fallback observability: `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`
- Phase inventory: `docs/development/current/main/phases/phase-29be/README.md`

## Implementation

### Step 1: strict/dev で “gate-candidate なのに lower_via_plan へ落ちる” を禁止

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

方針:
- `domain_plan.is_some()` なのに
  - `composer::try_shadow_adopt_core_plan(...)` が `None` で
  - `lower_via_plan(builder, domain_plan, ctx)` に進む
  というケースを strict/dev で fail-fast に寄せる（FlowBox freeze を出す）。

実装案（最小）:
- `if strict_or_dev && should_expect_shadow_adopt(&domain_plan, &outcome, ctx)` ブロックが既にある場合、
  その条件が真のときは `lower_via_plan` に到達しないことを構造で保証する（return Err）。
- さらに保険として、`return lower_via_plan(...)` 直前で
  - `if strict_or_dev && should_expect_plan(&outcome, ctx)` のような guard を置き、
    `flowbox/freeze code=composer_reject` を emit して Err にする。

注意:
- freeze code は SSOT の語彙に揃える（`planner_none` / `composer_reject` / `unstructured` / `unwind`）。
  このケースは `composer_reject` に統一するのが安全。

### Step 2: “gate では踏まない” を smoke で固定

方針:
- release は FlowBox タグが出ないため、「経路を踏んだ/踏んでない」を直接観測できない。
  その代わり strict/dev gate で **composer adopt が成立すること**を固定し、
  `lower_via_plan` に落ちたら strict/dev で freeze になることを smoke で固定する。

やること:
- 既存の strict/dev の adopt smokes（pattern2/6/7/phase1883/phase263 など）が
  `phase29ae_regression_pack_vm.sh` に入っていることを再確認し、必要なら追加する。
- 追加が必要なら “gate-candidate” の最小 fixture + strict smoke を 1 本だけ足す。

### Step 3: Inventory/Now 更新（SSOT）

- `docs/development/current/main/phases/phase-29be/README.md`
  - router fallback 行の `Gate impact` を「gate 外のみ許容（strict/dev では fail-fast）」に更新
  - P2 ✅ を付ける
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` /
  `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` を P3 closeout へ

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A`
- `git commit -m "phase29be(p2): gate must not rely on lower_via_plan"`
