Status: Done
Date: 2026-06-17
Scope: close the fastpath vocabulary slimming row without adding new fastpath
semantics.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1040-FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001.md

# FASTPATH-VOCAB-SLIM-CLOSEOUT-001

## Purpose

Prevent the local-first fastpath vocabulary from growing by near-synonym types.

This row does not introduce another fastpath, route, or backend consumer. It
adds the code-side vocabulary map that future changes must use before adding
more object/fastpath planning names.

## Change

Added `src/object_storage_plan/README.md` as the code-side entry map for the
seven allowed concept groups:

```text
storage
publication
alias
inventory
decision
fastpath
reachability
```

Added report fields that close the slimming row and forbid new synonym types
without a design row.

## Contract

```text
output_contract=fastpath-vocab-slim-closeout-v0
fastpath_vocab_slim_closeout=1
object_storage_plan_readme_defined=1
object_storage_plan_concept_group_count=7
fastpath_new_synonym_type_allowed=0
backend_behavior_changed=0
new_fastpath_enabled=0
route_priority_changed=0
product_default_changed=0
next_task=FRESH-COMPILER-OWNER-SELECTION-006
summary=ok
```

## Stop Lines

```text
do not add a new fastpath in this row
do not rename existing types without migration
do not add fallback facts
do not make inventory rows backend-readable
do not split PublicationPlan out until ObjectPlan becomes too large
```

