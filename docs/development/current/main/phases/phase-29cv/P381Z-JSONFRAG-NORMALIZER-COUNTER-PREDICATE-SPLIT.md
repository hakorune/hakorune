---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381Z, JsonFrag normalizer counter/predicate split
Related:
  - docs/development/current/main/phases/phase-29cv/P381Y-JSON-FRAG-FLOAT-HAD-BOOL-CONTRACT.md
  - lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
---

# P381Z: JsonFrag Normalizer Counter Predicate Split

## Problem

After P381Y, the direct Stage1 env EXE route reaches:

```text
JsonFragNormalizerBox._normalize_instructions_array/1
%r89 = phi i64 [ %r77, ... ]
%r77 defined with type i1 but expected i64
```

The normalizer initializes numeric loop indices/counters and predicate flags
with the same 0/1 source style. The pure-first route then infers some initial
zero values as Bool while later loop PHIs require i64.

## Decision

Split the carriers inside `_normalize_instructions_array/1`:

- numeric indices/counters use an explicit i64-producing initialization
- predicate-only state (`purify`) uses Bool conditions
- delimiter state (`first`) uses an explicit i64 sentinel because it crosses
  the three append loops and is compared as a 0/1 state
- unused `seen_ret` state is removed

This keeps the normalizer source-owned and avoids backend rescue conversion.

## Boundary

Allowed:

- rewrite local carrier initialization in the normalizer
- remove unused local state
- keep instruction normalization output unchanged

Not allowed:

- add a new backend body shape
- change normalization policy or ordering
- add C shim special handling

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381z_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `_normalize_instructions_array/1` no longer mixes Bool initial values into
  i64 loop-index or delimiter-state PHIs

## Result

Implemented. `JsonFragNormalizerBox._normalize_instructions_array/1` separates
loop counters from predicate state.
