Status: Done
Date: 2026-06-17
Scope: report-only user-box method publication classifier
Related:
  - docs/development/current/main/phases/phase-296x/296x-1064-USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-DESIGN-001.md

# USER-BOX-METHOD-PUBLICATION-CLASSIFIER-A-LITE-001

## Purpose

Add the missing report-only publication surface for user-box method direct
routes before any `LocalFastPathFact` producer is allowed to consume them.

The previous row fixed the design boundary:

```text
route-positive != publication-positive
```

This row implements the conservative classifier only. It does not create
backend-consumable facts.

## Implementation

Added:

```text
src/mir/user_box_method_publication.rs
```

The classifier reads `function.metadata.user_box_method_routes` and writes:

```text
function.metadata.user_box_method_publication_classifications
```

Rows are exported to MIR JSON as:

```text
user_box_method_publication_classifications
```

## V0 Acceptance

V0 proves only the narrow local case:

```text
receiver origin = same-block NewBox
no same-block alias publication before the callsite
=> publication_state=unpublished
```

Everything else remains conservative:

```text
param origin => maybe_published
call result => maybe_published
phi => maybe_published
field_get => maybe_published
cross-block NewBox => maybe_published
unknown => maybe_published
```

## Contract

```text
output_contract=user-box-method-publication-classifier-a-lite-v0
classifier_module=src/mir/user_box_method_publication.rs
classifier_report_only=1
backend_consumable=0
local_fastpath_fact_producer_enabled=0

same_block_newbox_unpublished_supported=1
same_block_alias_publication_forces_no_fact=1
cross_block_requires_dominance_proof=1
param_requires_interprocedural_publication_proof=1
call_result_requires_callee_publication_summary=1
unknown_forces_no_fact=1

mir_json_export=user_box_method_publication_classifications
product_default_changed=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0

next_task=USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-PREFLIGHT-001
summary=ok
```

## Stop Lines

```text
do not produce LocalFastPathFact in this row
do not treat param/call/phi/cross-block origins as unpublished
do not add dominance or interprocedural summaries in this row
do not change backend lowering
do not change route priority
```

## Validation

```text
cargo test -q user_box_method_publication --lib
```
