# 293x-507 GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001

Status: landed
Date: 2026-05-17

## Decision

`GLOBAL-STRING-BODY-ANALYSIS-SPLIT-001` is a BoxShape cleanup for global-call
generic string body analysis. It splits the large body-analysis owner by
analysis phase without changing accepted routes or backend behavior.

## Scope

- Add a submodule directory under
  `src/mir/global_call_route_plan/generic_string_body_analysis/`.
- Keep the public entry point
  `generic_pure_string_instruction_reject_reason(...)` stable.
- Move coherent analysis phases out of
  `src/mir/global_call_route_plan/generic_string_body_analysis.rs`.
- Preserve value-class mutation, reject reasons, and return-contract behavior.

## Stop Lines

- Do not change accepted global-call route behavior.
- Do not change generic string value-class semantics, reject reason spelling,
  route proof strings, return contracts, or backend emission.
- Do not touch generic method route planning, allocator behavior, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GSBA.1` | Add the submodule owner and move pure helper groups only. | focused tests remain green. | no behavior change |
| `GSBA.2` | Move one instruction-family handler group behind the new owner. | value-class tests remain green. | no reject spelling change |
| `GSBA.3` | Move one call/extern/global-call handler group behind the new owner. | route tests remain green. | no accepted route change |
| `GSBA.4` | Verify and close out. | required evidence is green. | no adjacent cleanup |

## Required Evidence

```text
cargo test -q global_call_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row split `generic_string_body_analysis.rs` into a small dispatcher/context
owner plus two phase modules:

```text
src/mir/global_call_route_plan/generic_string_body_analysis.rs
src/mir/global_call_route_plan/generic_string_body_analysis/value_transfer.rs
src/mir/global_call_route_plan/generic_string_body_analysis/call_transfer.rs
```

The public entry point
`generic_pure_string_instruction_reject_reason(...)` remains stable. Value-class
transfer, call/extern/global call acceptance, reject reasons, route contracts,
and backend behavior are unchanged.

Evidence:

```text
cargo test -q global_call_route_plan
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
