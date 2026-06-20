# 296x-1497 POST-RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after documenting the rustc semantic adapter boundary.

This row must not invoke rustc internals or widen lifecycle extraction before
one owner is selected.

## Selected By

```text
296x-1496-RUSTC-SEMIR-INTERNAL-ADAPTER-BOUNDARY-DESIGN-001
```

## Candidate Owners

```text
A. rustc adapter crate/tool preflight design
   value: choose where rustc_private lives and how toolchain version is
          isolated
   risk: still design-heavy but required before implementation

B. rustc adapter minimal implementation probe
   value: start extracting one HIR-level subject identity fact
   risk: may open rustc_private details before tool boundary is fixed

C. source-shape probe retirement policy
   value: prevent Python extractors from becoming long-term authority
   risk: cleanup-only unless tied to a rustc adapter implementation

D. return to emitter parity
   value: consume already verified extracted facts in emitter path
   risk: delays rustc semantic adapter replacement
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
rustc_internal_adapter_started=0
wider_context_extraction_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_invoke_rustc_internals_in_selection=1
do_not_start_adapter_implementation_in_selection=1
do_not_widen_source_shape_extraction_in_selection=1
do_not_change_backend_in_selection=1
```
