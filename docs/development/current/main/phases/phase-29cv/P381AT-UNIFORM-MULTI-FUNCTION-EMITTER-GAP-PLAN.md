---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv plan for closing `missing_multi_function_emitter` through a uniform MIR function emitter instead of new body-specific `.inc` growth
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P381AR-INC-BOUNDARY-TRUTH-AND-RUNTIME-DECL-ATTR-AUDIT.md
  - docs/development/current/main/phases/phase-29cv/P381AS-LOWERING-PLAN-TIER-VOCAB-REIFY.md
  - src/mir/global_call_route_plan/model.rs
  - lang/c-abi/shims/README.md
---

# P381AT: Uniform Multi-Function Emitter Gap Plan

## Problem

`missing_multi_function_emitter` means the MIR side already knows:

- the callee symbol
- the arity match result
- enough target facts to say "this is a same-module MIR call"

but Stage0 still cannot emit the callee body and keep the call inside the same
LLVM module.

The wrong fix would be to keep widening `GlobalCallTargetShape` and `.inc`
body-specific emitters until every current helper family gets a custom bridge.
That breaks the Stage0 size guard and teaches the backend selfhost-helper
semantics it does not own.

## Decision

Treat `missing_multi_function_emitter` as a **uniform MIR function emission**
gap, not as permission for new body classifiers.

The next implementation path should be:

1. **Seed from accepted entry roots**
   - when `global.user_call` has a supported same-module target, seed emission
     from the current entry function and follow only the transitive closure of
     direct MIR function calls needed by that entry
2. **Emit functions by uniform MIR ABI**
   - parameters: existing i64 / handle ABI only
   - return: existing i64 / handle ABI only
   - call sites lower by MIR symbol, not source-owner name meaning
3. **Keep legality MIR-owned**
   - `.hako` / MIR continue to own route proof, target facts, return shape,
     value demand, and blocker diagnostics
   - Stage0 only consumes those facts and emits function declarations /
     definitions
4. **Retire temporary capsules deliberately**
   - every `GlobalCallTargetShape::*Body` temporary capsule must either
     disappear through source cleanup or become unnecessary once the uniform
     emitter can lower the function directly

## First implementation slices

Smallest safe order:

1. add a backend-local "selected function set" pass driven by the already chosen
   entry root and same-module direct-call edges
2. teach ny-llvmc to emit declarations first, then bodies for that selected set
3. lower same-module calls using existing uniform ABI (`direct_function_call`)
   without reading helper-family semantics
4. only after that, start retiring temporary `GlobalCallTargetShape` capsules
   that existed only because the callee body could not be emitted

## Boundary

Allowed:

- MIR-symbol-based same-module call emission
- transitive closure over direct MIR function calls from the active entry root
- fail-fast if a selected callee needs unsupported MIR op support

Not allowed:

- new body-specific `.inc` emitters as the default answer
- new source-owner-name switches in Stage0
- widening `GlobalCallTargetShape` without a temporary-capsule row and removal
  path
- whole-module scanning that emits unrelated helpers outside the active entry
  closure

## Acceptance

This card is planning-only. Acceptance is documentation and pointer truth:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- fixed the next structural answer for `missing_multi_function_emitter`
- kept the owner boundary aligned with Stage0 size guard / line-shape SSOT
- established that future implementation should emit selected MIR functions by
  uniform ABI rather than add more body-specific backend knowledge
