---
Status: SSOT
Date: 2026-08-08
Scope: current docs update policy for restart/current-lane pointers.
Related:
  - AGENTS.md
  - docs/development/current/main/design/agent-current-entry-contract-ssot.md
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
  - docs/development/current/main/design/current-docs-archive-policy-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
  - tools/checks/current_state_pointer_guard.sh
---

# Current Docs Update Policy

## Current Capsule

- **Current decision:** `CURRENT_STATE.toml` is the compact current pointer;
  active cards own execution and git history owns landed detail.
- **Current implementation status:** the pointer/mirror thinning policy and
  classified-red procedure are active; this policy update does not change the
  selected compiler lane.
- **Next ordered task:** apply the compact pointer, shallow task-name,
  semantic-boundary, and parent-baseline rules to the next selected row.
- **Production stop line:** documentation and guard cleanup cannot activate a
  production route or waive a failing current contract.
- **Retirement finish line:** current mirrors contain no copied history,
  `landed_tail` stays bounded, and durable rules have one tracked owner.

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

Current work is constrained to the active lane named by `CURRENT_STATE.toml`
and the durable workstream card it points to. The current active buckets are:

1. MirBuilder in-place responsibility replacement when selected by
   `CURRENT_STATE.toml`;
2. Language v1 semantic convergence when explicitly reopened;
3. Source Selfhost / MirBuilder Rust-to-Hako converter task order when
   explicitly reopened;
4. mimalloc migration and optimization when reopened by `CURRENT_STATE.toml`;
5. direct memory / DirectArray language substrate when it reduces allocator
   workaround pressure or clarifies future fast-path ownership;
6. Array / representation fast paths only when selected by current perf
   evidence or by the active direct-memory substrate workstream;
7. docs and shell hygiene.

These buckets are the work taxonomy. Do not open a new active lane outside
them without updating this policy and `CURRENT_STATE.toml`.

### Operational mode rule

To keep the rules usable, every active turn is classified as exactly one of
three modes:

```text
Fast path    = closed mapping; reuse one owner; edit/test/close
Design stop  = open mapping or authority; brief only; no code/fixture/fallback
Closeout     = classify evidence; update owner docs; commit/push or retain blocker
```

The mode is selected before editing by the explicit
`CURRENT_STATE.toml.work_mode` scalar (`fast`, `design_stop`, or `closeout`).
`current_blocker_token` and `current_design_stop` are explanatory pointers,
not classifiers. A worker review, local test, or legacy parity result is
evidence for the mode decision, not permission to cross it. This prevents a
small green probe from silently becoming a production claim.

### Source-backed semantic receipt gate

When a row proposes a new `Verified*` or `Prepared*` semantic product, its
design brief must name the source authority and the canonical issuer before
implementation. If a required receiver/Home, effect, lifecycle,
suspension/control, or ABI receipt has no source-backed issuer, the row stays
in `design_stop` as `NoSafeSlice`; do not fill the gap with an empty/default
receipt, body inference, a name lookup, or a physical MIR projection.

Aggregates may co-seal receipts already issued by their owners, but they may
not become a second semantic authority. In particular, language-semantic
`Pure`/Home/effect/ABI facts do not use MIR `EffectMask` or
`FunctionSignature` as their source of truth. `NoSafeSlice` is a development
state, not a `Candidate`/`Declined`/`Unresolved`/`Rejected` disposition. A
parked I0 must explicitly choose either the missing issuer slice or an
intentional park, and implementation slices must update the affected module
README and `docs/reference/**` in the same closeout.

Per-card mandatory docs updates are limited to:

1. `CURRENT_STATE.toml`
2. the active card
3. owning `docs/reference/**`, module README, or code/test docs only when the
   card changes their contract; close those updates in the implementation
   slice rather than an unnamed later docs task

Changes to the ignored root `AGENTS.md` router are policy-level changes, not
per-card mirror work. When the router is reorganized, update
`agent-current-entry-contract-ssot.md` in the same slice and keep the local
file limited to short routing rules; durable procedure and design prose stays
in its owning tracked document.

### Practical mode check

The mode check is intentionally finite and must happen before editing:

```text
work_mode = "design_stop"?
  -> Design stop
source -> Facts -> Recipe -> failure boundary is one sentence?
  no -> Design stop
  yes -> choose exactly one: BoxCount or BoxShape
```

`NoSafeSlice` means that the current design cannot yet issue a safe product;
it is not a source disposition and must not be relabeled as a convenient
`Declined`/`Unresolved` result. Do not add guessed operation counts, a route
specific adapter, or a green fixture to cross this boundary. The next card
must either close the missing schema/authority or explicitly park the family.

This is a routing aid, not a new task hierarchy. The active lane and blocker
remain owned by `CURRENT_STATE.toml`, and detailed execution remains in the
rolling workstream card.

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

Backend production/reference/nonconsumer roles are live selection state. They
belong in `CURRENT_STATE.toml` and the active card, not as a fixed VM/AOT/JIT
priority in `AGENTS.md` or another restart mirror. The root router keeps only
the durable rule that existing backend code and tests do not authorize parity
or production work for an unselected backend.

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

### Workstream One-Card Law

For ordinary workstream progress, consultation, design stop, selection,
execution, recount, and closeout are states inside the existing rolling
workstream card. They are not separate document types.

```text
default investigation-file delta per row/cell = 0
landed detail authority = git history
rolling-card landed record = row / commit / measured result
```

External AI advice is distilled into the active decision, evidence, rejected
alternatives, acceptance, and hard stops. Do not preserve the full answer as a
new current document.

### Minimal Execution Brief Law

An accepted row is a one-screen implementation contract. It is not the design
consultation, an implementation forecast, or a second test specification.

Keep only four blocks:

```text
Change:
  exact transition and any old authority deleted atomically

Contract:
  one responsibility plus the semantics/owners that must not move

Done:
  focused observable result and one stable gate entry

Stop:
  conditions that return the row to design
```

The row token, parent, and ceremony tier may be one metadata line above the
four blocks. Do not create empty sections merely to satisfy the shape.

Rules:

- candidate comparisons end when D0 closes; keep only the selected decision
  and, when needed, one sentence explaining the decisive boundary;
- routing, claim, publication, admission, and lifecycle cards must include the
  finite state/transition inventory required by the tracked
  `agent-current-entry-contract-ssot.md` classification-completeness check;
  the inventory is the compact exception to the prose budget, not a reason to
  merge `Absent`/`Unavailable`/`Unresolved`/`NoCandidate` into `None` or a
  compatibility label;
- do not copy full type definitions, fixture bodies, exhaustive rejection
  tables, file-by-file edit plans, guard string assertions, generic gate
  commands, or expected LOC ranges into an execution brief;
- an S0 with no old authority says `old authority: none` in `Change`; it does
  not need a separate atomic-delete section;
- ceremony is a routing label, not a requirement to write more prose;
- active execution detail should normally fit in about 20–40 lines. This is an
  editorial target, not a guard or implementation permission gate;
- T2 gets a separate durable design SSOT only when the decision remains useful
  across rows or workstreams. Otherwise the rolling card holds the compact
  decision directly;
- closeout is one compact record:
  `row / commit / result / focused evidence / material structural delta /
  next blocker`;
- omit structural delta when it is immaterial. Record measured values, never a
  pre-implementation LOC promise;
- git history owns the detailed landed diff. Source, tests, and the reusable
  guard own executable detail. Later rows must not copy it back into docs.

### Implementation-Coupled Commit Law

For a bounded row whose design is already determined by code authority, keep
selection, implementation, and closeout in one implementation commit.

```text
default bounded row:
  select in rolling card -> edit/test -> compact closeout = one refactor commit

separate docs commit only for:
  external consultation, language-spec Decision, NoSafeSlice/NoStandaloneRow,
  or a Refactor Series contract that must precede multiple code commits
```

Do not create `task -> implementation -> closeout` as three commits for each
AST kind. Run a fresh census at a responsibility-batch boundary, not
automatically after every small constructor. Proof and old-edge deletion remain
mandatory; only repeated prose ceremony is removed.

For a green bounded implementation slice, commit and push at the slice
boundary before starting another design or implementation slice, unless the
user explicitly requests a local-only checkpoint. A push is a delivery
boundary, not evidence that a production route is active; the active pointer
and closeout evidence still decide that claim.

### Responsibility-Family Amortization Law

Documentation is amortized over a semantic responsibility family, not emitted
once per AST constructor. A family is a reusable authority boundary such as a
compositional expression closure, control, call/object, nested-owner, or
compatibility-retirement boundary.

```text
one family decision
  -> one or more implementation-coupled constructor edits
  -> one batch-boundary census / compact closeout
```

Rules:

- adding another constructor to an already accepted family does not create a
  task document, selection document, or closeout document;
- record such an edit as one compact table/queue mutation in the implementation
  commit; git history and the executable proof remain the detailed record;
- do not reopen D0 for each constructor unless source authority, failure owner,
  result policy, language semantics, or the family invariant changes;
- three consecutive same-family constructor rows are a mandatory batching
  trigger: stop creating per-constructor rows and use one family-level batch;
- batch selection must still name the production caller and the old authority
  removed; batching never permits mixed semantic responsibilities or fallback;
- run a fresh broad census once at the family/batch boundary, not after each
  constructor.

### Active-Card Budget and No-Growth Law

An active rolling card is a restart surface, not an append-only ledger. Active
rolling cards have an editorial target of about 800 lines and the existing
current-state guard enforces a hard 1,000-line limit.

```text
rolling card <= 1,000 lines:
  ordinary compact updates allowed

rolling card > 1,000 lines:
  replacement/compaction edits only
  net landed-history growth forbidden
```

An over-budget card may only replace its current four-block brief or current
pointer in place while shrinking overall. Before the next batch closeout, move or delete superseded
landed prose using the bounded archive policy and keep only:

- current decision / consultation;
- current four-block execution brief;
- compact active queue;
- active sunset/fence registry;
- a short closed tail.

Do not evade this law by creating a second `*-current.md`, a per-constructor
investigation file, or a new ledger that merely copies the rolling card.

A new narrative document is allowed only for a durable cross-workstream
contract, machine-consumed stable artifact, irreproducible evidence, incident
audit, genuine independently owned workstream fork, or independently readable
reference shard. A new investigation document must state:

```text
Exception:
ParentCurrentCard:
```

A status transition, another consultation, a checklist, or a landed commit is
not an exception.

## Ceremony Calibration and Proof Sunset Policy

The cost of a row is approximately its ceremony cost multiplied by the number
of route/stage cells. The project must reduce repeated cells without weakening
source authority, fail-fast, or evidence requirements.

### MirBuilder in-place replacement override

While `MIRBUILDER-INPLACE-REPLACEMENT0` is active, the durable architecture
decision is made once in
`design/mirbuilder-inplace-replacement-policy-ssot.md`.

Moving already-existing behavior into a new responsibility owner is T0 by
default. It does not open a new consultation, route family, or S0/I0/P0/G0
card chain.

```text
normal T0 cell:
  one atomic production switch + old-path deletion
  one focused fixture
  one shared pack guard assertion

split T0 cell:
  at most one S0 commit
  immediately followed by I0/R0
  no intervening row
```

`I0`, `P0`, `G0`, and `CUT0` use the exact meanings in the in-place
replacement policy. In particular, a disconnected candidate or a route with
`production consumers = 0` cannot close I0 or CUT0.

The batch-proof trigger may genericize transport shared by several live
responsibility owners. It may not justify building Raw/Normal/Canonical/Legacy
parallel production routes.

Use the smallest ceremony tier that matches the novelty of the cell:

- **T0 mechanical cell**: an already-selected owner-chain pattern is applied
  to another route with no new authority, identity, failure stage, or policy.
  Do not open a full consultation. Record the reused template, the
  route-specific delta, one focused fixture, an existing batch/lane guard
  assertion, and the sunset reference. A new per-cell guard is not allowed.
- **T1 bounded extension**: an existing owner or policy gains a new field or
  route-specific witness. Use a short design note and focused acceptance
  evidence. Escalate to T2 if the extension changes who owns truth or failure.
- **T2 new authority**: a new identity issuer, source authority, physical
  owner, publication terminal, failure owner, or policy boundary. Require a
  full design-stop brief before implementation.

The token suffix does not select the tier. Classify the actual boundary:

```text
T0  source/Facts/Recipe, accepted shape, issuer, failure terminal,
    identity/ABI/schema, and production route are unchanged; a complete
    caller/cfg census proves a private mechanical move only
T1  an existing owner gains one bounded field or witness while truth and
    failure ownership stay fixed
T2  any authority, identity, accepted shape, ABI/schema, publication,
    failure stage, lifetime/concurrency/rollback rule changes or is unknown
```

A one-line accessor can therefore be T2 when its constructor or foreign-ID
surface is unclassified, while an `I0`-named private move can be T0. Do not
infer ceremony from `R0`/`S0`/`I0` spelling.

When the same owner-chain pattern appears in two routes, the next repeated
route is a **batch-proof trigger**. Before adding a third hand-expanded cell,
define one generic proof parameterized by a route specification. Preserve
route-specific semantic witnesses; genericize transport and lifecycle only.
Do not use a generic wrapper to hide a real policy difference.

Every disconnected proof, parity fixture, compatibility adapter, or temporary
guard must carry a sunset record with all four fields:

```text
sunset_id
owner of the retirement decision
retirement condition (normally production caller count = 0)
target row/card and evidence required for deletion
```

“Delete later” without these fields is not a retirement plan. A proof-only
addition may be accepted before its deletion row lands when production safety
requires the scaffold, but the active card must reserve that deletion row and
state the zero-consumer evidence. Repeated net additions with no retired
scaffold are a cleanup/design-review trigger, not a reason to relax the
contract.

A durable T0 refactor that adds no disconnected or temporary proof records
uses `sunset = n/a` and `net_proof_delta = 0`; it does not invent placeholder
retirement metadata. The full sunset record above is mandatory only when a
temporary/disconnected proof, adapter, fixture, or guard is added or retained.

When a row changes temporary proof scaffolding, record these fields in the
existing reusable proof inventory or guard metadata:

```text
ceremony_tier
sunset_id
proof_inventory_before
new_proofs
retired_or_merged_proofs
net_proof_delta
sunset_budget
sunset_row
retire_when
budget_repayment_evidence
```

The execution brief names only the `sunset_id` and material retirement
condition; it does not duplicate the inventory. The default target is
`net_proof_delta <= 0`. A positive delta is allowed only
for a T2 safety/ABI boundary and must name the sunset budget, repayment row,
retirement condition, and evidence that will repay it. Mechanical cells may
use the batch template and skip consultation, but they still need a focused
fixture, an existing batch/lane guard assertion, and sunset metadata. A
fast-path ceremony compresses repeated proof; it never waives proof.

## Active Docs Size Policy

The 800-line hard cap applies to source code files. It is not an active-docs
line cap.

Active restart / workstream / task-order / design docs should still stay
small enough to scan. Use these rules instead of a hard 800-line markdown
limit:

- keep active entry docs as pointers, current decisions, and next-action
  queues;
- keep the active execution section within the Minimal Execution Brief Law;
- move landed history, full inventories, and probe transcripts into phase
  cards, fixtures, ledgers, or investigation notes;
- keep `mirbuilder-rust-to-hako-converter-task-order-ssot.md` below 400 lines
  and 500 characters per line; `current_state_pointer_guard.sh` enforces the
  line-length boundary;
- when an active markdown file grows past roughly 1000 lines, open a docs-slim
  task or archive split before adding more historical prose;
- long archive / fixture / investigation docs may exceed that size when they
  are not restart entrypoints;
- do not split a doc just to satisfy a line count if the split would hide the
  SSOT or make the next action harder to find.

For source code, use the earlier design trigger owned by
`agent-current-entry-contract-ssot.md`: 760 lines requires a responsibility
split plan before semantic growth, while 800 lines remains the hard boundary.
Formatting compression is never an acceptable line-count fix.

## Active SSOT Header Policy

Every new durable SSOT, and every active SSOT receiving a material decision
revision, places a five-field Current Capsule near the beginning of its body:

```text
Current decision
Current implementation status
Next ordered task
Production stop line
Retirement finish line
```

The capsule is a navigation aid, not another current-state ledger. It must not
copy concrete `latest_card`, `current_blocker_token`, or `landed_tail` values.
Archived/historical notes and landed reference pages are exempt. The detailed
behavioral contract remains in the owning sections below the capsule.

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

## Unlocked Product Row First

```text
Decision:
  UNLOCKED-PRODUCT-ROW-FIRST-v1
```

Once a product-facing row's named prerequisites are green, a later-discovered
nonblocking proof, repair, cleanup, or cleanliness finding gets zero inserted
active rows ahead of it.

One already-accepted bounded detour may finish only through one predeclared
terminal. New findings inside that detour may not extend its terminal. A
detour may preempt again only when the selected product row's unchanged exact
gate proves that detour is a direct prerequisite; the card must name the
failing owner and the return row.

The current unlocked product is the finite in-place replacement parent:

```text
MIRBUILDER-INPLACE-REPLACEMENT0
  -> CALLABLE-DRAFT-PORT-CUTOVER0-I0-R0
```

Stage-B special activation, Ownership, selfhost migration, and cleanliness
findings are parked. They may not insert work ahead of the named production
replacement cell.

For this lane, “unlocked row first” means:

```text
new production edge active
+ selected old edge deleted
+ no fallback
```

It does not mean accumulating another disconnected proof before the
production edge.

### Production Cutover Convergence Budget

Every active replacement workstream names one finite first-production finish
line:

```text
named production caller
+ selected new authority
+ selected old authority deleted in the switch
+ fallback / retry after the switch = 0
+ focused production parity gate
```

Infrastructure receipts are prerequisites, not replacement progress. Each
prerequisite row must record the named caller, the exact gate that cannot pass
without it, the authority it creates, the old authority deleted (`none` when
honest), and the next cutover-facing row. A row with `old authority: none` is
active only when it is a direct prerequisite of that named gate; otherwise it
is parked.

The default budget is at most two consecutive implementation rows without a
production consumer, caller switch, or old-edge deletion. A longer bounded
Refactor Series requires one explicit T2 decision, one fixed terminal, and no
new rows inserted after the series starts. When the terminal lands, the next
row must return to the product-facing path. A newly found problem may preempt
only when the unchanged production gate fails at that exact owner; record the
return row instead of opening another general foundation.

Use this hardening admission classifier before inserting an audit finding:

```text
LiveBlocker     named current production/candidate reproducer reaches effect
                or publication
CutoverBlocker  the exact selected future-cutover path reaches the owner and
                its unchanged gate fails or cannot prove RejectBeforeEffect
LatentParked    neither reachability nor a named gate failure is present
CleanupParked   naming, LOC, generalization, or future platform/family only
```

Only `LiveBlocker` and `CutoverBlocker` may preempt an unlocked product row.
Reachable UB, corruption, or irreversible-effect risk on that exact path also
counts as a blocker. Stop the hardening tail when both blocker counts reach
zero, run the predeclared closeout gate, and retarget `CURRENT_STATE.toml`
explicitly to the named product row. Reopen only for a named gate failure, a
caller census changing from zero to nonzero, a new selected consumer, a
touched owner invalidating its proof, or a source/ABI/identity authority
change; grep-only suspicion and score/style findings do not reopen it.

This budget does not weaken source authority, atomic co-seal, fail-fast, or
full-coverage requirements. It changes task selection: correctness gaps stop
the named cutover, while unrelated generalization, cleanup, and future-family
support remain parked until after the first production replacement cell.

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

Zero-result machine-search loop breaker:

- after a design-stop consultation, at most one basis plus one inventory/rerun
  pair may be used to check an explicit machine-derived authority source;
- if that rerun still proves zero root authority or zero accepted evidence
  sources, return to the design stop instead of widening into another
  machine-search lane;
- parked future lanes may remain documented, but `next_documented_task` must
  not point to them until a later consultation or implementation result
  supplies a new non-zero authority source;
- diagnostic candidate names such as the narrowest-looking component are not
  selection authority unless a rerun proves exactly one root.

The stronger repeated-negative rule lives in
`agent-current-entry-contract-ssot.md`: three same-responsibility
`NoSafeSlice` / `NoStandaloneRow` outcomes force one premise audit, not another
consultation document or census. Keep that audit in the existing rolling card.

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

## Local Mechanical Selector Authority

Local agents may advance a selector without external design consultation when
the next card is selected by a mechanical, machine-checkable rule rather than a
semantic authority decision.

This authority is intentionally narrow. It exists to prevent operational
freshness checks and dependency-root selections from becoming repeated
consultation stops.

Allowed local selections:

- stale report / ledger rerun selected by hash or provenance freshness;
- native-owner checkpoint rerun selected by a fresh adoption delta;
- missing fixture formalization when a card references a non-existent durable
  fixture path;
- accepted fixture `selected_next_card` closeout when the fixture guard proves
  exactly one next card;
- dependency graph root blocker selection when the root is explicit and all
  other candidates depend on it;
- exactly-one guard-clean candidate selected by a previously documented proof
  tuple.

Required acceptance for local selection:

- the selector consumes current fixtures / cards / ledgers by stable path;
- the selected lane is reproducible from fixture fields, hashes, or dependency
  edges;
- when multiple lanes, freshness states, or blocker classes are plausible, a
  read-only worker inventory is run or explicitly waived with the reason in the
  card;
- forbidden proof axes are recorded as zero;
- the result does not change semantic ownership authority;
- the result does not claim Source Selfhost;
- if selection is not exactly one, the selector keeps the design stop active
  with a stable reason token.

External consultation is still required for:

- semantic owner or family selection when multiple proof-equivalent candidates
  remain;
- Source Selfhost claim or source authority map changes;
- generated artifact promoted to native edit authority;
- parent-owned surface promoted to standalone subject authority;
- forbidden non-claim boundary reinterpretation;
- ABI, backend route, language syntax, raw pointer / borrow semantics, or
  runtime fallback changes;
- new Python SemanticProjector or runner / VM / interpreter semantic owner
  claims.

Selectors using this authority should name it in the card or fixture as:

```text
local_selection_authority = LocalMechanicalSelectorAuthorityV1
```

If a worker inventory is used, record only the durable summary:

```text
worker_inventory = consumed
worker_inventory_scope = read_only_current_fixtures_cards_ledgers
```

Example:

```text
SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007 may locally select
MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-004 if the unconverted surface
report is stale after a native-owner adoption delta. If the report is fresh, it
may locally select SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-002 before any
blocker-class lane. It may not select a semantic owner, Source Selfhost claim,
ABI/syntax change, or parent-owned standalone promotion.
```

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
finite_product_goal = "..."
mirbuilder_north_star = "docs/.../mirbuilder-final-pipeline-ssot.md"
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

Do not add one scalar or path field for every completed row. A new
`CURRENT_STATE.toml` field is allowed only when current selection or restart
requires a machine-readable pointer that cannot be derived from an existing
field. Historical indexes, full card lists, and generated inventories belong
in an archive/ledger owned outside the current pointer. Compaction must not
change the active lane or blocker.

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

### Guard Result Classification

New or materially revised reusable guards and closeouts use exactly these
result classes:

- `current-change failure`: blocking;
- `known baseline debt`: separately tracked and nonblocking only under an
  existing active-card allowance with no regression and an exact owner;
- `informational census`: inventory only and never success evidence.

The class and owner must be visible in stable output or the check-script index.
An unclassified failure is blocking. A new row may repair baseline debt, but it
may not silently relabel or waive it. A census may select future work; it may
not prove correctness or completion. Existing green guards migrate when their
contract is next revised; this rule does not require a rename-only sweep.

For a behavior-preserving refactor, classify a red by rerunning the same
command at the parent commit before changing scope. Record the parent commit,
exact test, and result in the active card when the failure reproduces. If it
does not reproduce, treat it as a current-change failure. Do not weaken the
test or use a bare "known" label as a substitute for this comparison.

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
