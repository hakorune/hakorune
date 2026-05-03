---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P283a, module generic unary not consume
Related:
  - docs/development/current/main/phases/phase-29cv/P242A-GENERIC-I64-UNARY-NOT.md
  - docs/development/current/main/phases/phase-29cv/P282A-FALLBACK-AUTHORITY-OP-GUARD-NOT-FREE.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P283a: Module Generic Header-Present Route Closure

## Problem

After P282a, source-execution advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox._program_json_header_present/1
```

P242a already accepted `Bool|I64 --not--> Bool` in the Rust route classifier,
and this function routes as `generic_i64_body / DirectAbi`.

The first remaining gap is that the module generic C prepass/emitter does not
consume the already-owned `unop not` scalar fact.

A fresh P283 probe also showed a second fact publication gap in the same
blocker: `_program_json_header_present/1` uses `program_json_text.contains(...)`
where the receiver is untyped param text carried through copy/PHI, while the
needle is an explicit string constant. Rust generic-i64 classification already
accepts that body, but `generic_method_routes` did not publish a
`StringContains` LoweringPlan entry for this param-text receiver flow, so the C
prepass had no plan entry to consume.

## Decision

Add module generic support for MIR `unop` with operation `not`:

```text
Bool|I64 -> i1
```

This is generic scalar MIR-op support. It does not add a body-specific emitter
or a new function shape.

Also publish an existing `StringContains` route for this narrow method surface:

```text
RuntimeDataBox.contains(string needle)
receiver = function param text carried through copy/PHI
```

This keeps the backend on LoweringPlan consumption. The backend still does not
rediscover raw `contains` calls by method name.

## Non-Goals

- no `.hako` source workaround in `BuilderProgramJsonInputContractBox`
- no new `GlobalCallTargetShape`
- no collection method semantics
- no body-specific C emitter

## Acceptance

- `BuilderProgramJsonInputContractBox._program_json_header_present/1` advances
  past module generic prepass.
- The source-execution probe advances to the next blocker.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

MIR JSON now publishes two `generic_method.contains` routes for
`BuilderProgramJsonInputContractBox._program_json_header_present/1`:

```text
b4959.i3 RuntimeDataBox.contains -> StringContains / DirectAbi
b4961.i4 RuntimeDataBox.contains -> StringContains / DirectAbi
```

The module generic C prepass/emitter also consumes `unop not` as `Bool|I64 ->
i1`.

Fresh source-execution probe advanced past
`BuilderProgramJsonInputContractBox._program_json_header_present/1` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox.has_defs/1
```
