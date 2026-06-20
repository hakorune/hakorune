# 296x-1389 RUST-LIFECYCLE-NEXT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle migration owner after the BindingContext pilot and
oracle parity closeout.

Do not start another implementation row until the next owner is selected.

## Selected By

```text
296x-1388-MIRBUILDER-BINDING-CONTEXT-LIFECYCLE-ORACLE-PARITY-001
```

## Candidate Owners

```text
A. VariableContext lifecycle facts/plan pilot
   value: exercises returned/escaping mutable-map access and snapshot/restore
   risk: broader than BindingContext; may require API redesign or Deny boundary

B. HakoLifecycleResolver read-only skeleton
   value: starts consuming facts/plans without emitting Hako
   risk: easy to become a general resolver too early

C. Lifecycle emitter fail-fast stub
   value: enforces verified-plan-only route
   risk: premature without resolver/verifier path

D. BindingContext authority closeout docs
   value: records exact promotion boundary and non-goals
   risk: mostly docs; may not advance next implementation capability
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
binding_context_scope_preserved=1
mirbuilder_wide_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_start_VariableContext_before_selection=1
do_not_start_general_resolver_before_selection=1
do_not_start_emitter_before_selection=1
do_not_claim_MirBuilder_wide_lifecycle_parity=1
```

## Selection

```text
selected_next_owner=A-lite
selected_next_task=VARIABLE-CONTEXT-LIFECYCLE-GAP-INVENTORY-001
implementation_started=0
```

Rationale:

```text
VariableContext is the next MirBuilder-owned context after BindingContext, but
it introduces wider shapes: returned `&BTreeMap`, returned `&mut BTreeMap`,
snapshot/restore clone policy, SSA renaming, and carrier-sensitive iteration.
Those should be inventoried before a facts/plan pilot starts.
```

Parked:

```text
HakoLifecycleResolver read-only skeleton:
  parked until VariableContext gaps are inventoried

Lifecycle emitter fail-fast stub:
  parked until resolver/verifier path is selected

BindingContext authority closeout docs:
  partially satisfied by 296x-1388 closeout; no separate row now
```

## Closeout Evidence

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
binding_context_scope_preserved=1
mirbuilder_wide_claim=0
```

Next row:

```text
296x-1390-VARIABLE-CONTEXT-LIFECYCLE-GAP-INVENTORY-001
```
