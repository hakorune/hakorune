---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P284a, string corridor PHI receiver route
Related:
  - docs/development/current/main/phases/phase-29cv/P283A-MODULE-GENERIC-UNARY-NOT-CONSUME.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/string_corridor.rs
---

# P284a: String Corridor PHI Receiver Route

## Problem

After P283a, source-execution advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=BuilderProgramJsonInputContractBox.has_defs/1
```

The failing body reaches:

```hako
local defs_seg = s.substring(lb + 1, rb)
...
if defs_seg.indexOf("\"name\":\"") < 0 { return 0 }
```

MIR already records the `substring` result as a string-corridor `str.slice`
producer, and `generic_method_routes` publishes the `substring` LoweringPlan.
The remaining gap is that the `indexOf` receiver is not the direct substring
result. It is carried through copy/PHI values after null/empty guards.

## Decision

Extend generic-method receiver-origin lookup to follow string-corridor facts
through copy-normalized PHI values:

```text
str.slice producer -> copy/PHI receiver -> StringBox origin
```

This is MIR-owned route fact propagation. The C shim continues to consume only
LoweringPlan entries and does not classify raw method names.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C emitter widening
- no `.hako` source workaround in `BuilderProgramJsonInputContractBox.has_defs`
- no collection semantics

## Acceptance

- `BuilderProgramJsonInputContractBox.has_defs/1` advances past module generic
  prepass.
- The source-execution probe advances to the next blocker.
- `cargo test -q generic_method_route_plan::tests::string_routes --lib`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

MIR JSON now publishes the guarded substring receiver route in
`BuilderProgramJsonInputContractBox.has_defs/1`:

```text
b5008.i4 RuntimeDataBox.indexOf -> StringIndexOf / DirectAbi
```

Fresh source-execution probe advanced past
`BuilderProgramJsonInputContractBox.has_defs/1` to:

```text
reason=module_generic_body_emit_failed
target_shape_blocker_symbol=CliRunLowerBox._emit_mir/3
```
