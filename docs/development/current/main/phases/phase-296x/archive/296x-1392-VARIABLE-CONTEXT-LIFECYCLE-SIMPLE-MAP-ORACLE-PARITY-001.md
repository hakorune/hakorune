# 296x-1392 VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-ORACLE-PARITY-001

Status: closed
Date: 2026-06-20

## Purpose

Compare the VariableContext simple-map lifecycle pilot against Rust oracle
vectors before any broader VariableContext authority claim.

## Selected By

```text
296x-1391-VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001
```

## Scope

Oracle vectors for included simple-map behavior:

```text
new/default
lookup
contains
len
is_empty
insert
remove
deterministic iteration
SSA overwrite
TrivialMemory Drop erase
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-simple-map-oracle-vectors-v0.json
```

Still excluded:

```text
variable_map()
variable_map_mut()
snapshot()
restore()
carrier extraction consumers
PHI planner integration
```

## Acceptance

```text
variable_context_simple_map_oracle_vectors_exist=1
variable_context_simple_map_plan_matches_oracle=1
ordered_map_determinism_checked=1
ssa_overwrite_checked=1
drop_erase_claim_limited_to_TrivialMemory=1
hako_authority_promoted_for_VariableContext_simple_map_only=1
returned_map_methods_excluded=1
snapshot_restore_excluded=1
carrier_consumers_excluded=1
general_resolver_implemented=0
converter_lifecycle_emission_added=0
rust_lifetime_syntax_added=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_simple_map_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_model_variable_map_mut=1
do_not_model_snapshot_restore=1
do_not_claim_full_VariableContext_parity=1
do_not_claim_carrier_or_PHI_parity=1
do_not_implement_general_resolver=1
```

## Closeout Evidence

```text
variable_context_simple_map_oracle_vectors=green
variable_context_simple_map_plan_matches_oracle=green
ordered_map_determinism_checked=green
ssa_overwrite_checked=green
drop_erase_claim_limited_to_TrivialMemory=green
hako_authority_promoted_for_VariableContext_simple_map_only=green
returned_map_methods_excluded=green
snapshot_restore_excluded=green
carrier_consumers_excluded=green
general_resolver_implemented=0
converter_lifecycle_emission_added=0
rust_lifetime_syntax_added=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_simple_map_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Next row:

```text
296x-1393-RUST-LIFECYCLE-POST-VARIABLE-SIMPLE-MAP-OWNER-SELECTION-001
```
