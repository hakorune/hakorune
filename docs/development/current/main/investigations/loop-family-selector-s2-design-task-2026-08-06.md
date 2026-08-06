---
Status: Closed worker-reviewed design / caller-zero implementation
Date: 2026-08-06
Decision: LOOP-FAMILY-SELECTOR-S2-DESIGN
Authority: docs/development/current/main/design/loop-family-observation-policy-ssot.md
---

# Loop-family selector S2 design stop

## Purpose

Define the sole consumer of the landed
`VerifiedLoopFamilyAdmissionWindowV1`. This card is a design boundary only;
it does not open a selector implementation, Recipe handoff, Builder/MIR
caller, physical cutover, or legacy retirement.

## Accepted input

The selector consumes only `Ready(window)` from the assembler, exactly once.
Therefore its input is already a fixed five-row product whose rows are only
`Candidate` or `Declined`; row-level `Rejected`/`Unresolved`, missing or
duplicate tags, identity/mode/coverage mismatches, and their evidence stop in
the assembler and never reach the selector. The window owns the resolver-
issued identity lease, fixed five family rows, common mode, and complete
coverage. The selector must not re-resolve source, inspect AST, read route
IDs/schedules/cursors, or infer family precedence.

## Fixed disposition algebra

```text
one Candidate + four Declined -> Selected(candidate)
two or more Candidates        -> Rejected(Overlap)
five Declined                  -> Unresolved(OutOfWindow)
NoCandidate                    -> not an S2 outcome; requires a separate
                                  whole-unit proof product (M8)
```

Candidate payloads and the resolver lease remain opaque, move-only capabilities
of the selected product. The selector may count candidate dispositions and
apply semantic overlap; the assembler may not. `OutOfWindow` is the sealed
five-row negative result for this window. It is not `NoCandidate`, because the
common window covers one loop-family envelope, not the whole program.

## Output boundary

The future `CanonicalLoopFamilySelectionV1` must retain the selected family
tag, its typed candidate payload, the source lease, and the sealed mode. A
selector rejection/unresolved result must retain the consumed window's lease,
all five rows, and the typed reason (`Overlap` or `OutOfWindow`). There is no
row-rejected selector arm because the assembler owns that boundary. The
selector must not issue `LoopBindingKeyV1`, Recipe/JoinSig, ValueId/PHI,
Builder/MIR, or runtime effects. A later demand/Recipe card owns those
products.

## Acceptance evidence required before implementation

- one focused consumer test for one candidate plus four declines;
- two-candidate overlap rejection with both payloads retained;
- assembler-focused evidence that non-Ready rows never enter the selector;
- five-declined `OutOfWindow` distinction;
- move-only one-shot window consumption and no source relookup;
- caller-zero/line guard with selector-only ownership;
- same-commit reference-document update and post-implementation receipt.

The implementation belongs in a new `family_selector.rs` and focused test
module. The old `family_selection.rs` Generic marker remains historical
test-only evidence; it is not promoted or used as the canonical selector.
Selector S2 must remain below 800 lines, keep production callers at zero, and
not import AST, resolver issuers, structural-facts producers, route policy,
Recipe/JoinSig, Builder/MIR, retry, or fallback.

Until those design points are accepted and taskized, the current goal stops at
this consultation boundary. Production selection and legacy deletion remain
closed.

## Design closeout

Independent worker reviews confirmed the boundary: the assembler owns every
row-level failure, while a new selector consumes only `Ready(window)` and
handles Candidate/Declined overlap. `NoCandidate` remains outside S2. The
implementation task is
`loop-family-selector-s2-implementation-task-2026-08-06.md`; production
selection and legacy deletion remain closed until later activation gates.

## Implementation closeout

The caller-zero implementation is landed in `family_selector.rs` with three
focused tests and the shared selector/caller-zero guard. The selector consumes
only `Ready(window)` and preserves lease/row evidence on `Selected`,
`Rejected(Overlap)`, and `Unresolved(OutOfWindow)`. The implementation commit
also synchronizes the loop SSOT, reference matrix, module README, workstream,
and current mirrors. The next shallow gate is the existing
`GENERIC-SELECTION-OPEN-D0` candidate-envelope design; Recipe handoff,
physical/production activation, and legacy retirement remain closed.
