# 296x-1388 MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001

Status: open
Date: 2026-06-20

## Purpose

Compare the BindingContext lifecycle pilot against Rust oracle vectors before
promoting any Hako authority claim.

This row should verify behavior for the BindingContext family only.

## Selected By

```text
296x-1387-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-PILOT-001
```

## Scope

Compare oracle vectors for:

```text
new/default
is_empty
len
contains
lookup
insert
remove
clear_for_function_entry
deterministic iteration expectation
memory-only Drop erase evidence
```

Allowed implementation shape:

```text
fixture/probe first
BindingContext only
no general resolver
no converter lifecycle emission
no VariableContext
```

## Acceptance

```text
binding_context_oracle_vectors_exist=1
binding_context_plan_matches_oracle=1
ordered_map_determinism_checked=1
drop_erase_claim_limited_to_TrivialMemory=1
hako_authority_promoted_for_BindingContext_only=1
general_resolver_implemented=0
converter_lifecycle_emission_added=0
rust_lifetime_syntax_added=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_generalize_to_VariableContext=1
do_not_implement_full_lifecycle_resolver=1
do_not_emit_verified_plan_to_Hako_yet=1
do_not_claim_MirBuilder_wide_lifecycle_parity=1
```
