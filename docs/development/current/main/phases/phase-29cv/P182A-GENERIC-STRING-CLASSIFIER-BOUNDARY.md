---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P182a, generic string classifier boundary before JsonFrag normalizer support
Related:
  - docs/development/current/main/phases/phase-29cv/P181-GENERIC-STRING-RECURSIVE-ACCUMULATOR-FLOW.md
  - src/mir/global_call_route_plan/generic_string_body.rs
  - src/mir/global_call_route_plan/generic_i64_body.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P182a: Generic String Classifier Boundary

## Problem

After P181, the source-execution probe advanced to:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

`JsonFragNormalizerBox._normalize_instructions_array/1` is not a string
accumulator body. It normalizes an instruction array with collection state:
`ArrayBox` lists, `MapBox` dedupe state, const canonicalization, phi/const/other
grouping, and return repair.

Adding this body understanding to `generic_pure_string_body` would make the
generic string classifier own collection-normalizer semantics. That would turn
the route classifier into a second MIR body compiler instead of a small
DirectAbi eligibility classifier.

## Decision

P182a is BoxShape-only. Do not make
`JsonFragNormalizerBox._normalize_instructions_array/1` lowerable by widening
`generic_pure_string_body`.

Forbidden in this card:

- no `JsonFragNormalizerBox` or `_normalize_instructions_array` by-name
  exception in `generic_string_body`
- no normalizer-specific void-sentinel rule inside the generic string
  classifier
- no backend/C-side shape rediscovery
- no new `target_shape` or `return_shape`
- no behavior change

The next accepted normalizer support must be a later BoxCount card with its own
classifier/shape contract, fixture, and LoweringPlan SSOT row if a durable
target shape is introduced.

## Boundary

`generic_string_body` remains limited to string-flow shapes:

- string return and `StringBox` handle return
- string-or-void sentinel return
- string concat, compare, substring, length, and indexOf flow
- self-recursive string accumulator flow
- string PHI and narrow void-sentinel boundary checks needed for string return

Collection normalizer semantics are out of scope:

- `ArrayBox` / `MapBox` normalizer state
- JsonFrag object scanning policy
- const dedupe and canonicalization policy
- return repair and instruction array reorder policy
- purify policy

## Split Direction

Keep route ownership layered:

- MIR builder owns MIR and primary type/meaning facts.
- `global_call_route_plan` reads MIR facts and classifies DirectAbi eligibility.
- Body-shape classifiers stay small and shape-specific.
- C shims consume LoweringPlan and emit; they do not rediscover body meaning.

The immediate cleanup path is:

- extract behavior-preserving helper modules from the large classifier/emitter
  files before adding normalizer acceptance
- keep `generic_string_body` free of JsonFrag normalizer-specific vocabulary
- introduce a dedicated normalizer classifier only when the next BoxCount card
  adds a fixture-backed accepted shape

## Acceptance

P182a is behavior-preserving. The current blocker tuple must remain the same:

```text
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
```

Suggested gate:

```bash
cargo test -q refresh_module_global_call_routes_accepts_self_recursive_generic_pure_string_body --lib
cargo test -q refresh_module_global_call_routes_marks_void_sentinel_const_reason --lib
cargo test -q runtime_methods --lib
cargo test -q void_sentinel --lib
cargo test -q global_call_routes --lib
cargo fmt --check
cargo build -q --release --bin hakorune
if rg -n 'JsonFragNormalizerBox|_normalize_instructions_array' src/mir/global_call_route_plan/generic_string_body.rs; then exit 1; fi
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p182a_classifier_boundary_stage1_cli_env.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
