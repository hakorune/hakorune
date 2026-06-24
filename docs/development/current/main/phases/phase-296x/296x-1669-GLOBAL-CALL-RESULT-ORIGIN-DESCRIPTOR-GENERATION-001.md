---
Status: Complete
Date: 2026-06-24
Scope: Generate global-call result-origin metadata.
---

# GLOBAL-CALL-RESULT-ORIGIN-DESCRIPTOR-GENERATION-001

## Decision

Use `GlobalCallProof::result_origin()` as the source authority for global-call
result-origin publication.

The C shim currently maps `result_origin` strings to local origin constants. This
slice moves that mapping into the generated global-call descriptor registry.

## Source Authority

```text
src/mir/global_call_route_plan/model.rs
  GlobalCallProof::as_json_name()
  GlobalCallProof::result_origin()
```

## Acceptance

```text
generated registry owns proof -> result_origin -> C origin kind mapping
lowering_plan_global_call_view_result_origin_kind has no handwritten
result_origin string table
global-call route emission behavior changed = 0
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
user-box result-origin mapping generation = 0
generic-method result-origin mapping generation = 0
global-call definition-owner generation = 0
```
