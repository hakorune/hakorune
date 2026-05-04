---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381Y, JsonFrag float read had predicate cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381X-FUNC-LOWERING-CLI-GUARD-BOOL-CONTRACT.md
  - lang/src/shared/json/utils/json_frag.hako
---

# P381Y: JsonFrag Float Had Bool Contract

## Problem

After P381X, the direct Stage1 env EXE route reaches:

```text
JsonFragBox.read_float_from/2
%r272 = icmp eq i64 %r199, 0
%r199 defined with type i1 but expected i64
```

The `had` local records whether the float scanner consumed at least one
numeric character. It is a predicate, not an arithmetic counter.

## Decision

Represent `had` as a Bool carrier and test it as a condition.

## Boundary

Allowed:

- change `had` from 0/1 numeric style to Bool style
- keep the read result and null-on-miss behavior unchanged

Not allowed:

- add backend Bool/i64 implicit repair
- change JSON float parsing policy
- add a new route shape

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381y_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `JsonFragBox.read_float_from/2` no longer emits a Bool value into an i64
  equality comparison

## Result

Implemented. `JsonFragBox.read_float_from/2` keeps `had` as a Bool predicate
instead of a numeric sentinel.
