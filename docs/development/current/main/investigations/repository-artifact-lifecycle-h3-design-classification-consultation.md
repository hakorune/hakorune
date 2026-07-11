---
Status: Active implementation workstream (hako-alloc-segment family)
Date: 2026-07-11
Owner: repository-artifact-lifecycle-current.md
Decision: C1 accepted; C2 hako-alloc-segment design accepted; residual sidecar ownership Q1-Q4 accepted and landed
---

# H3 Design Registry Classification Consultation

## Current Evidence

```text
design direct files = 849
registered rows = 120
unregistered files = 727
warning baseline = 732
registry violations = 0

accepted roles:
  authority
  navigation
  supporting
  status-ledger
  superseded
```

The registry schema, no-growth rule, precedence-cycle guard, sidecar ownership,
and README navigation boundary are implemented. What remains is semantic role
classification; filename/status/reference popularity cannot decide it safely.

## C1 Closeout (Accepted)

C1 used only explicit README section evidence. The reviewed rows are now in
`design/INDEX.md` with `classification_basis` recorded per row. No file move
was performed, and the remaining backlog stays warning-unregistered.

```text
c1_review_basis = explicit README section evidence
c1_review_rows = 112
c1_role_counts = authority:107, supporting:2, status-ledger:3
registered_rows = 117
unregistered_current = 732
unregistered_baseline = 732
registry_violations = 0
design_file_move_started = 0
```

C2 is now the active stop: remaining files are grouped by explicit owner
family, and ambiguous families require a focused consultation before a role
or physical move is assigned.

## C2 Deterministic Queue

The inventory now exposes a review queue only. It groups each remaining
unregistered direct file by a case-folded three-token filename prefix. This
is a scheduling key, not an owner claim and not a role assignment.

```text
queue_basis = deterministic three-token filename prefix queue only
family_count = 393
multi_file_family_count = 37
singleton_family_count = 356
largest_family = hako-alloc-segment:163
role_assignment = none
```

## C2 First Family Review: hako-alloc-segment

The design consultation is accepted. This family is a deterministic queue
family only, not a semantic owner family. It is isolated from external
direct-design references, but its internal graph contains many row-to-row
links and its documents require explicit content review.

```text
family_files = 163
status_counts = SSOT:64, accepted:21, active:69, mimap_active:9
filename_markers = ledger:98, closeout:64, diagnostics:29, bridge:27
body_markers = model_only:27, guard_only:16, proof_only:6
external_incoming_references = 0
authority_spine = unresolved
role_assignment = none
physical_move = forbidden
```

Questions for this family:

1. Is `hako-alloc-segment` one owner family, or must allocation,
   arena-backing, segment-map, and lifecycle become separate families?
2. Which registered authority, if any, is the precedence parent for the
   family: `hako-alloc-policy-state-contract-ssot.md`, the mimalloc
   port boundary, or a new family authority?
3. Which accepted `SSOT` rows are normative owners, and which are
   supporting/status-ledger evidence despite their historical `SSOT` label?
4. May closeout/diagnostic/ledger rows be classified after explicit
   content review, or do they require a family-owned supersession map?
5. If the family is isolated, may its closed subclusters move together only
   after `superseded_by` and internal-link closure are recorded?

The complete family summary is generated in the repository lifecycle
manifest. C2 must review an authority spine, precedence parent, and
retirement condition for each family; a family without a clear spine stops
for a focused consultation.

## Accepted C2 Design

```text
semantic_subfamilies:
  segment-lifecycle-and-membership
  segment-allocation-and-local-reuse
  segment-arena-backing-and-residence
  segment-map-and-release

new_family_authority_document = 0
historical_ssot_suffix_implies_authority = 0
c2_superseded_assignment = 0
c2_physical_move = 0
```

Authority remains anchored in the existing chain:

```text
INDEX.md
  -> hako-alloc-policy-state-contract-ssot.md
  -> hako-alloc-mimalloc-port-identity-boundary-ssot.md
  -> mimalloc-hako-port-implementation-plan-ssot.md
  -> mimalloc-lifecycle-rewrite-blueprint-ssot.md
```

The lifecycle blueprint is an existing authority candidate to register;
no umbrella authority document is created. Proof/pilot rows are supporting
candidates, model/ledger/readiness rows are status-ledger candidates,
diagnostic/closeout rows are sidecar candidates only after one-base review,
and bridge rows remain individual supporting review candidates.

S1 was complete at the start of this review: the existing authority chain was
represented in `INDEX.md`, the lifecycle blueprint was registered as its child
authority, and no new authority document was created. The initial 163 rows
were then projected for S2 content review.

The S2 projection is generated at
`tools/checks/manifests/hako_alloc_segment_family_projection_v0.json` for the
159 remaining rows. It contains body evidence and role hints only; owner,
precedence, and sidecar fields remain empty until explicit review.

## C2 S3 Lifecycle/Membership Review

The first content-review slice is complete. Two proof-only supporting rows were
registered under `mimalloc-lifecycle-rewrite-blueprint-ssot.md`:

```text
hako-alloc-segment-lifecycle-scalar-state-ssot.md
hako-alloc-segment-page-membership-scalar-ssot.md
```

Their guard-only closeout documents are owned sidecars. No new authority was
created, and neither superseded assignment nor physical movement was made.
The remaining queue is 159 rows; the next explicit review slice is
allocation/local-reuse.

The allocation-readiness scalar is also reviewed in this S3 pass as a
proof-only supporting row under `mimalloc-hako-port-implementation-plan-ssot.md`.
Its guard-only closeout is an owned sidecar. The modeled-consume row is
classified as `supporting`, while the modeled ledger is `status-ledger`; their
two closeout documents are owned sidecars. The registry now has 123 rows and
five owned sidecars, with 153 rows remaining. The next explicit review slice
is local-free.

The clear local-free chain is now also classified: candidate and apply-plan
rows are `status-ledger`, while page-apply and integration are `supporting`;
their two closeouts are owned sidecars. The registry now has 127 rows and
seven owned sidecars, with 147 rows remaining. The reuse closeout lacking a
current direct base row stays unclassified until its owner is explicit.

## C2 Residual Sidecar Ownership Design Stop

Further classification is intentionally stopped. The scalar-lane closeout
claims closure for multiple base rows, so assigning it as a sidecar to any one
base would falsify ownership. The local-free reuse closeouts refer to MIMAP
rows whose direct base documents are not present in the current design root,
so registering them as sidecars would create orphan ownership. No new
authority document is proposed.

Consultation questions:

```text
1. Should the multi-base scalar-lane closeout remain an unregistered grouped
   closeout, or should the registry gain an explicit group/closeout relation?
2. Should reuse closeouts remain unregistered until their base documents are
   restored/registered, or should an existing owner row absorb them?
3. May the blocked-substrate matrix be registered independently as a
   status-ledger row while the closeout ownership stop remains open?
```

Until these questions are answered, superseded assignment, physical movement,
and baseline lowering remain disabled.

## Consultation Request: Residual Sidecar Ownership

Please decide the registry representation for the four remaining local-free
residual groups below. This is a design consultation only; no registry schema,
role assignment, file move, or baseline change is requested before the answer.

### Evidence

```text
registered_rows = 127
owned_sidecars = 7
remaining_hako_alloc_segment_rows = 147
precedence_cycles = 0
external_incoming_references_for_family = 0
physical_move = 0
```

The registry currently enforces these laws:

```text
one direct document = at most one registry row
one sidecar = exactly one owning base row
sidecar != independent document row
superseded row requires superseded_by
no physical move before reachable-reference closure
```

### Affected Documents

```text
multi-base closeout:
  hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout-ssot.md
  closes MIMAP-107A, MIMAP-109A, and MIMAP-111A together

closeout without current direct base:
  hako-alloc-segment-allocation-modeled-local-free-reuse-closeout-ssot.md
  hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-closeout-ssot.md

independent residual candidate:
  hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md
```

### Questions For Decision

```text
Q1. Multi-base closeout:
    Keep it warning-unregistered as a grouped historical closeout, or add a
    typed group/closeout relation to the registry without weakening the
    one-owner sidecar invariant?

Q2. Missing-base closeouts:
    Keep both reuse closeouts warning-unregistered until their direct base
    documents are restored/registered, or assign them to an existing owner
    row? If assigning, which durable parent is authoritative?

Q3. Blocked-substrate matrix:
    Register it independently as a status-ledger row under
    mimalloc-hako-port-implementation-plan-ssot.md while Q1/Q2 remain open,
    or keep the entire residual group stopped as one transaction?

Q4. No new authority:
    Confirm that no umbrella SSOT and no filename-derived role rule should be
    introduced to resolve these cases.
```

### Constraints After Decision

```text
allowed:
  explicit INDEX row/sidecar updates
  generated inventory/projection refresh
  focused orphan/cycle/reference guards

forbidden:
  assigning one multi-base closeout to an arbitrary base
  absorbing orphan closeouts into an unrelated owner
  lowering the 732 baseline before the reviewed batch is closed
  superseded assignment or physical movement
```

### Proposed Minimal Follow-up

After Q1-Q4 are answered, land one focused batch only: either the approved
registry relation/ownership representation or the blocked-substrate matrix
row. Re-run strict inventory and pointer/docs guards before selecting the next
local-free or arena/map review slice.

### Accepted Decisions (Q1-Q4) And Landing

```text
Q1 = no group/closeout relation; multi-base closeouts are independent
     status-ledger rows whose classification_basis names the closed MIMAP set
     (scalar-lane MIMAP-107A/109A/111A; map-readiness MIMAP-149A/151A/153A —
     the second multi-base closeout was found during full content review)
Q2 = no absorption into an unrelated owner; the four reuse closeouts are
     independent status-ledger rows whose classification_basis records that
     their direct base rows (MIMAP-126A/130A/134A/138A) are absent from the
     design root
Q3 = blocked-substrate matrix registered independently as status-ledger under
     mimalloc-hako-port-implementation-plan-ssot.md
Q4 = confirmed: no umbrella authority, no filename-derived role rule; every
     row was confirmed against its body MIMAP statement
```

Landing evidence (single batch, full remaining family):

```text
rows_added = 67 (supporting:20, status-ledger:47)
owned_sidecars_added = 80 (76 stem-paired + 4 resolved by MIMAP close statements)
registered_count = 127 -> 194
owned_sidecar_count = 7 -> 87
unregistered = 715 -> 568
unregistered_baseline = 732 -> 568
registry_violations = 0
strict inventory check = green
current pointer guard = green
hako_alloc_segment_family_remaining = 0
physical_move = 0 (C3 unchanged)
```

## Minimum Next Slice

```text
H3-C2-HAKO-ALLOC-SEGMENT-FAMILY-CLASSIFICATION

S1 authority spine and existing authority registration
S2 explicit subfamily/content review projection for all 163 rows
S3 registry rows and sidecar ownership landing
S4 exact baseline update only after the batch is review-green
S5 cycle/orphan/reference/pointer/docs-slim/dev-gate guards
```

No role is assigned from filename suffix or historical SSOT header alone.
Superseded assignment and physical movement remain C3 work.

## Proposed Classification Order

```text
C1 root authority review:
  DOCS_LAYOUT / AGENTS / CURRENT_STATE / INDEX seed union

C2 owner-family review:
  group remaining files by explicit owner/prefix family
  select one authority spine per family
  classify explanations as supporting
  classify mutable ledgers as status-ledger

C3 supersession review:
  require superseded_by
  require root-reachability/reference closure
  then move to design/superseded

C4 strict closeout:
  every direct file is a row or owned sidecar
  unregistered = 0
  README projection checked/generated from INDEX
```

## Questions

```text
1. May C1 classify only documents explicitly named by current root surfaces,
   leaving the rest warning-unregistered until owner-family review?

2. Is one authority spine per owner family the required default, with multiple
   authority rows allowed only when precedence_parent makes the split explicit?

3. Should `*-closeout*`, `*-inventory*`, `*-report*`, and mutable TOML proof
   artifacts default to review candidates for status-ledger, never automatic
   classification?

4. Should an owner-family with no clear authority spine stop for a focused
   consultation instead of assigning supporting by heuristic?

5. Is physical movement forbidden until the entire superseded row's incoming
   reachable-reference set is zero, even when its owner family is otherwise
   classified?
```

## Recommended Answer

Accept all five. Classification should advance in reviewed owner-family
batches; suffixes and reference counts generate queues only and never assign
roles.

## Minimum Next Slice After Acceptance

```text
1. Generate deterministic C1 review queue.
2. Add reviewed C1 rows to INDEX.
3. Lower unregistered baseline to the new exact count.
4. Verify no precedence cycle, orphan sidecar, or README authority drift.
5. Do not move design files in C1.
```

## Non-Claims

```text
design_registry_complete = 0
design_registry_decided = 1
unregistered_design_files = 732
h3_c2_hako_alloc_segment_design_accepted = 1
hako_alloc_segment_semantic_subfamily_count = 4
hako_alloc_segment_all_individual_roles_reviewed = 0
hako_alloc_segment_new_authority_document = 0
hako_alloc_segment_superseded_assignment = 0
hako_alloc_segment_physical_move = 0
heuristic_role_assignment = 0
design_file_move_started = 0
strict_design_registry_guard = 0
failure_outcome_design_accepted = 1
selfhost_claim = 0
```

## C2 Singleton M-Z Residual Consultation List

The singleton m-z slice (177 files) landed 172 rows and 1 owned sidecar. The
following items were intentionally not classified/registered and require a
focused consultation before a role is assigned.

Unregistered (4):

```text
1. mirbuilder-rust-to-hako-converter-task-order-ssot.md
   current-only restart-entry task order that other rows use as precedence
   parent; behaves like a mutable task order (status-ledger) yet is used as
   a family spine (authority). Its two dependents were landed with a
   provisional parent (selfhost-parser-mirbuilder-migration-order-ssot.md)
   and must be reparented if this row is accepted as authority.
2. mimalloc-port-remaining-inventory-ssot.md
   header says "historical SSOT" and the body declares the D206 direction
   historical, but there is no explicit Moved-to / superseded_by target;
   superseded (needs superseded_by) vs historical status-ledger is open.
3. pattern-naming-migration-ssot.md
   completed historical migration plan (Phases A/B/C done 2026-01-29) with
   no Moved-to stub; superseded vs historical status-ledger is open
   (recipe-file-naming-unification-ssot.md is its result ledger).
4. source-selfhost-wider-route-selection-basis-ssot.md
   consultation-gated route-selection basis; reads as a stop-line
   (authority) but functions as a decision-basis record (status-ledger).
```

Registered with a tentative role, flagged borderline (3 + sidecar owner):

```text
5. mimalloc-hakorune-{brand-type,record,capability-surface}-vocabulary/
   surface SSOTs (MIMAP-005A/B/D): registered as supporting blueprint
   models, but they define normative brand/record/capability vocabulary
   and may deserve authority under the lifecycle blueprint.
6. rustc-semir-internal-adapter-boundary.md: Status "Design" registered as
   authority with two Design-stage supporting children; confirm whether
   Design-stage boundary docs may hold authority before acceptance.
7. promoted-name-resolution-deny-closeout.md: landed as an owned sidecar of
   promoted-body-locals-lifecycle-inventory.md; the closeout also touches
   the PHI-carrier consumer inventory, so the one-owner choice should be
   confirmed.
```

No physical move, no supersession execution, and no baseline change beyond the
reviewed batch (568 -> 395) were made in this slice.

## A-L Singleton Residual Consultation List

The a..l singleton batch (C2-singleton-al) landed 156 rows
(authority:32, supporting:66, status-ledger:52, superseded:6). The two
base-absent closeouts (`hako-alloc-backend-matcher-no-growth-closeout-ssot.md`,
`hako-alloc-execution-seam-summary-closeout-ssot.md`) were registered as
independent status-ledger rows per the accepted Q2 precedent. Six files stay
warning-unregistered pending consultation:

```text
1. exitbranch-ssot.md AND exit-branch-feature-ssot.md
   Two active SSOTs both defining the CorePlan "ExitBranch" feature (one framed
   compiler-cleanliness/BoxShape, one framed exit-carrying branch
   commonization). Owner is undecided; registering both as supporting would
   duplicate the feature's truth. Which is the owner row, and does the other
   become superseded or an owned sidecar?

2. collection-raw-substrate-contract-ssot.md
   Title is contract-shaped (authority candidate) but Status is provisional and
   the body mixes a boundary contract with a collection-owner cutover
   first-order ledger. Authority vs supporting (under
   raw-array-substrate-ssot.md) vs status-ledger?

3. hakorune-naming-and-rename-task-order-ssot.md
   Mixes a guard-enforced naming charter (authority; naming_charter_guard.sh)
   with a nyash->hakorune rename task order (status-ledger). Register whole doc
   as authority, or require a content split first? Its child inventory
   (hakorune-stage-term-existing-name-migration-inventory.md) was registered
   with provisional parent INDEX.md until this is decided.

4. boxbase-new-external-consultation-question.md
   Consultation paired 1:1 with box-identity-view-allocation-design-note.md
   (itself supporting, not an authority base). Independent supporting row, or
   owned sidecar of a non-authority base — which does the one-owner sidecar
   law prefer here?

5. joinir-pattern-selection-shadow-ssot.md
   Status retired (2026-03-05, commit 0df74eaa5); the shadow modules were
   removed and there is no successor document (planner trace moved to code
   `trace_try_take_planner`). superseded_by has no document target: is a code
   reference acceptable, or does it stay warning-unregistered until C3?
```

No physical move, no new authority beyond the reviewed
`allocator-replacement-hook-boundary-ssot.md` spine, and no baseline change
beyond the exact landed count (395 -> 239).
