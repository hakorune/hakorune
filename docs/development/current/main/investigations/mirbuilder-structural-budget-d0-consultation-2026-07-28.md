---
Status: closed; growth-failure rule superseded by measurement-only policy
Date: 2026-07-28
Decision: minimal four-metric structural observation
Scope: measure MirBuilder source/test footprint without creating a planning subsystem
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Execution:
  - docs/development/current/main/investigations/mirbuilder-structural-budget0-closeout-task-2026-07-28.md
---

# MirBuilder structural budget D0

## Accepted decision, as amended

Structural size is a result metric, not MirBuilder design authority.

Implement only:

```text
two fixed source roots
four find/wc measurements
one TSV baseline row
one measurement report in the existing shared guard
```

Measured roots:

```text
src/mir/builder
crates/hakorune_mir_builder
```

Filename split:

```text
source = *.rs excluding *test*.rs
test   = *test*.rs
```

Baseline values:

```text
source_files = 952
source_loc   = 182452
test_files   = 139
test_loc     = 40826
```

Every closeout records the four current values and their delta from baseline.
An increase is a review fact, not an automatic failure. Pack close or an
explicit structural review may update the baseline to the current measurement.

## Correction of the rejected design

The earlier draft expanded a size check into:

```text
three typed products
closed-world classification rules
eight rule shards
path-set digests
four final-X derivations
Python checker and self-tests
```

That design is rejected. It would add a management subsystem to solve a file
growth problem and delay the actual MirBuilder replacement.

Also rejected:

```text
Keep / Merge / Delete / Proof planning ledger
open / settled state
repository-wide ownership census
throwaway prototype requirement for budget acceptance
absolute final X as completion authority
```

## Completion law

Semantic evidence remains authoritative:

```text
packs closed
old production authority = 0
fallback / retry = 0
parity green
```

The structural observation makes implementation and test footprint changes
visible while semantic replacement progresses. It does not decide whether a
responsibility owner is acceptable.

Existing policy already forbids per-cell shell guards and files at or above
800 lines. No separate check-file inventory is required.

## External responsibility escape

The known external Context crate is included directly in the measured roots:

```text
crates/hakorune_mir_builder
```

Do not build an external ownership manifest. If a future MirBuilder
responsibility moves to another root, update the fixed measured-root list
explicitly in policy and the shared guard.

## Handoff

Execution authority:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget0-closeout-task-2026-07-28.md
```

The minimal ratchet is landed and the accepted Binary Option A execution task
is active. The Binary accounting decision was already accepted; no seventh row
was created in the budget task.
