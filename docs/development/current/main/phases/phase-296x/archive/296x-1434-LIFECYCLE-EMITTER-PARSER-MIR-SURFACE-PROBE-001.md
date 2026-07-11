# 296x-1434 LIFECYCLE-EMITTER-PARSER-MIR-SURFACE-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Make the existing bounded lifecycle emitter surface parser/MIR-checkable.

## Selected By

```text
296x-1433-POST-PROMOTED-NAME-RESOLUTION-DENY-OWNER-SELECTION-001
```

## Scope

```text
input_surface=docs/development/current/main/design/fixtures/rust-lifecycle/carrier-info-merge-from-emitter-surface-v0.hako
subject=CarrierInfo::merge_from
plan_kind=OwnedCarrierInfoMerge
```

Allowed:

```text
make the fixture parse/MIR-checkable
add a focused guard
keep comments / TODO body bounded to verified plan surface
```

Forbidden:

```text
generated_program_execution_claim=0
backend_behavior_changed=0
converter_core_changed=0
Rust_behavior_changed=0
trim_route_lowering_claim=0
join_id_producer=0
```

## Expected Artifact

```text
guard=tools/checks/rust_lifecycle_emitter_surface_mir_guard.sh
```

## Acceptance

```text
surface_parse_or_mir_emit=green
emitted_subject=CarrierInfo::merge_from
emitted_plan_kind=OwnedCarrierInfoMerge
denied_boundaries_preserved=1
generated_program_execution_claim=0
backend_behavior_changed=0
converter_core_changed=0
```

## Closeout

```text
surface_parse_or_mir_emit=green
emitted_subject=CarrierInfo::merge_from
emitted_plan_kind=OwnedCarrierInfoMerge
denied_boundaries_preserved=1
generated_program_execution_claim=0
backend_behavior_changed=0
converter_core_changed=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_emitter_surface_mir_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-emitter-surface-mir-v0
surface_parse_or_mir_emit=green
emitted_subject=CarrierInfo::merge_from
emitted_plan_kind=OwnedCarrierInfoMerge
denied_boundaries_preserved=1
generated_program_execution_claim=0
backend_behavior_changed=0
converter_core_changed=0
summary=ok
```

Next:

```text
296x-1435-POST-LIFECYCLE-EMITTER-SURFACE-MIR-OWNER-SELECTION-001
```

Checks:

```bash
bash tools/checks/rust_lifecycle_emitter_surface_mir_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_emit_executable_program_claim=1
do_not_rewrite_converter_core=1
do_not_add_join_id_producer=1
do_not_lower_trim_route=1
```
