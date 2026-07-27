---
Status: accepted execution task
Date: 2026-07-28
Decision: MIRBUILDER-STRUCTURAL-BUDGET0-CLOSEOUT
Ceremony: policy housekeeping; not a production replacement cell
Commits:
  - one minimal ratchet commit
Parent:
  - docs/development/current/main/investigations/mirbuilder-structural-budget-d0-consultation-2026-07-28.md
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# MIRBUILDER-STRUCTURAL-BUDGET0-CLOSEOUT

## Decision

Implement Structural Budget as a small result metric:

```text
four find/wc measurements
+ one ratchet row
+ one shared-guard comparison
```

Do not build a structural planning system. Semantic completion remains the
authority for whether MirBuilder replacement is finished.

## Measured roots

Measure exactly:

```text
src/mir/builder
crates/hakorune_mir_builder
```

The second root prevents moving Context or another MirBuilder responsibility
outside `src/mir/builder` to create a false reduction.

Do not recursively discover or classify other repository paths. Adding another
MirBuilder-owned source root later is an explicit policy update.

## Four metrics

Use filename partition `*test*.rs`:

```text
source_files
  *.rs excluding *test*.rs

source_loc
  wc -l over source_files

test_files
  *test*.rs

test_loc
  wc -l over test_files
```

Frozen baseline:

```text
source_files = 952
source_loc   = 182452
test_files   = 139
test_loc     = 40826
```

The split is intentionally mechanical. It is not a semantic ownership
classification.

## Ratchet row

Add one compact TSV under the existing design fixtures:

```text
docs/development/current/main/design/fixtures/
mirbuilder-structural-ratchet.tsv
```

Schema:

```text
source_files	source_loc	test_files	test_loc
```

It contains one current ceiling row only.

Normal check fails if any measured value exceeds its ceiling:

```text
measured source_files > ceiling source_files
measured source_loc   > ceiling source_loc
measured test_files   > ceiling test_files
measured test_loc     > ceiling test_loc
```

At a macro-pack close, update each ceiling to:

```text
min(previous ceiling, measured value)
```

Source headroom cannot compensate for test growth, and file-count headroom
cannot compensate for LOC growth.

## Shared guard

Append one small check to:

```text
tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Requirements:

```text
no new checker executable
no Python module
no per-cell shell wrapper
no new guard mode
no path manifest
no generated report
```

The existing shared guard reads the one TSV row, runs the four measurements,
and fails on growth. Keep the added shell compact and readable.

Existing rules still apply:

```text
new per-cell shell guards = 0
all modified source/check files < 800 lines
five-cell rolling production Rust LOC <= 0
```

These prevent one-line wrapper proliferation without a new check inventory.

## What is not being built

Explicitly rejected:

```text
ObservedOwnedFootprintV1
ClassifiedOwnedFootprintV1
AcceptedStructuralEnvelopeV1
Keep / Merge / Delete / Proof ledger
open / settled obligation state
rule shards
path-set digests
external-source manifest
repository-wide check classification
Python checker and checker self-test suite
resolve / report / inventory / completion modes
precomputed final X
```

Those mechanisms manage a shrink plan rather than shrinking MirBuilder. Their
own footprint would work against the purpose of the metric.

## Completion meaning

Structural metrics are a regression guard, not product authority.

`MIRBUILDER-INPLACE-REPLACEMENT0` still closes through semantic evidence:

```text
all packs closed
old owners and selected edges = 0
fallback / retry / reselection = 0
detached production-capable routes = 0
accepted vocabulary classified
full parity green
```

The four metrics show that the implementation/proof footprint did not grow
while achieving those results.

## Implementation boundary

One commit:

```text
tools(mir): ratchet structural footprint
```

Include only:

```text
one TSV ceiling row
small shared-guard measurement/comparison
focused shell behavior check if existing guard tests provide a natural home
policy/task/current closeout
Binary D0 unpark
```

Production MirBuilder source remains unchanged. The seventh replacement
manifest row remains absent.

## Acceptance

```text
measured source_files = 952
measured source_loc   = 182452
measured test_files   = 139
measured test_loc     = 40826

all measured values <= ratchet row
ratchet rows = 1
measured roots = exactly 2
new checker executables = 0
new per-cell guards = 0
production Rust edit = 0
seventh replacement row = 0
all touched source/check files < 800 lines
```

## Gate order

```bash
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Record the four measured values in the closeout.

## Hard stops

```text
the two measured roots do not reproduce the baseline
implementation needs a new checker program or rule manifest
metric naming requires semantic file classification
shared guard reaches 800 lines
production MirBuilder or Binary source must change
seventh replacement row is added
```

## Handoff

After this one commit:

```text
minimal structural ratchet closed
-> Binary accounting D0 resumes
-> seventh Binary cell selection
```
