# 296x-1421 POST-JOIN-ID-VOCABULARY-DECISION-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `CarrierVar.join_id` is parked as
test-fixture/stale vocabulary for the lifecycle lane.

## Selected By

```text
296x-1420-PHI-CARRIER-JOIN-ID-VOCABULARY-DECISION-001
```

## Candidate Owners

```text
A. trim_helper lifecycle inventory/probe
   value: isolates route-specific metadata denied by resolver/emitter
   risk: can expand into all trim route semantics

B. Expand emitter probe to parser/MIR-checkable surface
   value: moves beyond comment-level fixture after join_id is parked
   risk: can become generated-program claim or converter rewrite

C. promoted_body_locals lifecycle probe
   value: isolates owned promoted-name metadata used by resolver lookup
   risk: can expand into body-local promotion route design

D. Ownership-aware converter reference
   value: fixes the user-facing and implementer-facing meaning of
   "converter translates Rust ownership into .hako"
   risk: docs-only detour if it does not return to owner selection
```

## Recommended Direction

```text
recommended=D-lite
reason=before continuing trim_helper / promoted_body_locals work, document the
precise boundary that the converter renders verified lifecycle plans and does
not choose ownership, borrow, move, or Drop policy from Rust syntax.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

## Selection

```text
selected_owner=D-lite
selected_next_task=RUST-TO-HAKO-OWNERSHIP-CONVERTER-REFERENCE-001
selected_reason=the lifecycle lane has enough passive facts/plan/verifier/
emitter vocabulary that the converter boundary must be restated before more
owners are added. The converter/emitter is a verified-plan renderer, not the
ownership policy owner.
```

Parked:

```text
trim_helper lifecycle inventory/probe:
  parked until ownership converter reference is closed

emitter acceptance expansion:
  parked; do not mix parser/MIR surface expansion with converter ownership
  semantics

promoted_body_locals lifecycle probe:
  parked; do not mix body-local promotion route design with converter
  ownership reference
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

Next:

```text
296x-1422-RUST-TO-HAKO-OWNERSHIP-CONVERTER-REFERENCE-001
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_expand_emitter_before_selection=1
do_not_mix_trim_helper_with_promoted_body_locals=1
do_not_reopen_join_id_in_this_selection_row=1
do_not_let_converter_choose_lifecycle_policy=1
```
