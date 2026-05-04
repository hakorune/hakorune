---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AM, PatternUtil bool compare carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AL-MIR-JSON-PHI-INCOMING-I64-SEEDS.md
  - lang/src/mir/builder/internal/pattern_util_box.hako
---

# P381AM: PatternUtil Bool Compare Text Carriers

## Problem

After P381AL, the direct Stage1 env EXE route reaches:

```text
PatternUtilBox.find_local_bool_before/3
%r818 = phi i64 [ %r799, ... ], [ %r783, ... ]
%r799 defined with type i1 but expected i64
```

The shared bool-local resolver uses `null` as an internal miss sentinel for
Compare-side values and later merges it with integer payloads from local-int
resolution.

## Decision

Use text sentinels for owner-internal Compare-side value carriers and convert to
i64 only after both sides are present.

## Boundary

Allowed:

- change `lhs_v`/`rhs_v` internal sentinels from `null` to `""` in
  `find_local_bool_before`
- keep bool-local resolution behavior unchanged

Not allowed:

- add backend null/i64 PHI repair
- widen pattern acceptance
- change callers

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381am_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `PatternUtilBox.find_local_bool_before/3` no longer merges null sentinel with
  Compare-side numeric payloads

## Result

Implemented. Compare-side carriers now use text sentinels in
`find_local_bool_before`. The direct Stage1 env EXE route progressed past that
function and exposed the same carrier issue in `find_local_int_before`.
