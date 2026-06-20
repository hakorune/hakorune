# 296x-1465 POST-LIFECYCLE-PROJECTION-REFERENCE-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after documenting the Rust-to-Hako lifecycle projection
reference.

This row must not implement resolver, verifier, or emitter behavior.

## Selected By

```text
296x-1464-RUST-LIFECYCLE-OWNERSHIP-PROJECTION-REFERENCE-001
```

## Candidate Owners

```text
A. return to trim route executable pilot fixture selection
   value: resumes the active trim route lowering lane
   risk: behavior-changing pilot follows soon

B. RustLifecycleFacts adapter inventory for BindingContext/VariableContext
   value: identifies the exact rustc facts needed before lifecycle-aware
          converter work
   risk: docs/inventory step before implementation

C. HakoLifecycleVerifier first negative/positive fixture
   value: turns lifecycle projection into a checkable gate
   risk: opens verifier implementation surface
```

## Recommended Direction

```text
recommended=B
reason=the reference says the adapter must produce facts before the converter
can claim lifecycle-aware emission; inventorying the first concrete fact slice
keeps policy out of the converter.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
resolver_implementation_started=0
emitter_implementation_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_emit_lifecycle_aware_hako_without_verified_plan=1
do_not_change_skeleton_converter_semantics=1
do_not_start_rustc_adapter_implementation_in_selection=1
```
