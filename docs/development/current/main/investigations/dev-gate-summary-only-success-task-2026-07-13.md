---
Status: Closed
Date: 2026-07-13
Scope: `tools/checks/dev_gate.sh quick` green-path output only
Related:
  - tools/checks/dev_gate.sh
  - tools/checks/lib/dev_gate_group.sh
  - tools/checks/lib/dev_gate_group_test.sh
  - docs/tools/check-scripts-index.md
---

# Dev Gate Summary-Only Success V1

## Problem

The compact runner hides child stdout/stderr, but a green quick run still
prints one line for each of its 66 steps. Those repeated success lines do not
help the normal pass/fail decision.

## Authority boundary

```text
green-path presentation:
  tools/checks/lib/dev_gate_group.sh

step membership:
  tools/checks/lib/dev_gate_quick_steps.sh (unchanged)

child result and failure diagnostics:
  unchanged
```

## Contract

- Default success prints only the group PASS summary.
- Default success does not print child output or per-step `ok` lines.
- Failure still names the exact step, preserves its exit status, prints a
  bounded tail, and retains the complete log.
- `DEV_GATE_VERBOSE=1` still streams child output and prints step boundaries.
- `--list`, quick membership, execution order, and stop-on-first-failure remain
  unchanged.

The outer `dev_gate.sh` profile summary remains separate, so a green quick run
has two stable lines: group evidence and profile evidence.

## Fixture

`tools/checks/lib/dev_gate_group_test.sh` locks the summary-only success path,
verbose diagnosis, and retained failure evidence.

## Non-goals

- no compiler or B0-L3b semantic change
- no quick step removal or parallel execution
- no warning suppression or child guard rewrite
- no new environment variable

## Acceptance

```bash
bash tools/checks/lib/dev_gate_group_test.sh
bash -n tools/checks/dev_gate.sh tools/checks/lib/dev_gate_group.sh \
  tools/checks/lib/dev_gate_group_test.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout evidence

- library contract: `[dev-gate-group-test] ok`
- real quick profile: `PASS 66/66`
- green quick output: 2 lines total
- current-state pointer guard: green
- active compiler lane remains B0-L3b; this tools-only task does not move it
