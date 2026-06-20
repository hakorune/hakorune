# 296x-1501 POST-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-OWNER-SELECTION-001

Status: closed
Date: 2026-06-20

## Purpose

Select the next owner after the standalone rustc semantic adapter tool
preflight is green.

This row must not implement fact extraction before one owner is selected.

## Selected By

```text
296x-1500-RUSTC-SEMIR-ADAPTER-TOOL-PREFLIGHT-IMPLEMENTATION-001
```

## Candidate Owners

```text
A. HIR item/provenance inventory probe
   value: first rustc semantic adapter step with no lifecycle policy
   risk: rustc_private details and toolchain support

B. toolchain compatibility guard hardening
   value: classify nightly/stable/rustc_private readiness before HIR probe
   risk: diagnostic-only row if A can fail-fast clearly

C. source-shape probe retirement policy
   value: define demotion path now that standalone tool exists
   risk: policy before rustc facts are emitted

D. return to emitter parity
   value: continue lifecycle projection without rustc internals
   risk: delays adapter replacement
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
facts_generated=0
backend_behavior_changed=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Selection

```text
selected_owner=B
selected_next_task=RUSTC-SEMIR-ADAPTER-TOOLCHAIN-COMPAT-PREFLIGHT-001
selected_reason=The standalone adapter tool preflight is green, but HIR
inventory is the first row that may need rustc_private/toolchain support.
Before that, add a diagnostic-only compatibility preflight that classifies the
local rustc channel/sysroot readiness without generating facts or falling back
to source-shape probes.
implementation_started=0
facts_generated=0
backend_behavior_changed=0
```

Non-selected owners:

```text
A. HIR item/provenance inventory probe:
  parked until toolchain compatibility is explicit and fail-fast

C. source-shape probe retirement policy:
  parked until rustc semantic facts exist

D. return to emitter parity:
  parked until rustc semantic adapter replacement has a readiness answer
```

## Closeout

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
facts_generated=0
backend_behavior_changed=0
```

## Stop Line

```text
do_not_start_fact_extraction_in_selection=1
do_not_add_Hako_policy_in_selection=1
do_not_change_backend_in_selection=1
```
