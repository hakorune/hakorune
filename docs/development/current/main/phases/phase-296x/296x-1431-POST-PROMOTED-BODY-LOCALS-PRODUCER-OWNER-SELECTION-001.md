# 296x-1431 POST-PROMOTED-BODY-LOCALS-PRODUCER-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after trim and DigitPos promoted-body-local
name producers are fixture-guarded.

## Selected By

```text
296x-1430-PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001
```

## Candidate Owners

```text
A. promoted-name resolution deny closeout
   value: explicitly keeps resolve_promoted_join_id blocked until join_id
   producer exists
   risk: can reopen join_id too early

B. Expand emitter probe to parser/MIR-checkable surface
   value: moves beyond comment-level fixture now that denied owners are named
   risk: can become generated-program claim or converter rewrite

C. trim route lowering inventory
   value: documents the next layer after producer-only lifecycle facts
   risk: can reopen route lowering semantics too early
```

## Recommended Direction

```text
recommended=A-lite
reason=promoted_body_locals producers are now fixture-guarded, but promoted
name resolution still depends on join_id. Close that deny boundary before
emitter acceptance or trim route lowering work.
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

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_reopen_join_id_producer=1
do_not_expand_emitter_before_selection=1
do_not_claim_trim_route_lowering_complete=1
do_not_make_converter_core_policy_owner=1
```

