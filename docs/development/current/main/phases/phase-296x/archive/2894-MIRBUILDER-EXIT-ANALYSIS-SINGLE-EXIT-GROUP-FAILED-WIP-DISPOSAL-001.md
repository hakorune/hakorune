---
Status: Landed
Date: 2026-07-05
Scope: Failed MirBuilder leaf-pilot WIP disposal.
---

# MIRBUILDER-EXIT-ANALYSIS-SINGLE-EXIT-GROUP-FAILED-WIP-DISPOSAL-001

## Decision

Discard the failed `exit_analysis_is_single_exit_group` adoption WIP instead
of restoring or committing it.

## Reason

The WIP tried to treat `exit_analysis_is_single_exit_group` as another narrow
leaf parity pilot. It did not pass the `.hako` EXE parity gate:

```text
unsupported pure shape for current backend recipe
first_op=mir_call
owner_hint=backend_lowering
reason=global_call_arity_mismatch
callee_symbol=ControlFormBox.is_single_exit_group/2
```

An earlier variant also required new box/object support and hit
`unsupported_newbox_type`. That means this is not a safe leaf-pilot adoption;
it crosses into typed-object/static-helper/backend recipe support and should
not be used as the next migration slice.

## Disposed WIP

```text
stash: wip/exit-analysis-is-single-exit-group (fails parity gate)
files included:
  exit-analysis fixtures
  failed parity gate script
  failed adoption cards
  ControlFormBox helper edit
  stale CURRENT_STATE/task-order pointer edits
```

## Boundaries

- No HakoAdopted decision for `exit_analysis_is_single_exit_group`.
- No check-script index entry for the failed gate.
- No `.hako` helper change is retained.
- Source Selfhost remains unclaimed.
- Next valid work remains the hard-authority pivot:
  select a smallest Fact-owner or REGISTRY-rule contract with fixture-backed
  parity.
