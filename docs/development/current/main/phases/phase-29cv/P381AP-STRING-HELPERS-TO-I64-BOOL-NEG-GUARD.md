---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AP, StringHelpers.to_i64 sign predicate cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AO-RETURN-CALL-ARITY-I64-GUARD.md
  - lang/src/shared/common/string_helpers.hako
---

# P381AP: StringHelpers to_i64 Bool Neg Guard

## Problem

After P381AO, the direct Stage1 env EXE route reaches:

```text
StringHelpers.to_i64/1
%r79 = icmp eq i64 %r70, 1
%r70 defined with type i1 but expected i64
```

The owner uses `neg` as a numeric `0/1` local, but the value is a predicate:

```hako
local neg = 0
if first == "-" { neg = 1 } else { neg = 0 }
if neg == 1 { ... }
```

The backend correctly sees the sign branch as Bool-like, while later source
comparisons force an i64 comparison.

## Decision

Make `neg` an explicit Bool predicate and branch on it directly. Seed `i` and
`acc` from local i64-derived values so sign handling cannot inject Bool values
into scalar counters.

## Boundary

Allowed:

- change `neg` from numeric `0/1` to Bool
- replace `neg == 1` checks with direct Bool checks
- seed `i`/`acc` from local i64-derived zero/one values
- keep `to_i64` parsing semantics unchanged

Not allowed:

- add backend i1-to-i64 repair
- widen generic classifier rules
- change numeric parsing behavior

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ap_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `StringHelpers.to_i64/1` no longer emits an i1 value into an i64 comparison

## Result

Implemented in `StringHelpers.to_i64/1`:

- `neg` is now a Bool predicate.
- `i` and `acc` are seeded from local i64-derived zero/one values.

The direct Stage1 env EXE route now compiles successfully and produces an
executable.
