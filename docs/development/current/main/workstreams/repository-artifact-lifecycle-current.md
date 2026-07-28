---
Status: Active bounded interlude at R3
Date: 2026-07-28
Decision: Restore `current` as an authority surface and stop one-decision-one-file growth
Owner: ../design/current-docs-archive-policy-ssot.md
Current lane: follow ../CURRENT_STATE.toml
---

# Repository Artifact Lifecycle Current

## Decision

`docs/development/current/main` is for active authority and navigation, not for
everything reachable from current text.

The selected recovery has two independent duties:

```text
stop new narrative-file growth
move already-historical phase material out of current
```

File and line counts are measured results. They are not implementation
permission gates and do not replace reference-closure checks.

`CURRENT_STATE.toml` now selects R3. This bounded activation runs through R3,
one first R4 batch, and `DOCS-MEANING-RECOVERY-RETURN0`. It then returns
to the MirBuilder workstream at
`NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0`; it does not jump directly to
Candidate A. The returned MirBuilder train closes the general Program verified
owner and current-normal result parity, then passes the parked Ownership/View
readiness train before any default-ingress cutover.

R2 closed resolver/relocator support for the live, global archive, and
transitional nested-archive locations. R3 now authorizes only the exact
two-file pilot below. Compiler/runtime/backend edits and a tenth replacement
row remain forbidden throughout this interlude.

## Scheduled first activation

The first activation is deliberately smaller than the complete R1-R6 backlog:

```text
R1
-> R2
-> R3
-> first R4 batch
-> DOCS-MEANING-RECOVERY-RETURN0
-> NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
```

Exact boundary:

```text
R1  restore existing lifecycle gates
R2  close global/transitional archive resolution
R3  move the exact two-file pilot
R4  move one bounded reference-closed nested-archive batch

RETURN0:
  strict lifecycle inventory green
  pointer guard green
  reference/link closure green
  current-doc counts recorded
  worktree clean
  CURRENT_STATE returned to MirBuilder
```

R5 stale-phase cohorts and R6 design/investigation retirement remain scheduled
cleanup debt. They are not prerequisites for the first return to compiler
work.

The global queue after RETURN0 is authoritative in
`mirbuilder-inplace-replacement-current.md`:

```text
general Program verified owner
-> function-plan families
-> verified aggregate / DraftSeal / atomic publication
-> current-normal result parity
-> Candidate A technical readiness audit
-> Ownership/View Pack A-E + product readiness
-> Candidate A final re-evaluation
-> atomic default-ingress cutover
```

R5/R6 may run only at a later explicitly selected clean-worktree milestone.
They do not silently become the current blocker when RETURN0 closes.

## Measured baseline

Physical Markdown census after the 2026-07-28 Ownership grammar task:

```text
docs, excluding docs/private       10,955 files
docs/development                    9,858 files
docs/development/current/main       7,092 files / 779,724 LOC

current/main/phases                 5,631 files / 465,956 LOC
current/main/design                   846 files / 143,596 LOC
current/main/investigations           439 files / 162,132 LOC
current/main/workstreams                9 files /   5,397 LOC
current/main root                     166 files /   2,568 LOC

nested phase archives              2,614 files / 231,629 LOC
phase-296x/archive                  2,517 files / 207,500 LOC
```

Tracked Markdown, excluding `docs/private`, is 10,949 files / 1,191,196 LF
lines. The physical/tracked difference is six ignored or untracked historical
entry files. The recovery gates use tracked paths; the physical census remains
an observation.

The current investigation growth is concentrated in July:

```text
investigations total                 439 files / 162,132 LOC
created in 2026-07 by git date       338 files / 134,326 LOC

consultation                          61 files
design-stop                           25
execution-task                        86
task-map                               7
filename containing `task`           176
```

The repeated shape is:

```text
question
-> consultation
-> design stop
-> task map
-> execution task
-> closeout or mirror
```

Those are normally states of one workstream, not six independent document
owners.

## Landed lifecycle summary

Prior H0-H2 work already:

```text
archived 1,092 unreferenced phase-296x direct cards in place
moved 285 inactive phase directories to the global development archive
moved 2,351 unreachable partial-phase files
drained mechanically safe whole-phase and partial-phase candidates to zero
```

Therefore age alone does not authorize another phase move. The remaining
90-day-old phase directories require current backlink adjudication or an
atomic path rewrite.

Git history owns the detailed batch ledger. It is intentionally not repeated
in this rolling card.

## R1 closeout

The substrate repair established:

```text
design registry:
  mirbuilder-final-pipeline-ssot.md classified as authority
  mirbuilder-inplace-replacement-policy-ssot.md classified as authority
  unregistered baseline remains 77

repository lifecycle strict check = green
DOCS-SLIM-001                    = green
DOCS-SLIM-002                    = green

phase-293x archive entry:
  canonical path = docs/development/archive/phases/phase-293x/archive/README.md
  duplicate current-tree restoration = forbidden
```

Restoring the masked executable entries exposed two later-history contract
drifts:

```text
DOCS-SLIM-003:
  its repository-wide current-pointer-pin scan now includes 201 historical,
  non-executable row guards

DOCS-SLIM-026:
  its 19 converted historical guards are structurally resolver-aware, but
  executing every old guard also reasserts unrelated retired source/dev-gate
  snapshots
```

R1 resolved those drifts without reviving historical guards:

```text
DOCS-SLIM-003:
  original explicit ten-guard regression set = retained and green
  later historical row guards                = not current authority

DOCS-SLIM-026:
  shared helper presence across 19 scripts = exact
  raw resolver-leak assertions             = 0
  bash syntax                              = green
  unrelated historical guard execution    = not an R1 requirement

physical archive move = 0
```

## R2 closeout

The one existing phase-card resolver now recognizes:

```text
live phase path
global development archive path
transitional nested archive path
```

It enumerates every candidate in this order:

```text
live
-> global archive
-> transitional nested archive
```

The resolver classifies a file as a forwarding stub only when it has both a
`Moved to:` row and a `# Moved` heading. A stub is not authority. Exactly one
full copy resolves; zero is missing; more than one fails with every full path.
The phase-293x bucket calculation now accepts arbitrary numeric card prefixes,
including four-digit cards.

Both existing relocators reuse one source-layout-to-canonical-target function:

```text
live phase path
top-level transitional phase path
phase-local nested archive path
-> docs/development/archive/phases/<phase>/...
```

For phase-296x nested cards, the exact target is:

```text
docs/development/archive/phases/phase-296x/cards/<filename>
```

Duplicate targets and every occupied destination, including a forwarding stub,
fail during dry-run. The generated lifecycle manifest uses the same target
normalization.

Closeout evidence:

```text
strict lifecycle inventory                 = green
whole-phase relocator dry-run              = green, 0 files
partial-phase relocator dry-run            = green, 0 files
DOCS-SLIM-001/002/003/026                  = green
current-state pointer guard                = green
physical Markdown move                     = 0
new checker/test file                       = 0
modified source/check files >= 800 lines   = 0
```

## Current R3 task

Move only the exact pilot named below. Do not select a generated R4 batch in
the same commit.

## Final archive shape

Historical development phase material converges on:

```text
docs/development/archive/phases/<phase>/...
```

Existing current-local archive roots are transitional:

```text
docs/development/current/main/phases/archive/
docs/development/current/main/phases/<phase>/archive/
```

They are drained only by reference-closed bounded moves. The general
`docs/archive/` tree remains a separate repository-wide archive and is not a
substitute destination for development phase history.

### Current authority roots

Current status is granted only by:

```text
CURRENT_STATE.toml path fields
thin fixed restart entries
active workstream cards
active card / phase status
registered live design and reference authorities
explicit stable external entrypoints
```

A historical document linking another historical document creates a move
cluster. It does not make both documents current forever.

`src/`, `tools/`, or another tracked document referencing a historical path is
not by itself a move prohibition. The batch must either rewrite that reference
atomically or retain the exact stable entrypoint under the stub law below.

## Workstream one-card law

Each active workstream owns exactly one rolling current card:

```text
docs/development/current/main/workstreams/<workstream>-current.md
```

These are states inside that card, not reasons for new files:

```text
consultation
design stop
selection
execution
gate result
closeout
recount
next row
```

The rolling card retains only:

```text
active decision
exact task
gates
hard stops
short landed ledger: row / commit / measured result
```

Git history owns replaced prose and detailed landed chronology.

The default investigation-file delta for an ordinary row or cell is zero.
External AI advice is distilled into decision, evidence, rejected alternatives,
acceptance, and hard stops; the full answer is not copied into a new current
file.

### New narrative-file exceptions

A new narrative document requires one of:

```text
durable cross-workstream normative contract
machine-consumed stable artifact or schema
irreproducible evidence/reproduction that must be retained
security, legal, release, or incident audit
genuine workstream fork with an independent owner and lifecycle
reference shard that remains independent after aggressive compaction
```

For investigations, the document must name:

```text
Exception:
ParentCurrentCard:
```

Another AI consultation, a status transition, a checklist, or a landed commit
is not an exception.

## Stub law

```text
rewritable repository-internal reference:
  rewrite it and leave no stub

stable public entry or unrewritable external consumer:
  leave one compact forwarding stub

whole phase needing a historical entry:
  at most one phase README stub

current pointer target:
  do not move and do not replace with a stub
```

Per-card stubs are forbidden by default. A stub is a compatibility surface,
not an archival receipt.

## Execution train

### R0 — growth stop (closed)

Update the current docs policy and layout so the one-card law and exception
fields are explicit. Reuse an existing docs/current guard; do not create a new
shell wrapper or a per-workstream manifest.

Acceptance:

```text
one active current card per workstream
ordinary row/cell investigation-file delta = 0
new investigation without named exception = rejected
current pointer semantics unchanged
```

Closeout:

```text
commit = 5d71ff9d61
new rolling workstream card = 0
ordinary row/cell investigation-file delta rule = 0
physical archive move = 0
```

### R1 — archive substrate recovery

Restore the existing lifecycle machinery before moving anything:

```text
adjudicate the 79 vs 77 design-registry drift
restore required executable modes
restore or deliberately retire the missing phase-293x archive entry
make strict inventory green
make DOCS-SLIM-001/002/003/026 green
```

Do not hide the design-registry drift by merely increasing a numeric baseline.
Classify the exact new files or explicitly accept a reviewed new baseline.

### R2 — global phase resolver

Extend the existing phase-card resolver and relocation tools to recognize:

```text
live phase path
transitional nested phase archive
global development phase archive
```

Target lookup order:

```text
live
-> global archive
-> transitional nested archive until drained
```

The lifecycle inventory must distinguish:

```text
active authority root
rewritable inbound reference
historical move-cluster edge
stable external entrypoint
```

Do not add SHA-256 path ledgers, per-file disposition tables, or a second
archive checker.

### R3 — phase-296x nested-archive pilot

After R1/R2 are green, the first bounded physical proof is exactly two closed,
path-unreferenced cards:

```text
1776-MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROMOTION-DECISION-001.md
1777-MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-SHADOW-PROMOTION-DECISION-001.md
```

Pilot facts:

```text
source:
  docs/development/current/main/phases/phase-296x/archive/

target:
  docs/development/archive/phases/phase-296x/cards/

files:
  2

LOC:
  199

tracked inbound path references:
  0

forwarding stubs:
  0
```

The pilot updates the existing generated lifecycle manifest in the same
commit. If the global partial-phase destination is not accepted by the
resolver dry-run, the move count remains zero.

### R4 — nested archive batches

After the pilot, select candidates from the generated unreachable set, not
from filename ranges or age.

```text
normal batch maximum = 200 files
never split a weakly connected historical cluster
cluster larger than 200 = one dedicated reviewed batch
destination collision = 0
reachable incoming edge = 0 or atomically rewritten
per-card stub = 0
```

Phase-296x has 1,597 currently unreachable archived-in-place documents, but
that number is evidence, not blanket movement permission.

### R5 — stale phase cohorts

The 2026-07-28 age census found:

```text
immediate phase directories             = 108
last path touch older than 90 days       = 59
archive-ready after atomic reference rewrite = 20
retain pending backlink adjudication     = 39
```

The first review cohort is:

```text
phase-29z
phase-268
phase-275
phase-29aw
phase-29bh
phase-29bi
phase-29bj
phase-29bk
phase-29bn
phase-29bo
phase-29br
phase-29cd
phase-29ce
phase-29cf
phase-29cj
phase-29cl
phase-29co
phase-29cp
phase-96x
phase-290x
```

The 90-day threshold only selects review candidates. Each cohort still needs
active-root exclusion, exact reference rewrite, collision checks, and strict
green before `git mv`.

### R6 — design and investigation retirement

Only after phase relocation is routine:

```text
finish design authority classification
archive superseded design clusters with exact inbound-reference closure
archive historical investigation clusters
retain one rolling workstream card
retire obsolete DOCS-SLIM ordinal guards and numeric-history assertions
keep one durable artifact-lifecycle guard
```

The actual design registry is `design/INDEX.md`. Policy and tooling must agree
on that path before any design mass move.

## Commit train

```text
docs: stop current narrative file growth
tools(docs): restore artifact lifecycle gates
tools(docs): resolve global phase archives
docs: relocate first phase archive pilot
docs: relocate bounded phase archive batch
docs: archive reviewed stale phase cohort
tools(docs): consolidate artifact lifecycle guards
```

Do not mix a physical move with an unrelated language, compiler, runtime,
backend, or ownership change.

## Batch gates

Preflight:

```bash
git status --short
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
python3 tools/docs/archive_unreachable_phase_clusters.py
python3 tools/docs/archive_unreachable_partial_phase_clusters.py --max-files 200
```

After every physical batch:

```bash
python3 tools/docs/repository_artifact_lifecycle_inventory.py --check --strict
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/docs_slim_001_archive_policy_guard.sh
bash tools/checks/docs_slim_002_archive_manifest_guard.sh
bash tools/checks/docs_slim_003_guard_reference_decoupling_guard.sh
bash tools/checks/docs_slim_026_phase_card_resolver_leak_helper_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```

The relocator's preserved-link and old-path checks are the reference-closure
authority. Do not claim a separate generic Markdown link checker exists.

## Completion

This workstream closes only when:

```text
current pointer targets remain live
current authority roots are explicit and finite
nested phase archives are drained or own an exact retained exception
reviewed stale phase cohorts live under the global development archive
one rolling card owns each active workstream
ordinary status transitions create zero new investigation files
design and investigation authorities are classified
archive lifecycle guard count is consolidated
unresolved tracked references = 0
physical file and LOC deltas are reported
```

There is no final arbitrary file-count or LOC cap. A smaller `current` tree is
the result of restoring the authority boundary.

## Hard stops

```text
dirty worktree before a physical batch
current pointer target selected for movement
strict inventory red
unresolved tracked reference
archive target collision
age used as sole movement authority
historical backlink treated as permanent current authority
per-card forwarding-stub multiplication
new file-per-decision ceremony
new archive checker or path-digest ledger
production source or language behavior mixed into a docs move
```
