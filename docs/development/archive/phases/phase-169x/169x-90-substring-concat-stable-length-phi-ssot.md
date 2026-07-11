# 169x-90: substring concat stable-length phi contract SSOT

Status: SSOT
Date: 2026-04-12
Scope: add a narrow string-relation proof for the live `kilo_micro_substring_concat` loop-carried route so the merged header `phi` can reuse the stable source-length scalar without widening the proof-bearing plan window.

## Goal

- keep the current `phi_merge` stop-line for plan windows
- add only the scalar witness that the exact front still lacks:
  - merged header source `%21` has a loop-stable source-length scalar
- use that witness to collapse:
  - `substring_len_hii(left) + const_len + substring_len_hii(right)`
  - into
  - `source_len + const_len`
- refresh direct/post-sink and pure-first contracts to the resulting live body shape

## Diagnosis

Current direct/post-sink MIR for `bench_kilo_micro_substring_concat.hako` already carries:

- shared-source concat-triplet proof on helper `%36`
- `PhiCarryBase` relations:
  - `%22 = preserve_plan_window`
  - `%21 = stop_at_merge`

But the loop body still keeps:

- `%88 = substring_len_hii(%21, 0, split)`
- `%89 = substring_len_hii(%21, split, len)`
- `%31 = %88 + 2 + %89`

The missing proof is narrower than a full merged-plan-window carry:

- the merged header source `%21` keeps the same source-length scalar `%5`
- the backedge helper plan already proves a fixed-width outer window (`start=1`, `end=len+1`)

That proof should be explicit metadata, not another exact-shape rediscovery in sink/backend code.

## Authority Order

1. canonical MIR + string facts
2. string relation metadata
3. string sink rewrite
4. direct/post-sink smoke and pure-first exact seed

This phase only adds step 2 for the current exact route.

## Fix

### 1. Add a narrow string relation

- relation kind: `stable_length_scalar`
- owner: `src/mir/string_corridor_relation.rs`
- carrier: `FunctionMetadata.string_corridor_relations`
- witness payload:
  - relation target = merged header string value
  - base value = current carried helper/root
  - length value = loop-stable scalar length

The relation is allowed only when:

- current string continuity is already known through `PhiCarryBase`
- the merged header route is still `stop_at_merge`
- the carried helper plan proves a fixed-width window whose width equals the scalar length
- the scalar length originates from the entry path

### 2. Consume that relation in `string_corridor_sink`

- keep the existing complementary substring-length fusion logic
- widen it narrowly so the same proof can accept the current loop-carried route when the right-end scalar is the relation-carried stable length
- do not widen beyond the current complementary pair plus middle-const shape

### 3. Refresh exact contracts

- direct/post-sink shape smoke must pin the new body:
  - no loop `substring_len_hii`
  - still one `substring_concat3_hhhii`
  - merged `%21` still `stop_at_merge`
  - `%21` now also exposes `stable_length_scalar`
- pure-first seed must accept the new live shape
- daily owner fixture must follow the refreshed post-sink body

## Acceptance

- direct MIR on `bench_kilo_micro_substring_concat.hako` exposes `stable_length_scalar` on the merged header route
- loop body no longer keeps `substring_len_hii` on that exact front
- direct/post-sink smoke is green on the refreshed body
- pure-first exact build/asm stays green
- exact perf on `kilo_micro_substring_concat` moves in the right direction

## Non-Goals

- no generic merged-plan-window carry
- no new borrowed-corridor fusion beyond the exact front
- no DCE/escape work mixed into this phase
