# 296x-1415 POST-READONLY-RESOLVER-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after the read-only resolver skeleton is
diagnostic-guarded.

## Selected By

```text
296x-1414-HAKO-LIFECYCLE-RESOLVER-READONLY-SKELETON-001
```

## Candidate Owners

```text
A. Lifecycle verifier result vocabulary
   value: defines the positive input required before emitter work
   risk: can become broad if it tries to prove every plan family

B. join_id vocabulary retirement/design decision
   value: resolves test-only/stale vs real producer before future resolver use
   risk: can expand into JoinIR carrier value-space redesign

C. trim_helper lifecycle inventory/probe
   value: isolates route-specific metadata denied by resolver skeleton
   risk: can expand into all trim route semantics
```

## Recommended Direction

```text
recommended=A-lite
reason=the emitter contract already requires VerifierResult, and the read-only
resolver now produces diagnostic Allow/Deny only. Define verifier result
vocabulary before any emitter probe.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
converter_emission_added=0
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
do_not_start_emitter_before_verifier_result_owner_selection=1
do_not_turn_readonly_resolver_into_verifier=1
do_not_mix_join_id_design_with_verifier_vocab=1
```
