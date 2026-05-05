---
Status: Accepted
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv P381CA, first ny-llvmc uniform MIR function selected-set slice for `missing_multi_function_emitter`
Related:
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381CA: Uniform MIR Function Candidate Emit

## Problem

`missing_multi_function_emitter` is no longer a target-shape invitation. MIR has
already published a same-module `global.user_call` with `target_exists=true`,
`arity_matches=true`, and a safe `target_symbol`. The Stage0 gap is that
ny-llvmc must select that function, emit its definition in the same module, and
only then lower the call.

## Decision

Add the first backend-local uniform MIR candidate predicate:

```text
global.user_call
  tier=Unsupported
  emit_kind=unsupported
  proof=typed_global_call_contract_missing
  reason=missing_multi_function_emitter
  target_exists=true
  arity_matches=true
```

ny-llvmc may use this only to seed and validate same-module function emission.
It must not infer source-owner semantics from the callee name, and it must not
publish string/array/map origins unless MIR has a dedicated direct contract.

## Rules

Allowed:

- seed the entry-reachable selected set from uniform MIR candidates
- emit declarations before bodies and require the target definition before a
  call uses the symbol
- fail-fast inside the selected callee if its MIR body needs unsupported
  operation or ownership facts

Forbidden:

- adding a new `GlobalCallTargetShape` for these helpers
- lowering a same-module user call as an unresolved external declaration
- treating `target_shape_blocker_*` as permission for source-owner-specific C
  logic

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in /tmp/hakorune_stage1_cli_env_parse_probe.mir.json \
  --emit obj --out /tmp/hakorune_stage1_cli_env_parse_probe.o
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

Expected: the trace may still fail on the next unsupported selected body, but a
same-module `missing_multi_function_emitter` site is no longer rejected before
the target function has been selected for uniform MIR emission.

## Result

Implemented.

The selected-set planner now records entry-reachable same-module
`missing_multi_function_emitter` targets as uniform MIR candidates. The call
emitter accepts those sites only when the selected target will be defined in the
same LLVM module. The candidate call publishes only `i64` type information; it
does not infer stronger origins from the callee name or blocker metadata.

Observed probe movement:

```text
before: module_generic_prepass_failed target_shape_blocker_symbol=BuildBox._emit_program_json_from_scan_src/1
after:  module_generic_prepass_failed target_shape_blocker_symbol=BuildBox._parse_program_json/2
```

The remaining stop is now inside the selected parser handoff body, so the next
slice is source-owner cleanup or a narrower MIR-owned parser-return contract,
not another target-shape capsule.
