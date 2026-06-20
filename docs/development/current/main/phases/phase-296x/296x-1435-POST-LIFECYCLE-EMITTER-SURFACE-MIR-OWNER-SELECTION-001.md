# 296x-1435 POST-LIFECYCLE-EMITTER-SURFACE-MIR-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after the bounded
`CarrierInfo::merge_from` emitter surface reaches MIR emit.

## Selected By

```text
296x-1434-LIFECYCLE-EMITTER-PARSER-MIR-SURFACE-PROBE-001
```

## Candidate Owners

```text
A. trim route lowering inventory
   value: documents the next route layer after lifecycle producer facts and
   emitter surface acceptance
   risk: can reopen route lowering semantics too early

B. second lifecycle emitter surface
   value: proves another verified-plan surface can be rendered
   risk: can become converter rewrite without a new verified plan

C. rustc lifecycle facts adapter probe
   value: begins external rustc facts production
   risk: design-heavy; may require toolchain and schema consultation
```

## Recommended Direction

```text
recommended=A-lite
reason=the emitter surface now reaches MIR emit. Before adding more emitter
surfaces or rustc adapter work, document the trim route lowering boundary that
must remain separate from lifecycle producer facts.
```

## Decision

```text
selected_next_task=TRIM-ROUTE-LOWERING-INVENTORY-001
selected_scope=inventory-only boundary documentation for trim route lowering
selected_reason=trim_helper production, promoted_body_locals recording,
promoted-name denial, and emitter surface MIR acceptance are now bounded.
The remaining ambiguity is where actual trim route lowering starts, and that
must be named before adding another emitter surface or rustc adapter probe.
implementation_started=0
```

## Parked Owners

```text
second lifecycle emitter surface:
  parked until trim route lowering boundary is documented.

rustc lifecycle facts adapter probe:
  parked until the current route-local lifecycle chain has a clear lowering
  owner boundary.
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
do_not_implement_trim_route_lowering_in_selection=1
do_not_add_second_emitter_surface_before_selection=1
do_not_start_rustc_adapter_without_design_card=1
do_not_claim_generated_program_execution=1
```
