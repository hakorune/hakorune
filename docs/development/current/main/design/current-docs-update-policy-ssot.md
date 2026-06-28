---
Status: SSOT
Date: 2026-05-22
Scope: current docs update policy for restart/current-lane pointers.
Related:
  - AGENTS.md
  - docs/development/current/main/design/agent-current-entry-contract-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
  - tools/checks/current_state_pointer_guard.sh
---

# Current Docs Update Policy

## Problem

Small implementation cards were forcing updates to too many human-written
mirrors:

- `CURRENT_TASK.md`
- `AGENTS.md`
- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/10-Now.md`
- phase README
- taskboard / ledger
- `docs/development/current/main/CURRENT_STATE.toml`
- the active card
- `tools/checks/current_state_stale_pointer_patterns.txt` only when stale
  pointer guard fixtures change

That made card work depend on manual ledger synchronization instead of one
clear current-state owner.

## Decision

`docs/development/current/main/CURRENT_STATE.toml` is the machine-readable SSOT
for the current lane, blocker, phase pointers, and latest card pointer.

Current work is constrained to four buckets:

1. mimalloc migration and optimization
2. direct memory / DirectArray language substrate when it reduces allocator
   workaround pressure or clarifies future fast-path ownership
3. Array / representation fast paths only when selected by mimalloc perf
   evidence or by the active direct-memory substrate workstream
4. docs and shell hygiene

These buckets are the work taxonomy. Do not open a new active lane outside
them without updating this policy and `CURRENT_STATE.toml`.

Per-card mandatory docs updates are limited to:

1. `CURRENT_STATE.toml`
2. the active card
3. code/test docs only when the card changes their contract

Do not update `CURRENT_TASK.md`, `05-Restart-Quick-Resume.md`, `10-Now.md`,
`AGENTS.md`, phase README, taskboards, or ledgers for every landed card.

Update those mirrors only when one of these changes:

- active lane
- restart order
- phase status path
- durable design/update policy
- a taskboard or ledger's own stable contract
- root AI/developer instruction contract in `AGENTS.md`

Changing only `current_blocker_token`, `latest_card`, `latest_card_path`, or
`landed_tail` is a `CURRENT_STATE.toml` update. Thin mirrors should point to
those fields by name instead of repeating the concrete row token.

## Row and Guard Growth Policy

The previous phase-296x cadence produced too many docs-only rows and per-row
shell guards. New work should avoid making a row for every observation.

Rules:

- one active working card per bucket; use sections inside that card for
  inventory, selection, smoke, and closeout notes;
- do not create a new numbered row for a small inventory or a single source
  scan unless it changes implementation scope or durable policy;
- docs-only rows are allowed for durable policy, but not as a repeating
  thinking log;
- after one docs-only decision, the next step must be code, perf evidence,
  guard consolidation, or explicit closeout;
- do not create a dedicated `.sh` guard for every row;
- prefer one reusable lane guard per bucket:
  - mimalloc: source/perf/current-owner guard
  - array/fastpath: representation fastpath guard
  - docs-sh hygiene: docs/index/current-state guard
- add a new shell guard only when it will be reused or when it validates real
  code/perf behavior that cannot be covered by an existing lane guard;
- historical row guards remain callable for traceability, but new current work
  should not keep extending the per-row guard list;
- `docs/tools/check-scripts-index.md` should document stable public entries,
  not every one-off diagnostic probe;
- no fast path lane opens unless current mimalloc perf evidence names a
  concrete owner family and a positive-net implementation path.

Immediate cleanup policy for phase-296x:

- keep rows through the current card as history;
- stop adding numbered rows for inventory-only steps;
- slim `CURRENT_STATE.toml` to the last few landed cards;
- fold future notes into the active mimalloc working card or a single
  investigation note when needed;
- classify old per-row guards as legacy traceability unless they are part of a
  reusable bucket guard.

## Docs Loop Breaker Policy

Docs-first means contract-first. It does not mean docs-only iteration can keep
the active blocker open indefinitely.

After a docs-only decision, consultation summary, frontier refresh, or design
stop, the next active blocker must be one of:

- implementation or generated artifact materialization;
- executable Hako projector / verifier / guard work;
- a code-facing guard consolidation that removes duplicated expectations;
- a measured perf or smoke result that changes the selected owner;
- an explicit closeout that parks the lane.

Do not create a second consecutive docs-only card for the same blocker unless
one of these is true:

- the previous docs card changed the durable policy owner;
- new source evidence invalidated the selected implementation owner;
- a reviewer found a concrete contradiction in the acceptance contract;
- the lane is being explicitly parked or closed.

When a card exists only to document a design stop, it must also name the next
code-facing owner and a fail-fast boundary. The next card may refine the
implementation shape, but it must not repeat the same design consultation as a
new task.

If the active blocker is the explicit design-stop frontier, treat that as a
pause point for goal-driven execution: do not invent a fresh executable owner
from historical mirrors, and do not use docs-only follow-ups to keep the same
goal moving without a new frontier result.

Implementation cards should include this acceptance line when applicable:

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

Allowed documentation during an implementation card is limited to:

- the active phase card closeout;
- the owning SSOT when the contract changed;
- `CURRENT_STATE.toml` pointer fields;
- thin task-order pointer updates when the active blocker changes.

Everything else is a Ghost Task or commit-message note unless it changes a
durable contract.

## Workstream Card / Ghost Task Policy

Docs-first means contract-first, not row-first.

Use one active Workstream Card for day-to-day work in a bucket. A workstream
card may cover several days or a week. It owns the current goal, hypothesis,
checklist, short evidence, decisions, and parking lot for that bucket.

Allowed active workstream examples:

- `docs/development/current/main/workstreams/mimalloc-current.md`
- `docs/development/current/main/workstreams/direct-memory-current.md`
- `docs/development/current/main/workstreams/array-fastpath-current.md`
- `docs/development/current/main/workstreams/docs-sh-hygiene-current.md`

Use the Workstream Card for:

- inventory notes
- owner selection notes
- smoke notes
- small closeout notes
- nonkeeper notes
- parking-lot items

Do not create a new row/card/guard by default.

Create a numbered row only when at least one of these is true:

- active lane changes
- implementation boundary changes
- keeper / nonkeeper decision must be durable
- a new contract, ABI, verifier, or measurement policy is introduced
- an external reviewer or future implementer will need the decision as a stable
  historical anchor
- the decision cannot be represented by direct SSOT edit plus Workstream Card
  note

Ghost Tasks do not update `CURRENT_STATE.toml` and do not get a new row. Record
them in the commit message and, when useful, as one checklist item in the active
Workstream Card.

Ghost Task examples:

- grep / source inventory
- file-by-file checks
- small refactors
- guard wording changes
- stale pointer fixes
- existing guard condition additions
- typo / link fixes
- nonkeeper experiments that are immediately reverted

Use SSOT Direct Edit when the design truth changes. Edit the owning
`design/*.md` directly and put the reason in the commit message. Do not create
a transfer chain of investigation note -> row -> SSOT unless the decision needs
that historical anchor.

`CURRENT_STATE.toml` remains a pointer file. It may name the active workstream,
but it must not become the daily progress log.

## Current State Shape

`CURRENT_STATE.toml` should stay compact:

```toml
active_lane = "..."
active_phase = "..."
phase_status = "..."
method_anchor = "..."
taskboard = "..."
current_blocker_token = "..."

latest_card = "291x-121"
latest_card_path = "docs/.../291x-121-..."
latest_card_summary = "..."

landed_tail = [
  "last few cards only",
]
```

Full landed history belongs in phase docs and cards, not in current mirrors.
`docs/development/current/main/design/current-docs-archive-policy-ssot.md`
owns archive buckets and landed ledger policy.
Phase-local archive manifests own safe-move inventory before physical card
moves.

`landed_tail` should stay short:

```text
target maximum:
  12 rows
```

## Guard Contract

`tools/checks/current_state_pointer_guard.sh` verifies:

- required current-state scalar fields exist
- referenced repo-relative paths exist
- `latest_card_path` matches `latest_card`
- root/current/restart docs still point at `CURRENT_STATE.toml`
- thin mirrors name the `active_lane` field instead of repeating its current
  value
- thin mirrors name the `current_blocker_token` field instead of repeating its
  current value
- stale pointer patterns from
  `tools/checks/current_state_stale_pointer_patterns.txt` are absent from
  current docs

The guard must not require every current mirror to repeat latest-card history.
Past row guards must not pin `CURRENT_STATE.latest_card`,
`latest_card_path`, `current_blocker_token`, or `landed_tail` rows as proof
that the row landed. Use the row card, durable SSOT, check-script index,
fixtures, or an explicit phase-card resolver instead.

### Provenance Guard Rule

Historical row guards should stay useful after the current blocker advances.
They must validate the row's own durable evidence, not demand that the row is
still the current pointer.

Required pattern:

```text
row-owned evidence:
  card token
  fixture kind / output_contract
  durable SSOT references
  check-script index entry

current-state evidence:
  latest_card_path exists
  current token is either this row or a known follow-on row
```

Forbidden pattern:

```text
CURRENT_STATE.latest_card == this row token
CURRENT_STATE.current_blocker_token == this row token
CURRENT_STATE.landed_tail contains this row text
```

When a follow-on card advances the lane, update older row guards only if they
would otherwise false-red on current pointer drift. The update should add the
follow-on token to an explicit allow-list and must not weaken the row-owned
fixture or SSOT assertions.

## Non-Goals

- no generated-doc helper in this card
- no physical archive/move of old phase history
- no behavior or compiler changes

## Update Checklist

For a normal implementation card:

1. add/update the card
2. update `latest_card`, `latest_card_path`, `latest_card_summary`, and
   `landed_tail` in `CURRENT_STATE.toml`
3. run `bash tools/checks/current_state_pointer_guard.sh`

Only update mirrors if the card changes the active lane, restart order, phase
status path, or a durable design policy. Do not update mirrors just because the
blocker token advanced.

## Phase Row Writer Pilot

Use `tools/docs/phase_row.py` for new row boilerplate when a row needs the
usual card / current-state / short queue / check-index synchronization.

Rules:

- run the helper without `--write` first and inspect the dry-run output;
- use `--write` only for the narrow row being opened or landed;
- do not use the helper to regenerate historical phase ledgers;
- do not treat generated text as evidence that the row is complete;
- keep bespoke `.sh` guards for rows that execute builds, measurements, or
  nontrivial validation.

The intent is to move repetitive synchronization into tooling while preserving
the existing SSOT contract: `CURRENT_STATE.toml` remains the compact current
pointer, and row cards remain the human-readable decision record.

## Applied Lane Policy

Allocator provider rows M87 and later follow
`docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md`:
per-row work updates the row SSOT/card, `CURRENT_STATE.toml`, and guard wiring
when needed. Phase README, phase taskboards, and global taskboards are updated
at closeout or when their own stable contract changes, not for every row.
