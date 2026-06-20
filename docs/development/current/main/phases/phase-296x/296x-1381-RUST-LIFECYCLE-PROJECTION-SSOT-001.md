# 296x-1381 RUST-LIFECYCLE-PROJECTION-SSOT-001

Status: planned
Date: 2026-06-20

## Purpose

Fix the ownership boundary for Rust-to-Hako migration before attempting a
MirBuilder lifecycle pilot.

The goal is not to translate Rust lifetime syntax into `.hako`. The goal is:

```text
rustc-proven lifecycle facts
  -> Hako-owned lifecycle plan
  -> verifier
  -> .hako / canonical MIR emitter
```

## Selected By

```text
RustSubset crate-bundle and MirBuilder migration design discussion
```

## SSOT

```text
docs/development/current/main/design/rust-lifecycle-projection-ssot.md
```

## Decision

```text
converter_is_not_ownership_owner=1
rust_adapter_emits_facts_only=1
hako_lifecycle_resolver_owns_representation_policy=1
verifier_owns_projection_validity=1
emitter_reads_verified_plan_only=1
rust_lifetime_syntax_added=0
```

## Task Sequence

```text
1. RUST-LIFECYCLE-PROJECTION-SSOT-001
   Close this docs/design boundary.

2. RUST-LIFECYCLE-FACTS-VOCAB-000
   Add passive RustLifecycleFacts-v0 vocabulary only.

3. HAKO-LIFECYCLE-PLAN-VOCAB-000
   Add passive HakoLifecyclePlan-v0 vocabulary only.

4. MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
   Project BindingContext BTreeMap/&self/&mut self/memory-only Drop into
   OrderedMapBox and Hako lifecycle plans.

5. MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001
   Compare against Rust oracle vectors and promote only the BindingContext
   family if green.
```

## Acceptance

```text
rust_lifecycle_projection_ssot_exists=1
adapter_resolver_verifier_emitter_boundaries_documented=1
converter_direct_ownership_policy_forbidden=1
first_pilot=BindingContext
current_filebox_crate_bundle_blocker_unchanged=1
implementation_started=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_start_lifecycle_pilot_before_crate_bundle_transport_closes=1
do_not_let_adapter_choose_Hako_representation=1
do_not_add_Rust_lifetime_syntax_to_Hako=1
do_not_erase_Drop_without_positive_TrivialMemory_fact=1
do_not_map_every_Drop_impl_to_fini=1
do_not_map_Arc_to_ordinary_box_when_Arc_behavior_is_observed=1
do_not_claim_crate_wide_executable_parity=1
```

## Notes

This row is a planned follow-up. It does not replace the active 296x-1380
FileBox dynamic path loop blocker, and it does not reopen crate aggregation.
