---
Status: historical ledger
Date: 2026-08-18
Decision: preserve landed design/implementation history outside the current-owner SSOT
Scope: Loop common physical demand/session D0/I0, canary, and closeout history
Related:
  - ../loop-common-physical-demand-and-session-ssot.md
  - ../current-docs-archive-policy-ssot.md
---

# Loop Common Physical Demand and Session — historical ledger

This is historical evidence only. It is not a current task, pointer, semantic
authority, production switch, or blocker source. Continue from
CURRENT_STATE.toml and
../loop-common-physical-demand-and-session-ssot.md.

The current-owner SSOT was 7,949 lines at the 2026-08-18 compaction point.
Its append-only D0/I0 and canary prose was mixed with the durable contract.
The current owner now keeps the contract and the live 2026-08-18 chain; this
ledger records what was removed from that live surface.

## Archived slices

| Source slice at compaction | Historical contents | Current treatment |
| --- | --- | --- |
| 124–5,040 | Callable/G0/Common-V2 D0, I0, design stops, canaries, and closeouts from 2026-08-07 through 2026-08-17 | summarized in the current capsule; exact prose remains in Git history |
| 5,129–5,443 | detailed Common-V2 pre-session chronology and transport receipts | compact contract summary remains live |
| 6,132–6,819 | finite ladder, Dynamic overlay, and closed preparation/physicalizer receipts | compact execution ladder remains live |
| 6,853–7,058 | pre-Return-read canary and closeout receipts | current execution boundary resumes at the 2026-08-18 Return-source chain |

The source snapshot is commit 9a8f398ed4. Exact historical text is
recoverable with git show / git log --follow for the current-owner path; this
ledger deliberately avoids copying the 400+ KiB append-only body back into the
live current surface.

## Archived heading index

The headings below are navigation evidence for the removed slices. They are
not independent authority and must not be used to select a new implementation
row.

### Historical pre-cutover authority coverage census (snapshot before G0 D0)
#### Census result and next co-seal design stop
### LOOP-SEMANTIC-PROGRAM-COSEAL-CALLABLE-I0 implementation receipt (2026-08-17)
### LOOP-SEMANTIC-PROGRAM-COSEAL-ALL-FAMILY-R0 accepted design boundary (2026-08-17)
### LOOP-PRECUTOVER-AUTHORITY-G0-D0 accepted design boundary (2026-08-17)
#### G0 parent BoxShape to settle before I0
#### LOOP-PRECUTOVER-AUTHORITY-G0-SOURCE-COHORT-D0 accepted design boundary
#### LOOP-PRECUTOVER-AUTHORITY-G0-I0 implementation receipt (2026-08-17)
### Post-G0 design stop: split S6C physical entry from Generic G0
### `LOOP-GENERIC-G0-PHYSICAL-ENTRY-SOURCE-PROJECTION-D0` (BoxShape accepted; source projection I0 next)
### `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-ENTRY-D0` (BoxShape accepted; Generic-only)
### `LOOP-GENERIC-G0-PHYSICAL-EFFECT-PROJECTION-D0` (accepted BoxShape; I0 landed)
### `LOOP-GENERIC-G0-PHYSICAL-FUNCTION-SKELETON-D0` (accepted BoxShape, 2026-08-17)
### `LOOP-GENERIC-G0-PHYSICAL-ENTRY-LANE-ADOPTION-D0` (accepted BoxShape, 2026-08-17)
### `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` (next design stop)
#### Adoption I0 implementation census (2026-08-17)
### Generic G0 source-projection child tasks (ordered; post-adoption checkpoint next)
#### `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-D0` (accepted 2026-08-17)
#### `LOOP-GENERIC-G0-FUNCTION-EFFECT-PROJECTION-I0` (landed 2026-08-17)
#### `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-D0`
#### `LOOP-GENERIC-G0-RESULT-ABI-TRANSPORT-I0`
#### `LOOP-GENERIC-G0-COMPLETION-PROJECTION-D0` (accepted BoxShape 2026-08-17)
#### `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` (landed transport I0)
#### `LOOP-GENERIC-G0-BODY-EFFECT-TRANSPORT-D0` implementation receipt (2026-08-17)
#### `LOOP-GENERIC-G0-TOPLEVEL-DECLARATION-HEADER-I0` (accepted bounded source projection)
#### `MIRBUILDER-CANARY-CONVERGENCE-CHECKPOINT-R0` (parked after the parent cohort)
#### `MIRBUILDER-CANARY-CONVERGENCE-MANIFEST-R0` (read-only taskization, 2026-08-17)
### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-CONTRACT-D0` (next design stop)
### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-EMISSION-D0` (accepted ownership BoxShape)
#### D0 audit result — emitter admission BoxShape accepted 2026-08-17
#### Admission I0 closeout and bounded retirement order (2026-08-17)
#### Generic G0 detached-entry retirement R0 closeout (2026-08-17)
#### Production candidate census R0 — Generic remains caller-zero (2026-08-17)
#### Production selection D0 closeout — retain the selected-Dynamic arm (2026-08-18)
#### `DYNAMIC-EXIT-PHYSICAL-SESSION-P0` — next design stop
#### H2-S2-S1-R1 selected-initializer bridge feasibility closeout (design stop, 2026-08-18)
#### `PHYSICAL-INPUT-AUTHORITY-I0` — Dynamic result/input conformance design stop (2026-08-18)
#### `PHYSICAL-INPUT-AUTHORITY-D0` — accepted BoxShape (2026-08-18)
#### Post-Dynamic unification audit receipt (2026-08-18)
### Accepted fast slice: transfer-authority negative evidence (2026-08-18)
### Accepted design stop: topology retirement census (2026-08-18)
### Accepted design stop: If branch/merge coverage consultation (2026-08-18)
### Scope correction: LoopRecipe If coverage (2026-08-18)
### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RELATION-D0 (2026-08-18)
#### LOOP-PHYSICAL-IF-CONTINUATION-RELATION-I0 — execution brief
### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-CONSUMER-D0 (2026-08-18)
#### LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0 — execution brief
#### LOOP-PHYSICAL-IF-CONTINUATION-TARGET-PLACEMENT-I0 — implementation receipt (2026-08-18)
#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-BRANCH-EMISSION-D0 (2026-08-18)
#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-SPLIT-TERMINAL-AUTHORITY-D0 (2026-08-18)
#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-COSEAL-D0 (2026-08-18)
#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-ISSUER-BOUNDARY-D0 (2026-08-18)
#### Accepted D0: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-D0 (2026-08-18)
#### Next execution: LOOP-PHYSICAL-IF-CONTINUATION-RETURN-SOURCE-RECIPE-BINDING-I0
#### Semantic-program consume D0 — accepted BoxShape (2026-08-17)
#### Semantic-program consume I0 closeout (2026-08-17)
#### Structural convergence audit — migration thickness is classified, not an authority (2026-08-17)
#### Structural-debt disposition follow-up — parked behind production selection (2026-08-17)
#### Structural debt review follow-up — accepted disposition, still parked (2026-08-18)
#### Session-preflight issuer census — design stop remains open (2026-08-17)
#### Session-preflight entry-row retention closeout (2026-08-17)
#### Session-preflight I0 closeout (2026-08-17)
#### Next design stop — Generic-to-common operation-emitter owner
#### Dispatcher input census — design-only refinement (2026-08-17)
#### Dispatcher preflight I0 closeout (2026-08-17)
#### Operation emitter I0 closeout (2026-08-17)
#### Session-preflight D0 decision — accepted BoxShape (2026-08-17)
#### `LOOP-GENERIC-G0-SEALED-CONSUME-I0` closeout (2026-08-17)
### `LOOP-GENERIC-G0-PHYSICAL-OPERATION-COHORT-D0` (accepted BoxShape)
### Canonical session admission D0 (accepted three-step boundary)
### RESOLVED-BLOCK-EXPR-EXPECTATION-I0 implementation receipt (2026-08-17)
### CALLABLE-BLOCK-EXPR-EXPECTATION-TRANSPORT-I0 implementation receipt (2026-08-17)
### LOOP-COMMON-V2-CANONICAL-SESSION-ADMISSION-I0 implementation receipt (2026-08-17)
### LOOP-COMMON-V2-PHYSICAL-SESSION-I0 implementation receipt (2026-08-17)
### LOOP-S6C-COMMON-V2-PHYSICAL-FUNCTION-ENTRY-D0/I0 (S6C-only; landed)
### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-FUNCTION-SKELETON`
### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-LANE-ADOPTION`
### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-ENTRY-SESSION-SEAM`
### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-SESSION-STAMP-RETENTION-I0` implementation receipt (2026-08-17)
### Accepted D0 / active I0: `LOOP-COMMON-V2-PHYSICAL-LAYOUT-INPUT-D0`
### Physical layout input I0 implementation receipt (2026-08-17)
### Source-segment allocation boundary (landed 2026-08-17)
### Segment block allocation I0 implementation receipt (2026-08-17)
### `LOOP-COMMON-V2-PHYSICAL-AFTER-BOUNDARY-D0` — accepted BoxShape
### After-boundary transport I0 implementation receipt (2026-08-17)
### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-D0` — accepted BoxShape
### `LOOP-COMMON-V2-PHYSICAL-AFTER-ALLOCATION-I0` — landed 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-EDGE-D0` — accepted BoxShape 2026-08-17
### Branch-plan transport I0 implementation receipt (2026-08-17)
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-CARRIER-D0` — accepted BoxShape 2026-08-17
### Condition producer relation I0 implementation receipt (2026-08-17)
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-D0` — parent design stop 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-RESULT-BOXSHAPE-D0` — design stop 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-OPERAND-PRODUCER-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-OPERAND-INVENTORY-I0` — landed 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RESULT-I0` — landed caller-zero canary 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-I0` — landed caller-zero 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-TARGET-PLAN-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-CONDITION-BLOCK-TARGET-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIVER-OPERAND-D0` — accepted BoxShape 2026-08-18
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-CALL-DIRECT-EMITTER-I0` — landed caller-zero canary 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-PHYSICAL-RESULT-D0` — design stop 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-LENGTH-RECEIPT-LIFETIME-I0` — landed caller-zero 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-AFTER-CONDITION-BOOL-RESULT-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-D0` — accepted BoxShape 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-SOURCE-TRANSPORT-I0` — landed caller-zero slice 2026-08-17
### `LOOP-COMMON-V2-PHYSICAL-INITIAL-INDEX-SEED-I0` — landed caller-zero effect slice 2026-08-17
### Pre-cutover authority correction (2026-08-08)
#### S6C complete-V2 pre-session admission (2026-08-15)
### Common V2 pre-session BoxShape (accepted boundary; source implementation landed)
### Common V2 transport R0 (accepted boundary; installed Port implementation landed)
### Common V2 I0 issuer contract (accepted and landed caller-zero, 2026-08-16)
## Finite implementation ladder
### Selected Dynamic first-cutover overlay (2026-08-11)
### Pre-cutover execution briefs
### Post-Dynamic audit additions (2026-08-11)
### Closed implementation receipt: `CANONICAL-FUNCTION-FINISH-TERMINAL-R0`
### Closed design correction receipt: `LOOP-PHYSICAL-PREPARE-DESIGN-CORRECTION-R0`
### Closed implementation receipt: `LOOP-RECIPE-RECURSIVE-PHYSICALIZER-P0`
### Operation/effect design boundary
### Operation physicalizer design closeout
### ReadBinding leaf D0 correction (2026-08-07; Decision: accepted and landed)
#### ReadBinding source/effect mapping matrix
#### Entry, placement, service, and failure contracts
## Current execution boundary
### Callable physical-canary preparation slice (2026-08-07)
### Callable full physical canary closeout (2026-08-08)
### Generic G0 exact-ingress I0 closeout (2026-08-08)
### Recursive segment plan R1 closeout (2026-08-08)
### Segment block cutover R2 closeout (2026-08-08)
### R3-I0 implementation receipt (2026-08-08; Decision: accepted)
### G0 I1 D1 review closeout (2026-08-08; Decision: accepted)
### Common Predicate/carrier I0 closeout (2026-08-08; Decision: accepted)

## 2026-08-19 recompaction

The current-owner file grew again from its compacted form to 5,108 lines by
accumulating the 2026-08-18/19 S6C, Residence, DraftSeal, and backend design
stops. The source snapshot for this second compaction is commit `44e4df38a0`.
Exact text remains recoverable with:

```text
git show 44e4df38a0:docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md
```

No historical body was copied into this ledger. The live authority was
rewritten as durable contract plus one active brief and compact task table.

| Source slice at recompaction | Historical contents | Current treatment |
| --- | --- | --- |
| 4–55 | repeated frontmatter activation chronology | removed; current state is in `CURRENT_STATE.toml` |
| 229–1,874 | S6C corridor, Residence lifecycle, DraftSeal consumer, ABI, carrier, Trap, and detached Finish D0/I0 prose | compact landed ledger plus durable lifecycle law |
| 2,470–2,616 | old R0 audit, production census, and admission discussion | Git history only; no active selection authority |
| 2,689–2,722 | per-row implementation/documentation instructions | replaced by the repository-wide current-doc policy |
| 2,730–3,283 | Generic/Return/Branch/Return-read/shared-segment chronology | compact authority graph and landed row |
| 3,284–3,958 | TextEq/V9/ExactText residence consultations and closeouts | compact S6C source/representation/lifetime law |
| 3,959–4,982 | V9 MIR, portable TextEq, TextRef, wire/lease/StringBox and legacy verdict chronology | compact fast-corridor and explicit non-authority sections |
| 4,983–5,108 | real-candidate JSON carrier lineage and selected-C design stop | retained as the active six-line brief and ordered remaining tasks |

Grouped heading index for the second compaction:

- S6C scalar-scan corridor source, base-root, cursor, predicate/index, and
  scalar-equality rows;
- Text Formal Residence authority, materializer, DraftSeal ingress/consumer,
  finish-or-abort ABI, lifecycle carrier, Trap terminal, and detached Finish
  rows;
- Generic G0 Return source/Recipe/Join, Return-read, Branch emission, and
  shared-segment rows;
- TextEq Substring V9 target/callout/residence/ExactText ingress, V9 runtime
  producer, TextRef bridge/scope, wire ingress, StringBox reachability, and
  C-speed/legacy verdict rows;
- real-candidate carrier materialization/transport/JSON and synthetic textual
  C lowering fixture rows.

These groups are navigation evidence only. Current task selection must use
`CURRENT_STATE.toml` and the compact current-owner brief.

## Retention and restart rule

- Keep this ledger for traceability; do not append new execution receipts here.
- New decisions belong in the current-owner SSOT first, with one compact active
  row and the required reference/README evidence.
- If a historical detail is needed, inspect the source commit and the exact
  implementation/test evidence rather than restoring it to the current SSOT.
- The current blocker remains the one named by CURRENT_STATE.toml; archived
  NoSafeSlice text cannot advance or replace it.
