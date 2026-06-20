# 296x-1511 BINDING-CONTEXT-HAKO-LIFECYCLE-PROJECTION-AND-AUTHORITY-PROMOTION-001

Status: open
Date: 2026-06-20

## Purpose

Project the selected BindingContext rustc facts into the existing
HakoLifecyclePlan path and decide the first family-scoped authority promotion
surface.

This row may use the checked HIR owner contract, THIR body inventory, and MIR
lifecycle facts for BindingContext. It must preserve Rust bootstrap, Rust
oracle vectors, and explicit compatibility routes.

## Selected By

```text
296x-1510-RUSTC-SEMIR-ADAPTER-BINDING-CONTEXT-MIR-LIFECYCLE-FACTS-001
```

## Scope

Allowed:

```text
BindingContext family only
HakoLifecyclePlan projection from checked rustc facts
verifier Allow/Deny for BindingContext plan
oracle parity against existing BindingContext vectors
canonical MIR parity if the row reaches executable Hako authority
explicit selfhost-mainline authority promotion wording
Rust oracle/bootstrap/compat retention wording
focused guard
```

Forbidden:

```text
crate-wide MirBuilder authority claim
VariableContext promotion
parser/resolver migration
Rust bootstrap removal
silent fallback from Hako mainline to Rust oracle
backend behavior change outside the selected family
wide lifecycle resolver rewrite
```

## Acceptance

```text
binding_context_family_selected=1
hako_lifecycle_plan_projected=1
binding_context_verifier_green=1
rust_oracle_available=1
rust_bootstrap_available=1
rust_compat_route_explicit=1
silent_rust_fallback=0
wide_mirbuilder_authority_claim=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rustc_semir_adapter_binding_context_mir_lifecycle_facts_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_promote_more_than_BindingContext_family=1
do_not_remove_Rust_bootstrap=1
do_not_silently_fallback_to_Rust_oracle=1
do_not_claim_crate_wide_MirBuilder_authority=1
do_not_mix_VariableContext_or_parser_migration=1
```
