---
Status: Design stop / implementation not authorized
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

The selector consumes the admission window exactly once. The window already
owns the resolver-issued identity lease, fixed five family rows, common mode,
and complete coverage. The selector must not re-resolve source, inspect AST,
read route IDs/schedules/cursors, or infer family precedence.

## Fixed disposition algebra

```text
one Candidate + four Declined -> Selected(candidate)
two or more Candidates        -> Rejected(Overlap)
any row Rejected               -> Rejected(row evidence)
no Rejected + any Unresolved   -> Unresolved(row evidence)
five Declined                  -> Unresolved(OutOfWindow)
NoCandidate                    -> forbidden until a sealed whole-unit proof
```

Candidate payloads and the resolver lease remain opaque, move-only capabilities
of the selected product. The selector may count candidate dispositions and
apply semantic overlap; the assembler may not. `NoCandidate` is not a synonym
for five family declines because the common window covers one loop-family
envelope, not the whole program.

## Output boundary

The future `CanonicalLoopFamilySelectionV1` must retain the selected family
tag, its typed candidate payload, the source lease, and the sealed mode. A
rejection/unresolved result must retain the consumed window evidence. It must
not issue `LoopBindingKeyV1`, Recipe/JoinSig, ValueId/PHI, Builder/MIR, or
runtime effects. A later demand/Recipe card owns those products.

## Acceptance evidence required before implementation

- one focused consumer test for one candidate plus four declines;
- two-candidate overlap rejection with both payloads retained;
- row-rejected dominance over unresolved evidence;
- five-declined `OutOfWindow` distinction;
- move-only one-shot window consumption and no source relookup;
- caller-zero/line guard with selector-only ownership;
- same-commit reference-document update and post-implementation receipt.

Until those design points are accepted and taskized, the current goal stops at
this consultation boundary. Production selection and legacy deletion remain
closed.
