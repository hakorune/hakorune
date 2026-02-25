---
Status: Ready
Scope: code + docs
---

# Phase 29bd P1: Converge strict/dev fallback → `flowbox/freeze` (toward fallback=0)

## Goal

Phase 29bd P0 の棚卸し表に基づき、strict/dev での「候補っぽいのに Ok(None) で落ちる」経路を潰して、
fallback 観測を `flowbox/freeze` の code に一本化する。

release 既定挙動・恒常ログは不変。

## SSOT

- Purity Stage-2: `docs/development/current/main/design/coreplan-purity-stage2-ssot.md`
- FlowBox fallback codes: `docs/development/current/main/design/flowbox-fallback-observability-ssot.md`
- Gate: `docs/development/current/main/phases/phase-29ae/README.md`
- Inventory: `docs/development/current/main/phases/phase-29bd/README.md` の `Inventory (P0)`

## Non-goals

- subset 拡張（facts/extractors/planner の拡張）
- 新しい env var の追加
- release のログ追加

## Implementation (recommended order)

### Step 1: strict/dev の “planner None” を `Freeze(planner_none)` へ寄せる

対象（優先）:

- `src/mir/builder/control_flow/plan/single_planner/rules.rs`
- `src/mir/builder/control_flow/plan/planner/candidates.rs`

方針:

- strict/dev で、gate-target 形状（例: regression pack対象）が planner で `Ok(None)` になった場合は、
  `flowbox/freeze code=planner_none` で可視化できる状態へ寄せる。

実装指針（例）:

- `single_planner` 側で planner outcome を見て、
  - 「pattern_kind が loop 系で、facts が存在し、candidate を期待できる」なら
    - `Err(Freeze::contract(...).to_string())` を返す（strict/dev only）
  - それ以外は従来どおり（Ok(None) or legacy）

注意:

- エラー文字列は release では観測されないため、strict/dev 限定で差分が出ても許容。
  ただし “過剰に freeze しない” ため、判定条件は `phase-29bd/README.md` の表に合わせて最小にする。

### Step 2: strict/dev の “composer reject” を `Freeze(composer_reject)` へ寄せる

対象（優先）:

- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

方針:

- facts/plan が揃っていて “採用を期待する” 場合に `Ok(None)` で静かに落ちない。
- strict/dev では `flowbox/freeze code=composer_reject` が出る（router 側で観測できること）。

実装指針:

- `try_compose_core_loop_for_domain_plan(...)` / `try_compose_core_loop_from_facts(...)` の戻り `Ok(None)` について、
  - “明らかに非対象” のみ許容
  - “対象っぽいのに拒否” は strict/dev で fail-fast へ寄せる（Err or Freeze）

### Step 3: Inventory table の更新（SSOT）

更新:

- `docs/development/current/main/phases/phase-29bd/README.md`

やること:

- P1で触った地点について、`Current behavior` → `Strict/Dev policy` が実装と一致するように更新
- `Freeze code` を `flowbox-fallback-observability-ssot.md` の語彙に合わせる

## Acceptance

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `rg -n \"\\[plan/fallback:\" src/mir` が 0 件（維持）

## Commit

- `git add -A`
- `git commit -m \"phase29bd(p1): converge strict/dev fallback to flowbox freeze\"`

