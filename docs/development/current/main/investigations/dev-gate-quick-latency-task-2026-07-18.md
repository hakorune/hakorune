# Dev Gate Quick Latency Task

Status: Parked; decision locked
Date: 2026-07-18
Scope: `tools/checks/dev_gate.sh quick` developer latency only
Active compiler lane delta: 0

## Problem

The complete quick profile is still the stable daily gate, but its wall time is
not predictably quick.

```text
reported partially cold run: 134s
warm observations:           16.5s and 42s
quick membership:            66 sequential steps
approximate Cargo launches:  76
```

The variance matters as much as the warm minimum. A partially invalidated run
can rebuild several different artifact tuples:

```text
release + LTO ny-llvmc
root default
root vm-reference
nyash_kernel
cargo check
```

The repository target directory is already large, and no shared compiler cache
is currently selected. Therefore this task must measure cold, partially
invalidated, and warm behavior separately. A single warm result is not a
performance authority.

## Decision

Do not introduce a second stable `ultra-quick` profile. Keep `quick` as the one
complete daily/PR gate and add a conservative changed-path selector for the
edit-run loop.

Do not parallelize the current runner first. Its tested contract stops after
the first failing step, while Cargo jobs also share target locks and CPU. Safe
parallel execution would change failure scheduling and is a separate design
row.

```text
DEV-GATE-Q0-M0
  -> DEV-GATE-Q0-C0
  -> DEV-GATE-Q0-SEL0
  -> DEV-GATE-Q0-G0

park:
  DEV-GATE-Q0-PAR0
```

The sole first code-facing row is `DEV-GATE-Q0-M0`. This task remains parked
while the callable-result expression-spine lane is active.

## Authority boundary

```text
complete quick membership:
  tools/checks/lib/dev_gate_quick_steps.sh

group execution/failure evidence:
  tools/checks/lib/dev_gate_group.sh

stable public gate entry:
  tools/checks/dev_gate.sh quick

changed-path selection:
  one manifest consumed through the existing manifest runner family
```

The selector does not become a second quick inventory. It maps changed paths
to existing row/proof/public guard entries. Unknown paths, changes to selector
authority, and unclassified paths must return an explicit `QUICK-REQUIRED`
decision.

## DEV-GATE-Q0-M0 — measurement inventory

Production/check behavior delta: 0.

Add one machine-readable inventory of every quick step with:

```text
stable step id and label
command/public entry
artifact tuple
read/write class
declared dependencies
cold / partial-invalidation / warm timing samples
exit status
```

Expose timing through an explicit measurement CLI surface, not a hidden
environment toggle. Timing is diagnostic evidence and must never become a CI
pass/fail threshold.

M0 must identify at least:

```text
source/docs/git-only structural guards
Cargo artifact producers
artifact-reading execution/acceptance checks
duplicate Cargo invocations by tuple/filter/features/profile
release+LTO work triggered by a nominally quick guard
```

## DEV-GATE-Q0-C0 — Cargo and barrier cleanup

Preserve all 66 checks and their exact filters/features/results while reducing
redundant artifact work.

The required order is:

```text
structural read-only guards
-> explicit Cargo artifact barriers grouped by exact tuple
-> artifact-reading execution/acceptance checks
```

Evaluate `profile.quick` only with cold, partial-invalidation, and warm
evidence. It is selected only if result parity is exact and it does not create
an unacceptable extra artifact/space frontier.

C0 does not run Cargo commands concurrently against one target directory and
does not delete tests or weaken features to obtain a smaller number.

## DEV-GATE-Q0-SEL0 — changed-path selector

Add one manifest-backed selector for the local edit-run loop.

```text
known path:
  select the existing focused guard/proof entry

unknown or ambiguous path:
  QUICK-REQUIRED

selector/manifest/runner change:
  QUICK-REQUIRED

silent skip:
  forbidden
```

The selector must provide a check-only report containing the changed path,
matched rule, selected public entries, and reason. Reuse
`run_row_guard.sh`, `run_proof_app.sh`, and `manifest_runner.py`; do not add a
second execution/selection framework.

The complete `quick` profile remains required for PR/milestone closeout and
whenever the selector says `QUICK-REQUIRED`.

## DEV-GATE-Q0-G0 — closeout

Lock:

```text
quick step coverage remains 66/66
direct entry and selector result parity
unknown-path fail-fast
selector manifest coverage
Cargo tuple producer cardinality
failure exit/label/log parity
cold / partial / warm evidence
docs/tools/check-scripts-index.md entry
```

The desired 40–60 second range is an optimization target for affected
partially invalidated runs, not a stable correctness contract.

## DEV-GATE-Q0-PAR0 — parked parallel runner

Open only if M0/C0/SEL0 leave material latency that cannot be removed by
artifact reuse and focused selection.

PAR0 requires a separate decision because parallel batches alter the current
"first failure stops all later steps" contract. It must define deterministic
diagnostic ordering, cancellation, resource bounds, and Cargo isolation before
implementation. Shell `&` plus `wait` is not an authorized implementation.

## Acceptance

```text
quick membership loss = 0
quick result/exit parity = exact
test filter or feature removal = 0
unclassified changed paths silently skipped = 0
selector result = direct public-entry result
compile owner count per exact Cargo tuple = 1 where consolidation is valid
release/default/vm-reference/kernel artifact conflation = 0
first failing label, exit code, bounded tail, and full log remain available
timing threshold used as CI correctness gate = 0
compiler/MIR/runtime semantic delta = 0
```

## Stop conditions

Stop the current row if any of the following becomes necessary:

1. Run Cargo commands concurrently against the same target directory.
2. Create separate target directories that regress cold time or disk use
   without an explicit resource decision.
3. Remove a test filter, feature, or quick member to improve timing.
4. Let a parallel batch violate the current first-failure contract.
5. Let the changed-path selector silently skip an unknown path.
6. Duplicate quick membership inside the selector manifest.
7. Add a selector authority separate from the existing manifest runner family.
8. Add a hidden environment toggle.
9. Mix this tools/DX change with a callable-result or other semantic row.

## Final lock

> Quick-gate latency is a separate tools/DX task. The complete 66-step `quick`
> profile remains the sole stable daily/PR gate. `DEV-GATE-Q0-M0` first records
> per-step timing, exact Cargo artifact tuples, reads/writes, and dependencies;
> `C0` then consolidates redundant artifact work without removing checks;
> `SEL0` adds one conservative manifest-backed changed-path selector whose
> unknown result is explicitly `QUICK-REQUIRED`; and `G0` locks parity and
> evidence. A permanent `ultra-quick` profile is rejected because it would
> duplicate daily-gate authority. Parallel execution remains parked in PAR0
> because it changes first-failure scheduling and shared Cargo resource laws.
