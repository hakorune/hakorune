---
Status: accepted policy task
Date: 2026-07-28
Decision: MIRBUILDER-STRUCTURAL-BUDGET-D0
Scope: define the absolute structural completion envelope for MIRBUILDER-INPLACE-REPLACEMENT0
Parent:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/investigations/binary-source-partition-cell-accounting-d0-consultation-2026-07-28.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
---

# MirBuilder structural budget D0

## Decision

`MIRBUILDER-INPLACE-REPLACEMENT0` completion requires both:

```text
semantic completion
AND
absolute structural completion
```

Pack closure, old-edge zero, and parity green are necessary but no longer
sufficient. Total files and LOC owned by MirBuilder must also fit a
precommitted final envelope.

The final numeric envelope is not guessed in this card. D0 first classifies
the complete owned footprint as `Keep`, `Merge`, `Delete`, or `Proof`, then
fixes `X_files`, `X_builder_loc`, and `X_check_loc`. No production replacement
cell resumes before those caps and their measurement command are accepted.

## Why this is required

The five-cell rolling production Rust budget prevents recent implementation
growth, but it forgets older additions when they leave the window. It also
does not charge per-cell parity fixtures or check scripts when they live
outside the measured production surface.

Therefore this graph can satisfy the current law while remaining large:

```text
new owner
+ three parity fixtures
+ private proof helper
+ shared-guard assertions
- old production facade
```

A small, strong language needs the retained proof and orchestration footprint
to be part of completion accounting.

## Frozen observation baseline

Baseline commit:

```text
f0256073d5 docs(mir): ask binary cell accounting boundary
```

Canonical builder observation:

```bash
find src/mir/builder -type f -name '*.rs' | wc -l
find src/mir/builder -type f -name '*.rs' -print0 \
  | xargs -0 wc -l | tail -n 1
```

Observed:

```text
builder Rust files                         = 1,081
builder total Rust LOC                     = 221,957

coarse *_tests.rs files                    = 133
coarse *_tests.rs LOC                      = 39,806
```

The `*_tests.rs` observation is not the final Proof classification. Inline
tests, raw/parity files with other names, and non-test code in mixed files must
be classified explicitly.

Until final X is accepted, the baseline is a closed-cell high-water ceiling:

```text
builder Rust files <= 1,081
builder total LOC  <= 221,957
```

Raising either ceiling requires a T2 policy decision. Falling below it creates
headroom but does not by itself define final completion.

## Owned footprint

### A. Builder tree

Every Rust file below:

```text
src/mir/builder/**/*.rs
```

is counted, including:

```text
production implementation
cfg(test) fixture modules
raw production-ingress fixtures
historical parity references
snapshot helpers
test-only compatibility owners
```

No test filename or `cfg(test)` exclusion is allowed in the total LOC number.

### B. MirBuilder-owned checks

Every structural/proof script whose primary contract protects
`src/mir/builder` must be listed in the D0 structural manifest and counted.

This includes:

```text
private per-family Python helpers
shared MirBuilder replacement guard
manifest runners or data used only by this workstream
```

Repository-wide generic guard infrastructure is counted only if D0 classifies
it as MirBuilder-owned. A filename pattern is not authority; the manifest is.

### C. Navigation docs

Task history and investigations are not added to the Rust/check LOC scalar.
Current navigation docs remain subject to compact-pointer and 800-line rules.
Duplicate current authority must still be merged or retired.

## Classification

Every owned source/check file receives exactly one disposition:

```text
Keep
  final semantic owner or irreducible shared infrastructure

Merge
  useful behavior/proof whose separate file or duplicated shell is unnecessary

Delete
  retired authority, unused facade, stale route, duplicated guard, or obsolete
  fixture

Proof
  minimum retained test/parity/guard evidence with no production authority
```

Required columns:

```text
path
kind = source | check
pack
disposition
semantic_owner_or_proof_contract
current_loc
target_path_or_delete_condition
```

Unclassified files prevent completion.

## Final caps

D0 computes:

```text
X_files
  final src/mir/builder Rust file count

X_builder_loc
  final total LOC of every src/mir/builder Rust file

X_check_loc
  final total LOC of the manifest-listed MirBuilder checks
```

The caps derive from the classified keep-set:

```text
Keep target LOC
+ Merge target LOC after consolidation
+ minimum Proof target LOC
= accepted final cap
```

Do not derive X from a percentage chosen for appearance. Do not include
unclassified “temporary” headroom. If implementation evidence later proves a
cap impossible, reopen D0/T2 and explain the new irreducible owner; do not
silently move X.

## Ratchet law

Each replacement closeout records:

```text
builder Rust files before / after / delta
builder total Rust LOC before / after / delta
MirBuilder-owned check LOC before / after / delta
production Rust LOC delta
five-cell rolling production Rust LOC
```

The total builder/check numbers charge fixture and guard additions even when
production LOC is negative.

At each macro-pack close:

```text
new pack ceiling = min(previous ceiling, measured owned footprint)
```

The ceiling never rises without T2. Between pack closures, a cell may use
headroom created by prior deletion, but its closed state may not exceed the
current high-water ceiling or violate the five-cell production budget.

## Proof preservation

The structural cap must not be met by deleting evidence blindly.

Semantic completion still requires:

```text
focused production-ingress behavior
historical parity where it protects a live replacement
failure and same-Builder reuse
shared owner/edge guard
full accepted corpus/backend parity
```

Prefer:

```text
one real ingress fixture over one facade fixture
one parameterized parity table over repeated family files
one shared guard over per-cell wrappers
one snapshot surface over duplicated snapshot structs
```

Proof deletion is accepted only when the surviving proof directly covers the
same contract.

## Cell-selection consequence

Before choosing a cell, compare:

```text
old authority removed
new production code added
new Proof LOC retained
new check LOC retained
file count delta
```

“Negative production LOC” alone is no longer sufficient.

For the pending Binary decision, a cell that reuses existing raw/parity suites
and deletes the dead predecessor chain has lower owned cost than three
accounting cells with orphan stages or duplicated proof updates. This is
evidence for the Binary D0, not an automatic selection; the source-partition
versus semantic-owner accounting law remains explicit.

## Deliverables

```text
1. structural manifest covering all owned source/check files
2. Keep/Merge/Delete/Proof totals by macro pack
3. accepted X_files
4. accepted X_builder_loc
5. accepted X_check_loc
6. one stable measurement/check entry
7. workstream dashboard baseline and remaining structural debt
8. Binary D0 re-evaluation under the accepted envelope
```

## Hard stops

```text
do not choose X by aesthetics alone
do not exclude tests from builder total LOC
do not hide proof cost in tools/checks
do not delete parity without equivalent surviving evidence
do not merge files past the 800-line boundary
do not compress multiple semantic owners into one mixed module merely to lower
file count
do not resume Binary or another production cell before final caps are accepted
```

## Non-claims

```text
no source, test, guard, or manifest implementation change in this D0 card
no claim that 1,081 files or 221,957 LOC is an acceptable final size
no language, runtime, backend, ownership, or selfhost scope change
no seventh production cell selection
```
