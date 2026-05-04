---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AG, loop multi-carrier register-number scalar contract
Related:
  - docs/development/current/main/phases/phase-29cv/P381AF-IF-NESTED-CMP-SIDE-BOOL-GUARD.md
  - lang/src/mir/builder/internal/lower_loop_multi_carrier_box.hako
---

# P381AG: Loop Multi-Carrier Register I64 Contract

## Problem

After P381AF, the direct Stage1 env EXE route reaches:

```text
LowerLoopMultiCarrierBox._emit_multi_count_json/7
%r392 = call i64 @"nyash.string.concat3_hhh"(..., i64 %r211)
%r211 defined with type i1 but expected i64
```

The owner emits MIR JSON text and carries register-number locals across the
`limit_kind` branch. Literal register id `1` is inferred as Bool in the PHI and
later used as a scalar/text-concat operand.

## Decision

Make register-number seed locals explicit i64 values before the text emitter
branches. This keeps the fix in the owner-local MIR JSON emitter and avoids a
backend i1/i64 concat repair.

## Boundary

Allowed:

- make register-number seeds explicit i64 scalars
- keep emitted MIR JSON and accepted multi-carrier shape unchanged

Not allowed:

- add backend Bool-to-i64 concat repair
- widen multi-carrier loop acceptance
- add route shapes

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381ag_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `LowerLoopMultiCarrierBox._emit_multi_count_json/7` no longer passes a Bool
  PHI to string concat for register-number text emission

## Result

Implemented. Register-number seeds now enter the text emitter as explicit i64
scalars. The direct Stage1 env EXE route progressed past
`LowerLoopMultiCarrierBox._emit_multi_count_json/7` and exposed the next
owner-local text sentinel issue in `LowerLoopSumBcBox.try_lower/1`.
