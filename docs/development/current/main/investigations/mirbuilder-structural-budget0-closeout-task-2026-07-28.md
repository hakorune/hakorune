---
Status: closed; growth-failure contract superseded
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

## Later policy correction

The four measurements and compact TSV remain useful. The original automatic
failure on footprint growth and the five-cell non-positive gate are
superseded by the current in-place replacement policy. The shared guard now
reports current values against the baseline; semantic authority, fallback
zero, old-edge deletion, parity, and the 800-line source/check boundary remain
the hard gates.

## Closeout

Landed evidence:

```text
source_files = 952
source_loc   = 182452
test_files   = 139
test_loc     = 40826

ratchet rows              = 1
measured roots            = 2
new checker executables   = 0
new per-cell guards       = 0
production Rust edits     = 0
seventh replacement rows  = 0
shared guard              = green
```

The accepted Binary Option A task is now active.

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

## Baseline row

Add one compact TSV under the existing design fixtures:

```text
docs/development/current/main/design/fixtures/
mirbuilder-structural-ratchet.tsv
```

Schema:

```text
source_files	source_loc	test_files	test_loc
```

It contains one current baseline row only.

The shared guard reports each measured value and its signed baseline delta:

```text
measured source_files - baseline source_files
measured source_loc   - baseline source_loc
measured test_files   - baseline test_files
measured test_loc     - baseline test_loc
```

Pack close or an explicit structural review may update the baseline to the
current values. No dimension acts as implementation permission.

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
and reports their deltas. Keep the shell compact and readable.

Existing rules still apply:

```text
new per-cell shell guards = 0
all modified source/check files < 800 lines
five-cell rolling production Rust LOC = historical trend only
```

The hard rules prevent one-line wrapper proliferation without making total LOC
an architecture selector.

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

The four metrics record the implementation/proof footprint observed with those
results.

## Implementation boundary

One commit:

```text
tools(mir): ratchet structural footprint
```

Include only:

```text
one TSV baseline row
small shared-guard measurement/report
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
minimal structural observation closed
-> accepted Binary Option A task activates
-> BINARY-SOURCE-PARTITION-CUTOVER0-I0-R0
```
