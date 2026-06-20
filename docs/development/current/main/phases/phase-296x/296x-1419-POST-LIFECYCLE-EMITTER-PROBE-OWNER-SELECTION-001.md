# 296x-1419 POST-LIFECYCLE-EMITTER-PROBE-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after the first bounded lifecycle emitter
surface is fixture-guarded.

## Selected By

```text
296x-1418-RUST-TO-HAKO-LIFECYCLE-EMITTER-PROBE-001
```

## Candidate Owners

```text
A. Expand emitter probe to one executable/checkable Hako skeleton surface
   value: moves from comment-level surface to parser/MIR acceptance if safe
   risk: may become generated-program claim or converter rewrite

B. join_id vocabulary retirement/design decision
   value: resolves test-only/stale vs real producer before future resolver use
   risk: can expand into JoinIR carrier value-space redesign

C. trim_helper lifecycle inventory/probe
   value: isolates route-specific metadata denied by resolver skeleton
   risk: can expand into all trim route semantics
```

## Recommended Direction

```text
recommended=B-lite
reason=the lifecycle pipeline now has facts, plan, resolver diagnostics,
VerifierResult, and one bounded emitter surface. The largest unresolved
semantic debt remains join_id test-only vocabulary, which should be decided
before expanding emitter or resolver coverage.
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
do_not_expand_emitter_before_owner_selection=1
do_not_delete_or_implement_join_id_in_this_selection_row=1
do_not_mix_trim_helper_probe_with_join_id_decision=1
```
