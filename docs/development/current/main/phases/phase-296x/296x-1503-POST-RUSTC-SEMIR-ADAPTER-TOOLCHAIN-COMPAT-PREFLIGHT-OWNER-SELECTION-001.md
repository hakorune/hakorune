# 296x-1503 POST-RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after the standalone rustc semantic adapter reports
toolchain compatibility.

The previous row is diagnostic-only and reports the local toolchain as:

```text
rustc_channel=stable_or_release
rustc_private_readiness=requires_nightly_or_bootstrap
```

This row decides whether to start a rustc-internal inventory probe, first
document a toolchain setup path, or return to verified-plan emitter work.

## Selected By

```text
296x-1502-RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-001
```

## Candidate Owners

```text
A. HIR item/provenance inventory probe
   value: first rustc semantic adapter step with no Hako lifecycle policy
   risk: current local stable toolchain reports requires_nightly_or_bootstrap

B. rustc adapter toolchain setup / override design
   value: decide nightly / RUSTC_BOOTSTRAP / pinned toolchain route before
          rustc_private extraction rows
   risk: design row before new facts exist

C. source-shape probe retirement policy
   value: prevent legacy source-shape probes from competing with rustc facts
   risk: premature before rustc semantic facts are emitted

D. return to lifecycle emitter parity
   value: continue verified-plan renderer work without rustc internals
   risk: delays replacement of hand-authored adapter facts
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
facts_generated=0
hako_plan_emitted=0
hako_source_emitted=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_extract_HIR_THIR_MIR_in_selection=1
do_not_add_rustc_private_dependency_in_product_crates=1
do_not_generate_lifecycle_facts_in_selection=1
do_not_emit_HakoLifecyclePlan_in_selection=1
do_not_change_backend=1
```
