# 296x-1499 POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-DESIGN-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next owner after documenting the rustc semantic adapter tool
preflight contract.

This row must not implement the adapter before one owner is selected.

## Selected By

```text
296x-1498-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-DESIGN-001
```

## Candidate Owners

```text
A. rustc adapter tool preflight implementation
   value: create the standalone adapter tool skeleton and diagnostic-only
          preflight guard
   risk: may expose local toolchain rustc_private limitations

B. root/workspace rustc-private guard
   value: add a no-rustc-private-in-product guard before tool implementation
   risk: guard-only row if A will cover it

C. source-shape probe retirement policy
   value: document when Python extractors stop being authority
   risk: policy before actual rustc adapter exists

D. return to emitter parity
   value: continue lifecycle pipeline using existing extracted facts
   risk: delays rustc semantic adapter replacement
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
adapter_tool_created=0
rustc_internal_adapter_started=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_create_adapter_tool_in_selection=1
do_not_add_rustc_private_dependency_in_selection=1
do_not_generate_facts_in_selection=1
do_not_change_backend_in_selection=1
```
