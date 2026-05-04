---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AN, PatternUtil int binary carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AM-PATTERN-UTIL-BOOL-COMPARE-TEXT-CARRIERS.md
  - lang/src/mir/builder/internal/pattern_util_box.hako
---

# P381AN: PatternUtil Int Binary Text Carriers

## Problem

After P381AM, the direct Stage1 env EXE route reaches:

```text
PatternUtilBox.find_local_int_before/3
%r1335 = phi i64 [ %r1321, ... ], [ %r1301, ... ]
%r1321 defined with type i1 but expected i64
```

The shared int-local resolver uses `null` as an internal miss sentinel for
Binary-side values and later merges it with integer payloads.

## Decision

Use text sentinels for owner-internal Binary-side value carriers and convert to
i64 only after both sides are present.

## Boundary

Allowed:

- change `lhs_v`/`rhs_v` internal sentinels from `null` to `""` in
  `find_local_int_before`
- keep local-int resolution behavior unchanged

Not allowed:

- add backend null/i64 PHI repair
- widen pattern acceptance
- change callers

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381an_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `PatternUtilBox.find_local_int_before/3` no longer merges null sentinel with
  Binary-side numeric payloads

## Result

Implemented. Binary-side carriers now use text sentinels in
`find_local_int_before`. The direct Stage1 env EXE route progressed past that
function and exposed the next scalar/guard role split issue in
`ReturnCallLowerBox.lower/6`.
