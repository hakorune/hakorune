---
Status: resolved
Date: 2026-07-28
Decision: minimal four-metric ratchet
Scope: prevent MirBuilder source/test footprint growth without creating a planning subsystem
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Execution:
  - docs/development/current/main/investigations/mirbuilder-structural-budget0-closeout-task-2026-07-28.md
---

# MirBuilder structural budget D0

## Accepted decision

Structural size is a result metric, not MirBuilder design authority.

Implement only:

```text
two fixed source roots
four find/wc measurements
one TSV ratchet row
one comparison in the existing shared guard
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

Frozen values:

```text
source_files = 952
source_loc   = 182452
test_files   = 139
test_loc     = 40826
```

Every closed state must remain at or below the ratchet row in all four
dimensions. Pack close lowers each ceiling to the measured minimum.

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

The structural ratchet only proves that implementation and test footprint did
not grow while the semantic replacement progressed.

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

After the minimal ratchet lands, activate the accepted Binary Option A
execution task immediately. The Binary accounting decision is already
accepted; no seventh row is created in the budget task.
