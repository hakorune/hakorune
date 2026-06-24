---
Status: Complete
Date: 2026-06-24
Scope: Generate the first global-call runtime route matcher.
---

# GLOBAL-CALL-RUNTIME-ROUTE-DESCRIPTOR-GENERATION-001

## Decision

Use `lowering_plan_global_call_view_is_stage1_emit_program_json()` as the next
same-module global-call descriptor consumer.

This moves the C-side fixed runtime route tuple for
`stage1.emit_program_json_v0` into generated global-call descriptor data.

## Source Authority

```text
src/mir/global_call_route_plan/route.rs
  GlobalCallLoweringOverride

src/mir/global_call_route_plan/model.rs
  GlobalCallProof::as_json_name()
  GlobalCallReturnContract::as_json_name()
  GlobalCallReturnContract::value_demand()
```

## Acceptance

```text
generated registry owns route_kind / proof / symbol / return_shape / value_demand
for GlobalCallLoweringOverride runtime routes.

lowering_plan_global_call_view_is_stage1_emit_program_json has no handwritten
route/proof/symbol/shape/demand tuple.

global-call proof registry remains generated
user-box method behavior changed = 0
extern descriptor behavior changed = 0
new canonical MIR instruction = 0
runtime fallback = 0
```

## Verification

```text
python3 tools/global_call_route_descriptor_codegen.py --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result:

```text
All commands above are green.
```

## Non-Claims

```text
global-call direct function emission generation = 0
same-module function definition descriptor generation = 0
user-box method descriptor generation = 0
```
