---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AF, nested-if compare side predicate cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AE-IF-COMPARE-VARVAR-NAME-TEXT-SENTINELS.md
  - lang/src/mir/builder/internal/lower_if_nested_box.hako
---

# P381AF: If Nested Compare Side Bool Guard

## Problem

After P381AE, the direct Stage1 env EXE route reaches:

```text
LowerIfNestedBox._read_cmp_side_int/4
%r173 = icmp eq i64 %r159, 1
%r159 defined with type i1 but expected i64
```

The owner uses `use_var` as a numeric `0/1` control flag and later compares it
with `== 1`. The pure-first emitter correctly treats the flag as Bool in a PHI,
but the owner source still asks for an i64 comparison.

## Decision

Represent `use_var` as a Bool predicate and branch directly on it.

## Boundary

Allowed:

- change owner-local branch guards from numeric `0/1` to Bool predicates
- keep accepted nested-if shapes and emitted MIR unchanged

Not allowed:

- add backend i1-to-i64 compare repair
- widen nested-if acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381af_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerIfNestedBox._read_cmp_side_int/4` no longer compares a Bool PHI with
  numeric `1`

## Result

Implemented. `use_var` and `use_int` now use Bool predicates and direct branch
guards. The direct Stage1 env EXE route progressed past
`LowerIfNestedBox._read_cmp_side_int/4` and exposed the next owner-local scalar
seed issue in `LowerLoopMultiCarrierBox._emit_multi_count_json/7`.
