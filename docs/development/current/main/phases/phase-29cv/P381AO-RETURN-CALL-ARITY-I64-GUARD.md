---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AO, return-call arity scalar and scan guard cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381AN-PATTERN-UTIL-INT-BINARY-TEXT-CARRIERS.md
  - lang/src/mir/builder/func_lowering/return_call_lower_box.hako
---

# P381AO: Return Call Arity I64 Guard

## Problem

After P381AN, the direct Stage1 env EXE route reaches:

```text
ReturnCallLowerBox.lower/6
%r382 = phi i64 [ %r371, ... ], [ %r108, ... ]
%r371 defined with type i1 but expected i64
```

The owner uses numeric `0/1` locals for two different roles:

- `call_arity` is a scalar used in MIR function-name text.
- `args_active` is a loop predicate.

The no-args path lets `call_arity = 0` be inferred as Bool and later merged with
scalar arity payloads.

## Decision

Make `call_arity` an explicit i64 scalar and make `args_active` a Bool
predicate.

## Boundary

Allowed:

- change `call_arity` seed to explicit i64 zero
- change `args_active` from numeric `0/1` to Bool
- keep emitted MIR JSON unchanged

Not allowed:

- add backend i1-to-i64 PHI repair
- widen Return(Call) acceptance
- change function target resolution

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ao_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `ReturnCallLowerBox.lower/6` no longer merges Bool scan state with scalar
  arity state

## Result

Implemented in `ReturnCallLowerBox.lower/6`:

- `call_arity` now starts from an explicit i64 zero via `StringHelpers.to_i64("0")`.
- `args_active` is now a Bool predicate.

The direct Stage1 env EXE route advanced past `ReturnCallLowerBox.lower/6` and
now stops in `StringHelpers.to_i64/1`.
