---
Status: Ready
Scope: docs-only
---

# Phase 29bc P0: Composer API SSOT (docs-first)

## Objective

`plan/composer` の “入口” を SSOT として固定し、不要な dead_code/scaffold を削除できる状態にする。

## SSOT (Public entrypoints)

### CoreLoop (single entry)

- `src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs`
  - `try_compose_core_loop_from_facts(...)` が **唯一の CoreLoop 合成入口**

### Shadow/release adopt (entrypoints)

以下は “入口” として残してよい（router から呼ばれるため）:

- `try_shadow_adopt_core_plan(...)`
- `try_release_adopt_core_plan(...)`
- `try_shadow_adopt_is_integer(...)` / `try_release_adopt_is_integer(...)`
- `try_shadow_adopt_nested_minimal(...)` / `try_release_adopt_nested_minimal(...)`
- `strict_nested_loop_guard(...)`

### BranchN (match return)

- `compose_match_return_branchn(...)`（match_return subset の入口）

## Non-SSOT (remove candidates)

以下は “足場” として残っているだけで、入口 SSOT ではない（P1 で削除候補）:

- `try_compose_domain_plan_from_canonical_facts(...)`
- `try_compose_core_plan_via_normalizer(...)`
- `try_compose_core_plan_direct(...)`
- `try_compose_core_plan_from_canonical_facts(...)`

## Acceptance

- `docs/development/current/main/phases/phase-29bc/README.md` から参照導線が辿れる
- Gate は P1 のあとに回す（P0 は docs-only）

