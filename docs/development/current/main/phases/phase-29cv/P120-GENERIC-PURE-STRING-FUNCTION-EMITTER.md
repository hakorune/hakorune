---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P120, ny-llvmc same-module generic pure string function emitter
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P119-PER-FUNCTION-GENERIC-LOWERING-STATE.md
  - src/mir/global_call_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P120: Generic Pure String Function Emitter

## Problem

P119 gave ny-llvmc a per-function generic pure lowering state seam, but
same-module string helper calls such as `Stage1ModeContractBox.resolve_mode/0`
still stopped at `missing_multi_function_emitter`.

Lowering only the call is forbidden: it would externalize a MIR function that is
present in the same module. The backend must emit a definition first, then allow
the direct call.

## Decision

Add the second lowerable `global_call_routes` target shape:

```text
generic_pure_string_body
  -> DirectAbi
  -> direct_function_call
  -> typed_global_call_generic_pure_string
```

MIR owns the classification. ny-llvmc only validates the typed
`LoweringPlanGlobalCallView`, records the entry-reachable transitive closure of
generic string targets, emits definitions for that closure, and then allows
direct calls.

The accepted body subset is deliberately narrow:

- string/i64 consts
- copy
- phi
- string equality/inequality compare
- branch/jump/return
- string `+` concat and i64 arithmetic
- `env.get/1`
- same-module global calls whose plan is either `generic_pure_string_body` or
  existing `numeric_i64_leaf`
- `keepalive` / `release_strong` as no-op lifetime markers

## Rules

Allowed:

- classify string-handle ABI signatures (`String`, `StringBox`, `Integer`, or
  `Unknown`) when the body analysis proves all returns are string handles
- emit non-entry generic string definitions with a fresh per-function lowering
  state
- seed ny-llvmc module function emission from the selected entry function and
  follow only reachable direct generic string calls

Forbidden:

- scanning the whole module and defining every string-looking helper
- direct-calling a same-module generic string symbol unless it is in the
  definition plan
- raw by-name lowering for `Stage1ModeContractBox.*`
- changing VM/source-execution or vm-hako behavior

## Evidence

Focused canary:

```bash
target/release/hakorune --emit-mir-json /tmp/p120_stage1_cli_env.mir.json \
  lang/src/runner/stage1_cli_env.hako

# /tmp/p120_resolve_mode_canary.mir.json keeps main + resolve_mode transitives.
target/release/ny-llvmc --in /tmp/p120_resolve_mode_canary.mir.json \
  --emit obj --out /tmp/p120_resolve_mode_canary.o
```

Observed:

```text
object written: /tmp/p120_resolve_mode_canary.o
```

Full `lang/src/runner/stage1_cli_env.hako` now passes through
`Stage1ModeContractBox.resolve_mode/0` and its transitive string helpers. The
next stop is a later authority dispatch call:

```text
first_block=14948 first_inst=0 first_op=mir_call
reason=unknown_global_callee
mname=main._run_emit_program_mode/0
```

## Acceptance

- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- focused `resolve_mode/0` canary emits an object.
- full `stage1_cli_env.hako` no longer stops at
  `Stage1ModeContractBox.resolve_mode/0`; it stops later at the next
  unsupported authority call.
