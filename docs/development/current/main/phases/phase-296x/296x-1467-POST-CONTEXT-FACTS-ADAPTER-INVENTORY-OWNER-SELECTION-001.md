# 296x-1467 POST-CONTEXT-FACTS-ADAPTER-INVENTORY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after inventorying BindingContext / VariableContext
RustLifecycleFacts adapter requirements.

This row must not implement adapter, resolver, verifier, or emitter behavior.

## Selected By

```text
296x-1466-RUST-LIFECYCLE-FACTS-ADAPTER-CONTEXT-INVENTORY-001
```

## Candidate Owners

```text
A. BindingContext adapter fact fixture
   value: smallest compact RustLifecycleFacts bundle; mirrors already-green
          BindingContext lifecycle pilot
   risk: starts fixture schema before rustc tool integration

B. VariableContext adapter fact fixture
   value: covers returned-borrow boundaries and snapshot/restore facts
   risk: broader than first fixture

C. HakoLifecycleVerifier context facts fixture
   value: starts checking facts + plan before adapter implementation
   risk: verifier surface may be premature without one adapter-like fact bundle

D. return to trim route executable fixture selection
   value: resumes trim route lowering lane
   risk: pauses lifecycle projection thread
```

## Recommended Direction

```text
recommended=A
reason=BindingContext is the smallest context family with deterministic map,
read/write receiver, and TrivialMemory Drop facts; it should establish the
adapter fact fixture shape before VariableContext or verifier work.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
adapter_implementation_started=0
resolver_implementation_started=0
verifier_implementation_started=0
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
do_not_start_rustc_toolchain_in_selection=1
do_not_change_converter_core=1
do_not_emit_lifecycle_aware_hako=1
```
