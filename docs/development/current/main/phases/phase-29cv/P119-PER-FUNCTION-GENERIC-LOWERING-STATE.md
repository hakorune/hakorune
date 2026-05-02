---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P119, ny-llvmc generic pure per-function lowering state seam
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P114-GENERIC-PURE-MODULE-VIEW-ENTRY-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P118-GLOBAL-CALL-LEAF-MODULE-FUNCTION-EMITTER.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P119: Per-Function Generic Lowering State

## Problem

P118 deliberately keeps non-leaf same-module calls, including
`Stage1ModeContractBox.resolve_mode/0`, on the
`missing_multi_function_emitter` stop-line. Emitting only the call would
externalize a same-module function because the generic pure backend still emits
the selected entry function as one monolithic lowering scope.

The next cleanup is structural: separate the generic pure function view and
per-function lowering state before adding any wider function-body emission.

## Decision

Add an explicit function owner seam inside ny-llvmc generic pure:

```text
module view
  -> selected entry function view
  -> per-function generic lowering state reset
  -> entry body emission
```

The C shim now has:

- `hako_llvmc_generic_pure_function_view` for one MIR function's metadata,
  blocks, block count, and entry selection
- `program.entry` as the selected entry function view
- `GenericPureFunctionLoweringState` for generic pure register facts, string
  facts, PHI records, owned string constants, and trace budgets
- `reset_generic_pure_function_lowering_state(...)` before lowering the active
  function body

This is behavior-preserving. It does not make `resolve_mode/0` lowerable and it
does not add a generic multi-function body emitter yet.

## Rules

Allowed:

- bind the existing entry lowering to an explicit active function view
- reset all generic pure lowering facts through one per-function state object
- keep P118 numeric leaf emission as a separate sealed capsule

Forbidden:

- treating `missing_multi_function_emitter` as permission to lower a call
- emitting a call to a same-module function that only has a declaration
- adding raw `Stage1ModeContractBox.*` matchers
- changing VM/source-execution or vm-hako behavior in this card

## Next Boundary

The next card can reuse the same state reset seam to emit non-entry
same-module function bodies. That card must still prove function definitions
before enabling `UserGlobalCall` callsites.

Expected shape:

```text
module-wide declaration/need scan
  -> per-target function view
  -> reset GenericPureFunctionLoweringState
  -> emit quoted LLVM function definition
  -> plan-first UserGlobalCall direct call
```

## Acceptance

- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- P118 numeric leaf calls remain lowerable.
- Full `lang/src/runner/stage1_cli_env.hako` still stops at
  `missing_multi_function_emitter` until the real multi-function body emitter
  lands.

## Follow-up

P120 used this seam to emit same-module generic pure string definitions before
allowing direct calls. Later P147-P150 cleanup moved the active source-execution
stop past `Stage1ModeContractBox.resolve_mode/0` and BuildBox parse-source
narrowing; the current stop is the parser authority boundary recorded in P151.
