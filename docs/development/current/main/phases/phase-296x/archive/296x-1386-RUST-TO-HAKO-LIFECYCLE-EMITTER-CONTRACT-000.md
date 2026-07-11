# 296x-1386 RUST-TO-HAKO-LIFECYCLE-EMITTER-CONTRACT-000

Status: closed
Date: 2026-06-20

## Purpose

Document the converter/emitter contract for rendering verified
`HakoLifecyclePlan-v0` into `.hako` / canonical MIR surfaces.

This row does not implement emission. It only fixes the contract that the
emitter reads verified plans and does not choose ownership policy.

## Selected By

```text
296x-1385-HAKO-LIFECYCLE-PLAN-VOCAB-000
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
docs/development/current/main/design/rust-lifecycle-facts-vocab-v0.md
docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md
docs/development/current/main/design/rust-to-hako-lifecycle-emitter-contract.md
```

## Scope

Define the passive emitter contract:

```text
input:
  verified HakoLifecyclePlan-v0

allowed:
  render record / box / function / method text
  render verified cleanup / birth / field initializer shapes
  render verified BorrowView / TransferOwned surfaces
  preserve diagnostics / source provenance
  fail-fast when plan verification is missing

forbidden:
  choose record vs box from Rust syntax
  choose OrderedMapBox directly from BTreeMap syntax
  erase Drop without a verified plan
  turn &mut into mutation without a verified non-escape plan
  turn Arc/Rc into ordinary boxes without observation facts
  invent fallback ownership when facts or plans are unknown
```

## Acceptance

```text
lifecycle_emitter_contract_exists=1
emitter_reads_verified_plan_only=1
converter_direct_ownership_policy_forbidden=1
emitter_fail_fast_on_missing_plan=1
emitter_behavior_added=0
resolver_behavior_added=0
binding_context_pilot_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_implement_converter_emission_in_this_row=1
do_not_start_BindingContext_lifecycle_pilot=1
do_not_add_Rust_lifetime_syntax=1
do_not_add_fallback_ownership_policy=1
```

## Next

```text
296x-1387-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
```

## Closeout Evidence

```text
lifecycle_emitter_contract_exists=1
emitter_reads_verified_plan_only=1
converter_direct_ownership_policy_forbidden=1
emitter_fail_fast_on_missing_plan=1
emitter_behavior_added=0
resolver_behavior_added=0
binding_context_pilot_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
