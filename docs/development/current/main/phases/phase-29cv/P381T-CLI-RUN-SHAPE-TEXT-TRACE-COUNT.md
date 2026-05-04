---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381T, CliRunShapeScanner trace count carrier cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381S-FALLBACK-INLINE-TAIL-RETIRE.md
  - lang/src/mir/builder/func_body/cli_run_shape_box.hako
---

# P381T: CLI Run Shape Text Trace Count

## Problem

After P381S, the direct Stage1 env EXE route reaches:

```text
CliRunShapeScannerBox.scan/1
%r137 = add i64 %r130, 1
%r130 defined with type i1 but expected i64
```

`CliRunShapeScannerBox` is observability-only and always returns `null`.
Its branch counter only exists for the optional
`[builder/cli:run_shape]` trace. Keeping that trace counter as a numeric
carrier forces the pure-first route to type a local across Bool branches and
i64 arithmetic.

## Decision

Keep the scanner as an observability-only source owner and make the trace
branch count a text carrier. The scanner should not require backend Bool-to-i64
rescue and should not add a new lowering shape.

## Boundary

Allowed:

- change the trace counter carrier from numeric to text
- keep the same coarse branch-detection probes
- keep scanner return behavior unchanged (`null`)

Not allowed:

- add generic backend acceptance for Bool/i64 arithmetic repair
- add a new `GlobalCallTargetShape`
- change Stage1 CLI lowering policy

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381t_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- direct Stage1 env EXE route no longer fails on the
  `CliRunShapeScannerBox.scan/1` trace counter
- phase29cg remains green after the route repair series

## Result

Implemented. `CliRunShapeScannerBox.scan/1` now keeps trace counters as text
transition state instead of numeric predicate-like locals.
