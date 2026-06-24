---
Status: Complete
Date: 2026-06-24
Scope: Generate global-call definition-owner metadata.
---

# GLOBAL-CALL-DEFINITION-OWNER-DESCRIPTOR-GENERATION-001

## Decision

Use `GlobalCallProof::definition_owner()` as the source authority for
global-call definition-owner validation.

The C shim currently compares `definition_owner` strings directly for
global-call views. This slice keeps the consumer categories but verifies the
view's owner against generated proof metadata first.

## Source Authority

```text
src/mir/global_call_route_plan/model.rs
  GlobalCallProof::definition_owner()
  GlobalCallDefinitionOwner::as_json_name()
```

## Acceptance

```text
generated registry owns proof -> definition_owner mapping
lowering_plan_global_call_view_definition_owner_is validates through registry
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
user-box definition-owner mapping generation = 0
same-module function definition descriptor generation = 0
global-call direct-target predicate generation = 0
```
