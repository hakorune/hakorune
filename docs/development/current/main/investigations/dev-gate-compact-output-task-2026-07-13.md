---
Status: Closed
Date: 2026-07-13
Scope: `tools/checks/dev_gate.sh quick` success-output compaction only
Related:
  - tools/checks/dev_gate.sh
  - tools/checks/lib/dev_gate_group.sh
  - tools/checks/lib/dev_gate_quick_steps.sh
  - docs/tools/check-scripts-index.md
---

# Dev Gate Compact Output V1

## Problem

The quick profile owns 66 steps. Its shared group runner currently streams
every child stdout/stderr line, so repeated `cargo test -q` warnings turn a
green incremental run into roughly 9,000 lines of output.

## Authority boundary

```text
output policy owner:
  tools/checks/lib/dev_gate_group.sh

step membership owner:
  tools/checks/lib/dev_gate_quick_steps.sh (unchanged)

child guard result owner:
  each existing guard (unchanged)
```

The runner captures output; it does not suppress compiler warnings through
`RUSTFLAGS`, reinterpret child exit status, or change which checks run.

## Contract

- Default mode prints one `ok` line per successful step and one final PASS
  summary.
- Successful child stdout/stderr is hidden from the terminal.
- On failure, the original nonzero status is returned, the failing step is
  named, a bounded log tail is printed, and the complete log is retained.
- `DEV_GATE_VERBOSE=1` preserves streamed child output for interactive
  diagnosis.
- `--list` output and quick-profile membership remain unchanged.
- Later steps do not execute after the first failed step.

## Fixtures

One library-level shell contract test covers:

1. compact success hides child stdout/stderr and emits the PASS summary;
2. verbose success exposes child stdout/stderr;
3. compact failure preserves exit status, prints the marker, retains the full
   log, and does not execute the following step.

## Non-goals

- no B0-L3a or compiler semantic change
- no quick-profile step removal or allocator-wide reshuffle
- no child-guard output rewrite
- no warning disable flag

## Acceptance

```bash
bash tools/checks/lib/dev_gate_group_test.sh
bash -n tools/checks/dev_gate.sh tools/checks/lib/dev_gate_group.sh \
  tools/checks/lib/dev_gate_group_test.sh
tools/checks/dev_gate.sh --list
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout evidence

- library contract: `[dev-gate-group-test] ok`
- real quick profile: `PASS 66/66`
- compact success output: 68 lines total (66 step summaries, PASS, profile)
- current-state pointer guard: green
- active compiler lane remains B0-L3a; this tools-only task does not move it
