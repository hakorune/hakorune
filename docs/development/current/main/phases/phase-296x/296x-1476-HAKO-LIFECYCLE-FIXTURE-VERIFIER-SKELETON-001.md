# 296x-1476 HAKO-LIFECYCLE-FIXTURE-VERIFIER-SKELETON-001

Status: open
Date: 2026-06-20

## Purpose

Add a small fixture-only verifier skeleton that checks existing
RustLifecycleAdapterFacts / HakoLifecyclePlan / HakoLifecycleVerifierResult
JSON fixtures.

This is not rustc integration and not lifecycle-aware converter emission.

## Selected By

```text
296x-1475-POST-VARIABLE-CONTEXT-ADAPTER-VERIFIER-OWNER-SELECTION-001
```

## Scope

Create a reusable checker under:

```text
tools/rust_lifecycle/
```

It should verify the existing context fixtures:

```text
binding-context-adapter-facts-v0.json
binding-context-plan-v0.json
binding-context-adapter-verifier-result-v0.json

variable-context-adapter-facts-v0.json
variable-context-*-plan-v0.json
variable-context-adapter-verifier-result-v0.json
```

The checker is allowed to read JSON fixtures and report pass/fail. It must not:

```text
invoke rustc
rewrite converter output
emit .hako
choose Hako lifecycle policy
change backend behavior
```

## Acceptance

```text
fixture_verifier_skeleton_exists=1
binding_context_case_verified=1
variable_context_case_verified=1
rustc_toolchain_integration_started=0
resolver_implementation_started=0
emitter_implementation_started=0
converter_core_changed=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_fixture_verifier_skeleton_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_invoke_rustc=1
do_not_choose_hako_policy=1
do_not_emit_lifecycle_aware_hako=1
do_not_change_converter_core=1
```
